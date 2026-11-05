//! Private local proof boundary for replacement-executor orientation.
//!
//! The proof compares two independently delivered, context-free orientations
//! from the same authoritative source state. It does not provide a runtime
//! consumer or authority. Fresh authority is resolved through a separate
//! injected boundary only after exact semantic equivalence is established.

use serde::Serialize;

use super::{
    collection_commitment, collection_error, cross_gate_lifecycle, delivery,
    ContinuationAggregateComposition, ContinuationCollectedWorkDisposition,
    ContinuationCollectedWorkIdentity, ContinuationControlDisposition,
};
use crate::continuation_context::{
    ContinuationContextDecisionPosture, ContinuationContextGap, ContinuationUnderstandingClaim,
};
use crate::{ActorId, SpecContentHash, WorkflowOsError};

const PROOF_VERSION: &str = "workflow-os/replacement-executor-proof/v1";

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReplacementExecutorOrientation {
    recipient: ActorId,
    projection_commitment: SpecContentHash,
    claims: Vec<ContinuationUnderstandingClaim>,
    gaps: Vec<ContinuationContextGap>,
    posture: ContinuationContextDecisionPosture,
    control_disposition: ContinuationControlDisposition,
    work_identities: Vec<ContinuationCollectedWorkIdentity>,
    delivery_commitment: SpecContentHash,
    receipt_commitment: SpecContentHash,
    non_authorizing: bool,
}

impl ReplacementExecutorOrientation {
    fn receive<R: delivery::ContinuationProjectionRecipient>(
        aggregate: &ContinuationAggregateComposition,
        recipient: &R,
    ) -> Result<Self, WorkflowOsError> {
        let (delivered, receipt) = delivery::deliver_exact_projection(aggregate, recipient)?;
        if !delivered.is_non_authorizing() || !receipt.is_non_authorizing() {
            return Err(proof_error(
                "orientation.authority_invalid",
                "replacement orientation and receipt must remain non-authorizing",
            ));
        }
        Ok(Self {
            recipient: delivered.recipient().clone(),
            projection_commitment: delivered.understanding().commitment().clone(),
            claims: delivered.understanding().claims().to_vec(),
            gaps: delivered.gaps().to_vec(),
            posture: delivered.decision_posture(),
            control_disposition: delivered.control_disposition(),
            work_identities: delivered.work_identities().to_vec(),
            delivery_commitment: delivered.commitment().clone(),
            receipt_commitment: receipt.commitment().clone(),
            non_authorizing: true,
        })
    }

    pub(super) fn claims(&self) -> &[ContinuationUnderstandingClaim] {
        &self.claims
    }

    pub(super) fn gaps(&self) -> &[ContinuationContextGap] {
        &self.gaps
    }

    pub(super) const fn posture(&self) -> ContinuationContextDecisionPosture {
        self.posture
    }

    pub(super) const fn is_non_authorizing(&self) -> bool {
        self.non_authorizing
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReplacementFreshAuthorityRequest {
    scope_commitment: SpecContentHash,
    aggregate_proof_commitment: SpecContentHash,
    decision_commitment: SpecContentHash,
    actor: ActorId,
    operation_identity: SpecContentHash,
    cross_gate_lifecycle_commitment: Option<SpecContentHash>,
    commitment: SpecContentHash,
    non_authorizing: bool,
}

impl ReplacementFreshAuthorityRequest {
    fn new(
        aggregate: &ContinuationAggregateComposition,
        actor: ActorId,
        operation_identity: SpecContentHash,
        cross_gate_lifecycle: Option<&cross_gate_lifecycle::CrossGateLifecycleBinding>,
    ) -> Result<Self, WorkflowOsError> {
        let scope_commitment =
            collection_commitment("replacement-authority-scope", &aggregate.scope)?;
        let aggregate_proof_commitment = aggregate.proof.commitment.clone();
        let decision_commitment = aggregate.decision.commitment().clone();
        let commitment = collection_commitment(
            "replacement-fresh-authority-request",
            &(
                &scope_commitment,
                &aggregate_proof_commitment,
                &decision_commitment,
                &actor,
                &operation_identity,
                cross_gate_lifecycle
                    .map(cross_gate_lifecycle::CrossGateLifecycleBinding::commitment),
                true,
            ),
        )?;
        Ok(Self {
            scope_commitment,
            aggregate_proof_commitment,
            decision_commitment,
            actor,
            operation_identity,
            cross_gate_lifecycle_commitment: cross_gate_lifecycle
                .map(|binding| binding.commitment().clone()),
            commitment,
            non_authorizing: true,
        })
    }

    pub(super) const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    pub(super) const fn operation_identity(&self) -> &SpecContentHash {
        &self.operation_identity
    }

    pub(super) const fn cross_gate_lifecycle_commitment(&self) -> Option<&SpecContentHash> {
        self.cross_gate_lifecycle_commitment.as_ref()
    }
}

pub(super) trait ReplacementExecutorFreshAuthorityResolver {
    /// Resolves current authority for this exact payload-free request only.
    /// The request is non-authorizing and no authority payload is returned.
    fn resolve_fresh(
        &self,
        request: &ReplacementFreshAuthorityRequest,
    ) -> Result<(), WorkflowOsError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReplacementFreshAuthorityPosture {
    NotRequestedBlocked,
    NotRequestedPaused,
    NotRequestedStopped,
    ResolvedSeparately,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReplacementExecutorProof {
    version: &'static str,
    original_projection_commitment: SpecContentHash,
    replacement_projection_commitment: SpecContentHash,
    original_delivery_commitment: SpecContentHash,
    replacement_delivery_commitment: SpecContentHash,
    original_receipt_commitment: SpecContentHash,
    replacement_receipt_commitment: SpecContentHash,
    gap_count: usize,
    authority_request_commitment: Option<SpecContentHash>,
    authority_posture: ReplacementFreshAuthorityPosture,
    commitment: SpecContentHash,
    non_authorizing: bool,
}

impl ReplacementExecutorProof {
    pub(super) const fn authority_posture(&self) -> ReplacementFreshAuthorityPosture {
        self.authority_posture
    }

    pub(super) const fn is_non_authorizing(&self) -> bool {
        self.non_authorizing
    }
}

pub(super) fn prove_replacement_executor_equivalence<O, N, A>(
    original_aggregate: &ContinuationAggregateComposition,
    replacement_aggregate: &ContinuationAggregateComposition,
    original_recipient: &O,
    replacement_recipient: &N,
    authority: &A,
) -> Result<
    (
        ReplacementExecutorOrientation,
        ReplacementExecutorOrientation,
        ReplacementExecutorProof,
    ),
    WorkflowOsError,
>
where
    O: delivery::ContinuationProjectionRecipient,
    N: delivery::ContinuationProjectionRecipient,
    A: ReplacementExecutorFreshAuthorityResolver,
{
    if original_aggregate.scope != replacement_aggregate.scope
        || original_aggregate.proof.commitment != replacement_aggregate.proof.commitment
    {
        return Err(proof_error(
            "aggregate.mismatch",
            "original and replacement orientations must derive from the same exact aggregate",
        ));
    }
    let original = ReplacementExecutorOrientation::receive(original_aggregate, original_recipient)?;
    let replacement =
        ReplacementExecutorOrientation::receive(replacement_aggregate, replacement_recipient)?;

    if original.recipient == replacement.recipient {
        return Err(proof_error(
            "recipient.not_distinct",
            "replacement proof requires distinct original and replacement recipients",
        ));
    }

    if original.projection_commitment != replacement.projection_commitment
        || original.claims != replacement.claims
        || original.gaps != replacement.gaps
        || original.posture != replacement.posture
        || original.control_disposition != replacement.control_disposition
        || original.work_identities != replacement.work_identities
    {
        return Err(proof_error(
            "understanding.mismatch",
            "replacement executor did not receive the same bounded understanding",
        ));
    }
    complete_replacement_proof(
        original_aggregate,
        replacement_aggregate,
        original,
        replacement,
        None,
        authority,
    )
}

pub(super) fn prove_cross_gate_replacement_executor_equivalence<O, N, A>(
    original_aggregate: &ContinuationAggregateComposition,
    replacement_aggregate: &ContinuationAggregateComposition,
    original_recipient: &O,
    replacement_recipient: &N,
    original_lifecycle: &cross_gate_lifecycle::CrossGateLifecycleBinding,
    replacement_lifecycle: &cross_gate_lifecycle::CrossGateLifecycleBinding,
    authority: &A,
) -> Result<
    (
        ReplacementExecutorOrientation,
        ReplacementExecutorOrientation,
        ReplacementExecutorProof,
    ),
    WorkflowOsError,
>
where
    O: delivery::ContinuationProjectionRecipient,
    N: delivery::ContinuationProjectionRecipient,
    A: ReplacementExecutorFreshAuthorityResolver,
{
    if original_lifecycle != replacement_lifecycle
        || !original_lifecycle.is_non_authorizing()
        || !replacement_lifecycle.is_non_authorizing()
    {
        return Err(proof_error(
            "lifecycle_binding.mismatch",
            "replacement authority requires one exact non-authorizing cross-gate lifecycle binding",
        ));
    }
    if original_aggregate.scope.run_id.as_ref() != Some(original_lifecycle.run_id())
        || replacement_aggregate.scope.run_id.as_ref() != Some(replacement_lifecycle.run_id())
    {
        return Err(proof_error(
            "lifecycle_binding.scope_mismatch",
            "cross-gate lifecycle binding must identify the exact run-bound continuation scope",
        ));
    }
    if original_aggregate.scope != replacement_aggregate.scope
        || original_aggregate.proof.commitment != replacement_aggregate.proof.commitment
    {
        return Err(proof_error(
            "aggregate.mismatch",
            "original and replacement orientations must derive from the same exact aggregate",
        ));
    }
    let original = ReplacementExecutorOrientation::receive(original_aggregate, original_recipient)?;
    let replacement =
        ReplacementExecutorOrientation::receive(replacement_aggregate, replacement_recipient)?;
    if original.recipient == replacement.recipient {
        return Err(proof_error(
            "recipient.not_distinct",
            "replacement proof requires distinct original and replacement recipients",
        ));
    }
    if original.projection_commitment != replacement.projection_commitment
        || original.claims != replacement.claims
        || original.gaps != replacement.gaps
        || original.posture != replacement.posture
        || original.control_disposition != replacement.control_disposition
        || original.work_identities != replacement.work_identities
    {
        return Err(proof_error(
            "understanding.mismatch",
            "replacement executor did not receive the same bounded understanding",
        ));
    }
    complete_replacement_proof(
        original_aggregate,
        replacement_aggregate,
        original,
        replacement,
        Some(replacement_lifecycle),
        authority,
    )
}

fn complete_replacement_proof<A: ReplacementExecutorFreshAuthorityResolver>(
    original_aggregate: &ContinuationAggregateComposition,
    replacement_aggregate: &ContinuationAggregateComposition,
    original: ReplacementExecutorOrientation,
    replacement: ReplacementExecutorOrientation,
    cross_gate_lifecycle: Option<&cross_gate_lifecycle::CrossGateLifecycleBinding>,
    authority: &A,
) -> Result<
    (
        ReplacementExecutorOrientation,
        ReplacementExecutorOrientation,
        ReplacementExecutorProof,
    ),
    WorkflowOsError,
> {
    debug_assert_eq!(original_aggregate.scope, replacement_aggregate.scope);
    let (authority_request, authority_posture) =
        authority_request_for(replacement_aggregate, &replacement, cross_gate_lifecycle)?;
    let commitment = collection_commitment(
        "replacement-executor-proof",
        &(
            PROOF_VERSION,
            &original.projection_commitment,
            &replacement.projection_commitment,
            &original.delivery_commitment,
            &replacement.delivery_commitment,
            &original.receipt_commitment,
            &replacement.receipt_commitment,
            original.gaps.len(),
            authority_request
                .as_ref()
                .map(ReplacementFreshAuthorityRequest::commitment),
            authority_posture,
            true,
        ),
    )?;
    let proof = ReplacementExecutorProof {
        version: PROOF_VERSION,
        original_projection_commitment: original.projection_commitment.clone(),
        replacement_projection_commitment: replacement.projection_commitment.clone(),
        original_delivery_commitment: original.delivery_commitment.clone(),
        replacement_delivery_commitment: replacement.delivery_commitment.clone(),
        original_receipt_commitment: original.receipt_commitment.clone(),
        replacement_receipt_commitment: replacement.receipt_commitment.clone(),
        gap_count: original.gaps.len(),
        authority_request_commitment: authority_request
            .as_ref()
            .map(|request| request.commitment().clone()),
        authority_posture,
        commitment,
        non_authorizing: true,
    };
    if let Some(request) = authority_request.as_ref() {
        authority.resolve_fresh(request)?;
    }
    Ok((original, replacement, proof))
}

fn authority_request_for(
    aggregate: &ContinuationAggregateComposition,
    orientation: &ReplacementExecutorOrientation,
    cross_gate_lifecycle: Option<&cross_gate_lifecycle::CrossGateLifecycleBinding>,
) -> Result<
    (
        Option<ReplacementFreshAuthorityRequest>,
        ReplacementFreshAuthorityPosture,
    ),
    WorkflowOsError,
> {
    if orientation.posture != ContinuationContextDecisionPosture::ReadyForAuthorityResolution {
        return Ok((None, ReplacementFreshAuthorityPosture::NotRequestedBlocked));
    }
    match orientation.control_disposition {
        ContinuationControlDisposition::Pause | ContinuationControlDisposition::Stop
            if orientation.work_identities.iter().any(|item| {
                item.disposition == ContinuationCollectedWorkDisposition::LawfulNext
            }) =>
        {
            Err(proof_error(
                "control.identity_conflict",
                "pause or stop orientation cannot carry a lawful-next work identity",
            ))
        }
        ContinuationControlDisposition::Pause => {
            Ok((None, ReplacementFreshAuthorityPosture::NotRequestedPaused))
        }
        ContinuationControlDisposition::Stop => {
            Ok((None, ReplacementFreshAuthorityPosture::NotRequestedStopped))
        }
        ContinuationControlDisposition::None => Err(proof_error(
            "control.missing",
            "ready replacement orientation requires one current resume intent",
        )),
        ContinuationControlDisposition::Resume => {
            let lawful_next = orientation
                .work_identities
                .iter()
                .filter(|item| item.disposition == ContinuationCollectedWorkDisposition::LawfulNext)
                .collect::<Vec<_>>();
            let [operation] = lawful_next.as_slice() else {
                return Err(proof_error(
                    "operation.identity_invalid",
                    "resume orientation requires exactly one stable lawful-next identity",
                ));
            };
            if orientation.work_identities.iter().any(|item| {
                item.disposition == ContinuationCollectedWorkDisposition::Completed
                    && item.identity == operation.identity
            }) {
                return Err(proof_error(
                    "next_action.repeats_completed",
                    "lawful next action identity is already recorded as completed",
                ));
            }
            let cross_gate_lifecycle = cross_gate_lifecycle.ok_or_else(|| {
                proof_error(
                    "lifecycle_binding.missing",
                    "fresh replacement authority requires the current cross-gate lifecycle binding",
                )
            })?;
            if cross_gate_lifecycle.lawful_next_operation_identity() != &operation.identity {
                return Err(proof_error(
                    "lifecycle_binding.operation_mismatch",
                    "cross-gate lifecycle binding identifies another lawful-next operation",
                ));
            }
            let request = ReplacementFreshAuthorityRequest::new(
                aggregate,
                orientation.recipient.clone(),
                operation.identity.clone(),
                Some(cross_gate_lifecycle),
            )?;
            Ok((
                Some(request),
                ReplacementFreshAuthorityPosture::ResolvedSeparately,
            ))
        }
    }
}

fn proof_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    collection_error(&format!("replacement_executor.{code}"), message)
}
