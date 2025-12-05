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
    fn generate(&mut self, hir_id: HirId, title: &str, info: &[String]) -> Box<str> {
        let span_node = hir_span(hir_id, self.tcx);
        // error!(span_node = %self.src_map.span_to_snippet(span_node).unwrap());
        let span_body = self.tcx.source_span(hir_id.owner);

        // Point out an unsafe call with underlines.
        let range = match range_of_call(span_body, span_node) {
            Ok(range) => range,
            // FIXME: point out the real function callsite in macros.
            // Currently, only the macro callsite is reported.
            Err(range) => range,
        };
        let anno_call = Level::Error.span(range).label("For this unsafe call.");

        let src_body = self.src_map.span_to_snippet(span_body).unwrap();
        let file_and_line = self.src_map.lookup_line(span_body.lo()).unwrap();
        let line_start = file_and_line.line + 1; // adjust to starting from 1
        let origin = file_and_line.sf.name.prefer_local().to_string_lossy();
        let snippet = Snippet::source(&src_body).line_start(line_start).origin(&origin).fold(true);

        // Point out the problematic snippet.
        let msg = Level::Error
            .title(title)
            .snippet(snippet.annotation(anno_call))
            .footers(info.iter().map(|info| Level::Info.title(info)));
        Renderer::styled().render(msg).to_string().into()
    }

    /// Add a diagnostic based on an unsafe call. Title is the first line of error msg.
    pub fn push_missing_discharge(&mut self, hir_id: HirId, title: &str, info: &[String]) {
        let render = self.generate(hir_id, title, info);
        self.diagnostics.push(Diagnostic::missing_discharge(render));
    }

    pub fn push_duplicate_discharge(&mut self, hir_id: HirId, title: &str) {
        let render = self.generate(hir_id, title, &[]);
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
        crate::asterinas => { tcx.hir().span(hir_id) }
        _ => { tcx.hir_span(hir_id) }
    }
}

/// Compute relative position of a node in a body node.
///
/// Return value always points to the range within span_body, but the meaning of status:
/// * Ok: span_node is in span_body, i.e. function callsite is in body scope
/// * Err: span_node is out of span_body, probably owing to macro expansion,
///   thus find the ancestor span in span_body. Need to attach real callsite
///   as per the error status.
fn range_of_call(span_body: Span, span_node: Span) -> Result<Range<usize>, Range<usize>> {
    let gen_range = |span: Span| {
        let body_lo = span_body.lo().0;
        let node_lo = span.lo().0;
        let node_hi = span.hi().0;
        (node_lo - body_lo) as usize..(node_hi - body_lo) as usize
        // error!(?span_body, ?span_node, body_lo, node_lo, node_hi, ?range,);
    };

    if span_body.contains(span_node) {
        Ok(gen_range(span_node))
    } else {
        assert!(
            span_node.from_expansion(),
            "{span_node:?} is not from a macro expansion, neither directly within {span_body:?}"
        );
        for data in span_node.macro_backtrace() {
            let span = data.call_site;
            if span_body.contains(span) {
                return Err(gen_range(span));
            }
        }
        unreachable!("Failed to find how {span_node:?} is called in {span_body:?}");
    }
}
