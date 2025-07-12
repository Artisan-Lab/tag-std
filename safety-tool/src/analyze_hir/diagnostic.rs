/// A report / diagnosis to display.
pub enum DiagnosticKind {
    /// A non-existent tag is specified.
    InvaidTag,
    /// A missing `#[discharges]` attribute on a call with context of source code.
    MissingDischarges,
}

pub struct Diagnostic {
    pub render: Box<str>,
    #[allow(dead_code)]
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn invalid_tag(render: Box<str>) -> Self {
        Diagnostic { render, kind: DiagnosticKind::InvaidTag }
    }

    pub fn missing_discharges(render: Box<str>) -> Self {
        Diagnostic { render, kind: DiagnosticKind::MissingDischarges }
    }
}
