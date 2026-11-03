//! Private Phase D accepted-intent capture and collection boundary.
//!
//! The boundary stores only a bounded semantic effect attributed to one actor.
//! It never stores raw conversation, grants authority, or consumes approval or
//! capability state. Capture is append-only, optimistic-revision fenced, and
//! supersession is committed atomically with the replacement record.

use std::{collections::BTreeSet, fmt};

use serde::Serialize;

use super::{
    collection_commitment, collection_error, empty_accounting_claims_for,
    ContinuationCollectedAccounting, ContinuationCollectionAnchor,
    ContinuationCollectionObservation, ContinuationCollectionRequest,
    ContinuationCollectionResultPosture, ContinuationCollectorKind, ContinuationCollectorRevision,
    ContinuationContextCollector, ALL_COLLECTOR_KINDS,
};
use crate::continuation_context::{
    AcceptedIntentKind, AcceptedIntentRecord, AcceptedIntentRecordId, AcceptedIntentSupersession,
    ContinuationContextScope,
};
use crate::{SpecContentHash, WorkflowOsError};

const MAX_ACCEPTED_INTENTS_PER_SCOPE: usize = 128;

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AcceptedIntentStoreSnapshot {
    scope: ContinuationContextScope,
    records: Vec<AcceptedIntentRecord>,
    revision: SpecContentHash,
}

impl fmt::Debug for AcceptedIntentStoreSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcceptedIntentStoreSnapshot")
            .field("scope", &"[REDACTED]")
            .field("records", &self.records.len())
            .field("revision", &"[REDACTED]")
            .finish()
    }
}

impl AcceptedIntentStoreSnapshot {
    fn new(
        scope: ContinuationContextScope,
        mut records: Vec<AcceptedIntentRecord>,
    ) -> Result<Self, WorkflowOsError> {
        records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
        validate_records(&scope, &records)?;
        let revision = collection_commitment("accepted-intent-snapshot", &(&scope, &records))?;
        Ok(Self {
            scope,
            records,
            revision,
        })
    }
}

/// Store contract required by accepted-intent capture and collection.
///
/// Implementations must compare and replace atomically. Returning `false`
/// means the expected revision was stale and no state was changed.
trait AcceptedIntentRecordStore {
    fn read_snapshot(
        &self,
        scope: &ContinuationContextScope,
    ) -> Result<AcceptedIntentStoreSnapshot, WorkflowOsError>;

    fn compare_and_replace(
        &self,
        expected_revision: &SpecContentHash,
        replacement: &AcceptedIntentStoreSnapshot,
    ) -> Result<bool, WorkflowOsError>;
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AcceptedIntentCaptureRequest {
    record: AcceptedIntentRecord,
    expected_revision: SpecContentHash,
}

impl fmt::Debug for AcceptedIntentCaptureRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcceptedIntentCaptureRequest")
            .field("record", &self.record)
            .field("expected_revision", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AcceptedIntentCaptureDisposition {
    Stored,
    Replayed,
}

#[derive(Clone, Eq, PartialEq)]
struct AcceptedIntentCaptureResult {
    disposition: AcceptedIntentCaptureDisposition,
    record_id: AcceptedIntentRecordId,
    revision: SpecContentHash,
}

impl fmt::Debug for AcceptedIntentCaptureResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcceptedIntentCaptureResult")
            .field("disposition", &self.disposition)
            .field("record_id", &"[REDACTED]")
            .field("revision", &"[REDACTED]")
            .finish()
    }
}

#[allow(clippy::too_many_lines)]
fn capture_accepted_intent<S: AcceptedIntentRecordStore>(
    store: &S,
    mut request: AcceptedIntentCaptureRequest,
) -> Result<AcceptedIntentCaptureResult, WorkflowOsError> {
    request.record.supersedes.sort();
    if request.record.supersession != AcceptedIntentSupersession::Current {
        return Err(intent_error(
            "capture.record_not_current",
            "a newly captured accepted-intent record must be current",
        ));
    }
    if request
        .record
        .supersedes
        .windows(2)
        .any(|pair| pair[0] == pair[1])
    {
        return Err(intent_error(
            "capture.supersedes_duplicate",
            "accepted-intent supersession contains a duplicate identity",
        ));
    }

    let snapshot = store.read_snapshot(&request.record.scope)?;
    if let Some(existing) = snapshot
        .records
        .iter()
        .find(|existing| existing.record_id == request.record.record_id)
    {
        if existing == &request.record {
            return Ok(AcceptedIntentCaptureResult {
                disposition: AcceptedIntentCaptureDisposition::Replayed,
                record_id: existing.record_id.clone(),
                revision: snapshot.revision,
            });
        }
        return Err(intent_error(
            "capture.record_conflict",
            "accepted-intent identity is already bound to different content",
        ));
    }
    if snapshot.revision != request.expected_revision {
        return Err(intent_error(
            "capture.revision_stale",
            "accepted-intent capture expected an older store revision",
        ));
    }
    if snapshot
        .records
        .iter()
        .any(|existing| existing.directive_commitment == request.record.directive_commitment)
    {
        return Err(intent_error(
            "capture.directive_conflict",
            "attributable directive is already bound to another accepted-intent record",
        ));
    }

    let supersedes = request.record.supersedes.iter().collect::<BTreeSet<_>>();
    for record_id in &supersedes {
        let Some(existing) = snapshot
            .records
            .iter()
            .find(|existing| &existing.record_id == *record_id)
        else {
            return Err(intent_error(
                "capture.superseded_missing",
                "accepted-intent capture cannot supersede a missing record",
            ));
        };
        if existing.supersession != AcceptedIntentSupersession::Current {
            return Err(intent_error(
                "capture.superseded_not_current",
                "accepted-intent capture can supersede only a current record",
            ));
        }
        if !same_supersession_family(existing.kind, request.record.kind) {
            return Err(intent_error(
                "capture.superseded_kind_mismatch",
                "accepted-intent capture cannot supersede a record from another intent family",
            ));
        }
    }

    let conflicting_current = snapshot.records.iter().find(|existing| {
        existing.supersession == AcceptedIntentSupersession::Current
            && same_singleton_family(existing.kind, request.record.kind)
            && !supersedes.contains(&existing.record_id)
    });
    if conflicting_current.is_some() {
        return Err(intent_error(
            "capture.current_conflict",
            "accepted-intent singleton replacement must supersede the exact current record",
        ));
    }

    let mut records = snapshot.records;
    for existing in &mut records {
        if supersedes.contains(&existing.record_id) {
            existing.supersession = AcceptedIntentSupersession::Superseded;
        }
    }
    records.push(request.record.clone());
    let replacement = AcceptedIntentStoreSnapshot::new(request.record.scope.clone(), records)?;
    if !store.compare_and_replace(&snapshot.revision, &replacement)? {
        return Err(intent_error(
            "capture.concurrent_update",
            "accepted-intent store changed before atomic capture completed",
        ));
    }
    let verified = store.read_snapshot(&request.record.scope)?;
    if verified.revision != replacement.revision || !verified.records.contains(&request.record) {
        return Err(intent_error(
            "capture.readback_mismatch",
            "accepted-intent capture could not verify the committed replacement",
        ));
    }
    Ok(AcceptedIntentCaptureResult {
        disposition: AcceptedIntentCaptureDisposition::Stored,
        record_id: request.record.record_id,
        revision: verified.revision,
    })
}

fn same_singleton_family(left: AcceptedIntentKind, right: AcceptedIntentKind) -> bool {
    left != AcceptedIntentKind::Amendment && same_supersession_family(left, right)
}

fn same_supersession_family(left: AcceptedIntentKind, right: AcceptedIntentKind) -> bool {
    matches!(
        (left, right),
        (AcceptedIntentKind::Objective, AcceptedIntentKind::Objective)
            | (AcceptedIntentKind::Amendment, AcceptedIntentKind::Amendment)
            | (AcceptedIntentKind::Priority, AcceptedIntentKind::Priority)
            | (
                AcceptedIntentKind::Pause | AcceptedIntentKind::Stop | AcceptedIntentKind::Resume,
                AcceptedIntentKind::Pause | AcceptedIntentKind::Stop | AcceptedIntentKind::Resume,
            )
    )
}

fn validate_records(
    scope: &ContinuationContextScope,
    records: &[AcceptedIntentRecord],
) -> Result<(), WorkflowOsError> {
    if records.len() > MAX_ACCEPTED_INTENTS_PER_SCOPE {
        return Err(intent_error(
            "snapshot.record_limit",
            "accepted-intent snapshot exceeds its bounded record limit",
        ));
    }
    if records.iter().any(|record| &record.scope != scope) {
        return Err(intent_error(
            "snapshot.scope_mismatch",
            "accepted-intent snapshot contains a record for another scope",
        ));
    }
    let record_ids = records
        .iter()
        .map(|record| &record.record_id)
        .collect::<BTreeSet<_>>();
    if record_ids.len() != records.len() {
        return Err(intent_error(
            "snapshot.record_duplicate",
            "accepted-intent snapshot contains duplicate record identities",
        ));
    }
    let directive_commitments = records
        .iter()
        .map(|record| &record.directive_commitment)
        .collect::<BTreeSet<_>>();
    if directive_commitments.len() != records.len() {
        return Err(intent_error(
            "snapshot.directive_duplicate",
            "accepted-intent snapshot contains duplicate directive commitments",
        ));
    }
    for family in [
        AcceptedIntentKind::Objective,
        AcceptedIntentKind::Priority,
        AcceptedIntentKind::Pause,
    ] {
        if records
            .iter()
            .filter(|record| {
                record.supersession == AcceptedIntentSupersession::Current
                    && same_singleton_family(record.kind, family)
            })
            .count()
            > 1
        {
            return Err(intent_error(
                "snapshot.current_conflict",
                "accepted-intent snapshot contains conflicting current singleton records",
            ));
        }
    }
    for record in records {
        for superseded_id in &record.supersedes {
            if superseded_id == &record.record_id {
                return Err(intent_error(
                    "snapshot.supersession_self_reference",
                    "accepted-intent record cannot supersede itself",
                ));
            }
            let Some(superseded) = records
                .iter()
                .find(|candidate| &candidate.record_id == superseded_id)
            else {
                return Err(intent_error(
                    "snapshot.supersession_dangling",
                    "accepted-intent record contains a dangling supersession reference",
                ));
            };
            if superseded.supersession != AcceptedIntentSupersession::Superseded
                || !same_supersession_family(superseded.kind, record.kind)
            {
                return Err(intent_error(
                    "snapshot.supersession_invalid",
                    "accepted-intent record contains invalid supersession lineage",
                ));
            }
        }
    }
    Ok(())
}

struct AcceptedIntentContinuationSourceSet<'a, S> {
    store: &'a S,
}

impl<'a, S: AcceptedIntentRecordStore> AcceptedIntentContinuationSourceSet<'a, S> {
    const fn new(store: &'a S) -> Self {
        Self { store }
    }

    fn capture_anchor(
        &self,
        scope: &ContinuationContextScope,
    ) -> Result<ContinuationCollectionAnchor, WorkflowOsError> {
        let snapshot = self.store.read_snapshot(scope)?;
        let mut revisions = Vec::with_capacity(ALL_COLLECTOR_KINDS.len());
        for kind in ALL_COLLECTOR_KINDS {
            let revision = if kind == ContinuationCollectorKind::AcceptedIntent {
                snapshot.revision.clone()
            } else {
                collection_commitment("accepted-intent-outside-phase-d", &kind)?
            };
            revisions.push(ContinuationCollectorRevision { kind, revision });
        }
        ContinuationCollectionAnchor::new(None, None, revisions)
    }

    fn collector(&'a self) -> AcceptedIntentContinuationCollector<'a, S> {
        AcceptedIntentContinuationCollector { sources: self }
    }
}

struct AcceptedIntentContinuationCollector<'a, S> {
    sources: &'a AcceptedIntentContinuationSourceSet<'a, S>,
}

impl<S: AcceptedIntentRecordStore> ContinuationContextCollector
    for AcceptedIntentContinuationCollector<'_, S>
{
    fn kind(&self) -> ContinuationCollectorKind {
        ContinuationCollectorKind::AcceptedIntent
    }

    fn collect(
        &self,
        request: &ContinuationCollectionRequest,
    ) -> Result<ContinuationCollectionObservation, WorkflowOsError> {
        let snapshot = self.sources.store.read_snapshot(&request.scope)?;
        let expected = request
            .anchor
            .revision_for(ContinuationCollectorKind::AcceptedIntent)?;
        if &snapshot.revision != expected {
            return Err(intent_error(
                "collector.revision_drifted",
                "accepted-intent store changed after the collection anchor was captured",
            ));
        }
        let (posture, accounting) = if snapshot.records.is_empty() {
            let accounting = empty_accounting_claims_for(
                request.purpose,
                request.scope.run_id.is_some(),
                ContinuationCollectorKind::AcceptedIntent.source_class(),
            )?
            .into_iter()
            .map(|claim_kind| ContinuationCollectedAccounting { claim_kind })
            .collect();
            (ContinuationCollectionResultPosture::Empty, accounting)
        } else {
            (ContinuationCollectionResultPosture::Observed, Vec::new())
        };
        Ok(ContinuationCollectionObservation {
            kind: ContinuationCollectorKind::AcceptedIntent,
            request_commitment: request.commitment.clone(),
            revision: snapshot.revision,
            posture,
            source: None,
            accepted_intents: snapshot.records,
            accounting,
        })
    }
}

fn intent_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    collection_error(&format!("accepted_intent.{code}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::sync::Mutex;

    use super::*;
    use crate::continuation_context::{
        ContinuationContextPurpose, ContinuationContextSensitivity, ContinuationContextText,
    };
    use crate::{ActorId, ProjectId};

    struct InMemoryAcceptedIntentStore {
        snapshot: Mutex<AcceptedIntentStoreSnapshot>,
    }

    impl InMemoryAcceptedIntentStore {
        fn new(scope: ContinuationContextScope) -> Self {
            Self {
                snapshot: Mutex::new(
                    AcceptedIntentStoreSnapshot::new(scope, Vec::new()).expect("empty snapshot"),
                ),
            }
        }
    }

    impl AcceptedIntentRecordStore for InMemoryAcceptedIntentStore {
        fn read_snapshot(
            &self,
            scope: &ContinuationContextScope,
        ) -> Result<AcceptedIntentStoreSnapshot, WorkflowOsError> {
            let snapshot = self.snapshot.lock().expect("store lock").clone();
            if &snapshot.scope != scope {
                return Err(intent_error(
                    "store.scope_mismatch",
                    "accepted-intent store was opened for another scope",
                ));
            }
            Ok(snapshot)
        }

        fn compare_and_replace(
            &self,
            expected_revision: &SpecContentHash,
            replacement: &AcceptedIntentStoreSnapshot,
        ) -> Result<bool, WorkflowOsError> {
            let mut snapshot = self.snapshot.lock().expect("store lock");
            if &snapshot.revision != expected_revision {
                return Ok(false);
            }
            *snapshot = replacement.clone();
            Ok(true)
        }
    }

    fn scope() -> ContinuationContextScope {
        ContinuationContextScope {
            project_id: ProjectId::new("project/accepted-intent").expect("project"),
            workflow_id: None,
            run_id: None,
            phase: Some(ContinuationContextText::new("implementation").expect("phase")),
        }
    }

    fn record(id: &str, directive: &str, kind: AcceptedIntentKind) -> AcceptedIntentRecord {
        AcceptedIntentRecord {
            record_id: AcceptedIntentRecordId::new(id).expect("record id"),
            actor: ActorId::new("user/delegated-maintainer").expect("actor"),
            directive_commitment: SpecContentHash::from_text(directive),
            scope: scope(),
            kind,
            meaning: ContinuationContextText::new(format!("bounded meaning for {id}"))
                .expect("meaning"),
            supersedes: Vec::new(),
            supersession: AcceptedIntentSupersession::Current,
        }
    }

    fn request(
        record: AcceptedIntentRecord,
        expected_revision: SpecContentHash,
        supersedes: Vec<AcceptedIntentRecordId>,
    ) -> AcceptedIntentCaptureRequest {
        let mut record = record;
        record.supersedes = supersedes;
        AcceptedIntentCaptureRequest {
            record,
            expected_revision,
        }
    }

    #[test]
    fn capture_persists_attribution_and_replays_exact_record() {
        let store = InMemoryAcceptedIntentStore::new(scope());
        let initial = store.read_snapshot(&scope()).expect("initial");
        let intent = record(
            "intent/objective-a",
            "directive-a",
            AcceptedIntentKind::Objective,
        );
        let stored = capture_accepted_intent(
            &store,
            request(intent.clone(), initial.revision, Vec::new()),
        )
        .expect("capture");
        assert_eq!(stored.disposition, AcceptedIntentCaptureDisposition::Stored);
        let replay =
            capture_accepted_intent(&store, request(intent, stored.revision.clone(), Vec::new()))
                .expect("replay");
        assert_eq!(
            replay.disposition,
            AcceptedIntentCaptureDisposition::Replayed
        );
        assert_eq!(replay.revision, stored.revision);
    }

    #[test]
    fn singleton_replacement_requires_exact_atomic_supersession() {
        let store = InMemoryAcceptedIntentStore::new(scope());
        let initial = store.read_snapshot(&scope()).expect("initial");
        let first = record(
            "intent/objective-a",
            "directive-a",
            AcceptedIntentKind::Objective,
        );
        let first_result =
            capture_accepted_intent(&store, request(first.clone(), initial.revision, Vec::new()))
                .expect("first capture");
        let second = record(
            "intent/objective-b",
            "directive-b",
            AcceptedIntentKind::Objective,
        );
        let error = capture_accepted_intent(
            &store,
            request(second.clone(), first_result.revision.clone(), Vec::new()),
        )
        .expect_err("implicit replacement must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.accepted_intent.capture.current_conflict"
        );
        let replacement_request =
            request(second, first_result.revision, vec![first.record_id.clone()]);
        let replaced = capture_accepted_intent(&store, replacement_request.clone())
            .expect("exact replacement");
        let replay =
            capture_accepted_intent(&store, replacement_request).expect("exact replacement replay");
        assert_eq!(
            replay.disposition,
            AcceptedIntentCaptureDisposition::Replayed
        );
        assert_eq!(replay.revision, replaced.revision);
        let mut omitted_lineage = replayed_record(&store, "intent/objective-b");
        omitted_lineage.supersedes.clear();
        let error = capture_accepted_intent(
            &store,
            request(omitted_lineage, replay.revision.clone(), Vec::new()),
        )
        .expect_err("replay without durable supersession lineage must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.accepted_intent.capture.record_conflict"
        );
        let snapshot = store.read_snapshot(&scope()).expect("snapshot");
        assert_eq!(snapshot.records.len(), 2);
        assert_eq!(
            snapshot.records[0].supersession,
            AcceptedIntentSupersession::Superseded
        );
        assert_eq!(
            snapshot.records[1].supersession,
            AcceptedIntentSupersession::Current
        );
    }

    fn replayed_record(store: &InMemoryAcceptedIntentStore, id: &str) -> AcceptedIntentRecord {
        store
            .read_snapshot(&scope())
            .expect("snapshot")
            .records
            .into_iter()
            .find(|record| record.record_id.as_str() == id)
            .expect("stored record")
    }

    #[test]
    fn supersession_cannot_cross_intent_families() {
        let store = InMemoryAcceptedIntentStore::new(scope());
        let initial = store.read_snapshot(&scope()).expect("initial");
        let objective = record(
            "intent/objective-a",
            "directive-a",
            AcceptedIntentKind::Objective,
        );
        let stored = capture_accepted_intent(
            &store,
            request(objective.clone(), initial.revision, Vec::new()),
        )
        .expect("capture");
        let priority = record(
            "intent/priority-a",
            "directive-b",
            AcceptedIntentKind::Priority,
        );
        let error = capture_accepted_intent(
            &store,
            request(priority, stored.revision, vec![objective.record_id]),
        )
        .expect_err("cross-family supersession must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.accepted_intent.capture.superseded_kind_mismatch"
        );
    }

    #[test]
    fn stale_revision_and_directive_rebinding_fail_closed() {
        let store = InMemoryAcceptedIntentStore::new(scope());
        let initial = store.read_snapshot(&scope()).expect("initial");
        let first = record(
            "intent/amendment-a",
            "directive-a",
            AcceptedIntentKind::Amendment,
        );
        capture_accepted_intent(&store, request(first, initial.revision.clone(), Vec::new()))
            .expect("capture");
        let stale = capture_accepted_intent(
            &store,
            request(
                record(
                    "intent/amendment-b",
                    "directive-b",
                    AcceptedIntentKind::Amendment,
                ),
                initial.revision,
                Vec::new(),
            ),
        )
        .expect_err("stale revision");
        assert_eq!(
            stale.code(),
            "continuation_collection.accepted_intent.capture.revision_stale"
        );
        let current = store.read_snapshot(&scope()).expect("current");
        let duplicate_directive = capture_accepted_intent(
            &store,
            request(
                record(
                    "intent/amendment-c",
                    "directive-a",
                    AcceptedIntentKind::Amendment,
                ),
                current.revision,
                Vec::new(),
            ),
        )
        .expect_err("directive rebinding");
        assert_eq!(
            duplicate_directive.code(),
            "continuation_collection.accepted_intent.capture.directive_conflict"
        );
    }

    #[test]
    fn collector_is_revision_fenced_and_non_authorizing() {
        let store = InMemoryAcceptedIntentStore::new(scope());
        let sources = AcceptedIntentContinuationSourceSet::new(&store);
        let anchor = sources.capture_anchor(&scope()).expect("anchor");
        let request = ContinuationCollectionRequest::new(
            ContinuationContextPurpose::Implementation,
            scope(),
            ActorId::new("agent/replacement").expect("actor"),
            ContinuationContextSensitivity::Restricted,
            4096,
            anchor,
        )
        .expect("request");
        let initial = store.read_snapshot(&scope()).expect("initial");
        capture_accepted_intent(
            &store,
            request_for_store(
                record(
                    "intent/objective-a",
                    "directive-a",
                    AcceptedIntentKind::Objective,
                ),
                initial.revision,
            ),
        )
        .expect("capture");
        let error = sources
            .collector()
            .collect(&request)
            .expect_err("revision drift must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.accepted_intent.collector.revision_drifted"
        );

        let anchor = sources.capture_anchor(&scope()).expect("fresh anchor");
        let request = ContinuationCollectionRequest::new(
            ContinuationContextPurpose::Implementation,
            scope(),
            ActorId::new("agent/replacement").expect("actor"),
            ContinuationContextSensitivity::Restricted,
            4096,
            anchor,
        )
        .expect("request");
        let observation = sources.collector().collect(&request).expect("collect");
        assert_eq!(
            observation.posture,
            ContinuationCollectionResultPosture::Observed
        );
        assert_eq!(observation.accepted_intents.len(), 1);
        assert!(observation.source.is_none());
    }

    fn request_for_store(
        record: AcceptedIntentRecord,
        expected_revision: SpecContentHash,
    ) -> AcceptedIntentCaptureRequest {
        request(record, expected_revision, Vec::new())
    }

    #[test]
    fn debug_output_redacts_actor_meaning_and_commitments() {
        let store = InMemoryAcceptedIntentStore::new(scope());
        let snapshot = store.read_snapshot(&scope()).expect("snapshot");
        let request = request(
            record(
                "intent/private",
                "private-directive",
                AcceptedIntentKind::Amendment,
            ),
            snapshot.revision,
            Vec::new(),
        );
        let rendered = format!("{request:?}");
        assert!(!rendered.contains("user/delegated-maintainer"));
        assert!(!rendered.contains("bounded meaning"));
        assert!(!rendered.contains("private-directive"));
        let result = AcceptedIntentCaptureResult {
            disposition: AcceptedIntentCaptureDisposition::Stored,
            record_id: AcceptedIntentRecordId::new("intent/private-result").expect("record"),
            revision: SpecContentHash::from_text("private-revision"),
        };
        let rendered = format!("{result:?}");
        assert!(!rendered.contains("intent/private-result"));
        assert!(!rendered.contains("private-revision"));
    }
}
