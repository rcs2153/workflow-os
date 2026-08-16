# Authorized Execution Continuity Event And State Projection Plan Blocker Fix Review

## 1. Executive Verdict

Plan accepted; proceed to atomic event/state projection implementation.

The corrected plan closes all six planning blockers and is implementable
against the current runtime and SQLite architecture. Implementation must remain
limited to the atomic projection contract and proof. It does not authorize a
supervisor or operational execution-window opening.

## 2. Scope Verification

The correction remains planning-only. No Rust model, runtime event, state
trait, SQLite schema, migration, executor path, supervisor, approval behavior,
provider mutation, CLI, workflow schema, nested runtime, or release posture was
changed.

## 3. Global Event And Snapshot Atomicity

Accepted. The corrected plan requires all SQLite event writers to share a
transaction-scoped append and derived-snapshot primitive. Independent snapshot
writes become monotonic cursor-checked writes and cannot regress or skip durable
history. This closes the stale overwrite path identified in review.

The existing source-level `RunSnapshotStore` contract may remain, but the
SQLite implementation must reject stale and history-inconsistent projections.
Conformance must cover legacy caller behavior after atomic append begins saving
the derived snapshot.

## 4. Cursor Semantics

Accepted. The plan now separates the expected-input cursor from the contiguous
committed-result cursor and defines the authoritative record write set for all
five operation families. Historical exact replay validates event-stream prefix
lineage and lawful later successors rather than requiring current state to
equal a historical cursor.

The implementation must preserve the accepted owner-to-target replay rules
when window cursors advance. No operation may use the newly allocated result
cursor as if it were caller-observed pre-operation authority.

## 5. Security-Rejection Projection

Accepted. Both applied and durably committed security-rejection results receive
one closed, bounded event. Rejection carries only a stable class, grants no
authority, and changes no workflow status. Exact replay returns its original
event binding without duplication.

## 6. Runtime Transition Matrix

Accepted. The single continuity event is status-preserving and forbidden after
workflow terminal state. Applied operations cannot occur from
`WaitingForApproval`; only bounded committed rejection disclosure is permitted
there. Workflow resume and terminal transitions remain separate events and are
not smuggled into continuity projection.

## 7. Transaction-Scoped Semantic Reuse

Accepted. The plan now requires private `&Transaction` operation bodies shared
by standalone and projected SQLite APIs. It forbids nested public transactions
and duplicated state machines. Parity tests are mandatory, so the refactor
cannot silently change the accepted semantic V2 behavior.

## 8. Commitment And Schema

Accepted. The projection commitment now has versioned, domain-separated,
canonical length-prefixed material. It excludes raw serialized JSON and display
text. SQLite adapter schema V3 is explicit, with atomic V2-to-V3 upgrade,
checksums, interruption behavior, and old-writer rejection.

The event ID remains kernel-allocated and relationally bound rather than being
derived from a public digest.

## 9. Privacy And Error Assessment

Accepted. The event, snapshot cache, binding, commitment, and errors remain
limited to bounded IDs, enums, revisions, counts, timestamps, and commitments.
No raw source/spec contents, transcripts, command output, provider payloads,
environment values, paths, credentials, tokens, or capability material are
authorized.

Implementation must preserve stable non-leaking corruption and ambiguous-commit
errors and deterministic error precedence.

## 10. Test Assessment

The corrected matrix is sufficient for implementation. It now covers stale
snapshot rejection, generic event races, committed-rejection replay, all
per-operation cursor write sets, lawful successor replay, the complete runtime
transition matrix, transaction-scoped parity, commitment determinism, V2-to-V3
upgrade, restart, corruption, and all-five-family commit faults.

Full workspace and integration validation remains required.

## 11. Remaining Limitation

The current continuity capability does not expose an operational
execution-window opening transaction. Test fixtures can seed windows, but the
future supervisor cannot claim an end-to-end production path from that fact.
This does not block projection of the five already accepted operations. It must
be addressed explicitly before or within a separately governed supervisor
vertical-slice plan, including its own event and authority binding.

## 12. Governed Re-Review Record

- workflow: `dg/review`;
- run: `run-1786867617773207000-2`;
- approval:
  `approval/run-1786867617773207000-2/review-scope-approved`;
- presentation: `presentation/b21cf2534dae9c86`;
- presentation hash:
  `b21cf2534dae9c86b78b09ebc95b8740b550a86f525ea2e90255ec88f1e81c96`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented and assessed;
- governed status: completed; and
- out-of-kernel work: source inspection, re-review writing, and validation
  were performed by the external executor. The kernel did not edit files, run
  checks, schedule an agent, or mutate a provider.

## 13. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed;
- current runtime event, snapshot, continuity, and SQLite transaction APIs:
  inspected directly.

## 14. Blockers

None.

## 15. Non-Blocking Follow-Ups

- Require an explicit operational window-opening boundary before the future
  supervisor claims end-to-end continuity.
- Keep report citation and operator rendering out of the implementation.
- Preserve SQLite as scoped opt-in support rather than a default backend.

## 16. Recommended Next Phase

Implement the atomic continuity event/state projection contract and SQLite V3
proof only. Perform focused maintainer/security review before any trusted-host
supervisor work.
