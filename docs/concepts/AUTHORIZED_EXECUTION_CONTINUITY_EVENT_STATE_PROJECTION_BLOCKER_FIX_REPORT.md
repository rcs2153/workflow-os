# Authorized Execution Continuity Event And State Projection Blocker Fix Report

## 1. Executive Summary

The focused projection blockers are fixed. Projected-operation reconciliation
now opens a fresh SQLite connection and returns the original committed
continuity disposition together with a fully validated event/snapshot binding.
Partial, conflicting, or drifted durable presence fails closed with
`state.continuity_projection.corrupt`.

The fix also adds reusable in-memory-reference/SQLite fault observations for
all five continuity operation families. Before-commit faults write nothing;
during- and after-commit acknowledgement faults are ambiguous and require
reconciliation; exact replay appends no duplicate event.

This remains an internal, opt-in SQLite capability. It does not add a trusted
host supervisor, executor redispatch, provider mutation, public schema, CLI
surface, filesystem projection, or PostgreSQL projection.

## 2. Blockers Fixed

The fix addresses each blocker from the phase review:

1. projected reconciliation returns and verifies both the durable operation
   disposition and complete projection binding;
2. the all-five-operation projected fault matrix is compared against a bounded
   in-memory reference observation and exercised against SQLite;
3. injected during-commit uncertainty now reports
   `state.sqlite.commit_ambiguous`, rather than a known pre-commit failure;
4. generic event append and projected continuity mutation contend for one
   exact cursor; and
5. filesystem and PostgreSQL projected methods are proved unsupported before
   backend writes or connections.

## 3. Implementation Approach

`AuthorizedExecutionContinuityProjectionStore` now has a private
`reconcile_projected_operation` operation. Its durable result is either:

- `DurablyCommitted`, carrying the original bounded continuity disposition and
  validated `ContinuityProjectionBinding`; or
- `ConfirmedAbsent`, proving that neither the operation nor conflicting
  projection/receipt presence exists.

SQLite reconciliation reads through a fresh connection. It validates the
operation and receipt, relational binding metadata, canonical projection
commitment, expected and result event cursors, exact historical event payload,
snapshot commitment at the result cursor, and the current snapshot derived
from the complete lawful event history.

## 4. Validation Boundary

Complete durable presence is accepted only when all continuity, event,
snapshot, and binding records agree. Missing bindings, mismatched request or
receipt identity, relational drift, event drift, or snapshot drift returns the
same stable non-leaking corruption code.

Complete absence remains retryable. The implementation does not infer a
projection from an operation row, fabricate a binding, or repair state during
reconciliation.

## 5. Fault And Conformance Proof

The projected fault harness covers register yield, transition wait, consume
directive, record attempt outcome, and recover ambiguous attempt.

For each family:

- a before-commit fault returns `state.sqlite.write_failed`, leaves three seed
  events, stores no binding, and reconciles as absent;
- a during-commit fault returns `state.sqlite.commit_ambiguous`, commits one
  event and binding, reconciles on a fresh connection, and exact-replays
  without duplication; and
- an after-commit fault has the same durable ambiguous-acknowledgement posture.

One generic event writer racing a projected operation proves only one writer
can claim sequence four. Unsupported filesystem projection leaves its local
state layout unchanged, and unsupported PostgreSQL projection opens no
connection.

## 6. Privacy And Redaction

Reconciliation errors use fixed codes and messages. Tests inject secret-like
conflict and snapshot values and verify they do not appear in Debug output.
The returned binding remains private and contains bounded identifiers,
revisions, cursors, and commitments only. No prompt, transcript, source,
command output, provider payload, path, environment value, credential, token,
or bearer authority is added.

## 7. Tests Added

Focused coverage now includes:

- all five projected operation families across before/during/after faults;
- in-memory reference and SQLite observation parity;
- complete fresh-connection reconciliation returning disposition and binding;
- complete absence;
- missing binding;
- request conflict and snapshot drift with non-leaking errors;
- generic-event/projected-operation cursor contention;
- exact replay without another event; and
- filesystem/PostgreSQL unsupported zero-write behavior.

The complete `workflow-core` library suite passes with 319 tests.

## 8. Commands Run And Results

- focused projected reconciliation, fault, contention, and unsupported tests:
  passed;
- `workflow-core` library suite: passed, 319 tests; and
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `CARGO_TARGET_DIR=/private/tmp/workflow-os-target cargo test --workspace`:
  passed. The isolated target directory avoided a desktop-environment launch
  delay while preserving the complete workspace test scope;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 9. Scope Explicitly Not Completed

This fix does not implement operational execution-window opening, a
trusted-host supervisor, scheduler or executor redispatch, automatic approval
or evidence satisfaction, filesystem or PostgreSQL projection, provider
writes, public workflow schema, CLI behavior, hosted runtime, nested
harnesses, Reasoning Lineage, or production certification.

## 10. Remaining Known Limitations

SQLite projection remains opt-in and local. The in-memory reference adapter in
the fault harness proves the closed projected commit observations; SQLite
remains the only backend implementing the complete durable relational
contract. Current snapshots remain deterministic caches of the event stream,
not independent authority.

## 11. Governed Fix Record

- workflow: `dg/blocker`;
- run: `run-1786888521804520000-2`;
- approval: `approval/run-1786888521804520000-2/fix-approved`;
- presentation: `presentation/5e9bf5f5b5a6001e`;
- presentation hash:
  `5e9bf5f5b5a6001e995a3e7238d66da03bbc4ca16b5171e5bb551e7ffd0314fc`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- governed run status: completed; and
- out-of-kernel work: the external executor edited source and docs and ran
  validation. The kernel did not edit files, execute checks, commit, push,
  merge, schedule an executor, or mutate a provider.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Proceed to
operational execution-window opening and one injected local trusted-host
supervisor vertical slice only if the fix is accepted.
