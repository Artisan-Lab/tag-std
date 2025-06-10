use super::super::{HirFn, is_tool_attr};
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::{HirId, def_id::DefId};
use rustc_middle::ty::TyCtxt;
use safety_parser::property_attr::{parse_inner_attr_from_str, property::Kind, utils::expr_ident};

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
        let func = Func {
            tool_attrs: tcx
                .hir_attrs(hid)
                .iter()
                .filter_map(|attr| opt_attribute_to_string(tcx, attr))
                .collect(),
            def_path: tcx.def_path_debug_str(def_id),
            function: rustc_hir_pretty::id_to_string(&tcx, hid),
        };

        Data { hash, func }
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Property {
    property: Box<str>,
}

impl Property {
    pub fn new_with_hir_id(hir_id: HirId, tcx: TyCtxt) -> Vec<Self> {
        let mut v = Vec::new();

        tcx.hir_attrs(hir_id)
            .iter()
            .filter_map(|attr| opt_attribute_to_string(tcx, attr))
            .for_each(|s| push_properties(&s, &mut v));

        v
    }

    pub fn as_str(&self) -> &str {
        &self.property
    }
}

fn push_properties(s: &str, v: &mut Vec<Property>) {
    if let Some(property) = parse_inner_attr_from_str(s)
        && property.kind == Kind::Memo
    {
        let s = expr_ident(&property.expr[0]).to_string();
        v.push(Property { property: s.into_boxed_str() })
    }
}
