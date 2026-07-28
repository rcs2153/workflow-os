# Authoritative Artifact Concurrent Reconciliation Blocker Fix Report

## 1. Executive Summary

Full workspace validation for the authoritative-governance scaffold opt-in
reproducibly exposed a concurrency defect in the existing authoritative
WorkReport artifact path. Two identical concurrent persistence calls could
both observe an empty approval proof-marker projection store. One call then
published a projection record while the other observed the final filename
before its JSON payload was complete and failed duplicate reconciliation.

The local projection store now publishes complete, synced records atomically.
The projection persistence helper also re-reads concurrent duplicates and
classifies an identical record as already present while continuing to reject a
conflicting record.

## 2. Blocker Fixed

Before this fix, the composed exactly-once test could produce:

```text
Persisted + PersistenceFailed
```

instead of:

```text
Persisted + AlreadyPersisted
```

The failure appeared under full-suite load even when repeated isolated runs
usually passed. It blocked a truthful `cargo test --workspace` result.

## 3. Implementation Approach

- Write each projection payload to a unique temporary file.
- Flush and sync the complete temporary payload.
- Atomically publish it with duplicate-safe hard-link semantics.
- Sync the containing directory after publication.
- Remove the temporary file on success or failure.
- When publication reports a duplicate, read the existing record.
- Treat an identical record as `AlreadyPresent`.
- Fail closed on a conflicting or unreadable duplicate.

No retry delay, scheduler assumption, overwrite, or weakened assertion was
introduced.

## 4. Validation Boundary

The fix changes only approval proof-marker projection persistence and its
composition into existing authoritative artifact persistence. It does not
change:

- approval requirements;
- projection identity;
- artifact identity;
- public duplicate errors from the raw projection store;
- report or scaffold schemas;
- provider behavior; or
- external-write authority.

## 5. Test Coverage

Focused coverage now synchronizes two projection writers at a barrier and
requires exactly one persisted result plus one already-present result. The
existing composed artifact test continues to require exactly one persisted
artifact plus one reconciled duplicate.

The composed test also passed 50 consecutive isolated repetitions after the
fix.

## 6. Validation Performed

Passed:

- barrier-synchronized projection concurrency regression;
- composed authoritative artifact concurrency regression;
- 50 consecutive composed-regression repetitions;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations` under Node 20;
- `npm run check:docs`; and
- `git diff --check`.

## 7. Governed Phase Record

- workflow id: `dg/blocker`
- run id: `run-1785214462210834000-2`
- approval id: `approval/run-1785214462210834000-2/fix-approved`
- presentation id: `presentation/f6061b759498e361`
- approval outcome: granted by delegated maintainer
- approval proof: persisted
- event summary: 39 events, one approval, zero retries, zero escalations
- out-of-kernel work: repository edits and validation were performed by Codex
  under the approved blocker scope

## 8. Remaining Limitations

- The store remains local-filesystem only.
- Atomic publication depends on same-filesystem hard-link semantics.
- This fix does not provide a distributed transaction or hosted lock service.
- Batch projection writes remain record-atomic rather than transaction-atomic
  across multiple distinct records.

## 9. Recommended Next Phase

Complete full workspace validation and close the authoritative-governance
scaffold opt-in phase. Then perform the already recommended disposable
external-repository evaluation. Do not broaden providers, OpenShell, or
external-write authority as part of this blocker fix.
