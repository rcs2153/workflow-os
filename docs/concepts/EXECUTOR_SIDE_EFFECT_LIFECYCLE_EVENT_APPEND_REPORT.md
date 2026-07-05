# Executor SideEffect Lifecycle Event Append Report

## 1. Executive Summary

The explicit local executor SideEffect lifecycle event append slice is implemented.

`LocalExecutionRequest` can now accept `LocalExecutionSideEffectLifecycleEventInput` values for attempted, completed, and failed SideEffect lifecycle events. Those inputs are constructed from validated `SideEffectLifecycleTransitionResult` values, preserving the store-backed SideEffect transition helper as the lifecycle authority.

This phase remains local, explicit, and opt-in. It does not perform provider writes, execute side effects, write report artifacts, add CLI behavior, add schemas, update examples, introduce hosted behavior, implement reasoning lineage, introduce recursive agents or agent swarms, enable Level 3/4 autonomy, or change release posture.

## 2. Scope Completed

- Added `LocalExecutionSideEffectLifecycleEventInput`.
- Added construction from validated `SideEffectLifecycleTransitionResult`.
- Added redaction-safe `Debug` behavior for the new input.
- Added `LocalExecutionRequest.side_effect_lifecycle_events`.
- Threaded lifecycle event inputs through the local executor execution plan.
- Appended `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed` workflow events before local skill invocation when the targeted step is active.
- Preserved existing explicit `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` append behavior.
- Kept generic caller-supplied `LocalExecutionSideEffectEventInput` rejected for attempted/completed/failed lifecycle states.
- Added focused executor tests.
- Updated roadmap and planning documentation.

## 3. Scope Explicitly Not Completed

- No provider writes.
- No live GitHub pull request comments.
- No Jira writes.
- No CI mutation.
- No runtime side-effect execution.
- No automatic lifecycle transition from default executor paths.
- No automatic report artifact writes.
- No CLI commands, rendering, or export.
- No workflow schema changes.
- No examples.
- No hosted or distributed runtime behavior.
- No credential loading or secret-provider integration.
- No rollback or compensation behavior.
- No reasoning lineage.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy.
- No release posture changes.

## 4. Helper/API Summary

The new API surface is:

- `LocalExecutionSideEffectLifecycleEventInput`
- `LocalExecutionSideEffectLifecycleEventInput::from_transition_result(...)`
- `LocalExecutionRequest.side_effect_lifecycle_events`

The constructor accepts the active step/skill identity and a validated transition result. It rejects transition outputs whose lifecycle is not attempted, completed, or failed, and it rejects inconsistent transition outputs where the transitioned record and workflow event disagree on side-effect ID or lifecycle state.

## 5. Lifecycle Behavior

The executor now appends lifecycle events as follows:

- `SideEffectLifecycleState::Attempted` becomes `WorkflowRunEventKind::SideEffectAttempted`.
- `SideEffectLifecycleState::Completed` becomes `WorkflowRunEventKind::SideEffectCompleted`.
- `SideEffectLifecycleState::Failed` becomes `WorkflowRunEventKind::SideEffectFailed`.

The events are appended before the targeted local skill invocation, after policy decision recording for the active step. Inputs for non-active steps are ignored until their target step is active, matching the existing proposed/denied/skipped SideEffect append posture.

## 6. Validation Boundary Summary

Validation ensures:

- lifecycle inputs are built from validated transition output;
- lifecycle state is attempted, completed, or failed;
- transition record and transition event agree on side-effect ID and lifecycle state;
- input skill ID and skill version match the active invocation;
- event step, skill, skill version, and correlation identity match when present;
- generic explicit SideEffect event inputs still reject attempted/completed/failed states.

Failures return stable non-leaking errors and produce no partial lifecycle event append.

## 7. Event/Audit Summary

The executor appends the reference-only `SideEffectWorkflowEvent` payload returned by the transition helper. Existing audit projection for SideEffect workflow event kinds remains the audit boundary; this phase does not introduce a new audit model or audit payload shape.

## 8. Workflow Semantics Summary

This phase does not change normal workflow pass/fail semantics. Lifecycle event append is an explicit request input. If lifecycle append validation fails during execution, the run fails closed before local skill invocation, matching existing explicit SideEffect append failure posture.

The executor does not mutate SideEffect stores, call providers, write artifacts, or create CLI output as part of this path.

## 9. Redaction/Privacy Summary

The new input `Debug` implementation redacts step IDs, skill IDs, skill versions, side-effect IDs, and reference values. Tests cover secret-like identifier non-leakage through Debug.

The executor appends reference-only SideEffect workflow events. It does not copy raw provider payloads, raw command output, raw CI logs, raw GitHub/Jira bodies, raw spec contents, raw parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or secret-like metadata.

## 10. Test Coverage Summary

Focused tests cover:

- attempted lifecycle transition result appends `SideEffectAttempted`;
- completed lifecycle transition result appends `SideEffectCompleted`;
- failed lifecycle transition result appends `SideEffectFailed`;
- lifecycle event ordering before local skill invocation;
- audit sink projection for attempted/completed/failed workflow events;
- identity mismatch fails closed before invocation;
- generic explicit SideEffect event inputs still reject completed lifecycle state;
- lifecycle input Debug output redacts identifiers and stable references;
- existing local executor behavior continues to pass.

## 11. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test local_executor side_effect_lifecycle -- --nocapture` - passed.
- `cargo test -p workflow-core --test local_executor executor_appends_ -- --nocapture` - passed.
- `cargo test -p workflow-core --test local_executor generic_side_effect_event_input_still_rejects_completed_lifecycle -- --nocapture` - passed.
- `cargo test -p workflow-core --test local_executor lifecycle_transition_event_identity_mismatch -- --nocapture` - passed.
- `cargo test -p workflow-core --test local_executor -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - initially found a documentation markdown issue and test-helper lint issues; after fixes, passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783269923858957000-2 --phase implementation` - passed.

## 12. Remaining Known Limitations

- Lifecycle event append and store transition are not atomic in one executor method.
- The executor does not perform the store-backed transition itself.
- A store transition success followed by event append failure still requires future reconciliation planning.
- Default executor paths do not automatically append attempted/completed/failed lifecycle events.
- Provider write orchestration is not implemented.
- Runtime side-effect execution is not implemented.
- CLI behavior, schemas, examples, hosted behavior, and release posture changes remain deferred.

## 13. Recommended Next Phase

Recommended next phase: **Executor SideEffect lifecycle event append review**.

This implementation is write-readiness-adjacent and should be reviewed before additional provider-write orchestration, runtime side-effect execution planning, or live mutation work.

## 14. Dogfood Governance

- Workflow: `dg/implement`.
- Run: `run-1783269923858957000-2`.
- Approval: `approval/run-1783269923858957000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: add bounded local executor/helper support for attempted/completed/failed SideEffect lifecycle event append from validated store-backed transition results plus focused tests, docs, and report.
- Strict non-goals: no provider writes, runtime side effects, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.
- Phase close status: completed.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations.
- Event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `StepScheduled`.
- Out-of-kernel work: repository code edits, docs edits, formatting, validation commands, and report writing were performed by the agent outside the kernel under the governed implementation phase boundary.
