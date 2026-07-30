use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, CapabilityGrantId, CapabilityReference, CapabilityResourceKind,
    GovernedContextAccessLevel, GovernedContextReferenceKind, HarnessContractId,
    HarnessContractVersion, RequiredContextRequirementId, SpecContentHash, StepId, Timestamp,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowRunId,
};

const RECEIPT_ID_PREFIX: &str = "authority-receipt/";

/// Versioned local authority-receipt model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReceiptVersion {
    /// Initial deterministic local unsigned receipt.
    V1,
}

impl AuthorityReceiptVersion {
    const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/authority-receipt/v1",
        }
    }
}

impl<'de> Deserialize<'de> for AuthorityReceiptVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "authority receipt version is invalid",
            )),
        }
    }
}

/// Deterministic identifier derived from a receipt commitment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct AuthorityReceiptId(String);

impl AuthorityReceiptId {
    /// Returns the stable receipt identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_commitment(commitment: &SpecContentHash) -> Self {
        Self(format!("{RECEIPT_ID_PREFIX}{}", commitment.as_str()))
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        let suffix = self.0.strip_prefix(RECEIPT_ID_PREFIX).ok_or_else(|| {
            receipt_error(
                "id.invalid",
                "authority receipt identifier is not derived from a receipt commitment",
            )
        })?;
        if suffix.len() != 64
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(receipt_error(
                "id.invalid",
                "authority receipt identifier is not derived from a receipt commitment",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for AuthorityReceiptId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AuthorityReceiptId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for AuthorityReceiptId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(String::deserialize(deserializer)?);
        value
            .validate()
            .map_err(|_| serde::de::Error::custom("authority receipt identifier is invalid"))?;
        Ok(value)
    }
}

/// Source class claimed by a receipt record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReceiptSourceKind {
    /// The registered local current-authority source resolved the exact call.
    RegisteredCurrentAuthorityResolutionV1,
}

impl<'de> Deserialize<'de> for AuthorityReceiptSourceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "registered_current_authority_resolution_v1" => {
                Ok(Self::RegisteredCurrentAuthorityResolutionV1)
            }
            _ => Err(serde::de::Error::custom(
                "authority receipt source kind is invalid",
            )),
        }
    }
}

/// Source-freshness posture claimed at receipt issuance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReceiptFreshnessPosture {
    /// The source was fresh for the exact point-in-time assessment.
    FreshAtIssuance,
}

impl<'de> Deserialize<'de> for AuthorityReceiptFreshnessPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "fresh_at_issuance" => Ok(Self::FreshAtIssuance),
            _ => Err(serde::de::Error::custom(
                "authority receipt freshness posture is invalid",
            )),
        }
    }
}

/// Time-bound meaning of the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReceiptValidity {
    /// The receipt records one assessment and grants no future use.
    PointInTimeOnly,
}

impl<'de> Deserialize<'de> for AuthorityReceiptValidity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "point_in_time_only" => Ok(Self::PointInTimeOnly),
            _ => Err(serde::de::Error::custom(
                "authority receipt validity is invalid",
            )),
        }
    }
}

/// Authenticity posture of the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReceiptSignaturePosture {
    /// Local deterministic record without a cryptographic issuer signature.
    LocalUnsigned,
}

impl<'de> Deserialize<'de> for AuthorityReceiptSignaturePosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "local_unsigned" => Ok(Self::LocalUnsigned),
            _ => Err(serde::de::Error::custom(
                "authority receipt signature posture is invalid",
            )),
        }
    }
}

/// Explicitly non-authorizing effect of the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReceiptEffect {
    /// Inspectable evidence only; consumers must resolve current authority again.
    EvidenceOnlyNotAuthorization,
}

impl<'de> Deserialize<'de> for AuthorityReceiptEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "evidence_only_not_authorization" => Ok(Self::EvidenceOnlyNotAuthorization),
            _ => Err(serde::de::Error::custom(
                "authority receipt effect is invalid",
            )),
        }
    }
}

/// Redaction posture enforced by the receipt model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReceiptRedactionPosture {
    /// The receipt stores references and commitments, never source payloads.
    ReferenceOnly,
}

impl<'de> Deserialize<'de> for AuthorityReceiptRedactionPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "reference_only" => Ok(Self::ReferenceOnly),
            _ => Err(serde::de::Error::custom(
                "authority receipt redaction posture is invalid",
            )),
        }
    }
}

/// Trust posture of a serialized receipt claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityReceiptClaimVerificationPosture {
    /// Structure is self-consistent, but source provenance is not authenticated.
    UnverifiedSerializedClaim,
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityReceiptRecord {
    version: AuthorityReceiptVersion,
    receipt_id: AuthorityReceiptId,
    execution_binding_hash: SpecContentHash,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    actor: ActorId,
    harness_contract_id: HarnessContractId,
    harness_contract_version: HarnessContractVersion,
    contract_content_hash: SpecContentHash,
    maximum_sensitivity: WorkReportSensitivity,
    requirement_id: RequiredContextRequirementId,
    target_kind: GovernedContextReferenceKind,
    access_level: GovernedContextAccessLevel,
    requested_sensitivity: WorkReportSensitivity,
    capability: CapabilityReference,
    resource_kind: CapabilityResourceKind,
    resource_scope_commitment: SpecContentHash,
    grant_id: CapabilityGrantId,
    source_kind: AuthorityReceiptSourceKind,
    source_snapshot_commitment: SpecContentHash,
    fact_set_commitment: SpecContentHash,
    assessment_commitment: SpecContentHash,
    issued_at: Timestamp,
    freshness_posture: AuthorityReceiptFreshnessPosture,
    validity: AuthorityReceiptValidity,
    signature_posture: AuthorityReceiptSignaturePosture,
    effect: AuthorityReceiptEffect,
    redaction_posture: AuthorityReceiptRedactionPosture,
    receipt_commitment: SpecContentHash,
}

impl AuthorityReceiptRecord {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        self.receipt_id.validate()?;
        if self.maximum_sensitivity == WorkReportSensitivity::Unknown
            || self.requested_sensitivity == WorkReportSensitivity::Unknown
        {
            return Err(receipt_error(
                "sensitivity.unknown",
                "authority receipt sensitivity must be known",
            ));
        }
        if self.requested_sensitivity > self.maximum_sensitivity {
            return Err(receipt_error(
                "sensitivity.exceeds_binding",
                "authority receipt requested sensitivity exceeds the execution binding",
            ));
        }
        if self.resource_kind == CapabilityResourceKind::Unknown {
            return Err(receipt_error(
                "resource.kind_unknown",
                "authority receipt resource kind must be known",
            ));
        }
        let expected_commitment = compute_receipt_commitment(self);
        if self.receipt_commitment != expected_commitment {
            return Err(receipt_error(
                "commitment.mismatch",
                "authority receipt commitment is invalid",
            ));
        }
        if self.receipt_id != AuthorityReceiptId::from_commitment(&expected_commitment) {
            return Err(receipt_error(
                "id.mismatch",
                "authority receipt identifier does not match its commitment",
            ));
        }
        Ok(())
    }
}

/// Payload-free local authority receipt.
///
/// This trusted model is serialize-only and has no production constructor in
/// the model-only phase. Deserialization intentionally yields
/// [`UnverifiedAuthorityReceipt`] instead. A future Core-owned producer must
/// prove exact operation outcome and source provenance before constructing a
/// trusted receipt.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct AuthorityReceipt {
    record: AuthorityReceiptRecord,
}

impl AuthorityReceipt {
    /// Validates the deterministic receipt identity and complete commitment.
    ///
    /// Validation proves internal consistency only. It does not restore source
    /// freshness, authorize execution, or make serialized claims trusted.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking errors for inconsistent receipt fields.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the receipt model version.
    #[must_use]
    pub const fn version(&self) -> AuthorityReceiptVersion {
        self.record.version
    }

    /// Returns the deterministic receipt identifier.
    #[must_use]
    pub const fn receipt_id(&self) -> &AuthorityReceiptId {
        &self.record.receipt_id
    }

    /// Returns the exact immutable execution-binding commitment.
    #[must_use]
    pub const fn execution_binding_hash(&self) -> &SpecContentHash {
        &self.record.execution_binding_hash
    }

    /// Returns the exact workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.record.workflow_id
    }

    /// Returns the exact run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.record.run_id
    }

    /// Returns the exact step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.record.step_id
    }

    /// Returns the exact actor identity.
    #[must_use]
    pub const fn actor(&self) -> &ActorId {
        &self.record.actor
    }

    /// Returns the harness-contract identity.
    #[must_use]
    pub const fn harness_contract_id(&self) -> &HarnessContractId {
        &self.record.harness_contract_id
    }

    /// Returns the harness-contract version.
    #[must_use]
    pub const fn harness_contract_version(&self) -> &HarnessContractVersion {
        &self.record.harness_contract_version
    }

    /// Returns the exact required-context contract commitment.
    #[must_use]
    pub const fn contract_content_hash(&self) -> &SpecContentHash {
        &self.record.contract_content_hash
    }

    /// Returns the immutable execution-binding sensitivity ceiling.
    #[must_use]
    pub const fn maximum_sensitivity(&self) -> WorkReportSensitivity {
        self.record.maximum_sensitivity
    }

    /// Returns the required-context requirement identity.
    #[must_use]
    pub const fn requirement_id(&self) -> &RequiredContextRequirementId {
        &self.record.requirement_id
    }

    /// Returns the governed-context target class.
    #[must_use]
    pub const fn target_kind(&self) -> GovernedContextReferenceKind {
        self.record.target_kind
    }

    /// Returns the exact governed-context access level.
    #[must_use]
    pub const fn access_level(&self) -> GovernedContextAccessLevel {
        self.record.access_level
    }

    /// Returns the exact requested sensitivity.
    #[must_use]
    pub const fn requested_sensitivity(&self) -> WorkReportSensitivity {
        self.record.requested_sensitivity
    }

    /// Returns the exact capability reference.
    #[must_use]
    pub const fn capability(&self) -> &CapabilityReference {
        &self.record.capability
    }

    /// Returns the resource class without exposing the resource reference.
    #[must_use]
    pub const fn resource_kind(&self) -> CapabilityResourceKind {
        self.record.resource_kind
    }

    /// Returns the resource-scope commitment without the resource reference.
    #[must_use]
    pub const fn resource_scope_commitment(&self) -> &SpecContentHash {
        &self.record.resource_scope_commitment
    }

    /// Returns the selected grant identity.
    #[must_use]
    pub const fn grant_id(&self) -> &CapabilityGrantId {
        &self.record.grant_id
    }

    /// Returns the claimed source class.
    #[must_use]
    pub const fn source_kind(&self) -> AuthorityReceiptSourceKind {
        self.record.source_kind
    }

    /// Returns the exact source-snapshot commitment.
    #[must_use]
    pub const fn source_snapshot_commitment(&self) -> &SpecContentHash {
        &self.record.source_snapshot_commitment
    }

    /// Returns the exact current-authority fact-set commitment.
    #[must_use]
    pub const fn fact_set_commitment(&self) -> &SpecContentHash {
        &self.record.fact_set_commitment
    }

    /// Returns the current-authority assessment commitment.
    #[must_use]
    pub const fn assessment_commitment(&self) -> &SpecContentHash {
        &self.record.assessment_commitment
    }

    /// Returns when the point-in-time assessment was recorded.
    #[must_use]
    pub const fn issued_at(&self) -> Timestamp {
        self.record.issued_at
    }

    /// Returns the claimed source-freshness posture at issuance.
    #[must_use]
    pub const fn freshness_posture(&self) -> AuthorityReceiptFreshnessPosture {
        self.record.freshness_posture
    }

    /// Returns the point-in-time-only validity posture.
    #[must_use]
    pub const fn validity(&self) -> AuthorityReceiptValidity {
        self.record.validity
    }

    /// Returns the local unsigned authenticity posture.
    #[must_use]
    pub const fn signature_posture(&self) -> AuthorityReceiptSignaturePosture {
        self.record.signature_posture
    }

    /// Returns the explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> AuthorityReceiptEffect {
        self.record.effect
    }

    /// Returns the payload-free redaction posture.
    #[must_use]
    pub const fn redaction_posture(&self) -> AuthorityReceiptRedactionPosture {
        self.record.redaction_posture
    }

    /// Returns the complete deterministic receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        &self.record.receipt_commitment
    }
}

impl fmt::Debug for AuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_receipt("AuthorityReceipt", &self.record, formatter)
    }
}

/// Structurally valid but unauthenticated serialized receipt claim.
///
/// This type deliberately has no conversion into [`AuthorityReceipt`]. A
/// future verifier must consult an authoritative source or store before any
/// serialized claim can become trusted evidence.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct UnverifiedAuthorityReceipt {
    record: AuthorityReceiptRecord,
}

impl UnverifiedAuthorityReceipt {
    /// Validates only deterministic field and commitment consistency.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking errors for inconsistent claim fields.
    pub fn validate_claim(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the fixed unverified trust posture.
    #[must_use]
    pub const fn verification_posture(&self) -> AuthorityReceiptClaimVerificationPosture {
        AuthorityReceiptClaimVerificationPosture::UnverifiedSerializedClaim
    }

    /// Returns the claimed receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &AuthorityReceiptId {
        &self.record.receipt_id
    }

    /// Returns the claimed receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        &self.record.receipt_commitment
    }
}

impl fmt::Debug for UnverifiedAuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_receipt("UnverifiedAuthorityReceipt", &self.record, formatter)
    }
}

impl<'de> Deserialize<'de> for UnverifiedAuthorityReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let record = AuthorityReceiptRecord::deserialize(deserializer)?;
        record
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid unverified authority receipt claim"))?;
        Ok(Self { record })
    }
}

fn debug_receipt(
    name: &str,
    record: &AuthorityReceiptRecord,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("version", &record.version)
        .field("receipt_id", &"[REDACTED]")
        .field("execution_identity", &"[REDACTED]")
        .field("maximum_sensitivity", &record.maximum_sensitivity)
        .field("requirement_id", &"[REDACTED]")
        .field("target_kind", &record.target_kind)
        .field("access_level", &record.access_level)
        .field("requested_sensitivity", &record.requested_sensitivity)
        .field("capability", &"[REDACTED]")
        .field("resource_kind", &record.resource_kind)
        .field("resource_scope_commitment", &"[REDACTED]")
        .field("grant_id", &"[REDACTED]")
        .field("source_kind", &record.source_kind)
        .field("source_commitments", &"[REDACTED]")
        .field("issued_at", &"[REDACTED]")
        .field("freshness_posture", &record.freshness_posture)
        .field("validity", &record.validity)
        .field("signature_posture", &record.signature_posture)
        .field("effect", &record.effect)
        .field("redaction_posture", &record.redaction_posture)
        .field("receipt_commitment", &"[REDACTED]")
        .finish_non_exhaustive()
}

fn compute_receipt_commitment(record: &AuthorityReceiptRecord) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "receipt_version",
        record.version.identifier().as_bytes(),
    );
    hash_text(
        &mut hasher,
        "execution_binding_hash",
        record.execution_binding_hash.as_str(),
    );
    hash_text(&mut hasher, "workflow_id", record.workflow_id.as_str());
    hash_text(&mut hasher, "run_id", record.run_id.as_str());
    hash_text(&mut hasher, "step_id", record.step_id.as_str());
    hash_text(&mut hasher, "actor", record.actor.as_str());
    hash_text(
        &mut hasher,
        "harness_contract_id",
        record.harness_contract_id.as_str(),
    );
    hash_text(
        &mut hasher,
        "harness_contract_version",
        record.harness_contract_version.as_str(),
    );
    hash_text(
        &mut hasher,
        "contract_content_hash",
        record.contract_content_hash.as_str(),
    );
    hash_text(
        &mut hasher,
        "maximum_sensitivity",
        sensitivity_label(record.maximum_sensitivity),
    );
    hash_text(
        &mut hasher,
        "requirement_id",
        record.requirement_id.as_str(),
    );
    hash_text(
        &mut hasher,
        "target_kind",
        target_kind_label(record.target_kind),
    );
    hash_text(
        &mut hasher,
        "access_level",
        access_level_label(record.access_level),
    );
    hash_text(
        &mut hasher,
        "requested_sensitivity",
        sensitivity_label(record.requested_sensitivity),
    );
    hash_text(&mut hasher, "capability", record.capability.as_str());
    hash_text(
        &mut hasher,
        "resource_kind",
        resource_kind_label(record.resource_kind),
    );
    hash_text(
        &mut hasher,
        "resource_scope_commitment",
        record.resource_scope_commitment.as_str(),
    );
    hash_text(&mut hasher, "grant_id", record.grant_id.as_str());
    hash_text(
        &mut hasher,
        "source_kind",
        "registered_current_authority_resolution_v1",
    );
    hash_text(
        &mut hasher,
        "source_snapshot_commitment",
        record.source_snapshot_commitment.as_str(),
    );
    hash_text(
        &mut hasher,
        "fact_set_commitment",
        record.fact_set_commitment.as_str(),
    );
    hash_text(
        &mut hasher,
        "assessment_commitment",
        record.assessment_commitment.as_str(),
    );
    hash_text(&mut hasher, "issued_at", &record.issued_at.to_rfc3339());
    hash_text(&mut hasher, "freshness_posture", "fresh_at_issuance");
    hash_text(&mut hasher, "validity", "point_in_time_only");
    hash_text(&mut hasher, "signature_posture", "local_unsigned");
    hash_text(&mut hasher, "effect", "evidence_only_not_authorization");
    hash_text(&mut hasher, "redaction_posture", "reference_only");
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_text(hasher: &mut Sha256, label: &str, value: &str) {
    hash_field(hasher, label, value.as_bytes());
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &[u8]) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value);
}

const fn resource_kind_label(kind: CapabilityResourceKind) -> &'static str {
    match kind {
        CapabilityResourceKind::Repository => "repository",
        CapabilityResourceKind::Workflow => "workflow",
        CapabilityResourceKind::LocalProject => "local_project",
        CapabilityResourceKind::AdapterResource => "adapter_resource",
        CapabilityResourceKind::ExternalResource => "external_resource",
        CapabilityResourceKind::ContextReference => "context_reference",
        CapabilityResourceKind::Unknown => "unknown",
    }
}

const fn target_kind_label(kind: GovernedContextReferenceKind) -> &'static str {
    match kind {
        GovernedContextReferenceKind::EvidenceReference => "evidence_reference",
        GovernedContextReferenceKind::WorkflowEvent => "workflow_event",
        GovernedContextReferenceKind::AuditEvent => "audit_event",
        GovernedContextReferenceKind::ValidationDiagnostic => "validation_diagnostic",
        GovernedContextReferenceKind::ApprovalDecision => "approval_decision",
        GovernedContextReferenceKind::PolicyDecision => "policy_decision",
        GovernedContextReferenceKind::SideEffect => "side_effect",
        GovernedContextReferenceKind::TypedHandoff => "typed_handoff",
        GovernedContextReferenceKind::WorkReport => "work_report",
    }
}

const fn access_level_label(level: GovernedContextAccessLevel) -> &'static str {
    match level {
        GovernedContextAccessLevel::ReferenceOnly => "reference_only",
        GovernedContextAccessLevel::BoundedMetadata => "bounded_metadata",
    }
}

const fn sensitivity_label(value: WorkReportSensitivity) -> &'static str {
    match value {
        WorkReportSensitivity::Public => "public",
        WorkReportSensitivity::Internal => "internal",
        WorkReportSensitivity::Confidential => "confidential",
        WorkReportSensitivity::Regulated => "regulated",
        WorkReportSensitivity::Secret => "secret",
        WorkReportSensitivity::Unknown => "unknown",
    }
}

fn receipt_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("authority_receipt.{suffix}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;

    fn timestamp() -> Timestamp {
        Timestamp::parse_rfc3339("2026-07-30T10:00:00Z").expect("timestamp")
    }

    fn test_receipt(grant_id: &str) -> AuthorityReceipt {
        let mut record = AuthorityReceiptRecord {
            version: AuthorityReceiptVersion::V1,
            receipt_id: AuthorityReceiptId(String::new()),
            execution_binding_hash: SpecContentHash::from_text("execution binding"),
            workflow_id: WorkflowId::new("authority/build").expect("workflow"),
            run_id: WorkflowRunId::new("run-authority").expect("run"),
            step_id: StepId::new("consume").expect("step"),
            actor: ActorId::new("agent/consumer").expect("actor"),
            harness_contract_id: HarnessContractId::new("harness/context").expect("harness"),
            harness_contract_version: HarnessContractVersion::new("v1").expect("version"),
            contract_content_hash: SpecContentHash::from_text("contract"),
            maximum_sensitivity: WorkReportSensitivity::Internal,
            requirement_id: RequiredContextRequirementId::new("required/report-reference")
                .expect("requirement"),
            target_kind: GovernedContextReferenceKind::WorkReport,
            access_level: GovernedContextAccessLevel::ReferenceOnly,
            requested_sensitivity: WorkReportSensitivity::Internal,
            capability: CapabilityReference::new("context.reference.read").expect("capability"),
            resource_kind: CapabilityResourceKind::ContextReference,
            resource_scope_commitment: SpecContentHash::from_text("resource scope"),
            grant_id: CapabilityGrantId::new(grant_id).expect("grant"),
            source_kind: AuthorityReceiptSourceKind::RegisteredCurrentAuthorityResolutionV1,
            source_snapshot_commitment: SpecContentHash::from_text("snapshot"),
            fact_set_commitment: SpecContentHash::from_text("fact set"),
            assessment_commitment: SpecContentHash::from_text("assessment"),
            issued_at: timestamp(),
            freshness_posture: AuthorityReceiptFreshnessPosture::FreshAtIssuance,
            validity: AuthorityReceiptValidity::PointInTimeOnly,
            signature_posture: AuthorityReceiptSignaturePosture::LocalUnsigned,
            effect: AuthorityReceiptEffect::EvidenceOnlyNotAuthorization,
            redaction_posture: AuthorityReceiptRedactionPosture::ReferenceOnly,
            receipt_commitment: SpecContentHash::from_text("pending"),
        };
        record.receipt_commitment = compute_receipt_commitment(&record);
        record.receipt_id = AuthorityReceiptId::from_commitment(&record.receipt_commitment);
        record.validate().expect("valid record");
        AuthorityReceipt { record }
    }

    #[test]
    fn v1_receipt_is_deterministic_and_has_fixed_non_authorizing_posture() {
        let first = test_receipt("grant/exact");
        let second = test_receipt("grant/exact");

        assert_eq!(first.receipt_id(), second.receipt_id());
        assert_eq!(first.receipt_commitment(), second.receipt_commitment());
        assert_eq!(first.validity(), AuthorityReceiptValidity::PointInTimeOnly);
        assert_eq!(
            first.signature_posture(),
            AuthorityReceiptSignaturePosture::LocalUnsigned
        );
        assert_eq!(
            first.effect(),
            AuthorityReceiptEffect::EvidenceOnlyNotAuthorization
        );
        assert_eq!(
            first.receipt_commitment().as_str(),
            "497264b2e810a4df75691617ac79e80499358e38d1e28275ec5fbe5bfe37d1ab"
        );
    }

    #[test]
    fn serialized_receipt_deserializes_only_as_unverified_claim() {
        let receipt = test_receipt("grant/exact");
        let serialized = serde_json::to_string(&receipt).expect("serialize");
        let claim: UnverifiedAuthorityReceipt =
            serde_json::from_str(&serialized).expect("unverified claim");

        assert_eq!(
            claim.verification_posture(),
            AuthorityReceiptClaimVerificationPosture::UnverifiedSerializedClaim
        );
        assert_eq!(claim.receipt_id(), receipt.receipt_id());
        assert_eq!(claim.receipt_commitment(), receipt.receipt_commitment());
        claim.validate_claim().expect("self-consistent claim");
    }

    #[test]
    fn a_different_self_consistent_claim_remains_unverified() {
        let forged = test_receipt("grant/different");
        let serialized = serde_json::to_string(&forged).expect("serialize");
        let claim: UnverifiedAuthorityReceipt =
            serde_json::from_str(&serialized).expect("unverified claim");

        assert_eq!(
            claim.verification_posture(),
            AuthorityReceiptClaimVerificationPosture::UnverifiedSerializedClaim
        );
    }

    #[test]
    fn tampering_and_unknown_fields_fail_closed_without_leaking_values() {
        let receipt = test_receipt("grant/exact");
        let mut value = serde_json::to_value(&receipt).expect("json");
        value["grant_id"] = serde_json::Value::String("grant/substituted".to_owned());
        let error = serde_json::from_value::<UnverifiedAuthorityReceipt>(value)
            .expect_err("tampered claim");
        assert_eq!(
            error.to_string(),
            "invalid unverified authority receipt claim"
        );
        assert!(!error.to_string().contains("grant/substituted"));

        let mut unknown = serde_json::to_value(&receipt).expect("json");
        unknown["provider_payload"] = serde_json::Value::String("token-secret".to_owned());
        let error = serde_json::from_value::<UnverifiedAuthorityReceipt>(unknown)
            .expect_err("unknown fields must fail");
        assert!(!error.to_string().contains("token-secret"));
    }

    #[test]
    fn debug_and_serialization_are_payload_free() {
        let receipt = test_receipt("grant/exact");
        let serialized = serde_json::to_string(&receipt).expect("serialize");
        let debug = format!("{receipt:?}");

        for forbidden in [
            "provider_payload",
            "command_output",
            "raw_spec",
            "private_key",
            "authorization_header",
        ] {
            assert!(!serialized.contains(forbidden));
            assert!(!debug.contains(forbidden));
        }
        for redacted in [
            receipt.receipt_id().as_str(),
            receipt.workflow_id().as_str(),
            receipt.run_id().as_str(),
            receipt.step_id().as_str(),
            receipt.actor().as_str(),
            receipt.grant_id().as_str(),
            receipt.receipt_commitment().as_str(),
        ] {
            assert!(!debug.contains(redacted));
        }
        assert!(debug.contains("[REDACTED]"));
    }

    #[test]
    fn commitment_framing_separates_ambiguous_field_values() {
        let first = test_receipt("grant/a-b");
        let second = test_receipt("grant/a");

        assert_ne!(first.receipt_commitment(), second.receipt_commitment());
    }
}
