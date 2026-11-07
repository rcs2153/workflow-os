//! Payload-free binding for a live sandbox continuation grant. Verification
//! must use facts resolved under the admission transaction, not a tool's claims.

use sha2::{Digest, Sha256};

use crate::provider_write::GitHubCommentContinuationAction;
use crate::{
    ActorId, ApprovalDecision, ApprovalDecisionKind, ApprovalPresentationRecord,
    ApprovalReferenceId, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId,
    CapabilityDelegationPosture, CapabilityGrant, CapabilityGrantLifecycle,
    CapabilityGrantRequirements, CapabilityGrantScope, CapabilityResourceKind, EventId,
    SpecContentHash, Timestamp, WorkReportSensitivity, WorkflowOsError,
};

const LIVE_CAPABILITY: &str = "live-sandbox/github-pr-comment-continuation";

#[derive(Clone, Copy)]
pub(crate) struct LiveSandboxContinuationFacts<'a> {
    pub(crate) action: GitHubCommentContinuationAction<'a>,
    pub(crate) replacement: &'a ActorId,
    pub(crate) scope: &'a CapabilityGrantScope,
    pub(crate) approval: &'a ApprovalReferenceId,
    pub(crate) approval_event: &'a EventId,
    pub(crate) decision: &'a ApprovalDecision,
    pub(crate) presentation: &'a ApprovalPresentationRecord,
    pub(crate) approval_expires_at: Option<Timestamp>,
    pub(crate) window: &'a AuthorizedExecutionWindowId,
    pub(crate) attempt: &'a AuthorizedExecutionAttemptId,
    pub(crate) window_expires_at: Timestamp,
    pub(crate) requirements: &'a CapabilityGrantRequirements,
    pub(crate) evidence_commitment: &'a SpecContentHash,
    pub(crate) policy_commitment: &'a SpecContentHash,
    pub(crate) principal_registration_commitment: &'a SpecContentHash,
    pub(crate) sensitivity: WorkReportSensitivity,
}

impl LiveSandboxContinuationFacts<'_> {
    pub(crate) fn grant_commitment(
        &self,
        grant: &CapabilityGrant,
        observed_at: Timestamp,
    ) -> Result<SpecContentHash, WorkflowOsError> {
        let expires_at = grant.expires_at().ok_or_else(denied)?;
        let proof = self.decision.proof_marker.as_ref().ok_or_else(denied)?;
        let freshness_ms = proof.proof_freshness_limit_ms().ok_or_else(denied)?;
        let presentation_age = observed_at.as_offset_date_time()
            - self.presentation.presented_at().as_offset_date_time();
        if self.scope.run_id() != Some(self.action.run)
            || self.scope.step_id() != Some(self.action.step)
            || self.scope.harness_contract_id().is_none()
            || self.decision.decision != ApprovalDecisionKind::Granted
            || self.decision.approval_id != self.approval.as_str()
            || self.presentation.approval_id() != self.approval.as_str()
            || self.presentation.run_id() != self.action.run
            || self.presentation.step_id() != Some(self.action.step)
            || self.presentation.workflow_id() != self.scope.workflow_id()
            || self.presentation.presentation_id() != proof.presentation_id()
            || self.presentation.content_hash() != proof.presentation_content_hash()
            || self.presentation.sensitivity() != proof.proof_record_sensitivity()
            || proof.proof_validated_at() < self.presentation.presented_at()
            || proof.proof_validated_at() != self.decision.decided_at
            || presentation_age.is_negative()
            || presentation_age.unsigned_abs() > std::time::Duration::from_millis(freshness_ms)
            || self
                .approval_expires_at
                .is_some_and(|expiry| expires_at > expiry)
            || self.replacement == self.action.original_actor
            || self.replacement == &self.decision.actor
            || grant.subject() != self.replacement
            || grant.issuer() != &self.decision.actor
            || grant.capability().as_str() != LIVE_CAPABILITY
            || grant.scope() != self.scope
            || grant.resource().kind() != CapabilityResourceKind::AdapterResource
            || grant.resource().reference() != self.action.target.reference()
            || grant.requirements() != self.requirements
            || !self.requirements.approval_ids().contains(self.approval)
            || grant.lifecycle() != CapabilityGrantLifecycle::Active
            || grant.delegation() != CapabilityDelegationPosture::Disabled
            || self.sensitivity == WorkReportSensitivity::Unknown
            || self.sensitivity > grant.sensitivity_ceiling()
            || grant.issued_at() < self.decision.decided_at
            || grant.issued_at() > observed_at
            || observed_at >= expires_at
            || expires_at > self.window_expires_at
        {
            return Err(denied());
        }

        let action = self.action.commitment()?;
        if self.presentation.requested_action() != self.action.approval_label()? {
            return Err(denied());
        }
        let bytes = serde_json::to_vec(&(
            (grant, action, self.sensitivity),
            (self.approval_event, self.decision, self.approval_expires_at),
            (self.window, self.attempt, self.window_expires_at),
            (
                self.evidence_commitment,
                self.policy_commitment,
                self.principal_registration_commitment,
            ),
        ))
        .map_err(|_| denied())?;
        let mut hash = Sha256::new();
        hash.update(b"workflow-os/github-pr-comment-live-continuation-grant/v1\0");
        hash.update(bytes);
        SpecContentHash::new(format!("{:x}", hash.finalize()))
    }

    pub(crate) fn verify_bound_grant(
        &self,
        grant: &CapabilityGrant,
        observed_at: Timestamp,
        expected: &SpecContentHash,
    ) -> Result<(), WorkflowOsError> {
        if self.grant_commitment(grant, observed_at)? != *expected {
            return Err(denied());
        }
        Ok(())
    }
}

fn denied() -> WorkflowOsError {
    WorkflowOsError::security(
        "live_sandbox_continuation.grant_denied",
        "live sandbox continuation grant does not match current authority",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::too_many_lines)]
mod tests {
    use super::*;
    use crate::{
        compute_approval_presentation_content_hash, ApprovalDecisionProofEnforcementMode,
        ApprovalDecisionProofMarker, ApprovalDecisionProofMarkerDefinition,
        ApprovalDecisionProofValidationPolicy, ApprovalPresentationChannel, ApprovalPresentationId,
        ApprovalPresentationRecordDefinition, ApprovalPresentationSensitivity,
        CapabilityGrantDefinition, CapabilityGrantId, CapabilityReference, CapabilityResourceScope,
        CorrelationId, GitHubPullRequestCommentTarget, GitHubPullRequestCommentWriteMode,
        HarnessContractId, IdempotencyKey, RedactionMetadata, SideEffectId, StepId, WorkflowId,
        WorkflowRunId,
    };

    #[test]
    fn live_grant_matches_exact_approval_window_evidence_and_principal_revision() {
        let action = GitHubCommentContinuationAction {
            run: &WorkflowRunId::new("run-live").expect("run"),
            step: &StepId::new("comment").expect("step"),
            bundle_root: &SpecContentHash::from_text("bundle"),
            original_actor: &ActorId::new("agent/original").expect("original"),
            side_effect: &SideEffectId::new("effect/comment").expect("effect"),
            target: &GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target"),
            body: "approved synthetic comment",
            idempotency_key: &IdempotencyKey::new("comment/run-live").expect("key"),
            mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
        };
        let scope = CapabilityGrantScope::new(
            WorkflowId::new("test/live").expect("workflow"),
            Some(action.run.clone()),
            Some(action.step.clone()),
            Some(HarnessContractId::new("harness/replacement").expect("harness")),
        )
        .expect("scope");
        let replacement = ActorId::new("agent/replacement").expect("replacement");
        let issuer = ActorId::new("user/operator").expect("issuer");
        let now = Timestamp::parse_rfc3339("2026-11-07T07:00:00Z").expect("issued");
        let expiry = Timestamp::parse_rfc3339("2026-11-07T07:04:00Z").expect("expiry");
        let approval = ApprovalReferenceId::new("approval/run-live/comment").expect("approval");
        let requirements =
            CapabilityGrantRequirements::new(vec![], vec![approval.clone()], vec![], vec![])
                .expect("requirements");
        let label = action.approval_label().expect("label");
        let sensitivity = ApprovalPresentationSensitivity::Internal;
        let content_hash = compute_approval_presentation_content_hash(
            action.run,
            approval.as_str(),
            scope.workflow_id(),
            None,
            None,
            Some(action.step),
            &label,
            "Synthetic sandbox proof",
            "One exact action",
            &["No broad writes".into()],
            &["Disposable pull request".into()],
            &["Verify target".into()],
            "Test exact binding",
            "Reserve exact attempt",
            &ApprovalPresentationChannel::Terminal,
            sensitivity,
        )
        .expect("presentation content");
        let presentation_id = ApprovalPresentationId::new("presentation/live").expect("id");
        let presentation = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
            presentation_id: presentation_id.clone(),
            run_id: action.run.clone(),
            approval_id: approval.as_str().into(),
            workflow_id: scope.workflow_id().clone(),
            workflow_version: None,
            schema_version: None,
            step_id: Some(action.step.clone()),
            requested_action: label,
            work_summary: "Synthetic sandbox proof".into(),
            approved_scope: "One exact action".into(),
            strict_non_goals: vec!["No broad writes".into()],
            expected_touched_surfaces: vec!["Disposable pull request".into()],
            validation_expectations: vec!["Verify target".into()],
            why_now: "Test exact binding".into(),
            next_action: "Reserve exact attempt".into(),
            presented_at: now,
            presented_by: issuer.clone(),
            channel: ApprovalPresentationChannel::Terminal,
            content_hash: content_hash.clone(),
            redaction: RedactionMetadata::empty(),
            sensitivity,
        })
        .expect("presentation");
        let decision = ApprovalDecision {
            approval_id: approval.as_str().into(),
            actor: issuer.clone(),
            decided_at: now,
            decision: ApprovalDecisionKind::Granted,
            reason: "Exact synthetic action reviewed".into(),
            correlation_id: CorrelationId::new("correlation/live").expect("correlation"),
            proof_marker: Some(
                ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
                    enforcement_mode:
                        ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
                    presentation_id,
                    presentation_content_hash: content_hash,
                    proof_validated_at: now,
                    proof_validation_policy:
                        ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
                    proof_age_ms: Some(0),
                    proof_freshness_limit_ms: Some(60_000),
                    proof_record_sensitivity: sensitivity,
                    redaction: RedactionMetadata::empty(),
                })
                .expect("marker"),
            ),
        };
        let facts = LiveSandboxContinuationFacts {
            action,
            replacement: &replacement,
            scope: &scope,
            approval: &approval,
            approval_event: &EventId::new("event/approved").expect("event"),
            decision: &decision,
            presentation: &presentation,
            approval_expires_at: Some(expiry),
            window: &AuthorizedExecutionWindowId::new("window/live").expect("window"),
            attempt: &AuthorizedExecutionAttemptId::new("attempt/live").expect("attempt"),
            window_expires_at: expiry,
            requirements: &requirements,
            evidence_commitment: &SpecContentHash::from_text("passing evidence"),
            policy_commitment: &SpecContentHash::from_text("exact policy"),
            principal_registration_commitment: &SpecContentHash::from_text("registration one"),
            sensitivity: WorkReportSensitivity::Internal,
        };
        let grant = |capability, lifecycle| {
            CapabilityGrant::new(CapabilityGrantDefinition {
                grant_id: CapabilityGrantId::new("grant/live").expect("grant"),
                subject: replacement.clone(),
                capability: CapabilityReference::new(capability).expect("capability"),
                resource: CapabilityResourceScope::new(
                    CapabilityResourceKind::AdapterResource,
                    action.target.reference(),
                )
                .expect("resource"),
                scope: scope.clone(),
                issuer: issuer.clone(),
                issued_at: now,
                expires_at: Some(expiry),
                lifecycle,
                revocation_reference: (lifecycle == CapabilityGrantLifecycle::Revoked)
                    .then(|| "revocation/live".into()),
                delegation: CapabilityDelegationPosture::Disabled,
                requirements: requirements.clone(),
                sensitivity_ceiling: WorkReportSensitivity::Internal,
                redaction: RedactionMetadata::empty(),
            })
            .expect("grant")
        };
        let active = grant(LIVE_CAPABILITY, CapabilityGrantLifecycle::Active);
        let binding = facts.grant_commitment(&active, now).expect("binding");
        facts
            .verify_bound_grant(&active, now, &binding)
            .expect("exact recheck");
        for changed in [
            LiveSandboxContinuationFacts {
                action: GitHubCommentContinuationAction {
                    body: "changed body",
                    ..action
                },
                ..facts
            },
            LiveSandboxContinuationFacts {
                evidence_commitment: &SpecContentHash::from_text("changed evidence"),
                ..facts
            },
            LiveSandboxContinuationFacts {
                policy_commitment: &SpecContentHash::from_text("changed policy"),
                ..facts
            },
            LiveSandboxContinuationFacts {
                principal_registration_commitment: &SpecContentHash::from_text(
                    "changed registration",
                ),
                ..facts
            },
            LiveSandboxContinuationFacts {
                replacement: action.original_actor,
                ..facts
            },
            LiveSandboxContinuationFacts {
                window: &AuthorizedExecutionWindowId::new("window/other").expect("other window"),
                ..facts
            },
        ] {
            assert!(changed.verify_bound_grant(&active, now, &binding).is_err());
        }
        let fixture = grant(
            "fixture/github-pr-comment-continuation",
            CapabilityGrantLifecycle::Active,
        );
        let revoked = grant(LIVE_CAPABILITY, CapabilityGrantLifecycle::Revoked);
        let stale = Timestamp::parse_rfc3339("2026-11-07T07:01:01Z").expect("stale");
        for (invalid, at) in [
            (&fixture, now),
            (&revoked, now),
            (&active, stale),
            (&active, expiry),
        ] {
            assert!(facts.verify_bound_grant(invalid, at, &binding).is_err());
        }
    }
}
