# Authorized Execution Continuity Event And State Projection Report

## 1. Executive Summary

Workflow OS now binds each accepted SQLite authorized-execution continuity
mutation to one bounded runtime event and one deterministic run-snapshot
projection in the same immediate transaction. Continuity state, operation
receipt, runtime history, snapshot cursor, and relational projection binding
therefore commit or roll back together.

The implementation remains a local, opt-in SQLite capability. It does not
open execution windows, schedule or redispatch an executor, approve gates,
execute tools, add provider mutations, or expose workflow or CLI schema.

## 2. Scope Completed

- Added one closed, versioned
  `AuthorizedExecutionContinuityProjected` event payload.
- Added bounded applied and committed-security-rejection dispositions for all
  five accepted continuity operation families.
- Added expected-input and contiguous committed-result cursors.
- Added a private operation-to-event projection binding and canonical
  projection commitment.
- Added a non-authoritative `last_continuity_projection` snapshot cache.
- Reused one transaction-scoped SQLite continuity path for standalone and
  projected operations.
- Made generic SQLite event append derive and persist its snapshot before the
  same commit.
- Made independent snapshot writes monotonic and event-history-validating.
- Added explicit SQLite schema V3, relational constraints, and atomic V2-to-V3
  migration.
- Added exact replay that returns the original durable projection binding
  without appending another event.
- Declared filesystem and PostgreSQL projection unsupported with a stable
  fail-closed error.

## 3. Scope Explicitly Not Completed

This phase does not add a supervisor, scheduler, executor redispatch, model
turn creation, operational execution-window opening, automatic approval,
evidence satisfaction, policy inference, command or tool execution, provider
mutation, filesystem or PostgreSQL projection, default SQLite selection,
public schema, SDK, CLI, report rendering, nested harness execution, hosted or
distributed runtime, or Reasoning Lineage.

## 4. Event And Projection Model

The public event-safe vocabulary records only validated operation kind,
disposition, stable result or rejection class, operation and receipt
identities, projection commitment, exact cursors, and bounded target identity
and revision. The private binding additionally ties the operation request,
workflow/run/window identity, event, target, and derived snapshot commitment
together.

The event is inspection history, not authority. Current continuation posture
must still be read from authoritative continuity state.

## 5. Atomic SQLite Boundary

Projected operations use one `BEGIN IMMEDIATE` transaction. The transaction
validates continuity and runtime cursors, applies the existing semantic V2
transition, writes the operation and receipt, appends the projection event,
rehydrates the complete event history, persists the derived snapshot, stores
the relational binding, and commits once.

Before-commit failures roll back every write. Exact replay validates and
returns the original binding. Relational and bounded-JSON drift fails closed as
SQLite record corruption.

## 6. Schema And Migration

SQLite adapter schema V3 adds full event identity, snapshot cursor and
commitment columns, continuity window run identity, and the projection-binding
table with operation, event, cursor, target, and window foreign keys.

V1-to-V2 and V2-to-V3 upgrades remain separate and explicit. V2 databases
with committed continuity operations fail with
`state.sqlite.schema.upgrade_legacy_projection_required`; the migration does
not fabricate historical projection events. Incomplete, drifted, or newer
schemas fail closed.

## 7. Runtime And Replay Semantics

The projection event is status-preserving. Applied yield registration is
limited to running or retrying runs. Other applied operations are limited to
the accepted non-terminal states. Waiting-for-approval accepts only committed
security-rejection disclosure. Created, validated, and terminal histories
reject projection.

Exact replay adds no event and does not advance the snapshot. Same-operation
different-content replay fails before a write. A later lawful event-stream
successor does not invalidate the historical projection prefix.

## 8. Privacy And Redaction

Projection models, Debug output, serialization errors, SQL mapping errors, and
reconciliation errors are restricted to bounded identifiers, closed enums,
revisions, timestamps, counts, and commitments. They do not store source or
spec contents, prompts, transcripts, command output, provider payloads,
environment values, paths, credentials, tokens, or capability material.

## 9. Test Coverage

Focused tests cover all five projected operation families, one-event and
one-snapshot advancement, exact replay without duplication, relational
binding drift, runtime transition and serde rejection, atomic generic event
append, stale snapshot refusal, explicit V1-to-V2-to-V3 migration, rejection
of unprojectable V2 continuity history, concurrent writers, and existing
continuity conformance behavior.

The complete `workflow-core` library suite passes with 313 tests. Runtime event
tests pass with 50 tests, and SQLite backend integration tests pass with 16
tests.

## 10. Validation

Completed validation:

- `cargo test -p workflow-core --lib`: passed, 313 tests;
- `cargo test -p workflow-core --test runtime_events`: passed, 50 tests;
- `cargo test -p workflow-core --test sqlite_state_backend`: passed, 16 tests;
- focused all-five-family projected operation test: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 11. Remaining Known Limitations

- No operational execution-window opening path exists.
- The trusted-host supervisor cannot yet consume this projection capability.
- Filesystem and PostgreSQL projection remain unsupported.
- SQLite remains opt-in and is not production-certified.
- The event and snapshot disclose committed continuity facts but do not resume
  an executor or satisfy governance obligations.

## 12. Recommended Next Phase

Perform a focused maintainer and security review of this atomic projection
implementation. Only after acceptance should Workflow OS plan one injected
local trusted-host supervisor vertical slice with an explicit operational
window-opening boundary.

## 13. Governed Phase Record

- workflow: `dg/runtime-composition`;
- run: `run-1786868232612835000-2`;
- approval:
  `approval/run-1786868232612835000-2/composition-approved`;
- presentation: `presentation/a285ac3f7359cea2`;
- presentation hash:
  `a285ac3f7359cea22aa304720fea19ca6383a89f16499c66c0f57f1ebf69f57a`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- governed run status: completed; and
- out-of-kernel work: the external executor edited source and documentation
  and ran validation. The kernel did not edit files, execute checks, append
  source-control changes, schedule an agent, or mutate a provider.

## 14. Fix-Forward Note

The focused review correctly found that the original implementation did not
yet prove complete projected reconciliation or an all-five-family projected
fault matrix. Those findings remain preserved in the review. The bounded
follow-up is implemented and documented in [Authorized Execution Continuity
Event And State Projection Blocker Fix
Report](AUTHORIZED_EXECUTION_CONTINUITY_EVENT_STATE_PROJECTION_BLOCKER_FIX_REPORT.md).
Acceptance still requires a focused blocker-fix re-review.
