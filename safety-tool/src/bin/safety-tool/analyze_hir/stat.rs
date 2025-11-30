use camino::Utf8PathBuf;
use rustc_hir::def_id::CrateNum;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::CrateType as RawCrateType;
use safety_parser::safety::parse_attr_and_get_properties;
use safety_tool::stat::*;

pub fn new(tcx: TyCtxt) -> Stat {
    Stat {
        krate: new_crate(tcx),
        specs: safety_parser::configuration::CACHE.clone(),
        funcs: Vec::new(),
        metrics: Metrics { coverage_rate: 0 },
    }
}

fn new_crate(tcx: TyCtxt) -> Krate {
    let local_crate = CrateNum::ZERO;

    let name = tcx.crate_name(local_crate).to_string();
    let path = {
        let path = || Utf8PathBuf::from(tcx.sess.io.input.source_name().prefer_local().to_string());
        path().canonicalize_utf8().unwrap_or_else(|_| path())
    };
    let typ = crate_type(tcx.crate_types());
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();

    Krate { name, path, typ, version }
}

fn crate_type(v: &[RawCrateType]) -> CrateType {
    if v.contains(&RawCrateType::Executable) {
        CrateType::Bin
    } else {
        assert!(!v.is_empty(), "There is no crate type available.");
        CrateType::Lib
    }
}

pub fn new_caller(f: &super::HirFn, tcx: TyCtxt, attrs: &[String]) -> Func {
    let span = tcx.hir_span_with_body(f.hir_id);
    let src_map = tcx.sess.source_map();
    let file_lines = src_map
        .span_to_lines(span)
        .unwrap_or_else(|err| panic!("Failed to know {span:?}:\n{err:?}"));

    let mut tags = Vec::new();
    for attr in attrs {
        let props = parse_attr_and_get_properties(attr);
        for prop in props {
            for tag in prop.tags {
                if let Some(v_sp) = tag.args_in_any_tag() {
                    let ele = safety_tool::stat::Tag::requires_any(v_sp);
                    tags.push(ele);
                } else {
                    let ele = safety_tool::stat::Tag::requires_vanilla(tag);
                    tags.push(ele);
                }
            }
        }
    }

    Func {
        name: tcx.def_path_str(f.local),
        tags,
        path: file_lines.file.name.prefer_local().to_string().into(),
        span: {
            use std::fmt::Write;
            let mut buf = String::new();
            match file_lines.lines.as_slice() {
                [first, .., last] => {
                    _ = write!(&mut buf, "{}:{}", first.line_index, last.line_index)
                }
                [line] => _ = write!(&mut buf, "{}", line.line_index),
                [] => (),
            }
            buf
        },
        unsafe_calls: Vec::new(),
    }
}

pub fn update_unsafe_calls(func: &mut Func, tcx: TyCtxt) {}
