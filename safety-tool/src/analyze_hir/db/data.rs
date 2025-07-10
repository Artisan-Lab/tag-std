use super::super::{HirFn, is_tool_attr};
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::{Attribute, HirId, def_id::DefId};
use rustc_middle::ty::TyCtxt;
use safety_parser::property_attr::{parse_inner_attr_from_str, property::Kind, utils::expr_ident};
use std::{fmt, sync::LazyLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimaryKey {
    pub hash1: u64,
    pub hash2: u64,
}

impl PrimaryKey {
    fn new(def_id: DefId, tcx: TyCtxt) -> Self {
        let (hash1, hash2) = tcx.def_path_hash(def_id).0.split();
        PrimaryKey { hash1: hash1.as_u64(), hash2: hash2.as_u64() }
    }
}

#[derive(Debug)]
pub struct Func {
    /// Safety tool attributes
    pub tool_attrs: Vec<String>,
    /// Definition path (for debug purpose)
    pub def_path: String,
    /// Function source code without attributes (for debug purpose)
    pub function: String,
}

#[derive(Debug)]
pub struct Data {
    pub hash: PrimaryKey,
    pub func: Func,
}

impl Data {
    pub fn new(hir_fn: &HirFn, tcx: TyCtxt) -> Self {
        let def_id = hir_fn.local.to_def_id();
        let hash = PrimaryKey::new(def_id, tcx);

        let hid = hir_fn.hir_id;

        crossfig::switch! {
            crate::asterinas => {
                let function = rustc_hir_pretty::id_to_string(&tcx.hir(), hid);
            }
            _ => {
                let function = rustc_hir_pretty::id_to_string(&tcx, hid);
            }
        }

        let func = Func {
            tool_attrs: get_attrs(tcx, hid)
                .filter_map(|attr| opt_attribute_to_string(tcx, attr))
                .collect(),
            def_path: tcx.def_path_debug_str(def_id),
            function,
        };

        Data { hash, func }
    }
}

fn get_attrs(tcx: TyCtxt, hid: HirId) -> impl Iterator<Item = &Attribute> {
    crossfig::switch! {
        crate::asterinas => {
            tcx.hir_attrs(hid.owner).get(hid.local_id).iter()
        }
        _ => {tcx.hir_attrs(hid).iter() }
    }
}

fn opt_attribute_to_string(tcx: TyCtxt<'_>, attr: &rustc_hir::Attribute) -> Option<String> {
    is_tool_attr(attr).then(|| attribute_to_string(tcx, attr))
}

fn attribute_to_string(tcx: TyCtxt<'_>, attr: &rustc_hir::Attribute) -> String {
    rustc_hir_pretty::attribute_to_string(&tcx, attr).trim().to_owned()
}

pub type TagsState = FxIndexMap<Property, bool>;

#[derive(Debug, Default)]
pub struct ToolAttrs {
    map: FxIndexMap<PrimaryKey, Box<[Property]>>,
    /// State of safety tags shows if thet are discharged.
    tagged: TagsState,
}

impl ToolAttrs {
    pub fn new(data: &[Data]) -> Self {
        Self {
            map: data
                .iter()
                .filter(|d| !d.func.tool_attrs.is_empty())
                .map(|d| {
                    let mut v = Vec::with_capacity(d.func.tool_attrs.len());
                    d.func.tool_attrs.iter().for_each(|s| push_properties(s, &mut v));
                    (d.hash, v.into_boxed_slice())
                })
                .collect(),
            tagged: FxIndexMap::default(),
        }
    }

    pub fn get_tags(&mut self, def_id: DefId, tcx: TyCtxt) -> Option<&mut TagsState> {
        let key = PrimaryKey::new(def_id, tcx);
        self.get_tags_via_key(key)
    }

    fn get_tags_via_key(&mut self, key: PrimaryKey) -> Option<&mut TagsState> {
        let properties = self.map.get(&key)?;
        self.tagged.clear();
        self.tagged.extend(properties.iter().map(|p| (p.clone(), false)));
        Some(&mut self.tagged)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Property {
    property: Box<str>,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.property)
    }
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt(&self.property, f)
    }
}

impl Property {
    pub fn new_with_hir_id(hir_id: HirId, tcx: TyCtxt) -> Vec<Self> {
        let mut v = Vec::new();

        get_attrs(tcx, hir_id)
            .filter_map(|attr| opt_attribute_to_string(tcx, attr))
            .for_each(|s| push_properties(&s, &mut v));

        v
    }

    pub fn as_str(&self) -> &str {
        &self.property
    }
}

fn push_properties(s: &str, v: &mut Vec<Property>) {
    // `DISCHARGES_ALL_PROPERTIES=0` or unset will only check Memo properties.
    // When the env var is set, all properties will be checked.
    static DISCHARGES_ALL_PROPERTIES: LazyLock<bool> = LazyLock::new(|| {
        std::env::var("DISCHARGES_ALL_PROPERTIES").map(|var| var != "0").unwrap_or(false)
    });

    if let Some(property) = parse_inner_attr_from_str(s) {
        let property = if property.kind == Kind::Memo {
            // `Memo(Prop)` or `Memo_Prop` are normalized to call expr
            expr_ident(&property.expr[0]).to_string()
        } else if *DISCHARGES_ALL_PROPERTIES {
            property.kind_property()
        } else {
            return;
        }
        .into_boxed_str();

        v.push(Property { property });
    }
}
