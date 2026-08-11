use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::executor::SuccessfulGovernanceDecisionAuthorityReceiptProof;
use crate::{
    ApprovalReferenceId, EventId, ImmutableRunBundleBinding, SpecContentHash, Timestamp,
    WorkflowId, WorkflowOsError, WorkflowRunId,
};

const RECEIPT_ID_PREFIX: &str = "governance-decision-authority-receipt/";

/// Version of the decision-time governance authority receipt model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionAuthorityReceiptVersion {
    /// Initial payload-free local receipt.
    V1,
}

impl<'de> Deserialize<'de> for GovernanceDecisionAuthorityReceiptVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "governance decision authority receipt version is invalid",
            )),
        }
    }
}

/// Exact successful operation evidenced by the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionAuthorityReceiptOperationKind {
    /// One proof-enforced current-fact approval resume succeeded.
    ApprovalResumeReassessmentV1,
}

impl<'de> Deserialize<'de> for GovernanceDecisionAuthorityReceiptOperationKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "approval_resume_reassessment_v1" => Ok(Self::ApprovalResumeReassessmentV1),
            _ => Err(serde::de::Error::custom(
                "governance decision authority receipt operation is invalid",
            )),
        }
    }
}

/// Freshness posture recorded for the exact decision call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionAuthorityReceiptFreshnessPosture {
    /// Current facts were fresh when the decision was issued.
    FreshAtIssuance,
}

/// Time-bound meaning of the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionAuthorityReceiptValidity {
    /// The receipt cannot authorize a later operation.
    PointInTimeOnly,
}

/// Signature posture of the local receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionAuthorityReceiptSignaturePosture {
    /// Local deterministic evidence without an issuer signature.
    LocalUnsigned,
}

/// Explicitly non-authorizing effect of the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionAuthorityReceiptEffect {
    /// Evidence only; current authority must be resolved again.
    EvidenceOnlyNotAuthorization,
}

/// Redaction posture enforced by the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionAuthorityReceiptRedactionPosture {
    /// Only stable references and commitments are retained.
    ReferenceOnly,
}

macro_rules! fixed_posture_deserialize {
    ($type:ty, $wire:literal, $variant:path, $message:literal) => {
        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match String::deserialize(deserializer)?.as_str() {
                    $wire => Ok($variant),
                    _ => Err(serde::de::Error::custom($message)),
                }
            }
        }
    };
}

fixed_posture_deserialize!(
    GovernanceDecisionAuthorityReceiptFreshnessPosture,
    "fresh_at_issuance",
    GovernanceDecisionAuthorityReceiptFreshnessPosture::FreshAtIssuance,
    "governance decision authority receipt freshness is invalid"
);
fixed_posture_deserialize!(
    GovernanceDecisionAuthorityReceiptValidity,
    "point_in_time_only",
    GovernanceDecisionAuthorityReceiptValidity::PointInTimeOnly,
    "governance decision authority receipt validity is invalid"
);
fixed_posture_deserialize!(
    GovernanceDecisionAuthorityReceiptSignaturePosture,
    "local_unsigned",
    GovernanceDecisionAuthorityReceiptSignaturePosture::LocalUnsigned,
    "governance decision authority receipt signature posture is invalid"
);
fixed_posture_deserialize!(
    GovernanceDecisionAuthorityReceiptEffect,
    "evidence_only_not_authorization",
    GovernanceDecisionAuthorityReceiptEffect::EvidenceOnlyNotAuthorization,
    "governance decision authority receipt effect is invalid"
);
fixed_posture_deserialize!(
    GovernanceDecisionAuthorityReceiptRedactionPosture,
    "reference_only",
    GovernanceDecisionAuthorityReceiptRedactionPosture::ReferenceOnly,
    "governance decision authority receipt redaction posture is invalid"
);

/// Deterministic receipt identity derived from the complete commitment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct GovernanceDecisionAuthorityReceiptId(String);

impl GovernanceDecisionAuthorityReceiptId {
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
                "governance decision authority receipt id is invalid",
            )
        })?;
        if suffix.len() != 64
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(receipt_error(
                "id.invalid",
                "governance decision authority receipt id is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for GovernanceDecisionAuthorityReceiptId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("GovernanceDecisionAuthorityReceiptId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernanceDecisionAuthorityReceiptId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(String::deserialize(deserializer)?);
        value.validate().map_err(|_| {
            serde::de::Error::custom("governance decision authority receipt id is invalid")
        })?;
        Ok(value)
    }
}

/// Trust posture of a serialized receipt claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GovernanceDecisionAuthorityReceiptClaimVerificationPosture {
    /// The claim is structurally valid but has no authenticated producer.
    UnverifiedSerializedClaim,
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GovernanceDecisionAuthorityReceiptRecord {
    version: GovernanceDecisionAuthorityReceiptVersion,
    receipt_id: GovernanceDecisionAuthorityReceiptId,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    approval_reference_id: ApprovalReferenceId,
    approval_decision_event_id: EventId,
    approval_proof_marker_commitment: SpecContentHash,
    immutable_run_bundle: ImmutableRunBundleBinding,
    governance_assessment_binding_commitment: SpecContentHash,
    source_registration_commitment: SpecContentHash,
    decision_time_snapshot_commitment: SpecContentHash,
    fact_set_commitment: SpecContentHash,
    fact_count: u32,
    assessment_aggregate_fingerprint: SpecContentHash,
    operation_kind: GovernanceDecisionAuthorityReceiptOperationKind,
    issued_at: Timestamp,
    freshness_posture: GovernanceDecisionAuthorityReceiptFreshnessPosture,
    validity: GovernanceDecisionAuthorityReceiptValidity,
    signature_posture: GovernanceDecisionAuthorityReceiptSignaturePosture,
    effect: GovernanceDecisionAuthorityReceiptEffect,
    redaction_posture: GovernanceDecisionAuthorityReceiptRedactionPosture,
    receipt_commitment: SpecContentHash,
}

#[derive(Serialize)]
struct GovernanceDecisionAuthorityReceiptCommitmentInput<'a> {
    domain: &'static str,
    version: GovernanceDecisionAuthorityReceiptVersion,
    workflow_id: &'a WorkflowId,
    run_id: &'a WorkflowRunId,
    approval_reference_id: &'a ApprovalReferenceId,
    approval_decision_event_id: &'a EventId,
    approval_proof_marker_commitment: &'a SpecContentHash,
    immutable_run_bundle: &'a ImmutableRunBundleBinding,
    governance_assessment_binding_commitment: &'a SpecContentHash,
    source_registration_commitment: &'a SpecContentHash,
    decision_time_snapshot_commitment: &'a SpecContentHash,
    fact_set_commitment: &'a SpecContentHash,
    fact_count: u32,
    assessment_aggregate_fingerprint: &'a SpecContentHash,
    operation_kind: GovernanceDecisionAuthorityReceiptOperationKind,
    issued_at: Timestamp,
    freshness_posture: GovernanceDecisionAuthorityReceiptFreshnessPosture,
    validity: GovernanceDecisionAuthorityReceiptValidity,
    signature_posture: GovernanceDecisionAuthorityReceiptSignaturePosture,
    effect: GovernanceDecisionAuthorityReceiptEffect,
    redaction_posture: GovernanceDecisionAuthorityReceiptRedactionPosture,
}

impl GovernanceDecisionAuthorityReceiptRecord {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        self.receipt_id.validate()?;
        if self.fact_count == 0 {
            return Err(receipt_error(
                "fact_count.invalid",
                "governance decision authority receipt fact count is invalid",
            ));
        }
        let expected = receipt_commitment(self)?;
        if self.receipt_commitment != expected {
            return Err(receipt_error(
                "commitment.mismatch",
                "governance decision authority receipt commitment is invalid",
            ));
        }
        if self.receipt_id != GovernanceDecisionAuthorityReceiptId::from_commitment(&expected) {
            return Err(receipt_error(
                "id.mismatch",
                "governance decision authority receipt id is invalid",
            ));
        }
        Ok(())
    }
}

/// Payload-free evidence of one successful proof-enforced approval resume.
///
/// The trusted type is serialize-only. It cannot be created from public field
/// definitions and never grants authority for a later operation.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct GovernanceDecisionAuthorityReceipt {
    record: GovernanceDecisionAuthorityReceiptRecord,
}

impl GovernanceDecisionAuthorityReceipt {
    // Consuming the opaque proof prevents accidental reuse across receipts.
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn from_successful_approval_resume(
        proof: SuccessfulGovernanceDecisionAuthorityReceiptProof,
    ) -> Result<Self, WorkflowOsError> {
        let mut record = GovernanceDecisionAuthorityReceiptRecord {
            version: GovernanceDecisionAuthorityReceiptVersion::V1,
            receipt_id: GovernanceDecisionAuthorityReceiptId(String::new()),
            workflow_id: proof.workflow_id().clone(),
            run_id: proof.run_id().clone(),
            approval_reference_id: proof.approval_reference_id().clone(),
            approval_decision_event_id: proof.approval_decision_event_id().clone(),
            approval_proof_marker_commitment: proof.approval_proof_marker_commitment().clone(),
            immutable_run_bundle: proof.immutable_run_bundle().clone(),
            governance_assessment_binding_commitment: proof
                .governance_assessment_binding_commitment()
                .clone(),
            source_registration_commitment: proof.source_registration_commitment().clone(),
            decision_time_snapshot_commitment: proof.decision_time_snapshot_commitment().clone(),
            fact_set_commitment: proof.fact_set_commitment().clone(),
            fact_count: proof.fact_count(),
            assessment_aggregate_fingerprint: proof.assessment_aggregate_fingerprint().clone(),
            operation_kind:
                GovernanceDecisionAuthorityReceiptOperationKind::ApprovalResumeReassessmentV1,
            issued_at: proof.issued_at(),
            freshness_posture: GovernanceDecisionAuthorityReceiptFreshnessPosture::FreshAtIssuance,
            validity: GovernanceDecisionAuthorityReceiptValidity::PointInTimeOnly,
            signature_posture: GovernanceDecisionAuthorityReceiptSignaturePosture::LocalUnsigned,
            effect: GovernanceDecisionAuthorityReceiptEffect::EvidenceOnlyNotAuthorization,
            redaction_posture: GovernanceDecisionAuthorityReceiptRedactionPosture::ReferenceOnly,
            receipt_commitment: SpecContentHash::from_text("pending"),
        };
        record.receipt_commitment = receipt_commitment(&record)?;
        record.receipt_id =
            GovernanceDecisionAuthorityReceiptId::from_commitment(&record.receipt_commitment);
        record.validate()?;
        Ok(Self { record })
    }

    /// Validates deterministic identity and commitment consistency only.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when the receipt identity,
    /// fact count, or complete commitment is inconsistent.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the deterministic receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &GovernanceDecisionAuthorityReceiptId {
        &self.record.receipt_id
    }

    /// Returns the exact run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.record.run_id
    }

    /// Returns the exact workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.record.workflow_id
    }

    /// Returns the exact approval reference.
    #[must_use]
    pub const fn approval_reference_id(&self) -> &ApprovalReferenceId {
        &self.record.approval_reference_id
    }

    /// Returns the exact approval decision event identity.
    #[must_use]
    pub const fn approval_decision_event_id(&self) -> &EventId {
        &self.record.approval_decision_event_id
    }

    /// Returns the number of committed decision-time facts.
    #[must_use]
    pub const fn fact_count(&self) -> u32 {
        self.record.fact_count
    }

    /// Returns the complete deterministic receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        &self.record.receipt_commitment
    }

    /// Returns the explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> GovernanceDecisionAuthorityReceiptEffect {
        self.record.effect
    }

    /// Returns the point-in-time validity posture.
    #[must_use]
    pub const fn validity(&self) -> GovernanceDecisionAuthorityReceiptValidity {
        self.record.validity
    }
}

impl fmt::Debug for GovernanceDecisionAuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_record(
            "GovernanceDecisionAuthorityReceipt",
            &self.record,
            formatter,
        )
    }
}

/// Structurally valid but unauthenticated serialized receipt claim.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct UnverifiedGovernanceDecisionAuthorityReceipt {
    record: GovernanceDecisionAuthorityReceiptRecord,
}

impl UnverifiedGovernanceDecisionAuthorityReceipt {
    /// Validates structure and commitment consistency without restoring trust.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when the serialized claim
    /// has an inconsistent identity, fact count, or complete commitment.
    pub fn validate_claim(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the fixed unverified trust posture.
    #[must_use]
    pub const fn verification_posture(
        &self,
    ) -> GovernanceDecisionAuthorityReceiptClaimVerificationPosture {
        GovernanceDecisionAuthorityReceiptClaimVerificationPosture::UnverifiedSerializedClaim
    }

    /// Returns the claimed receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &GovernanceDecisionAuthorityReceiptId {
        &self.record.receipt_id
    }

    /// Returns the claimed workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.record.workflow_id
    }

    /// Returns the claimed run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.record.run_id
    }

    /// Returns the claimed approval reference.
    #[must_use]
    pub const fn approval_reference_id(&self) -> &ApprovalReferenceId {
        &self.record.approval_reference_id
    }

    /// Returns the claimed approval-decision event reference.
    #[must_use]
    pub const fn approval_decision_event_id(&self) -> &EventId {
        &self.record.approval_decision_event_id
    }

    /// Returns the claimed complete receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        &self.record.receipt_commitment
    }

    /// Returns the claimed explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> GovernanceDecisionAuthorityReceiptEffect {
        self.record.effect
    }

    /// Returns the claimed point-in-time validity posture.
    #[must_use]
    pub const fn validity(&self) -> GovernanceDecisionAuthorityReceiptValidity {
        self.record.validity
    }

    /// Returns the claimed local unsigned signature posture.
    #[must_use]
    pub const fn signature_posture(&self) -> GovernanceDecisionAuthorityReceiptSignaturePosture {
        self.record.signature_posture
    }
}

impl<'de> Deserialize<'de> for UnverifiedGovernanceDecisionAuthorityReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let record = GovernanceDecisionAuthorityReceiptRecord::deserialize(deserializer)?;
        record.validate().map_err(|_| {
            serde::de::Error::custom(
                "invalid unverified governance decision authority receipt claim",
            )
        })?;
        Ok(Self { record })
    }
}

impl fmt::Debug for UnverifiedGovernanceDecisionAuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_record(
            "UnverifiedGovernanceDecisionAuthorityReceipt",
            &self.record,
            formatter,
        )
    }
}

fn receipt_commitment(
    record: &GovernanceDecisionAuthorityReceiptRecord,
) -> Result<SpecContentHash, WorkflowOsError> {
    serde_json::to_vec(&GovernanceDecisionAuthorityReceiptCommitmentInput {
        domain: "workflow-os/governance-decision-authority-receipt/v1",
        version: record.version,
        workflow_id: &record.workflow_id,
        run_id: &record.run_id,
        approval_reference_id: &record.approval_reference_id,
        approval_decision_event_id: &record.approval_decision_event_id,
        approval_proof_marker_commitment: &record.approval_proof_marker_commitment,
        immutable_run_bundle: &record.immutable_run_bundle,
        governance_assessment_binding_commitment: &record.governance_assessment_binding_commitment,
        source_registration_commitment: &record.source_registration_commitment,
        decision_time_snapshot_commitment: &record.decision_time_snapshot_commitment,
        fact_set_commitment: &record.fact_set_commitment,
        fact_count: record.fact_count,
        assessment_aggregate_fingerprint: &record.assessment_aggregate_fingerprint,
        operation_kind: record.operation_kind,
        issued_at: record.issued_at,
        freshness_posture: record.freshness_posture,
        validity: record.validity,
        signature_posture: record.signature_posture,
        effect: record.effect,
        redaction_posture: record.redaction_posture,
    })
    .map(SpecContentHash::from_bytes)
    .map_err(|_| {
        receipt_error(
            "commitment.failed",
            "governance decision authority receipt commitment could not be created",
        )
    })
}

fn debug_record(
    name: &str,
    record: &GovernanceDecisionAuthorityReceiptRecord,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("version", &record.version)
        .field("receipt_id", &"[REDACTED]")
        .field("workflow_identity", &"[REDACTED]")
        .field("approval_reference", &"[REDACTED]")
        .field("approval_decision_event", &"[REDACTED]")
        .field("commitments", &"[REDACTED]")
        .field("fact_count", &record.fact_count)
        .field("operation_kind", &record.operation_kind)
        .field("issued_at", &"[REDACTED]")
        .field("freshness_posture", &record.freshness_posture)
        .field("validity", &record.validity)
        .field("signature_posture", &record.signature_posture)
        .field("effect", &record.effect)
        .field("redaction_posture", &record.redaction_posture)
        .finish_non_exhaustive()
}

fn receipt_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("governance_decision_authority_receipt.{suffix}"),
        message,
    )
}
