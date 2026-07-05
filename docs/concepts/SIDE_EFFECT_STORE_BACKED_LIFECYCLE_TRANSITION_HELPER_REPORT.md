# SideEffect Store-Backed Lifecycle Transition Helper Report

## 1. Executive Summary

The store-backed SideEffect lifecycle transition helper phase is implemented. Workflow OS can now load an existing `SideEffectRecord` from a `SideEffectRecordStore`, validate an attempted/completed/failed lifecycle transition through the existing pure helper boundary, update the stored record, and return the same reference-only `SideEffectWorkflowEvent` payload for a future executor append path.

This remains pre-write governance infrastructure. It does not call providers, execute side effects, append workflow events, write report artifacts, expose CLI behavior, add schemas, add examples, or change release posture.

## 2. Scope Completed

- Added store-backed transition input types for attempted, completed, and failed lifecycle transitions.
- Added explicit store-backed helper functions:
  - `transition_side_effect_to_attempted_in_store(...)`;
  - `transition_side_effect_to_completed_in_store(...)`;
  - `transition_side_effect_to_failed_in_store(...)`.
- Added `SideEffectRecordStore::update_side_effect_record(...)` as an explicit replace boundary for reviewed lifecycle transitions.
- Implemented update support for local and in-memory state backends.
- Preserved the existing pure transition helpers as the only transition construction path.
- Returned validated reference-only event payloads without appending events.
- Added focused tests for successful store-backed transitions, missing prior records, read failures, write failures, non-leakage, and repeated transition rejection.

## 3. Scope Explicitly Not Completed

- No provider calls.
- No runtime side-effect execution.
- No executor append path for attempted/completed/failed lifecycle events.
- No workflow run mutation by these helpers.
- No report artifact writes.
- No CLI behavior.
- No workflow schema changes.
- No examples.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy.
- No release posture changes.

## 4. Helper API Summary

The new helpers accept a `SideEffectRecordStore`, a stable `SideEffectId`, transition timestamp, bounded summary, additional stable references, and transition-specific outcome or reason fields. They load the prior record, call the matching pure transition helper, update the stored record through the store boundary, and return `SideEffectLifecycleTransitionResult`.

The helpers do not fabricate `SideEffectRecord` values, do not recreate evidence, do not append workflow events, and do not infer provider outcomes.

## 5. Store Boundary Summary

`SideEffectRecordStore::write_side_effect_record(...)` remains create-only. Lifecycle transitions preserve `SideEffectId`, so this phase adds an explicit `update_side_effect_record(...)` boundary for replacing an existing record with the same immutable workflow/run identity.

Stores must fail closed when the record is missing, invalid, or identity-mismatched. Unsupported stores receive a default stable unsupported error until they implement the update boundary.

## 6. Lifecycle And Idempotency Summary

The store-backed helpers enforce the same lifecycle semantics as the pure helpers:

- `Proposed` can transition to `Attempted` only when authority allows it.
- `Attempted` can transition to `Completed` only with a valid outcome reference.
- `Attempted` can transition to `Failed` only with a failure reference or stable reason code.
- Repeating an attempted transition after the store has already moved the record to attempted fails closed through lifecycle validation.

Full replay/idempotent-return semantics remain deferred. The current helper favors fail-closed behavior over ambiguous state.

## 7. Event Boundary Summary

The helpers return a reference-only event payload for future executor integration, but they do not append workflow events. Event append ordering, reconciliation, audit projection, and executor mutation remain separate future phases.

## 8. Redaction And Privacy Summary

The implementation does not copy provider payloads, command output, CI logs, raw specs, parser payloads, credentials, authorization headers, private keys, token-like values, or filesystem paths into transition errors. Store read/write failures are mapped to stable non-leaking transition error codes.

## 9. Test Coverage Summary

Focused tests cover:

- proposed-to-attempted store-backed transition;
- attempted-to-completed store-backed transition;
- attempted-to-failed store-backed transition;
- stable ID preservation;
- store update rather than duplicate create;
- missing prior record failure;
- store read failure mapping;
- store write failure mapping without partial update;
- repeated transition rejection;
- non-leaking error messages;
- existing pure transition behavior.

## 10. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test side_effect`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 11. Remaining Known Limitations

- Store-backed helpers do not append workflow events.
- Store-backed helpers do not call providers or execute writes.
- Replay-as-success semantics are deferred.
- Executor ordering between provider attempt, store update, event append, audit projection, and report artifact citation remains future work.
- Automatic validation in existing executor/report paths remains deferred.
- A review blocker found that the public store update boundary needed lifecycle enforcement in addition to documentation intent. That blocker is fixed in [SideEffect Store-Backed Lifecycle Transition Blocker Fix Report](SIDE_EFFECT_STORE_BACKED_LIFECYCLE_TRANSITION_BLOCKER_FIX_REPORT.md).

## 12. Recommended Next Phase

Recommended next phase: store-backed lifecycle transition helper review.

The helper is write-readiness-adjacent and should receive focused maintainer review before executor attempted/completed/failed event append behavior, provider write orchestration, or runtime side-effect execution is planned.

## 13. Dogfood Governance

- Workflow: `dg/implement`
- Run: `run-1783266426808532000-2`
- Approval: `approval/run-1783266426808532000-2/implementation-approved`
- Approval outcome: granted by delegated maintainer.
- Phase close status: completed.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations.
- Event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.
- Out-of-kernel work: repository edits, test execution, documentation updates, and validation commands were performed by the agent outside the kernel under the governed phase boundary.
