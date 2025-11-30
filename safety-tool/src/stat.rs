use camino::Utf8PathBuf;
use safety_parser::safety::PropertiesAndReason;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Stat {
    #[serde(rename = "crate")]
    pub krate: Krate,
    pub specs: Vec<Spec>,
    pub funcs: Vec<Func>,
    pub metrics: Metrics,
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
pub struct Spec {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Func {
    pub name: String,
    pub tags: Tags,
    pub path: Utf8PathBuf,
    pub span: String,
    pub unsafe_calls: Vec<Func>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tags {
    pub predicate: Predicate,
    pub tag: PropertiesAndReason,
    pub doc: Option<String>,
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
