# Executor SideEffect Event Append Report

## 1. Executive Summary

The first executor SideEffect event append slice is implemented.

`LocalExecutionRequest` now accepts explicit `LocalExecutionSideEffectEventInput` values, and `LocalExecutor::execute(...)` can append validated `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` workflow events before the targeted local skill invocation.

This remains a disclosure-only runtime event path. It does not execute side effects, create `SideEffectRecord` persistence, mutate providers, enable writes, add schemas, add CLI behavior, update examples, or change release posture.

## 2. Scope Completed

- Added explicit `LocalExecutionSideEffectEventInput` for targeted local step/skill invocation disclosure.
- Added `LocalExecutionRequest.side_effect_events`.
- Reused existing `SideEffectWorkflowEvent` constructors and validation.
- Appended only `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped`.
- Inserted append behavior after local step policy evaluation and before `SkillInvocationRequested`.
- Reused existing generic workflow-event audit projection.
- Added focused local executor tests for supported lifecycle states, later-step targeting, unsupported lifecycle fail-closed behavior, audit projection, and Debug redaction.
- Updated roadmap and concept/planning docs to describe the implemented boundary.

## 3. Scope Explicitly Not Completed

- No runtime side-effect execution.
- No `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor append support.
- No write-capable adapters.
- No provider mutations.
- No local filesystem writes through runtime side effects.
- No `SideEffectRecord` persistence in this executor append phase. A later explicit local `SideEffectRecordStore` persistence slice is documented in [SideEffect Record Store Report](SIDE_EFFECT_RECORD_STORE_REPORT.md).
- No SideEffect store in this executor append phase. A later explicit local `SideEffectRecordStore` persistence slice is documented in [SideEffect Record Store Report](SIDE_EFFECT_RECORD_STORE_REPORT.md).
- No automatic SideEffect discovery.
- No EvidenceReference side-effect attachment.
- No approval-side-effect linkage implementation.
- No report artifact referential integrity changes.
- No dedicated SideEffect audit sink emission.
- No SideEffect observability metrics.
- No workflow-declared SideEffect configuration.
- No runtime configuration for side effects.
- No workflow schema fields.
- No CLI rendering or commands.
- No examples.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No rollback or compensation behavior.
- No Level 3/4 autonomy enablement.
- No release posture changes.

## 4. API Summary

The new executor input is:

- `LocalExecutionSideEffectEventInput`

It contains:

- target `StepId`;
- target `SkillId`;
- target `SkillVersion`;
- validated `SideEffectWorkflowEvent`.

`LocalExecutionRequest` now carries:

- `side_effect_events: Vec<LocalExecutionSideEffectEventInput>`.

The input has a redaction-safe `Debug` implementation that redacts target identifiers, side-effect IDs, and reference strings while exposing lifecycle state and bounded counts.

## 5. Event Ordering Summary

For matching explicit inputs, the local executor appends supported SideEffect workflow events in this order:

1. `StepScheduled`
2. `PolicyDecisionRecorded`
3. `SideEffectProposed`, `SideEffectDenied`, or `SideEffectSkipped`
4. optional before-skill hook events, if supplied
5. `SkillInvocationRequested`

The event does not replace policy evaluation, create approvals, create evidence, create `SideEffectRecord` persistence, or imply that a side effect was attempted.

## 6. Validation Boundary Summary

The executor validates:

- the explicit input target step matches the active step before appending;
- target skill ID and skill version match the active skill;
- optional payload step/skill/correlation identity matches the active invocation context when present;
- lifecycle state is one of `Proposed`, `Denied`, or `Skipped`;
- the payload was already validated by `SideEffectWorkflowEvent::new(...)`;
- runtime event idempotency requirements are satisfied.

Unsupported `Attempted`, `Completed`, and `Failed` inputs fail closed before `SkillInvocationRequested`.

## 7. Audit Summary

No dedicated SideEffect audit sink behavior was added.

The implementation relies on existing generic `AuditEvent::from_workflow_event(...)` projection. Focused tests verify `SideEffectProposed` reaches the local audit sink through that existing projection path.

## 8. Redaction And Privacy Summary

The implementation:

- does not copy raw provider payloads;
- does not copy command output;
- does not copy parser payloads;
- does not copy raw specs;
- does not copy environment variable values;
- does not store credentials, tokens, authorization headers, or private keys;
- uses validated `SideEffectWorkflowEvent` payloads;
- uses redaction-safe `Debug` for executor-specific input;
- emits stable, non-leaking failure codes for unsupported lifecycle input.

## 9. Test Coverage Summary

Added tests cover:

- explicit `SideEffectProposed` append before skill invocation;
- generic audit projection for an appended SideEffect workflow event;
- explicit `SideEffectDenied` append;
- explicit `SideEffectSkipped` append;
- later-step targeting without firing on the current step;
- fail-closed rejection for unsupported `Attempted` append input;
- no `SkillInvocationRequested` after unsupported SideEffect lifecycle failure;
- redaction-safe `Debug` for executor SideEffect inputs.

Existing executor, WorkReport, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests are expected to continue passing.

## 10. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 11. Remaining Known Limitations

- Append behavior is explicit input only.
- Missing SideEffect input does not auto-discover side effects from skills, adapters, reports, audit records, or workflow specs.
- `Attempted`, `Completed`, and `Failed` remain deferred because they imply runtime side-effect execution semantics.
- No side-effect persistence existed in this executor append phase. A later explicit local `SideEffectRecordStore` persistence slice is documented in [SideEffect Record Store Report](SIDE_EFFECT_RECORD_STORE_REPORT.md).
- No EvidenceReference side-effect attachment exists.
- No runtime side-effect execution or write-capable adapter behavior exists.
- No schema or CLI surface exists for configuring SideEffect events.

## 12. Recommended Next Phase

Recommended next phase: **executor SideEffect event append review**.

The append path should be reviewed before any planning for runtime side-effect execution, write-capable adapters, or attempted/completed/failed SideEffect lifecycle events.
