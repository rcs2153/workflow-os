use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Mutex;

use serde::Serialize;

use crate::{
    ExternalActionAuthorizationProjection, ExternalActionAuthorizationProjectionId,
    ExternalActionSuccessorBinding, ExternalActionTransitionKind,
    ExternalActionTransitionOperationId, SpecContentHash, Timestamp, WorkflowOsError,
};

const COVERAGE_DOMAIN: &str = "workflow-os/external-action-fixture-source-coverage/v1";
const ANCHOR_DOMAIN: &str = "workflow-os/external-action-fixture-source-anchor/v1";
const OBSERVATION_DOMAIN: &str = "workflow-os/external-action-fixture-observation/v1";
const PROVIDER_REVISION_DOMAIN: &str = "workflow-os/external-action-fixture-provider-revision/v1";
const PROVIDER_RECEIPT_DOMAIN: &str = "workflow-os/external-action-fixture-provider-receipt/v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExternalActionFixtureSourceClass {
    ImmutableRun,
    ApprovalPresentation,
    ApprovalDecision,
    CurrentCapability,
    PolicyAndGovernance,
    ExecutionWindow,
    Revocation,
    ProviderResource,
    ProviderEvidence,
    TransitionLifecycle,
}

const REQUIRED_SOURCE_CLASSES: [ExternalActionFixtureSourceClass; 10] = [
    ExternalActionFixtureSourceClass::ImmutableRun,
    ExternalActionFixtureSourceClass::ApprovalPresentation,
    ExternalActionFixtureSourceClass::ApprovalDecision,
    ExternalActionFixtureSourceClass::CurrentCapability,
    ExternalActionFixtureSourceClass::PolicyAndGovernance,
    ExternalActionFixtureSourceClass::ExecutionWindow,
    ExternalActionFixtureSourceClass::Revocation,
    ExternalActionFixtureSourceClass::ProviderResource,
    ExternalActionFixtureSourceClass::ProviderEvidence,
    ExternalActionFixtureSourceClass::TransitionLifecycle,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExternalActionFixtureSourceDisposition {
    Available,
    Missing,
    Unsupported,
    Denied,
    Failed,
    Stale,
    Drifted,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub(crate) struct ExternalActionFixtureSourceRecord {
    class: ExternalActionFixtureSourceClass,
    source_revision: SpecContentHash,
    disposition: ExternalActionFixtureSourceDisposition,
}

impl ExternalActionFixtureSourceRecord {
    pub(crate) const fn new(
        class: ExternalActionFixtureSourceClass,
        source_revision: SpecContentHash,
        disposition: ExternalActionFixtureSourceDisposition,
    ) -> Self {
        Self {
            class,
            source_revision,
            disposition,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct ExternalActionFixtureSourceCoverage {
    records: Vec<ExternalActionFixtureSourceRecord>,
    coverage_commitment: SpecContentHash,
}

impl ExternalActionFixtureSourceCoverage {
    pub(crate) fn new(
        mut records: Vec<ExternalActionFixtureSourceRecord>,
    ) -> Result<Self, WorkflowOsError> {
        records.sort();
        let present = records
            .iter()
            .map(|record| record.class)
            .collect::<BTreeSet<_>>();
        let required = REQUIRED_SOURCE_CLASSES.into_iter().collect::<BTreeSet<_>>();
        if records.len() != REQUIRED_SOURCE_CLASSES.len() || present != required {
            return Err(fixture_error(
                "coverage.incomplete",
                "external action fixture source coverage is incomplete or duplicated",
            ));
        }
        let coverage_commitment = commitment(COVERAGE_DOMAIN, &records)?;
        Ok(Self {
            records,
            coverage_commitment,
        })
    }

    pub(crate) fn require_complete(&self) -> Result<(), WorkflowOsError> {
        if self
            .records
            .iter()
            .any(|record| record.disposition != ExternalActionFixtureSourceDisposition::Available)
        {
            return Err(fixture_error(
                "coverage.unavailable",
                "external action fixture required source coverage is unavailable",
            ));
        }
        Ok(())
    }

    pub(crate) const fn commitment(&self) -> &SpecContentHash {
        &self.coverage_commitment
    }
}

impl fmt::Debug for ExternalActionFixtureSourceCoverage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureSourceCoverage")
            .field("source_count", &self.records.len())
            .field("coverage_commitment", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct ExternalActionFixtureSourceAnchor {
    coverage_commitment: SpecContentHash,
    provider_revision: SpecContentHash,
    anchor_commitment: SpecContentHash,
}

impl ExternalActionFixtureSourceAnchor {
    pub(crate) fn new(
        coverage: &ExternalActionFixtureSourceCoverage,
        provider_revision: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        coverage.require_complete()?;
        let anchor_commitment =
            commitment(ANCHOR_DOMAIN, &(coverage.commitment(), &provider_revision))?;
        Ok(Self {
            coverage_commitment: coverage.commitment().clone(),
            provider_revision,
            anchor_commitment,
        })
    }

    pub(crate) fn verify_unchanged(&self, current: &Self) -> Result<(), WorkflowOsError> {
        if self != current {
            return Err(fixture_error(
                "anchor.drifted",
                "external action fixture source anchor drifted during composition",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for ExternalActionFixtureSourceAnchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureSourceAnchor")
            .field("coverage_commitment", &"[REDACTED]")
            .field("provider_revision", &"[REDACTED]")
            .field("anchor_commitment", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct ExternalActionFixtureProviderObservation {
    provider_revision: SpecContentHash,
    repository_commitment: SpecContentHash,
    base_target_commitment: SpecContentHash,
    reviewed_head_commitment: SpecContentHash,
    required_checks_commitment: SpecContentHash,
    pull_request_state_commitment: SpecContentHash,
    required_checks_posture: ExternalActionFixtureRequiredChecksPosture,
    connector_posture: ExternalActionFixtureConnectorPosture,
    observed_at: Timestamp,
    observation_commitment: SpecContentHash,
}

impl ExternalActionFixtureProviderObservation {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        provider_revision: SpecContentHash,
        repository_commitment: SpecContentHash,
        base_target_commitment: SpecContentHash,
        reviewed_head_commitment: SpecContentHash,
        required_checks_commitment: SpecContentHash,
        pull_request_state_commitment: SpecContentHash,
        required_checks_posture: ExternalActionFixtureRequiredChecksPosture,
        connector_posture: ExternalActionFixtureConnectorPosture,
        observed_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        let observation_commitment = commitment(
            OBSERVATION_DOMAIN,
            &(
                &provider_revision,
                &repository_commitment,
                &base_target_commitment,
                &reviewed_head_commitment,
                &required_checks_commitment,
                &pull_request_state_commitment,
                required_checks_posture,
                connector_posture,
                observed_at,
            ),
        )?;
        Ok(Self {
            provider_revision,
            repository_commitment,
            base_target_commitment,
            reviewed_head_commitment,
            required_checks_commitment,
            pull_request_state_commitment,
            required_checks_posture,
            connector_posture,
            observed_at,
            observation_commitment,
        })
    }

    pub(crate) const fn provider_revision(&self) -> &SpecContentHash {
        &self.provider_revision
    }

    pub(crate) const fn observation_commitment(&self) -> &SpecContentHash {
        &self.observation_commitment
    }

    pub(crate) const fn repository_commitment(&self) -> &SpecContentHash {
        &self.repository_commitment
    }

    pub(crate) const fn base_target_commitment(&self) -> &SpecContentHash {
        &self.base_target_commitment
    }

    pub(crate) const fn reviewed_head_commitment(&self) -> &SpecContentHash {
        &self.reviewed_head_commitment
    }

    pub(crate) const fn required_checks_posture(
        &self,
    ) -> ExternalActionFixtureRequiredChecksPosture {
        self.required_checks_posture
    }

    pub(crate) const fn connector_posture(&self) -> ExternalActionFixtureConnectorPosture {
        self.connector_posture
    }
}

impl fmt::Debug for ExternalActionFixtureProviderObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureProviderObservation")
            .field("provider_revision", &"[REDACTED]")
            .field("observed_at", &self.observed_at)
            .field("observation_commitment", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct ExternalActionFixtureProviderOperation {
    operation_id: ExternalActionTransitionOperationId,
    transition_kind: ExternalActionTransitionKind,
    expected_provider_revision: SpecContentHash,
    expected_successor_number: Option<u64>,
}

impl ExternalActionFixtureProviderOperation {
    pub(crate) const fn new(
        operation_id: ExternalActionTransitionOperationId,
        transition_kind: ExternalActionTransitionKind,
        expected_provider_revision: SpecContentHash,
    ) -> Self {
        Self {
            operation_id,
            transition_kind,
            expected_provider_revision,
            expected_successor_number: None,
        }
    }

    pub(crate) const fn for_successor(
        operation_id: ExternalActionTransitionOperationId,
        transition_kind: ExternalActionTransitionKind,
        expected_provider_revision: SpecContentHash,
        expected_successor_number: u64,
    ) -> Self {
        Self {
            operation_id,
            transition_kind,
            expected_provider_revision,
            expected_successor_number: Some(expected_successor_number),
        }
    }

    pub(crate) const fn operation_id(&self) -> &ExternalActionTransitionOperationId {
        &self.operation_id
    }

    pub(crate) const fn expected_provider_revision(&self) -> &SpecContentHash {
        &self.expected_provider_revision
    }

    pub(crate) fn operation_content_commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        commitment(
            "workflow-os/external-action-fixture-provider-operation/v1",
            &(
                &self.operation_id,
                self.transition_kind,
                self.expected_successor_number,
            ),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExternalActionFixtureRequiredChecksPosture {
    Pending,
    Failed,
    TerminalGreen,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExternalActionFixtureConnectorPosture {
    Available,
    RefusedBeforeDispatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionFixtureProviderApplyPosture {
    Applied,
    ExactReplay,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ExternalActionFixtureProviderApplyResult {
    posture: ExternalActionFixtureProviderApplyPosture,
    operation_content_commitment: SpecContentHash,
    provider_revision: SpecContentHash,
    provider_receipt_commitment: SpecContentHash,
    created_successor: Option<ExternalActionFixtureCreatedSuccessor>,
    exact_merge_result: Option<ExternalActionFixtureExactMergeResult>,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct ExternalActionFixtureCreatedSuccessor {
    pull_request_number: u64,
    observed_at: Timestamp,
}

impl ExternalActionFixtureCreatedSuccessor {
    pub(crate) const fn pull_request_number(&self) -> u64 {
        self.pull_request_number
    }

    pub(crate) const fn observed_at(&self) -> Timestamp {
        self.observed_at
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct ExternalActionFixtureExactMergeResult {
    merge_sha_commitment: SpecContentHash,
    merge_result_commitment: SpecContentHash,
}

impl ExternalActionFixtureExactMergeResult {
    pub(crate) const fn merge_sha_commitment(&self) -> &SpecContentHash {
        &self.merge_sha_commitment
    }

    pub(crate) const fn merge_result_commitment(&self) -> &SpecContentHash {
        &self.merge_result_commitment
    }
}

impl fmt::Debug for ExternalActionFixtureExactMergeResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureExactMergeResult")
            .field("merge_sha_commitment", &"[REDACTED]")
            .field("merge_result_commitment", &"[REDACTED]")
            .finish()
    }
}

impl ExternalActionFixtureProviderApplyResult {
    pub(crate) const fn posture(&self) -> ExternalActionFixtureProviderApplyPosture {
        self.posture
    }

    pub(crate) const fn provider_receipt_commitment(&self) -> &SpecContentHash {
        &self.provider_receipt_commitment
    }

    pub(crate) fn validate_for_operation(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<(), WorkflowOsError> {
        if self.operation_content_commitment != operation.operation_content_commitment()? {
            return Err(fixture_error(
                "provider.result_binding_mismatch",
                "fixture provider result does not match the exact requested operation",
            ));
        }
        let shape_valid = match operation.transition_kind {
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft => {
                self.created_successor.is_some() && self.exact_merge_result.is_none()
            }
            ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen => {
                self.created_successor.is_none() && self.exact_merge_result.is_none()
            }
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead => {
                self.created_successor.is_none() && self.exact_merge_result.is_some()
            }
            _ => false,
        };
        if !shape_valid {
            return Err(fixture_error(
                "provider.result_shape_invalid",
                "fixture provider result shape does not match the requested transition",
            ));
        }
        Ok(())
    }

    pub(crate) const fn created_successor(&self) -> Option<&ExternalActionFixtureCreatedSuccessor> {
        self.created_successor.as_ref()
    }

    pub(crate) const fn exact_merge_result(
        &self,
    ) -> Option<&ExternalActionFixtureExactMergeResult> {
        self.exact_merge_result.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn substituted_create_result(observed_at: Timestamp) -> Self {
        Self {
            posture: ExternalActionFixtureProviderApplyPosture::Applied,
            operation_content_commitment: SpecContentHash::from_text("substituted-operation"),
            provider_revision: SpecContentHash::from_text("substituted-revision"),
            provider_receipt_commitment: SpecContentHash::from_text("substituted-receipt"),
            created_successor: Some(ExternalActionFixtureCreatedSuccessor {
                pull_request_number: 999,
                observed_at,
            }),
            exact_merge_result: None,
        }
    }
}

impl fmt::Debug for ExternalActionFixtureProviderApplyResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureProviderApplyResult")
            .field("posture", &self.posture)
            .field("operation_content_commitment", &"[REDACTED]")
            .field("provider_revision", &"[REDACTED]")
            .field("provider_receipt_commitment", &"[REDACTED]")
            .field("created_successor", &self.created_successor.is_some())
            .field("exact_merge_result", &self.exact_merge_result.is_some())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionFixtureProviderDispatchOutcome {
    Applied,
    MayHaveStartedAmbiguous,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ExternalActionFixtureLookupCoverage {
    operation_content_commitment: SpecContentHash,
    provider_revision: SpecContentHash,
    coverage_commitment: SpecContentHash,
}

impl ExternalActionFixtureLookupCoverage {
    pub(crate) fn new(
        operation: &ExternalActionFixtureProviderOperation,
        provider_revision: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        let operation_content_commitment = operation.operation_content_commitment()?;
        let coverage_commitment = commitment(
            "workflow-os/external-action-fixture-lookup-coverage/v1",
            &(&operation_content_commitment, &provider_revision),
        )?;
        Ok(Self {
            operation_content_commitment,
            provider_revision,
            coverage_commitment,
        })
    }

    pub(crate) fn validate_for_operation(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<(), WorkflowOsError> {
        let expected = Self::new(operation, self.provider_revision.clone())?;
        if &expected != self {
            return Err(fixture_error(
                "provider.lookup_coverage_mismatch",
                "fixture provider lookup coverage does not match the exact operation",
            ));
        }
        Ok(())
    }

    pub(crate) fn validate_authoritative_not_found_for_operation(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<(), WorkflowOsError> {
        self.validate_for_operation(operation)?;
        if &self.provider_revision != operation.expected_provider_revision() {
            return Err(fixture_error(
                "provider.lookup_coverage_not_authoritative",
                "fixture provider absence lookup does not close the attempted revision window",
            ));
        }
        Ok(())
    }

    pub(crate) const fn coverage_commitment(&self) -> &SpecContentHash {
        &self.coverage_commitment
    }
}

impl fmt::Debug for ExternalActionFixtureLookupCoverage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureLookupCoverage")
            .field("operation_content_commitment", &"[REDACTED]")
            .field("provider_revision", &"[REDACTED]")
            .field("coverage_commitment", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum ExternalActionFixtureProviderLookupOutcome {
    ExactFound {
        result: ExternalActionFixtureProviderApplyResult,
        coverage: ExternalActionFixtureLookupCoverage,
    },
    AuthoritativeNotFound {
        coverage: ExternalActionFixtureLookupCoverage,
    },
    Inconclusive {
        coverage: ExternalActionFixtureLookupCoverage,
    },
    Unavailable,
}

impl fmt::Debug for ExternalActionFixtureProviderLookupOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureProviderLookupOutcome")
            .field(
                "posture",
                &match self {
                    Self::ExactFound { .. } => "exact_found",
                    Self::AuthoritativeNotFound { .. } => "authoritative_not_found",
                    Self::Inconclusive { .. } => "inconclusive",
                    Self::Unavailable => "unavailable",
                },
            )
            .finish()
    }
}

pub(crate) trait ExternalActionFixtureProviderSource {
    fn observe(&self) -> Result<ExternalActionFixtureProviderObservation, WorkflowOsError>;

    fn compare_and_apply(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<
        (
            ExternalActionFixtureProviderDispatchOutcome,
            Option<ExternalActionFixtureProviderApplyResult>,
        ),
        WorkflowOsError,
    >;

    fn lookup_operation(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<ExternalActionFixtureProviderLookupOutcome, WorkflowOsError>;
}

struct FixtureProviderState {
    revision_sequence: u64,
    revision: SpecContentHash,
    next_pull_request_number: u64,
    pull_request: Option<FixturePullRequest>,
    required_checks_posture: ExternalActionFixtureRequiredChecksPosture,
    connector_posture: ExternalActionFixtureConnectorPosture,
    repository_commitment: SpecContentHash,
    base_target_commitment: SpecContentHash,
    reviewed_head_commitment: SpecContentHash,
    receipts: BTreeMap<ExternalActionTransitionOperationId, FixtureProviderReceipt>,
    ambiguous_next_apply: bool,
    compare_and_apply_count: usize,
    lookup_count: usize,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
struct FixturePullRequest {
    number: u64,
    reviewed_head_commitment: SpecContentHash,
    ready: bool,
    exact_merge_result: Option<ExternalActionFixtureExactMergeResult>,
}

struct FixtureProviderReceipt {
    operation_content_commitment: SpecContentHash,
    receipt_commitment: SpecContentHash,
    resulting_revision: SpecContentHash,
    created_successor: Option<ExternalActionFixtureCreatedSuccessor>,
    exact_merge_result: Option<ExternalActionFixtureExactMergeResult>,
}

pub(crate) struct InMemoryExternalActionFixtureProvider {
    state: Mutex<FixtureProviderState>,
    observed_at: Timestamp,
}

impl InMemoryExternalActionFixtureProvider {
    pub(crate) fn new(observed_at: Timestamp) -> Self {
        Self {
            state: Mutex::new(FixtureProviderState {
                revision_sequence: 0,
                revision: SpecContentHash::from_text("provider-revision-0"),
                next_pull_request_number: 42,
                pull_request: None,
                required_checks_posture: ExternalActionFixtureRequiredChecksPosture::Pending,
                connector_posture: ExternalActionFixtureConnectorPosture::Available,
                repository_commitment: SpecContentHash::from_text("rcs2153/workflow-os"),
                base_target_commitment: SpecContentHash::from_text(
                    "main@1111111111111111111111111111111111111111",
                ),
                reviewed_head_commitment: SpecContentHash::from_text(
                    "2222222222222222222222222222222222222222",
                ),
                receipts: BTreeMap::new(),
                ambiguous_next_apply: false,
                compare_and_apply_count: 0,
                lookup_count: 0,
            }),
            observed_at,
        }
    }

    #[cfg(test)]
    pub(crate) fn operation_count(&self) -> Result<usize, WorkflowOsError> {
        self.state
            .lock()
            .map(|state| state.receipts.len())
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))
    }

    #[cfg(test)]
    pub(crate) fn compare_and_apply_count(&self) -> Result<usize, WorkflowOsError> {
        self.state
            .lock()
            .map(|state| state.compare_and_apply_count)
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))
    }

    #[cfg(test)]
    pub(crate) fn lookup_count(&self) -> Result<usize, WorkflowOsError> {
        self.state
            .lock()
            .map(|state| state.lookup_count)
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))
    }

    #[cfg(test)]
    pub(crate) fn set_required_checks_posture(
        &self,
        posture: ExternalActionFixtureRequiredChecksPosture,
    ) -> Result<(), WorkflowOsError> {
        self.mutate_fixture_state(|state| state.required_checks_posture = posture)
    }

    #[cfg(test)]
    pub(crate) fn set_connector_posture(
        &self,
        posture: ExternalActionFixtureConnectorPosture,
    ) -> Result<(), WorkflowOsError> {
        self.mutate_fixture_state(|state| state.connector_posture = posture)
    }

    #[cfg(test)]
    pub(crate) fn set_reviewed_head_commitment(
        &self,
        commitment: SpecContentHash,
    ) -> Result<(), WorkflowOsError> {
        self.mutate_fixture_state(|state| state.reviewed_head_commitment = commitment)
    }

    #[cfg(test)]
    pub(crate) fn make_next_apply_ambiguous(&self) -> Result<(), WorkflowOsError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        state.ambiguous_next_apply = true;
        Ok(())
    }

    #[cfg(test)]
    fn mutate_fixture_state(
        &self,
        mutate: impl FnOnce(&mut FixtureProviderState),
    ) -> Result<(), WorkflowOsError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        mutate(&mut state);
        state.revision_sequence += 1;
        state.revision = commitment(
            PROVIDER_REVISION_DOMAIN,
            &(
                &state.repository_commitment,
                &state.base_target_commitment,
                &state.reviewed_head_commitment,
                state.required_checks_posture,
                state.connector_posture,
                &state.pull_request,
                state.revision_sequence,
            ),
        )?;
        Ok(())
    }
}

impl ExternalActionFixtureProviderSource for InMemoryExternalActionFixtureProvider {
    fn observe(&self) -> Result<ExternalActionFixtureProviderObservation, WorkflowOsError> {
        let state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        let required_checks_commitment = commitment(
            "workflow-os/external-action-fixture-checks/v1",
            &(
                &state.reviewed_head_commitment,
                state.required_checks_posture,
            ),
        )?;
        let pull_request_state_commitment = commitment(
            "workflow-os/external-action-fixture-pull-request-state/v1",
            &state.pull_request,
        )?;
        ExternalActionFixtureProviderObservation::new(
            state.revision.clone(),
            state.repository_commitment.clone(),
            state.base_target_commitment.clone(),
            state.reviewed_head_commitment.clone(),
            required_checks_commitment,
            pull_request_state_commitment,
            state.required_checks_posture,
            state.connector_posture,
            self.observed_at,
        )
    }

    #[allow(clippy::too_many_lines)]
    fn compare_and_apply(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<
        (
            ExternalActionFixtureProviderDispatchOutcome,
            Option<ExternalActionFixtureProviderApplyResult>,
        ),
        WorkflowOsError,
    > {
        let mut state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        state.compare_and_apply_count += 1;
        let operation_content_commitment = operation.operation_content_commitment()?;
        if let Some(receipt) = state.receipts.get(&operation.operation_id) {
            if receipt.operation_content_commitment != operation_content_commitment {
                return Err(fixture_error(
                    "provider.operation_conflict",
                    "fixture provider operation identity conflicts with prior content",
                ));
            }
            return Ok((
                ExternalActionFixtureProviderDispatchOutcome::Applied,
                Some(ExternalActionFixtureProviderApplyResult {
                    posture: ExternalActionFixtureProviderApplyPosture::ExactReplay,
                    operation_content_commitment,
                    provider_revision: receipt.resulting_revision.clone(),
                    provider_receipt_commitment: receipt.receipt_commitment.clone(),
                    created_successor: receipt.created_successor.clone(),
                    exact_merge_result: receipt.exact_merge_result.clone(),
                }),
            ));
        }
        if state.revision != operation.expected_provider_revision {
            return Err(fixture_error(
                "provider.revision_drift",
                "fixture provider revision changed before operation",
            ));
        }
        if state.connector_posture == ExternalActionFixtureConnectorPosture::RefusedBeforeDispatch {
            return Err(fixture_error(
                "provider.refused_before_dispatch",
                "fixture provider refused before dispatch",
            ));
        }
        match operation.transition_kind {
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft => {
                if state.pull_request.is_some() || operation.expected_successor_number.is_some() {
                    return Err(fixture_error(
                        "provider.successor_conflict",
                        "fixture provider draft creation conflicts with successor state",
                    ));
                }
            }
            ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen => {
                let Some(pull_request) = state.pull_request.as_ref() else {
                    return Err(fixture_error(
                        "provider.successor_missing",
                        "fixture provider successor is missing",
                    ));
                };
                if operation.expected_successor_number != Some(pull_request.number)
                    || pull_request.reviewed_head_commitment != state.reviewed_head_commitment
                    || pull_request.ready
                {
                    return Err(fixture_error(
                        "provider.successor_conflict",
                        "fixture provider successor does not match the mark-ready operation",
                    ));
                }
                if state.required_checks_posture
                    != ExternalActionFixtureRequiredChecksPosture::TerminalGreen
                {
                    return Err(fixture_error(
                        "provider.checks_not_green",
                        "fixture provider required checks are not terminal green",
                    ));
                }
            }
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead => {
                let Some(pull_request) = state.pull_request.as_ref() else {
                    return Err(fixture_error(
                        "provider.successor_missing",
                        "fixture provider successor is missing",
                    ));
                };
                if operation.expected_successor_number != Some(pull_request.number)
                    || pull_request.reviewed_head_commitment != state.reviewed_head_commitment
                    || !pull_request.ready
                    || pull_request.exact_merge_result.is_some()
                {
                    return Err(fixture_error(
                        "provider.successor_conflict",
                        "fixture provider successor does not match the squash-merge operation",
                    ));
                }
                if state.required_checks_posture
                    != ExternalActionFixtureRequiredChecksPosture::TerminalGreen
                {
                    return Err(fixture_error(
                        "provider.checks_not_green",
                        "fixture provider required checks are not terminal green",
                    ));
                }
            }
            _ => {
                return Err(fixture_error(
                    "provider.transition_unsupported",
                    "fixture provider transition is not implemented in this phase",
                ));
            }
        }
        let prior_revision = state.revision.clone();
        state.revision_sequence += 1;
        let resulting_revision = commitment(
            PROVIDER_REVISION_DOMAIN,
            &(
                &prior_revision,
                &operation.operation_id,
                operation.transition_kind,
                state.revision_sequence,
            ),
        )?;
        let created_successor = (operation.transition_kind
            == ExternalActionTransitionKind::GitHubPullRequestCreateDraft)
            .then(|| {
                let successor = ExternalActionFixtureCreatedSuccessor {
                    pull_request_number: state.next_pull_request_number,
                    observed_at: self.observed_at,
                };
                state.next_pull_request_number += 1;
                successor
            });
        if let Some(successor) = &created_successor {
            state.pull_request = Some(FixturePullRequest {
                number: successor.pull_request_number,
                reviewed_head_commitment: state.reviewed_head_commitment.clone(),
                ready: false,
                exact_merge_result: None,
            });
        } else if operation.transition_kind
            == ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen
        {
            let Some(pull_request) = state.pull_request.as_mut() else {
                return Err(fixture_error(
                    "provider.successor_missing",
                    "fixture provider successor is missing",
                ));
            };
            pull_request.ready = true;
        }
        let exact_merge_result = if operation.transition_kind
            == ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead
        {
            let pull_request = state.pull_request.as_ref().ok_or_else(|| {
                fixture_error(
                    "provider.successor_missing",
                    "fixture provider successor is missing",
                )
            })?;
            let merge_sha_commitment = commitment(
                "workflow-os/external-action-fixture-merge-sha/v1",
                &(
                    &operation_content_commitment,
                    pull_request.number,
                    &pull_request.reviewed_head_commitment,
                ),
            )?;
            let merge_result_commitment = commitment(
                "workflow-os/external-action-fixture-exact-merge-result/v1",
                &(
                    &operation_content_commitment,
                    pull_request.number,
                    &pull_request.reviewed_head_commitment,
                    &merge_sha_commitment,
                    &resulting_revision,
                ),
            )?;
            Some(ExternalActionFixtureExactMergeResult {
                merge_sha_commitment,
                merge_result_commitment,
            })
        } else {
            None
        };
        if let Some(result) = &exact_merge_result {
            let pull_request = state.pull_request.as_mut().ok_or_else(|| {
                fixture_error(
                    "provider.successor_missing",
                    "fixture provider successor is missing",
                )
            })?;
            pull_request.exact_merge_result = Some(result.clone());
        }
        let receipt = commitment(
            PROVIDER_RECEIPT_DOMAIN,
            &(
                &operation.operation_id,
                &operation_content_commitment,
                &prior_revision,
                &resulting_revision,
                &created_successor,
                &exact_merge_result,
            ),
        )?;
        state.revision.clone_from(&resulting_revision);
        state.receipts.insert(
            operation.operation_id.clone(),
            FixtureProviderReceipt {
                operation_content_commitment: operation_content_commitment.clone(),
                receipt_commitment: receipt.clone(),
                resulting_revision: resulting_revision.clone(),
                created_successor: created_successor.clone(),
                exact_merge_result: exact_merge_result.clone(),
            },
        );
        let result = ExternalActionFixtureProviderApplyResult {
            posture: ExternalActionFixtureProviderApplyPosture::Applied,
            operation_content_commitment,
            provider_revision: resulting_revision,
            provider_receipt_commitment: receipt,
            created_successor,
            exact_merge_result,
        };
        if state.ambiguous_next_apply {
            state.ambiguous_next_apply = false;
            Ok((
                ExternalActionFixtureProviderDispatchOutcome::MayHaveStartedAmbiguous,
                None,
            ))
        } else {
            Ok((
                ExternalActionFixtureProviderDispatchOutcome::Applied,
                Some(result),
            ))
        }
    }

    fn lookup_operation(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<ExternalActionFixtureProviderLookupOutcome, WorkflowOsError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        state.lookup_count += 1;
        let operation_content_commitment = operation.operation_content_commitment()?;
        let coverage = ExternalActionFixtureLookupCoverage::new(operation, state.revision.clone())?;
        let Some(receipt) = state.receipts.get(&operation.operation_id) else {
            return Ok(
                ExternalActionFixtureProviderLookupOutcome::AuthoritativeNotFound { coverage },
            );
        };
        if receipt.operation_content_commitment != operation_content_commitment {
            return Err(fixture_error(
                "provider.operation_conflict",
                "fixture provider operation identity conflicts with prior content",
            ));
        }
        Ok(ExternalActionFixtureProviderLookupOutcome::ExactFound {
            result: ExternalActionFixtureProviderApplyResult {
                posture: ExternalActionFixtureProviderApplyPosture::ExactReplay,
                operation_content_commitment,
                provider_revision: receipt.resulting_revision.clone(),
                provider_receipt_commitment: receipt.receipt_commitment.clone(),
                created_successor: receipt.created_successor.clone(),
                exact_merge_result: receipt.exact_merge_result.clone(),
            },
            coverage,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionSuccessorBindingWritePosture {
    Inserted,
    ExactReplay,
}

pub(crate) trait ExternalActionSuccessorBindingStore {
    fn put_exact(
        &self,
        projection: &ExternalActionAuthorizationProjection,
        binding: &ExternalActionSuccessorBinding,
    ) -> Result<ExternalActionSuccessorBindingWritePosture, WorkflowOsError>;

    fn read_exact(
        &self,
        projection: &ExternalActionAuthorizationProjection,
    ) -> Result<Option<ExternalActionSuccessorBinding>, WorkflowOsError>;
}

#[derive(Default)]
pub(crate) struct InMemoryExternalActionSuccessorBindingStore {
    entries:
        Mutex<BTreeMap<ExternalActionAuthorizationProjectionId, ExternalActionSuccessorBinding>>,
}

impl ExternalActionSuccessorBindingStore for InMemoryExternalActionSuccessorBindingStore {
    fn put_exact(
        &self,
        projection: &ExternalActionAuthorizationProjection,
        binding: &ExternalActionSuccessorBinding,
    ) -> Result<ExternalActionSuccessorBindingWritePosture, WorkflowOsError> {
        binding.validate_for_projection(projection)?;
        let mut entries = self.entries.lock().map_err(|_| {
            fixture_error(
                "successor.store_unavailable",
                "external action successor-binding store is unavailable",
            )
        })?;
        match entries.get(projection.projection_id()) {
            Some(existing) if existing == binding => {
                Ok(ExternalActionSuccessorBindingWritePosture::ExactReplay)
            }
            Some(_) => Err(fixture_error(
                "successor.conflict",
                "external action successor binding conflicts with durable Core state",
            )),
            None => {
                entries.insert(projection.projection_id().clone(), binding.clone());
                Ok(ExternalActionSuccessorBindingWritePosture::Inserted)
            }
        }
    }

    fn read_exact(
        &self,
        projection: &ExternalActionAuthorizationProjection,
    ) -> Result<Option<ExternalActionSuccessorBinding>, WorkflowOsError> {
        let entries = self.entries.lock().map_err(|_| {
            fixture_error(
                "successor.store_unavailable",
                "external action successor-binding store is unavailable",
            )
        })?;
        let binding = entries.get(projection.projection_id()).cloned();
        if let Some(binding) = &binding {
            binding.validate_for_projection(projection)?;
        }
        Ok(binding)
    }
}

impl fmt::Debug for InMemoryExternalActionSuccessorBindingStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryExternalActionSuccessorBindingStore")
            .finish_non_exhaustive()
    }
}

fn commitment<T: Serialize>(
    domain: &'static str,
    value: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(&(domain, value)).map_err(|_| {
        fixture_error(
            "commitment.failed",
            "external action fixture commitment failed",
        )
    })?;
    Ok(SpecContentHash::from_bytes(bytes))
}

fn fixture_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("external_action_fixture_lifecycle.{code}"), message)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::sync::{Arc, Barrier};
    use std::thread;

    use super::*;
    use crate::{
        derive_external_action_successor_binding, external_action_authorization,
        ExternalActionTransitionOperation,
    };

    fn time(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("time")
    }

    fn records(
        override_record: Option<(
            ExternalActionFixtureSourceClass,
            ExternalActionFixtureSourceDisposition,
        )>,
    ) -> Vec<ExternalActionFixtureSourceRecord> {
        REQUIRED_SOURCE_CLASSES
            .into_iter()
            .map(|class| {
                let disposition = override_record
                    .filter(|(candidate, _)| *candidate == class)
                    .map_or(
                        ExternalActionFixtureSourceDisposition::Available,
                        |(_, value)| value,
                    );
                ExternalActionFixtureSourceRecord::new(
                    class,
                    SpecContentHash::from_text(&format!("revision-{class:?}")),
                    disposition,
                )
            })
            .collect()
    }

    fn projection() -> ExternalActionAuthorizationProjection {
        external_action_authorization::external_action_test_projection().expect("projection")
    }

    fn apply_result(
        provider: &InMemoryExternalActionFixtureProvider,
        operation: &ExternalActionFixtureProviderOperation,
        context: &str,
    ) -> ExternalActionFixtureProviderApplyResult {
        let (posture, result) = provider.compare_and_apply(operation).expect(context);
        assert_eq!(
            posture,
            ExternalActionFixtureProviderDispatchOutcome::Applied
        );
        result.expect("applied provider result")
    }

    fn fixture_operation(
        projection: &ExternalActionAuthorizationProjection,
        transition_kind: ExternalActionTransitionKind,
        revision: SpecContentHash,
    ) -> ExternalActionFixtureProviderOperation {
        let operation =
            ExternalActionTransitionOperation::from_projection(projection, transition_kind)
                .expect("operation");
        ExternalActionFixtureProviderOperation::new(
            operation.operation_id().clone(),
            operation.transition_kind(),
            revision,
        )
    }

    fn fixture_successor_operation(
        projection: &ExternalActionAuthorizationProjection,
        transition_kind: ExternalActionTransitionKind,
        revision: SpecContentHash,
        successor_number: u64,
    ) -> ExternalActionFixtureProviderOperation {
        let operation =
            ExternalActionTransitionOperation::from_projection(projection, transition_kind)
                .expect("operation");
        ExternalActionFixtureProviderOperation::for_successor(
            operation.operation_id().clone(),
            operation.transition_kind(),
            revision,
            successor_number,
        )
    }

    fn create_and_mark_ready(
        provider: &InMemoryExternalActionFixtureProvider,
        projection: &ExternalActionAuthorizationProjection,
    ) {
        let create = fixture_operation(
            projection,
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            provider
                .observe()
                .expect("observe create")
                .provider_revision()
                .clone(),
        );
        apply_result(provider, &create, "create draft");
        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks green");
        let ready = fixture_successor_operation(
            projection,
            ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
            provider
                .observe()
                .expect("observe ready")
                .provider_revision()
                .clone(),
            42,
        );
        apply_result(provider, &ready, "mark ready");
    }

    #[test]
    fn coverage_requires_every_closed_source_class_and_available_posture() {
        let complete = ExternalActionFixtureSourceCoverage::new(records(None)).expect("coverage");
        complete.require_complete().expect("complete");

        let mut missing = records(None);
        missing.pop();
        assert_eq!(
            ExternalActionFixtureSourceCoverage::new(missing)
                .expect_err("missing class")
                .code(),
            "external_action_fixture_lifecycle.coverage.incomplete"
        );

        let stale = ExternalActionFixtureSourceCoverage::new(records(Some((
            ExternalActionFixtureSourceClass::ProviderEvidence,
            ExternalActionFixtureSourceDisposition::Stale,
        ))))
        .expect("coverage");
        assert_eq!(
            stale.require_complete().expect_err("stale source").code(),
            "external_action_fixture_lifecycle.coverage.unavailable"
        );
    }

    #[test]
    fn source_anchor_rejects_provider_or_coverage_drift() {
        let coverage = ExternalActionFixtureSourceCoverage::new(records(None)).expect("coverage");
        let first = ExternalActionFixtureSourceAnchor::new(
            &coverage,
            SpecContentHash::from_text("provider-revision-1"),
        )
        .expect("anchor");
        let same = ExternalActionFixtureSourceAnchor::new(
            &coverage,
            SpecContentHash::from_text("provider-revision-1"),
        )
        .expect("anchor");
        first.verify_unchanged(&same).expect("stable");

        let changed = ExternalActionFixtureSourceAnchor::new(
            &coverage,
            SpecContentHash::from_text("provider-revision-2"),
        )
        .expect("anchor");
        assert_eq!(
            first.verify_unchanged(&changed).expect_err("drift").code(),
            "external_action_fixture_lifecycle.anchor.drifted"
        );
    }

    #[test]
    fn provider_compare_and_operate_rejects_drift_and_replays_exact_operation() {
        let provider = InMemoryExternalActionFixtureProvider::new(time("2026-11-01T20:10:00Z"));
        let projection_value = projection();
        let observed = provider.observe().expect("observe");
        let operation = fixture_operation(
            &projection_value,
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            observed.provider_revision().clone(),
        );
        let applied = apply_result(&provider, &operation, "apply");
        assert_eq!(
            applied.posture,
            ExternalActionFixtureProviderApplyPosture::Applied
        );

        let replay = apply_result(&provider, &operation, "replay");
        assert_eq!(
            replay.posture,
            ExternalActionFixtureProviderApplyPosture::ExactReplay
        );
        assert_eq!(
            replay.provider_receipt_commitment,
            applied.provider_receipt_commitment
        );
        assert_eq!(replay.provider_revision, applied.provider_revision);

        provider
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::TerminalGreen)
            .expect("checks green");
        let current = provider.observe().expect("observe");
        let ready_operation = ExternalActionTransitionOperation::from_projection(
            &projection_value,
            ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
        )
        .expect("ready operation");
        let advance = ExternalActionFixtureProviderOperation::for_successor(
            ready_operation.operation_id().clone(),
            ready_operation.transition_kind(),
            current.provider_revision().clone(),
            42,
        );
        apply_result(&provider, &advance, "advance");
        let substituted_successor = ExternalActionFixtureProviderOperation::for_successor(
            ready_operation.operation_id().clone(),
            ready_operation.transition_kind(),
            current.provider_revision().clone(),
            43,
        );
        assert_eq!(
            provider
                .compare_and_apply(&substituted_successor)
                .expect_err("successor substitution rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.operation_conflict"
        );
        assert_eq!(
            provider
                .lookup_operation(&substituted_successor)
                .expect_err("reconciliation successor substitution rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.operation_conflict"
        );
        let replay_after_advance = apply_result(&provider, &operation, "replay");
        assert_eq!(replay_after_advance, replay);

        let different = fixture_operation(
            &projection_value,
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            SpecContentHash::from_text("provider-revision-0"),
        );
        assert_eq!(
            provider
                .compare_and_apply(&different)
                .expect_err("revision drift")
                .code(),
            "external_action_fixture_lifecycle.provider.revision_drift"
        );
    }

    #[test]
    fn provider_compare_and_operate_admits_only_one_concurrent_mutation() {
        let provider = Arc::new(InMemoryExternalActionFixtureProvider::new(time(
            "2026-11-01T20:10:00Z",
        )));
        let projection = projection();
        let revision = provider
            .observe()
            .expect("observe")
            .provider_revision()
            .clone();
        let first = fixture_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            revision.clone(),
        );
        let second = fixture_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
            revision,
        );
        let barrier = Arc::new(Barrier::new(3));
        let handles = [first, second].into_iter().map(|operation| {
            let provider = Arc::clone(&provider);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                provider.compare_and_apply(&operation).is_ok()
            })
        });
        let handles = handles.collect::<Vec<_>>();
        barrier.wait();
        let successes = handles
            .into_iter()
            .map(|handle| handle.join().expect("join"))
            .filter(|succeeded| *succeeded)
            .count();
        assert_eq!(successes, 1);
    }

    #[test]
    fn provider_squash_merge_records_payload_free_result_and_replays_exactly() {
        let provider = InMemoryExternalActionFixtureProvider::new(time("2026-11-01T20:10:00Z"));
        let projection = projection();
        create_and_mark_ready(&provider, &projection);
        let merge = fixture_successor_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            provider
                .observe()
                .expect("observe merge")
                .provider_revision()
                .clone(),
            42,
        );

        let applied = apply_result(&provider, &merge, "squash merge");
        assert_eq!(
            applied.posture(),
            ExternalActionFixtureProviderApplyPosture::Applied
        );
        assert!(applied.created_successor().is_none());
        let result = applied.exact_merge_result().expect("exact merge result");
        assert_ne!(
            result.merge_sha_commitment(),
            result.merge_result_commitment()
        );
        assert_eq!(provider.operation_count().expect("operation count"), 3);

        let replay = apply_result(&provider, &merge, "exact replay");
        assert_eq!(
            replay.posture(),
            ExternalActionFixtureProviderApplyPosture::ExactReplay
        );
        assert_eq!(replay.provider_revision, applied.provider_revision);
        assert_eq!(
            replay.provider_receipt_commitment(),
            applied.provider_receipt_commitment()
        );
        assert_eq!(replay.exact_merge_result(), applied.exact_merge_result());
        assert_eq!(provider.operation_count().expect("operation count"), 3);
        let lookup = provider.lookup_operation(&merge).expect("lookup");
        assert!(matches!(
            lookup,
            ExternalActionFixtureProviderLookupOutcome::ExactFound { .. }
        ));
        let looked_up = match lookup {
            ExternalActionFixtureProviderLookupOutcome::ExactFound { result, coverage } => {
                coverage.validate_for_operation(&merge).expect("coverage");
                result
            }
            _ => return,
        };
        assert_eq!(
            looked_up.posture(),
            ExternalActionFixtureProviderApplyPosture::ExactReplay
        );
        assert_eq!(looked_up.provider_revision, applied.provider_revision);
        assert_eq!(
            looked_up.provider_receipt_commitment(),
            applied.provider_receipt_commitment()
        );
        assert_eq!(looked_up.exact_merge_result(), applied.exact_merge_result());

        let substituted = fixture_successor_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            SpecContentHash::from_text("irrelevant-replay-revision"),
            43,
        );
        assert_eq!(
            provider
                .compare_and_apply(&substituted)
                .expect_err("successor substitution rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.operation_conflict"
        );
        assert_eq!(
            provider
                .lookup_operation(&substituted)
                .expect_err("lookup substitution rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.operation_conflict"
        );
    }

    #[test]
    fn provider_squash_merge_requires_ready_unchanged_head_and_green_checks() {
        let projection = projection();

        let not_ready = InMemoryExternalActionFixtureProvider::new(time("2026-11-01T20:10:00Z"));
        let create = fixture_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            not_ready
                .observe()
                .expect("observe create")
                .provider_revision()
                .clone(),
        );
        apply_result(&not_ready, &create, "create draft");
        let merge = fixture_successor_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            not_ready
                .observe()
                .expect("observe merge")
                .provider_revision()
                .clone(),
            42,
        );
        assert_eq!(
            not_ready
                .compare_and_apply(&merge)
                .expect_err("draft pull request rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.successor_conflict"
        );

        let changed_head = InMemoryExternalActionFixtureProvider::new(time("2026-11-01T20:10:00Z"));
        create_and_mark_ready(&changed_head, &projection);
        changed_head
            .set_reviewed_head_commitment(SpecContentHash::from_text("changed-head"))
            .expect("change head");
        let merge = fixture_successor_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            changed_head
                .observe()
                .expect("observe changed head")
                .provider_revision()
                .clone(),
            42,
        );
        assert_eq!(
            changed_head
                .compare_and_apply(&merge)
                .expect_err("changed head rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.successor_conflict"
        );

        let failed_checks =
            InMemoryExternalActionFixtureProvider::new(time("2026-11-01T20:10:00Z"));
        create_and_mark_ready(&failed_checks, &projection);
        failed_checks
            .set_required_checks_posture(ExternalActionFixtureRequiredChecksPosture::Failed)
            .expect("checks failed");
        let merge = fixture_successor_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            failed_checks
                .observe()
                .expect("observe failed checks")
                .provider_revision()
                .clone(),
            42,
        );
        assert_eq!(
            failed_checks
                .compare_and_apply(&merge)
                .expect_err("failed checks rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.checks_not_green"
        );
    }

    #[test]
    fn provider_squash_merge_atomically_rejects_revision_drift_without_mutation() {
        let provider = InMemoryExternalActionFixtureProvider::new(time("2026-11-01T20:10:00Z"));
        let projection = projection();
        create_and_mark_ready(&provider, &projection);
        let merge = fixture_successor_operation(
            &projection,
            ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead,
            provider
                .observe()
                .expect("observe merge")
                .provider_revision()
                .clone(),
            42,
        );
        provider
            .set_connector_posture(ExternalActionFixtureConnectorPosture::RefusedBeforeDispatch)
            .expect("advance revision");

        assert_eq!(
            provider
                .compare_and_apply(&merge)
                .expect_err("stale revision rejected")
                .code(),
            "external_action_fixture_lifecycle.provider.revision_drift"
        );
        assert_eq!(provider.operation_count().expect("operation count"), 2);
        assert!(matches!(
            provider.lookup_operation(&merge).expect("lookup"),
            ExternalActionFixtureProviderLookupOutcome::AuthoritativeNotFound { .. }
        ));
    }

    #[test]
    fn successor_store_is_core_owned_exact_and_conflict_rejecting() {
        let projection = projection();
        let binding =
            derive_external_action_successor_binding(&projection, 42, time("2026-11-01T20:10:00Z"))
                .expect("binding");
        let conflicting =
            derive_external_action_successor_binding(&projection, 43, time("2026-11-01T20:10:00Z"))
                .expect("binding");
        let store = InMemoryExternalActionSuccessorBindingStore::default();
        assert_eq!(
            store.put_exact(&projection, &binding).expect("insert"),
            ExternalActionSuccessorBindingWritePosture::Inserted
        );
        assert_eq!(
            store.put_exact(&projection, &binding).expect("replay"),
            ExternalActionSuccessorBindingWritePosture::ExactReplay
        );
        assert_eq!(
            store
                .put_exact(&projection, &conflicting)
                .expect_err("conflict")
                .code(),
            "external_action_fixture_lifecycle.successor.conflict"
        );
        assert_eq!(store.read_exact(&projection).expect("read"), Some(binding));
    }
}
