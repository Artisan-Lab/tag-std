use annotate_snippets::{Level, Renderer, Snippet};
use rustc_hir::HirId;
use rustc_middle::ty::TyCtxt;
use rustc_span::{
    Span,
    source_map::{SourceMap, get_source_map},
};
use std::{ops::Range, sync::Arc};

/// A report / diagnostic to display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DiagnosticKind {
    // A non-existent tag is specified.
    // InvaidTag,
    /// A missing `#[discharges]` attribute on a call with context of source code.
    MissingDischarge,
    /// The tag has been discharged.
    DuplicatedDischarge,
}

struct Diagnostic {
    render: Box<str>,
    kind: DiagnosticKind,
}

impl Diagnostic {
    fn missing_discharge(render: Box<str>) -> Self {
        Diagnostic { render, kind: DiagnosticKind::MissingDischarge }
    }

    pub fn duplicated_discharge(render: Box<str>) -> Self {
        Diagnostic { render, kind: DiagnosticKind::DuplicatedDischarge }
    }
}

/// How to emit diagnostics.
#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::enum_variant_names)]
enum ExitAndEmit {
    /// Emit all diagnostics, and abort if any exists.
    #[default]
    AbortAndEmit,
    /// Don't emit any diagnostic, and abort if any exists.
    AbortAndNoEmit,
    /// Emit all diagnostics, and don't abort if any exists.
    SlienceAndEmit,
    /// Don't all diagnostics, and don't abort if any exists.
    SlienceAndNoEmit,
}

impl ExitAndEmit {
    /// specified by environment variable `EXIT_AND_EMIT`.
    /// If unset, the default behavior is [`Self::AbortAndEmit`].
    /// If set to a invald value, panic.
    fn new() -> Self {
        const VAR: &str = "EXIT_AND_EMIT";
        std::env::var(VAR).map(|var|{
            match var.to_lowercase().as_str() {
                "abort_and_emit" => Self::AbortAndEmit,
                "abort_and_no_emit" => Self::AbortAndNoEmit,
                "silence_and_emit" => Self::SlienceAndEmit,
                "silence_and_no_emit" => Self::SlienceAndNoEmit,
                _=> panic!("Invalid value of env var {VAR:?}={var}.\n\
                    Choose one among abort_and_emit, abort_and_no_emit, silence_and_emit, and silence_and_no_emit.")
            }
        }).unwrap_or_default()
    }

    /// Abort if diagnostics exist.
    fn should_abort(self) -> bool {
        matches!(self, ExitAndEmit::AbortAndEmit | ExitAndEmit::AbortAndNoEmit)
    }

    /// Emit diagnostics if exist.
    fn should_emit(self) -> bool {
        matches!(self, ExitAndEmit::AbortAndEmit | ExitAndEmit::SlienceAndEmit)
    }
}

fn total(diagnostics: &[Diagnostic]) {
    use annotate_snippets::renderer::{AnsiColor, Style};
    use itertools::Itertools;

    let counts = diagnostics.iter().counts_by(|d| d.kind);
    let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
    eprintln!("{style}Total counts of diagnostics from safety-tool: {counts:?}{style:#}\n");
}

pub struct EmitDiagnostics<'tcx> {
    tcx: TyCtxt<'tcx>,
    src_map: Arc<SourceMap>,
    diagnostics: Vec<Diagnostic>,
    exit_and_emit: ExitAndEmit,
}

impl<'tcx> EmitDiagnostics<'tcx> {
    pub fn new(tcx: TyCtxt) -> EmitDiagnostics {
        EmitDiagnostics {
            tcx,
            src_map: get_source_map().expect("Failed to get source map."),
            diagnostics: Vec::new(),
            exit_and_emit: ExitAndEmit::new(),
        }
    }

    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    #[must_use]
    fn generate(&mut self, hir_id: HirId, title: &str) -> Box<str> {
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

    /// Add a diagnostic based on an unsafe call. Title is the first line of error msg.
    pub fn push_missing_discharge(&mut self, hir_id: HirId, title: &str) {
        let render = self.generate(hir_id, title);
        self.diagnostics.push(Diagnostic::missing_discharge(render));
    }

    pub fn push_duplicate_discharge(&mut self, hir_id: HirId, title: &str) {
        let render = self.generate(hir_id, title);
        self.diagnostics.push(Diagnostic::duplicated_discharge(render));
    }

    /// Emit diagnostics, respecting EXIT_AND_EMIT.
    pub fn emit(self) {
        let Self { diagnostics, exit_and_emit, .. } = self;
        if !diagnostics.is_empty() {
            if exit_and_emit.should_emit() {
                for diagnostic in &diagnostics {
                    eprintln!("{}\n", diagnostic.render)
                }
                total(&diagnostics);
            }
            if exit_and_emit.should_abort() {
                std::process::abort()
            }
        }
    }
}

/// Get HIR node span.
fn hir_span(hir_id: HirId, tcx: TyCtxt) -> Span {
    crossfig::switch! {
        crate::std => { tcx.hir_span(hir_id) }
        _ => { tcx.hir().span(hir_id) }
    }
}

/// Compute relative position of a node in a body node.
fn anno_span(span_body: Span, span_node: Span) -> Range<usize> {
    let body_lo = span_body.lo().0;
    let node_lo = span_node.lo().0;
    let node_hi = span_node.hi().0;
    (node_lo - body_lo) as usize..(node_hi - body_lo) as usize
}
