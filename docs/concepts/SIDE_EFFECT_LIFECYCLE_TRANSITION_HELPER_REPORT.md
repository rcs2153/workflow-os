# SideEffect Lifecycle Transition Helper Report

Status: Implementation complete.

## 1. Executive Summary

Workflow OS now has pure SideEffect lifecycle transition helpers for the pre-write boundary:

- `Proposed -> Attempted`;
- `Attempted -> Completed`;
- `Attempted -> Failed`.

The helpers construct validated `SideEffectRecord` values and reference-only `SideEffectWorkflowEvent` payloads from explicit inputs. They do not call providers, append workflow events, write stores, execute side effects, expose CLI behavior, change schemas, update examples, add hosted behavior, implement reasoning lineage, expand autonomy, or change release posture.

## 2. Scope Completed

- Added pure transition input types:
  - `SideEffectAttemptTransitionInput`;
  - `SideEffectCompleteTransitionInput`;
  - `SideEffectFailTransitionInput`.
- Added `SideEffectLifecycleTransitionResult`.
- Added pure transition helpers:
  - `transition_side_effect_to_attempted(...)`;
  - `transition_side_effect_to_completed(...)`;
  - `transition_side_effect_to_failed(...)`.
- Exported the helper API from `workflow-core`.
- Added focused lifecycle transition tests.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- provider calls;
- live GitHub pull request comment creation;
- runtime side-effect execution;
- automatic executor attempted/completed/failed append behavior;
- store-backed lifecycle writes;
- CLI mutation commands or rendering;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Helper API Summary

The helpers accept an explicit prior `SideEffectRecord`, transition timestamp, bounded optional summary, additional stable references, and evidence-reference counts.

Completed transitions additionally require a validated `SideEffectOutcomeReference`.

Failed transitions require a validated failure outcome reference or stable non-secret reason code.

Each helper returns:

- a transitioned `SideEffectRecord`;
- a reference-only `SideEffectWorkflowEvent` payload for a future caller to append later.

The helpers do not append that event.

## 5. Lifecycle Boundary Summary

The implementation enforces:

- attempted transitions require a prior `Proposed` record;
- completed transitions require a prior `Attempted` record;
- failed transitions require a prior `Attempted` record;
- denied and skipped records cannot transition to attempted;
- completed records require outcome references through existing `SideEffectRecord` validation;
- failed records require a failure reference or stable reason code through existing validation;
- attempted/completed/failed records require allowed authority through existing validation;
- attempted/completed/failed records reject unknown capability through existing validation.

## 6. Event Boundary Summary

The transition result includes a `SideEffectWorkflowEvent` payload with:

- stable `SideEffectId`;
- lifecycle state;
- step/skill/correlation context preserved from the record;
- stable references;
- evidence-reference count supplied by the caller;
- outcome-reference count derived from the transitioned record;
- redaction metadata and sensitivity preserved from the record.

The helper does not create a `WorkflowRunEvent`, append event history, mutate snapshots, or emit audit records.

## 7. Redaction And Privacy Summary

The helpers rely on existing `SideEffectRecord`, `SideEffectWorkflowEvent`, reference, outcome, summary, reason-code, and redaction validation.

They do not copy:

- raw provider payloads;
- raw request bodies;
- raw command output;
- raw CI logs;
- raw GitHub pull request bodies or diffs;
- credentials;
- authorization headers;
- token-like values.

Errors use stable non-leaking codes. Debug output for the transition result reports counts and lifecycle posture only.

## 8. Test Coverage Summary

Focused tests cover:

- proposed-to-attempted transition;
- attempted-to-completed transition;
- attempted-to-failed transition;
- denied/skipped prior state rejection;
- non-attempted completed rejection;
- missing failed reason/reference rejection;
- allowed authority requirement;
- identity preservation;
- reference-only event payload shape;
- no store mutation;
- no event append behavior;
- debug/serialization non-leakage;
- secret-like summary rejection without leaking the value;
- existing SideEffect approval-linkage and model tests.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test side_effect`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

Governed implementation:

- workflow: `dg/implement`;
- run: `run-1783263516877372000-2`;
- approval: `approval/run-1783263516877372000-2/implementation-approved`;
- approval outcome: granted by delegated maintainer;
- phase closeout: completed;
- events: 39 total, 1 approval, 0 retries, 0 escalations.

## 10. Remaining Known Limitations

- Store-backed lifecycle transition writes are not implemented.
- Executor attempted/completed/failed append behavior is not implemented.
- Provider calls are not implemented.
- Live GitHub sandbox mutation is not implemented.
- CLI mutation behavior is not implemented.
- Idempotent replay behavior remains conservative and should be planned before store-backed transition writes.

## 11. Recommended Next Phase

Proceed to SideEffect lifecycle transition helper review.

Do not proceed to live provider mutation yet. After review, the next likely implementation should be store-backed lifecycle transition planning or helper implementation, depending on review findings.
