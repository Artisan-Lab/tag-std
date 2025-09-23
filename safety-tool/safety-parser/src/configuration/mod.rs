//! Property definition through config file.
use indexmap::IndexMap;
use serde::Deserialize;
use std::{fs, sync::LazyLock};

pub mod env;

pub type Str = Box<str>;
pub type OptStr = Option<Box<str>>;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub package: Option<Package>,
    pub tag: IndexMap<Str, Tag>,
    #[serde(default)]
    pub doc: GenDocOption,
}

impl Configuration {
    pub fn read_toml(path: &str) -> Self {
        if !fs::exists(path).unwrap() {
            panic!("{path:?} doesn't exist.")
        }
        let text =
            &fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {path}:\n{e}"));
        toml::from_str(text).unwrap_or_else(|e| panic!("Failed to parse {path}:\n{e}"))
    }
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: Str,
    pub version: OptStr,
    pub crate_name: OptStr,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    #[serde(default)]
    pub args: Box<[Str]>,
    pub desc: OptStr,
    pub expr: OptStr,
    #[serde(default = "default_types")]
    pub types: Box<[TagType]>,
    pub url: OptStr,
}

#[derive(Clone, Copy, Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TagType {
    #[default]
    Precond,
    Hazard,
    Option,
}

impl TagType {
    pub fn new(s: &str) -> Self {
        match s {
            "precond" => Self::Precond,
            "hazard" => Self::Hazard,
            "option" => Self::Option,
            _ => panic!("Only support: precond, hazard, and option."),
        }
    }
}

/// If types field doesn't exist, default to Precond.
fn default_types() -> Box<[TagType]> {
    Box::new([TagType::Precond])
}

#[derive(Clone, Copy, Debug, Deserialize, Default)]
pub struct GenDocOption {
    /// Generate `/// Safety` at the beginning.
    #[serde(default)]
    pub heading_safety_title: bool,
    /// Generate `Tag:` before `desc`.
    #[serde(default)]
    pub heading_tag: bool,
}

impl GenDocOption {
    fn merge(&mut self, other: &Self) {
        if other.heading_safety_title {
            self.heading_safety_title = true;
        }
        if other.heading_tag {
            self.heading_tag = true;
        }
    }
}

/// `any` tag is denied in user's spec, and special in doc generation.
pub const ANY: &str = "any";

/// Data shared in `#[safety]` proc macro.
#[derive(Debug)]
struct Key {
    /// Tag defined in config file.
    tag: Tag,
    /// File path where the tag is defined: we must be sure each tag only
    /// derives from single file path.
    #[allow(dead_code)]
    src: Str,
}

#[derive(Default)]
struct Cache {
    /// Defined tags.
    map: IndexMap<Str, Key>,
    /// Merged doc generation options: if any is true, set true.
    doc: GenDocOption,
}

static CACHE: LazyLock<Cache> = LazyLock::new(|| {
    let mut cache = Cache::default();

    let configs: Vec<_> = env::toml_file_paths()
        .into_iter()
        .map(|f| (Configuration::read_toml(&f), f.into_boxed_str()))
        .collect();
    let cap = configs.iter().map(|c| c.0.tag.len()).sum();
    cache.map.reserve(cap);

    for (config, path) in configs {
        for (name, tag) in config.tag {
            if &*name == ANY {
                panic!("`any` is a builtin tag. Please remove it from spec.");
            }
            if let Some(old) = cache.map.get(&name) {
                panic!("Tag {name:?} has been defined: {old:?}");
            }
            _ = cache.map.insert(name, Key { tag, src: path.clone() });
        }
        cache.doc.merge(&config.doc);
    }

    cache.map.sort_unstable_keys();
    eprintln!("Got {} tags.", cache.map.len());
    cache
});

pub fn get_tag(name: &str) -> &'static Tag {
    &CACHE.map.get(name).unwrap_or_else(|| panic!("Tag {name:?} is not defined")).tag
}

pub fn get_tag_opt(name: &str) -> Option<&'static Tag> {
    CACHE.map.get(name).map(|val| &val.tag)
}

pub fn doc_option() -> GenDocOption {
    CACHE.doc
}

pub struct DefinedTag {
    pub name: &'static str,
    pub args: &'static Tag,
}

/// Get all tags defined in all spec TOMLs.
pub fn get_tags() -> Box<[DefinedTag]> {
    CACHE.map.iter().map(|(k, v)| DefinedTag { name: k, args: &v.tag }).collect()
}
