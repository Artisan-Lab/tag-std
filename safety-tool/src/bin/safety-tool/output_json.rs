use crate::{CrateDef, internal, rustc_public};
use indexmap::IndexMap;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::CrateType;
use safety_parser::{
    configuration,
    json::{Ouput, OutputProperties},
    safety::parse_attr_and_get_properties,
};
use std::{env, fs, path::PathBuf};

/// Emit annotatation to `$UPG_DIR/_tags/$crate.json`.
pub fn run(tcx: TyCtxt) {
    // Make sure UPG_DIR is created before this.
    let dir = env::var("UPG_DIR").unwrap();
    let dir = PathBuf::from(dir).canonicalize().unwrap();

    let local_crate = rustc_public::local_crate();
    let crate_name = local_crate.name.as_str();

    let fn_defs = local_crate.fn_defs();
    let mut safety_tags =
        IndexMap::<String, Vec<OutputProperties>>::with_capacity(fn_defs.len() / 3);

    for fn_def in fn_defs {
        let fn_name = format!("{crate_name}::{}", tcx.def_path_str(internal(tcx, fn_def.def_id())));
        let mut tags = Vec::new();
        crossfig::switch! {
            crate::asterinas => { let attrs = fn_def.all_attrs(); }
            _ => { let attrs = fn_def.all_tool_attrs(); }
        }
        for attr in attrs {
            // The attribute prettified like "#[rapx::requires(ValidPtr(v), InitializedInLen(l))]\n",
            // even though the source code is not formatted. So we can rely on the prefix.
            let attr = attr.as_str();
            if attr.starts_with("#[rapx::") {
                let v_sp = parse_attr_and_get_properties(attr);
                tags.extend(v_sp.into_iter().map(OutputProperties::new));
            }
        }
        if !tags.is_empty() {
            safety_tags.insert(fn_name, tags);
        }
    }

    if safety_tags.is_empty() {
        return;
    }

    // Write JSON to `$UPG_DIR/_tags/$crate_name`.
    let dir_rapx = dir.join("_tags");
    _ = fs::create_dir(&dir_rapx);
    let file_name = dir_rapx.join({
        let types = tcx.crate_types();
        let is_bin = types.contains(&CrateType::Executable);
        // If a project has both lib and bin, append `-bin` for the latter.
        if is_bin { format!("{crate_name}-bin.json") } else { format!("{crate_name}.json") }
    });
    let file = fs::File::create(file_name).unwrap();
    let output = Ouput {
        v_fn: safety_tags,
        spec: {
            let mut spec = configuration::CACHE.map.clone();
            // Clear src for space.
            spec.values_mut().for_each(|v| v.src = Box::default());
            spec
        },
    };
    serde_json::to_writer_pretty(file, &output).unwrap();
}
