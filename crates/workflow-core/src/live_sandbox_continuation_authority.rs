//! Payload-free binding for a live sandbox continuation grant. Verification
//! must use facts resolved under the admission transaction, not a tool's claims.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::provider_write::GitHubCommentContinuationAction;
use crate::{
    Action, ActorId, ApprovalDecision, ApprovalDecisionKind, ApprovalPresentationRecord,
    ApprovalReferenceId, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, Capability,
    CapabilityDelegationPosture, CapabilityGrant, CapabilityGrantLifecycle,
    CapabilityGrantRequirements, CapabilityGrantScope, CapabilityResourceKind, EventId,
    GitHubPullRequestCommentTarget, PolicyDecision, SpecContentHash, Timestamp,
    WorkReportSensitivity, WorkflowOsError,
};

pub(crate) const LIVE_CAPABILITY: &str = "live-sandbox/github-pr-comment-continuation";

/// Exact target selected by the trusted operator, not by a replacement harness.
/// No public decoder, provider credentials, or repository-wide authorization.
pub(crate) struct LiveSandboxPolicyTarget {
    target: GitHubPullRequestCommentTarget,
    operator: ActorId,
    revision: SpecContentHash,
    valid_until: Timestamp,
}

impl LiveSandboxPolicyTarget {
    pub(crate) fn provision(
        target: GitHubPullRequestCommentTarget,
        operator: ActorId,
        revision: SpecContentHash,
        valid_until: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        target.validate()?;
        Ok(Self {
            target,
            operator,
            revision,
            valid_until,
        })
    }
}

/// Provisioned by the trusted operator, never deserialized from a harness.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LiveSandboxPrincipalVerifier {
    principal: ActorId,
    harness: crate::HarnessContractId,
    digest: [u8; 32],
}

impl LiveSandboxPrincipalVerifier {
    pub(crate) fn new(
        principal: ActorId,
        harness: crate::HarnessContractId,
        credential: &[u8; 32],
    ) -> Self {
        let digest = Self::derive(&principal, &harness, credential);
        Self {
            principal,
            harness,
            digest,
        }
    }

    pub(crate) const fn principal(&self) -> &ActorId {
        &self.principal
    }

    pub(crate) const fn harness(&self) -> &crate::HarnessContractId {
        &self.harness
    }

    pub(crate) fn authenticate(&self, credential: &[u8; 32]) -> Result<(), WorkflowOsError> {
        let derived = Self::derive(&self.principal, &self.harness, credential);
        if bool::from(self.digest.ct_eq(&derived)) {
            Ok(())
        } else {
            Err(denied())
        }
    }

    pub(crate) fn commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        let mut hash = Sha256::new();
        hash.update(b"workflow-os/live-sandbox-registration/v1\0");
        hash.update(serde_json::to_vec(self).map_err(|_| denied())?);
        SpecContentHash::new(format!("{:x}", hash.finalize()))
    }

    fn derive(
        principal: &ActorId,
        harness: &crate::HarnessContractId,
        credential: &[u8; 32],
    ) -> [u8; 32] {
        let mut hash = Sha256::new();
        hash.update(b"workflow-os/live-sandbox-principal/v1\0");
        hash.update(principal.as_str().as_bytes());
        hash.update(b"\0");
        hash.update(harness.as_str().as_bytes());
        hash.update(b"\0");
        hash.update(credential);
        hash.finalize().into()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct LiveSandboxContinuationFacts<'a> {
    pub(crate) action: GitHubCommentContinuationAction<'a>,
    pub(crate) policy_target: &'a LiveSandboxPolicyTarget,
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
    pub(crate) sensitivity: WorkReportSensitivity,
}

pub(crate) struct ResolvedLiveEffect {
    pub(crate) evidence: SpecContentHash,
    pub(crate) policy: SpecContentHash,
    pub(crate) sensitivity: WorkReportSensitivity,
}

impl LiveSandboxContinuationFacts<'_> {
    /// Evaluate only this private capability; the general policy engine is unchanged.
    pub(crate) fn scoped_policy_decision(
        &self,
        now: Timestamp,
    ) -> Result<PolicyDecision, WorkflowOsError> {
        self.verify_exact_approval(now)?;
        let target = self.policy_target;
        if &target.target != self.action.target
            || target.operator != self.decision.actor
            || self.replacement == self.action.original_actor
            || self.replacement == &target.operator
            || self.window_expires_at > target.valid_until
            || now >= self.window_expires_at
        {
            return Err(denied());
        }
        let material = serde_json::to_vec(&(
            self.action.commitment()?,
            (
                &target.target,
                &target.operator,
                &target.revision,
                target.valid_until,
            ),
            (
                self.approval,
                self.approval_event,
                self.presentation.content_hash(),
                self.decision.decided_at,
                self.approval_expires_at,
            ),
            (
                self.replacement,
                self.scope,
                self.window,
                self.attempt,
                self.window_expires_at,
            ),
            self.requirements,
        ))
        .map_err(|_| denied())?;
        let mut hash = Sha256::new();
        hash.update(b"workflow-os/live-sandbox-comment-policy/v1\0");
        hash.update(material);
        Ok(PolicyDecision {
            allowed: true,
            requires_approval: true,
            reason_codes: vec![format!("live_sandbox_comment_scope/{:x}", hash.finalize())],
            violations: Vec::new(),
            action: Action::InvokeAdapter,
            capabilities: vec![Capability::ExternalWrite, Capability::AdapterInvoke],
            actor: Some(self.action.original_actor.clone()),
            workflow_id: Some(self.scope.workflow_id().clone()),
            run_id: Some(self.action.run.clone()),
            correlation_id: Some(self.decision.correlation_id.clone()),
        })
    }

    pub(crate) fn verify_exact_approval(
        &self,
        observed_at: Timestamp,
    ) -> Result<(), WorkflowOsError> {
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
                .is_some_and(|expiry| observed_at >= expiry)
            || self.decision.decided_at > observed_at
            || self.presentation.requested_action() != self.action.approval_label()?
        {
            return Err(denied());
        }
        Ok(())
    }

    pub(crate) fn grant_commitment(
        &self,
        grant: &CapabilityGrant,
        observed_at: Timestamp,
        registration: &SpecContentHash,
        effect: &ResolvedLiveEffect,
    ) -> Result<SpecContentHash, WorkflowOsError> {
        self.verify_exact_approval(observed_at)?;
        let expires_at = grant.expires_at().ok_or_else(denied)?;
        let sensitivity = self.sensitivity.max(effect.sensitivity);
        if self
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
            || sensitivity == WorkReportSensitivity::Unknown
            || sensitivity > grant.sensitivity_ceiling()
            || grant.issued_at() < self.decision.decided_at
            || grant.issued_at() > observed_at
            || observed_at >= expires_at
            || expires_at > self.window_expires_at
        {
            return Err(denied());
        }

        let action = self.action.commitment()?;
        let bytes = serde_json::to_vec(&(
            (grant, action, sensitivity),
            (self.approval_event, self.decision, self.approval_expires_at),
            (self.window, self.attempt, self.window_expires_at),
            (&effect.evidence, &effect.policy, registration),
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
        registration: &SpecContentHash,
        effect: &ResolvedLiveEffect,
        expected: &SpecContentHash,
    ) -> Result<(), WorkflowOsError> {
        if self.grant_commitment(grant, observed_at, registration, effect)? != *expected {
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
pub(crate) mod tests {
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

    #[derive(Clone, Copy)]
    pub(crate) struct LiveGrantTestContext<'a> {
        pub(crate) action: GitHubCommentContinuationAction<'a>,
        pub(crate) workflow: &'a WorkflowId,
        pub(crate) window: &'a AuthorizedExecutionWindowId,
        pub(crate) attempt: &'a AuthorizedExecutionAttemptId,
        pub(crate) now: Timestamp,
        pub(crate) expiry: Timestamp,
        pub(crate) window_expiry: Timestamp,
    }

    pub(crate) fn with_live_grant_test_facts<T>(
        context: LiveGrantTestContext<'_>,
        test: impl FnOnce(
            LiveSandboxContinuationFacts<'_>,
            &dyn Fn(&str, CapabilityGrantLifecycle) -> CapabilityGrant,
        ) -> T,
    ) -> T {
        let LiveGrantTestContext {
            action,
            workflow,
            window,
            attempt,
            now,
            expiry,
            window_expiry,
        } = context;
        let scope = CapabilityGrantScope::new(
            workflow.clone(),
            Some(action.run.clone()),
            Some(action.step.clone()),
            Some(HarnessContractId::new("harness/replacement").expect("harness")),
        )
        .expect("scope");
        let replacement = ActorId::new("agent/replacement").expect("replacement");
        let issuer = ActorId::new("user/operator").expect("issuer");
        let approval =
            ApprovalReferenceId::new(format!("approval/{}/comment", action.run)).expect("approval");
        let requirements = CapabilityGrantRequirements::new(
            vec![],
            vec![approval.clone()],
            vec![],
            vec![crate::LocalCheckResultId::new("check/live-sandbox").expect("check")],
        )
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
        let policy_target = LiveSandboxPolicyTarget::provision(
            action.target.clone(),
            issuer.clone(),
            SpecContentHash::from_text("test sandbox target"),
            window_expiry,
        )
        .expect("test sandbox policy target");
        let facts = LiveSandboxContinuationFacts {
            action,
            policy_target: &policy_target,
            replacement: &replacement,
            scope: &scope,
            approval: &approval,
            approval_event: &EventId::new("event/approved").expect("event"),
            decision: &decision,
            presentation: &presentation,
            approval_expires_at: Some(expiry),
            window,
            attempt,
            window_expires_at: window_expiry,
            requirements: &requirements,
            sensitivity: WorkReportSensitivity::Internal,
        };
        let grant = |capability: &str, lifecycle: CapabilityGrantLifecycle| {
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
        test(facts, &grant)
    }

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
        let now = Timestamp::parse_rfc3339("2026-11-07T07:00:00Z").expect("issued");
        let expiry = Timestamp::parse_rfc3339("2026-11-07T07:04:00Z").expect("expiry");
        with_live_grant_test_facts(
            LiveGrantTestContext {
                action,
                workflow: &WorkflowId::new("test/live").expect("workflow"),
                window: &AuthorizedExecutionWindowId::new("window/live").expect("window"),
                attempt: &AuthorizedExecutionAttemptId::new("attempt/live").expect("attempt"),
                now,
                expiry,
                window_expiry: expiry,
            },
            |facts, grant| {
                let active = grant(LIVE_CAPABILITY, CapabilityGrantLifecycle::Active);
                let registration = SpecContentHash::from_text("registration one");
                let effect = ResolvedLiveEffect {
                    evidence: SpecContentHash::from_text("passing evidence"),
                    policy: SpecContentHash::from_text("exact policy"),
                    sensitivity: WorkReportSensitivity::Internal,
                };
                let binding = facts
                    .grant_commitment(&active, now, &registration, &effect)
                    .expect("binding");
                facts
                    .verify_bound_grant(&active, now, &registration, &effect, &binding)
                    .expect("exact recheck");
                assert!(facts
                    .verify_bound_grant(
                        &active,
                        now,
                        &SpecContentHash::from_text("changed registration"),
                        &effect,
                        &binding,
                    )
                    .is_err());
                for changed in [
                    LiveSandboxContinuationFacts {
                        action: GitHubCommentContinuationAction {
                            body: "changed body",
                            ..action
                        },
                        ..facts
                    },
                    LiveSandboxContinuationFacts {
                        replacement: action.original_actor,
                        ..facts
                    },
                    LiveSandboxContinuationFacts {
                        window: &AuthorizedExecutionWindowId::new("window/other")
                            .expect("other window"),
                        ..facts
                    },
                ] {
                    assert!(changed
                        .verify_bound_grant(&active, now, &registration, &effect, &binding)
                        .is_err());
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
                    assert!(facts
                        .verify_bound_grant(invalid, at, &registration, &effect, &binding)
                        .is_err());
                }
            },
        );
    }
}
