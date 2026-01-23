use crate::{
    configuration,
    safety::{PropertiesAndReason, Property},
};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct Ouput {
    pub v_fn: IndexMap<String, Vec<OutputProperties>>,
    pub spec: IndexMap<Box<str>, configuration::Key>,
}

#[derive(Serialize)]
pub struct OutputProperties {
    pub tags: Vec<OutputProperty>,
    pub desc: Option<Box<str>>,
    pub doc: Box<str>,
}

impl OutputProperties {
    pub fn new(sp: PropertiesAndReason) -> Self {
        let doc = sp.gen_hover_doc();
        let tags = sp.tags.into_vec().into_iter().map(OutputProperty::new).collect();
        OutputProperties { tags, desc: sp.desc, doc }
    }
}

#[derive(Serialize)]
pub struct OutputProperty {
    pub sp: Property,
    pub doc: Box<str>,
}

impl OutputProperty {
    pub fn new(sp: Property) -> Self {
        OutputProperty { doc: sp.gen_doc().unwrap_or_default().into(), sp }
    }
}
