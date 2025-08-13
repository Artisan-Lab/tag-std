use crate::analyze_hir::db::TagState;

use super::{
    db::{Property, ToolAttrs},
    diagnostic::Diagnostic,
};
use annotate_snippets::*;
use rustc_hir::{
    def::{DefKind, Res},
    def_id::DefId,
    intravisit::*,
    *,
};
use rustc_middle::ty::{TyCtxt, TypeckResults};
use rustc_span::{Span, source_map::SourceMap};
use std::ops::Range;

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
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(tag_state) = tool_attrs.get_tags(self.def_id, tcx) else {
            // No tool attrs to be checked.
            return;
        };
        debug!(?tag_state);

        let mut check = |hir_id: HirId| {
            debug!(?hir_id, ?fn_hir_id);

            let properties = Property::new_with_hir_id(hir_id, tcx);

            let is_empty = properties.is_empty();
            if !is_empty {
                for tag in &properties {
                    tag_state.discharge(tag);
                }
                // only checks if Safety tags exist
                check_tag_state(tcx, src_map, tag_state, hir_id, diagnostics);
            }
            is_empty
        };
        check(self.hir_id);

        crossfig::switch! {
            crate::asterinas => { let parent_hirs = tcx.hir().parent_id_iter(self.hir_id); }
            _ => { let parent_hirs = tcx.hir_parent_id_iter(self.hir_id); }
        }

        for parent in parent_hirs {
            let empty = check(parent);
            // Stop at first tool attrs or the function item.
            // For a function inside a nested module, hir_parent_id_iter
            // will pop up to the crate root, thus it's necessary to
            // stop when reaching the fn item.
            if !empty || parent == fn_hir_id {
                break;
            }
        }

        // make sure Safety tags are all discharged
        check_tag_state(tcx, src_map, tag_state, self.hir_id, diagnostics);
    }
}

fn check_tag_state(
    tcx: TyCtxt,
    src_map: &SourceMap,
    tag_state: &mut TagState,
    hir_id: HirId,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let undischarged = tag_state.undischarged();
    let len = undischarged.len();
    if len != 0 {
        let undischarged_str = undischarged.join("\n");
        let span_node = hir_span(hir_id, tcx);
        let span_body = tcx.source_span(hir_id.owner);
        let newline = if len == 1 { " " } else { "\n" };
        let plural = if undischarged_str.matches(',').count() == 0 { "Tag is" } else { "Tags are" };
        let title = format!("{plural} not discharged:{newline}{undischarged_str}");

        let anno_call =
            Level::Error.span(anno_span(span_body, span_node)).label("For this unsafe call.");
        let render = gen_diagnosis_for_a_func(span_body, src_map, &title, anno_call);
        diagnostics.push(Diagnostic::missing_discharges(render));
    }
}

fn hir_span(hir_id: HirId, tcx: TyCtxt) -> Span {
    crossfig::switch! {
        crate::std => { tcx.hir_span(hir_id) }
        _ => { tcx.hir().span(hir_id) }
    }
}

fn gen_diagnosis_for_a_func(
    span_body: Span,
    src_map: &SourceMap,
    title: &str,
    anno: Annotation,
) -> Box<str> {
    let src_body = src_map.span_to_snippet(span_body).unwrap();
    let file_and_line = src_map.lookup_line(span_body.lo()).unwrap();
    let line_start = file_and_line.line + 1; // adjust to starting from 1
    let origin = file_and_line.sf.name.prefer_local().to_string_lossy();
    let snippet = Snippet::source(&src_body).line_start(line_start).origin(&origin).fold(true);

    let msg = Level::Error.title(title).snippet(snippet.annotation(anno));
    Renderer::styled().render(msg).to_string().into()
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

crossfig::switch! {
    crate::asterinas => {
        impl<'tcx> Visitor<'tcx> for Calls<'tcx> {
            type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
            type Result = ();

            fn nested_visit_map(&mut self) -> Self::Map {
                self.tcx.hir()
            }

            fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) -> Self::Result {
                self.inner_visit_expr(ex)
            }
        }
    }
    _ => {
        impl<'tcx> Visitor<'tcx> for Calls<'tcx> {
            type MaybeTyCtxt = TyCtxt<'tcx>;
            type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
            type Result = ();

            fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
                self.tcx
            }

            fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) -> Self::Result {
                self.inner_visit_expr(ex)
            }
        }
    }
}

impl<'tcx> Calls<'tcx> {
    fn inner_visit_expr(&mut self, ex: &'tcx Expr<'tcx>) {
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

    pub fn get_unsafe_calls(&self) -> Vec<&Call> {
        self.calls
            .iter()
            .filter(|call| self.tcx.fn_sig(call.def_id).skip_binder().safety().is_unsafe())
            .collect()
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
