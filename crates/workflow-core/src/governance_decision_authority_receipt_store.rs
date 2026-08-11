use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ApprovalReferenceId, EventId, GovernanceDecisionAuthorityReceipt,
    GovernanceDecisionAuthorityReceiptClaimVerificationPosture,
    GovernanceDecisionAuthorityReceiptEffect, GovernanceDecisionAuthorityReceiptId,
    GovernanceDecisionAuthorityReceiptSignaturePosture, GovernanceDecisionAuthorityReceiptValidity,
    SpecContentHash, UnverifiedGovernanceDecisionAuthorityReceipt, WorkflowId, WorkflowOsError,
    WorkflowRunId,
};

/// Structurally verified receipt data loaded from a persistence boundary.
///
/// This record is local, unsigned, point-in-time evidence only. Reading it does
/// not restore the trusted in-memory receipt type and cannot authorize another
/// operation.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct PersistedGovernanceDecisionAuthorityReceiptRecord {
    claim: UnverifiedGovernanceDecisionAuthorityReceipt,
}

impl PersistedGovernanceDecisionAuthorityReceiptRecord {
    /// Validates deterministic receipt identity and commitment consistency.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the persisted claim is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.claim.validate_claim()
    }

    /// Returns the explicit unverified serialized-claim posture.
    #[must_use]
    pub const fn verification_posture(
        &self,
    ) -> GovernanceDecisionAuthorityReceiptClaimVerificationPosture {
        self.claim.verification_posture()
    }

    /// Returns the deterministic receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &GovernanceDecisionAuthorityReceiptId {
        self.claim.receipt_id()
    }

    /// Returns the committed workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        self.claim.workflow_id()
    }

    /// Returns the committed run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        self.claim.run_id()
    }

    /// Returns the committed approval reference.
    #[must_use]
    pub const fn approval_reference_id(&self) -> &ApprovalReferenceId {
        self.claim.approval_reference_id()
    }

    /// Returns the committed approval-decision event reference.
    #[must_use]
    pub const fn approval_decision_event_id(&self) -> &EventId {
        self.claim.approval_decision_event_id()
    }

    /// Returns the complete deterministic receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        self.claim.receipt_commitment()
    }

    /// Returns the explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> GovernanceDecisionAuthorityReceiptEffect {
        self.claim.effect()
    }

    /// Returns the point-in-time validity posture.
    #[must_use]
    pub const fn validity(&self) -> GovernanceDecisionAuthorityReceiptValidity {
        self.claim.validity()
    }

    /// Returns the local unsigned signature posture.
    #[must_use]
    pub const fn signature_posture(&self) -> GovernanceDecisionAuthorityReceiptSignaturePosture {
        self.claim.signature_posture()
    }
}

impl<'de> Deserialize<'de> for PersistedGovernanceDecisionAuthorityReceiptRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let claim = UnverifiedGovernanceDecisionAuthorityReceipt::deserialize(deserializer)?;
        claim.validate_claim().map_err(|_| {
            serde::de::Error::custom(
                "invalid persisted governance decision authority receipt record",
            )
        })?;
        Ok(Self { claim })
    }
}

impl std::fmt::Debug for PersistedGovernanceDecisionAuthorityReceiptRecord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PersistedGovernanceDecisionAuthorityReceiptRecord")
            .field("verification_posture", &self.verification_posture())
            .field("effect", &self.effect())
            .field("validity", &self.validity())
            .field("signature_posture", &self.signature_posture())
            .field("receipt_identity", &"[REDACTED]")
            .field("workflow_identity", &"[REDACTED]")
            .field("approval_reference", &"[REDACTED]")
            .field("commitment", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Result of a create-only receipt-record write.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GovernanceDecisionAuthorityReceiptWriteOutcome {
    /// The receipt record was written for the first time.
    Written,
    /// The exact receipt record already exists.
    AlreadyExists,
}

/// Transport-neutral create-only persistence contract for decision-time
/// governance authority receipt records.
///
/// Implementations accept only a trusted in-memory receipt for writes. Reads
/// return structurally verified, explicitly non-authorizing persisted records.
pub trait GovernanceDecisionAuthorityReceiptRecordStore {
    /// Writes one trusted receipt using create-only, exact-idempotent semantics.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when validation or persistence fails,
    /// existing content is corrupt, or the same receipt identity has conflicting
    /// content.
    fn write_governance_decision_authority_receipt(
        &self,
        receipt: &GovernanceDecisionAuthorityReceipt,
    ) -> Result<GovernanceDecisionAuthorityReceiptWriteOutcome, WorkflowOsError>;

    /// Reads one exact persisted receipt record by stable identity.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when stored data cannot be read,
    /// fails validation, or does not match its storage address.
    fn read_governance_decision_authority_receipt(
        &self,
        receipt_id: &GovernanceDecisionAuthorityReceiptId,
    ) -> Result<Option<PersistedGovernanceDecisionAuthorityReceiptRecord>, WorkflowOsError>;
}
