/// A report / diagnostic to display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticKind {
    // A non-existent tag is specified.
    // InvaidTag,
    /// A missing `#[discharges]` attribute on a call with context of source code.
    MissingDischarges,
}

pub struct Diagnostic {
    pub render: Box<str>,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    // pub fn invalid_tag(render: Box<str>) -> Self {
    //     Diagnostic { render, kind: DiagnosticKind::InvaidTag }
    // }

    pub fn missing_discharges(render: Box<str>) -> Self {
        Diagnostic { render, kind: DiagnosticKind::MissingDischarges }
    }
}

/// How to emit diagnostics.
#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::enum_variant_names)]
pub enum ExitAndEmit {
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
    pub fn new() -> Self {
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
    pub fn should_abort(self) -> bool {
        matches!(self, ExitAndEmit::AbortAndEmit | ExitAndEmit::AbortAndNoEmit)
    }

    /// Emit diagnostics if exist.
    pub fn should_emit(self) -> bool {
        matches!(self, ExitAndEmit::AbortAndEmit | ExitAndEmit::SlienceAndEmit)
    }
}

pub fn total(diagnostics: &[Diagnostic]) {
    use annotate_snippets::renderer::{AnsiColor, Style};
    use itertools::Itertools;

    let counts = diagnostics.iter().counts_by(|d| d.kind);
    let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
    eprintln!("{style}Total counts of diagnostics from safety-tool: {counts:?}{style:#}\n");
}
