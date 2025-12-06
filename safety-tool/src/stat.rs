use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use itertools::Itertools;
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
            func.update_metrics(&mut self.metrics.funcs);
        }

        // Merge metrics of funcs.
        self.metrics.funcs.merge();

        // Sort and deduplicate functions for tags.
        for item in self.specs.map.values_mut() {
            item.usage.functions.sort_unstable();
            item.usage.functions.dedup();
        }

        // Update metrics as per specs.
        self.specs.update_metrics(&mut self.metrics);

        // Sort.
        self.sort();
    }

    fn sort(&mut self) {
        // Sort by function name in alphabet order.
        self.funcs.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        // Sort by occurence in descending order first and then by function name in alphabet order.
        self.metrics.used.sort_unstable_by(|a_name, a_cov, b_name, b_cov| {
            (b_cov.occurence, a_name).cmp(&(a_cov.occurence, b_name))
        });
        // Sort by unsafe call counts.
        self.metrics.funcs.safe.unsafe_calls.sort_unstable_keys();
        self.metrics.funcs.r#unsafe.unsafe_calls.sort_unstable_keys();

        // Tags in spec have been in alphabet order, and unused list are in the insertion order.
    }

    /// This method should be called after self is fully computed.
    /// The write happens when at least 1 tag is used, and the
    /// env var `SP_OUT_DIR` is set.
    /// If the dir is not present, it'll be created.
    pub fn write_to_file(&self) {
        if self.metrics.used_tags != 0
            && let Some(path) = self.krate.output_json_file_path()
            && let Ok(file) = fs::File::create(path)
        {
            _ = serde_json::to_writer_pretty(file, self);
            self.write_call_tree();
        }
    }

    pub fn write_call_tree(&self) {
        use std::io::Write;
        let Some(mut file) = self.krate.output_tree_file_path() else { return };
        for func in &self.funcs {
            if func.unsafe_calls.is_empty() && func.has_no_tag() {
                // Skip functions that have no unsafe calls and no tags.
                continue;
            }
            let tree = termtree::Tree::new(func.root_node());
            let leaves = func.unsafe_calls.iter().map(|c| {
                let name = &*c.name;
                if let Some(local_fn) = self.funcs.iter().find(|f| f.name == name) {
                    local_fn.root_node()
                } else {
                    c.name.to_owned()
                }
            });
            let tree = tree.with_leaves(leaves);
            _ = writeln!(&mut file, "{tree}");
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
    fn output_file_path(&self, ext: &str) -> Option<Utf8PathBuf> {
        let dir = out_dir()?;
        let prefix = match self.typ {
            CrateType::Bin => "bin-",
            CrateType::Lib => "",
        };
        let file = format!("{prefix}{}.{ext}", self.name);
        Some(dir.join(file))
    }

    fn output_json_file_path(&self) -> Option<Utf8PathBuf> {
        self.output_file_path("json")
    }

    fn output_tree_file_path(&self) -> Option<fs::File> {
        fs::File::create(self.output_file_path("txt")?).ok()
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

                let occurence = coverage.as_vanilla + coverage.in_any;
                assert_eq!(coverage.occurence, occurence, "{name} has unbalanced occurence");

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
    pub item: Key,
    pub usage: Usage,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Usage {
    pub types: IndexMap<TagTypeUsage, u16>,
    pub predicates: IndexMap<Predicate, u16>,
    pub functions: Vec<Box<str>>,
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

    fn push_function(&mut self, name: &str) {
        self.functions.push(name.into());
    }

    fn is_unused(&self) -> bool {
        self.types.is_empty() && self.predicates.is_empty()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Func {
    pub name: String,
    pub safe: bool,
    pub path: Utf8PathBuf,
    pub span: String,
    pub tags: Vec<Tag>,
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
                    usage.push_function(&self.name);
                }
                TagType::Any(props) => {
                    for prop in props {
                        for tag in &prop.tags {
                            let name = tag.tag.name();
                            let usage = specs.get_usage_mut(name);
                            usage.increment_type_any();
                            usage.increment_predicate(predicate);
                            usage.push_function(&self.name);
                        }
                    }
                }
            }
        }
    }

    fn update_metrics(&self, metrics_funcs: &mut MetricsFunctions) {
        let m = if self.safe { &mut metrics_funcs.safe } else { &mut metrics_funcs.r#unsafe };
        m.total.funcs += 1;

        if !self.tags.is_empty() {
            m.total.funcs_with_tags_declared += 1;
        }
        if self.unsafe_calls.iter().any(|c| !c.tags.is_empty()) {
            m.total.funcs_with_tags_discharged += 1;
        }

        m.total.declared_tags += self.tags.len() as u16;
        m.total.discharged_tags +=
            self.unsafe_calls.iter().map(|c| c.tags.len() as u16).sum::<u16>();

        let unsafe_calls = self.unsafe_calls.len() as u16;
        m.total.unsafe_calls += unsafe_calls;
        m.unsafe_calls.entry(unsafe_calls).and_modify(|c| *c += 1).or_insert(1);
    }

    fn has_no_tag(&self) -> bool {
        // There must be a tag as an element in self.tags. It could possibly be
        // empty for Any TagType, but that's still an `any` tag.
        self.tags.is_empty()
    }

    fn root_node(&self) -> String {
        if self.has_no_tag() {
            self.name.to_owned()
        } else {
            let tags = self.tags.iter().map(|tag| &tag.tag).format_with(", ", |ele, f| f(ele));
            format!("{} {{ {tags} }}", self.name)
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

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagType::Vanilla(prop) => write!(f, "{}", prop.tag.name()),
            TagType::Any(props) => {
                let iter = props.iter().flat_map(|any| any.tags.iter().map(|tag| tag.tag.name()));
                let iter_fmt = iter.format_with(", ", |ele, f| f(&ele));
                write!(f, "{iter_fmt}")
            }
        }
    }
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
    pub funcs: MetricsFunctions,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetricsCoverage {
    /// Sum of the requires, checked, and delegated (or equivalently as_vanilla + in_any).
    pub occurence: u16,
    /// How many times does the tag is used in `requires` predicate?
    pub requires: u16,
    /// How many times does the tag is used in `checked` predicate?
    pub checked: u16,
    /// How many times does the tag is used in `delegated` predicate?
    pub delegated: u16,
    /// How many times does the tag is used individually?
    pub as_vanilla: u16,
    /// How many times does the tag is used in `any` tag?
    pub in_any: u16,
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
        *occurence = *requires + *checked + *delegated;
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetricsFunctions {
    pub total: MetricsFuncsTotal,
    pub safe: MetricsFuncs,
    pub r#unsafe: MetricsFuncs,
}

impl MetricsFunctions {
    fn merge(&mut self) {
        self.total.merge(&self.safe.total);
        self.total.merge(&self.r#unsafe.total);
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetricsFuncs {
    pub total: MetricsFuncsTotal,
    pub unsafe_calls: IndexMap<u16, u16>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MetricsFuncsTotal {
    pub funcs: u16,
    pub funcs_with_tags_declared: u16,
    pub funcs_with_tags_discharged: u16,
    pub declared_tags: u16,
    pub discharged_tags: u16,
    pub unsafe_calls: u16,
}

impl MetricsFuncsTotal {
    fn merge(&mut self, other: &Self) {
        self.funcs += other.funcs;
        self.funcs_with_tags_declared += other.funcs_with_tags_declared;
        self.funcs_with_tags_discharged += other.funcs_with_tags_discharged;
        self.declared_tags += other.declared_tags;
        self.discharged_tags += other.discharged_tags;
        self.unsafe_calls += other.unsafe_calls;
    }
}
