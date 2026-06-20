# Executor SideEffect Event Append Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation delivers the intended bounded slice: explicit local executor append support for `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` workflow events before local skill invocation. It remains disclosure-only, uses existing model validation, preserves workflow event sourcing, and does not introduce writes or runtime side-effect execution.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- explicit `LocalExecutionSideEffectEventInput`;
- `LocalExecutionRequest.side_effect_events`;
- local executor append support for `SideEffectProposed`;
- local executor append support for `SideEffectDenied`;
- local executor append support for `SideEffectSkipped`;
- fail-closed behavior for unsupported attempted/completed/failed inputs;
- generic audit projection through the existing workflow-event audit path;
- focused tests and documentation updates.

No accidental implementation found for:

- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` append support;
- write-capable adapters;
- provider mutations;
- local filesystem writes through runtime side effects;
- `SideEffectRecord` persistence;
- SideEffect store behavior;
- automatic SideEffect discovery;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- report artifact referential integrity changes;
- dedicated SideEffect audit sink emission;
- SideEffect observability metrics;
- workflow-declared SideEffect configuration;
- runtime SideEffect configuration;
- workflow schema fields;
- CLI rendering or commands;
- examples;
- hosted/distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 3. API Assessment

The API is small and appropriate for the phase.

`LocalExecutionSideEffectEventInput` carries:

- target `StepId`;
- target `SkillId`;
- target `SkillVersion`;
- validated `SideEffectWorkflowEvent`.

Adding `side_effect_events` to `LocalExecutionRequest` is consistent with the accepted plan and mirrors the explicit pre-invocation hook posture. The input is explicit, caller-supplied, and does not read hidden global state, runtime config, workflow schema fields, provider state, local files, or external services.

The exported type is reasonable for the current preview surface, but it remains a public Rust API addition. Compatibility hardening should be revisited before a stable release.

## 4. Event Ordering Assessment

The ordering is appropriate:

1. `StepScheduled`
2. `PolicyDecisionRecorded`
3. SideEffect disclosure event, if supplied and validated
4. optional before-skill hook events
5. `SkillInvocationRequested`

This preserves the key invariant: policy is evaluated before the disclosure event, and the disclosure event is appended before skill invocation. The implementation does not replace policy evaluation, create policy decisions, create approvals, or imply a side-effect attempt.

## 5. Lifecycle Assessment

The supported lifecycle set is correctly bounded:

- `Proposed`;
- `Denied`;
- `Skipped`.

The unsupported set remains blocked:

- `Attempted`;
- `Completed`;
- `Failed`.

This is the right boundary. Proposed/denied/skipped can honestly describe intent or non-attempted outcomes without implying mutation. Attempted/completed/failed would imply runtime execution and adapter outcome semantics that do not exist yet.

## 6. Validation And Failure Assessment

The executor validates:

- input target step before append;
- target skill ID and version against the active skill;
- optional payload step ID, skill ID, skill version, and correlation ID against the active invocation context;
- supported lifecycle state;
- existing `SideEffectWorkflowEvent` model validation.

Unsupported lifecycle input fails closed before `SkillInvocationRequested`, appends no SideEffect event, and records a stable failure code.

One nuance: failure is represented as a failed workflow run after `RunStarted` and policy evaluation, rather than returning a top-level executor `Err`. That matches the existing before-skill hook failure posture and is acceptable for this phase, but docs for future phases should consistently call this "fail the scoped run with a structured runtime failure" rather than "return an executor error" when the run already exists.

## 7. Audit And Observability Assessment

The implementation correctly reuses the existing generic workflow-event audit projection. It does not add a dedicated SideEffect audit sink or observability metric.

This is consistent with the approved scope. Dedicated audit/observability behavior should remain a separate phase if needed.

## 8. Privacy And Redaction Assessment

The implementation remains redaction-safe.

Verified:

- no raw provider payloads are copied;
- no raw command output is copied;
- no parser payloads are copied;
- no raw spec contents are copied;
- no environment variable values are copied;
- no credentials, tokens, authorization headers, or private keys are stored;
- `SideEffectWorkflowEvent` constructors validate references and redaction metadata;
- `LocalExecutionSideEffectEventInput` has redaction-safe `Debug`;
- unsupported lifecycle errors use stable, non-leaking messages.

Serialization of the SideEffect event payload remains governed by the existing model tests. The new executor-specific input is not a serialized public schema.

## 9. Runtime And Idempotency Assessment

Workflow events remain the runtime source of truth. The SideEffect append path uses normal event append and rehydration behavior, and duplicate run IDs still rehydrate existing runs rather than appending duplicate disclosures.

The SideEffect event idempotency key is present and bounded. It is compact and sufficient for the current state-preserving disclosure-only event path because it is not used as the idempotency store key for external mutation.

Before any attempted/completed/failed lifecycle or write execution phase, SideEffect event idempotency should be hardened to use a more semantically scoped seed that includes active run/step/skill/lifecycle context without exceeding identifier bounds.

## 10. Test Quality Assessment

Strong coverage added:

- explicit proposed append before skill invocation;
- denied append;
- skipped append;
- later-step targeting;
- unsupported attempted lifecycle fails before skill invocation;
- no SideEffectAttempted event on unsupported input;
- no `SkillInvocationRequested` after unsupported input;
- generic audit projection for proposed SideEffect event;
- redaction-safe executor input Debug;
- existing executor, runtime, WorkReport, EvidenceReference, adapter, validation, and CLI tests still pass.

Shallow or missing coverage:

- no direct test for skill ID/version mismatch rejection;
- no direct test for payload step/skill/correlation mismatch rejection;
- unsupported lifecycle coverage tests `Attempted` but not `Completed` and `Failed`;
- audit assertion is explicit for `Proposed`; denied/skipped rely on existing generic audit projection tests;
- no test proves duplicate run replay with side-effect inputs does not append duplicate SideEffect events, though existing duplicate-run behavior and event rehydration tests cover the broader mechanism.

These are non-blocking because the implementation path is explicit, bounded, and passes broader runtime/event tests.

## 11. Documentation Review

Docs were updated and remain honest.

Verified docs state:

- explicit local executor proposed/denied/skipped SideEffect append is implemented;
- runtime side-effect execution is not implemented;
- writes are not implemented;
- provider mutations are not implemented;
- SideEffect persistence/store behavior is not implemented;
- automatic SideEffect discovery is not implemented;
- EvidenceReference side-effect attachment is not implemented;
- schemas are not implemented;
- CLI rendering/commands are not implemented;
- examples are not updated;
- hosted/distributed runtime behavior is not implemented;
- reasoning lineage is not implemented;
- release posture is unchanged.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Add direct tests for skill ID/version mismatch rejection.
- Add direct tests for payload step/skill/correlation mismatch rejection.
- Add direct tests that `Completed` and `Failed` are rejected like `Attempted`.
- Add a duplicate-run replay test with supplied SideEffect inputs to lock in non-duplication behavior.
- Before any write-capable phase, redesign SideEffect event idempotency keys around active run/step/skill/lifecycle context while respecting identifier length limits.
- Align future failure-behavior wording around run-scoped fail-closed behavior when execution has already started.

## 14. Recommended Next Phase

Recommended next phase: SideEffect persistence and discovery planning before runtime side-effect execution planning.

Reason: the executor can now durably disclose proposed/denied/skipped SideEffect events, but there is still no SideEffect source-of-truth record, persistence model, or discovery policy for reports/evidence beyond explicitly supplied IDs. Those foundations should be clarified before attempted/completed/failed events or write-capable adapters are planned.

Runtime side-effect execution and write-capable adapters should remain deferred until persistence, idempotency, authority, approval, audit, evidence, and report citation boundaries are reviewed together.

## 15. Validation

Validation commands run for this review:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
