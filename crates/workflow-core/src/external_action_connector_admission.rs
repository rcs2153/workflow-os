use serde::Serialize;

use crate::current_authority_source::{
    RegisteredCurrentAuthorityConsumerResult, RegisteredCurrentAuthorityUseInput,
    RegisteredCurrentAuthorityUsePosture, RegisteredExactGrantVerificationInput,
    RegisteredInMemoryCurrentAuthoritySource,
};
use crate::{
    ApprovalDecision, ApprovalDecisionKind, ApprovalDecisionProofEnforcementMode,
    ApprovalDecisionProofValidationPolicy, ApprovalPresentationRecord,
    AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, CapabilityReference,
    CapabilityResourceKind, EventId, ExternalActionAuthorizationProjection,
    ExternalActionEvidencePredicate, ExternalActionTransitionKind,
    ExternalActionTransitionLifecycleCommand, ExternalActionTransitionLifecycleOutcome,
    ExternalActionTransitionLifecycleStatus, ExternalActionTransitionOperation,
    LocalExternalActionTransitionLifecycleStore, RedactionMetadata, RequiredContextContractBinding,
    RequiredContextExecutionBinding, SpecContentHash, Timestamp, WorkflowOsError,
};

const EXTERNAL_ACTION_CAPABILITY: &str = "github.pull_request.lifecycle";

pub(crate) struct ExternalActionConnectorFixtureFacts<'a> {
    pub(crate) approval_presentation: &'a ApprovalPresentationRecord,
    pub(crate) approval_decision: &'a ApprovalDecision,
    pub(crate) policy_decision_event_id: &'a EventId,
    pub(crate) governance_assessment_commitment: &'a SpecContentHash,
    pub(crate) execution_window_id: &'a AuthorizedExecutionWindowId,
    pub(crate) execution_window_open: bool,
    pub(crate) revocation_source_commitment: &'a SpecContentHash,
    pub(crate) revoked: bool,
    pub(crate) current_resource: &'a crate::GitHubExternalActionResourceBinding,
    pub(crate) evidence: &'a [ExternalActionEvidencePredicate],
}

pub(crate) struct ExternalActionConnectorAdmissionInput<'a, C> {
    pub(crate) projection: &'a ExternalActionAuthorizationProjection,
    pub(crate) transition_kind: ExternalActionTransitionKind,
    pub(crate) authority_source: &'a RegisteredInMemoryCurrentAuthoritySource,
    pub(crate) execution_binding: &'a RequiredContextExecutionBinding,
    pub(crate) required_context_contract: &'a RequiredContextContractBinding,
    pub(crate) lifecycle_store: &'a LocalExternalActionTransitionLifecycleStore,
    pub(crate) fixture_facts: ExternalActionConnectorFixtureFacts<'a>,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) redaction: &'a RedactionMetadata,
    pub(crate) connector: &'a mut C,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionFixtureConnectorResult {
    Succeeded,
    Failed,
    Ambiguous,
}

pub(crate) trait ExternalActionFixtureConnector {
    fn execute(
        &mut self,
        capability: ExternalActionConnectorUseCapability<'_>,
    ) -> ExternalActionFixtureConnectorResult;
}

pub(crate) struct ExternalActionConnectorUseCapability<'call> {
    operation: &'call ExternalActionTransitionOperation,
    attempt_id: &'call AuthorizedExecutionAttemptId,
    authority_commitment: SpecContentHash,
}

impl ExternalActionConnectorUseCapability<'_> {
    pub(crate) const fn operation(&self) -> &ExternalActionTransitionOperation {
        self.operation
    }

    pub(crate) const fn attempt_id(&self) -> &AuthorizedExecutionAttemptId {
        self.attempt_id
    }

    pub(crate) const fn authority_commitment(&self) -> &SpecContentHash {
        &self.authority_commitment
    }
}

impl std::fmt::Debug for ExternalActionConnectorUseCapability<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalActionConnectorUseCapability")
            .field("transition_kind", &self.operation.transition_kind())
            .field("attempt_id", &self.attempt_id)
            .field("authority_commitment", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionConnectorAdmissionPosture {
    Succeeded,
    Failed,
    Ambiguous,
    ExistingTerminal,
    RecoveryRequired,
}

#[derive(Debug)]
pub(crate) struct ExternalActionConnectorAdmissionOutcome {
    posture: ExternalActionConnectorAdmissionPosture,
    status: ExternalActionTransitionLifecycleStatus,
}

impl ExternalActionConnectorAdmissionOutcome {
    pub(crate) const fn posture(&self) -> ExternalActionConnectorAdmissionPosture {
        self.posture
    }

    pub(crate) const fn status(&self) -> ExternalActionTransitionLifecycleStatus {
        self.status
    }
}

pub(crate) fn execute_fixture_external_action<C: ExternalActionFixtureConnector>(
    input: &mut ExternalActionConnectorAdmissionInput<'_, C>,
) -> Result<ExternalActionConnectorAdmissionOutcome, WorkflowOsError> {
    validate_static_input(input)?;
    let operation = ExternalActionTransitionOperation::from_projection(
        input.projection,
        input.transition_kind,
    )?;
    let mut captured = None;
    let use_outcome = input.authority_source.use_current_authority(
        &RegisteredCurrentAuthorityUseInput {
            execution_binding: input.execution_binding,
            contract: input.required_context_contract,
            evaluated_at: input.evaluated_at,
            redaction: input.redaction,
        },
        |current_authority| {
            let result = execute_under_current_authority(input, &operation, current_authority);
            let posture = match &result {
                Ok(outcome)
                    if outcome.posture == ExternalActionConnectorAdmissionPosture::Succeeded =>
                {
                    RegisteredCurrentAuthorityConsumerResult::Succeeded
                }
                Ok(outcome)
                    if outcome.posture == ExternalActionConnectorAdmissionPosture::Ambiguous =>
                {
                    RegisteredCurrentAuthorityConsumerResult::OutcomeAmbiguous
                }
                Ok(_) | Err(_) => RegisteredCurrentAuthorityConsumerResult::Failed,
            };
            captured = Some(result);
            posture
        },
    )?;

    match use_outcome.posture() {
        RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded
        | RegisteredCurrentAuthorityUsePosture::ConsumerFailed
        | RegisteredCurrentAuthorityUsePosture::ConsumerOutcomeAmbiguous => {
            captured.ok_or_else(|| {
                admission_error(
                    "consumer.result_missing",
                    "external action connector result was not captured",
                )
            })?
        }
        RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse => Err(admission_error(
            "authority.blocked",
            "external action connector authority is not currently sufficient",
        )),
        RegisteredCurrentAuthorityUsePosture::SourceFailure => Err(admission_error(
            "authority.source_failed",
            "external action connector authority source failed",
        )),
    }
}

fn execute_under_current_authority<C: ExternalActionFixtureConnector>(
    input: &mut ExternalActionConnectorAdmissionInput<'_, C>,
    operation: &ExternalActionTransitionOperation,
    current_authority: &crate::current_authority_source::RegisteredCurrentAuthorityUseCapability<
        '_,
    >,
) -> Result<ExternalActionConnectorAdmissionOutcome, WorkflowOsError> {
    let authority_commitment = derive_authority_commitment(input, operation, current_authority)?;
    input
        .lifecycle_store
        .initialize(operation, input.evaluated_at)?;
    let snapshot = input
        .lifecycle_store
        .read_snapshot(operation.operation_id())?;
    if let Some(outcome) = existing_lifecycle_outcome(snapshot.status()) {
        return Ok(outcome);
    }
    let attempt_id = admit_attempt(input, operation, &authority_commitment, &snapshot)?;
    let connector_result = input
        .connector
        .execute(ExternalActionConnectorUseCapability {
            operation,
            attempt_id: &attempt_id,
            authority_commitment: authority_commitment.clone(),
        });
    record_connector_outcome(
        input,
        operation,
        attempt_id,
        &authority_commitment,
        connector_result,
    )
}

fn derive_authority_commitment<C>(
    input: &ExternalActionConnectorAdmissionInput<'_, C>,
    operation: &ExternalActionTransitionOperation,
    current_authority: &crate::current_authority_source::RegisteredCurrentAuthorityUseCapability<
        '_,
    >,
) -> Result<SpecContentHash, WorkflowOsError> {
    let expected_capability = CapabilityReference::new(EXTERNAL_ACTION_CAPABILITY)?;
    let grant_commitment =
        current_authority.verify_exact_grant(&RegisteredExactGrantVerificationInput {
            grant_id: input.projection.capability_grant_id(),
            expected_capability: &expected_capability,
            expected_resource_kind: CapabilityResourceKind::Repository,
            expected_resource_reference: input.projection.resource().repository(),
            execution_binding: input.execution_binding,
            evaluated_at: input.evaluated_at,
            requested_sensitivity: input.projection.sensitivity(),
        })?;
    commitment(
        "external-action-connector-current-authority-v1",
        &(
            input.projection.projection_commitment(),
            operation.operation_definition_commitment(),
            input.execution_binding.binding_hash(),
            grant_commitment,
            input.fixture_facts.policy_decision_event_id,
            input.fixture_facts.governance_assessment_commitment,
            input.fixture_facts.execution_window_id,
            input.fixture_facts.revocation_source_commitment,
            input.evaluated_at,
        ),
    )
}

fn existing_lifecycle_outcome(
    status: ExternalActionTransitionLifecycleStatus,
) -> Option<ExternalActionConnectorAdmissionOutcome> {
    if matches!(
        status,
        ExternalActionTransitionLifecycleStatus::OutcomeSucceeded
            | ExternalActionTransitionLifecycleStatus::ReconciledSucceeded
            | ExternalActionTransitionLifecycleStatus::Consumed
            | ExternalActionTransitionLifecycleStatus::Denied
            | ExternalActionTransitionLifecycleStatus::Revoked
    ) {
        return Some(ExternalActionConnectorAdmissionOutcome {
            posture: ExternalActionConnectorAdmissionPosture::ExistingTerminal,
            status,
        });
    }
    if matches!(
        status,
        ExternalActionTransitionLifecycleStatus::Reserved
            | ExternalActionTransitionLifecycleStatus::Admitted
            | ExternalActionTransitionLifecycleStatus::Attempted
            | ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous
            | ExternalActionTransitionLifecycleStatus::Waiting
    ) {
        return Some(ExternalActionConnectorAdmissionOutcome {
            posture: ExternalActionConnectorAdmissionPosture::RecoveryRequired,
            status,
        });
    }
    None
}

fn admit_attempt<C>(
    input: &ExternalActionConnectorAdmissionInput<'_, C>,
    operation: &ExternalActionTransitionOperation,
    authority_commitment: &SpecContentHash,
    initial_snapshot: &crate::ExternalActionTransitionLifecycleSnapshot,
) -> Result<AuthorizedExecutionAttemptId, WorkflowOsError> {
    let attempt_identity = commitment(
        "external-action-connector-attempt-identity-v1",
        &(
            operation.operation_id(),
            initial_snapshot.cursor().sequence(),
            initial_snapshot.cursor().head_commitment(),
        ),
    )?;
    let attempt_id = AuthorizedExecutionAttemptId::new(format!(
        "external-action-attempt/{}",
        attempt_identity.as_str()
    ))?;
    append(
        input,
        operation,
        initial_snapshot,
        &ExternalActionTransitionLifecycleCommand::Reserve {
            attempt_id: attempt_id.clone(),
        },
    )?;
    let snapshot = input
        .lifecycle_store
        .read_snapshot(operation.operation_id())?;
    append(
        input,
        operation,
        &snapshot,
        &ExternalActionTransitionLifecycleCommand::Admit {
            attempt_id: attempt_id.clone(),
            admission_commitment: authority_commitment.clone(),
        },
    )?;
    let snapshot = input
        .lifecycle_store
        .read_snapshot(operation.operation_id())?;
    let attempt_commitment = commitment(
        "external-action-connector-attempt-v1",
        &(
            operation.operation_definition_commitment(),
            &attempt_id,
            &authority_commitment,
        ),
    )?;
    append(
        input,
        operation,
        &snapshot,
        &ExternalActionTransitionLifecycleCommand::MarkAttempted {
            attempt_id: attempt_id.clone(),
            attempt_commitment,
        },
    )?;
    Ok(attempt_id)
}

fn record_connector_outcome<C>(
    input: &ExternalActionConnectorAdmissionInput<'_, C>,
    operation: &ExternalActionTransitionOperation,
    attempt_id: AuthorizedExecutionAttemptId,
    authority_commitment: &SpecContentHash,
    connector_result: ExternalActionFixtureConnectorResult,
) -> Result<ExternalActionConnectorAdmissionOutcome, WorkflowOsError> {
    let snapshot = input
        .lifecycle_store
        .read_snapshot(operation.operation_id())?;
    let (outcome, posture) = match connector_result {
        ExternalActionFixtureConnectorResult::Succeeded => (
            ExternalActionTransitionLifecycleOutcome::Succeeded,
            ExternalActionConnectorAdmissionPosture::Succeeded,
        ),
        ExternalActionFixtureConnectorResult::Failed => (
            ExternalActionTransitionLifecycleOutcome::Failed,
            ExternalActionConnectorAdmissionPosture::Failed,
        ),
        ExternalActionFixtureConnectorResult::Ambiguous => (
            ExternalActionTransitionLifecycleOutcome::Ambiguous,
            ExternalActionConnectorAdmissionPosture::Ambiguous,
        ),
    };
    let observation_commitment = commitment(
        "external-action-connector-observation-v1",
        &(
            operation.operation_definition_commitment(),
            &attempt_id,
            authority_commitment,
            outcome,
        ),
    )?;
    append(
        input,
        operation,
        &snapshot,
        &ExternalActionTransitionLifecycleCommand::ObserveOutcome {
            attempt_id,
            outcome,
            observation_commitment,
        },
    )?;
    let terminal = input
        .lifecycle_store
        .read_snapshot(operation.operation_id())?;
    Ok(ExternalActionConnectorAdmissionOutcome {
        posture,
        status: terminal.status(),
    })
}

fn append<C>(
    input: &ExternalActionConnectorAdmissionInput<'_, C>,
    operation: &ExternalActionTransitionOperation,
    snapshot: &crate::ExternalActionTransitionLifecycleSnapshot,
    command: &ExternalActionTransitionLifecycleCommand,
) -> Result<(), WorkflowOsError> {
    input.lifecycle_store.append(
        operation.operation_id(),
        snapshot.cursor(),
        command,
        input.evaluated_at,
    )?;
    Ok(())
}

fn validate_static_input<C>(
    input: &ExternalActionConnectorAdmissionInput<'_, C>,
) -> Result<(), WorkflowOsError> {
    if input.projection.connector_kind() != crate::ExternalConnectorKind::GitHub
        || input.execution_binding.workflow_id() != input.projection.workflow_id()
        || input.execution_binding.run_id() != input.projection.run_id()
        || input.execution_binding.actor() != input.projection.actor()
        || input.execution_binding.immutable_run_bundle().root_hash()
            != input.projection.immutable_run_bundle_commitment()
    {
        return Err(admission_error(
            "binding.mismatch",
            "external action connector binding does not match the immutable projection",
        ));
    }
    if input.evaluated_at < input.projection.not_before()
        || input.evaluated_at >= input.projection.expires_at()
    {
        return Err(admission_error(
            "projection.not_current",
            "external action connector projection is not currently usable",
        ));
    }
    if !input.fixture_facts.execution_window_open
        || input.fixture_facts.execution_window_id != input.projection.execution_window_id()
        || input.fixture_facts.revoked
        || input.fixture_facts.revocation_source_commitment
            != input.projection.revocation_source_commitment()
        || input.fixture_facts.policy_decision_event_id
            != input.projection.policy_decision_event_id()
        || input.fixture_facts.governance_assessment_commitment
            != input.projection.governance_assessment_commitment()
        || input.fixture_facts.current_resource != input.projection.resource()
    {
        return Err(admission_error(
            "current_facts.mismatch",
            "external action connector current facts do not satisfy the projection",
        ));
    }
    validate_approval(input)?;
    validate_evidence(input)?;
    Ok(())
}

fn validate_approval<C>(
    input: &ExternalActionConnectorAdmissionInput<'_, C>,
) -> Result<(), WorkflowOsError> {
    let presentation = input.fixture_facts.approval_presentation;
    let decision = input.fixture_facts.approval_decision;
    let expected_action = format!(
        "authorize external action scope {}",
        input
            .projection
            .approval_presentation_scope_commitment()
            .as_str()
    );
    let non_goals_commitment = commitment(
        "external-action-presentation-non-goals-v1",
        &presentation.strict_non_goals(),
    )?;
    if presentation.presentation_id() != input.projection.presentation_id()
        || presentation.content_hash() != input.projection.presentation_content_hash()
        || presentation.run_id() != input.projection.run_id()
        || presentation.workflow_id() != input.projection.workflow_id()
        || presentation.approval_id() != input.projection.approval_reference_id().as_str()
        || presentation.requested_action() != expected_action
        || &non_goals_commitment != input.projection.strict_non_goals_commitment()
        || presentation.presented_at() > decision.decided_at
        || decision.decided_at > input.evaluated_at
        || decision.approval_id != presentation.approval_id()
        || decision.actor != *input.projection.actor()
        || decision.decision != ApprovalDecisionKind::Granted
    {
        return Err(admission_error(
            "approval.mismatch",
            "external action connector approval proof does not match the projection",
        ));
    }
    let Some(marker) = decision.proof_marker.as_ref() else {
        return Err(admission_error(
            "approval.proof_missing",
            "external action connector approval decision is missing presentation proof",
        ));
    };
    if marker.enforcement_mode()
        != ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired
        || marker.proof_validation_policy()
            != ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch
        || marker.presentation_id() != presentation.presentation_id()
        || marker.presentation_content_hash() != presentation.content_hash()
        || marker.proof_validated_at() != decision.decided_at
    {
        return Err(admission_error(
            "approval.proof_mismatch",
            "external action connector approval decision proof is invalid",
        ));
    }
    Ok(())
}

fn validate_evidence<C>(
    input: &ExternalActionConnectorAdmissionInput<'_, C>,
) -> Result<(), WorkflowOsError> {
    let expected = input
        .projection
        .transition(input.transition_kind)
        .ok_or_else(|| {
            admission_error(
                "transition.not_projected",
                "external action connector transition is not projected",
            )
        })?
        .evidence_predicates();
    if expected != input.fixture_facts.evidence {
        return Err(admission_error(
            "evidence.mismatch",
            "external action connector evidence does not match the projected predicates",
        ));
    }
    for predicate in input.fixture_facts.evidence {
        let observed_at = predicate.observed_at().as_offset_date_time();
        let evaluated_at = input.evaluated_at.as_offset_date_time();
        let max_age_seconds = i64::try_from(predicate.max_age_seconds()).map_err(|_| {
            admission_error(
                "evidence.max_age_invalid",
                "external action connector evidence freshness bound is invalid",
            )
        })?;
        if observed_at > evaluated_at
            || observed_at + time::Duration::seconds(max_age_seconds) < evaluated_at
        {
            return Err(admission_error(
                "evidence.stale",
                "external action connector evidence is stale or future-dated",
            ));
        }
    }
    Ok(())
}

fn commitment<T: Serialize>(
    domain: &'static str,
    value: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(&(domain, value)).map_err(|_| {
        admission_error(
            "commitment.failed",
            "external action connector commitment failed",
        )
    })?;
    Ok(SpecContentHash::from_bytes(bytes))
}

fn admission_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("external_action_connector_admission.{code}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::too_many_lines)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::current_authority_source::{
        registered_current_authority_test_availability, registered_current_authority_test_fixture,
        registered_current_authority_test_grants_with_first_lifecycle,
        registered_current_authority_test_references,
        registered_current_authority_test_source_with_inventory,
    };
    use crate::{
        compute_approval_presentation_content_hash,
        compute_external_action_approval_scope_commitment, ApprovalDecisionProofMarker,
        ApprovalDecisionProofMarkerDefinition, ApprovalPresentationChannel,
        ApprovalPresentationContentHash, ApprovalPresentationId,
        ApprovalPresentationRecordDefinition, ApprovalPresentationSensitivity, ApprovalReferenceId,
        CapabilityAvailability, CapabilityAvailabilityRecord, CapabilityDelegationPosture,
        CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
        CapabilityGrantRequirements, CapabilityGrantScope, CapabilityResourceScope, CorrelationId,
        ExternalActionAuthorizationProjectionDefinition, ExternalActionEvidencePredicateKind,
        ExternalActionPayloadClass, ExternalActionTransition, ExternalConnectorKind,
        GitHubExternalActionResourceBinding, SpecContentHash, WorkReportSensitivity,
    };

    static ROOT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "workflow-os-external-action-admission-{}-{}",
                std::process::id(),
                ROOT_SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).expect("root");
            Self(path)
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[derive(Default)]
    struct RecordingConnector {
        calls: usize,
        result: Option<ExternalActionFixtureConnectorResult>,
        saw_authority: bool,
        attempt_ids: Vec<String>,
    }

    impl ExternalActionFixtureConnector for RecordingConnector {
        fn execute(
            &mut self,
            capability: ExternalActionConnectorUseCapability<'_>,
        ) -> ExternalActionFixtureConnectorResult {
            self.calls += 1;
            self.attempt_ids
                .push(capability.attempt_id().as_str().to_owned());
            self.saw_authority = !capability.authority_commitment().as_str().is_empty()
                && capability.operation().transition_kind()
                    == ExternalActionTransitionKind::GitHubPullRequestCreateDraft;
            self.result
                .unwrap_or(ExternalActionFixtureConnectorResult::Succeeded)
        }
    }

    struct Fixture {
        projection: ExternalActionAuthorizationProjection,
        source: RegisteredInMemoryCurrentAuthoritySource,
        binding: RequiredContextExecutionBinding,
        contract: RequiredContextContractBinding,
        presentation: ApprovalPresentationRecord,
        decision: ApprovalDecision,
        policy_event_id: EventId,
        governance_commitment: SpecContentHash,
        window_id: AuthorizedExecutionWindowId,
        revocation_commitment: SpecContentHash,
        resource: GitHubExternalActionResourceBinding,
        evidence: Vec<ExternalActionEvidencePredicate>,
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("timestamp")
    }

    fn predicate(kind: ExternalActionEvidencePredicateKind) -> ExternalActionEvidencePredicate {
        ExternalActionEvidencePredicate::new(
            kind,
            SpecContentHash::from_text(&format!("source-{kind:?}")),
            SpecContentHash::from_text(&format!("subject-{kind:?}")),
            timestamp("2026-07-26T10:20:00Z"),
            600,
        )
        .expect("predicate")
    }

    fn fixture(external_lifecycle: CapabilityGrantLifecycle) -> Fixture {
        let (contract, binding) = registered_current_authority_test_fixture();
        let capability = CapabilityReference::new(EXTERNAL_ACTION_CAPABILITY).expect("capability");
        let resource_scope =
            CapabilityResourceScope::new(CapabilityResourceKind::Repository, "rcs2153/workflow-os")
                .expect("resource");
        let grant_id = CapabilityGrantId::new("grant/external-action").expect("grant");
        let external_grant = CapabilityGrant::new(CapabilityGrantDefinition {
            grant_id: grant_id.clone(),
            subject: binding.actor().clone(),
            capability: capability.clone(),
            resource: resource_scope.clone(),
            scope: CapabilityGrantScope::new(
                binding.workflow_id().clone(),
                Some(binding.run_id().clone()),
                Some(binding.step_id().clone()),
                Some(binding.harness_contract_id().clone()),
            )
            .expect("scope"),
            issuer: crate::ActorId::new("system/authority").expect("issuer"),
            issued_at: timestamp("2026-07-26T10:05:00Z"),
            expires_at: Some(timestamp("2026-07-26T11:00:00Z")),
            lifecycle: external_lifecycle,
            revocation_reference: (external_lifecycle == CapabilityGrantLifecycle::Revoked)
                .then(|| "revocation/external-action".to_owned()),
            delegation: CapabilityDelegationPosture::Disabled,
            requirements: CapabilityGrantRequirements::default(),
            sensitivity_ceiling: WorkReportSensitivity::Internal,
            redaction: RedactionMetadata::empty(),
        })
        .expect("grant");
        let external_availability = CapabilityAvailabilityRecord::new(
            capability,
            resource_scope,
            CapabilityAvailability::Available,
            timestamp("2026-07-26T10:20:00Z"),
            RedactionMetadata::empty(),
        )
        .expect("availability");
        let mut grants = registered_current_authority_test_grants_with_first_lifecycle(
            &contract,
            CapabilityGrantLifecycle::Active,
        );
        grants.push(external_grant);
        grants.sort_by(|left, right| left.grant_id().cmp(right.grant_id()));
        let mut availability = registered_current_authority_test_availability(&contract);
        availability.push(external_availability);
        let source = registered_current_authority_test_source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            grants,
            availability,
            registered_current_authority_test_references(&contract),
        );

        let resource = GitHubExternalActionResourceBinding::new(
            "rcs2153/workflow-os",
            "main",
            "1111111111111111111111111111111111111111",
            "codex/external-action-admission",
            "2222222222222222222222222222222222222222",
        )
        .expect("resource");
        let evidence = vec![
            predicate(ExternalActionEvidencePredicateKind::BaseTargetMatches),
            predicate(ExternalActionEvidencePredicateKind::PayloadCommitmentMatches),
            predicate(ExternalActionEvidencePredicateKind::ReviewedHeadMatches),
        ];
        let transition = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            SpecContentHash::from_text("create-operation"),
            ExternalActionPayloadClass::GitHubDraftPullRequest,
            SpecContentHash::from_text("create-payload"),
            evidence.clone(),
        )
        .expect("transition");
        let strict_non_goals = vec!["No live provider access".to_owned()];
        let strict_non_goals_commitment = commitment(
            "external-action-presentation-non-goals-v1",
            &strict_non_goals.as_slice(),
        )
        .expect("non-goals commitment");
        let transitions = vec![transition];
        let scope_commitment = compute_external_action_approval_scope_commitment(
            ExternalConnectorKind::GitHub,
            "github-installation/workflow-os",
            &resource,
            &transitions,
            &strict_non_goals_commitment,
        )
        .expect("scope");
        let approval_id = "approval/external-action";
        let presentation_id =
            ApprovalPresentationId::new("presentation/external-action").expect("presentation");
        let requested_action = format!(
            "authorize external action scope {}",
            scope_commitment.as_str()
        );
        let channel = ApprovalPresentationChannel::Terminal;
        let presentation_hash = compute_approval_presentation_content_hash(
            binding.run_id(),
            approval_id,
            binding.workflow_id(),
            None,
            None,
            None,
            &requested_action,
            "Execute one fixture connector transition",
            "Create one exact draft pull request fixture",
            &strict_non_goals,
            &["workflow-core fixture connector".to_owned()],
            &["current authority and evidence must pass".to_owned()],
            "The connector admission phase is authorized",
            "Invoke the fixture once",
            &channel,
            ApprovalPresentationSensitivity::Internal,
        )
        .expect("presentation hash");
        let presentation = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
            presentation_id: presentation_id.clone(),
            run_id: binding.run_id().clone(),
            approval_id: approval_id.to_owned(),
            workflow_id: binding.workflow_id().clone(),
            workflow_version: None,
            schema_version: None,
            step_id: None,
            requested_action,
            work_summary: "Execute one fixture connector transition".to_owned(),
            approved_scope: "Create one exact draft pull request fixture".to_owned(),
            strict_non_goals,
            expected_touched_surfaces: vec!["workflow-core fixture connector".to_owned()],
            validation_expectations: vec!["current authority and evidence must pass".to_owned()],
            why_now: "The connector admission phase is authorized".to_owned(),
            next_action: "Invoke the fixture once".to_owned(),
            presented_at: timestamp("2026-07-26T10:21:00Z"),
            presented_by: binding.actor().clone(),
            channel,
            content_hash: presentation_hash.clone(),
            redaction: RedactionMetadata::empty(),
            sensitivity: ApprovalPresentationSensitivity::Internal,
        })
        .expect("presentation");
        let decided_at = timestamp("2026-07-26T10:22:00Z");
        let marker = ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
            enforcement_mode: ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
            presentation_id: presentation_id.clone(),
            presentation_content_hash: presentation_hash.clone(),
            proof_validated_at: decided_at,
            proof_validation_policy:
                ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
            proof_age_ms: Some(60_000),
            proof_freshness_limit_ms: Some(86_400_000),
            proof_record_sensitivity: ApprovalPresentationSensitivity::Internal,
            redaction: RedactionMetadata::empty(),
        })
        .expect("marker");
        let decision = ApprovalDecision {
            approval_id: approval_id.to_owned(),
            actor: binding.actor().clone(),
            decided_at,
            decision: ApprovalDecisionKind::Granted,
            reason: "bounded fixture transition".to_owned(),
            correlation_id: CorrelationId::new("correlation/external-action").expect("correlation"),
            proof_marker: Some(marker),
        };
        let policy_event_id = EventId::new("event/external-action").expect("event");
        let governance_commitment = SpecContentHash::from_text("governance");
        let window_id = AuthorizedExecutionWindowId::new("window/external-action").expect("window");
        let revocation_commitment = SpecContentHash::from_text("revocation-source");
        let projection = ExternalActionAuthorizationProjection::new(
            ExternalActionAuthorizationProjectionDefinition {
                workflow_id: binding.workflow_id().clone(),
                run_id: binding.run_id().clone(),
                approval_reference_id: ApprovalReferenceId::new(approval_id).expect("approval"),
                presentation_id,
                presentation_content_hash: ApprovalPresentationContentHash::new(
                    presentation_hash.as_str(),
                )
                .expect("hash"),
                approval_presentation_scope_commitment: scope_commitment,
                capability_grant_id: grant_id,
                policy_decision_event_id: policy_event_id.clone(),
                governance_assessment_commitment: governance_commitment.clone(),
                execution_window_id: window_id.clone(),
                immutable_run_bundle_commitment: binding.immutable_run_bundle().root_hash().clone(),
                connector_kind: ExternalConnectorKind::GitHub,
                connector_account_reference: "github-installation/workflow-os".to_owned(),
                resource: resource.clone(),
                transitions,
                actor: binding.actor().clone(),
                delegator: crate::ActorId::new("user/repository-owner").expect("delegator"),
                delegate: binding.actor().clone(),
                delegation_scope_commitment: SpecContentHash::from_text("delegation"),
                strict_non_goals_commitment,
                issued_at: timestamp("2026-07-26T10:20:00Z"),
                not_before: timestamp("2026-07-26T10:20:00Z"),
                expires_at: timestamp("2026-07-26T11:00:00Z"),
                revocation_source_commitment: revocation_commitment.clone(),
                sensitivity: WorkReportSensitivity::Internal,
            },
        )
        .expect("projection");
        let evidence = projection
            .transition(ExternalActionTransitionKind::GitHubPullRequestCreateDraft)
            .expect("projected transition")
            .evidence_predicates()
            .to_vec();
        Fixture {
            projection,
            source,
            binding,
            contract,
            presentation,
            decision,
            policy_event_id,
            governance_commitment,
            window_id,
            revocation_commitment,
            resource,
            evidence,
        }
    }

    fn execute(
        fixture: &Fixture,
        store: &LocalExternalActionTransitionLifecycleStore,
        connector: &mut RecordingConnector,
    ) -> Result<ExternalActionConnectorAdmissionOutcome, WorkflowOsError> {
        execute_at(fixture, store, connector, timestamp("2026-07-26T10:25:00Z"))
    }

    fn execute_at(
        fixture: &Fixture,
        store: &LocalExternalActionTransitionLifecycleStore,
        connector: &mut RecordingConnector,
        evaluated_at: Timestamp,
    ) -> Result<ExternalActionConnectorAdmissionOutcome, WorkflowOsError> {
        execute_fixture_external_action(&mut ExternalActionConnectorAdmissionInput {
            projection: &fixture.projection,
            transition_kind: ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            authority_source: &fixture.source,
            execution_binding: &fixture.binding,
            required_context_contract: &fixture.contract,
            lifecycle_store: store,
            fixture_facts: ExternalActionConnectorFixtureFacts {
                approval_presentation: &fixture.presentation,
                approval_decision: &fixture.decision,
                policy_decision_event_id: &fixture.policy_event_id,
                governance_assessment_commitment: &fixture.governance_commitment,
                execution_window_id: &fixture.window_id,
                execution_window_open: true,
                revocation_source_commitment: &fixture.revocation_commitment,
                revoked: false,
                current_resource: &fixture.resource,
                evidence: &fixture.evidence,
            },
            evaluated_at,
            redaction: &RedactionMetadata::empty(),
            connector,
        })
    }

    #[test]
    fn current_authority_capability_enters_fixture_once_and_terminal_replay_does_not_reinvoke() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let mut connector = RecordingConnector::default();

        let first = execute(&fixture, &store, &mut connector).expect("first");
        assert_eq!(
            first.posture(),
            ExternalActionConnectorAdmissionPosture::Succeeded
        );
        assert_eq!(
            first.status(),
            ExternalActionTransitionLifecycleStatus::OutcomeSucceeded
        );
        assert_eq!(connector.calls, 1);
        assert!(connector.saw_authority);

        let replay = execute(&fixture, &store, &mut connector).expect("replay");
        assert_eq!(
            replay.posture(),
            ExternalActionConnectorAdmissionPosture::ExistingTerminal
        );
        assert_eq!(connector.calls, 1);
    }

    #[test]
    fn revoked_projected_grant_blocks_before_lifecycle_or_connector_entry() {
        let fixture = fixture(CapabilityGrantLifecycle::Revoked);
        let root = TestRoot::new();
        let store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let mut connector = RecordingConnector::default();

        let error = execute(&fixture, &store, &mut connector).expect_err("blocked");
        assert_eq!(
            error.code(),
            "current_authority.source.registered.use.grant_not_authorized"
        );
        assert_eq!(connector.calls, 0);
    }

    #[test]
    fn stale_evidence_blocks_before_current_authority_or_connector_entry() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let mut connector = RecordingConnector::default();

        let error = execute_at(
            &fixture,
            &store,
            &mut connector,
            timestamp("2026-07-26T10:40:01Z"),
        )
        .expect_err("blocked");
        assert_eq!(
            error.code(),
            "external_action_connector_admission.evidence.stale"
        );
        assert_eq!(connector.calls, 0);
    }

    #[test]
    fn changed_head_blocks_before_connector_entry() {
        let mut fixture = fixture(CapabilityGrantLifecycle::Active);
        fixture.resource = GitHubExternalActionResourceBinding::new(
            "rcs2153/workflow-os",
            "main",
            "1111111111111111111111111111111111111111",
            "codex/external-action-admission",
            "3333333333333333333333333333333333333333",
        )
        .expect("resource");
        let root = TestRoot::new();
        let store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let mut connector = RecordingConnector::default();

        let error = execute(&fixture, &store, &mut connector).expect_err("blocked");
        assert_eq!(
            error.code(),
            "external_action_connector_admission.current_facts.mismatch"
        );
        assert_eq!(connector.calls, 0);
    }

    #[test]
    fn missing_presentation_proof_blocks_before_connector_entry() {
        let mut fixture = fixture(CapabilityGrantLifecycle::Active);
        fixture.decision.proof_marker = None;
        let root = TestRoot::new();
        let store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let mut connector = RecordingConnector::default();

        let error = execute(&fixture, &store, &mut connector).expect_err("blocked");
        assert_eq!(
            error.code(),
            "external_action_connector_admission.approval.proof_missing"
        );
        assert_eq!(connector.calls, 0);
    }

    #[test]
    fn fixture_failure_and_ambiguity_are_durable_and_not_blindly_retried() {
        for (result, expected_posture, expected_status) in [
            (
                ExternalActionFixtureConnectorResult::Failed,
                ExternalActionConnectorAdmissionPosture::Failed,
                ExternalActionTransitionLifecycleStatus::OutcomeFailed,
            ),
            (
                ExternalActionFixtureConnectorResult::Ambiguous,
                ExternalActionConnectorAdmissionPosture::Ambiguous,
                ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous,
            ),
        ] {
            let fixture = fixture(CapabilityGrantLifecycle::Active);
            let root = TestRoot::new();
            let store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
            let mut connector = RecordingConnector {
                result: Some(result),
                ..RecordingConnector::default()
            };
            let outcome = execute(&fixture, &store, &mut connector).expect("outcome");
            assert_eq!(outcome.posture(), expected_posture);
            assert_eq!(outcome.status(), expected_status);
            assert_eq!(connector.calls, 1);
            if result == ExternalActionFixtureConnectorResult::Failed {
                connector.result = Some(ExternalActionFixtureConnectorResult::Succeeded);
                let retry = execute(&fixture, &store, &mut connector).expect("retry");
                assert_eq!(
                    retry.posture(),
                    ExternalActionConnectorAdmissionPosture::Succeeded
                );
                assert_eq!(connector.calls, 2);
                assert_ne!(connector.attempt_ids[0], connector.attempt_ids[1]);
            } else {
                let replay = execute(&fixture, &store, &mut connector).expect("replay");
                assert_eq!(
                    replay.posture(),
                    ExternalActionConnectorAdmissionPosture::RecoveryRequired
                );
                assert_eq!(connector.calls, 1);
            }
        }
    }
}
