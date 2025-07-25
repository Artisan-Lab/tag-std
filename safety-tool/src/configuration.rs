//! Property definition through config file.
use indexmap::IndexMap;
use serde::Deserialize;

pub type Str = Box<str>;
pub type OptStr = Option<Box<str>>;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub package: Package,
    pub tag: IndexMap<Str, Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: Box<str>,
    pub version: OptStr,
    pub crate_name: OptStr,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub args: Box<[Str]>,
    pub desc: OptStr,
    pub expr: OptStr,
    pub url: OptStr,
}
