use super::super::{HirFn, is_tool_attr};
use itertools::Itertools;
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::{Attribute, HirId, def_id::DefId};
use rustc_middle::ty::TyCtxt;
use safety_parser::{
    configuration::Tag,
    safety::{Property as SP, parse_attr_and_get_properties},
};
use std::{borrow::Cow, fmt};

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

fn get_attrs(tcx: TyCtxt<'_>, hid: HirId) -> impl Iterator<Item = &'_ Attribute> {
    crossfig::switch! {
        crate::asterinas => {
            tcx.hir_attrs(hid.owner).get(hid.local_id).iter()
        }
        _ => {tcx.hir_attrs(hid).iter() }
    }
}

/// Convert the tool attribute to string.
fn opt_attribute_to_string(tcx: TyCtxt<'_>, attr: &rustc_hir::Attribute) -> Option<String> {
    is_tool_attr(attr).then(|| attribute_to_string(tcx, attr))
}

fn attribute_to_string(tcx: TyCtxt<'_>, attr: &rustc_hir::Attribute) -> String {
    rustc_hir_pretty::attribute_to_string(&tcx, attr).trim().to_owned()
}

#[derive(Debug, Default)]
pub struct TagState {
    /// Each tag must be discharged.
    vanilla: FxIndexMap<Property, bool>,
    /// Any one of the tags must be discharged. `any` tag can be specified multiple times.
    /// There won't be empty Map because [`args_in_any_tag`] never construct empty SP arguments.
    ///
    /// [`args_in_any_tag`]: safety_parser::safety::Property::args_in_any_tag
    group_of_any: Vec<FxIndexMap<Property, bool>>,
    /// If undischarged is called once. This ensures undischarged diagnostics are emitted only once.
    undischarged: bool,
}

impl TagState {
    fn clear(&mut self) {
        self.vanilla.clear();
        self.group_of_any.clear();
    }

    fn refresh(&mut self, props: &Properties) {
        self.clear();
        self.vanilla.extend(props.vanilla.iter().map(|p| (p.clone(), false)));
        self.group_of_any.extend(
            props.group_of_any.iter().map(|v| v.iter().map(|p| (p.clone(), false)).collect()),
        );
    }

    pub fn discharge(&mut self, prop: &Property) -> Result<(), String> {
        if let Some(state) = self.vanilla.get_mut(prop) {
            if *state {
                return Err(format!("{prop:?} has already been discharged"));
            }
            *state = true;
        } else {
            for group in &mut self.group_of_any {
                if let Some(state) = group.get_mut(prop) {
                    if *state {
                        return Err(format!("{prop:?} has already been discharged"));
                    }
                    *state = true;
                }
            }
        }
        Ok(())
    }

    // Returns true if there are SPs undischarged.
    // Returns false if SPs are fully discharged:
    // * each vanilla SP is discharged
    // * and at least one SP in each group_of_any is discharged
    // pub fn is_fully_discharged(&self) -> bool {
    //     self.vanilla.values().all(|b| *b)
    //         && self.group_of_any.iter().all(|g| g.values().any(|b| *b))
    // }

    pub fn undischarged(&mut self) -> Undischarged {
        let mut undischarged = Undischarged::default();
        if self.undischarged {
            return undischarged;
        } else {
            self.undischarged = true;
        }

        let vanilla = self
            .vanilla
            .iter()
            .filter_map(|(sp, state)| {
                if !*state {
                    undischarged.v_sp.push(sp.clone());
                    Some(sp.name())
                } else {
                    None
                }
            })
            .format_with(", ", |sp, f| f(&format_args!("`{sp}`")))
            .to_string();
        if !vanilla.is_empty() {
            undischarged.v_tags_displayed.push(vanilla);
        }

        for group in &self.group_of_any {
            let mut v_any_sp = Vec::with_capacity(group.len());
            if !group.values().any(|state| *state) {
                let any = group.keys().format_with(", or ", |sp, f| {
                    v_any_sp.push(sp.clone());
                    f(&format_args!("`{sp}`"))
                });
                undischarged.v_tags_displayed.push(any.to_string());
                undischarged.v_any_sp.push(v_any_sp);
            }
        }
        undischarged
    }
}

#[derive(Default)]
pub struct Undischarged {
    /// Each string is not mere tag name: it's a collection of tag names.
    pub v_tags_displayed: Vec<String>,
    /// Tags that should have been discharged individually.
    pub v_sp: Vec<Property>,
    /// Each element is a group of tags in `any` tag.
    pub v_any_sp: Vec<Vec<Property>>,
}

impl Undischarged {
    pub fn title(&self) -> String {
        let len = self.v_tags_displayed.len();
        if len == 0 {
            return String::new();
        }

        let undischarged_str = self.v_tags_displayed.join("\n");
        let newline = if len == 1 { " " } else { "\n" };
        let plural = if undischarged_str.matches(',').count() == 0 { "Tag is" } else { "Tags are" };
        format!("{plural} not discharged:{newline}{undischarged_str}")
    }

    pub fn info(&self) -> Vec<String> {
        let capacity = self.v_sp.len() + self.v_any_sp.iter().map(|v| v.len()).sum::<usize>();
        let mut v = Vec::with_capacity(capacity);

        for sp in &self.v_sp {
            v.push(format!("`{}`: {}", sp.name_with_args(), sp.info()));
        }

        for (idx, any) in self.v_any_sp.iter().enumerate() {
            for sp in any {
                v.push(format!("[any#{idx}] `{}`: {}", sp.name_with_args(), sp.info()));
            }
        }

        v
    }
}

#[derive(Debug, Default)]
struct Properties {
    vanilla: Vec<Property>,
    group_of_any: Vec<Box<[Property]>>,
}

impl Properties {
    fn push_attr(&mut self, attr: &str) {
        let props = &*parse_attr_and_get_properties(attr);

        // Usually tags are vanilla, so reserve enough sapce.
        let cap = props.iter().map(|prop| prop.tags.len()).sum();
        self.vanilla.reserve(cap);

        for prop in props {
            for tag in &*prop.tags {
                if let Some(v_sp) = tag.args_in_any_tag() {
                    // Push SPs in `any`
                    let iter = v_sp.iter().flat_map(|p| p.tags.iter().map(to_prop));
                    self.group_of_any.push(iter.collect());
                } else {
                    self.vanilla.push(to_prop(tag));
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct ToolAttrs {
    map: FxIndexMap<PrimaryKey, Properties>,
    /// State of safety tags shows if thet are discharged.
    tagged: TagState,
}

impl ToolAttrs {
    pub fn new(data: &[Data]) -> Self {
        Self {
            map: data
                .iter()
                .filter(|d| !d.func.tool_attrs.is_empty())
                .map(|d| {
                    let mut props = Properties::default();
                    d.func.tool_attrs.iter().for_each(|s| props.push_attr(s));
                    (d.hash, props)
                })
                .collect(),
            tagged: Default::default(),
        }
    }

    pub fn get_tags(&mut self, def_id: DefId, tcx: TyCtxt) -> Option<&mut TagState> {
        let key = PrimaryKey::new(def_id, tcx);
        self.get_tags_via_key(key)
    }

    fn get_tags_via_key(&mut self, key: PrimaryKey) -> Option<&mut TagState> {
        let props = self.map.get(&key)?;
        self.tagged.refresh(props);
        Some(&mut self.tagged)
    }
}

#[derive(Clone)]
pub struct Property {
    // SP name. This represents a unique property, so spec is not involved
    // when Self type is implemented basic traits.
    name: Box<str>,
    spec: Option<&'static Tag>,
}

impl std::hash::Hash for Property {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Ord for Property {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Property {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Property {}

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt(&self.name, f)
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_with_args(&self) -> Cow<'_, str> {
        if let Some(tag) = &self.spec
            && !tag.args.is_empty()
        {
            return format!("{}({})", self.name, tag.args.join(", ")).into();
        }
        self.name().into()
    }

    pub fn info(&self) -> Cow<'static, str> {
        const SP_DESC: &str = "This SP has no description.";

        if let Some(tag) = self.spec {
            return match (&tag.desc, &tag.url) {
                (Some(desc), None) => format!("{desc}").into(),
                (Some(desc), Some(url)) => format!("{desc}\n See {url}",).into(),
                (None, None) => SP_DESC.into(),
                (None, Some(url)) => format!("See {url}").into(),
            };
        }
        SP_DESC.into()
    }
}

fn push_properties(s: &str, v: &mut Vec<Property>) {
    let properties = &*parse_attr_and_get_properties(s);
    let cap = properties.iter().map(|prop| prop.tags.len()).sum();
    v.reserve(cap);
    for property in properties {
        for tag in &property.tags {
            v.push(to_prop(tag));
        }
    }
}

fn to_prop(sp: &SP) -> Property {
    Property { name: sp.tag.name().into(), spec: sp.tag.get_spec() }
}

pub fn tool_attr_on_hir<'tcx>(
    hir_id: HirId,
    tcx: TyCtxt<'tcx>,
) -> impl 'tcx + Iterator<Item = String> {
    get_attrs(tcx, hir_id).filter_map(move |attr| opt_attribute_to_string(tcx, attr))
}
