use camino::Utf8PathBuf;
use rustc_hir::{HirId, def_id::DefId};
use rustc_middle::ty::TyCtxt;
use rustc_session::config::CrateType as RawCrateType;
use rustc_span::Span;
use safety_parser::safety::{PropertiesAndReason, parse_attr_and_get_properties};
pub use safety_tool::stat::*;

pub fn new(tcx: TyCtxt) -> Stat {
    Stat {
        krate: new_crate(tcx),
        specs: Specs::new(),
        funcs: Vec::new(),
        metrics: Metrics::default(),
    }
}

fn new_crate(tcx: TyCtxt) -> Krate {
    let local_crate = rustc_hir::def_id::LOCAL_CRATE;

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

fn hir_span(fn_hir_id: HirId, tcx: TyCtxt) -> Span {
    crossfig::switch! {
        crate::std => { tcx.hir_span_with_body(fn_hir_id) }
        _ => {
            match tcx.hir_node(fn_hir_id) {
                rustc_hir::Node::Item(caller) => caller.span,
                rustc_hir::Node::ImplItem(caller) => caller.span,
                rustc_hir::Node::Expr(callee) => callee.span,
                x => unimplemented!("{x:?}"),
            }
        }
    }
}

pub fn new_func(fn_hir_id: HirId, fn_def_id: DefId, tcx: TyCtxt) -> Func {
    let span = hir_span(fn_hir_id, tcx);

    let src_map = tcx.sess.source_map();
    let file_lines = src_map
        .span_to_lines(span)
        .unwrap_or_else(|err| panic!("Failed to know {span:?}:\n{err:?}"));

    Func {
        name: tcx.def_path_str(fn_def_id),
        safe: tcx.fn_sig(fn_def_id).skip_binder().safety().is_safe(),
        tags: Vec::new(),
        path: file_lines.file.name.prefer_local().to_string().into(),
        span: {
            use std::fmt::Write;
            let mut buf = String::new();
            // The line index stars from 0, while the first line in editors start from 1.
            match file_lines.lines.as_slice() {
                [first, .., last] => {
                    _ = write!(&mut buf, "{}:{}", first.line_index + 1, last.line_index + 1)
                }
                [line] => _ = write!(&mut buf, "{}", line.line_index + 1),
                [] => (),
            }
            buf
        },
        unsafe_calls: Vec::new(),
    }
}

pub fn new_caller(fn_hir_id: HirId, tcx: TyCtxt, attrs: &[String]) -> Func {
    // The caller's hir node owner is itself.
    let fn_def_id = fn_hir_id.owner.to_def_id();
    let mut func = new_func(fn_hir_id, fn_def_id, tcx);

    for attr in attrs {
        let props = parse_attr_and_get_properties(attr);
        push_tag(props, &mut func.tags);
    }

    func
}

/// Split a list of PropertiesAndReason into Tags.
pub fn push_tag(props: impl IntoIterator<Item = PropertiesAndReason>, tags: &mut Vec<Tag>) {
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

pub fn new_callee(fn_hir_id: HirId, fn_def_id: DefId, tcx: TyCtxt, tags: Vec<Tag>) -> Func {
    let mut func = new_func(fn_hir_id, fn_def_id, tcx);
    func.tags = tags;
    func
}
