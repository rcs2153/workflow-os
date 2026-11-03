//! Private Phase B collectors backed by existing kernel read contracts.
//!
//! These collectors are intentionally not registered with a runtime consumer.
//! They prove exact, non-consuming reads and source-specific revision fencing
//! for the first store-backed continuation-context batch.

use serde::Serialize;

use super::{
    collection_commitment, collection_error, collector_expectation_for,
    empty_accounting_claims_for, ContinuationCollectedAccounting, ContinuationCollectedSource,
    ContinuationCollectionAnchor, ContinuationCollectionObservation, ContinuationCollectionRequest,
    ContinuationCollectionResultPosture, ContinuationCollectorKind, ContinuationCollectorRevision,
    ContinuationContextAccess, ContinuationContextCollector,
    ContinuationContextCollectorExpectation, ContinuationContextFreshness,
    ContinuationContextIntegrity, ContinuationContextSensitivity, ContinuationContextSourceId,
    ContinuationUnderstandingCandidate, ContinuationUnderstandingClaimKind, ALL_COLLECTOR_KINDS,
};
use crate::{
    continuation_context::ContinuationContextText, EventLogStore, ImmutableRunBundleDefinitionKind,
    ImmutableRunBundleSensitivity, ImmutableRunBundleStore, RunSnapshotStore,
    SideEffectRecordStore, SpecContentHash, StoredImmutableRunBundle, WorkReportArtifactStore,
    WorkflowOsError, WorkflowRunEvent, WorkflowRunSnapshot, WorkflowRunStatus,
};

const STORE_BACKED_KINDS: [ContinuationCollectorKind; 12] = [
    ContinuationCollectorKind::ValidatedDeclaration,
    ContinuationCollectorKind::ImmutableRunBundle,
    ContinuationCollectorKind::DurableRunState,
    ContinuationCollectorKind::EventHistory,
    ContinuationCollectorKind::Policy,
    ContinuationCollectorKind::Approval,
    ContinuationCollectorKind::Capability,
    ContinuationCollectorKind::RequiredContext,
    ContinuationCollectorKind::Evidence,
    ContinuationCollectorKind::Check,
    ContinuationCollectorKind::SideEffect,
    ContinuationCollectorKind::WorkReport,
];

/// Existing read contracts required by the first store-backed batch.
trait ContinuationStateReadStore:
    EventLogStore
    + RunSnapshotStore
    + crate::ApprovalPresentationRecordStore
    + SideEffectRecordStore
    + WorkReportArtifactStore
{
}

impl<T> ContinuationStateReadStore for T where
    T: EventLogStore
        + RunSnapshotStore
        + crate::ApprovalPresentationRecordStore
        + SideEffectRecordStore
        + WorkReportArtifactStore
{
}

/// Private source set. It borrows existing stores and cannot write through any
/// of the interfaces exposed here.
struct StoreBackedContinuationSourceSet<'a, S, B> {
    state: &'a S,
    bundles: &'a B,
}

impl<'a, S, B> StoreBackedContinuationSourceSet<'a, S, B>
where
    S: ContinuationStateReadStore,
    B: ImmutableRunBundleStore,
{
    const fn new(state: &'a S, bundles: &'a B) -> Self {
        Self { state, bundles }
    }

    fn collector(
        &'a self,
        kind: ContinuationCollectorKind,
    ) -> Result<StoreBackedContinuationCollector<'a, S, B>, WorkflowOsError> {
        if !STORE_BACKED_KINDS.contains(&kind) {
            return Err(store_error(
                "collector.kind_not_store_backed",
                "collector kind is outside the Phase B store-backed batch",
            ));
        }
        Ok(StoreBackedContinuationCollector {
            kind,
            sources: self,
        })
    }

    fn capture_anchor(
        &self,
        purpose: crate::continuation_context::ContinuationContextPurpose,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<ContinuationCollectionAnchor, WorkflowOsError> {
        let mut revisions = Vec::with_capacity(ALL_COLLECTOR_KINDS.len());
        for kind in ALL_COLLECTOR_KINDS {
            let revision = if STORE_BACKED_KINDS.contains(&kind) {
                self.read(kind, purpose, scope)?.revision
            } else {
                fixed_revision(kind, "outside-phase-b")?
            };
            revisions.push(ContinuationCollectorRevision { kind, revision });
        }
        let run_bound = scope.run_id.is_some();
        let immutable_bundle = run_bound
            .then(|| revision_for(&revisions, ContinuationCollectorKind::ImmutableRunBundle))
            .transpose()?;
        let event_cursor = run_bound
            .then(|| revision_for(&revisions, ContinuationCollectorKind::EventHistory))
            .transpose()?;
        ContinuationCollectionAnchor::new(immutable_bundle, event_cursor, revisions)
    }

    fn read(
        &self,
        kind: ContinuationCollectorKind,
        purpose: crate::continuation_context::ContinuationContextPurpose,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let expectation =
            collector_expectation_for(purpose, scope.run_id.is_some(), kind.source_class());
        if expectation == ContinuationContextCollectorExpectation::Inapplicable {
            return StoreBackedRead::without_source(
                kind,
                ContinuationCollectionResultPosture::Inapplicable,
                "inapplicable",
            );
        }
        let read = match kind {
            ContinuationCollectorKind::ValidatedDeclaration => self.read_declarations(scope),
            ContinuationCollectorKind::ImmutableRunBundle => self.read_bundle(scope),
            ContinuationCollectorKind::DurableRunState => self.read_durable_state(scope),
            ContinuationCollectorKind::EventHistory => self.read_event_history(scope),
            ContinuationCollectorKind::Policy => self.read_policies(scope),
            ContinuationCollectorKind::Approval => self.read_approvals(scope),
            ContinuationCollectorKind::Capability
            | ContinuationCollectorKind::RequiredContext
            | ContinuationCollectorKind::Evidence => StoreBackedRead::without_source(
                kind,
                ContinuationCollectionResultPosture::Unsupported,
                "stable-read-contract-unavailable",
            ),
            ContinuationCollectorKind::Check => self.read_checks(scope),
            ContinuationCollectorKind::SideEffect => self.read_side_effects(scope),
            ContinuationCollectorKind::WorkReport => self.read_work_reports(scope),
            _ => Err(store_error(
                "collector.kind_not_store_backed",
                "collector kind is outside the Phase B store-backed batch",
            )),
        };
        match read {
            Ok(mut value) => {
                if value.posture == ContinuationCollectionResultPosture::Empty {
                    value.accounting = empty_accounting_claims_for(
                        purpose,
                        scope.run_id.is_some(),
                        kind.source_class(),
                    )?
                    .into_iter()
                    .map(|claim_kind| ContinuationCollectedAccounting { claim_kind })
                    .collect();
                }
                Ok(value)
            }
            Err(error) => StoreBackedRead::failure(kind, error.code()),
        }
    }

    fn exact_bundle(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<Option<StoredImmutableRunBundle>, WorkflowOsError> {
        let (Some(run_id), Some(workflow_id)) = (&scope.run_id, &scope.workflow_id) else {
            return Ok(None);
        };
        let Some(snapshot) = self.state.load_snapshot(run_id)? else {
            return Ok(None);
        };
        if &snapshot.identity.workflow_id != workflow_id {
            return Err(store_error(
                "scope.workflow_mismatch",
                "stored run snapshot does not match the requested workflow identity",
            ));
        }
        let Some(binding) = snapshot.identity.immutable_run_bundle.as_ref() else {
            return Ok(None);
        };
        let bundle = self
            .bundles
            .read_exact_bundle(run_id, binding.bundle_id())?;
        if bundle.manifest().root_hash() != binding.root_hash()
            || bundle.manifest().workflow_id() != workflow_id
        {
            return Err(store_error(
                "bundle.binding_mismatch",
                "stored immutable bundle does not match the durable run binding",
            ));
        }
        Ok(Some(bundle))
    }

    fn read_declarations(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(bundle) = self.exact_bundle(scope)? else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::ValidatedDeclaration,
                ContinuationCollectionResultPosture::Unsupported,
                "immutable-bundle-unavailable",
            );
        };
        let records = bundle.definition_records();
        if records.is_empty() {
            return StoreBackedRead::empty(
                ContinuationCollectorKind::ValidatedDeclaration,
                records,
            );
        }
        StoreBackedRead::observed(
            ContinuationCollectorKind::ValidatedDeclaration,
            records,
            bundle_sensitivity(bundle.manifest().sensitivity()),
            Vec::new(),
        )
    }

    fn read_bundle(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(bundle) = self.exact_bundle(scope)? else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::ImmutableRunBundle,
                ContinuationCollectionResultPosture::Unsupported,
                "immutable-bundle-unavailable",
            );
        };
        let claims = bundle
            .definition_records()
            .iter()
            .find_map(|record| record.canonical_definition().as_workflow())
            .map(|workflow| {
                ContinuationContextText::new(workflow.display_name.clone()).map(|value| {
                    vec![ContinuationUnderstandingCandidate {
                        kind: ContinuationUnderstandingClaimKind::Objective,
                        value,
                    }]
                })
            })
            .transpose()?
            .unwrap_or_default();
        StoreBackedRead::observed(
            ContinuationCollectorKind::ImmutableRunBundle,
            &(
                bundle.manifest(),
                bundle.definition_records(),
                bundle.local_check_declaration_set_records(),
            ),
            bundle_sensitivity(bundle.manifest().sensitivity()),
            claims,
        )
    }

    fn read_durable_state(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(run_id) = &scope.run_id else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::DurableRunState,
                ContinuationCollectionResultPosture::Unsupported,
                "run-scope-unavailable",
            );
        };
        let Some(snapshot) = self.state.load_snapshot(run_id)? else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::DurableRunState,
                ContinuationCollectionResultPosture::Unavailable,
                "snapshot-unavailable",
            );
        };
        validate_snapshot_scope(&snapshot, scope)?;
        let events = self.state.read_events(run_id)?;
        let current = events.last().is_some_and(|event| {
            event.sequence_number == snapshot.last_sequence_number
                && event.event_id == snapshot.last_event_id
        });
        let posture = if current {
            ContinuationCollectionResultPosture::Observed
        } else {
            ContinuationCollectionResultPosture::Stale
        };
        StoreBackedRead::with_source(
            ContinuationCollectorKind::DurableRunState,
            &snapshot,
            posture,
            if current {
                ContinuationContextFreshness::Current
            } else {
                ContinuationContextFreshness::StaleBlocking
            },
            ContinuationContextSensitivity::Internal,
            run_status_claims(snapshot.status)?,
        )
    }

    fn read_event_history(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(run_id) = &scope.run_id else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::EventHistory,
                ContinuationCollectionResultPosture::Unsupported,
                "run-scope-unavailable",
            );
        };
        let events = self.state.read_events(run_id)?;
        if events.is_empty() {
            return StoreBackedRead::empty(ContinuationCollectorKind::EventHistory, &events);
        }
        validate_event_scope(&events, scope)?;
        StoreBackedRead::observed(
            ContinuationCollectorKind::EventHistory,
            &events,
            ContinuationContextSensitivity::Internal,
            Vec::new(),
        )
    }

    fn read_policies(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(bundle) = self.exact_bundle(scope)? else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::Policy,
                ContinuationCollectionResultPosture::Unsupported,
                "immutable-bundle-unavailable",
            );
        };
        let records = bundle
            .definition_records()
            .iter()
            .filter(|record| record.kind() == ImmutableRunBundleDefinitionKind::Policy)
            .collect::<Vec<_>>();
        if records.is_empty() {
            return StoreBackedRead::empty(ContinuationCollectorKind::Policy, &records);
        }
        StoreBackedRead::observed(
            ContinuationCollectorKind::Policy,
            &records,
            bundle_sensitivity(bundle.manifest().sensitivity()),
            Vec::new(),
        )
    }

    fn read_approvals(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(run_id) = &scope.run_id else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::Approval,
                ContinuationCollectionResultPosture::Unsupported,
                "run-scope-unavailable",
            );
        };
        let Some(snapshot) = self.state.load_snapshot(run_id)? else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::Approval,
                ContinuationCollectionResultPosture::Unavailable,
                "snapshot-unavailable",
            );
        };
        validate_snapshot_scope(&snapshot, scope)?;
        let mut presentations = self.state.list_approval_presentation_records(run_id)?;
        presentations.sort_by(|left, right| {
            left.presentation_id()
                .as_str()
                .cmp(right.presentation_id().as_str())
        });
        let payload = (&snapshot.approval_requests, &presentations);
        if snapshot.approval_requests.is_empty() && presentations.is_empty() {
            return StoreBackedRead::empty(ContinuationCollectorKind::Approval, &payload);
        }
        let claims = if snapshot
            .approval_requests
            .iter()
            .any(|request| request.decision.is_some())
        {
            vec![candidate(
                ContinuationUnderstandingClaimKind::Decision,
                "an approval decision is recorded for the run",
            )?]
        } else {
            Vec::new()
        };
        StoreBackedRead::observed(
            ContinuationCollectorKind::Approval,
            &payload,
            ContinuationContextSensitivity::Restricted,
            claims,
        )
    }

    fn read_checks(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(bundle) = self.exact_bundle(scope)? else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::Check,
                ContinuationCollectionResultPosture::Unsupported,
                "immutable-bundle-unavailable",
            );
        };
        let Some(run_id) = &scope.run_id else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::Check,
                ContinuationCollectionResultPosture::Unsupported,
                "run-scope-unavailable",
            );
        };
        let events = self.state.read_events(run_id)?;
        validate_event_scope(&events, scope)?;
        let result_events = events
            .iter()
            .filter(|event| {
                matches!(
                    event.kind,
                    crate::WorkflowRunEventKind::LocalCheckResultRecorded(_)
                )
            })
            .collect::<Vec<_>>();
        let payload = (bundle.local_check_declaration_set_records(), result_events);
        if payload.0.is_empty() && payload.1.is_empty() {
            return StoreBackedRead::empty(ContinuationCollectorKind::Check, &payload);
        }
        StoreBackedRead::observed(
            ContinuationCollectorKind::Check,
            &payload,
            bundle_sensitivity(bundle.manifest().sensitivity()),
            vec![
                candidate(
                    ContinuationUnderstandingClaimKind::CheckRequirement,
                    "the immutable run binds local check obligations",
                )?,
                candidate(
                    ContinuationUnderstandingClaimKind::ValidationCommitment,
                    "local check posture must be resolved before continuation",
                )?,
            ],
        )
    }

    fn read_side_effects(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let (Some(run_id), Some(workflow_id)) = (&scope.run_id, &scope.workflow_id) else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::SideEffect,
                ContinuationCollectionResultPosture::Unsupported,
                "run-scope-unavailable",
            );
        };
        let mut records = self
            .state
            .list_side_effect_records_for_workflow_run(workflow_id, run_id)?;
        records.sort_by(|left, right| {
            left.side_effect_id()
                .as_str()
                .cmp(right.side_effect_id().as_str())
        });
        if records.is_empty() {
            return StoreBackedRead::empty(ContinuationCollectorKind::SideEffect, &records);
        }
        StoreBackedRead::observed(
            ContinuationCollectorKind::SideEffect,
            &records,
            ContinuationContextSensitivity::Restricted,
            vec![candidate(
                ContinuationUnderstandingClaimKind::SideEffectPosture,
                "side-effect records exist for the run",
            )?],
        )
    }

    fn read_work_reports(
        &self,
        scope: &crate::continuation_context::ContinuationContextScope,
    ) -> Result<StoreBackedRead, WorkflowOsError> {
        let Some(run_id) = &scope.run_id else {
            return StoreBackedRead::without_source(
                ContinuationCollectorKind::WorkReport,
                ContinuationCollectionResultPosture::Unsupported,
                "run-scope-unavailable",
            );
        };
        let mut records = self.state.list_work_report_artifacts(run_id)?;
        records.sort_by(|left, right| left.report_id().as_str().cmp(right.report_id().as_str()));
        if records.is_empty() {
            return StoreBackedRead::empty(ContinuationCollectorKind::WorkReport, &records);
        }
        StoreBackedRead::observed(
            ContinuationCollectorKind::WorkReport,
            &records,
            ContinuationContextSensitivity::Restricted,
            vec![candidate(
                ContinuationUnderstandingClaimKind::ReportingCommitment,
                "a work report artifact is recorded for the run",
            )?],
        )
    }
}

struct StoreBackedContinuationCollector<'a, S, B> {
    kind: ContinuationCollectorKind,
    sources: &'a StoreBackedContinuationSourceSet<'a, S, B>,
}

impl<S, B> ContinuationContextCollector for StoreBackedContinuationCollector<'_, S, B>
where
    S: ContinuationStateReadStore,
    B: ImmutableRunBundleStore,
{
    fn kind(&self) -> ContinuationCollectorKind {
        self.kind
    }

    fn collect(
        &self,
        request: &ContinuationCollectionRequest,
    ) -> Result<ContinuationCollectionObservation, WorkflowOsError> {
        let read = self
            .sources
            .read(self.kind, request.purpose, &request.scope)?;
        if &read.revision != request.anchor.revision_for(self.kind)? {
            return Err(store_error(
                "collector.source_drift",
                "store-backed source revision changed after anchor capture",
            ));
        }
        read.into_observation(self.kind, request)
    }
}

struct StoreBackedRead {
    revision: SpecContentHash,
    posture: ContinuationCollectionResultPosture,
    freshness: Option<ContinuationContextFreshness>,
    sensitivity: Option<ContinuationContextSensitivity>,
    claims: Vec<ContinuationUnderstandingCandidate>,
    accounting: Vec<ContinuationCollectedAccounting>,
}

impl StoreBackedRead {
    fn observed<T: Serialize + ?Sized>(
        kind: ContinuationCollectorKind,
        value: &T,
        sensitivity: ContinuationContextSensitivity,
        claims: Vec<ContinuationUnderstandingCandidate>,
    ) -> Result<Self, WorkflowOsError> {
        Self::with_source(
            kind,
            value,
            ContinuationCollectionResultPosture::Observed,
            ContinuationContextFreshness::Current,
            sensitivity,
            claims,
        )
    }

    fn with_source<T: Serialize + ?Sized>(
        kind: ContinuationCollectorKind,
        value: &T,
        posture: ContinuationCollectionResultPosture,
        freshness: ContinuationContextFreshness,
        sensitivity: ContinuationContextSensitivity,
        claims: Vec<ContinuationUnderstandingCandidate>,
    ) -> Result<Self, WorkflowOsError> {
        Ok(Self {
            revision: source_revision(kind, value)?,
            posture,
            freshness: Some(freshness),
            sensitivity: Some(sensitivity),
            claims,
            accounting: Vec::new(),
        })
    }

    fn empty<T: Serialize + ?Sized>(
        kind: ContinuationCollectorKind,
        value: &T,
    ) -> Result<Self, WorkflowOsError> {
        Ok(Self {
            revision: source_revision(kind, value)?,
            posture: ContinuationCollectionResultPosture::Empty,
            freshness: None,
            sensitivity: None,
            claims: Vec::new(),
            accounting: Vec::new(),
        })
    }

    fn without_source(
        kind: ContinuationCollectorKind,
        posture: ContinuationCollectionResultPosture,
        reason: &str,
    ) -> Result<Self, WorkflowOsError> {
        Ok(Self {
            revision: fixed_revision(kind, reason)?,
            posture,
            freshness: None,
            sensitivity: None,
            claims: Vec::new(),
            accounting: Vec::new(),
        })
    }

    fn failure(kind: ContinuationCollectorKind, code: &str) -> Result<Self, WorkflowOsError> {
        Self::without_source(kind, ContinuationCollectionResultPosture::Failed, code)
    }

    fn into_observation(
        self,
        kind: ContinuationCollectorKind,
        request: &ContinuationCollectionRequest,
    ) -> Result<ContinuationCollectionObservation, WorkflowOsError> {
        let materialized_bytes = serde_json::to_vec(&self.claims)
            .map_err(|_| store_error("claims.serialization_failed", "claim serialization failed"))?
            .len() as u64;
        let mut posture = self.posture;
        let source = match (self.freshness, self.sensitivity) {
            (Some(_freshness), Some(sensitivity)) if sensitivity > request.maximum_sensitivity => {
                posture = ContinuationCollectionResultPosture::Denied;
                None
            }
            (Some(_), Some(_)) if materialized_bytes > request.materialization_budget_bytes => {
                posture = ContinuationCollectionResultPosture::Failed;
                None
            }
            (Some(freshness), Some(sensitivity)) => Some(ContinuationCollectedSource {
                source_id: source_id(kind)?,
                freshness,
                integrity: ContinuationContextIntegrity::Verified,
                access: ContinuationContextAccess::Permitted,
                sensitivity,
                materialized_bytes,
                claims: self.claims,
            }),
            (None, None) => None,
            _ => {
                return Err(store_error(
                    "source.shape_invalid",
                    "store-backed source metadata is incomplete",
                ));
            }
        };
        Ok(ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: self.revision,
            posture,
            source,
            accounting: self.accounting,
        })
    }
}

fn validate_snapshot_scope(
    snapshot: &WorkflowRunSnapshot,
    scope: &crate::continuation_context::ContinuationContextScope,
) -> Result<(), WorkflowOsError> {
    if scope.run_id.as_ref() != Some(&snapshot.identity.run_id)
        || scope.workflow_id.as_ref() != Some(&snapshot.identity.workflow_id)
    {
        return Err(store_error(
            "scope.snapshot_mismatch",
            "stored snapshot does not match requested continuation scope",
        ));
    }
    Ok(())
}

fn validate_event_scope(
    events: &[WorkflowRunEvent],
    scope: &crate::continuation_context::ContinuationContextScope,
) -> Result<(), WorkflowOsError> {
    if events.iter().any(|event| {
        scope.run_id.as_ref() != Some(&event.run_id)
            || scope.workflow_id.as_ref() != Some(&event.workflow_id)
    }) {
        return Err(store_error(
            "scope.event_mismatch",
            "stored event history does not match requested continuation scope",
        ));
    }
    Ok(())
}

fn run_status_claims(
    status: WorkflowRunStatus,
) -> Result<Vec<ContinuationUnderstandingCandidate>, WorkflowOsError> {
    let (kind, value) = match status {
        WorkflowRunStatus::Created | WorkflowRunStatus::Validated => (
            ContinuationUnderstandingClaimKind::RemainingObligation,
            "the run has not started material execution",
        ),
        WorkflowRunStatus::Running | WorkflowRunStatus::Retrying => (
            ContinuationUnderstandingClaimKind::InProgressObligation,
            "the run has material work in progress",
        ),
        WorkflowRunStatus::WaitingForApproval => (
            ContinuationUnderstandingClaimKind::RegisteredWait,
            "the run is waiting for approval",
        ),
        WorkflowRunStatus::WaitingForExternalEvent => (
            ContinuationUnderstandingClaimKind::RegisteredWait,
            "the run is waiting for an external event",
        ),
        WorkflowRunStatus::Escalated => (
            ContinuationUnderstandingClaimKind::Blocker,
            "the run requires operator intervention",
        ),
        WorkflowRunStatus::Completed => (
            ContinuationUnderstandingClaimKind::CompletedObligation,
            "the run completed",
        ),
        WorkflowRunStatus::Failed | WorkflowRunStatus::Canceled => (
            ContinuationUnderstandingClaimKind::Blocker,
            "the run is terminal and cannot continue",
        ),
    };
    Ok(vec![candidate(kind, value)?])
}

fn candidate(
    kind: ContinuationUnderstandingClaimKind,
    value: &str,
) -> Result<ContinuationUnderstandingCandidate, WorkflowOsError> {
    Ok(ContinuationUnderstandingCandidate {
        kind,
        value: ContinuationContextText::new(value)?,
    })
}

const fn bundle_sensitivity(
    sensitivity: ImmutableRunBundleSensitivity,
) -> ContinuationContextSensitivity {
    match sensitivity {
        ImmutableRunBundleSensitivity::Internal => ContinuationContextSensitivity::Internal,
        ImmutableRunBundleSensitivity::Confidential | ImmutableRunBundleSensitivity::Restricted => {
            ContinuationContextSensitivity::Restricted
        }
    }
}

fn source_revision<T: Serialize + ?Sized>(
    kind: ContinuationCollectorKind,
    value: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    collection_commitment("store-backed-source", &(kind, value))
}

fn fixed_revision(
    kind: ContinuationCollectorKind,
    posture: &str,
) -> Result<SpecContentHash, WorkflowOsError> {
    collection_commitment("store-backed-posture", &(kind, posture))
}

fn revision_for(
    revisions: &[ContinuationCollectorRevision],
    kind: ContinuationCollectorKind,
) -> Result<SpecContentHash, WorkflowOsError> {
    revisions
        .iter()
        .find(|entry| entry.kind == kind)
        .map(|entry| entry.revision.clone())
        .ok_or_else(|| store_error("anchor.revision_missing", "source revision is missing"))
}

fn source_id(
    kind: ContinuationCollectorKind,
) -> Result<ContinuationContextSourceId, WorkflowOsError> {
    let value = match kind {
        ContinuationCollectorKind::ValidatedDeclaration => "store/validated-declaration",
        ContinuationCollectorKind::ImmutableRunBundle => "store/immutable-run-bundle",
        ContinuationCollectorKind::DurableRunState => "store/durable-run-state",
        ContinuationCollectorKind::EventHistory => "store/event-history",
        ContinuationCollectorKind::Policy => "store/policy",
        ContinuationCollectorKind::Approval => "store/approval",
        ContinuationCollectorKind::Capability => "store/capability",
        ContinuationCollectorKind::RequiredContext => "store/required-context",
        ContinuationCollectorKind::Evidence => "store/evidence",
        ContinuationCollectorKind::Check => "store/check",
        ContinuationCollectorKind::SideEffect => "store/side-effect",
        ContinuationCollectorKind::WorkReport => "store/work-report",
        _ => {
            return Err(store_error(
                "source_id.kind_invalid",
                "collector kind has no Phase B source identity",
            ));
        }
    };
    ContinuationContextSourceId::new(value)
}

fn store_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    collection_error(&format!("store_backed.{code}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic, clippy::too_many_lines)]

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::{
        build_immutable_run_bundle_with_local_check_declarations, load_project, ActorId,
        CorrelationId, EventId, EventSequenceNumber, ImmutableRunBundleBuildRequest,
        ImmutableRunBundleExecutionPosture, ImmutableRunBundleHandlerPosture,
        ImmutableRunBundleHandlerReference, ImmutableRunBundleId, ImmutableRunBundlePublishOutcome,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleVersion, LocalCheckCommandContract,
        LocalCheckCommandContractInventory, LocalImmutableRunBundleStore, LocalStateBackend,
        ProjectId, SchemaVersion, SkillId, SkillVersion, Timestamp, WorkflowId, WorkflowRun,
        WorkflowRunEventKind, WorkflowRunId, WorkflowVersion, SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new() -> Self {
            let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-continuation-store-backed-{}-{id}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test root");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.0.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("fixture parent");
            }
            fs::write(path, content).expect("fixture write");
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    struct Fixture {
        root: TestRoot,
        state: LocalStateBackend,
        bundles: LocalImmutableRunBundleStore,
        scope: crate::continuation_context::ContinuationContextScope,
        event: WorkflowRunEvent,
    }

    fn fixture() -> Fixture {
        let root = TestRoot::new();
        write_project(&root);
        let loaded = load_project(root.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project = loaded.bundle.expect("project");
        let validation = crate::validate_project_bundle(&project);
        assert!(!validation.has_errors(), "{:?}", validation.diagnostics);
        let workflow_id = WorkflowId::new("continuation/build").expect("workflow");
        let run_id = WorkflowRunId::new("run/store-backed").expect("run");
        let inventory = LocalCheckCommandContractInventory::new(vec![
            LocalCheckCommandContract::docs_check_model_only().expect("contract"),
        ])
        .expect("inventory");
        let bundle = build_immutable_run_bundle_with_local_check_declarations(
            ImmutableRunBundleBuildRequest {
                project: &project,
                workflow_id: &workflow_id,
                bundle_id: ImmutableRunBundleId::new("bundle/store-backed").expect("bundle"),
                bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
                run_id: run_id.clone(),
                resolved_execution_context_hash: SpecContentHash::from_text("context"),
                execution_posture: ImmutableRunBundleExecutionPosture::new(
                    Vec::new(),
                    ImmutableRunBundleReferencePosture::NotSupplied,
                    ImmutableRunBundleReferencePosture::NotSupplied,
                    ImmutableRunBundleReferencePosture::CommittedReference,
                )
                .expect("posture"),
                handlers: vec![ImmutableRunBundleHandlerReference {
                    skill_id: SkillId::new("local/check").expect("skill"),
                    skill_version: SkillVersion::new("v1").expect("skill version"),
                    posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
                }],
                created_at: Timestamp::parse_rfc3339("2026-11-03T12:00:00Z").expect("time"),
                created_by: ActorId::new("system/kernel").expect("actor"),
                sensitivity: ImmutableRunBundleSensitivity::Internal,
                redaction_required: true,
            },
            &inventory,
        )
        .expect("bundle build");
        let state = LocalStateBackend::new(root.path().join("state")).expect("state");
        let bundles = LocalImmutableRunBundleStore::new(root.path().join("bundles"));
        assert_eq!(
            bundles
                .publish_bundle_create_only(&bundle)
                .expect("bundle publish"),
            ImmutableRunBundlePublishOutcome::Published
        );
        let event = created_event(&bundle, &run_id, &workflow_id);
        state.append_event(&event).expect("event append");
        let run = WorkflowRun::rehydrate(std::slice::from_ref(&event)).expect("rehydrate");
        state.save_snapshot(&run.snapshot).expect("snapshot");
        Fixture {
            root,
            state,
            bundles,
            scope: crate::continuation_context::ContinuationContextScope {
                project_id: ProjectId::new("project/continuation").expect("project"),
                workflow_id: Some(workflow_id),
                run_id: Some(run_id),
                phase: None,
            },
            event,
        }
    }

    fn request(
        sources: &StoreBackedContinuationSourceSet<
            '_,
            LocalStateBackend,
            LocalImmutableRunBundleStore,
        >,
        scope: crate::continuation_context::ContinuationContextScope,
    ) -> ContinuationCollectionRequest {
        let purpose = crate::continuation_context::ContinuationContextPurpose::MaterialContinuation;
        let anchor = sources
            .capture_anchor(purpose, &scope)
            .expect("anchor capture");
        ContinuationCollectionRequest::new(
            purpose,
            scope,
            ActorId::new("user/delegated-maintainer").expect("actor"),
            ContinuationContextSensitivity::Restricted,
            32_768,
            anchor,
        )
        .expect("request")
    }

    #[test]
    fn store_backed_batch_observes_supported_sources_and_discloses_missing_contracts() {
        let fixture = fixture();
        let sources = StoreBackedContinuationSourceSet::new(&fixture.state, &fixture.bundles);
        let request = request(&sources, fixture.scope.clone());
        let expected = [
            (
                ContinuationCollectorKind::ValidatedDeclaration,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::ImmutableRunBundle,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::DurableRunState,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::EventHistory,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::Policy,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::Approval,
                ContinuationCollectionResultPosture::Empty,
            ),
            (
                ContinuationCollectorKind::Capability,
                ContinuationCollectionResultPosture::Unsupported,
            ),
            (
                ContinuationCollectorKind::RequiredContext,
                ContinuationCollectionResultPosture::Unsupported,
            ),
            (
                ContinuationCollectorKind::Evidence,
                ContinuationCollectionResultPosture::Unsupported,
            ),
            (
                ContinuationCollectorKind::Check,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::SideEffect,
                ContinuationCollectionResultPosture::Empty,
            ),
            (
                ContinuationCollectorKind::WorkReport,
                ContinuationCollectionResultPosture::Empty,
            ),
        ];
        for (kind, posture) in expected {
            let observation = sources
                .collector(kind)
                .expect("collector")
                .collect(&request)
                .expect("observation");
            assert_eq!(observation.posture, posture, "{kind:?}");
        }
    }

    #[test]
    fn changed_event_cursor_rejects_the_anchored_collector() {
        let fixture = fixture();
        let sources = StoreBackedContinuationSourceSet::new(&fixture.state, &fixture.bundles);
        let request = request(&sources, fixture.scope.clone());
        let validated = next_event(&fixture.event, WorkflowRunEventKind::RunValidated);
        fixture.state.append_event(&validated).expect("append");

        let error = sources
            .collector(ContinuationCollectorKind::EventHistory)
            .expect("collector")
            .collect(&request)
            .expect_err("drift must reject");
        assert_eq!(
            error.code(),
            "continuation_collection.store_backed.collector.source_drift"
        );
    }

    #[test]
    fn changed_snapshot_rejects_the_anchored_durable_state_collector() {
        let fixture = fixture();
        let sources = StoreBackedContinuationSourceSet::new(&fixture.state, &fixture.bundles);
        let request = request(&sources, fixture.scope.clone());
        let validated = next_event(&fixture.event, WorkflowRunEventKind::RunValidated);
        fixture.state.append_event(&validated).expect("append");
        let run = WorkflowRun::rehydrate(&[fixture.event.clone(), validated])
            .expect("rehydrate changed snapshot");
        fixture
            .state
            .save_snapshot(&run.snapshot)
            .expect("save changed snapshot");

        let error = sources
            .collector(ContinuationCollectorKind::DurableRunState)
            .expect("collector")
            .collect(&request)
            .expect_err("snapshot drift must reject");
        assert_eq!(
            error.code(),
            "continuation_collection.store_backed.collector.source_drift"
        );
    }

    #[test]
    fn unchanged_store_revisions_are_stable_after_backend_restart() {
        let fixture = fixture();
        let initial_sources =
            StoreBackedContinuationSourceSet::new(&fixture.state, &fixture.bundles);
        let purpose = crate::continuation_context::ContinuationContextPurpose::MaterialContinuation;
        let initial = initial_sources
            .capture_anchor(purpose, &fixture.scope)
            .expect("initial anchor");

        let reopened_state =
            LocalStateBackend::new(fixture.root.path().join("state")).expect("reopen state");
        let reopened_bundles =
            LocalImmutableRunBundleStore::new(fixture.root.path().join("bundles"));
        let reopened_sources =
            StoreBackedContinuationSourceSet::new(&reopened_state, &reopened_bundles);
        let reopened = reopened_sources
            .capture_anchor(purpose, &fixture.scope)
            .expect("reopened anchor");

        assert_eq!(initial, reopened);
    }

    #[test]
    fn collection_reads_do_not_consume_or_mutate_store_state() {
        let fixture = fixture();
        let sources = StoreBackedContinuationSourceSet::new(&fixture.state, &fixture.bundles);
        let request = request(&sources, fixture.scope.clone());
        let before_events = fixture
            .state
            .read_events(fixture.scope.run_id.as_ref().expect("run"))
            .expect("events");
        for kind in STORE_BACKED_KINDS {
            sources
                .collector(kind)
                .expect("collector")
                .collect(&request)
                .expect("collection");
        }
        let after_events = fixture
            .state
            .read_events(fixture.scope.run_id.as_ref().expect("run"))
            .expect("events");
        assert_eq!(before_events, after_events);
        assert_eq!(
            fixture
                .state
                .list_side_effect_records(fixture.scope.run_id.as_ref().expect("run"))
                .expect("side effects"),
            Vec::new()
        );
        assert_eq!(
            fixture
                .state
                .list_work_report_artifacts(fixture.scope.run_id.as_ref().expect("run"))
                .expect("reports"),
            Vec::new()
        );
    }

    fn write_project(root: &TestRoot) {
        root.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: continuation/project\n  name: Continuation Project\n"
            ),
        );
        root.write(
            "workflows/build.workflow.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: continuation/build\nversion: v1\ndisplay_name: Continue Build\ntriggers:\n  - id: manual-start\n    kind: manual\nsteps:\n  - id: inspect\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    local_check_requirements:\n      - id: docs-required\n        command_id: local-check/docs\n        requirement_level: required\n        minimum_assurance: kernel_observed_local_process\n        accepted_statuses: [passed]\n        freshness:\n          mode: no_reuse\n        exact_immutable_run_binding_required: true\n        truncation_allowed: false\n        network_maximum: disabled\n        side_effect_maximum: no_source_writes\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        root.write(
            "skills/check.skill.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Local Check\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: failed\n    description: Failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        root.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
            ),
        );
    }

    fn created_event(
        bundle: &crate::ImmutableRunBundleBuildResult,
        run_id: &WorkflowRunId,
        workflow_id: &WorkflowId,
    ) -> WorkflowRunEvent {
        WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(1).expect("sequence"),
            event_id: EventId::new("event/created").expect("event"),
            timestamp: Timestamp::parse_rfc3339("2026-11-03T12:00:01Z").expect("time"),
            run_id: run_id.clone(),
            workflow_id: workflow_id.clone(),
            schema_version: SchemaVersion::new(SUPPORTED_SCHEMA_VERSION).expect("schema"),
            workflow_version: WorkflowVersion::new("v1").expect("version"),
            spec_content_hash: bundle.manifest().workflow_content_hash().clone(),
            correlation_id: Some(
                CorrelationId::new("correlation/store-backed").expect("correlation"),
            ),
            actor: Some(ActorId::new("system/kernel").expect("actor")),
            idempotency_key: None,
            kind: WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: Some(bundle.manifest().run_binding()),
            },
        }
    }

    fn next_event(previous: &WorkflowRunEvent, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
        WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(previous.sequence_number.get() + 1)
                .expect("sequence"),
            event_id: EventId::new("event/validated").expect("event"),
            timestamp: Timestamp::parse_rfc3339("2026-11-03T12:00:02Z").expect("time"),
            run_id: previous.run_id.clone(),
            workflow_id: previous.workflow_id.clone(),
            schema_version: previous.schema_version.clone(),
            workflow_version: previous.workflow_version.clone(),
            spec_content_hash: previous.spec_content_hash.clone(),
            correlation_id: previous.correlation_id.clone(),
            actor: previous.actor.clone(),
            idempotency_key: None,
            kind,
        }
    }
}
