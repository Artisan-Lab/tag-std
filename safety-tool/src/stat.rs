use camino::Utf8PathBuf;
use safety_parser::{
    configuration::Cache,
    safety::{PropertiesAndReason, Property},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub struct Stat {
    #[serde(rename = "crate")]
    pub krate: Krate,
    pub specs: Cache,
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

#[derive(Debug, Deserialize, Serialize)]
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
