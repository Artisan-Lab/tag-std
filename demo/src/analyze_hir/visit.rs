use std::ops::Range;

use super::db::{Property, ToolAttrs};
use annotate_snippets::*;
use itertools::Itertools;
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::{
    def::{DefKind, Res},
    def_id::DefId,
    intravisit::*,
    *,
};
use rustc_middle::ty::{TyCtxt, TypeckResults};
use rustc_span::{Span, source_map::SourceMap};

#[derive(Debug)]
pub struct Call {
    /// function use id
    pub hir_id: HirId,
    /// function def id
    pub def_id: DefId,
}

impl Call {
    pub fn check_tool_attrs(
        &self,
        fn_hir_id: HirId,
        tcx: TyCtxt,
        src_map: &SourceMap,
        tool_attrs: &mut ToolAttrs,
    ) {
        let Some(tags_state) = tool_attrs.get_tags(self.def_id, tcx) else {
            // No tool attrs to be checked.
            return;
        };
        dbg!(&tags_state);

        let mut print = |hir_id: HirId| {
            eprintln!("hir_id={hir_id:?} fn_hir_id={fn_hir_id:?}");

            let properties = Property::new_with_hir_id(hir_id, tcx);

            for tag in &properties {
                let Some(state) = tags_state.get_mut(tag) else {
                    panic!("tag {tag:?} doesn't belong to tags {tags_state:?}")
                };
                assert!(!*state, "{tag:?} has already been discharged");
                *state = true;
            }
            let is_empty = properties.is_empty();
            if !is_empty {
                // only checks if Safety tags exist
                check_tag_state(tcx, src_map, tags_state, hir_id);
            }
            is_empty
        };
        print(self.hir_id);

        for parent in tcx.hir_parent_id_iter(self.hir_id) {
            let empty = print(parent);
            // Stop at first tool attrs or the function item.
            // For a function inside a nested module, hir_parent_id_iter
            // will pop up to the crate root, thus it's necessary to
            // stop when reaching the fn item.
            if !empty || parent == fn_hir_id {
                break;
            }
        }

        // make sure Safety tags are all discharged
        check_tag_state(tcx, src_map, tags_state, self.hir_id);
    }
}

fn check_tag_state(
    tcx: TyCtxt,
    src_map: &SourceMap,
    tags_state: &mut FxIndexMap<Property, bool>,
    hir_id: HirId,
) {
    let mut n = 0;
    let undischarged = tags_state
        .iter()
        .filter(|(_, state)| !*state)
        .map(|(tag, _)| {
            n += 1;
            tag.as_str()
        })
        .format_with(", ", |tag, f| f(&format_args!("`{tag}`")))
        .to_string();
    if n != 0 {
        let span_body = tcx.source_span(hir_id.owner);
        let is = if n == 1 { "is" } else { "are" };
        let title = format!("{undischarged} {is} not discharged");
        let span_node = tcx.hir_span(hir_id);
        let anno =
            Level::Error.span(anno_span(span_body, span_node)).label("For this unsafe call.");
        gen_diagnosis(span_body, src_map, &title, anno);
    }
}

fn gen_diagnosis(span_body: Span, src_map: &SourceMap, title: &str, anno: Annotation) {
    let src_body = src_map.span_to_snippet(span_body).unwrap();
    dbg!(&src_body);
    let file_and_line = src_map.lookup_line(span_body.lo()).unwrap();
    let line_start = file_and_line.line + 1; // adjust to starting from 1
    let origin = file_and_line.sf.name.prefer_local().to_string_lossy();
    let snippet = Snippet::source(&src_body).line_start(line_start).origin(&origin);

    let msg = Level::Error.title(title).snippet(snippet.annotation(anno));
    eprintln!("{}", Renderer::styled().render(msg));
    std::process::abort()
}

fn anno_span(span_body: Span, span_node: Span) -> Range<usize> {
    let body_lo = span_body.lo().0;
    let node_lo = span_node.lo().0;
    let node_hi = span_node.hi().0;
    (node_lo - body_lo) as usize..(node_hi - body_lo) as usize
}

pub struct Calls<'tcx> {
    tcx: TyCtxt<'tcx>,
    tyck: &'tcx TypeckResults<'tcx>,
    calls: Vec<Call>,
}

impl<'tcx> Visitor<'tcx> for Calls<'tcx> {
    type MaybeTyCtxt = TyCtxt<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
    type Result = ();

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) -> Self::Result {
        let hir_id = ex.hir_id;
        match ex.kind {
            ExprKind::Path(QPath::Resolved(_opt_ty, path)) => {
                if let Res::Def(DefKind::Fn, def_id) = path.res {
                    self.calls.push(Call { hir_id, def_id });
                }
            }
            // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/enum.ExprKind.html#variant.MethodCall
            //
            // res is Err, and  must resolve to get def_id, thus need to call
            // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TypeckResults.html#method.type_dependent_def_id
            ExprKind::MethodCall(..) => {
                if let Some(def_id) = self.tyck.type_dependent_def_id(hir_id) {
                    self.calls.push(Call { hir_id, def_id });
                } else {
                    eprintln!("Unable to resolve DefId from {:?}", ex.kind);
                }
            }
            _ => (),
        }
        walk_expr(self, ex)
    }
}

pub fn get_calls<'tcx>(
    tcx: TyCtxt<'tcx>,
    expr: &'tcx Expr<'tcx>,
    tyck: &'tcx TypeckResults<'tcx>,
) -> Calls<'tcx> {
    let mut calls = Calls { tcx, tyck, calls: Vec::new() };
    walk_expr(&mut calls, expr);
    calls
}

impl Calls<'_> {
    pub fn get_unsafe_calls(&self) -> Vec<&Call> {
        self.calls
            .iter()
            .filter(|call| self.tcx.fn_sig(call.def_id).skip_binder().safety().is_unsafe())
            .collect()
    }
}
