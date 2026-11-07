# Hosted Partial-Run Creation Recovery Review

## 1. Verdict

Accepted for a bounded implementation PR after one test-coverage correction.
The implementation closes the Core/PostgreSQL code gap for the three
reproduced early creation prefixes. It does not close the full P0 or authorize
the cross-harness milestone yet.

## 2. Authority And Transaction Review

The repair decision remains Core-owned. The executor reconstructs the expected
typed prefix from the exact current execution plan and immutable bundle. The
PostgreSQL backend acquires a transaction-scoped run lock, reads the complete
durable stream, verifies authoritative semantics, appends only a missing
suffix, rehydrates the run, and repairs the snapshot before commit.

The queue/work-item path is unchanged. After a scheduled prefix is proven, the
executor reuses the existing approval wait or atomic hosted dispatch seam.
There is no second dispatch path and no caller-authored event skipping.

## 3. Legacy Compatibility Review

Previously shipped creation events used generated event IDs and wall-clock
timestamps. Rejecting those headers would make the reproduced stranded runs
unrepairable. The classifier therefore ignores only those two legacy header
fields while requiring exact equality for sequence, run/workflow/schema/spec
identity, correlation, actor, idempotency, and typed payload.

Newly repaired suffix events use deterministic event IDs derived from a
domain-separated commitment over run ID, immutable bundle root, sequence, and
transition label. Their timestamp is the immutable bundle creation time.

This compatibility rule does not provide cryptographic protection from a
database owner who can rewrite both relational columns and event payloads.
That storage-owner trust boundary remains unchanged and must not be presented
as tamper-evident storage.

## 4. Replay, Concurrency, And Signals

Serializable transactions plus the run-scoped advisory lock elect one repair
winner. Exact replay appends no event and leaves a matching snapshot unchanged.
A restart uses the same durable classification. Injected suffix failures roll
back all writes from that repair attempt.

Audit, observability, and structured-log signals remain post-commit. A signal
failure cannot revoke durable authority, but no durable outbox or automatic
signal redelivery is implemented in this phase. The implementation report
discloses this explicitly.

## 5. Review Correction

The initial permanent test matrix covered valid legacy prefixes, three fault
boundaries, rollback, retry, restart, and concurrent repair. Review added a
negative legacy-prefix case that changes the actor while retaining a
legacy-looking event ID. Reconciliation rejects it with
`postgres_state.hosted_creation.prefix_conflict` and appends nothing.

## 6. Remaining P0 Gates

- Exact-head required CI must be terminal green.
- An independent clean-environment rerun must verify the parent failures and
  accepted correction.
- Broader hosted/API contention evidence remains required.
- Backup/restore rehearsal over an interrupted prefix remains required.
- Public operator/API projection of the bounded recovery classification is not
  claimed by this implementation.

Until those gates close, the Active Delivery Queue must not advance to the
cross-harness milestone.

## 7. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1794015079600737000-2`.
- Approval: `approval/run-1794015079600737000-2/review-scope-approved`.
- Approval presentation: `presentation/761e96c87dc6a0b4`.
- Approval outcome: granted under delegated maintainer authority with
  persisted presentation proof.
- Out-of-kernel work: diff inspection, source and test correction, validation,
  and this review record were performed by the delegated maintainer. The
  dogfood runner governed the phase and performed no repository mutation.
