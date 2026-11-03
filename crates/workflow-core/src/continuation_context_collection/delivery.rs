use std::fmt;

use serde::Serialize;

use super::{
    collection_commitment, collection_error, ContinuationAggregateComposition,
    ContinuationCollectedWorkIdentity, ContinuationControlDisposition,
};
use crate::continuation_context::{
    ContinuationContextDecisionPosture, ContinuationContextGap, ContinuationContextPurpose,
    ContinuationContextScope, ContinuationUnderstandingProjection,
};
use crate::{ActorId, SpecContentHash, WorkflowOsError};

const DELIVERY_VERSION: &str = "workflow-os/continuation-projection-delivery/v1";

/// Exact bounded projection offered to one replacement-executor identity.
///
/// This is private orientation data. It carries no operation, approval,
/// capability, or execution authority.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ContinuationProjectionDelivery {
    version: &'static str,
    purpose: ContinuationContextPurpose,
    scope: ContinuationContextScope,
    source_actor: ActorId,
    recipient: ActorId,
    collection_request_commitment: SpecContentHash,
    aggregate_proof_commitment: SpecContentHash,
    decision_commitment: SpecContentHash,
    decision_posture: ContinuationContextDecisionPosture,
    gaps: Vec<ContinuationContextGap>,
    control_disposition: ContinuationControlDisposition,
    work_identities: Vec<ContinuationCollectedWorkIdentity>,
    understanding: ContinuationUnderstandingProjection,
    commitment: SpecContentHash,
    non_authorizing: bool,
}

impl fmt::Debug for ContinuationProjectionDelivery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationProjectionDelivery")
            .field("version", &self.version)
            .field("purpose", &self.purpose)
            .field("scope", &"[REDACTED]")
            .field("source_actor", &"[REDACTED]")
            .field("recipient", &"[REDACTED]")
            .field("collection_request_commitment", &"[REDACTED]")
            .field("aggregate_proof_commitment", &"[REDACTED]")
            .field("decision_commitment", &"[REDACTED]")
            .field("decision_posture", &self.decision_posture)
            .field("gap_count", &self.gaps.len())
            .field("control_disposition", &self.control_disposition)
            .field("work_identity_count", &self.work_identities.len())
            .field("claim_count", &self.understanding.claims().len())
            .field("commitment", &"[REDACTED]")
            .field("non_authorizing", &self.non_authorizing)
            .finish()
    }
}

impl ContinuationProjectionDelivery {
    pub(super) fn new(
        aggregate: &ContinuationAggregateComposition,
        recipient: ActorId,
    ) -> Result<Self, WorkflowOsError> {
        let purpose = aggregate.purpose;
        let scope = aggregate.scope.clone();
        let source_actor = aggregate.source_actor.clone();
        let collection_request_commitment = aggregate.proof.request_commitment.clone();
        let aggregate_proof_commitment = aggregate.proof.commitment.clone();
        let decision_commitment = aggregate.decision.commitment().clone();
        let decision_posture = aggregate.decision.posture();
        let gaps = aggregate.decision.gaps().to_vec();
        let control_disposition = aggregate.control_disposition;
        let work_identities = aggregate.work_identities.clone();
        let understanding = aggregate.decision.understanding().clone();
        let commitment = collection_commitment(
            "projection-delivery",
            &(
                DELIVERY_VERSION,
                purpose,
                &scope,
                &source_actor,
                &recipient,
                &collection_request_commitment,
                &aggregate_proof_commitment,
                &decision_commitment,
                decision_posture,
                &gaps,
                control_disposition,
                &work_identities,
                understanding.commitment(),
                true,
            ),
        )?;
        Ok(Self {
            version: DELIVERY_VERSION,
            purpose,
            scope,
            source_actor,
            recipient,
            collection_request_commitment,
            aggregate_proof_commitment,
            decision_commitment,
            decision_posture,
            gaps,
            control_disposition,
            work_identities,
            understanding,
            commitment,
            non_authorizing: true,
        })
    }

    pub(super) const fn recipient(&self) -> &ActorId {
        &self.recipient
    }

    pub(super) const fn scope(&self) -> &ContinuationContextScope {
        &self.scope
    }

    pub(super) const fn decision_posture(&self) -> ContinuationContextDecisionPosture {
        self.decision_posture
    }

    pub(super) fn gaps(&self) -> &[ContinuationContextGap] {
        &self.gaps
    }

    pub(super) const fn control_disposition(&self) -> ContinuationControlDisposition {
        self.control_disposition
    }

    pub(super) fn work_identities(&self) -> &[ContinuationCollectedWorkIdentity] {
        &self.work_identities
    }

    pub(super) const fn understanding(&self) -> &ContinuationUnderstandingProjection {
        &self.understanding
    }

    pub(super) const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    pub(super) const fn is_non_authorizing(&self) -> bool {
        self.non_authorizing
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ContinuationProjectionAcknowledgmentPosture {
    ExactProjectionReceived,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ContinuationProjectionAcknowledgmentNonClaim {
    NotClaimed,
}

/// Recipient-authored receipt for one exact delivered projection.
///
/// Receipt proves only that the exact projection commitment was returned by
/// the selected recipient boundary. It does not prove comprehension and it
/// cannot authorize execution.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ContinuationProjectionDeliveryReceipt {
    version: &'static str,
    delivery_commitment: SpecContentHash,
    projection_commitment: SpecContentHash,
    recipient: ActorId,
    posture: ContinuationProjectionAcknowledgmentPosture,
    comprehension: ContinuationProjectionAcknowledgmentNonClaim,
    execution_authority: ContinuationProjectionAcknowledgmentNonClaim,
    commitment: SpecContentHash,
    non_authorizing: bool,
}

impl fmt::Debug for ContinuationProjectionDeliveryReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationProjectionDeliveryReceipt")
            .field("version", &self.version)
            .field("delivery_commitment", &"[REDACTED]")
            .field("projection_commitment", &"[REDACTED]")
            .field("recipient", &"[REDACTED]")
            .field("posture", &self.posture)
            .field("comprehension", &self.comprehension)
            .field("execution_authority", &self.execution_authority)
            .field("commitment", &"[REDACTED]")
            .field("non_authorizing", &self.non_authorizing)
            .finish()
    }
}

impl ContinuationProjectionDeliveryReceipt {
    pub(super) fn acknowledge_exact(
        delivery: &ContinuationProjectionDelivery,
        recipient: ActorId,
    ) -> Result<Self, WorkflowOsError> {
        if &recipient != delivery.recipient() {
            return Err(delivery_error(
                "receipt.recipient_mismatch",
                "projection acknowledgment recipient does not match the delivery recipient",
            ));
        }
        let projection_commitment = delivery.understanding().commitment().clone();
        let commitment = receipt_commitment(
            delivery.commitment(),
            &projection_commitment,
            &recipient,
            ContinuationProjectionAcknowledgmentPosture::ExactProjectionReceived,
            ContinuationProjectionAcknowledgmentNonClaim::NotClaimed,
            ContinuationProjectionAcknowledgmentNonClaim::NotClaimed,
            true,
        )?;
        Ok(Self {
            version: DELIVERY_VERSION,
            delivery_commitment: delivery.commitment().clone(),
            projection_commitment,
            recipient,
            posture: ContinuationProjectionAcknowledgmentPosture::ExactProjectionReceived,
            comprehension: ContinuationProjectionAcknowledgmentNonClaim::NotClaimed,
            execution_authority: ContinuationProjectionAcknowledgmentNonClaim::NotClaimed,
            commitment,
            non_authorizing: true,
        })
    }

    fn validate_exact(
        &self,
        delivery: &ContinuationProjectionDelivery,
    ) -> Result<(), WorkflowOsError> {
        if self.version != DELIVERY_VERSION
            || self.delivery_commitment != *delivery.commitment()
            || self.projection_commitment != *delivery.understanding().commitment()
            || self.recipient != *delivery.recipient()
            || self.posture != ContinuationProjectionAcknowledgmentPosture::ExactProjectionReceived
            || self.comprehension != ContinuationProjectionAcknowledgmentNonClaim::NotClaimed
            || self.execution_authority != ContinuationProjectionAcknowledgmentNonClaim::NotClaimed
            || !self.non_authorizing
        {
            return Err(delivery_error(
                "receipt.binding_mismatch",
                "projection acknowledgment does not match the exact delivered projection",
            ));
        }
        let expected = receipt_commitment(
            &self.delivery_commitment,
            &self.projection_commitment,
            &self.recipient,
            self.posture,
            self.comprehension,
            self.execution_authority,
            self.non_authorizing,
        )?;
        if self.commitment != expected {
            return Err(delivery_error(
                "receipt.commitment_mismatch",
                "projection acknowledgment commitment is invalid",
            ));
        }
        Ok(())
    }

    pub(super) const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    pub(super) const fn is_non_authorizing(&self) -> bool {
        self.non_authorizing
    }

    #[cfg(test)]
    pub(super) fn substitute_delivery_commitment_for_test(&mut self, value: SpecContentHash) {
        self.delivery_commitment = value;
    }

    #[cfg(test)]
    pub(super) fn substitute_projection_commitment_for_test(&mut self, value: SpecContentHash) {
        self.projection_commitment = value;
    }

    #[cfg(test)]
    pub(super) fn substitute_recipient_for_test(&mut self, value: ActorId) {
        self.recipient = value;
    }

    #[cfg(test)]
    pub(super) fn substitute_receipt_commitment_for_test(&mut self, value: SpecContentHash) {
        self.commitment = value;
    }
}

pub(super) trait ContinuationProjectionRecipient {
    fn recipient(&self) -> &ActorId;

    fn acknowledge(
        &self,
        delivery: &ContinuationProjectionDelivery,
    ) -> Result<ContinuationProjectionDeliveryReceipt, WorkflowOsError>;
}

pub(super) fn deliver_exact_projection<R: ContinuationProjectionRecipient>(
    aggregate: &ContinuationAggregateComposition,
    recipient: &R,
) -> Result<
    (
        ContinuationProjectionDelivery,
        ContinuationProjectionDeliveryReceipt,
    ),
    WorkflowOsError,
> {
    let delivery = ContinuationProjectionDelivery::new(aggregate, recipient.recipient().clone())?;
    let receipt = recipient.acknowledge(&delivery)?;
    receipt.validate_exact(&delivery)?;
    Ok((delivery, receipt))
}

fn receipt_commitment(
    delivery_commitment: &SpecContentHash,
    projection_commitment: &SpecContentHash,
    recipient: &ActorId,
    posture: ContinuationProjectionAcknowledgmentPosture,
    comprehension: ContinuationProjectionAcknowledgmentNonClaim,
    execution_authority: ContinuationProjectionAcknowledgmentNonClaim,
    non_authorizing: bool,
) -> Result<SpecContentHash, WorkflowOsError> {
    collection_commitment(
        "projection-delivery-receipt",
        &(
            DELIVERY_VERSION,
            delivery_commitment,
            projection_commitment,
            recipient,
            posture,
            comprehension,
            execution_authority,
            non_authorizing,
        ),
    )
}

fn delivery_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    collection_error(&format!("delivery.{code}"), message)
}
