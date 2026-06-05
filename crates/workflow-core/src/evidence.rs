use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, AdapterId, AdapterKind, CorrelationId, EventId, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, SchemaVersion, SkillId, SkillVersion, SpecContentHash,
    StepId, Timestamp, WorkflowId, WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const TITLE_MAX_BYTES: usize = 160;
const SUMMARY_MAX_BYTES: usize = 2_000;
const TARGET_FIELD_MAX_BYTES: usize = 512;
const METADATA_KEY_MAX_BYTES: usize = 64;
const METADATA_VALUE_MAX_BYTES: usize = 256;
const METADATA_MAX_ENTRIES: usize = 32;
const REDACTED: &str = "[REDACTED]";

static NEXT_EVIDENCE_ID: AtomicU64 = AtomicU64::new(1);

/// Stable identifier for a cited evidence reference.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EvidenceReferenceId(String);

impl EvidenceReferenceId {
    /// Generates a new evidence reference identifier.
    #[must_use]
    pub fn generate() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let counter = NEXT_EVIDENCE_ID.fetch_add(1, Ordering::Relaxed);
        Self(format!("evidence-{timestamp}-{counter}"))
    }

    /// Creates a new evidence reference identifier from validated text.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is empty, too long, or contains
    /// characters outside the canonical identifier character set.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("EvidenceReferenceId", &value)?;
        Ok(Self(value))
    }

    /// Returns the identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EvidenceReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for EvidenceReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("EvidenceReferenceId")
            .field(&self.0)
            .finish()
    }
}

impl From<EvidenceReferenceId> for String {
    fn from(value: EvidenceReferenceId) -> Self {
        value.0
    }
}

impl TryFrom<String> for EvidenceReferenceId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for EvidenceReferenceId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Identifier for an approval request or decision cited as evidence.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ApprovalReferenceId(String);

impl ApprovalReferenceId {
    /// Creates a validated approval reference identifier.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("ApprovalReferenceId", &value)?;
        Ok(Self(value))
    }

    /// Returns the identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ApprovalReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for ApprovalReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ApprovalReferenceId")
            .field(&self.0)
            .finish()
    }
}

impl From<ApprovalReferenceId> for String {
    fn from(value: ApprovalReferenceId) -> Self {
        value.0
    }
}

impl TryFrom<String> for ApprovalReferenceId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Identifier for a validation result or diagnostic cited as evidence.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ValidationReferenceId(String);

impl ValidationReferenceId {
    /// Creates a validated validation reference identifier.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("ValidationReferenceId", &value)?;
        Ok(Self(value))
    }

    /// Returns the identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidationReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for ValidationReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ValidationReferenceId")
            .field(&self.0)
            .finish()
    }
}

impl From<ValidationReferenceId> for String {
    fn from(value: ValidationReferenceId) -> Self {
        value.0
    }
}

impl TryFrom<String> for ValidationReferenceId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Domain-neutral kind of evidence being cited.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// Local file reference.
    LocalFile,
    /// Workflow OS spec file reference.
    SpecFile,
    /// Validation result or diagnostic.
    ValidationResult,
    /// Workflow run event.
    WorkflowEvent,
    /// Audit event.
    AuditEvent,
    /// Adapter invocation.
    AdapterInvocation,
    /// Adapter response summary.
    AdapterResponseSummary,
    /// Approval decision.
    ApprovalDecision,
    /// Policy decision.
    PolicyDecision,
    /// Operator note.
    OperatorNote,
    /// External object or URL reference.
    ExternalReference,
    /// Test result.
    TestResult,
    /// Command output summary.
    CommandOutput,
    /// Release review artifact.
    ReleaseReview,
    /// Live smoke evidence artifact.
    LiveSmokeEvidence,
}

/// Scope in which the evidence reference is meaningful.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceScope {
    /// Project-scoped evidence.
    Project,
    /// Workflow-scoped evidence.
    Workflow,
    /// Workflow-run-scoped evidence.
    Run,
    /// Workflow step evidence.
    Step,
    /// Skill evidence.
    Skill,
    /// Adapter evidence.
    Adapter,
    /// Audit evidence.
    Audit,
    /// Validation evidence.
    Validation,
    /// Approval evidence.
    Approval,
    /// Policy evidence.
    Policy,
    /// External evidence.
    External,
    /// Release evidence.
    Release,
    /// Operator-supplied evidence.
    Operator,
}

/// Component that created an evidence reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSourceComponent {
    /// Workflow validator.
    Validator,
    /// Runtime kernel.
    Runtime,
    /// Adapter layer.
    Adapter,
    /// CLI.
    Cli,
    /// Human operator.
    Operator,
    /// Skill implementation.
    Skill,
    /// Release review process.
    ReleaseReview,
    /// Test harness.
    Test,
    /// Unknown or external component.
    External,
}

/// Sensitivity classification for an evidence reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSensitivity {
    /// Public evidence.
    Public,
    /// Internal evidence.
    Internal,
    /// Confidential evidence.
    Confidential,
    /// Regulated evidence.
    Regulated,
    /// Secret evidence.
    Secret,
    /// Unknown sensitivity, treated conservatively.
    Unknown,
}

impl EvidenceSensitivity {
    /// Conservative default for evidence with unknown sensitivity.
    #[must_use]
    pub const fn conservative_default() -> Self {
        Self::Confidential
    }
}

/// Non-binding retention hint for evidence references.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRetentionHint {
    /// Short-lived local evidence.
    ShortLived,
    /// Retain with audit artifacts.
    AuditRetained,
    /// Retain with future report artifacts.
    ReportRetained,
    /// Underlying evidence remains external.
    ExternalOnly,
}

/// Redaction metadata for evidence references.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceRedactionMetadata {
    /// Redaction metadata using the core audit redaction shape.
    pub metadata: RedactionMetadata,
}

impl EvidenceRedactionMetadata {
    /// Creates evidence redaction metadata from field-level states.
    ///
    /// # Errors
    ///
    /// Returns an error when no field-level redaction states are supplied.
    pub fn new(field_states: Vec<RedactionFieldState>) -> Result<Self, WorkflowOsError> {
        if field_states.is_empty() {
            return Err(validation_error(
                "evidence.redaction.required",
                "evidence references require redaction metadata",
            ));
        }

        let redacted_fields = field_states
            .iter()
            .filter(|state| state.disposition == RedactionDisposition::Redacted)
            .map(|state| state.field.clone())
            .collect();

        Ok(Self {
            metadata: RedactionMetadata {
                redacted_fields,
                field_states,
            },
        })
    }

    /// Creates reference-only redaction metadata for one field.
    ///
    /// # Errors
    ///
    /// Returns an error when the field or reason is invalid.
    pub fn reference_only(field: &str, reason: &str) -> Result<Self, WorkflowOsError> {
        let field = bounded_plain_string("redaction field", field, METADATA_KEY_MAX_BYTES)?;
        let reason = bounded_plain_string("redaction reason", reason, METADATA_VALUE_MAX_BYTES)?;
        Self::new(vec![RedactionFieldState {
            field,
            disposition: RedactionDisposition::ReferenceOnly,
            reason,
        }])
    }

    /// Creates redacted metadata for one field.
    ///
    /// # Errors
    ///
    /// Returns an error when the field or reason is invalid.
    pub fn redacted(field: &str, reason: &str) -> Result<Self, WorkflowOsError> {
        let field = bounded_plain_string("redaction field", field, METADATA_KEY_MAX_BYTES)?;
        let reason = bounded_plain_string("redaction reason", reason, METADATA_VALUE_MAX_BYTES)?;
        Self::new(vec![RedactionFieldState {
            field,
            disposition: RedactionDisposition::Redacted,
            reason,
        }])
    }

    pub(crate) fn sanitized_for_attachment(&self) -> Result<Self, WorkflowOsError> {
        let mut states = Vec::new();
        for state in &self.metadata.field_states {
            states.push(RedactionFieldState {
                field: bounded_plain_string(
                    "redaction field",
                    &state.field,
                    METADATA_KEY_MAX_BYTES,
                )?,
                disposition: state.disposition,
                reason: bounded_plain_string(
                    "redaction reason",
                    &state.reason,
                    METADATA_VALUE_MAX_BYTES,
                )?,
            });
        }
        Self::new(states)
    }
}

/// Bounded, redaction-aware metadata for evidence references.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    try_from = "BTreeMap<String, String>",
    into = "BTreeMap<String, String>"
)]
pub struct EvidenceMetadata(BTreeMap<String, String>);

impl EvidenceMetadata {
    /// Creates bounded non-secret metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when metadata is too large or contains secret-like values.
    pub fn new(values: BTreeMap<String, String>) -> Result<Self, WorkflowOsError> {
        if values.len() > METADATA_MAX_ENTRIES {
            return Err(validation_error(
                "evidence.metadata.too_many_entries",
                "evidence metadata has too many entries",
            ));
        }

        let mut sanitized = BTreeMap::new();
        for (key, value) in values {
            let key = bounded_plain_string("metadata key", &key, METADATA_KEY_MAX_BYTES)?;
            let value =
                bounded_sanitized_string("metadata value", &value, METADATA_VALUE_MAX_BYTES, true)?;
            sanitized.insert(key, value);
        }

        Ok(Self(sanitized))
    }

    /// Creates empty evidence metadata.
    #[must_use]
    pub fn empty() -> Self {
        Self(BTreeMap::new())
    }

    /// Returns metadata entries.
    #[must_use]
    pub const fn entries(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}

impl fmt::Debug for EvidenceMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("EvidenceMetadata([REDACTED])")
    }
}

impl From<EvidenceMetadata> for BTreeMap<String, String> {
    fn from(value: EvidenceMetadata) -> Self {
        value.0
    }
}

impl TryFrom<BTreeMap<String, String>> for EvidenceMetadata {
    type Error = WorkflowOsError;

    fn try_from(value: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Typed reference target for an evidence reference.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EvidenceReferenceTarget {
    /// URI reference. Must not include credentials.
    Uri {
        /// Redacted-safe URI string.
        uri: String,
    },
    /// Internal Workflow OS object reference.
    Internal {
        /// Internal object type.
        object_type: String,
        /// Internal object identifier.
        id: String,
    },
    /// External provider object reference.
    External {
        /// External system name.
        system: String,
        /// Provider object reference.
        reference: String,
    },
    /// Local file path reference.
    File {
        /// Local file path.
        path: String,
    },
    /// Opaque non-secret reference.
    Opaque {
        /// Opaque reference text.
        reference: String,
    },
    /// Redacted command output summary reference.
    CommandOutput {
        /// Command or command family.
        command: String,
        /// Redacted output summary, never raw output.
        output_summary: String,
    },
}

impl<'de> Deserialize<'de> for EvidenceReferenceTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case")]
        enum EvidenceReferenceTargetWire {
            Uri {
                uri: String,
            },
            Internal {
                object_type: String,
                id: String,
            },
            External {
                system: String,
                reference: String,
            },
            File {
                path: String,
            },
            Opaque {
                reference: String,
            },
            CommandOutput {
                command: String,
                output_summary: String,
            },
        }

        match EvidenceReferenceTargetWire::deserialize(deserializer)? {
            EvidenceReferenceTargetWire::Uri { uri } => {
                Self::uri(uri).map_err(serde::de::Error::custom)
            }
            EvidenceReferenceTargetWire::Internal { object_type, id } => {
                Self::internal(&object_type, &id).map_err(serde::de::Error::custom)
            }
            EvidenceReferenceTargetWire::External { system, reference } => {
                Self::external(&system, &reference).map_err(serde::de::Error::custom)
            }
            EvidenceReferenceTargetWire::File { path } => {
                Self::file(&path).map_err(serde::de::Error::custom)
            }
            EvidenceReferenceTargetWire::Opaque { reference } => {
                Self::opaque(&reference).map_err(serde::de::Error::custom)
            }
            EvidenceReferenceTargetWire::CommandOutput {
                command,
                output_summary,
            } => Self::command_output(&command, &output_summary).map_err(serde::de::Error::custom),
        }
    }
}

impl EvidenceReferenceTarget {
    /// Creates a URI target.
    ///
    /// # Errors
    ///
    /// Returns an error when the URI is too large or secret-like.
    pub fn uri(value: impl AsRef<str>) -> Result<Self, WorkflowOsError> {
        Ok(Self::Uri {
            uri: bounded_sanitized_string(
                "target uri",
                value.as_ref(),
                TARGET_FIELD_MAX_BYTES,
                true,
            )?,
        })
    }

    /// Creates an internal object target.
    ///
    /// # Errors
    ///
    /// Returns an error when fields are invalid.
    pub fn internal(object_type: &str, id: &str) -> Result<Self, WorkflowOsError> {
        Ok(Self::Internal {
            object_type: bounded_plain_string(
                "target object type",
                object_type,
                METADATA_KEY_MAX_BYTES,
            )?,
            id: bounded_sanitized_string("target id", id, TARGET_FIELD_MAX_BYTES, true)?,
        })
    }

    /// Creates an external provider target.
    ///
    /// # Errors
    ///
    /// Returns an error when fields are invalid.
    pub fn external(system: &str, reference: &str) -> Result<Self, WorkflowOsError> {
        Ok(Self::External {
            system: bounded_plain_string("target system", system, METADATA_KEY_MAX_BYTES)?,
            reference: bounded_sanitized_string(
                "target reference",
                reference,
                TARGET_FIELD_MAX_BYTES,
                true,
            )?,
        })
    }

    /// Creates a local file target.
    ///
    /// # Errors
    ///
    /// Returns an error when the path is invalid.
    pub fn file(path: &str) -> Result<Self, WorkflowOsError> {
        Ok(Self::File {
            path: bounded_sanitized_string("target path", path, TARGET_FIELD_MAX_BYTES, true)?,
        })
    }

    /// Creates an opaque reference target.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is invalid.
    pub fn opaque(reference: &str) -> Result<Self, WorkflowOsError> {
        Ok(Self::Opaque {
            reference: bounded_sanitized_string(
                "target reference",
                reference,
                TARGET_FIELD_MAX_BYTES,
                true,
            )?,
        })
    }

    /// Creates a command output summary target.
    ///
    /// # Errors
    ///
    /// Returns an error when command or summary is invalid.
    pub fn command_output(command: &str, output_summary: &str) -> Result<Self, WorkflowOsError> {
        Ok(Self::CommandOutput {
            command: bounded_sanitized_string(
                "target command",
                command,
                TARGET_FIELD_MAX_BYTES,
                true,
            )?,
            output_summary: bounded_sanitized_string(
                "target output summary",
                output_summary,
                SUMMARY_MAX_BYTES,
                true,
            )?,
        })
    }

    pub(crate) fn sanitized_for_attachment(&self) -> Result<Self, WorkflowOsError> {
        match self {
            Self::Uri { uri } => Self::uri(uri),
            Self::Internal { object_type, id } => Self::internal(object_type, id),
            Self::External { system, reference } => Self::external(system, reference),
            Self::File { path } => Self::file(path),
            Self::Opaque { reference } => Self::opaque(reference),
            Self::CommandOutput {
                command,
                output_summary,
            } => Self::command_output(command, output_summary),
        }
    }
}

impl fmt::Debug for EvidenceReferenceTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uri { .. } => formatter.write_str("EvidenceReferenceTarget::Uri([REDACTED])"),
            Self::Internal { object_type, .. } => formatter
                .debug_struct("Internal")
                .field("object_type", object_type)
                .field("id", &REDACTED)
                .finish(),
            Self::External { system, .. } => formatter
                .debug_struct("External")
                .field("system", system)
                .field("reference", &REDACTED)
                .finish(),
            Self::File { .. } => formatter.write_str("EvidenceReferenceTarget::File([REDACTED])"),
            Self::Opaque { .. } => {
                formatter.write_str("EvidenceReferenceTarget::Opaque([REDACTED])")
            }
            Self::CommandOutput { .. } => {
                formatter.write_str("EvidenceReferenceTarget::CommandOutput([REDACTED])")
            }
        }
    }
}

/// Core evidence reference model.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct EvidenceReference {
    /// Stable evidence reference ID.
    pub id: EvidenceReferenceId,
    /// Evidence kind.
    pub kind: EvidenceKind,
    /// Non-secret bounded title.
    pub title: String,
    /// Typed reference target.
    pub target: EvidenceReferenceTarget,
    /// Component that created the reference.
    pub source_component: EvidenceSourceComponent,
    /// Evidence scope.
    pub scope: EvidenceScope,
    /// Workflow ID where applicable.
    pub workflow_id: Option<WorkflowId>,
    /// Workflow version where applicable.
    pub workflow_version: Option<WorkflowVersion>,
    /// Schema version where applicable.
    pub schema_version: Option<SchemaVersion>,
    /// Spec content hash where applicable.
    pub spec_hash: Option<SpecContentHash>,
    /// Workflow run ID where applicable.
    pub run_id: Option<WorkflowRunId>,
    /// Step ID where applicable.
    pub step_id: Option<StepId>,
    /// Skill ID where applicable.
    pub skill_id: Option<SkillId>,
    /// Skill version where applicable.
    pub skill_version: Option<SkillVersion>,
    /// Adapter ID where applicable.
    pub adapter_id: Option<AdapterId>,
    /// Adapter kind where applicable.
    pub adapter_kind: Option<AdapterKind>,
    /// Audit event ID where applicable.
    pub audit_event_id: Option<EventId>,
    /// Workflow event ID where applicable.
    pub workflow_event_id: Option<EventId>,
    /// Approval request or decision reference where applicable.
    pub approval_id: Option<ApprovalReferenceId>,
    /// Validation result or diagnostic reference where applicable.
    pub validation_result_id: Option<ValidationReferenceId>,
    /// Correlation ID where available.
    pub correlation_id: Option<CorrelationId>,
    /// Human actor where available.
    pub actor: Option<ActorId>,
    /// System actor where available.
    pub system_actor: Option<ActorId>,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Optional redacted summary.
    pub summary: Option<String>,
    /// Content hash where available and safe.
    pub content_hash: Option<SpecContentHash>,
    /// Provider `ETag`, version, or revision where available and safe.
    pub provider_etag_or_version: Option<String>,
    /// Redaction metadata.
    pub redaction_metadata: EvidenceRedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: EvidenceSensitivity,
    /// Optional retention hint.
    pub retention_hint: Option<EvidenceRetentionHint>,
    /// Bounded non-secret metadata.
    pub metadata: EvidenceMetadata,
}

impl<'de> Deserialize<'de> for EvidenceReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EvidenceReferenceWire {
            id: EvidenceReferenceId,
            kind: EvidenceKind,
            title: String,
            target: EvidenceReferenceTarget,
            source_component: EvidenceSourceComponent,
            scope: EvidenceScope,
            workflow_id: Option<WorkflowId>,
            workflow_version: Option<WorkflowVersion>,
            schema_version: Option<SchemaVersion>,
            spec_hash: Option<SpecContentHash>,
            run_id: Option<WorkflowRunId>,
            step_id: Option<StepId>,
            skill_id: Option<SkillId>,
            skill_version: Option<SkillVersion>,
            adapter_id: Option<AdapterId>,
            adapter_kind: Option<AdapterKind>,
            audit_event_id: Option<EventId>,
            workflow_event_id: Option<EventId>,
            approval_id: Option<ApprovalReferenceId>,
            validation_result_id: Option<ValidationReferenceId>,
            correlation_id: Option<CorrelationId>,
            actor: Option<ActorId>,
            system_actor: Option<ActorId>,
            created_at: Timestamp,
            summary: Option<String>,
            content_hash: Option<SpecContentHash>,
            provider_etag_or_version: Option<String>,
            redaction_metadata: EvidenceRedactionMetadata,
            sensitivity: EvidenceSensitivity,
            retention_hint: Option<EvidenceRetentionHint>,
            metadata: EvidenceMetadata,
        }

        let wire = EvidenceReferenceWire::deserialize(deserializer)?;
        let title = bounded_sanitized_string("evidence title", &wire.title, TITLE_MAX_BYTES, true)
            .map_err(serde::de::Error::custom)?;
        let summary = wire
            .summary
            .as_deref()
            .map(|value| {
                bounded_sanitized_string("evidence summary", value, SUMMARY_MAX_BYTES, true)
            })
            .transpose()
            .map_err(serde::de::Error::custom)?;
        let provider_etag_or_version = wire
            .provider_etag_or_version
            .as_deref()
            .map(|value| {
                bounded_sanitized_string(
                    "provider etag or version",
                    value,
                    TARGET_FIELD_MAX_BYTES,
                    true,
                )
            })
            .transpose()
            .map_err(serde::de::Error::custom)?;

        let reference = EvidenceReference {
            id: wire.id,
            kind: wire.kind,
            title,
            target: wire.target,
            source_component: wire.source_component,
            scope: wire.scope,
            workflow_id: wire.workflow_id,
            workflow_version: wire.workflow_version,
            schema_version: wire.schema_version,
            spec_hash: wire.spec_hash,
            run_id: wire.run_id,
            step_id: wire.step_id,
            skill_id: wire.skill_id,
            skill_version: wire.skill_version,
            adapter_id: wire.adapter_id,
            adapter_kind: wire.adapter_kind,
            audit_event_id: wire.audit_event_id,
            workflow_event_id: wire.workflow_event_id,
            approval_id: wire.approval_id,
            validation_result_id: wire.validation_result_id,
            correlation_id: wire.correlation_id,
            actor: wire.actor,
            system_actor: wire.system_actor,
            created_at: wire.created_at,
            summary,
            content_hash: wire.content_hash,
            provider_etag_or_version,
            redaction_metadata: wire.redaction_metadata,
            sensitivity: wire.sensitivity,
            retention_hint: wire.retention_hint,
            metadata: wire.metadata,
        };
        reference.validate().map_err(serde::de::Error::custom)?;
        Ok(reference)
    }
}

/// Required fields for constructing an evidence reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceReferenceRequiredFields {
    /// Stable evidence reference ID.
    pub id: EvidenceReferenceId,
    /// Evidence kind.
    pub kind: EvidenceKind,
    /// Non-secret bounded title.
    pub title: String,
    /// Typed reference target.
    pub target: EvidenceReferenceTarget,
    /// Component that created the reference.
    pub source_component: EvidenceSourceComponent,
    /// Evidence scope.
    pub scope: EvidenceScope,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Redaction metadata.
    pub redaction_metadata: EvidenceRedactionMetadata,
    /// Optional sensitivity classification. Defaults conservatively when absent.
    pub sensitivity: Option<EvidenceSensitivity>,
}

impl EvidenceReference {
    /// Creates a new evidence reference with required fields.
    ///
    /// # Errors
    ///
    /// Returns an error when bounded fields are invalid or scope validation fails.
    pub fn new(fields: EvidenceReferenceRequiredFields) -> Result<Self, WorkflowOsError> {
        let mut reference = Self {
            id: fields.id,
            kind: fields.kind,
            title: bounded_sanitized_string(
                "evidence title",
                &fields.title,
                TITLE_MAX_BYTES,
                true,
            )?,
            target: fields.target,
            source_component: fields.source_component,
            scope: fields.scope,
            workflow_id: None,
            workflow_version: None,
            schema_version: None,
            spec_hash: None,
            run_id: None,
            step_id: None,
            skill_id: None,
            skill_version: None,
            adapter_id: None,
            adapter_kind: None,
            audit_event_id: None,
            workflow_event_id: None,
            approval_id: None,
            validation_result_id: None,
            correlation_id: None,
            actor: None,
            system_actor: None,
            created_at: fields.created_at,
            summary: None,
            content_hash: None,
            provider_etag_or_version: None,
            redaction_metadata: fields.redaction_metadata,
            sensitivity: fields
                .sensitivity
                .unwrap_or_else(EvidenceSensitivity::conservative_default),
            retention_hint: None,
            metadata: EvidenceMetadata::empty(),
        };

        if reference.kind == EvidenceKind::ExternalReference
            && matches!(
                fields.sensitivity,
                None | Some(EvidenceSensitivity::Public | EvidenceSensitivity::Internal)
            )
        {
            reference.sensitivity = EvidenceSensitivity::conservative_default();
        }

        reference.validate_base()?;
        Ok(reference)
    }

    /// Adds immutable run identity fields.
    #[must_use]
    pub fn with_run_identity(
        mut self,
        workflow_id: WorkflowId,
        workflow_version: WorkflowVersion,
        schema_version: SchemaVersion,
        spec_hash: SpecContentHash,
        run_id: WorkflowRunId,
    ) -> Self {
        self.workflow_id = Some(workflow_id);
        self.workflow_version = Some(workflow_version);
        self.schema_version = Some(schema_version);
        self.spec_hash = Some(spec_hash);
        self.run_id = Some(run_id);
        self
    }

    /// Adds a step ID.
    #[must_use]
    pub fn with_step_id(mut self, step_id: StepId) -> Self {
        self.step_id = Some(step_id);
        self
    }

    /// Adds a skill reference.
    #[must_use]
    pub fn with_skill(mut self, skill_id: SkillId, skill_version: SkillVersion) -> Self {
        self.skill_id = Some(skill_id);
        self.skill_version = Some(skill_version);
        self
    }

    /// Adds an adapter reference.
    #[must_use]
    pub fn with_adapter(mut self, adapter_id: AdapterId, adapter_kind: AdapterKind) -> Self {
        self.adapter_id = Some(adapter_id);
        self.adapter_kind = Some(adapter_kind);
        self
    }

    /// Adds an audit event reference.
    #[must_use]
    pub fn with_audit_event_id(mut self, audit_event_id: EventId) -> Self {
        self.audit_event_id = Some(audit_event_id);
        self
    }

    /// Adds a workflow event reference.
    #[must_use]
    pub fn with_workflow_event_id(mut self, workflow_event_id: EventId) -> Self {
        self.workflow_event_id = Some(workflow_event_id);
        self
    }

    /// Adds an approval reference.
    #[must_use]
    pub fn with_approval_id(mut self, approval_id: ApprovalReferenceId) -> Self {
        self.approval_id = Some(approval_id);
        self
    }

    /// Adds a validation reference.
    #[must_use]
    pub fn with_validation_result_id(
        mut self,
        validation_result_id: ValidationReferenceId,
    ) -> Self {
        self.validation_result_id = Some(validation_result_id);
        self
    }

    /// Adds an optional summary.
    ///
    /// # Errors
    ///
    /// Returns an error when the summary is too large.
    pub fn set_summary(&mut self, summary: &str) -> Result<(), WorkflowOsError> {
        self.summary = Some(bounded_sanitized_string(
            "evidence summary",
            summary,
            SUMMARY_MAX_BYTES,
            true,
        )?);
        Ok(())
    }

    /// Adds a provider `ETag` or version.
    ///
    /// # Errors
    ///
    /// Returns an error when the value is too large.
    pub fn set_provider_etag_or_version(&mut self, value: &str) -> Result<(), WorkflowOsError> {
        self.provider_etag_or_version = Some(bounded_sanitized_string(
            "provider etag or version",
            value,
            TARGET_FIELD_MAX_BYTES,
            true,
        )?);
        if matches!(
            self.sensitivity,
            EvidenceSensitivity::Public | EvidenceSensitivity::Internal
        ) {
            self.sensitivity = EvidenceSensitivity::conservative_default();
        }
        Ok(())
    }

    /// Replaces metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when metadata is invalid.
    pub fn set_metadata(&mut self, metadata: EvidenceMetadata) {
        self.metadata = metadata;
    }

    /// Validates scope-specific evidence requirements.
    ///
    /// # Errors
    ///
    /// Returns an error when scope-specific required fields are missing.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.validate_base()?;

        match self.scope {
            EvidenceScope::Run => self.require_run_identity()?,
            EvidenceScope::Step => {
                self.require_run_identity()?;
                require_some(self.step_id.as_ref(), "evidence.scope.step_id_required")?;
            }
            EvidenceScope::Skill => {
                self.require_run_identity()?;
                require_some(self.skill_id.as_ref(), "evidence.scope.skill_id_required")?;
                require_some(
                    self.skill_version.as_ref(),
                    "evidence.scope.skill_version_required",
                )?;
            }
            EvidenceScope::Adapter => {
                require_some(
                    self.adapter_id.as_ref(),
                    "evidence.scope.adapter_id_required",
                )?;
                require_some(
                    self.adapter_kind.as_ref(),
                    "evidence.scope.adapter_kind_required",
                )?;
                if self.run_id.is_some()
                    || self.workflow_id.is_some()
                    || self.workflow_version.is_some()
                    || self.schema_version.is_some()
                    || self.spec_hash.is_some()
                {
                    self.require_run_identity()?;
                }
            }
            EvidenceScope::Audit => {
                require_some(
                    self.audit_event_id.as_ref(),
                    "evidence.scope.audit_event_id_required",
                )?;
            }
            EvidenceScope::Validation => {
                require_some(
                    self.validation_result_id.as_ref(),
                    "evidence.scope.validation_reference_required",
                )?;
            }
            EvidenceScope::Approval => {
                require_some(
                    self.approval_id.as_ref(),
                    "evidence.scope.approval_id_required",
                )?;
            }
            EvidenceScope::Policy => {
                if self.workflow_event_id.is_none() && self.audit_event_id.is_none() {
                    return Err(validation_error(
                        "evidence.scope.policy_reference_required",
                        "policy evidence requires a workflow event or audit event reference",
                    ));
                }
            }
            EvidenceScope::Workflow
            | EvidenceScope::Project
            | EvidenceScope::External
            | EvidenceScope::Release
            | EvidenceScope::Operator => {}
        }

        if self.kind == EvidenceKind::WorkflowEvent {
            require_some(
                self.workflow_event_id.as_ref(),
                "evidence.kind.workflow_event_id_required",
            )?;
        }

        if self.kind == EvidenceKind::AuditEvent {
            require_some(
                self.audit_event_id.as_ref(),
                "evidence.kind.audit_event_id_required",
            )?;
        }

        if self.kind == EvidenceKind::CommandOutput
            && !matches!(self.target, EvidenceReferenceTarget::CommandOutput { .. })
        {
            return Err(validation_error(
                "evidence.kind.command_output_target_required",
                "command output evidence requires a command output summary target",
            ));
        }

        if self.kind == EvidenceKind::CommandOutput
            && self
                .redaction_metadata
                .metadata
                .field_states
                .iter()
                .all(|state| state.disposition == RedactionDisposition::Safe)
        {
            return Err(validation_error(
                "evidence.kind.command_output_redaction_required",
                "command output evidence must be reference-only or redacted",
            ));
        }

        Ok(())
    }

    fn validate_base(&self) -> Result<(), WorkflowOsError> {
        if self.redaction_metadata.metadata.field_states.is_empty() {
            return Err(validation_error(
                "evidence.redaction.required",
                "evidence references require redaction metadata",
            ));
        }
        Ok(())
    }

    fn require_run_identity(&self) -> Result<(), WorkflowOsError> {
        require_some(
            self.workflow_id.as_ref(),
            "evidence.scope.workflow_id_required",
        )?;
        require_some(
            self.workflow_version.as_ref(),
            "evidence.scope.workflow_version_required",
        )?;
        require_some(
            self.schema_version.as_ref(),
            "evidence.scope.schema_version_required",
        )?;
        require_some(self.spec_hash.as_ref(), "evidence.scope.spec_hash_required")?;
        require_some(self.run_id.as_ref(), "evidence.scope.run_id_required")?;
        Ok(())
    }

    pub(crate) fn sanitized_for_attachment(&self) -> Result<Self, WorkflowOsError> {
        let mut reference = self.clone();
        reference.title =
            bounded_sanitized_string("evidence title", &reference.title, TITLE_MAX_BYTES, true)?;
        reference.target = reference.target.sanitized_for_attachment()?;
        reference.summary = reference
            .summary
            .as_deref()
            .map(|summary| {
                bounded_sanitized_string("evidence summary", summary, SUMMARY_MAX_BYTES, true)
            })
            .transpose()?;
        reference.provider_etag_or_version = reference
            .provider_etag_or_version
            .as_deref()
            .map(|value| {
                bounded_sanitized_string(
                    "provider etag or version",
                    value,
                    TARGET_FIELD_MAX_BYTES,
                    true,
                )
            })
            .transpose()?;
        reference.redaction_metadata = reference.redaction_metadata.sanitized_for_attachment()?;
        reference.metadata = EvidenceMetadata::new(reference.metadata.entries().clone())?;
        reference.validate()?;
        Ok(reference)
    }
}

impl fmt::Debug for EvidenceReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EvidenceReference")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("title", &REDACTED)
            .field("target", &self.target)
            .field("source_component", &self.source_component)
            .field("scope", &self.scope)
            .field("workflow_id", &self.workflow_id)
            .field("workflow_version", &self.workflow_version)
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &self.spec_hash)
            .field("run_id", &self.run_id)
            .field("step_id", &self.step_id)
            .field("skill_id", &self.skill_id)
            .field("skill_version", &self.skill_version)
            .field("adapter_id", &self.adapter_id)
            .field("adapter_kind", &self.adapter_kind)
            .field("audit_event_id", &self.audit_event_id)
            .field("workflow_event_id", &self.workflow_event_id)
            .field("approval_id", &self.approval_id)
            .field("validation_result_id", &self.validation_result_id)
            .field("correlation_id", &self.correlation_id)
            .field("actor", &self.actor)
            .field("system_actor", &self.system_actor)
            .field("created_at", &self.created_at)
            .field("summary", &self.summary.as_ref().map(|_| REDACTED))
            .field("content_hash", &self.content_hash)
            .field(
                "provider_etag_or_version",
                &self.provider_etag_or_version.as_ref().map(|_| REDACTED),
            )
            .field("redaction_metadata", &self.redaction_metadata)
            .field("sensitivity", &self.sensitivity)
            .field("retention_hint", &self.retention_hint)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl fmt::Display for EvidenceReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} {:?} {:?}",
            self.id, REDACTED, self.kind, self.scope
        )
    }
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "evidence.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > 128 {
        return Err(validation_error(
            "evidence.identifier.too_long",
            format!("{type_name} cannot exceed 128 bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "evidence.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    Ok(())
}

fn bounded_plain_string(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<String, WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "evidence.string.empty",
            format!("{field} cannot be empty"),
        ));
    }
    if value.len() > max_bytes {
        return Err(validation_error(
            "evidence.string.too_long",
            format!("{field} exceeds maximum length"),
        ));
    }
    if contains_secret_like(value) {
        return Err(security_error(
            "evidence.secret_like_value",
            format!("{field} contains secret-like content"),
        ));
    }
    Ok(value.to_owned())
}

fn bounded_sanitized_string(
    field: &'static str,
    value: &str,
    max_bytes: usize,
    redact_secret_like: bool,
) -> Result<String, WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "evidence.string.empty",
            format!("{field} cannot be empty"),
        ));
    }
    if value.len() > max_bytes {
        return Err(validation_error(
            "evidence.string.too_long",
            format!("{field} exceeds maximum length"),
        ));
    }
    if contains_secret_like(value) {
        if redact_secret_like {
            return Ok(REDACTED.to_owned());
        }
        return Err(security_error(
            "evidence.secret_like_value",
            format!("{field} contains secret-like content"),
        ));
    }
    Ok(value.to_owned())
}

fn require_some<T>(value: Option<&T>, code: &'static str) -> Result<(), WorkflowOsError> {
    if value.is_none() {
        return Err(validation_error(
            code,
            "evidence reference is missing required scope data",
        ));
    }
    Ok(())
}

fn validation_error(code: impl Into<String>, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

fn security_error(code: impl Into<String>, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::security(code, message)
}

fn contains_secret_like(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("authorization:")
        || lower.contains("bearer ")
        || lower.contains("api_token")
        || lower.contains("api-token")
        || lower.contains("private key")
        || lower.contains("begin private key")
        || lower.contains("password=")
        || lower.contains("token=")
        || lower.contains("secret=")
        || lower.contains("xoxb-")
        || lower.contains("ghp_")
        || lower.contains("github_pat_")
        || lower.contains("atatt")
        || lower.contains("raw ci log")
        || lower.contains("jira description:")
        || lower.contains("jira comment:")
}
