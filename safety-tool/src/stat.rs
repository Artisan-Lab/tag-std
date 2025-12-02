use camino::Utf8PathBuf;
use indexmap::IndexMap;
use safety_parser::{
    configuration::{CACHE, GenDocOption, Key},
    safety::{PropertiesAndReason, Property},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub struct Stat {
    #[serde(rename = "crate")]
    pub krate: Krate,
    pub specs: Spec,
    pub funcs: Vec<Func>,
    pub metrics: Metrics,
}

impl fmt::Debug for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stat")
            .field("krate", &self.krate)
            // .field("specs", &self.specs)
            .field("funcs", &self.funcs)
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl Stat {
    /// This should be called after all funcs and tags are collected.
    pub fn update_metrics(&mut self) {
        let specs = &mut self.specs;

        for func in &self.funcs {
            for tag in &func.tags {
                let predicate = tag.predicate;
                // Increment type and predicate usage count on each tag.
                match &tag.tag {
                    TagType::Vanilla(prop) => {
                        let name = prop.tag.name();
                        let usage = specs.get_usage_mut(name);
                        usage.increment_type_vanilla();
                        usage.increment_predicate(predicate);
                    }
                    TagType::Any(props) => {
                        for prop in props {
                            for tag in &prop.tags {
                                let name = tag.tag.name();
                                let usage = specs.get_usage_mut(name);
                                usage.increment_type_any();
                                usage.increment_predicate(predicate);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Krate {
    pub name: String,
    pub path: Utf8PathBuf,
    #[serde(rename = "type")]
    pub typ: CrateType,
    pub version: String,
}

/// `TyCtxt::crate_type` returns a list:
/// * Executable is compiled alone: a crate won't be a bin and lib at the same time
/// * multiple types of library is possible, when `[lib] crate-type = [ ... ]` in
///   Cargo.toml or multiple `--crate-type` are specified.
#[derive(Debug, Deserialize, Serialize)]
pub enum CrateType {
    /// i.e. CrateType::Executable in rustc_session
    Bin,
    /// i.e. CrateType::*lib in rustc_session
    Lib,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub map: IndexMap<Box<str>, SpecItem>,
    pub doc: GenDocOption,
}

impl Spec {
    /// Initialize spec from Cache with zero usage metrics.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let cache = &*CACHE;
        let iter = cache.map.iter();
        let map = iter
            .map(|(key, item)| {
                (key.clone(), SpecItem { item: item.clone(), usage: Usage::default() })
            })
            .collect();
        Spec { map, doc: cache.doc }
    }

    pub fn get_usage_mut(&mut self, tag_name: &str) -> &mut Usage {
        let val = self
            .map
            .get_mut(tag_name)
            .unwrap_or_else(|| panic!("{tag_name} is not in specification."));
        &mut val.usage
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecItem {
    item: Key,
    usage: Usage,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Usage {
    types: IndexMap<TagTypeUsage, u16>,
    predicates: IndexMap<Predicate, u16>,
}

impl Usage {
    pub fn increment_type_vanilla(&mut self) {
        self.types.entry(TagTypeUsage::Vanilla).and_modify(|c| *c += 1).or_insert(0);
    }

    pub fn increment_type_any(&mut self) {
        self.types.entry(TagTypeUsage::Any).and_modify(|c| *c += 1).or_insert(0);
    }

    pub fn increment_predicate(&mut self, predicate: Predicate) {
        self.predicates.entry(predicate).and_modify(|c| *c += 1).or_insert(0);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Func {
    pub name: String,
    pub tags: Vec<Tag>,
    pub path: Utf8PathBuf,
    pub span: String,
    pub unsafe_calls: Vec<Func>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub predicate: Predicate,
    pub tag: TagType,
    pub doc: Option<Box<str>>,
}

impl Tag {
    pub fn requires_vanilla(prop: Property) -> Tag {
        Tag { predicate: Predicate::Requires, tag: TagType::Vanilla(prop), doc: None }
    }

    pub fn requires_any(props: Vec<PropertiesAndReason>) -> Tag {
        Tag { predicate: Predicate::Requires, tag: TagType::Any(props), doc: None }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TagType {
    /// Single tag.
    Vanilla(Property),
    /// A set of tags in built-in `any` tag
    Any(Vec<PropertiesAndReason>),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum TagTypeUsage {
    Vanilla,
    Any,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Predicate {
    Requires,
    Checked,
    Delegated,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Callee {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metrics {
    pub coverage_rate: u8,
}
