//! Exact authority matching for the local, non-dispatching fixture boundary.
//!
//! Neither a registered verifier nor a matching grant proves durable current
//! state. The caller must obtain both from the trusted store and recheck that
//! state at the fixture's final read boundary.

use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::{
    ActorId, ApprovalPresentationContentHash, ApprovalPresentationId, ApprovalReferenceId,
    AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, CapabilityDelegationPosture,
    CapabilityGrant, CapabilityGrantLifecycle, CapabilityGrantRequirements, CapabilityGrantScope,
    CapabilityResourceScope, EventId, HarnessContractId, SideEffectId, SpecContentHash, Timestamp,
    WorkReportSensitivity, WorkflowOsError,
};

const CAPABILITY: &str = "fixture/github-pr-comment-continuation";

/// Durable verifier for one operator-provisioned local connector principal.
/// The store must bind its identity and current revision separately.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalFixturePrincipalVerifier {
    principal: ActorId,
    harness: HarnessContractId,
    digest: [u8; 32],
}

impl LocalFixturePrincipalVerifier {
    /// Derives a verifier from a high-entropy credential obtained from a trusted
    /// operator, not from model input. No raw credential is retained.
    #[must_use]
    pub fn new(principal: ActorId, harness: HarnessContractId, credential: &[u8; 32]) -> Self {
        let digest = credential_digest(&principal, &harness, credential);
        Self {
            principal,
            harness,
            digest,
        }
    }

    pub(crate) const fn principal(&self) -> &ActorId {
        &self.principal
    }

    pub(crate) const fn harness(&self) -> &HarnessContractId {
        &self.harness
    }

    /// Resolves the principal from its credential without accepting an actor label.
    ///
    /// # Errors
    ///
    /// Rejects a credential that does not match this verifier.
    pub fn authenticate(
        &self,
        credential: &[u8; 32],
    ) -> Result<VerifiedLocalFixturePrincipal<'_>, WorkflowOsError> {
        let candidate = credential_digest(&self.principal, &self.harness, credential);
        if bool::from(self.digest.ct_eq(&candidate)) {
            Ok(VerifiedLocalFixturePrincipal(self))
        } else {
            Err(authority_denied())
        }
    }
}

impl fmt::Debug for LocalFixturePrincipalVerifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("LocalFixturePrincipalVerifier([REDACTED])")
    }
}

/// In-memory proof of credential possession. It has no wire representation.
pub struct VerifiedLocalFixturePrincipal<'a>(&'a LocalFixturePrincipalVerifier);

impl VerifiedLocalFixturePrincipal<'_> {
    /// Returns whether the verifier is still the exact registration used.
    #[must_use]
    pub fn matches_registration(&self, current: &LocalFixturePrincipalVerifier) -> bool {
        self.0.principal == current.principal
            && self.0.harness == current.harness
            && bool::from(self.0.digest.ct_eq(&current.digest))
    }
}

fn credential_digest(
    principal: &ActorId,
    harness: &HarnessContractId,
    credential: &[u8; 32],
) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"workflow-os/local-fixture-principal/v1\0");
    hash.update(principal.as_str().as_bytes());
    hash.update(b"\0");
    hash.update(harness.as_str().as_bytes());
    hash.update(credential);
    hash.finalize().into()
}

/// Immutable facts recovered by the kernel for the original approved fixture.
#[derive(Clone, Copy, Serialize)]
pub struct LocalFixtureReplacementIntent<'a> {
    /// Original approved actor, not the connector principal.
    pub original_actor: &'a ActorId,
    /// Exact workflow, run, step, and replacement harness contract scope.
    pub scope: &'a CapabilityGrantScope,
    /// Exact provider target; validation alone cannot use it to dispatch.
    pub resource: &'a CapabilityResourceScope,
    /// Immutable run-bundle root commitment.
    pub bundle_root: &'a SpecContentHash,
    /// Commitment to the original request, including its original actor.
    pub action_commitment: &'a SpecContentHash,
    /// Durable approval reference.
    pub approval_id: &'a ApprovalReferenceId,
    /// Durable event that granted the original approval.
    pub approval_decision_event_id: &'a EventId,
    /// Durable approval-presentation reference.
    pub presentation_id: &'a ApprovalPresentationId,
    /// Exact content presented at approval.
    pub presentation_hash: &'a ApprovalPresentationContentHash,
    /// Exact attempted effect.
    pub side_effect_id: &'a SideEffectId,
    /// Opened operational window.
    pub window_id: &'a AuthorizedExecutionWindowId,
    /// Attempt to which the window is bound.
    pub attempt_id: &'a AuthorizedExecutionAttemptId,
    /// Required sensitivity resolved from the bound fixture context.
    pub sensitivity: WorkReportSensitivity,
}

impl LocalFixtureReplacementIntent<'_> {
    /// Derives a versioned payload-free commitment for durable storage.
    ///
    /// # Errors
    ///
    /// Rejects an unbounded scope or an intent that cannot be encoded.
    pub fn commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        if self.scope.run_id().is_none()
            || self.scope.step_id().is_none()
            || self.scope.harness_contract_id().is_none()
        {
            return Err(authority_denied());
        }
        let bytes = serde_json::to_vec(self).map_err(|_| authority_denied())?;
        let mut hash = Sha256::new();
        hash.update(b"workflow-os/local-fixture-replacement-intent/v1\0");
        hash.update(bytes);
        SpecContentHash::new(format!("{:x}", hash.finalize()))
    }
}

/// Current facts for checking one exact fixture grant. The kernel derives these
/// facts from the trusted store, not from the replacement harness.
#[derive(Clone, Copy)]
pub struct LocalFixtureReplacementGrantCheck<'a> {
    /// In-memory transport proof.
    pub principal: &'a VerifiedLocalFixturePrincipal<'a>,
    /// Current durable capability grant.
    pub grant: &'a CapabilityGrant,
    /// Original action binding persisted alongside the grant.
    pub stored_intent: &'a SpecContentHash,
    /// Exact intent reconstructed from current durable state.
    pub intent: &'a LocalFixtureReplacementIntent<'a>,
    /// Issuer selected by the operator authorization boundary.
    pub authorized_issuer: &'a ActorId,
    /// Independently resolved prerequisites for this fixture.
    pub current_requirements: &'a CapabilityGrantRequirements,
    /// Trusted time observed for this validation.
    pub observed_at: Timestamp,
    /// Exact current window expiry.
    pub window_expires_at: Timestamp,
}

/// Checks an authenticated principal against one exact fixture grant.
/// This predicate is not a dispatch authorization or a persisted receipt.
///
/// # Errors
///
/// Rejects actor, scope, issuer, prerequisite, intent, or lifetime mismatch.
pub fn validate_local_fixture_replacement_grant(
    input: LocalFixtureReplacementGrantCheck<'_>,
) -> Result<(), WorkflowOsError> {
    let LocalFixtureReplacementGrantCheck {
        principal,
        grant,
        stored_intent,
        intent,
        authorized_issuer,
        current_requirements,
        observed_at,
        window_expires_at,
    } = input;
    if grant.subject() != principal.0.principal()
        || grant.scope().harness_contract_id() != Some(principal.0.harness())
    {
        return Err(authority_denied());
    }
    validate_local_fixture_grant_binding(
        grant,
        stored_intent,
        intent,
        authorized_issuer,
        current_requirements,
        observed_at,
        window_expires_at,
    )
}

pub(crate) fn validate_local_fixture_grant_binding(
    grant: &CapabilityGrant,
    stored_intent: &SpecContentHash,
    intent: &LocalFixtureReplacementIntent<'_>,
    authorized_issuer: &ActorId,
    current_requirements: &CapabilityGrantRequirements,
    observed_at: Timestamp,
    window_expires_at: Timestamp,
) -> Result<(), WorkflowOsError> {
    let Some(expires_at) = grant.expires_at() else {
        return Err(authority_denied());
    };
    if grant.subject() == intent.original_actor
        || grant.subject() == grant.issuer()
        || grant.issuer() != authorized_issuer
        || grant.capability().as_str() != CAPABILITY
        || grant.scope() != intent.scope
        || grant.resource() != intent.resource
        || grant.lifecycle() != CapabilityGrantLifecycle::Active
        || grant.delegation() != CapabilityDelegationPosture::Disabled
        || intent.sensitivity == WorkReportSensitivity::Unknown
        || intent.sensitivity > grant.sensitivity_ceiling()
        || grant.requirements() != current_requirements
        || !current_requirements
            .approval_ids()
            .contains(intent.approval_id)
        || grant.issued_at() > observed_at
        || observed_at >= expires_at
        || expires_at > window_expires_at
        || &intent.commitment()? != stored_intent
    {
        return Err(authority_denied());
    }
    Ok(())
}

fn authority_denied() -> WorkflowOsError {
    WorkflowOsError::security(
        "local_fixture_replacement_authority.denied",
        "local fixture replacement authority could not be verified",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::too_many_lines)]
mod tests {
    use super::*;
    use crate::{
        CapabilityGrantDefinition, CapabilityGrantId, CapabilityReference, CapabilityResourceKind,
        RedactionMetadata, StepId, WorkflowId, WorkflowRunId,
    };

    #[test]
    fn credential_resolves_principal_and_detects_replacement_registration() {
        let actor = ActorId::new("agent/claude-desktop").expect("actor");
        let other = ActorId::new("agent/other").expect("other");
        let harness = HarnessContractId::new("harness/claude-desktop").expect("harness");
        let original =
            LocalFixturePrincipalVerifier::new(actor.clone(), harness.clone(), &[47; 32]);
        assert!(original.authenticate(&[48; 32]).is_err());
        let verified = original.authenticate(&[47; 32]).expect("exact proof");
        let durable = serde_json::to_string(&original).expect("persist verifier");
        assert!(!durable.contains("[47,47,47,47"));
        let reloaded = serde_json::from_str(&durable).expect("reload verifier");
        assert!(verified.matches_registration(&reloaded));
        assert!(
            !verified.matches_registration(&LocalFixturePrincipalVerifier::new(
                actor,
                harness.clone(),
                &[48; 32]
            ))
        );
        assert!(
            !verified.matches_registration(&LocalFixturePrincipalVerifier::new(
                other, harness, &[47; 32]
            ))
        );
        assert!(!format!("{original:?}").contains("claude-desktop"));
    }

    #[test]
    fn grant_is_exact_read_only_fixture_authority() {
        let original = ActorId::new("agent/codex").expect("original actor");
        let replacement = ActorId::new("agent/claude-desktop").expect("replacement");
        let issuer = ActorId::new("user/maintainer").expect("issuer");
        let scope = CapabilityGrantScope::new(
            WorkflowId::new("test/fixture").expect("workflow"),
            Some(WorkflowRunId::new("run-fixture").expect("run")),
            Some(StepId::new("comment").expect("step")),
            Some(HarnessContractId::new("harness/claude-desktop").expect("harness")),
        )
        .expect("scope");
        let resource = CapabilityResourceScope::new(
            CapabilityResourceKind::AdapterResource,
            "github/example/project/pull/7",
        )
        .expect("target");
        let approval = ApprovalReferenceId::new("approval/run-fixture/comment").expect("approval");
        let requirements =
            CapabilityGrantRequirements::new(vec![], vec![approval.clone()], vec![], vec![])
                .expect("requirements");
        let intent = LocalFixtureReplacementIntent {
            original_actor: &original,
            scope: &scope,
            resource: &resource,
            bundle_root: &SpecContentHash::from_text("bundle"),
            action_commitment: &SpecContentHash::from_text("original action"),
            approval_id: &approval,
            approval_decision_event_id: &EventId::new("event/approved").expect("approval event"),
            presentation_id: &ApprovalPresentationId::new("presentation/test")
                .expect("presentation"),
            presentation_hash: &ApprovalPresentationContentHash::new("a".repeat(64))
                .expect("presentation hash"),
            side_effect_id: &SideEffectId::new("effect/fixture").expect("effect"),
            window_id: &AuthorizedExecutionWindowId::new("window/fixture").expect("window"),
            attempt_id: &AuthorizedExecutionAttemptId::new("attempt/fixture").expect("attempt"),
            sensitivity: WorkReportSensitivity::Internal,
        };
        let commitment = intent.commitment().expect("exact intent");
        let start = Timestamp::parse_rfc3339("2026-11-07T07:00:00Z").expect("issued");
        let expires = Timestamp::parse_rfc3339("2026-11-07T07:04:00Z").expect("expires");
        let grant = |lifecycle, subject: &ActorId| {
            CapabilityGrant::new(CapabilityGrantDefinition {
                grant_id: CapabilityGrantId::new("grant/fixture").expect("grant"),
                subject: subject.clone(),
                capability: CapabilityReference::new(CAPABILITY).expect("capability"),
                resource: resource.clone(),
                scope: scope.clone(),
                issuer: issuer.clone(),
                issued_at: start,
                expires_at: Some(expires),
                lifecycle,
                revocation_reference: (lifecycle == CapabilityGrantLifecycle::Revoked)
                    .then(|| "revocation/fixture".into()),
                delegation: CapabilityDelegationPosture::Disabled,
                requirements: requirements.clone(),
                sensitivity_ceiling: WorkReportSensitivity::Internal,
                redaction: RedactionMetadata {
                    redacted_fields: vec![],
                    field_states: vec![],
                },
            })
            .expect("valid grant declaration")
        };
        let verifier = LocalFixturePrincipalVerifier::new(
            replacement.clone(),
            scope.harness_contract_id().expect("exact harness").clone(),
            &[42; 32],
        );
        let proof = verifier.authenticate(&[42; 32]).expect("replacement proof");
        let active = grant(CapabilityGrantLifecycle::Active, &replacement);
        let check = LocalFixtureReplacementGrantCheck {
            principal: &proof,
            grant: &active,
            stored_intent: &commitment,
            intent: &intent,
            authorized_issuer: &issuer,
            current_requirements: &requirements,
            observed_at: start,
            window_expires_at: expires,
        };
        assert!(validate_local_fixture_replacement_grant(check).is_ok());
        let changed_action = LocalFixtureReplacementIntent {
            action_commitment: &SpecContentHash::from_text("substituted action"),
            ..intent
        };
        let revoked = grant(CapabilityGrantLifecycle::Revoked, &replacement);
        let wrong_actor = grant(CapabilityGrantLifecycle::Active, &original);
        for invalid in [
            LocalFixtureReplacementGrantCheck {
                intent: &changed_action,
                ..check
            },
            LocalFixtureReplacementGrantCheck {
                grant: &revoked,
                ..check
            },
            LocalFixtureReplacementGrantCheck {
                grant: &wrong_actor,
                ..check
            },
            LocalFixtureReplacementGrantCheck {
                observed_at: expires,
                ..check
            },
            LocalFixtureReplacementGrantCheck {
                window_expires_at: start,
                ..check
            },
            LocalFixtureReplacementGrantCheck {
                authorized_issuer: &original,
                ..check
            },
        ] {
            assert_eq!(
                validate_local_fixture_replacement_grant(invalid)
                    .expect_err("authority denied")
                    .code(),
                "local_fixture_replacement_authority.denied"
            );
        }
    }
}
