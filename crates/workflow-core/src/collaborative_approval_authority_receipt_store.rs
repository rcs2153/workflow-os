//! Persistence boundary for collaborative approval-authority receipts.

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ApprovalReferenceId, CollaborativeApprovalAuthorityReceiptClaimVerificationPosture,
    CollaborativeApprovalAuthorityReceiptEffect, CollaborativeApprovalAuthorityReceiptId,
    CollaborativeApprovalAuthorityReceiptValidity, EventId,
    UnverifiedCollaborativeApprovalAuthorityReceipt, WorkflowOsError,
};

/// Structurally verified, explicitly non-authorizing persisted receipt data.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct PersistedCollaborativeApprovalAuthorityReceiptRecord {
    claim: UnverifiedCollaborativeApprovalAuthorityReceipt,
}

impl PersistedCollaborativeApprovalAuthorityReceiptRecord {
    /// Validates deterministic receipt identity and commitment consistency.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the persisted claim is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.claim.validate_claim()
    }

    /// Returns the unverified serialized-claim posture.
    #[must_use]
    pub const fn verification_posture(
        &self,
    ) -> CollaborativeApprovalAuthorityReceiptClaimVerificationPosture {
        self.claim.verification_posture()
    }

    /// Returns the deterministic receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &CollaborativeApprovalAuthorityReceiptId {
        self.claim.receipt_id()
    }

    /// Returns the exact approval reference.
    #[must_use]
    pub const fn approval_reference_id(&self) -> &ApprovalReferenceId {
        self.claim.approval_reference_id()
    }

    /// Returns the exact approval-decision event reference.
    #[must_use]
    pub const fn approval_decision_event_id(&self) -> &EventId {
        self.claim.approval_decision_event_id()
    }

    /// Returns the exact authority query commitment for trusted in-process
    /// replay binding checks.
    #[must_use]
    pub(crate) const fn query_commitment(&self) -> &crate::SpecContentHash {
        self.claim.query_commitment()
    }

    /// Returns the original database-time authority observation for trusted
    /// in-process replay verification.
    #[must_use]
    pub(crate) const fn observed_at(&self) -> crate::Timestamp {
        self.claim.observed_at()
    }

    /// Returns the complete deterministic receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &crate::SpecContentHash {
        self.claim.receipt_commitment()
    }

    /// Returns the explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> CollaborativeApprovalAuthorityReceiptEffect {
        self.claim.effect()
    }

    /// Returns the point-in-time validity posture.
    #[must_use]
    pub const fn validity(&self) -> CollaborativeApprovalAuthorityReceiptValidity {
        self.claim.validity()
    }
}

impl<'de> Deserialize<'de> for PersistedCollaborativeApprovalAuthorityReceiptRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let claim = UnverifiedCollaborativeApprovalAuthorityReceipt::deserialize(deserializer)?;
        claim.validate_claim().map_err(|_| {
            serde::de::Error::custom("invalid persisted collaborative approval authority receipt")
        })?;
        Ok(Self { claim })
    }
}

impl std::fmt::Debug for PersistedCollaborativeApprovalAuthorityReceiptRecord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PersistedCollaborativeApprovalAuthorityReceiptRecord")
            .field("verification_posture", &self.verification_posture())
            .field("effect", &self.effect())
            .field("validity", &self.validity())
            .field("receipt_identity", &"[REDACTED]")
            .field("approval_reference", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Outcome of one create-only PostgreSQL receipt write.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollaborativeApprovalAuthorityReceiptWriteOutcome {
    /// The receipt was written for the first time.
    Written,
    /// The exact receipt already existed.
    AlreadyExists,
}
