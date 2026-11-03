use serde::Serialize;

use crate::current_authority_source::{
    RegisteredCurrentAuthorityConsumerResult, RegisteredCurrentAuthorityUseInput,
    RegisteredCurrentAuthorityUsePosture, RegisteredExactGrantVerificationInput,
    RegisteredInMemoryCurrentAuthoritySource,
};
use crate::external_action_fixture_lifecycle::{
    ExternalActionFixtureConnectorPosture, ExternalActionFixtureProviderApplyPosture,
    ExternalActionFixtureProviderApplyResult, ExternalActionFixtureProviderOperation,
    ExternalActionFixtureProviderSource, ExternalActionFixtureRequiredChecksPosture,
    ExternalActionFixtureSourceAnchor, ExternalActionFixtureSourceClass,
    ExternalActionFixtureSourceCoverage, ExternalActionFixtureSourceDisposition,
    ExternalActionFixtureSourceRecord, ExternalActionSuccessorBindingStore,
    ExternalActionSuccessorBindingWritePosture,
};
use crate::{
    derive_external_action_successor_binding, ApprovalDecision, ApprovalDecisionKind,
    ApprovalDecisionProofEnforcementMode, ApprovalDecisionProofValidationPolicy,
    ApprovalPresentationRecord, AuthorizedExecutionActionReference, AuthorizedExecutionAttemptId,
    AuthorizedExecutionResourceReference, AuthorizedExecutionWaitCondition,
    AuthorizedExecutionWaitConditionDefinition, AuthorizedExecutionWaitConditionId,
    AuthorizedExecutionWaitConditionKind, AuthorizedExecutionWaitStatus,
    AuthorizedExecutionWakeTriggerKind, AuthorizedExecutionWindowId, CapabilityReference,
    CapabilityResourceKind, EventId, EventSequenceNumber, ExternalActionAuthorizationProjection,
    ExternalActionEvidencePredicate, ExternalActionSuccessorBinding, ExternalActionTransitionKind,
    ExternalActionTransitionLifecycleCommand, ExternalActionTransitionLifecycleOutcome,
    ExternalActionTransitionLifecycleStatus, ExternalActionTransitionOperation,
    ExternalActionWaitResolutionProof, LocalExternalActionTransitionLifecycleStore,
    RedactionMetadata, RequiredContextContractBinding, RequiredContextExecutionBinding,
    SpecContentHash, Timestamp, WorkflowOsError,
};

const EXTERNAL_ACTION_CAPABILITY: &str = "github.pull_request.lifecycle";

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) enum ExternalActionFixtureExecutionWindowPosture {
    Open,
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) enum ExternalActionFixtureRevocationPosture {
    Current,
    Revoked,
}

#[derive(Clone)]
pub(crate) struct ExternalActionFixtureDraftSourceDefinition {
    pub(crate) approval_presentation: ApprovalPresentationRecord,
    pub(crate) approval_decision: ApprovalDecision,
    pub(crate) policy_decision_event_id: EventId,
    pub(crate) governance_assessment_commitment: SpecContentHash,
    pub(crate) execution_window_id: AuthorizedExecutionWindowId,
    pub(crate) execution_window_posture: ExternalActionFixtureExecutionWindowPosture,
    pub(crate) revocation_source_commitment: SpecContentHash,
    pub(crate) revocation_posture: ExternalActionFixtureRevocationPosture,
    pub(crate) current_resource: crate::GitHubExternalActionResourceBinding,
    pub(crate) evidence: Vec<ExternalActionEvidencePredicate>,
}

#[derive(Clone)]
pub(crate) struct ExternalActionFixtureDraftSourceObservation {
    definition: ExternalActionFixtureDraftSourceDefinition,
}

impl std::fmt::Debug for ExternalActionFixtureDraftSourceObservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureDraftSourceObservation")
            .field("source_coverage", &"[REDACTED]")
            .field(
                "execution_window_posture",
                &self.definition.execution_window_posture,
            )
            .field("revocation_posture", &self.definition.revocation_posture)
            .field("evidence_count", &self.definition.evidence.len())
            .finish_non_exhaustive()
    }
}

pub(crate) trait ExternalActionFixtureDraftSource {
    fn observe(&self) -> Result<ExternalActionFixtureDraftSourceObservation, WorkflowOsError>;
}

pub(crate) struct InMemoryExternalActionFixtureDraftSource {
    observation: ExternalActionFixtureDraftSourceObservation,
}

impl InMemoryExternalActionFixtureDraftSource {
    pub(crate) fn new(definition: ExternalActionFixtureDraftSourceDefinition) -> Self {
        Self {
            observation: ExternalActionFixtureDraftSourceObservation { definition },
        }
    }
}

impl ExternalActionFixtureDraftSource for InMemoryExternalActionFixtureDraftSource {
    fn observe(&self) -> Result<ExternalActionFixtureDraftSourceObservation, WorkflowOsError> {
        Ok(self.observation.clone())
    }
}

pub(crate) struct ExternalActionFixtureDraftCreationInput<'a, F, P, S> {
    pub(crate) projection: &'a ExternalActionAuthorizationProjection,
    pub(crate) authority_source: &'a RegisteredInMemoryCurrentAuthoritySource,
    pub(crate) execution_binding: &'a RequiredContextExecutionBinding,
    pub(crate) required_context_contract: &'a RequiredContextContractBinding,
    pub(crate) lifecycle_store: &'a LocalExternalActionTransitionLifecycleStore,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) redaction: &'a RedactionMetadata,
    pub(crate) fact_source: &'a F,
    pub(crate) provider: &'a P,
    pub(crate) successor_store: &'a S,
}

pub(crate) struct ExternalActionFixtureMarkReadyInput<'a, F, P, S> {
    pub(crate) projection: &'a ExternalActionAuthorizationProjection,
    pub(crate) authority_source: &'a RegisteredInMemoryCurrentAuthoritySource,
    pub(crate) execution_binding: &'a RequiredContextExecutionBinding,
    pub(crate) required_context_contract: &'a RequiredContextContractBinding,
    pub(crate) lifecycle_store: &'a LocalExternalActionTransitionLifecycleStore,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) redaction: &'a RedactionMetadata,
    pub(crate) fact_source: &'a F,
    pub(crate) provider: &'a P,
    pub(crate) successor_store: &'a S,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionFixtureDraftCreationPosture {
    Created,
    ExactReplay,
    ReconciledSuccessor,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ExternalActionFixtureDraftCreationOutcome {
    posture: ExternalActionFixtureDraftCreationPosture,
    successor_binding: ExternalActionSuccessorBinding,
    provider_receipt_commitment: SpecContentHash,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionFixtureMarkReadyPosture {
    WaitingForEvidence,
    WaitingForConnector,
    MarkedReady,
    ExactReplay,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ExternalActionFixtureMarkReadyOutcome {
    posture: ExternalActionFixtureMarkReadyPosture,
    successor_binding: ExternalActionSuccessorBinding,
    wait_condition_id: Option<AuthorizedExecutionWaitConditionId>,
    provider_receipt_commitment: Option<SpecContentHash>,
}

impl ExternalActionFixtureMarkReadyOutcome {
    pub(crate) const fn posture(&self) -> ExternalActionFixtureMarkReadyPosture {
        self.posture
    }

    pub(crate) const fn wait_condition_id(&self) -> Option<&AuthorizedExecutionWaitConditionId> {
        self.wait_condition_id.as_ref()
    }
}

impl std::fmt::Debug for ExternalActionFixtureMarkReadyOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureMarkReadyOutcome")
            .field("posture", &self.posture)
            .field("successor_binding", &self.successor_binding)
            .field("wait_condition", &self.wait_condition_id.is_some())
            .field(
                "provider_receipt_commitment",
                &self
                    .provider_receipt_commitment
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl ExternalActionFixtureDraftCreationOutcome {
    pub(crate) const fn posture(&self) -> ExternalActionFixtureDraftCreationPosture {
        self.posture
    }

    pub(crate) const fn successor_binding(&self) -> &ExternalActionSuccessorBinding {
        &self.successor_binding
    }
}

impl std::fmt::Debug for ExternalActionFixtureDraftCreationOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureDraftCreationOutcome")
            .field("posture", &self.posture)
            .field("successor_binding", &self.successor_binding)
            .field("provider_receipt_commitment", &"[REDACTED]")
            .finish()
    }
}

struct FixtureProviderConnector<'a, P> {
    provider: &'a P,
    expected_provider_revision: SpecContentHash,
    expected_successor_number: Option<u64>,
    capture: Option<FixtureProviderConnectorCapture>,
}

enum FixtureProviderConnectorCapture {
    Applied(ExternalActionFixtureProviderApplyResult),
    PreMutationRefusal(FixtureProviderPreMutationRefusal),
    Failed(WorkflowOsError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FixtureProviderPreMutationRefusal {
    ChecksNotGreen,
    ConnectorUnavailable,
}

impl<P: ExternalActionFixtureProviderSource> ExternalActionFixtureConnector
    for FixtureProviderConnector<'_, P>
{
    fn execute(
        &mut self,
        capability: ExternalActionConnectorUseCapability<'_>,
    ) -> ExternalActionFixtureConnectorResult {
        let operation = self.expected_successor_number.map_or_else(
            || {
                ExternalActionFixtureProviderOperation::new(
                    capability.operation().operation_id().clone(),
                    capability.operation().transition_kind(),
                    self.expected_provider_revision.clone(),
                )
            },
            |successor| {
                ExternalActionFixtureProviderOperation::for_successor(
                    capability.operation().operation_id().clone(),
                    capability.operation().transition_kind(),
                    self.expected_provider_revision.clone(),
                    successor,
                )
            },
        );
        let (capture, posture) = match self.provider.compare_and_apply(&operation) {
            Ok(result) => (
                FixtureProviderConnectorCapture::Applied(result),
                ExternalActionFixtureConnectorResult::Succeeded,
            ),
            Err(error)
                if error.code() == "external_action_fixture.provider.refused_before_dispatch" =>
            {
                (
                    FixtureProviderConnectorCapture::PreMutationRefusal(
                        FixtureProviderPreMutationRefusal::ConnectorUnavailable,
                    ),
                    ExternalActionFixtureConnectorResult::Failed,
                )
            }
            Err(error) if error.code() == "external_action_fixture.provider.checks_not_green" => (
                FixtureProviderConnectorCapture::PreMutationRefusal(
                    FixtureProviderPreMutationRefusal::ChecksNotGreen,
                ),
                ExternalActionFixtureConnectorResult::Failed,
            ),
            Err(error) => (
                FixtureProviderConnectorCapture::Failed(error),
                ExternalActionFixtureConnectorResult::Failed,
            ),
        };
        self.capture = Some(capture);
        posture
    }
}

pub(crate) fn execute_fixture_draft_creation<
    F: ExternalActionFixtureDraftSource,
    P: ExternalActionFixtureProviderSource,
    S: ExternalActionSuccessorBindingStore,
>(
    input: &ExternalActionFixtureDraftCreationInput<'_, F, P, S>,
) -> Result<ExternalActionFixtureDraftCreationOutcome, WorkflowOsError> {
    let fact_observation = input.fact_source.observe()?;
    let before = input.provider.observe()?;
    let operation = ExternalActionTransitionOperation::from_projection(
        input.projection,
        ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
    )?;
    let source_coverage = compose_fixture_source_coverage(
        input.projection,
        input.execution_binding,
        input.evaluated_at,
        &fact_observation,
        &before,
        &operation,
    )?;
    let before_anchor = ExternalActionFixtureSourceAnchor::new(
        &source_coverage,
        before.provider_revision().clone(),
    )?;
    let current = input.provider.observe()?;
    let current_anchor = ExternalActionFixtureSourceAnchor::new(
        &source_coverage,
        current.provider_revision().clone(),
    )?;
    before_anchor.verify_unchanged(&current_anchor)?;

    let mut connector = FixtureProviderConnector {
        provider: input.provider,
        expected_provider_revision: current.provider_revision().clone(),
        expected_successor_number: None,
        capture: None,
    };
    let admission = execute_fixture_external_action(&mut ExternalActionConnectorAdmissionInput {
        projection: input.projection,
        transition_kind: ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
        authority_source: input.authority_source,
        execution_binding: input.execution_binding,
        required_context_contract: input.required_context_contract,
        lifecycle_store: input.lifecycle_store,
        fixture_facts: ExternalActionConnectorFixtureFacts {
            approval_presentation: &fact_observation.definition.approval_presentation,
            approval_decision: &fact_observation.definition.approval_decision,
            policy_decision_event_id: &fact_observation.definition.policy_decision_event_id,
            governance_assessment_commitment: &fact_observation
                .definition
                .governance_assessment_commitment,
            execution_window_id: &fact_observation.definition.execution_window_id,
            execution_window_open: fact_observation.definition.execution_window_posture
                == ExternalActionFixtureExecutionWindowPosture::Open,
            revocation_source_commitment: &fact_observation.definition.revocation_source_commitment,
            revoked: fact_observation.definition.revocation_posture
                == ExternalActionFixtureRevocationPosture::Revoked,
            current_resource: &fact_observation.definition.current_resource,
            evidence: &fact_observation.definition.evidence,
        },
        evaluated_at: input.evaluated_at,
        redaction: input.redaction,
        connector: &mut connector,
    })?;

    let (provider_result, repairing) = resolve_draft_provider_result(
        input.provider,
        &ExternalActionFixtureProviderOperation::new(
            operation.operation_id().clone(),
            operation.transition_kind(),
            current.provider_revision().clone(),
        ),
        admission.posture(),
        connector.capture,
    )?;
    persist_draft_successor(input, &provider_result, repairing)
}

fn compose_fixture_source_coverage(
    projection: &ExternalActionAuthorizationProjection,
    execution_binding: &RequiredContextExecutionBinding,
    evaluated_at: Timestamp,
    facts: &ExternalActionFixtureDraftSourceObservation,
    provider: &crate::external_action_fixture_lifecycle::ExternalActionFixtureProviderObservation,
    operation: &ExternalActionTransitionOperation,
) -> Result<ExternalActionFixtureSourceCoverage, WorkflowOsError> {
    let definition = &facts.definition;
    let revisions = [
        (
            ExternalActionFixtureSourceClass::ImmutableRun,
            projection.immutable_run_bundle_commitment().clone(),
        ),
        (
            ExternalActionFixtureSourceClass::ApprovalPresentation,
            SpecContentHash::from_text(definition.approval_presentation.content_hash().as_str()),
        ),
        (
            ExternalActionFixtureSourceClass::ApprovalDecision,
            commitment(
                "external-action-fixture-source-approval-decision-v1",
                &(operation.transition_kind(), &definition.approval_decision),
            )?,
        ),
        (
            ExternalActionFixtureSourceClass::CurrentCapability,
            commitment(
                "external-action-fixture-source-current-capability-v1",
                &(
                    operation.transition_kind(),
                    projection.capability_grant_id(),
                    execution_binding.binding_hash(),
                ),
            )?,
        ),
        (
            ExternalActionFixtureSourceClass::PolicyAndGovernance,
            commitment(
                "external-action-fixture-source-policy-v1",
                &(
                    operation.transition_kind(),
                    &definition.policy_decision_event_id,
                    &definition.governance_assessment_commitment,
                ),
            )?,
        ),
        (
            ExternalActionFixtureSourceClass::ExecutionWindow,
            commitment(
                "external-action-fixture-source-window-v1",
                &(
                    operation.transition_kind(),
                    &definition.execution_window_id,
                    definition.execution_window_posture,
                    evaluated_at,
                ),
            )?,
        ),
        (
            ExternalActionFixtureSourceClass::Revocation,
            commitment(
                "external-action-fixture-source-revocation-v1",
                &(
                    operation.transition_kind(),
                    &definition.revocation_source_commitment,
                    definition.revocation_posture,
                ),
            )?,
        ),
        (
            ExternalActionFixtureSourceClass::ProviderResource,
            commitment(
                "external-action-fixture-source-provider-resource-v1",
                &(
                    operation.transition_kind(),
                    provider.observation_commitment(),
                ),
            )?,
        ),
        (
            ExternalActionFixtureSourceClass::ProviderEvidence,
            commitment(
                "external-action-fixture-source-provider-evidence-v1",
                &(
                    operation.transition_kind(),
                    provider.observation_commitment(),
                    &definition.evidence,
                ),
            )?,
        ),
        (
            ExternalActionFixtureSourceClass::TransitionLifecycle,
            operation.operation_definition_commitment().clone(),
        ),
    ];
    ExternalActionFixtureSourceCoverage::new(
        revisions
            .into_iter()
            .map(|(class, revision)| {
                ExternalActionFixtureSourceRecord::new(
                    class,
                    revision,
                    ExternalActionFixtureSourceDisposition::Available,
                )
            })
            .collect(),
    )
}

fn resolve_draft_provider_result<P: ExternalActionFixtureProviderSource>(
    provider: &P,
    operation: &ExternalActionFixtureProviderOperation,
    admission_posture: ExternalActionConnectorAdmissionPosture,
    capture: Option<FixtureProviderConnectorCapture>,
) -> Result<(ExternalActionFixtureProviderApplyResult, bool), WorkflowOsError> {
    Ok(match capture {
        Some(FixtureProviderConnectorCapture::Applied(result)) => (result, false),
        Some(FixtureProviderConnectorCapture::Failed(error)) => return Err(error),
        Some(FixtureProviderConnectorCapture::PreMutationRefusal(_)) => {
            return Err(admission_error(
                "draft.provider_refused_before_mutation",
                "fixture draft provider refused before mutation",
            ));
        }
        None if admission_posture == ExternalActionConnectorAdmissionPosture::ExistingTerminal => (
            provider.lookup_operation(operation)?.ok_or_else(|| {
                admission_error(
                    "draft.reconciliation_inconclusive",
                    "fixture draft creation requires provider receipt reconciliation",
                )
            })?,
            true,
        ),
        None => {
            return Err(admission_error(
                "draft.provider_result_missing",
                "fixture draft creation provider result is missing",
            ));
        }
    })
}

fn persist_draft_successor<F, P, S: ExternalActionSuccessorBindingStore>(
    input: &ExternalActionFixtureDraftCreationInput<'_, F, P, S>,
    provider_result: &ExternalActionFixtureProviderApplyResult,
    repairing: bool,
) -> Result<ExternalActionFixtureDraftCreationOutcome, WorkflowOsError> {
    let created = provider_result.created_successor().ok_or_else(|| {
        admission_error(
            "draft.successor_missing",
            "fixture draft creation result is missing the successor identity",
        )
    })?;
    let binding = derive_external_action_successor_binding(
        input.projection,
        created.pull_request_number(),
        created.observed_at(),
    )?;
    if let Some(existing) = input
        .successor_store
        .read_exact(input.projection)
        .map_err(classify_successor_store_error)?
    {
        if existing != binding {
            return Err(admission_error(
                "draft.successor_persistence_conflict",
                "persisted successor binding does not match the provider result",
            ));
        }
        return Ok(ExternalActionFixtureDraftCreationOutcome {
            posture: ExternalActionFixtureDraftCreationPosture::ExactReplay,
            successor_binding: existing,
            provider_receipt_commitment: provider_result.provider_receipt_commitment().clone(),
        });
    }
    let write = input
        .successor_store
        .put_exact(input.projection, &binding)
        .map_err(|error| {
            if error.code() == "external_action_fixture_lifecycle.successor.conflict" {
                error
            } else {
                admission_error(
                    "draft.successor_persistence_ambiguous",
                    "provider creation succeeded but successor persistence is unresolved",
                )
            }
        })?;
    let persisted = input
        .successor_store
        .read_exact(input.projection)
        .map_err(classify_successor_store_error)?
        .ok_or_else(|| {
            admission_error(
                "draft.successor_persistence_ambiguous",
                "provider creation succeeded but successor persistence is unresolved",
            )
        })?;
    if persisted != binding {
        return Err(admission_error(
            "draft.successor_persistence_conflict",
            "persisted successor binding does not match the provider result",
        ));
    }
    let posture = if repairing {
        ExternalActionFixtureDraftCreationPosture::ReconciledSuccessor
    } else if provider_result.posture() == ExternalActionFixtureProviderApplyPosture::ExactReplay
        || write == ExternalActionSuccessorBindingWritePosture::ExactReplay
    {
        ExternalActionFixtureDraftCreationPosture::ExactReplay
    } else {
        ExternalActionFixtureDraftCreationPosture::Created
    };
    Ok(ExternalActionFixtureDraftCreationOutcome {
        posture,
        successor_binding: persisted,
        provider_receipt_commitment: provider_result.provider_receipt_commitment().clone(),
    })
}

fn classify_successor_store_error(error: WorkflowOsError) -> WorkflowOsError {
    if error.code() == "external_action_fixture_lifecycle.successor.store_unavailable"
        || error.code() == "external_action_connector_admission.test.successor_store_unavailable"
    {
        admission_error(
            "draft.successor_persistence_ambiguous",
            "provider creation succeeded but successor persistence is unresolved",
        )
    } else {
        error
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn execute_fixture_mark_ready<
    F: ExternalActionFixtureDraftSource,
    P: ExternalActionFixtureProviderSource,
    S: ExternalActionSuccessorBindingStore,
>(
    input: &ExternalActionFixtureMarkReadyInput<'_, F, P, S>,
) -> Result<ExternalActionFixtureMarkReadyOutcome, WorkflowOsError> {
    let fact_observation = input.fact_source.observe()?;
    let before = input.provider.observe()?;
    let operation = ExternalActionTransitionOperation::from_projection(
        input.projection,
        ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
    )?;
    let source_coverage = compose_fixture_source_coverage(
        input.projection,
        input.execution_binding,
        input.evaluated_at,
        &fact_observation,
        &before,
        &operation,
    )?;
    let before_anchor = ExternalActionFixtureSourceAnchor::new(
        &source_coverage,
        before.provider_revision().clone(),
    )?;
    let current = input.provider.observe()?;
    let current_anchor = ExternalActionFixtureSourceAnchor::new(
        &source_coverage,
        current.provider_revision().clone(),
    )?;
    before_anchor.verify_unchanged(&current_anchor)?;
    validate_fixture_provider_resource(input.projection, &current)?;

    let successor_binding = input
        .successor_store
        .read_exact(input.projection)?
        .ok_or_else(|| {
            admission_error(
                "ready.successor_missing",
                "fixture mark-ready requires the exact persisted successor binding",
            )
        })?;
    successor_binding.validate_for_projection(input.projection)?;

    {
        let mut validation_connector = FixtureProviderConnector {
            provider: input.provider,
            expected_provider_revision: current.provider_revision().clone(),
            expected_successor_number: Some(successor_binding.provider_pull_request_number()),
            capture: None,
        };
        validate_static_input(&ExternalActionConnectorAdmissionInput {
            projection: input.projection,
            transition_kind:
                ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
            authority_source: input.authority_source,
            execution_binding: input.execution_binding,
            required_context_contract: input.required_context_contract,
            lifecycle_store: input.lifecycle_store,
            fixture_facts: fixture_facts(&fact_observation),
            evaluated_at: input.evaluated_at,
            redaction: input.redaction,
            connector: &mut validation_connector,
        })?;
    }
    input
        .lifecycle_store
        .initialize(&operation, input.evaluated_at)?;

    if let Some(waiting) =
        resolve_or_preserve_ready_wait(input, &operation, &current, &successor_binding)?
    {
        return Ok(waiting);
    }
    if current.required_checks_posture()
        != ExternalActionFixtureRequiredChecksPosture::TerminalGreen
    {
        return register_ready_wait(
            input,
            &operation,
            &current,
            successor_binding,
            AuthorizedExecutionWaitConditionKind::CheckRequired,
        );
    }
    if current.connector_posture() == ExternalActionFixtureConnectorPosture::RefusedBeforeDispatch {
        return register_ready_wait(
            input,
            &operation,
            &current,
            successor_binding,
            AuthorizedExecutionWaitConditionKind::CapabilityUnavailable,
        );
    }

    let mut connector = FixtureProviderConnector {
        provider: input.provider,
        expected_provider_revision: current.provider_revision().clone(),
        expected_successor_number: Some(successor_binding.provider_pull_request_number()),
        capture: None,
    };
    let admission = execute_fixture_external_action(&mut ExternalActionConnectorAdmissionInput {
        projection: input.projection,
        transition_kind: ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
        authority_source: input.authority_source,
        execution_binding: input.execution_binding,
        required_context_contract: input.required_context_contract,
        lifecycle_store: input.lifecycle_store,
        fixture_facts: fixture_facts(&fact_observation),
        evaluated_at: input.evaluated_at,
        redaction: input.redaction,
        connector: &mut connector,
    })?;
    let provider_result = match connector.capture {
        Some(FixtureProviderConnectorCapture::Applied(result)) => result,
        Some(FixtureProviderConnectorCapture::PreMutationRefusal(refusal)) => {
            let refreshed = input.provider.observe()?;
            let wait_kind = match refusal {
                FixtureProviderPreMutationRefusal::ChecksNotGreen => {
                    AuthorizedExecutionWaitConditionKind::CheckRequired
                }
                FixtureProviderPreMutationRefusal::ConnectorUnavailable => {
                    AuthorizedExecutionWaitConditionKind::CapabilityUnavailable
                }
            };
            return register_ready_wait(
                input,
                &operation,
                &refreshed,
                successor_binding,
                wait_kind,
            );
        }
        Some(FixtureProviderConnectorCapture::Failed(error)) => return Err(error),
        None if admission.posture()
            == ExternalActionConnectorAdmissionPosture::ExistingTerminal =>
        {
            input
                .provider
                .lookup_operation(&ExternalActionFixtureProviderOperation::for_successor(
                    operation.operation_id().clone(),
                    operation.transition_kind(),
                    current.provider_revision().clone(),
                    successor_binding.provider_pull_request_number(),
                ))?
                .ok_or_else(|| {
                    admission_error(
                        "ready.reconciliation_inconclusive",
                        "fixture mark-ready requires the exact provider receipt",
                    )
                })?
        }
        None => {
            return Err(admission_error(
                "ready.provider_result_missing",
                "fixture mark-ready provider result is missing",
            ));
        }
    };
    if provider_result.created_successor().is_some() {
        return Err(admission_error(
            "ready.successor_unexpected",
            "fixture mark-ready must not create a successor identity",
        ));
    }
    Ok(ExternalActionFixtureMarkReadyOutcome {
        posture: if admission.posture() == ExternalActionConnectorAdmissionPosture::ExistingTerminal
            || provider_result.posture() == ExternalActionFixtureProviderApplyPosture::ExactReplay
        {
            ExternalActionFixtureMarkReadyPosture::ExactReplay
        } else {
            ExternalActionFixtureMarkReadyPosture::MarkedReady
        },
        successor_binding,
        wait_condition_id: None,
        provider_receipt_commitment: Some(provider_result.provider_receipt_commitment().clone()),
    })
}

fn fixture_facts(
    observation: &ExternalActionFixtureDraftSourceObservation,
) -> ExternalActionConnectorFixtureFacts<'_> {
    ExternalActionConnectorFixtureFacts {
        approval_presentation: &observation.definition.approval_presentation,
        approval_decision: &observation.definition.approval_decision,
        policy_decision_event_id: &observation.definition.policy_decision_event_id,
        governance_assessment_commitment: &observation.definition.governance_assessment_commitment,
        execution_window_id: &observation.definition.execution_window_id,
        execution_window_open: observation.definition.execution_window_posture
            == ExternalActionFixtureExecutionWindowPosture::Open,
        revocation_source_commitment: &observation.definition.revocation_source_commitment,
        revoked: observation.definition.revocation_posture
            == ExternalActionFixtureRevocationPosture::Revoked,
        current_resource: &observation.definition.current_resource,
        evidence: &observation.definition.evidence,
    }
}

fn validate_fixture_provider_resource(
    projection: &ExternalActionAuthorizationProjection,
    provider: &crate::external_action_fixture_lifecycle::ExternalActionFixtureProviderObservation,
) -> Result<(), WorkflowOsError> {
    let resource = projection.resource();
    let repository = SpecContentHash::from_text(resource.repository());
    let base = SpecContentHash::from_text(&format!(
        "{}@{}",
        resource.base_ref(),
        resource.base_commit_sha()
    ));
    let head = SpecContentHash::from_text(resource.reviewed_head_sha());
    if provider.repository_commitment() != &repository
        || provider.base_target_commitment() != &base
        || provider.reviewed_head_commitment() != &head
    {
        return Err(admission_error(
            "ready.provider_resource_mismatch",
            "fixture provider resource does not match the exact projected repository and head",
        ));
    }
    Ok(())
}

fn resolve_or_preserve_ready_wait<F, P, S>(
    input: &ExternalActionFixtureMarkReadyInput<'_, F, P, S>,
    operation: &ExternalActionTransitionOperation,
    provider: &crate::external_action_fixture_lifecycle::ExternalActionFixtureProviderObservation,
    successor_binding: &ExternalActionSuccessorBinding,
) -> Result<Option<ExternalActionFixtureMarkReadyOutcome>, WorkflowOsError> {
    let snapshot = input
        .lifecycle_store
        .read_snapshot(operation.operation_id())?;
    if snapshot.status() != ExternalActionTransitionLifecycleStatus::Waiting {
        return Ok(None);
    }
    let active = snapshot.active_wait().ok_or_else(|| {
        admission_error(
            "ready.wait_missing",
            "fixture mark-ready lifecycle is waiting without an active condition",
        )
    })?;
    let still_waiting = match active.kind() {
        AuthorizedExecutionWaitConditionKind::CheckRequired => {
            provider.required_checks_posture()
                != ExternalActionFixtureRequiredChecksPosture::TerminalGreen
        }
        AuthorizedExecutionWaitConditionKind::CapabilityUnavailable => {
            provider.connector_posture()
                == ExternalActionFixtureConnectorPosture::RefusedBeforeDispatch
        }
        _ => {
            return Err(admission_error(
                "ready.wait_kind_invalid",
                "fixture mark-ready active wait has an unsupported kind",
            ));
        }
    };
    if still_waiting {
        return Ok(Some(ExternalActionFixtureMarkReadyOutcome {
            posture: wait_posture(active.kind())?,
            successor_binding: successor_binding.clone(),
            wait_condition_id: Some(active.condition_id().clone()),
            provider_receipt_commitment: None,
        }));
    }
    input.lifecycle_store.append(
        operation.operation_id(),
        snapshot.cursor(),
        &ExternalActionTransitionLifecycleCommand::ResolveWait {
            proof: ExternalActionWaitResolutionProof::new(
                active,
                provider.observation_commitment().clone(),
                AuthorizedExecutionWaitStatus::Satisfied,
                input.evaluated_at,
            )?,
        },
        input.evaluated_at,
    )?;
    Ok(None)
}

#[allow(clippy::too_many_lines)]
fn register_ready_wait<F, P, S>(
    input: &ExternalActionFixtureMarkReadyInput<'_, F, P, S>,
    operation: &ExternalActionTransitionOperation,
    provider: &crate::external_action_fixture_lifecycle::ExternalActionFixtureProviderObservation,
    successor_binding: ExternalActionSuccessorBinding,
    kind: AuthorizedExecutionWaitConditionKind,
) -> Result<ExternalActionFixtureMarkReadyOutcome, WorkflowOsError> {
    let mut snapshot = input
        .lifecycle_store
        .read_snapshot(operation.operation_id())?;
    let attempt_id = if matches!(
        snapshot.status(),
        ExternalActionTransitionLifecycleStatus::Projected
            | ExternalActionTransitionLifecycleStatus::OutcomeFailed
            | ExternalActionTransitionLifecycleStatus::ReconciledFailed
    ) {
        let attempt_id = derive_attempt_id(operation, &snapshot)?;
        input.lifecycle_store.append(
            operation.operation_id(),
            snapshot.cursor(),
            &ExternalActionTransitionLifecycleCommand::Reserve {
                attempt_id: attempt_id.clone(),
            },
            input.evaluated_at,
        )?;
        snapshot = input
            .lifecycle_store
            .read_snapshot(operation.operation_id())?;
        attempt_id
    } else if snapshot.status() == ExternalActionTransitionLifecycleStatus::Reserved
        && snapshot.verified_wait_resume_attempt_id().is_some()
    {
        snapshot.active_attempt_id().cloned().ok_or_else(|| {
            admission_error(
                "ready.wait_attempt_missing",
                "fixture mark-ready wait is missing its reserved attempt",
            )
        })?
    } else {
        return Err(admission_error(
            "ready.wait_registration_illegal",
            "fixture mark-ready cannot register a wait from the current lifecycle posture",
        ));
    };
    let wait_commitment = commitment(
        "external-action-fixture-ready-wait-v1",
        &(
            operation.operation_id(),
            kind,
            provider.observation_commitment(),
            successor_binding.binding_commitment(),
        ),
    )?;
    let condition_id = AuthorizedExecutionWaitConditionId::new(format!(
        "external-action-wait/{}",
        wait_commitment.as_str()
    ))?;
    let wait = AuthorizedExecutionWaitCondition::new(AuthorizedExecutionWaitConditionDefinition {
        condition_id: condition_id.clone(),
        condition_version: 1,
        kind,
        workflow_id: input.projection.workflow_id().clone(),
        run_id: input.projection.run_id().clone(),
        window_id: input.projection.execution_window_id().clone(),
        action_reference: AuthorizedExecutionActionReference::new(format!(
            "external-action/{}",
            operation.operation_id().as_str()
        ))?,
        step_id: input.execution_binding.step_id().clone(),
        attempt_id,
        expected_sequence_number: EventSequenceNumber::new(snapshot.cursor().sequence())?,
        expected_event_id: EventId::new(format!(
            "event/external-action/{}",
            snapshot.cursor().head_commitment().as_str()
        ))?,
        required_reference: AuthorizedExecutionResourceReference::new(format!(
            "external-action-requirement/{}",
            wait_commitment.as_str()
        ))?,
        created_at: input.evaluated_at,
        deadline: Some(input.projection.expires_at()),
        wake_trigger: match kind {
            AuthorizedExecutionWaitConditionKind::CheckRequired => {
                AuthorizedExecutionWakeTriggerKind::CheckAccepted
            }
            AuthorizedExecutionWaitConditionKind::CapabilityUnavailable => {
                AuthorizedExecutionWakeTriggerKind::CapabilityAvailabilityChanged
            }
            _ => {
                return Err(admission_error(
                    "ready.wait_kind_invalid",
                    "fixture mark-ready wait kind is unsupported",
                ));
            }
        },
        status: AuthorizedExecutionWaitStatus::Waiting,
    })?;
    input.lifecycle_store.append(
        operation.operation_id(),
        snapshot.cursor(),
        &ExternalActionTransitionLifecycleCommand::RegisterWait {
            wait_condition: Box::new(wait),
        },
        input.evaluated_at,
    )?;
    Ok(ExternalActionFixtureMarkReadyOutcome {
        posture: wait_posture(kind)?,
        successor_binding,
        wait_condition_id: Some(condition_id),
        provider_receipt_commitment: None,
    })
}

fn wait_posture(
    kind: AuthorizedExecutionWaitConditionKind,
) -> Result<ExternalActionFixtureMarkReadyPosture, WorkflowOsError> {
    match kind {
        AuthorizedExecutionWaitConditionKind::CheckRequired => {
            Ok(ExternalActionFixtureMarkReadyPosture::WaitingForEvidence)
        }
        AuthorizedExecutionWaitConditionKind::CapabilityUnavailable => {
            Ok(ExternalActionFixtureMarkReadyPosture::WaitingForConnector)
        }
        _ => Err(admission_error(
            "ready.wait_kind_invalid",
            "fixture mark-ready wait kind is unsupported",
        )),
    }
}

fn derive_attempt_id(
    operation: &ExternalActionTransitionOperation,
    snapshot: &crate::ExternalActionTransitionLifecycleSnapshot,
) -> Result<AuthorizedExecutionAttemptId, WorkflowOsError> {
    let identity = commitment(
        "external-action-connector-attempt-identity-v1",
        &(
            operation.operation_id(),
            snapshot.cursor().sequence(),
            snapshot.cursor().head_commitment(),
        ),
    )?;
    AuthorizedExecutionAttemptId::new(format!("external-action-attempt/{}", identity.as_str()))
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
    if let Some(outcome) = existing_lifecycle_outcome(&snapshot) {
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
    snapshot: &crate::ExternalActionTransitionLifecycleSnapshot,
) -> Option<ExternalActionConnectorAdmissionOutcome> {
    let status = snapshot.status();
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
        ExternalActionTransitionLifecycleStatus::Admitted
            | ExternalActionTransitionLifecycleStatus::Attempted
            | ExternalActionTransitionLifecycleStatus::OutcomeAmbiguous
            | ExternalActionTransitionLifecycleStatus::Waiting
    ) {
        return Some(ExternalActionConnectorAdmissionOutcome {
            posture: ExternalActionConnectorAdmissionPosture::RecoveryRequired,
            status,
        });
    }
    if status == ExternalActionTransitionLifecycleStatus::Reserved
        && snapshot.verified_wait_resume_attempt_id().is_none()
    {
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
    if let Some(attempt_id) = initial_snapshot.verified_wait_resume_attempt_id().cloned() {
        append(
            input,
            operation,
            initial_snapshot,
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
                authority_commitment,
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
        return Ok(attempt_id);
    }
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
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

    use super::*;
    use crate::current_authority_source::{
        registered_current_authority_test_availability, registered_current_authority_test_fixture,
        registered_current_authority_test_grants_with_first_lifecycle,
        registered_current_authority_test_references,
        registered_current_authority_test_source_with_inventory,
    };
    use crate::external_action_fixture_lifecycle::{
        ExternalActionFixtureProviderObservation, InMemoryExternalActionFixtureProvider,
        InMemoryExternalActionSuccessorBindingStore,
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
        ExternalActionPayloadClass, ExternalActionSuccessorBinding, ExternalActionTransition,
        ExternalConnectorKind, GitHubExternalActionResourceBinding, SpecContentHash,
        WorkReportSensitivity,
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

    #[derive(Clone, Copy)]
    enum ProviderRaceKind {
        ChecksRegressed,
        ConnectorRefused,
    }

    struct DispatchRaceProvider {
        inner: InMemoryExternalActionFixtureProvider,
        race: ProviderRaceKind,
        armed: AtomicBool,
    }

    impl DispatchRaceProvider {
        fn new(observed_at: Timestamp, race: ProviderRaceKind) -> Self {
            Self {
                inner: InMemoryExternalActionFixtureProvider::new(observed_at),
                race,
                armed: AtomicBool::new(false),
            }
        }

        fn arm(&self) {
            self.armed.store(true, Ordering::SeqCst);
        }

        fn operation_count(&self) -> Result<usize, WorkflowOsError> {
            self.inner.operation_count()
        }

        fn set_required_checks_posture(
            &self,
            posture: ExternalActionFixtureRequiredChecksPosture,
        ) -> Result<(), WorkflowOsError> {
            self.inner.set_required_checks_posture(posture)
        }

        fn set_connector_posture(
            &self,
            posture: ExternalActionFixtureConnectorPosture,
        ) -> Result<(), WorkflowOsError> {
            self.inner.set_connector_posture(posture)
        }
    }

    impl ExternalActionFixtureProviderSource for DispatchRaceProvider {
        fn observe(&self) -> Result<ExternalActionFixtureProviderObservation, WorkflowOsError> {
            self.inner.observe()
        }

        fn compare_and_apply(
            &self,
            operation: &ExternalActionFixtureProviderOperation,
        ) -> Result<ExternalActionFixtureProviderApplyResult, WorkflowOsError> {
            if self.armed.swap(false, Ordering::SeqCst) {
                return match self.race {
                    ProviderRaceKind::ChecksRegressed => {
                        self.inner.set_required_checks_posture(
                            ExternalActionFixtureRequiredChecksPosture::Failed,
                        )?;
                        Err(WorkflowOsError::validation(
                            "external_action_fixture.provider.checks_not_green",
                            "fixture provider required checks are not terminal green",
                        ))
                    }
                    ProviderRaceKind::ConnectorRefused => {
                        self.inner.set_connector_posture(
                            ExternalActionFixtureConnectorPosture::RefusedBeforeDispatch,
                        )?;
                        Err(WorkflowOsError::validation(
                            "external_action_fixture.provider.refused_before_dispatch",
                            "fixture provider refused before dispatch",
                        ))
                    }
                };
            }
            self.inner.compare_and_apply(operation)
        }

        fn lookup_operation(
            &self,
            operation: &ExternalActionFixtureProviderOperation,
        ) -> Result<Option<ExternalActionFixtureProviderApplyResult>, WorkflowOsError> {
            self.inner.lookup_operation(operation)
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
        ready_evidence: Vec<ExternalActionEvidencePredicate>,
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
        let create_transition = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            SpecContentHash::from_text("create-operation"),
            ExternalActionPayloadClass::GitHubDraftPullRequest,
            SpecContentHash::from_text("create-payload"),
            evidence.clone(),
        )
        .expect("transition");
        let ready_transition = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
            SpecContentHash::from_text("ready-operation"),
            ExternalActionPayloadClass::GitHubPullRequestReadyState,
            SpecContentHash::from_text("ready-payload"),
            vec![
                predicate(ExternalActionEvidencePredicateKind::RequiredChecksTerminalGreen),
                predicate(ExternalActionEvidencePredicateKind::ReviewedHeadMatches),
            ],
        )
        .expect("ready transition");
        let ready_evidence = ready_transition.evidence_predicates().to_vec();
        let merge_transition = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            SpecContentHash::from_text("merge-operation"),
            ExternalActionPayloadClass::GitHubPullRequestSquashMerge,
            SpecContentHash::from_text("merge-payload"),
            vec![
                predicate(ExternalActionEvidencePredicateKind::PullRequestReady),
                predicate(ExternalActionEvidencePredicateKind::RequiredChecksTerminalGreen),
                predicate(ExternalActionEvidencePredicateKind::ReviewAndPolicySatisfied),
                predicate(ExternalActionEvidencePredicateKind::ReviewedHeadMatches),
            ],
        )
        .expect("merge transition");
        let strict_non_goals = vec!["No live provider access".to_owned()];
        let strict_non_goals_commitment = commitment(
            "external-action-presentation-non-goals-v1",
            &strict_non_goals.as_slice(),
        )
        .expect("non-goals commitment");
        let transitions = vec![create_transition, ready_transition, merge_transition];
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
            "Execute one bounded fixture pull-request lifecycle",
            "Create one draft, mark the exact successor ready, and later merge the unchanged head",
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
            work_summary: "Execute one bounded fixture pull-request lifecycle".to_owned(),
            approved_scope:
                "Create one draft, mark the exact successor ready, and later merge the unchanged head"
                    .to_owned(),
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
            ready_evidence,
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

    fn execute_draft<
        P: ExternalActionFixtureProviderSource,
        S: ExternalActionSuccessorBindingStore,
    >(
        fixture: &Fixture,
        lifecycle_store: &LocalExternalActionTransitionLifecycleStore,
        provider: &P,
        successor_store: &S,
    ) -> Result<ExternalActionFixtureDraftCreationOutcome, WorkflowOsError> {
        let fact_source = draft_fact_source(
            fixture,
            ExternalActionFixtureRevocationPosture::Current,
            fixture.resource.clone(),
        );
        execute_draft_with_source(
            fixture,
            lifecycle_store,
            provider,
            successor_store,
            &fact_source,
        )
    }

    fn draft_fact_source(
        fixture: &Fixture,
        revocation_posture: ExternalActionFixtureRevocationPosture,
        current_resource: GitHubExternalActionResourceBinding,
    ) -> InMemoryExternalActionFixtureDraftSource {
        InMemoryExternalActionFixtureDraftSource::new(ExternalActionFixtureDraftSourceDefinition {
            approval_presentation: fixture.presentation.clone(),
            approval_decision: fixture.decision.clone(),
            policy_decision_event_id: fixture.policy_event_id.clone(),
            governance_assessment_commitment: fixture.governance_commitment.clone(),
            execution_window_id: fixture.window_id.clone(),
            execution_window_posture: ExternalActionFixtureExecutionWindowPosture::Open,
            revocation_source_commitment: fixture.revocation_commitment.clone(),
            revocation_posture,
            current_resource,
            evidence: fixture.evidence.clone(),
        })
    }

    fn ready_fact_source(
        fixture: &Fixture,
        revocation_posture: ExternalActionFixtureRevocationPosture,
        current_resource: GitHubExternalActionResourceBinding,
    ) -> InMemoryExternalActionFixtureDraftSource {
        InMemoryExternalActionFixtureDraftSource::new(ExternalActionFixtureDraftSourceDefinition {
            approval_presentation: fixture.presentation.clone(),
            approval_decision: fixture.decision.clone(),
            policy_decision_event_id: fixture.policy_event_id.clone(),
            governance_assessment_commitment: fixture.governance_commitment.clone(),
            execution_window_id: fixture.window_id.clone(),
            execution_window_posture: ExternalActionFixtureExecutionWindowPosture::Open,
            revocation_source_commitment: fixture.revocation_commitment.clone(),
            revocation_posture,
            current_resource,
            evidence: fixture.ready_evidence.clone(),
        })
    }

    fn execute_draft_with_source<
        P: ExternalActionFixtureProviderSource,
        S: ExternalActionSuccessorBindingStore,
    >(
        fixture: &Fixture,
        lifecycle_store: &LocalExternalActionTransitionLifecycleStore,
        provider: &P,
        successor_store: &S,
        fact_source: &InMemoryExternalActionFixtureDraftSource,
    ) -> Result<ExternalActionFixtureDraftCreationOutcome, WorkflowOsError> {
        let redaction = RedactionMetadata::empty();
        execute_fixture_draft_creation(&ExternalActionFixtureDraftCreationInput {
            projection: &fixture.projection,
            authority_source: &fixture.source,
            execution_binding: &fixture.binding,
            required_context_contract: &fixture.contract,
            lifecycle_store,
            evaluated_at: timestamp("2026-07-26T10:25:00Z"),
            redaction: &redaction,
            fact_source,
            provider,
            successor_store,
        })
    }

    fn execute_ready<
        P: ExternalActionFixtureProviderSource,
        S: ExternalActionSuccessorBindingStore,
    >(
        fixture: &Fixture,
        lifecycle_store: &LocalExternalActionTransitionLifecycleStore,
        provider: &P,
        successor_store: &S,
    ) -> Result<ExternalActionFixtureMarkReadyOutcome, WorkflowOsError> {
        let fact_source = ready_fact_source(
            fixture,
            ExternalActionFixtureRevocationPosture::Current,
            fixture.resource.clone(),
        );
        execute_ready_with_source(
            fixture,
            lifecycle_store,
            provider,
            successor_store,
            &fact_source,
        )
    }

    fn execute_ready_with_source<
        P: ExternalActionFixtureProviderSource,
        S: ExternalActionSuccessorBindingStore,
    >(
        fixture: &Fixture,
        lifecycle_store: &LocalExternalActionTransitionLifecycleStore,
        provider: &P,
        successor_store: &S,
        fact_source: &InMemoryExternalActionFixtureDraftSource,
    ) -> Result<ExternalActionFixtureMarkReadyOutcome, WorkflowOsError> {
        let redaction = RedactionMetadata::empty();
        execute_fixture_mark_ready(&ExternalActionFixtureMarkReadyInput {
            projection: &fixture.projection,
            authority_source: &fixture.source,
            execution_binding: &fixture.binding,
            required_context_contract: &fixture.contract,
            lifecycle_store,
            evaluated_at: timestamp("2026-07-26T10:25:00Z"),
            redaction: &redaction,
            fact_source,
            provider,
            successor_store,
        })
    }

    struct FailOnceSuccessorStore {
        fail_next: AtomicBool,
        inner: InMemoryExternalActionSuccessorBindingStore,
    }

    impl FailOnceSuccessorStore {
        fn new() -> Self {
            Self {
                fail_next: AtomicBool::new(true),
                inner: InMemoryExternalActionSuccessorBindingStore::default(),
            }
        }
    }

    impl ExternalActionSuccessorBindingStore for FailOnceSuccessorStore {
        fn put_exact(
            &self,
            projection: &ExternalActionAuthorizationProjection,
            binding: &ExternalActionSuccessorBinding,
        ) -> Result<ExternalActionSuccessorBindingWritePosture, WorkflowOsError> {
            if self.fail_next.swap(false, Ordering::SeqCst) {
                return Err(admission_error(
                    "test.successor_store_unavailable",
                    "test successor store is unavailable",
                ));
            }
            self.inner.put_exact(projection, binding)
        }

        fn read_exact(
            &self,
            projection: &ExternalActionAuthorizationProjection,
        ) -> Result<Option<ExternalActionSuccessorBinding>, WorkflowOsError> {
            self.inner.read_exact(projection)
        }
    }

    struct FailReadOnceSuccessorStore {
        fail_next_read: AtomicBool,
        inner: InMemoryExternalActionSuccessorBindingStore,
    }

    impl FailReadOnceSuccessorStore {
        fn new() -> Self {
            Self {
                fail_next_read: AtomicBool::new(true),
                inner: InMemoryExternalActionSuccessorBindingStore::default(),
            }
        }
    }

    impl ExternalActionSuccessorBindingStore for FailReadOnceSuccessorStore {
        fn put_exact(
            &self,
            projection: &ExternalActionAuthorizationProjection,
            binding: &ExternalActionSuccessorBinding,
        ) -> Result<ExternalActionSuccessorBindingWritePosture, WorkflowOsError> {
            self.inner.put_exact(projection, binding)
        }

        fn read_exact(
            &self,
            projection: &ExternalActionAuthorizationProjection,
        ) -> Result<Option<ExternalActionSuccessorBinding>, WorkflowOsError> {
            if self.fail_next_read.swap(false, Ordering::SeqCst) {
                return Err(admission_error(
                    "test.successor_store_unavailable",
                    "test successor store is unavailable",
                ));
            }
            self.inner.read_exact(projection)
        }
    }

    #[test]
    fn phase_5b_creates_one_draft_binds_successor_and_replays_without_provider_entry() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();

        let created = execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("created");
        assert_eq!(
            created.posture(),
            ExternalActionFixtureDraftCreationPosture::Created
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        let replay =
            execute_draft(&fixture, &lifecycle_store, &provider, &successor_store).expect("replay");
        assert_eq!(
            replay.posture(),
            ExternalActionFixtureDraftCreationPosture::ExactReplay
        );
        assert_eq!(replay.successor_binding(), created.successor_binding());
        assert_eq!(provider.operation_count().expect("operation count"), 1);
    }

    #[test]
    fn phase_5b_repairs_successor_after_provider_success_without_duplicate_create() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = FailOnceSuccessorStore::new();

        let error = execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect_err("local persistence ambiguity");
        assert_eq!(
            error.code(),
            "external_action_connector_admission.draft.successor_persistence_ambiguous"
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        let repaired = execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("repaired");
        assert_eq!(
            repaired.posture(),
            ExternalActionFixtureDraftCreationPosture::ReconciledSuccessor
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);
    }

    #[test]
    fn phase_5b_repairs_after_post_provider_successor_read_failure() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = FailReadOnceSuccessorStore::new();

        let error = execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect_err("local read ambiguity");
        assert_eq!(
            error.code(),
            "external_action_connector_admission.draft.successor_persistence_ambiguous"
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        let repaired = execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("repaired");
        assert_eq!(
            repaired.posture(),
            ExternalActionFixtureDraftCreationPosture::ReconciledSuccessor
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);
    }

    #[test]
    fn phase_5b_rejects_local_successor_that_conflicts_with_provider_receipt() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();
        let conflicting = derive_external_action_successor_binding(
            &fixture.projection,
            43,
            timestamp("2026-07-26T10:24:00Z"),
        )
        .expect("conflicting binding");
        successor_store
            .put_exact(&fixture.projection, &conflicting)
            .expect("seed conflicting binding");

        let error = execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect_err("conflicting successor");
        assert_eq!(
            error.code(),
            "external_action_connector_admission.draft.successor_persistence_conflict"
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);
    }

    #[test]
    fn phase_5b_fixture_source_mismatch_blocks_before_provider_mutation() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();
        let mismatched_resource = GitHubExternalActionResourceBinding::new(
            "rcs2153/workflow-os",
            "main",
            "1111111111111111111111111111111111111111",
            "codex/substituted-source",
            "3333333333333333333333333333333333333333",
        )
        .expect("resource");
        let fact_source = draft_fact_source(
            &fixture,
            ExternalActionFixtureRevocationPosture::Current,
            mismatched_resource,
        );

        let error = execute_draft_with_source(
            &fixture,
            &lifecycle_store,
            &provider,
            &successor_store,
            &fact_source,
        )
        .expect_err("source mismatch");
        assert_eq!(
            error.code(),
            "external_action_connector_admission.current_facts.mismatch"
        );
        assert_eq!(provider.operation_count().expect("operation count"), 0);
    }

    #[test]
    fn phase_5c_pending_checks_wait_then_resume_and_mark_exact_successor_ready() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();

        execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("draft created");
        let waiting = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("evidence wait");
        assert_eq!(
            waiting.posture(),
            ExternalActionFixtureMarkReadyPosture::WaitingForEvidence
        );
        assert!(waiting.wait_condition_id().is_some());
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks green");
        let ready = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("marked ready");
        assert_eq!(
            ready.posture(),
            ExternalActionFixtureMarkReadyPosture::MarkedReady
        );
        assert_eq!(provider.operation_count().expect("operation count"), 2);

        let replay = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("ready replay");
        assert_eq!(
            replay.posture(),
            ExternalActionFixtureMarkReadyPosture::ExactReplay
        );
        assert_eq!(provider.operation_count().expect("operation count"), 2);
    }

    #[test]
    fn phase_5c_failed_checks_remain_waiting_without_connector_entry() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();

        execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("draft created");
        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::Failed)
            .expect("checks failed");
        let first = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("failed-check wait");
        let second = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("stable failed-check wait");
        assert_eq!(
            first.posture(),
            ExternalActionFixtureMarkReadyPosture::WaitingForEvidence
        );
        assert_eq!(first.wait_condition_id(), second.wait_condition_id());
        assert_eq!(provider.operation_count().expect("operation count"), 1);
    }

    #[test]
    fn phase_5c_connector_refusal_wait_resumes_without_second_authorization() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();

        execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("draft created");
        let waiting = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("head-change precondition wait");
        assert_eq!(
            waiting.posture(),
            ExternalActionFixtureMarkReadyPosture::WaitingForEvidence
        );
        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks green");
        provider
            .set_connector_posture(ExternalActionFixtureConnectorPosture::RefusedBeforeDispatch)
            .expect("connector refusal");
        let waiting = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("connector wait");
        assert_eq!(
            waiting.posture(),
            ExternalActionFixtureMarkReadyPosture::WaitingForConnector
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        provider
            .set_connector_posture(ExternalActionFixtureConnectorPosture::Available)
            .expect("connector available");
        let ready = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("marked ready");
        assert_eq!(
            ready.posture(),
            ExternalActionFixtureMarkReadyPosture::MarkedReady
        );
        assert_eq!(provider.operation_count().expect("operation count"), 2);
    }

    #[test]
    fn phase_5c_dispatch_time_connector_refusal_becomes_resumable_wait() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider = DispatchRaceProvider::new(
            timestamp("2026-07-26T10:24:00Z"),
            ProviderRaceKind::ConnectorRefused,
        );
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();

        execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("draft created");
        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks green");
        provider.arm();
        let waiting = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("dispatch refusal becomes wait");
        assert_eq!(
            waiting.posture(),
            ExternalActionFixtureMarkReadyPosture::WaitingForConnector
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        provider
            .set_connector_posture(ExternalActionFixtureConnectorPosture::Available)
            .expect("connector available");
        let ready = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("refusal wait resumes");
        assert_eq!(
            ready.posture(),
            ExternalActionFixtureMarkReadyPosture::MarkedReady
        );
        assert_eq!(provider.operation_count().expect("operation count"), 2);
    }

    #[test]
    fn phase_5c_dispatch_time_check_regression_becomes_resumable_wait() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider = DispatchRaceProvider::new(
            timestamp("2026-07-26T10:24:00Z"),
            ProviderRaceKind::ChecksRegressed,
        );
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();

        execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("draft created");
        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks green");
        provider.arm();
        let waiting = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("check regression becomes wait");
        assert_eq!(
            waiting.posture(),
            ExternalActionFixtureMarkReadyPosture::WaitingForEvidence
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks recover");
        let ready = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("check wait resumes");
        assert_eq!(
            ready.posture(),
            ExternalActionFixtureMarkReadyPosture::MarkedReady
        );
        assert_eq!(provider.operation_count().expect("operation count"), 2);
    }

    #[test]
    fn phase_5c_changed_head_and_revocation_fail_before_ready_connector_entry() {
        let fixture = fixture(CapabilityGrantLifecycle::Active);
        let root = TestRoot::new();
        let lifecycle_store = LocalExternalActionTransitionLifecycleStore::new(&root.0);
        let provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let successor_store = InMemoryExternalActionSuccessorBindingStore::default();

        execute_draft(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect("draft created");
        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks green");
        provider
            .set_reviewed_head_commitment(SpecContentHash::from_text(
                "3333333333333333333333333333333333333333",
            ))
            .expect("head changed");
        let changed = execute_ready(&fixture, &lifecycle_store, &provider, &successor_store)
            .expect_err("changed head rejected");
        assert_eq!(
            changed.code(),
            "external_action_connector_admission.ready.provider_resource_mismatch"
        );
        assert_eq!(provider.operation_count().expect("operation count"), 1);

        let fresh_provider =
            InMemoryExternalActionFixtureProvider::new(timestamp("2026-07-26T10:24:00Z"));
        let fresh_store = InMemoryExternalActionSuccessorBindingStore::default();
        let fresh_root = TestRoot::new();
        let fresh_lifecycle = LocalExternalActionTransitionLifecycleStore::new(&fresh_root.0);
        execute_draft(&fixture, &fresh_lifecycle, &fresh_provider, &fresh_store)
            .expect("fresh draft");
        let waiting = execute_ready(&fixture, &fresh_lifecycle, &fresh_provider, &fresh_store)
            .expect("revocation precondition wait");
        assert_eq!(
            waiting.posture(),
            ExternalActionFixtureMarkReadyPosture::WaitingForEvidence
        );
        fresh_provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("fresh checks green");
        let revoked_source = ready_fact_source(
            &fixture,
            ExternalActionFixtureRevocationPosture::Revoked,
            fixture.resource.clone(),
        );
        let revoked = execute_ready_with_source(
            &fixture,
            &fresh_lifecycle,
            &fresh_provider,
            &fresh_store,
            &revoked_source,
        )
        .expect_err("revoked source rejected");
        assert_eq!(
            revoked.code(),
            "external_action_connector_admission.current_facts.mismatch"
        );
        assert_eq!(
            fresh_provider.operation_count().expect("operation count"),
            1
        );
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
