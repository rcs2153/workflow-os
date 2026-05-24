use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Severity for a diagnostic emitted while loading, parsing, or validating project files.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    /// Informational message that does not indicate a problem.
    Info,
    /// Warning that should be visible but does not prevent execution by itself.
    Warning,
    /// Error that prevents the requested operation from safely continuing.
    Error,
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => formatter.write_str("info"),
            Self::Warning => formatter.write_str("warning"),
            Self::Error => formatter.write_str("error"),
        }
    }
}

/// Source position associated with a diagnostic.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceLocation {
    file_path: PathBuf,
    line: Option<u32>,
    column: Option<u32>,
    document_path: Option<String>,
}

impl SourceLocation {
    /// Creates a source location for a file path.
    #[must_use]
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
            line: None,
            column: None,
            document_path: None,
        }
    }

    /// Adds a one-based line number.
    #[must_use]
    pub const fn with_line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }

    /// Adds a one-based column number.
    #[must_use]
    pub const fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    /// Adds a JSON Pointer, YAML path, or similar document-local path.
    #[must_use]
    pub fn with_document_path(mut self, document_path: impl Into<String>) -> Self {
        self.document_path = Some(document_path.into());
        self
    }

    /// Returns the file path associated with the location.
    #[must_use]
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    /// Returns the one-based line number, if known.
    #[must_use]
    pub const fn line(&self) -> Option<u32> {
        self.line
    }

    /// Returns the one-based column number, if known.
    #[must_use]
    pub const fn column(&self) -> Option<u32> {
        self.column
    }

    /// Returns the JSON Pointer, YAML path, or similar document-local path, if known.
    #[must_use]
    pub fn document_path(&self) -> Option<&str> {
        self.document_path.as_deref()
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.file_path.display())?;

        if let Some(line) = self.line {
            write!(formatter, ":{line}")?;
            if let Some(column) = self.column {
                write!(formatter, ":{column}")?;
            }
        }

        if let Some(document_path) = &self.document_path {
            write!(formatter, " {document_path}")?;
        }

        Ok(())
    }
}

/// Structured diagnostic suitable for future CLI, editor, and validation output.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    code: String,
    message: String,
    source_location: Option<SourceLocation>,
}

impl Diagnostic {
    /// Creates a diagnostic with severity, stable code, and human-readable message.
    #[must_use]
    pub fn new(
        severity: DiagnosticSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            source_location: None,
        }
    }

    /// Creates an error diagnostic.
    #[must_use]
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(DiagnosticSeverity::Error, code, message)
    }

    /// Creates a warning diagnostic.
    #[must_use]
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(DiagnosticSeverity::Warning, code, message)
    }

    /// Creates an informational diagnostic.
    #[must_use]
    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(DiagnosticSeverity::Info, code, message)
    }

    /// Attaches a source location.
    #[must_use]
    pub fn with_source_location(mut self, source_location: SourceLocation) -> Self {
        self.source_location = Some(source_location);
        self
    }

    /// Returns the diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the human-readable diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional source location.
    #[must_use]
    pub const fn source_location(&self) -> Option<&SourceLocation> {
        self.source_location.as_ref()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source_location) = &self.source_location {
            write!(
                formatter,
                "{}: {}[{}]: {}",
                source_location, self.severity, self.code, self.message
            )
        } else {
            write!(
                formatter,
                "{}[{}]: {}",
                self.severity, self.code, self.message
            )
        }
    }
}
