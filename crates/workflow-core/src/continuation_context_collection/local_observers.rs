//! Private Phase C bounded local-observer collectors.
//!
//! Core accepts observations only from an explicitly injected observer. The
//! observer may inspect local Git/worktree state and three exact operating
//! documents, but Core validates the closed shapes, path-safety attestations,
//! size limits, sensitivity, request binding, and source revisions. No runtime
//! consumer is registered by this module.

use std::{collections::BTreeSet, fmt};

use serde::Serialize;

use super::{
    collection_commitment, collection_error, empty_accounting_claims_for,
    ContinuationCollectedAccounting, ContinuationCollectedSource, ContinuationCollectionAnchor,
    ContinuationCollectionObservation, ContinuationCollectionRequest,
    ContinuationCollectionResultPosture, ContinuationCollectorKind, ContinuationCollectorRevision,
    ContinuationContextAccess, ContinuationContextCollector, ContinuationContextFreshness,
    ContinuationContextIntegrity, ContinuationContextSensitivity, ContinuationContextSourceId,
    ALL_COLLECTOR_KINDS,
};
use crate::{ActorId, SpecContentHash, WorkflowOsError};

const MAX_DOCUMENT_BYTES: u64 = 256 * 1024;
const MAX_DOCUMENT_TOTAL_BYTES: u64 = 512 * 1024;
const MAX_CHANGE_COUNT: u32 = 100_000;

const LOCAL_OBSERVER_KINDS: [ContinuationCollectorKind; 3] = [
    ContinuationCollectorKind::TypedHandoff,
    ContinuationCollectorKind::GitObservation,
    ContinuationCollectorKind::OperatingDocument,
];

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct LocalObservationRequest {
    purpose: crate::continuation_context::ContinuationContextPurpose,
    scope: crate::continuation_context::ContinuationContextScope,
    actor: ActorId,
    observer_contract: SpecContentHash,
    commitment: SpecContentHash,
}

impl fmt::Debug for LocalObservationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalObservationRequest")
            .field("purpose", &self.purpose)
            .field("scope", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("observer_contract", &"[REDACTED]")
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

impl LocalObservationRequest {
    fn new(
        purpose: crate::continuation_context::ContinuationContextPurpose,
        scope: crate::continuation_context::ContinuationContextScope,
        actor: ActorId,
        observer_contract: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        let commitment = collection_commitment(
            "local-observer-request",
            &(purpose, &scope, &actor, &observer_contract),
        )?;
        Ok(Self {
            purpose,
            scope,
            actor,
            observer_contract,
            commitment,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct GitWorktreeObservation {
    request_commitment: SpecContentHash,
    repository_present: bool,
    head_commitment: Option<SpecContentHash>,
    worktree_commitment: SpecContentHash,
    tracked_change_count: u32,
    untracked_change_count: u32,
    conflict_count: u32,
}

impl fmt::Debug for GitWorktreeObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitWorktreeObservation")
            .field("request_commitment", &"[REDACTED]")
            .field("repository_present", &self.repository_present)
            .field(
                "head_commitment",
                &self.head_commitment.as_ref().map(|_| "[REDACTED]"),
            )
            .field("worktree_commitment", &"[REDACTED]")
            .field("tracked_change_count", &self.tracked_change_count)
            .field("untracked_change_count", &self.untracked_change_count)
            .field("conflict_count", &self.conflict_count)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum OperatingDocumentKind {
    AgentInstructions,
    EngineeringStandard,
    Roadmap,
}

impl OperatingDocumentKind {
    const fn canonical_relative_path(self) -> &'static str {
        match self {
            Self::AgentInstructions => "AGENTS.md",
            Self::EngineeringStandard => "docs/ENGINEERING_STANDARD.md",
            Self::Roadmap => "ROADMAP.md",
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct OperatingDocumentObservation {
    kind: OperatingDocumentKind,
    content_commitment: SpecContentHash,
    byte_len: u64,
    canonical_beneath_project_root: bool,
    symlink_free: bool,
    sensitivity: ContinuationContextSensitivity,
}

impl fmt::Debug for OperatingDocumentObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperatingDocumentObservation")
            .field("kind", &self.kind)
            .field("content_commitment", &"[REDACTED]")
            .field("byte_len", &self.byte_len)
            .field(
                "canonical_beneath_project_root",
                &self.canonical_beneath_project_root,
            )
            .field("symlink_free", &self.symlink_free)
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct OperatingDocumentObservationSet {
    request_commitment: SpecContentHash,
    documents: Vec<OperatingDocumentObservation>,
}

impl fmt::Debug for OperatingDocumentObservationSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperatingDocumentObservationSet")
            .field("request_commitment", &"[REDACTED]")
            .field("documents", &self.documents)
            .finish()
    }
}

trait ContinuationLocalObserver {
    fn contract_commitment(&self) -> SpecContentHash;

    fn observe_git_worktree(
        &self,
        request: &LocalObservationRequest,
    ) -> Result<GitWorktreeObservation, WorkflowOsError>;

    fn observe_operating_documents(
        &self,
        request: &LocalObservationRequest,
    ) -> Result<OperatingDocumentObservationSet, WorkflowOsError>;
}

struct BoundedLocalContinuationSourceSet<'a, O> {
    observer: &'a O,
}

impl<'a, O> BoundedLocalContinuationSourceSet<'a, O>
where
    O: ContinuationLocalObserver,
{
    const fn new(observer: &'a O) -> Self {
        Self { observer }
    }

    fn collector(
        &'a self,
        kind: ContinuationCollectorKind,
    ) -> Result<BoundedLocalContinuationCollector<'a, O>, WorkflowOsError> {
        if !LOCAL_OBSERVER_KINDS.contains(&kind) {
            return Err(local_error(
                "collector.kind_not_local_observer",
                "collector kind is outside the Phase C local-observer batch",
            ));
        }
        Ok(BoundedLocalContinuationCollector {
            kind,
            sources: self,
        })
    }

    fn capture_anchor(
        &self,
        base: &ContinuationCollectionAnchor,
        purpose: crate::continuation_context::ContinuationContextPurpose,
        scope: crate::continuation_context::ContinuationContextScope,
        actor: ActorId,
    ) -> Result<ContinuationCollectionAnchor, WorkflowOsError> {
        let local_request = LocalObservationRequest::new(
            purpose,
            scope,
            actor,
            self.observer.contract_commitment(),
        )?;
        let git = self.read_git(&local_request)?;
        let documents = self.read_documents(&local_request)?;
        let handoff_revision = fixed_revision(
            ContinuationCollectorKind::TypedHandoff,
            "stable-handoff-store-unavailable",
        )?;
        let mut revisions = Vec::with_capacity(ALL_COLLECTOR_KINDS.len());
        for kind in ALL_COLLECTOR_KINDS {
            let revision = match kind {
                ContinuationCollectorKind::GitObservation => git.revision.clone(),
                ContinuationCollectorKind::OperatingDocument => documents.revision.clone(),
                ContinuationCollectorKind::TypedHandoff => handoff_revision.clone(),
                _ => base.revision_for(kind)?.clone(),
            };
            revisions.push(ContinuationCollectorRevision { kind, revision });
        }
        ContinuationCollectionAnchor::new(
            base.immutable_bundle.clone(),
            base.event_cursor.clone(),
            revisions,
        )
    }

    fn read(
        &self,
        kind: ContinuationCollectorKind,
        request: &LocalObservationRequest,
    ) -> Result<LocalRead, WorkflowOsError> {
        match kind {
            ContinuationCollectorKind::GitObservation => self.read_git(request),
            ContinuationCollectorKind::OperatingDocument => self.read_documents(request),
            ContinuationCollectorKind::TypedHandoff => LocalRead::without_source(
                kind,
                ContinuationCollectionResultPosture::Unsupported,
                "stable-handoff-store-unavailable",
            ),
            _ => Err(local_error(
                "collector.kind_not_local_observer",
                "collector kind is outside the Phase C local-observer batch",
            )),
        }
    }

    fn read_git(&self, request: &LocalObservationRequest) -> Result<LocalRead, WorkflowOsError> {
        let observation = self.observer.observe_git_worktree(request)?;
        validate_git_observation(&observation, request)?;
        if !observation.repository_present {
            return LocalRead::empty(ContinuationCollectorKind::GitObservation, &observation);
        }
        LocalRead::observed(
            ContinuationCollectorKind::GitObservation,
            &observation,
            observation_size(&observation)?,
        )
    }

    fn read_documents(
        &self,
        request: &LocalObservationRequest,
    ) -> Result<LocalRead, WorkflowOsError> {
        let mut observation = self.observer.observe_operating_documents(request)?;
        validate_document_observations(&mut observation, request)?;
        if observation.documents.is_empty() {
            return LocalRead::empty(ContinuationCollectorKind::OperatingDocument, &observation);
        }
        let materialized_bytes = observation
            .documents
            .iter()
            .map(|document| document.byte_len)
            .sum();
        LocalRead::observed(
            ContinuationCollectorKind::OperatingDocument,
            &observation,
            materialized_bytes,
        )
    }
}

struct BoundedLocalContinuationCollector<'a, O> {
    kind: ContinuationCollectorKind,
    sources: &'a BoundedLocalContinuationSourceSet<'a, O>,
}

impl<O> ContinuationContextCollector for BoundedLocalContinuationCollector<'_, O>
where
    O: ContinuationLocalObserver,
{
    fn kind(&self) -> ContinuationCollectorKind {
        self.kind
    }

    fn collect(
        &self,
        request: &ContinuationCollectionRequest,
    ) -> Result<ContinuationCollectionObservation, WorkflowOsError> {
        let local_request = LocalObservationRequest::new(
            request.purpose,
            request.scope.clone(),
            request.actor.clone(),
            self.sources.observer.contract_commitment(),
        )?;
        let read = self.sources.read(self.kind, &local_request)?;
        if &read.revision != request.anchor.revision_for(self.kind)? {
            return Err(local_error(
                "collector.source_drift",
                "local observer source revision changed after anchor capture",
            ));
        }
        read.into_observation(self.kind, request)
    }
}

struct LocalRead {
    revision: SpecContentHash,
    posture: ContinuationCollectionResultPosture,
    materialized_bytes: u64,
}

impl LocalRead {
    fn observed<T: Serialize + ?Sized>(
        kind: ContinuationCollectorKind,
        value: &T,
        materialized_bytes: u64,
    ) -> Result<Self, WorkflowOsError> {
        Ok(Self {
            revision: source_revision(kind, value)?,
            posture: ContinuationCollectionResultPosture::Observed,
            materialized_bytes,
        })
    }

    fn empty<T: Serialize + ?Sized>(
        kind: ContinuationCollectorKind,
        value: &T,
    ) -> Result<Self, WorkflowOsError> {
        Ok(Self {
            revision: source_revision(kind, value)?,
            posture: ContinuationCollectionResultPosture::Empty,
            materialized_bytes: 0,
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
            materialized_bytes: 0,
        })
    }

    fn into_observation(
        self,
        kind: ContinuationCollectorKind,
        request: &ContinuationCollectionRequest,
    ) -> Result<ContinuationCollectionObservation, WorkflowOsError> {
        let posture = if self.posture == ContinuationCollectionResultPosture::Observed
            && self.materialized_bytes > request.materialization_budget_bytes
        {
            ContinuationCollectionResultPosture::Failed
        } else {
            self.posture
        };
        let source = if posture == ContinuationCollectionResultPosture::Observed {
            Some(ContinuationCollectedSource {
                source_id: source_id(kind)?,
                freshness: ContinuationContextFreshness::Current,
                integrity: ContinuationContextIntegrity::Verified,
                access: ContinuationContextAccess::Permitted,
                sensitivity: ContinuationContextSensitivity::Internal,
                materialized_bytes: self.materialized_bytes,
                claims: Vec::new(),
            })
        } else {
            None
        };
        let accounting = if posture == ContinuationCollectionResultPosture::Empty {
            empty_accounting_claims_for(
                request.purpose,
                request.scope.run_id.is_some(),
                kind.source_class(),
            )?
            .into_iter()
            .map(|claim_kind| ContinuationCollectedAccounting { claim_kind })
            .collect()
        } else {
            Vec::new()
        };
        Ok(ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: self.revision,
            posture,
            source,
            accepted_intents: Vec::new(),
            accounting,
        })
    }
}

fn validate_git_observation(
    observation: &GitWorktreeObservation,
    request: &LocalObservationRequest,
) -> Result<(), WorkflowOsError> {
    if observation.request_commitment != request.commitment {
        return Err(local_error(
            "git.request_mismatch",
            "Git observation is bound to another local-observer request",
        ));
    }
    if observation.tracked_change_count > MAX_CHANGE_COUNT
        || observation.untracked_change_count > MAX_CHANGE_COUNT
        || observation.conflict_count > MAX_CHANGE_COUNT
    {
        return Err(local_error(
            "git.change_count_limit",
            "Git observation exceeds the bounded change-count limit",
        ));
    }
    if !observation.repository_present
        && (observation.head_commitment.is_some()
            || observation.tracked_change_count != 0
            || observation.untracked_change_count != 0
            || observation.conflict_count != 0)
    {
        return Err(local_error(
            "git.absent_shape_invalid",
            "absent Git repository posture cannot carry repository state",
        ));
    }
    if observation.conflict_count > observation.tracked_change_count {
        return Err(local_error(
            "git.conflict_count_invalid",
            "Git conflict count cannot exceed tracked changes",
        ));
    }
    Ok(())
}

fn validate_document_observations(
    observation: &mut OperatingDocumentObservationSet,
    request: &LocalObservationRequest,
) -> Result<(), WorkflowOsError> {
    if observation.request_commitment != request.commitment {
        return Err(local_error(
            "documents.request_mismatch",
            "operating-document observation is bound to another local-observer request",
        ));
    }
    observation.documents.sort_by_key(|document| document.kind);
    let mut kinds = BTreeSet::new();
    let mut total = 0_u64;
    for document in &observation.documents {
        if !kinds.insert(document.kind) {
            return Err(local_error(
                "documents.duplicate",
                "operating-document observation contains a duplicate allowlisted document",
            ));
        }
        if !document.canonical_beneath_project_root || !document.symlink_free {
            return Err(local_error(
                "documents.path_unsafe",
                "operating-document observation failed canonical path or symlink enforcement",
            ));
        }
        if document.byte_len > MAX_DOCUMENT_BYTES {
            return Err(local_error(
                "documents.file_size_limit",
                "operating document exceeds the per-file size limit",
            ));
        }
        if document.sensitivity > ContinuationContextSensitivity::Internal {
            return Err(local_error(
                "documents.sensitivity_unsupported",
                "operating document exceeds the Phase C sensitivity ceiling",
            ));
        }
        total = total.checked_add(document.byte_len).ok_or_else(|| {
            local_error(
                "documents.total_size_overflow",
                "operating-document total size overflowed",
            )
        })?;
    }
    if total > MAX_DOCUMENT_TOTAL_BYTES {
        return Err(local_error(
            "documents.total_size_limit",
            "operating documents exceed the aggregate size limit",
        ));
    }
    Ok(())
}

fn observation_size<T: Serialize>(value: &T) -> Result<u64, WorkflowOsError> {
    serde_json::to_vec(value)
        .map(|serialized| serialized.len() as u64)
        .map_err(|_| local_error("observation.serialization_failed", "observation failed"))
}

fn source_revision<T: Serialize + ?Sized>(
    kind: ContinuationCollectorKind,
    value: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    collection_commitment("local-observer-source", &(kind, value))
}

fn fixed_revision(
    kind: ContinuationCollectorKind,
    posture: &str,
) -> Result<SpecContentHash, WorkflowOsError> {
    collection_commitment("local-observer-posture", &(kind, posture))
}

fn source_id(
    kind: ContinuationCollectorKind,
) -> Result<ContinuationContextSourceId, WorkflowOsError> {
    let value = match kind {
        ContinuationCollectorKind::GitObservation => "observer/git-worktree",
        ContinuationCollectorKind::OperatingDocument => "observer/operating-documents",
        _ => {
            return Err(local_error(
                "source_id.kind_invalid",
                "collector kind has no Phase C source identity",
            ));
        }
    };
    ContinuationContextSourceId::new(value)
}

fn local_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    collection_error(&format!("local_observer.{code}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use std::cell::RefCell;

    use super::*;
    use crate::{ProjectId, WorkflowId, WorkflowRunId};

    struct FixtureObserver {
        contract_commitment: RefCell<SpecContentHash>,
        git: RefCell<GitWorktreeObservation>,
        documents: RefCell<OperatingDocumentObservationSet>,
    }

    impl ContinuationLocalObserver for FixtureObserver {
        fn contract_commitment(&self) -> SpecContentHash {
            self.contract_commitment.borrow().clone()
        }

        fn observe_git_worktree(
            &self,
            _request: &LocalObservationRequest,
        ) -> Result<GitWorktreeObservation, WorkflowOsError> {
            Ok(self.git.borrow().clone())
        }

        fn observe_operating_documents(
            &self,
            _request: &LocalObservationRequest,
        ) -> Result<OperatingDocumentObservationSet, WorkflowOsError> {
            Ok(self.documents.borrow().clone())
        }
    }

    fn scope() -> crate::continuation_context::ContinuationContextScope {
        crate::continuation_context::ContinuationContextScope {
            project_id: ProjectId::new("project/local-observer").expect("project"),
            workflow_id: Some(WorkflowId::new("workflow/continue").expect("workflow")),
            run_id: Some(WorkflowRunId::new("run/local-observer").expect("run")),
            phase: None,
        }
    }

    fn actor() -> ActorId {
        ActorId::new("user/delegated-maintainer").expect("actor")
    }

    fn fixture_contract_commitment() -> SpecContentHash {
        SpecContentHash::from_text("fixture-local-observer-contract-v1")
    }

    fn local_request() -> LocalObservationRequest {
        LocalObservationRequest::new(
            crate::continuation_context::ContinuationContextPurpose::MaterialContinuation,
            scope(),
            actor(),
            fixture_contract_commitment(),
        )
        .expect("local request")
    }

    fn observer() -> FixtureObserver {
        let request = local_request();
        FixtureObserver {
            contract_commitment: RefCell::new(fixture_contract_commitment()),
            git: RefCell::new(GitWorktreeObservation {
                request_commitment: request.commitment.clone(),
                repository_present: true,
                head_commitment: Some(SpecContentHash::from_text("head")),
                worktree_commitment: SpecContentHash::from_text("worktree"),
                tracked_change_count: 2,
                untracked_change_count: 1,
                conflict_count: 0,
            }),
            documents: RefCell::new(OperatingDocumentObservationSet {
                request_commitment: request.commitment,
                documents: vec![
                    operating_document(OperatingDocumentKind::Roadmap, 1_024),
                    operating_document(OperatingDocumentKind::AgentInstructions, 512),
                ],
            }),
        }
    }

    fn operating_document(
        kind: OperatingDocumentKind,
        byte_len: u64,
    ) -> OperatingDocumentObservation {
        OperatingDocumentObservation {
            kind,
            content_commitment: SpecContentHash::from_text(kind.canonical_relative_path()),
            byte_len,
            canonical_beneath_project_root: true,
            symlink_free: true,
            sensitivity: ContinuationContextSensitivity::Internal,
        }
    }

    fn base_anchor() -> ContinuationCollectionAnchor {
        let revisions = ALL_COLLECTOR_KINDS
            .into_iter()
            .map(|kind| ContinuationCollectorRevision {
                kind,
                revision: SpecContentHash::from_text(&format!("base-{kind:?}")),
            })
            .collect();
        ContinuationCollectionAnchor::new(
            Some(SpecContentHash::from_text("bundle")),
            Some(SpecContentHash::from_text("cursor")),
            revisions,
        )
        .expect("base anchor")
    }

    fn request(
        sources: &BoundedLocalContinuationSourceSet<'_, FixtureObserver>,
    ) -> ContinuationCollectionRequest {
        let purpose = crate::continuation_context::ContinuationContextPurpose::MaterialContinuation;
        let anchor = sources
            .capture_anchor(&base_anchor(), purpose, scope(), actor())
            .expect("local anchor");
        ContinuationCollectionRequest::new(
            purpose,
            scope(),
            actor(),
            ContinuationContextSensitivity::Internal,
            MAX_DOCUMENT_TOTAL_BYTES,
            anchor,
        )
        .expect("request")
    }

    #[test]
    fn bounded_observers_produce_typed_postures_without_handoff_fabrication() {
        let observer = observer();
        let sources = BoundedLocalContinuationSourceSet::new(&observer);
        let request = request(&sources);
        let expected = [
            (
                ContinuationCollectorKind::GitObservation,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::OperatingDocument,
                ContinuationCollectionResultPosture::Observed,
            ),
            (
                ContinuationCollectorKind::TypedHandoff,
                ContinuationCollectionResultPosture::Unsupported,
            ),
        ];
        for (kind, posture) in expected {
            let observation = sources
                .collector(kind)
                .expect("collector")
                .collect(&request)
                .expect("observation");
            assert_eq!(observation.posture, posture);
        }
    }

    #[test]
    fn changed_git_revision_rejects_the_anchored_observation() {
        let observer = observer();
        let sources = BoundedLocalContinuationSourceSet::new(&observer);
        let request = request(&sources);
        observer.git.borrow_mut().worktree_commitment = SpecContentHash::from_text("changed");

        let error = sources
            .collector(ContinuationCollectorKind::GitObservation)
            .expect("collector")
            .collect(&request)
            .expect_err("drift must reject");
        assert_eq!(
            error.code(),
            "continuation_collection.local_observer.collector.source_drift"
        );
    }

    #[test]
    fn changed_observer_contract_rejects_the_anchored_observation() {
        let observer = observer();
        let sources = BoundedLocalContinuationSourceSet::new(&observer);
        let request = request(&sources);
        *observer.contract_commitment.borrow_mut() =
            SpecContentHash::from_text("substituted-local-observer-contract");

        let error = sources
            .collector(ContinuationCollectorKind::GitObservation)
            .expect("collector")
            .collect(&request)
            .expect_err("observer substitution must reject");
        assert_eq!(
            error.code(),
            "continuation_collection.local_observer.git.request_mismatch"
        );
    }

    #[test]
    fn unsafe_or_duplicate_operating_documents_fail_closed() {
        let observer = observer();
        observer.documents.borrow_mut().documents = vec![
            operating_document(OperatingDocumentKind::Roadmap, 1),
            operating_document(OperatingDocumentKind::Roadmap, 1),
        ];
        let sources = BoundedLocalContinuationSourceSet::new(&observer);
        let error = sources
            .capture_anchor(
                &base_anchor(),
                crate::continuation_context::ContinuationContextPurpose::MaterialContinuation,
                scope(),
                actor(),
            )
            .expect_err("duplicate documents must reject");
        assert_eq!(
            error.code(),
            "continuation_collection.local_observer.documents.duplicate"
        );

        observer.documents.borrow_mut().documents = vec![OperatingDocumentObservation {
            symlink_free: false,
            ..operating_document(OperatingDocumentKind::Roadmap, 1)
        }];
        let error = sources
            .capture_anchor(
                &base_anchor(),
                crate::continuation_context::ContinuationContextPurpose::MaterialContinuation,
                scope(),
                actor(),
            )
            .expect_err("unsafe path must reject");
        assert_eq!(
            error.code(),
            "continuation_collection.local_observer.documents.path_unsafe"
        );
    }

    #[test]
    fn document_order_does_not_change_the_source_revision() {
        let observer = observer();
        let sources = BoundedLocalContinuationSourceSet::new(&observer);
        let first = sources
            .capture_anchor(
                &base_anchor(),
                crate::continuation_context::ContinuationContextPurpose::MaterialContinuation,
                scope(),
                actor(),
            )
            .expect("first anchor");
        observer.documents.borrow_mut().documents.reverse();
        let second = sources
            .capture_anchor(
                &base_anchor(),
                crate::continuation_context::ContinuationContextPurpose::MaterialContinuation,
                scope(),
                actor(),
            )
            .expect("second anchor");
        assert_eq!(
            first
                .revision_for(ContinuationCollectorKind::OperatingDocument)
                .expect("first revision"),
            second
                .revision_for(ContinuationCollectorKind::OperatingDocument)
                .expect("second revision")
        );
    }

    #[test]
    fn debug_surfaces_do_not_expose_commitments_or_paths() {
        let observer = observer();
        let git = format!("{:?}", observer.git.borrow());
        let documents = format!("{:?}", observer.documents.borrow());
        assert!(!git.contains("SpecContentHash"));
        assert!(git.matches("[REDACTED]").count() >= 3);
        assert!(!documents.contains("AGENTS.md"));
        assert!(!documents.contains("ROADMAP.md"));
        assert!(documents.contains("[REDACTED]"));
    }
}
