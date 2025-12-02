use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use safety_parser::{
    configuration::{CACHE, GenDocOption, Key},
    safety::{PropertiesAndReason, Property},
};
use serde::{Deserialize, Serialize};
use std::{env, fmt, fs};

#[derive(Deserialize, Serialize)]
pub struct Stat {
    #[serde(rename = "crate")]
    pub krate: Krate,
    pub specs: Specs,
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
        // Update specs through callers and callees.
        for func in &self.funcs {
            func.update_specs(&mut self.specs);
            for callee in &func.unsafe_calls {
                callee.update_specs(&mut self.specs);
            }
        }

        // Update metrics.
        self.specs.update_metrics(&mut self.metrics);
    }

    /// This method should be called after self is fully computed.
    /// The write happens when at least 1 tag is used, and the
    /// env var `SP_OUT_DIR` is set.
    /// If the dir is not present, it'll be created.
    pub fn write_to_file(&self) {
        if self.metrics.used_tags != 0
            && let Some(path) = self.krate.output_file_path()
            && let Ok(file) = fs::File::create(path)
        {
            _ = serde_json::to_writer_pretty(file, self);
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

/// Read the env var `SP_OUT_DIR`. If the path doesn't exist,
/// crrate it non-recursively.
fn out_dir() -> Option<Utf8PathBuf> {
    let out_dir = "SP_OUT_DIR";
    let out_dir = env::var(out_dir).ok()?;
    let out_dir = Utf8Path::new(&out_dir);
    if !out_dir.exists() {
        fs::create_dir(out_dir).ok()?;
    }
    out_dir.canonicalize_utf8().ok()
}

impl Krate {
    fn output_file_path(&self) -> Option<Utf8PathBuf> {
        let dir = out_dir()?;
        let prefix = match self.typ {
            CrateType::Bin => "bin-",
            CrateType::Lib => "",
        };
        let file = format!("{prefix}{}.json", self.name);
        Some(dir.join(file))
    }
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
pub struct Specs {
    pub map: IndexMap<Box<str>, SpecItem>,
    pub doc: GenDocOption,
}

impl Specs {
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
        Specs { map, doc: cache.doc }
    }

    fn get_usage_mut(&mut self, tag_name: &str) -> &mut Usage {
        let val = self
            .map
            .get_mut(tag_name)
            .unwrap_or_else(|| panic!("{tag_name} is not in specification."));
        &mut val.usage
    }

    /// This method should be called after all tag usage is collected.
    fn update_metrics(&self, metrics: &mut Metrics) {
        metrics.total_tags = self.map.len().try_into().unwrap();
        for (name, item) in &self.map {
            if item.usage.is_unused() {
                // This tag is unused.
                metrics.unused.push(name.clone());
            } else {
                let coverage = MetricsCoverage::from_usage(&item.usage);
                metrics.used_tags += 1;
                metrics.coverage.merge(&coverage);
                let previous = metrics.used.insert(name.clone(), coverage);
                assert!(
                    previous.is_none(),
                    "{name}'s coverage has been inserted before: {previous:?}"
                );
            }
        }
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
    fn increment_type_vanilla(&mut self) {
        self.types.entry(TagTypeUsage::Vanilla).and_modify(|c| *c += 1).or_insert(1);
    }

    fn increment_type_any(&mut self) {
        self.types.entry(TagTypeUsage::Any).and_modify(|c| *c += 1).or_insert(1);
    }

    fn increment_predicate(&mut self, predicate: Predicate) {
        self.predicates.entry(predicate).and_modify(|c| *c += 1).or_insert(1);
    }

    fn is_unused(&self) -> bool {
        self.types.is_empty() && self.predicates.is_empty()
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

impl Func {
    fn update_specs(&self, specs: &mut Specs) {
        for tag in &self.tags {
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Metrics {
    /// Amount of tags in specification.
    pub total_tags: u16,
    /// How many tags are acutally used?
    pub used_tags: u16,
    /// How many times are these tags used in details?
    pub used: IndexMap<Box<str>, MetricsCoverage>,
    /// How many times are these tags used in general?
    pub coverage: MetricsCoverage,
    /// Unused tag names.
    pub unused: Vec<Box<str>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetricsCoverage {
    /// Sum of the other fields.
    occurence: u16,
    /// How many times does the tag is used individually?
    as_vanilla: u16,
    /// How many times does the tag is used in `any` tag?
    in_any: u16,
    /// How many times does the tag is used in `requires` predicate?
    requires: u16,
    /// How many times does the tag is used in `checked` predicate?
    checked: u16,
    /// How many times does the tag is used in `delegated` predicate?
    delegated: u16,
}

impl MetricsCoverage {
    fn from_usage(usage: &Usage) -> Self {
        let mut coverage = MetricsCoverage::default();
        let Self { occurence, as_vanilla, in_any, requires, checked, delegated } = &mut coverage;

        for (typ, count) in &usage.types {
            match typ {
                TagTypeUsage::Vanilla => *as_vanilla += count,
                TagTypeUsage::Any => *in_any += count,
            }
        }
        for (predicate, count) in &usage.predicates {
            match predicate {
                Predicate::Requires => *requires += count,
                Predicate::Checked => *checked += count,
                Predicate::Delegated => *delegated += count,
            }
        }
        *occurence = *as_vanilla + *in_any + *requires + *checked + *delegated;
        coverage
    }

    /// Add the number in detail to self.
    fn merge(&mut self, detail: &Self) {
        self.occurence += detail.occurence;
        self.as_vanilla += detail.as_vanilla;
        self.in_any += detail.in_any;
        self.requires += detail.requires;
        self.checked += detail.checked;
        self.delegated += detail.delegated;
    }
}
