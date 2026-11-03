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
    observed_at: Timestamp,
    observation_commitment: SpecContentHash,
}

impl ExternalActionFixtureProviderObservation {
    pub(crate) fn new(
        provider_revision: SpecContentHash,
        repository_commitment: SpecContentHash,
        base_target_commitment: SpecContentHash,
        reviewed_head_commitment: SpecContentHash,
        required_checks_commitment: SpecContentHash,
        pull_request_state_commitment: SpecContentHash,
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
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalActionFixtureProviderApplyPosture {
    Applied,
    ExactReplay,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ExternalActionFixtureProviderApplyResult {
    posture: ExternalActionFixtureProviderApplyPosture,
    provider_revision: SpecContentHash,
    provider_receipt_commitment: SpecContentHash,
    created_successor: Option<ExternalActionFixtureCreatedSuccessor>,
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

impl ExternalActionFixtureProviderApplyResult {
    pub(crate) const fn posture(&self) -> ExternalActionFixtureProviderApplyPosture {
        self.posture
    }

    pub(crate) const fn provider_receipt_commitment(&self) -> &SpecContentHash {
        &self.provider_receipt_commitment
    }

    pub(crate) const fn created_successor(&self) -> Option<&ExternalActionFixtureCreatedSuccessor> {
        self.created_successor.as_ref()
    }
}

impl fmt::Debug for ExternalActionFixtureProviderApplyResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionFixtureProviderApplyResult")
            .field("posture", &self.posture)
            .field("provider_revision", &"[REDACTED]")
            .field("provider_receipt_commitment", &"[REDACTED]")
            .field("created_successor", &self.created_successor.is_some())
            .finish()
    }
}

pub(crate) trait ExternalActionFixtureProviderSource {
    fn observe(&self) -> Result<ExternalActionFixtureProviderObservation, WorkflowOsError>;

    fn compare_and_apply(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<ExternalActionFixtureProviderApplyResult, WorkflowOsError>;

    fn lookup_operation(
        &self,
        operation_id: &ExternalActionTransitionOperationId,
    ) -> Result<Option<ExternalActionFixtureProviderApplyResult>, WorkflowOsError>;
}

struct FixtureProviderState {
    revision_sequence: u64,
    revision: SpecContentHash,
    next_pull_request_number: u64,
    receipts: BTreeMap<ExternalActionTransitionOperationId, FixtureProviderReceipt>,
}

struct FixtureProviderReceipt {
    transition_kind: ExternalActionTransitionKind,
    receipt_commitment: SpecContentHash,
    resulting_revision: SpecContentHash,
    created_successor: Option<ExternalActionFixtureCreatedSuccessor>,
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
                receipts: BTreeMap::new(),
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
}

impl ExternalActionFixtureProviderSource for InMemoryExternalActionFixtureProvider {
    fn observe(&self) -> Result<ExternalActionFixtureProviderObservation, WorkflowOsError> {
        let state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        ExternalActionFixtureProviderObservation::new(
            state.revision.clone(),
            SpecContentHash::from_text("repository"),
            SpecContentHash::from_text("base"),
            SpecContentHash::from_text("head"),
            SpecContentHash::from_text("checks"),
            SpecContentHash::from_text("pull-request-state"),
            self.observed_at,
        )
    }

    fn compare_and_apply(
        &self,
        operation: &ExternalActionFixtureProviderOperation,
    ) -> Result<ExternalActionFixtureProviderApplyResult, WorkflowOsError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        if let Some(receipt) = state.receipts.get(&operation.operation_id) {
            if receipt.transition_kind != operation.transition_kind {
                return Err(fixture_error(
                    "provider.operation_conflict",
                    "fixture provider operation identity conflicts with prior content",
                ));
            }
            return Ok(ExternalActionFixtureProviderApplyResult {
                posture: ExternalActionFixtureProviderApplyPosture::ExactReplay,
                provider_revision: receipt.resulting_revision.clone(),
                provider_receipt_commitment: receipt.receipt_commitment.clone(),
                created_successor: receipt.created_successor.clone(),
            });
        }
        if state.revision != operation.expected_provider_revision {
            return Err(fixture_error(
                "provider.revision_drift",
                "fixture provider revision changed before operation",
            ));
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
        let receipt = commitment(
            PROVIDER_RECEIPT_DOMAIN,
            &(
                &operation.operation_id,
                operation.transition_kind,
                &prior_revision,
                &resulting_revision,
                &created_successor,
            ),
        )?;
        state.revision.clone_from(&resulting_revision);
        state.receipts.insert(
            operation.operation_id.clone(),
            FixtureProviderReceipt {
                transition_kind: operation.transition_kind,
                receipt_commitment: receipt.clone(),
                resulting_revision: resulting_revision.clone(),
                created_successor: created_successor.clone(),
            },
        );
        Ok(ExternalActionFixtureProviderApplyResult {
            posture: ExternalActionFixtureProviderApplyPosture::Applied,
            provider_revision: resulting_revision,
            provider_receipt_commitment: receipt,
            created_successor,
        })
    }

    fn lookup_operation(
        &self,
        operation_id: &ExternalActionTransitionOperationId,
    ) -> Result<Option<ExternalActionFixtureProviderApplyResult>, WorkflowOsError> {
        let state = self
            .state
            .lock()
            .map_err(|_| fixture_error("provider.unavailable", "fixture provider unavailable"))?;
        Ok(state.receipts.get(operation_id).map(|receipt| {
            ExternalActionFixtureProviderApplyResult {
                posture: ExternalActionFixtureProviderApplyPosture::ExactReplay,
                provider_revision: receipt.resulting_revision.clone(),
                provider_receipt_commitment: receipt.receipt_commitment.clone(),
                created_successor: receipt.created_successor.clone(),
            }
        }))
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
        let applied = provider.compare_and_apply(&operation).expect("apply");
        assert_eq!(
            applied.posture,
            ExternalActionFixtureProviderApplyPosture::Applied
        );

        let replay = provider.compare_and_apply(&operation).expect("replay");
        assert_eq!(
            replay.posture,
            ExternalActionFixtureProviderApplyPosture::ExactReplay
        );
        assert_eq!(
            replay.provider_receipt_commitment,
            applied.provider_receipt_commitment
        );
        assert_eq!(replay.provider_revision, applied.provider_revision);

        let current = provider.observe().expect("observe");
        let advance = fixture_operation(
            &projection_value,
            ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
            current.provider_revision().clone(),
        );
        provider.compare_and_apply(&advance).expect("advance");
        let replay_after_advance = provider.compare_and_apply(&operation).expect("replay");
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
