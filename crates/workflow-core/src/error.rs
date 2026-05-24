use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::Diagnostic;

/// Stable category for a Workflow OS error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowOsErrorKind {
    /// Input could not be parsed.
    Parse,
    /// Input failed validation.
    Validation,
    /// Requested operation is unsupported by the current core.
    Unsupported,
    /// Requested operation is unsafe or denied by policy.
    PolicyDenied,
    /// Operation cannot proceed because core state is invalid or ambiguous.
    InvalidState,
    /// Sensitive data would be exposed or mishandled.
    Security,
    /// An internal invariant was violated.
    Internal,
}

impl fmt::Display for WorkflowOsErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse => formatter.write_str("parse"),
            Self::Validation => formatter.write_str("validation"),
            Self::Unsupported => formatter.write_str("unsupported"),
            Self::PolicyDenied => formatter.write_str("policy_denied"),
            Self::InvalidState => formatter.write_str("invalid_state"),
            Self::Security => formatter.write_str("security"),
            Self::Internal => formatter.write_str("internal"),
        }
    }
}

/// Structured error used by Workflow OS core.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowOsError {
    kind: WorkflowOsErrorKind,
    code: String,
    message: String,
    diagnostics: Vec<Diagnostic>,
}

impl WorkflowOsError {
    /// Creates a structured Workflow OS error.
    #[must_use]
    pub fn new(
        kind: WorkflowOsErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            message: message.into(),
            diagnostics: Vec::new(),
        }
    }

    /// Creates a validation error.
    #[must_use]
    pub fn validation(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(WorkflowOsErrorKind::Validation, code, message)
    }

    /// Creates an invalid state error.
    #[must_use]
    pub fn invalid_state(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(WorkflowOsErrorKind::InvalidState, code, message)
    }

    /// Creates a security error.
    #[must_use]
    pub fn security(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(WorkflowOsErrorKind::Security, code, message)
    }

    /// Attaches one diagnostic.
    #[must_use]
    pub fn with_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }

    /// Attaches multiple diagnostics.
    #[must_use]
    pub fn with_diagnostics(mut self, diagnostics: impl IntoIterator<Item = Diagnostic>) -> Self {
        self.diagnostics.extend(diagnostics);
        self
    }

    /// Returns the stable error kind.
    #[must_use]
    pub const fn kind(&self) -> WorkflowOsErrorKind {
        self.kind
    }

    /// Returns the stable error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns structured diagnostics associated with the error.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

impl fmt::Display for WorkflowOsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}[{}]: {}", self.kind, self.code, self.message)
    }
}

impl Error for WorkflowOsError {}

impl From<Diagnostic> for WorkflowOsError {
    fn from(diagnostic: Diagnostic) -> Self {
        Self::validation(
            diagnostic.code().to_owned(),
            diagnostic.message().to_owned(),
        )
        .with_diagnostic(diagnostic)
    }
}
