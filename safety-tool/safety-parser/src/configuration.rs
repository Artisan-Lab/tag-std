//! Property definition through config file.
use indexmap::IndexMap;
use serde::Deserialize;
use std::{
    env::{self, var},
    fs,
    path::Path,
    sync::LazyLock,
};

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

/// Single toml config file path.
pub const ENV_SP_FILE: &str = "SP_FILE";
/// Folder where all toml files are searched.
pub const ENV_SP_DIR: &str = "SP_DIR";
/// SP file to crate being compiled.
pub const LOCAL_SP_FILE: &str = "safety-tags.toml";
/// SP folder to crate being compiled.
pub const LOCAL_SP_DIR: &str = "safety-tags";

/// If ENV_SP_DIR or ENV_SP_DIR is provided, check tag and emit `#[doc]` for each tag.
/// If neither is provided, do nothing.
pub fn config_exists() -> bool {
    static EMIT: LazyLock<bool> = LazyLock::new(|| {
        crate_sp_paths().is_some() || var(ENV_SP_FILE).is_ok() || var(ENV_SP_DIR).is_ok()
    });
    *EMIT
}

/// Paths to toml config.
///
/// First, search `safety-tags.toml` or `safety-tags` folder
/// in the crate being compiled:
/// * `CARGO_MANIFEST_DIR/safety-tags.toml`
/// * `CARGO_MANIFEST_DIR/safety-tags/`
/// * if both exist, only respect safety-tags.toml
///
/// If no toml found, pass one of these env vars:
/// * if `SP_FILE` is specified, use that toml path
/// * if `SP_DIR` is specified, use that path to find toml files
/// * if both are given, only respect `SP_FILE`
pub fn toml_file_paths() -> Vec<String> {
    if let Some(paths) = crate_sp_paths() {
        paths
    } else if let Ok(file) = env::var(ENV_SP_FILE) {
        vec![file]
    } else if let Ok(dir) = env::var(ENV_SP_DIR) {
        list_toml_files(&dir)
    } else {
        panic!("Environment variable `SP_FILE` or `SP_DIR` should be specified.");
    }
}

fn list_toml_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("Failed to read {dir} folder:\n{e}")) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map(|ext| ext == "toml").unwrap_or(false) {
            files.push(path.into_os_string().into_string().unwrap());
        }
    }
    files
}

/// Search in the crate being compiled, i.e. `CARGO_MANIFEST_DIR/safety-tags.toml`
/// or `CARGO_MANIFEST_DIR/safety-tags/`.
pub fn crate_sp_paths() -> Option<Vec<String>> {
    if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") {
        let dir = Path::new(&*dir);
        let sp_file = dir.join(LOCAL_SP_FILE);
        let sp_dir = dir.join(LOCAL_SP_DIR);
        // eprintln!(
        //     "Try to read safety-tags.toml or safety-tags folder \
        //      from current crate folder: {dir:?}"
        // );
        if sp_file.exists() {
            return Some(vec![sp_file.to_str()?.to_owned()]);
        } else if sp_dir.exists() {
            return Some(list_toml_files(sp_dir.to_str()?));
        }
    }
    None
}

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

    let configs: Vec<_> = toml_file_paths()
        .into_iter()
        .map(|f| (Configuration::read_toml(&f), f.into_boxed_str()))
        .collect();
    let cap = configs.iter().map(|c| c.0.tag.len()).sum();
    cache.map.reserve(cap);

    for (config, path) in configs {
        for (name, tag) in config.tag {
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

pub fn doc_option() -> GenDocOption {
    CACHE.doc
}
