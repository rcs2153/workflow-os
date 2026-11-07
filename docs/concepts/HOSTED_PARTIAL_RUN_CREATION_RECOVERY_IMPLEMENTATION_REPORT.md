# Hosted Partial-Run Creation Recovery Implementation Report

## 1. Executive Summary

The bounded Core/PostgreSQL recovery seam for early hosted creation prefixes
is implemented. One exact immutable hosted request now produces a deterministic
expected creation prefix. PostgreSQL classifies the durable prefix and appends
only its missing lawful suffix under one transaction-scoped run lock, then
reprojects the snapshot in the same transaction.

This report does not close the P0. Focused review, exact-head CI, independent
rerun, broader hosted/API contention coverage, and backup/restore rehearsal
remain required by the accepted plan.

## 2. Scope Completed

- Added a PostgreSQL-owned creation-prefix reconciliation operation.
- Bound deterministic repaired-event identities to the run, immutable bundle
  root, sequence, and typed transition label.
- Used the immutable bundle creation timestamp for newly repaired suffix
  events.
- Serialized classification and repair with a transaction-scoped advisory
  lock keyed to the run.
- Verified authoritative event semantics before repair.
- Preserved compatibility with prefixes created by the prior random event-ID
  and timestamp implementation while rejecting changed authoritative fields.
- Committed missing events and the derived snapshot in one transaction.
- Returned post-commit events to the executor for audit, observability, and
  structured-log emission after durable authority exists.
- Reused the existing approval and atomic hosted-dispatch paths after the
  scheduled prefix is proven.

## 3. Failure And Replay Behavior

A database failure while appending any missing suffix event rolls back the
entire repair transaction. Exact retry can then repeat classification without
inventing history. A complete prefix replays without duplicate events and
does not rewrite an already-matching snapshot. A divergent event kind,
metadata field, typed payload, idempotency key, actor, correlation, or
immutable identity fails closed with a bounded prefix-conflict diagnostic.

Runtime signals remain post-commit and non-authoritative. A signal failure can
surface after durable commit, but it cannot roll back or duplicate the event
history. The implementation does not claim transactional delivery to external
audit, observability, or logging sinks.

## 4. Permanent Test Coverage

The PostgreSQL shared-state suite now proves:

- legacy partial prefixes ending after `RunCreated`, `RunValidated`, and
  `RunStarted`;
- injected failure at each corresponding missing transition;
- transaction rollback leaves the preexisting prefix unchanged;
- fault removal plus exact retry completes the prefix;
- a second retry appends nothing;
- a newly constructed backend replays the complete prefix;
- two concurrent repair workers elect one durable suffix; and
- a legacy-looking prefix with changed actor authority fails closed without a
  write; and
- the existing queue-insert recovery, changed-context rejection, restart, and
  concurrent dispatch tests continue to pass.

## 5. Validation

- `cargo fmt --all`: passed.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- Focused native PostgreSQL shared-state milestone test: passed against an
  isolated PostgreSQL 18.6 instance.
- `cargo test -p workflow-core --test postgres_state_backend --no-run`: passed.
- `cargo test -p workflow-core --test local_executor`: passed, 351 passed and
  one opt-in live test ignored.
- The unrestricted `cargo test -p workflow-core` run passed all 786 unit tests
  before becoming silent in the existing `audit_projection` integration
  target. It was terminated after several minutes without output. The same
  target passed 10 of 10 immediately when rerun in isolation; no complete
  unrestricted crate result is claimed.

Documentation checks, diff checks, governed phase closure, focused review, and
exact-head CI are recorded separately when this implementation phase
completes.

## 6. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1794013069279443000-2`.
- Approval: `approval/run-1794013069279443000-2/implementation-approved`.
- Approval presentation: `presentation/e9f0afa2f1df7c51`.
- Approval outcome: granted under delegated maintainer authority with
  persisted presentation proof.
- Out-of-kernel work: source edits, PostgreSQL fault injection, tests,
  validation, and this report were performed by the delegated maintainer. The
  dogfood runner governed the phase and did not execute those actions.
- Phase close: `Completed`, 39 events, one approval, zero retries, and zero
  escalations.

## 7. Explicit Non-Goals And Remaining Acceptance

This phase adds no schema, generic event-semantic migration, second dispatch
path, provider mutation family, nested harness, release, publication, or
hosted-production claim.

The P0 remains open until focused review accepts the code and transition
matrix, exact-head CI is green, and an independent clean-environment rerun
proves the parent failures are corrected. The accepted plan also retains
broader API contention and backup/restore evidence as closure requirements.
