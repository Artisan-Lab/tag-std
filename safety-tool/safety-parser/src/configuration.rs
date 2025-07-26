//! Property definition through config file.
use indexmap::IndexMap;
use serde::Deserialize;
use std::{env, fs, sync::LazyLock};

pub type Str = Box<str>;
pub type OptStr = Option<Box<str>>;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub package: Package,
    pub tag: IndexMap<Str, Tag>,
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
    pub name: Box<str>,
    pub version: OptStr,
    pub crate_name: OptStr,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
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

pub const DEFAULT_TYPE: &str = "precond";

/// Paths to toml config. Pass one of these env vars:
/// * if `SP_FILE` is specified, use that toml path
/// * if `SP_DIR` is specified, use that path to find toml files
/// * if both are given, only respect `SP_FILE`
fn toml_file_paths() -> Vec<String> {
    if let Ok(file) = env::var("SP_FILE") {
        vec![file]
    } else if let Ok(dir) = env::var("SP_DIR") {
        let mut files = Vec::new();
        for entry in
            fs::read_dir(&dir).unwrap_or_else(|e| panic!("Failed to read {dir} folder:\n{e}"))
        {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map(|ext| ext == "toml").unwrap_or(false) {
                files.push(path.into_os_string().into_string().unwrap());
            }
        }
        files
    } else {
        panic!("Environment variable `SP_FILE` or `SP_DIR` should be specified.");
    }
}

/// Data shared in `#[safety]` proc macro.
#[derive(Debug)]
pub struct Key {
    /// Tag defined in config file.
    pub tag: Tag,
    /// File path where the tag is defined: we must be sure each tag only
    /// derives from single file path.
    pub src: Str,
}

pub static TAGS: LazyLock<IndexMap<Str, Key>> = LazyLock::new(|| {
    let configs: Vec<_> =
        toml_file_paths().into_iter().map(|f| (Configuration::read_toml(&f), f)).collect();
    let cap = configs.iter().map(|c| c.0.tag.len()).sum();
    let mut map = IndexMap::with_capacity(cap);
    for (config, path) in configs {
        for (name, tag) in config.tag {
            if let Some(old) = map.get(&name) {
                panic!("Tag {name:?} has been defined: {old:?}");
            }
            _ = map.insert(name, Key { tag, src: (&*path).into() });
        }
    }
    map.sort_unstable_keys();
    println!("Got {} tags.", map.len());
    map
});

pub fn get_tag(name: &str) -> &'static Tag {
    &TAGS.get(name).unwrap_or_else(|| panic!("Tag {name:?} is not defined")).tag
}

#[test]
fn string_interpolation() {
    #[derive(serde::Serialize)]
    struct Val {
        a: u8,
        b: &'static str,
    }
    let s = "{a}, {b}";
    let mut template = tinytemplate::TinyTemplate::new();
    template.add_template("", s).unwrap();
    println!("rendered: {}", template.render("", &Val { a: 123, b: "hi" }).unwrap());
}
