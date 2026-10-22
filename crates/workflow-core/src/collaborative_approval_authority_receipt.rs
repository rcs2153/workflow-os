//! Payload-free evidence for one collaborative approval authority decision.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ApprovalDecisionKind, ApprovalReferenceId, CurrentAuthoritySourceId, EventId,
    HostedAuthorityRegistryRevision, SpecContentHash, Timestamp, WorkflowOsError,
};

const RECEIPT_ID_PREFIX: &str = "collaborative-approval-authority-receipt/";

/// Version of the collaborative approval-authority receipt format.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeApprovalAuthorityReceiptVersion {
    /// Initial payload-free receipt.
    V1,
}

/// Closed capability proved for the decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeApprovalAuthorityReceiptCapability {
    /// The principal could decide the exact collaborative approval.
    ApprovalDecide,
}

/// Source-model version used to resolve collaborative authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeApprovalAuthoritySourceModelVersion {
    /// Initial PostgreSQL collaborative identity authority model.
    V1,
}

/// Explicit effect of the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeApprovalAuthorityReceiptEffect {
    /// Evidence only; authority must be resolved again for another action.
    EvidenceOnlyNotAuthorization,
}

/// Point-in-time validity of the receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeApprovalAuthorityReceiptValidity {
    /// The receipt applies only to the recorded decision.
    PointInTimeOnly,
}

/// Trust posture of a deserialized receipt claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollaborativeApprovalAuthorityReceiptClaimVerificationPosture {
    /// Structure and commitments are valid, but producer trust is not restored.
    UnverifiedSerializedClaim,
}

/// Deterministic identity derived from the complete receipt commitment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct CollaborativeApprovalAuthorityReceiptId(String);

impl CollaborativeApprovalAuthorityReceiptId {
    /// Returns the stable identifier.
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
                "collaborative approval authority receipt id is invalid",
            )
        })?;
        if suffix.len() != 64
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(receipt_error(
                "id.invalid",
                "collaborative approval authority receipt id is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for CollaborativeApprovalAuthorityReceiptId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CollaborativeApprovalAuthorityReceiptId([REDACTED])")
    }
}

impl<'de> Deserialize<'de> for CollaborativeApprovalAuthorityReceiptId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(String::deserialize(deserializer)?);
        value.validate().map_err(|_| {
            serde::de::Error::custom("collaborative approval authority receipt id is invalid")
        })?;
        Ok(value)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReceiptRecord {
    version: CollaborativeApprovalAuthorityReceiptVersion,
    receipt_id: CollaborativeApprovalAuthorityReceiptId,
    approval_reference_id: ApprovalReferenceId,
    approval_decision_event_id: EventId,
    capability: CollaborativeApprovalAuthorityReceiptCapability,
    source_id: CurrentAuthoritySourceId,
    source_model_version: CollaborativeApprovalAuthoritySourceModelVersion,
    query_commitment: SpecContentHash,
    authority_revision: HostedAuthorityRegistryRevision,
    authority_revision_commitment: SpecContentHash,
    source_snapshot_commitment: SpecContentHash,
    active_grant_commitment: SpecContentHash,
    observed_at: Timestamp,
    decision_kind: ApprovalDecisionKind,
    effect: CollaborativeApprovalAuthorityReceiptEffect,
    validity: CollaborativeApprovalAuthorityReceiptValidity,
    receipt_commitment: SpecContentHash,
}

#[derive(Serialize)]
struct CommitmentInput<'a> {
    domain: &'static str,
    version: CollaborativeApprovalAuthorityReceiptVersion,
    approval_reference_id: &'a ApprovalReferenceId,
    approval_decision_event_id: &'a EventId,
    capability: CollaborativeApprovalAuthorityReceiptCapability,
    source_id: &'a CurrentAuthoritySourceId,
    source_model_version: CollaborativeApprovalAuthoritySourceModelVersion,
    query_commitment: &'a SpecContentHash,
    authority_revision: HostedAuthorityRegistryRevision,
    authority_revision_commitment: &'a SpecContentHash,
    source_snapshot_commitment: &'a SpecContentHash,
    active_grant_commitment: &'a SpecContentHash,
    observed_at: Timestamp,
    decision_kind: ApprovalDecisionKind,
    effect: CollaborativeApprovalAuthorityReceiptEffect,
    validity: CollaborativeApprovalAuthorityReceiptValidity,
}

impl ReceiptRecord {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        self.receipt_id.validate()?;
        if self.capability != CollaborativeApprovalAuthorityReceiptCapability::ApprovalDecide
            || self.effect
                != CollaborativeApprovalAuthorityReceiptEffect::EvidenceOnlyNotAuthorization
            || self.validity != CollaborativeApprovalAuthorityReceiptValidity::PointInTimeOnly
        {
            return Err(receipt_error(
                "posture.invalid",
                "collaborative approval authority receipt posture is invalid",
            ));
        }
        let expected = receipt_commitment(self)?;
        if self.receipt_commitment != expected
            || self.receipt_id
                != CollaborativeApprovalAuthorityReceiptId::from_commitment(&expected)
        {
            return Err(receipt_error(
                "commitment.mismatch",
                "collaborative approval authority receipt commitment is invalid",
            ));
        }
        Ok(())
    }
}

/// Internal proof material produced only by the future same-call consumer.
#[allow(dead_code)]
pub(crate) struct CollaborativeApprovalAuthorityReceiptProof {
    pub(crate) approval_reference_id: ApprovalReferenceId,
    pub(crate) approval_decision_event_id: EventId,
    pub(crate) source_id: CurrentAuthoritySourceId,
    pub(crate) query_commitment: SpecContentHash,
    pub(crate) authority_revision: HostedAuthorityRegistryRevision,
    pub(crate) authority_revision_commitment: SpecContentHash,
    pub(crate) source_snapshot_commitment: SpecContentHash,
    pub(crate) active_grant_commitment: SpecContentHash,
    pub(crate) observed_at: Timestamp,
    pub(crate) decision_kind: ApprovalDecisionKind,
}

/// Trusted, serialize-only evidence of one collaborative approval decision.
///
/// This value is evidence only. It cannot authorize a later decision and it
/// cannot be reconstructed from serialized data.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct CollaborativeApprovalAuthorityReceipt {
    record: ReceiptRecord,
}

impl CollaborativeApprovalAuthorityReceipt {
    #[allow(dead_code)]
    pub(crate) fn from_decision_proof(
        proof: CollaborativeApprovalAuthorityReceiptProof,
    ) -> Result<Self, WorkflowOsError> {
        let mut record = ReceiptRecord {
            version: CollaborativeApprovalAuthorityReceiptVersion::V1,
            receipt_id: CollaborativeApprovalAuthorityReceiptId(String::new()),
            approval_reference_id: proof.approval_reference_id,
            approval_decision_event_id: proof.approval_decision_event_id,
            capability: CollaborativeApprovalAuthorityReceiptCapability::ApprovalDecide,
            source_id: proof.source_id,
            source_model_version: CollaborativeApprovalAuthoritySourceModelVersion::V1,
            query_commitment: proof.query_commitment,
            authority_revision: proof.authority_revision,
            authority_revision_commitment: proof.authority_revision_commitment,
            source_snapshot_commitment: proof.source_snapshot_commitment,
            active_grant_commitment: proof.active_grant_commitment,
            observed_at: proof.observed_at,
            decision_kind: proof.decision_kind,
            effect: CollaborativeApprovalAuthorityReceiptEffect::EvidenceOnlyNotAuthorization,
            validity: CollaborativeApprovalAuthorityReceiptValidity::PointInTimeOnly,
            receipt_commitment: SpecContentHash::from_text("pending"),
        };
        record.receipt_commitment = receipt_commitment(&record)?;
        record.receipt_id =
            CollaborativeApprovalAuthorityReceiptId::from_commitment(&record.receipt_commitment);
        record.validate()?;
        Ok(Self { record })
    }

    /// Validates deterministic identity and commitment consistency.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for invalid posture, identity, or commitment.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the deterministic receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &CollaborativeApprovalAuthorityReceiptId {
        &self.record.receipt_id
    }

    /// Returns the exact approval reference.
    #[must_use]
    pub const fn approval_reference_id(&self) -> &ApprovalReferenceId {
        &self.record.approval_reference_id
    }

    /// Returns the exact approval-decision event reference.
    #[must_use]
    pub const fn approval_decision_event_id(&self) -> &EventId {
        &self.record.approval_decision_event_id
    }

    /// Returns the complete receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        &self.record.receipt_commitment
    }

    /// Returns the explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> CollaborativeApprovalAuthorityReceiptEffect {
        self.record.effect
    }

    /// Returns the point-in-time validity posture.
    #[must_use]
    pub const fn validity(&self) -> CollaborativeApprovalAuthorityReceiptValidity {
        self.record.validity
    }
}

impl fmt::Debug for CollaborativeApprovalAuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_record(
            "CollaborativeApprovalAuthorityReceipt",
            &self.record,
            formatter,
        )
    }
}

/// Structurally valid but unauthenticated serialized receipt claim.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct UnverifiedCollaborativeApprovalAuthorityReceipt {
    record: ReceiptRecord,
}

impl UnverifiedCollaborativeApprovalAuthorityReceipt {
    /// Validates structure and commitments without restoring producer trust.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for invalid posture, identity, or commitment.
    pub fn validate_claim(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the fixed unverified posture.
    #[must_use]
    pub const fn verification_posture(
        &self,
    ) -> CollaborativeApprovalAuthorityReceiptClaimVerificationPosture {
        CollaborativeApprovalAuthorityReceiptClaimVerificationPosture::UnverifiedSerializedClaim
    }

    /// Returns the deterministic receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &CollaborativeApprovalAuthorityReceiptId {
        &self.record.receipt_id
    }

    /// Returns the exact approval reference.
    #[must_use]
    pub const fn approval_reference_id(&self) -> &ApprovalReferenceId {
        &self.record.approval_reference_id
    }

    /// Returns the exact approval-decision event reference.
    #[must_use]
    pub const fn approval_decision_event_id(&self) -> &EventId {
        &self.record.approval_decision_event_id
    }

    /// Returns the exact authority query commitment for trusted in-process
    /// replay binding checks.
    #[must_use]
    pub(crate) const fn query_commitment(&self) -> &SpecContentHash {
        &self.record.query_commitment
    }

    /// Returns the original database-time authority observation for trusted
    /// in-process replay verification.
    #[must_use]
    pub(crate) const fn observed_at(&self) -> Timestamp {
        self.record.observed_at
    }

    /// Returns the authority revision bound into the historical proof.
    #[must_use]
    pub(crate) const fn authority_revision(&self) -> HostedAuthorityRegistryRevision {
        self.record.authority_revision
    }

    /// Returns the authority-revision commitment bound into the historical proof.
    #[must_use]
    pub(crate) const fn authority_revision_commitment(&self) -> &SpecContentHash {
        &self.record.authority_revision_commitment
    }

    /// Returns the source-snapshot commitment bound into the historical proof.
    #[must_use]
    pub(crate) const fn source_snapshot_commitment(&self) -> &SpecContentHash {
        &self.record.source_snapshot_commitment
    }

    /// Returns the active-grant commitment bound into the historical proof.
    #[must_use]
    pub(crate) const fn active_grant_commitment(&self) -> &SpecContentHash {
        &self.record.active_grant_commitment
    }

    /// Returns the exact decision kind bound into the historical proof.
    #[must_use]
    pub(crate) const fn decision_kind(&self) -> ApprovalDecisionKind {
        self.record.decision_kind
    }

    /// Returns the complete receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        &self.record.receipt_commitment
    }

    /// Returns the explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> CollaborativeApprovalAuthorityReceiptEffect {
        self.record.effect
    }

    /// Returns the point-in-time validity posture.
    #[must_use]
    pub const fn validity(&self) -> CollaborativeApprovalAuthorityReceiptValidity {
        self.record.validity
    }
}

impl<'de> Deserialize<'de> for UnverifiedCollaborativeApprovalAuthorityReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let record = ReceiptRecord::deserialize(deserializer)?;
        record.validate().map_err(|_| {
            serde::de::Error::custom("invalid collaborative approval authority receipt claim")
        })?;
        Ok(Self { record })
    }
}

impl fmt::Debug for UnverifiedCollaborativeApprovalAuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_record(
            "UnverifiedCollaborativeApprovalAuthorityReceipt",
            &self.record,
            formatter,
        )
    }
}

fn receipt_commitment(record: &ReceiptRecord) -> Result<SpecContentHash, WorkflowOsError> {
    serde_json::to_vec(&CommitmentInput {
        domain: "workflow-os/collaborative-approval-authority-receipt/v1",
        version: record.version,
        approval_reference_id: &record.approval_reference_id,
        approval_decision_event_id: &record.approval_decision_event_id,
        capability: record.capability,
        source_id: &record.source_id,
        source_model_version: record.source_model_version,
        query_commitment: &record.query_commitment,
        authority_revision: record.authority_revision,
        authority_revision_commitment: &record.authority_revision_commitment,
        source_snapshot_commitment: &record.source_snapshot_commitment,
        active_grant_commitment: &record.active_grant_commitment,
        observed_at: record.observed_at,
        decision_kind: record.decision_kind,
        effect: record.effect,
        validity: record.validity,
    })
    .map(SpecContentHash::from_bytes)
    .map_err(|_| {
        receipt_error(
            "commitment.failed",
            "collaborative approval authority receipt commitment could not be created",
        )
    })
}

fn debug_record(
    name: &str,
    record: &ReceiptRecord,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("version", &record.version)
        .field("receipt_id", &"[REDACTED]")
        .field("approval_reference", &"[REDACTED]")
        .field("approval_decision_event", &"[REDACTED]")
        .field("capability", &record.capability)
        .field("source", &"[REDACTED]")
        .field("authority_revision", &"[REDACTED]")
        .field("commitments", &"[REDACTED]")
        .field("observed_at", &"[REDACTED]")
        .field("decision_kind", &record.decision_kind)
        .field("effect", &record.effect)
        .field("validity", &record.validity)
        .finish()
}

fn receipt_error(suffix: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(
        format!("collaborative_approval_authority_receipt.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    fn receipt() -> CollaborativeApprovalAuthorityReceipt {
        CollaborativeApprovalAuthorityReceipt::from_decision_proof(
            CollaborativeApprovalAuthorityReceiptProof {
                approval_reference_id: ApprovalReferenceId::new("approval/test").expect("approval"),
                approval_decision_event_id: EventId::new("event/test-decision").expect("event"),
                source_id: CurrentAuthoritySourceId::new("core/postgres/collaborative-identity")
                    .expect("source"),
                query_commitment: SpecContentHash::from_text("query"),
                authority_revision: HostedAuthorityRegistryRevision::new(7).expect("revision"),
                authority_revision_commitment: SpecContentHash::from_text("revision"),
                source_snapshot_commitment: SpecContentHash::from_text("snapshot"),
                active_grant_commitment: SpecContentHash::from_text("grant"),
                observed_at: Timestamp::parse_rfc3339("2026-10-22T08:00:00Z").expect("timestamp"),
                decision_kind: ApprovalDecisionKind::Granted,
            },
        )
        .expect("receipt")
    }

    #[test]
    fn trusted_receipt_round_trips_only_as_unverified_claim() {
        let receipt = receipt();
        let encoded = serde_json::to_string(&receipt).expect("serialize");
        for forbidden in [
            "external_subject",
            "issuer",
            "audience",
            "provider_payload",
            "token",
            "decision_reason",
            "presentation_content",
            "workflow_content",
        ] {
            assert!(!encoded.contains(forbidden));
        }
        let claim: UnverifiedCollaborativeApprovalAuthorityReceipt =
            serde_json::from_str(&encoded).expect("claim");
        assert_eq!(claim.receipt_id(), receipt.receipt_id());
        assert_eq!(
            claim.verification_posture(),
            CollaborativeApprovalAuthorityReceiptClaimVerificationPosture::UnverifiedSerializedClaim
        );
        assert_eq!(
            claim.effect(),
            CollaborativeApprovalAuthorityReceiptEffect::EvidenceOnlyNotAuthorization
        );
    }

    #[test]
    fn tampered_claim_fails_closed_without_leaking_payload() {
        let secret = "token=collaborative-receipt-secret";
        let encoded = serde_json::to_string(&receipt())
            .expect("serialize")
            .replace('7', secret);
        let error =
            serde_json::from_str::<UnverifiedCollaborativeApprovalAuthorityReceipt>(&encoded)
                .expect_err("tampered claim rejected");
        assert!(!error.to_string().contains(secret));
    }

    #[test]
    fn debug_redacts_receipt_references_and_commitments() {
        let receipt = receipt();
        let rendered = format!("{receipt:?}");
        assert!(!rendered.contains("approval/test"));
        assert!(!rendered.contains("event/test-decision"));
        assert!(!rendered.contains(receipt.receipt_commitment().as_str()));
    }
}
