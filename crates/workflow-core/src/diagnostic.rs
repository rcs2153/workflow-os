use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    EvidenceKind, EvidenceRedactionMetadata, EvidenceReference, EvidenceReferenceId,
    EvidenceReferenceRequiredFields, EvidenceReferenceTarget, EvidenceScope, EvidenceSensitivity,
    EvidenceSourceComponent, Timestamp, WorkflowOsError,
};

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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    code: String,
    message: String,
    source_location: Option<SourceLocation>,
    /// Validated evidence references attached to this diagnostic.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_diagnostic_evidence_references"
    )]
    evidence_references: Vec<EvidenceReference>,
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
            evidence_references: Vec::new(),
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

    /// Returns validated evidence references attached to this diagnostic.
    #[must_use]
    pub fn evidence_references(&self) -> &[EvidenceReference] {
        &self.evidence_references
    }

    /// Attaches one validated evidence reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the evidence reference is invalid or uses an
    /// unsupported evidence kind or scope for diagnostic evidence.
    pub fn attach_evidence_reference(
        &mut self,
        evidence: &EvidenceReference,
    ) -> Result<(), WorkflowOsError> {
        let evidence = validate_diagnostic_evidence(evidence)?;
        self.evidence_references.push(evidence);
        Ok(())
    }

    /// Attaches multiple evidence references atomically.
    ///
    /// # Errors
    ///
    /// Returns an error when any evidence reference is invalid. No references
    /// are attached when validation fails.
    pub fn attach_evidence_references(
        &mut self,
        evidence_references: impl IntoIterator<Item = EvidenceReference>,
    ) -> Result<(), WorkflowOsError> {
        let evidence_references = validate_diagnostic_evidence_references(evidence_references)?;
        self.evidence_references.extend(evidence_references);
        Ok(())
    }

    /// Returns a new diagnostic with validated evidence references attached.
    ///
    /// # Errors
    ///
    /// Returns an error when any evidence reference is invalid.
    pub fn with_evidence_references(
        mut self,
        evidence_references: impl IntoIterator<Item = EvidenceReference>,
    ) -> Result<Self, WorkflowOsError> {
        self.attach_evidence_references(evidence_references)?;
        Ok(self)
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

impl Hash for Diagnostic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.severity.hash(state);
        self.code.hash(state);
        self.message.hash(state);
        self.source_location.hash(state);
    }
}

pub(crate) fn with_spec_file_evidence_from_source_location(
    mut diagnostic: Diagnostic,
) -> Diagnostic {
    let Some(source_location) = diagnostic.source_location().cloned() else {
        return diagnostic;
    };
    let Ok(evidence) = spec_file_evidence_for_source_location(&source_location) else {
        return diagnostic;
    };
    if diagnostic.attach_evidence_reference(&evidence).is_err() {
        return diagnostic;
    }
    diagnostic
}

fn spec_file_evidence_for_source_location(
    source_location: &SourceLocation,
) -> Result<EvidenceReference, WorkflowOsError> {
    EvidenceReference::new(EvidenceReferenceRequiredFields {
        id: EvidenceReferenceId::generate(),
        kind: EvidenceKind::SpecFile,
        title: "Spec file reference".to_owned(),
        target: EvidenceReferenceTarget::file(&source_location.file_path().display().to_string())?,
        source_component: EvidenceSourceComponent::Validator,
        scope: EvidenceScope::Project,
        created_at: Timestamp::now_utc(),
        redaction_metadata: EvidenceRedactionMetadata::reference_only(
            "target",
            "references spec file path; raw contents are not stored",
        )?,
        sensitivity: Some(EvidenceSensitivity::Confidential),
    })
}

fn validate_diagnostic_evidence_references(
    evidence_references: impl IntoIterator<Item = EvidenceReference>,
) -> Result<Vec<EvidenceReference>, WorkflowOsError> {
    let mut validated = Vec::new();
    for evidence in evidence_references {
        validated.push(validate_diagnostic_evidence(&evidence)?);
    }
    Ok(validated)
}

fn deserialize_diagnostic_evidence_references<'de, D>(
    deserializer: D,
) -> Result<Vec<EvidenceReference>, D::Error>
where
    D: Deserializer<'de>,
{
    let evidence_references = Vec::<EvidenceReference>::deserialize(deserializer)?;
    validate_diagnostic_evidence_references(evidence_references).map_err(serde::de::Error::custom)
}

fn validate_diagnostic_evidence(
    evidence: &EvidenceReference,
) -> Result<EvidenceReference, WorkflowOsError> {
    let evidence = evidence.sanitized_for_attachment()?;
    if !matches!(
        evidence.kind,
        EvidenceKind::ValidationResult | EvidenceKind::SpecFile
    ) {
        return Err(WorkflowOsError::validation(
            "diagnostic.evidence.kind_unsupported",
            "diagnostic evidence must be validation result or spec file evidence",
        ));
    }

    if !matches!(
        evidence.scope,
        EvidenceScope::Validation | EvidenceScope::Project | EvidenceScope::Workflow
    ) {
        return Err(WorkflowOsError::validation(
            "diagnostic.evidence.scope_unsupported",
            "diagnostic evidence must be validation, project, or workflow scoped",
        ));
    }

    Ok(evidence)
}
