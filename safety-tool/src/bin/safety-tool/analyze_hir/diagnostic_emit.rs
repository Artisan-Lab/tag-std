use super::diagnostic::Diagnostic;
use annotate_snippets::{Level, Renderer, Snippet};
use rustc_hir::HirId;
use rustc_middle::ty::TyCtxt;
use rustc_span::{
    Span,
    source_map::{SourceMap, get_source_map},
};
use std::{ops::Range, sync::Arc};

pub struct EmitDiagnostics<'tcx> {
    tcx: TyCtxt<'tcx>,
    src_map: Arc<SourceMap>,
    diagnostics: Vec<Diagnostic>,
}

impl<'tcx> EmitDiagnostics<'tcx> {
    pub fn new(tcx: TyCtxt) -> EmitDiagnostics {
        EmitDiagnostics {
            tcx,
            src_map: get_source_map().expect("Failed to get source map."),
            diagnostics: Vec::new(),
        }
    }

    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    pub fn take_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    #[must_use]
    pub fn generate(&mut self, hir_id: HirId, title: &str) -> Box<str> {
        let span_node = hir_span(hir_id, self.tcx);
        let span_body = self.tcx.source_span(hir_id.owner);

        // Point out an unsafe call with underlines.
        let anno_call =
            Level::Error.span(anno_span(span_body, span_node)).label("For this unsafe call.");

        let src_body = self.src_map.span_to_snippet(span_body).unwrap();
        let file_and_line = self.src_map.lookup_line(span_body.lo()).unwrap();
        let line_start = file_and_line.line + 1; // adjust to starting from 1
        let origin = file_and_line.sf.name.prefer_local().to_string_lossy();
        let snippet = Snippet::source(&src_body).line_start(line_start).origin(&origin).fold(true);

        // Point out the problematic snippet.
        let msg = Level::Error.title(title).snippet(snippet.annotation(anno_call));
        Renderer::styled().render(msg).to_string().into()
    }

    pub fn push(&mut self, hir_id: HirId, title: &str) {
        let render = self.generate(hir_id, title);
        self.diagnostics.push(Diagnostic::missing_discharges(render));
    }
}

fn hir_span(hir_id: HirId, tcx: TyCtxt) -> Span {
    crossfig::switch! {
        crate::std => { tcx.hir_span(hir_id) }
        _ => { tcx.hir().span(hir_id) }
    }
}

fn anno_span(span_body: Span, span_node: Span) -> Range<usize> {
    let body_lo = span_body.lo().0;
    let node_lo = span_node.lo().0;
    let node_hi = span_node.hi().0;
    (node_lo - body_lo) as usize..(node_hi - body_lo) as usize
}
