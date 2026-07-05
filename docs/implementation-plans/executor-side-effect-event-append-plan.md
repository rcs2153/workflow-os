# Executor SideEffect Event Append Plan

Status: Implemented for the first bounded executor append slice and accepted in [Executor SideEffect Event Append Review](../concepts/EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REVIEW.md). This plan follows the accepted SideEffect core model, WorkReport SideEffect citation vocabulary, terminal report and executor SideEffect report input propagation, model-only SideEffect workflow event vocabulary, and bounded generic SideEffect audit projection. `LocalExecutionRequest` can now accept explicit SideEffect event inputs, and the local executor can append validated `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` workflow events before local skill invocation. SideEffect persistence and discovery planning is documented in [SideEffect Persistence And Discovery Plan](side-effect-persistence-discovery-plan.md). Attempted/completed/failed append behavior remains separate and is now planned in [Executor SideEffect Lifecycle Event Append Plan](executor-side-effect-lifecycle-event-append-plan.md). This does not implement automatic SideEffect discovery, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, provider mutations, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has model vocabulary for representing SideEffect lifecycle moments in workflow events:

- `SideEffectProposed`;
- `SideEffectDenied`;
- `SideEffectSkipped`;
- `SideEffectAttempted`;
- `SideEffectCompleted`;
- `SideEffectFailed`.

It also has bounded generic audit projection for those events.

The next question is not how to perform writes. The next question is where an executor may append SideEffect lifecycle events when an already-governed workflow path needs durable side-effect disclosure.

This plan recommends a conservative first implementation: append only `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` events from explicit caller-supplied inputs in the local executor. Do not append `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` until runtime side-effect execution and write-capable adapter semantics are separately planned and reviewed.

## 2. Goals

- Define the smallest safe executor append boundary for SideEffect workflow events.
- Preserve `WorkflowRunEvent` as the runtime source of truth.
- Preserve replay determinism and current workflow pass/fail semantics.
- Reuse existing `SideEffectWorkflowEvent` model validation.
- Reuse existing generic `AuditEvent::from_workflow_event(...)` projection.
- Keep append behavior explicit and non-default.
- Support durable disclosure of proposed, denied, and skipped side effects without enabling writes.
- Avoid raw payload copying.
- Avoid fake SideEffect records, fake approvals, fake policy decisions, fake evidence, fake adapter telemetry, and fake outcomes.
- Prepare for future automatic WorkReport discovery without implementing it in this phase.

## 3. Non-Goals

Do not implement in the future append phase:

- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- local filesystem writes through runtime;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` append behavior;
- creation, mutation, or persistence of `SideEffectRecord` values;
- SideEffect store or persistence;
- automatic SideEffect discovery;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage implementation;
- report artifact referential integrity changes;
- dedicated SideEffect audit sink emission;
- SideEffect observability metrics;
- automatic executor SideEffect event append behavior;
- workflow-declared side-effect configuration;
- runtime configuration for side effects;
- workflow schema fields;
- CLI rendering, commands, or export;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented:

- `SideEffectId`;
- `SideEffectRecord`;
- `SideEffectLifecycleState`;
- `SideEffectTargetReference`;
- `SideEffectCapability`;
- `SideEffectAuthority`;
- `SideEffectIdempotencyBinding`;
- `SideEffectReference`;
- WorkReport SideEffect citation target vocabulary;
- terminal report helper support for supplied SideEffect IDs;
- executor report input propagation for supplied SideEffect IDs;
- model-only SideEffect workflow event vocabulary;
- state-preserving SideEffect workflow event replay from `Running`;
- idempotency-key requirement for SideEffect workflow events;
- generic audit projection for SideEffect workflow events.

Not implemented:

- SideEffect persistence;
- automatic SideEffect discovery;
- EvidenceReference side-effect attachment;
- runtime side-effect execution;
- write-capable adapter behavior.

Implemented after this plan:

- explicit local executor append behavior for caller-supplied `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` events.
- fail-closed rejection for `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed` append inputs until runtime side-effect execution is separately designed.

## 5. First Append Target Recommendation

Recommended first target: **explicit proposal/denial/skipping event append for caller-supplied SideEffect event payloads**.

The first executor append implementation should support only:

- `SideEffectProposed`;
- `SideEffectDenied`;
- `SideEffectSkipped`.

Rationale:

- These lifecycle states can honestly exist before any mutation attempt.
- They let the run durably disclose intended, blocked, or unsupported side effects.
- They are useful for final WorkReports and future discovery.
- They do not imply provider mutation.
- They can remain state-preserving from `Running`.

Defer:

- `SideEffectAttempted`;
- `SideEffectCompleted`;
- `SideEffectFailed`.

Reason: those events imply an attempted runtime mutation or adapter outcome. They should wait until runtime side-effect execution, idempotency replay, adapter write contracts, and provider mutation safety are separately designed.

## 6. Explicit Input Boundary

The future append implementation should accept explicit caller-supplied input.

Possible shape:

- `LocalExecutionRequest.side_effect_events`;
- or a narrower `LocalExecutionSideEffectEventInput` field;
- or a method-local helper used by a future executor wrapper.

The input should contain:

- side-effect event lifecycle kind;
- validated `SideEffectWorkflowEvent` payload or validated event definition;
- expected workflow identity;
- expected workflow version;
- expected schema version;
- expected spec hash;
- expected run ID;
- expected step ID where applicable;
- expected skill ID and version where applicable;
- explicit idempotency key or deterministic idempotency seed;
- sensitivity;
- redaction metadata.

The first implementation should not read hidden global state, runtime config, workflow schema fields, provider state, local files, or external services to construct side-effect events.

## 7. Identity And Scope Validation

Before appending a SideEffect workflow event, the executor must validate:

- run identity matches the active run;
- workflow identity matches the active run;
- workflow version matches the active run;
- schema version matches the active run;
- spec hash matches the active run;
- optional step ID matches the current or expected step context;
- optional skill ID/version matches the current or expected skill context;
- lifecycle kind matches payload lifecycle state;
- idempotency key is present;
- event payload passed `SideEffectWorkflowEvent` validation.

Identity mismatch should fail closed with stable, non-leaking error codes.

Do not append partial SideEffect event sequences.

## 8. Event Ordering Policy

For proposal/denial/skipping events, the first implementation should choose one concrete ordering point and avoid broad automatic behavior.

Recommended first ordering target:

1. `StepScheduled`
2. policy decision for the step or skill path
3. explicit SideEffect disclosure append, if supplied and validated
4. subsequent skill invocation or continuation behavior

The SideEffect event should not replace policy evaluation.

The SideEffect event should not create policy decisions.

The SideEffect event should not create approval requests.

The SideEffect event should not be appended after terminal states.

## 9. Lifecycle Semantics

Supported first lifecycle semantics:

- `Proposed`: a caller-supplied side-effect intent is disclosed as proposed, without attempt.
- `Denied`: a caller-supplied side-effect intent was blocked by policy, approval, capability, kill switch, validation, or unsupported posture.
- `Skipped`: a caller-supplied side-effect intent was intentionally not attempted.

Unsupported in the first append implementation:

- `Attempted`: requires runtime side-effect execution semantics.
- `Completed`: requires write-capable adapter outcome semantics.
- `Failed`: requires attempted side-effect failure semantics.

The future append path must not treat a proposed, denied, or skipped SideEffect event as evidence that a `SideEffectRecord` was created or persisted unless that becomes true in a separate persistence phase.

## 10. Failure Behavior

Recommended conservative behavior:

- If input validation fails before append, return a structured executor error and append no SideEffect events.
- If event construction fails, append no SideEffect events.
- If event append fails, return the existing append error without converting it into a user project diagnostic.
- If SideEffect disclosure is optional for the call path, failed append should fail the executor call rather than silently dropping a supplied governance event.
- If SideEffect disclosure is later contract-required, missing disclosure should fail closed before the scoped action.

Errors must use stable codes and must not leak:

- SideEffect target references;
- provider payloads;
- command output;
- parser payloads;
- paths;
- snippets;
- credentials;
- tokens;
- authorization headers;
- secret-like strings;
- raw redaction metadata values.

## 11. Idempotency And Replay

SideEffect workflow events already require idempotency keys.

The future executor append path should define deterministic idempotency keys from:

- run ID;
- step ID where available;
- skill ID/version where available;
- SideEffect ID;
- lifecycle state;
- append checkpoint name.

Duplicate execution behavior:

- duplicate executor calls must not append duplicate SideEffect events;
- replay must derive state from accepted workflow events;
- duplicate idempotency keys must not cause mutation attempts;
- absence of a SideEffect store must not trigger external lookup during replay.

## 12. Audit Behavior

SideEffect workflow events appended through the normal executor append path should project through the existing generic audit projection.

The first append implementation must not:

- call a dedicated SideEffect audit sink;
- create dedicated SideEffect audit records;
- emit SideEffect observability metrics;
- persist audit records outside existing behavior;
- add action vocabulary for side-effect proposal or denial.

The audit projection should remain bounded and reference-only.

## 13. Relationship To SideEffectRecord

The executor append path should not create or mutate `SideEffectRecord` values.

In this phase:

- the event cites `SideEffectId`;
- the event records lifecycle vocabulary and bounded references;
- the event does not prove that a durable SideEffect store exists;
- the event does not replace `SideEffectRecord` validation.

Future persistence planning should decide whether an appended SideEffect workflow event must refer to an existing SideEffect record, a proposed transient SideEffect ID, or both.

## 14. Relationship To Policy And Approval

SideEffect events must not create policy or approval decisions.

The future append implementation may cite stable references to existing policy or approval decisions only when those references are already available and supplied explicitly.

Denied SideEffect events may represent a policy or approval denial, but they must not become the authoritative approval or policy record.

Approval remains authority context. Approval is not a SideEffect lifecycle state.

## 15. Relationship To WorkReports And Discovery

This plan does not implement automatic WorkReport discovery.

However, appending bounded SideEffect workflow events prepares future discovery from accepted workflow event history.

Future discovery planning should decide:

- whether WorkReports cite SideEffect IDs from workflow events;
- whether WorkReports cite SideEffect IDs from a future SideEffect store;
- whether generic audit projections are acceptable discovery sources;
- how duplicates are deduplicated;
- how missing SideEffect records fail closed when report artifacts validate references.

Reports must not infer SideEffect history from prose, debug strings, local check output, command output, or adapter payloads.

## 16. Privacy And Redaction

Future appended SideEffect events must not store or copy:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- full side-effect target references;
- authority packets;
- reason-code bodies;
- outcome bodies;
- unbounded summaries.

The append path must use existing `SideEffectWorkflowEvent` constructors and must preserve Debug, serialization, deserialization, and audit projection safety.

## 17. Test Plan For Future Implementation

Future tests should cover:

- explicit proposed SideEffect input appends `SideEffectProposed`;
- explicit denied SideEffect input appends `SideEffectDenied`;
- explicit skipped SideEffect input appends `SideEffectSkipped`;
- attempted/completed/failed inputs are rejected in the first append implementation;
- event ordering is deterministic relative to policy decisions and skill invocation;
- appended SideEffect events project to generic audit events;
- no dedicated SideEffect audit records are emitted;
- no SideEffect observability metrics are emitted;
- no `SideEffectRecord` values are created or persisted;
- no automatic WorkReport discovery occurs;
- execute without explicit SideEffect input is unchanged;
- identity mismatch fails without leaking raw IDs or payloads;
- lifecycle mismatch fails without partial append;
- missing idempotency key fails without partial append;
- duplicate execution does not append duplicate SideEffect events;
- replay preserves run state;
- terminal state rejects SideEffect append;
- raw provider/spec/command/parser payload markers are not copied;
- secret-like redaction metadata and target-like values fail without leakage;
- existing runtime, audit, WorkReport, EvidenceReference, adapter, local check, hook, CLI, and docs tests still pass.

## 18. Proposed Implementation Sequence

1. Add an explicit executor input for proposed/denied/skipped SideEffect workflow events.
2. Validate input against run, workflow, schema, spec, step, and skill identity.
3. Reject attempted/completed/failed events in this first append path.
4. Use existing `SideEffectWorkflowEvent` constructors.
5. Append validated SideEffect workflow events through the existing event append pipeline.
6. Preserve state-preserving replay from `Running`.
7. Rely on existing generic audit projection.
8. Add focused executor, replay, audit, and non-leakage tests.
9. Review before any persistence, automatic discovery, attempted/completed/failed append behavior, runtime side-effect execution, or write-capable adapter work.

## 19. Deferred Work

- `SideEffectAttempted` append behavior;
- `SideEffectCompleted` append behavior;
- `SideEffectFailed` append behavior;
- SideEffect persistence;
- SideEffect store lookup or referential integrity;
- automatic WorkReport discovery;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- dedicated SideEffect audit sink or records;
- SideEffect observability metrics;
- workflow-declared side-effect configuration;
- runtime side-effect configuration;
- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- CLI behavior;
- schema changes;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation semantics.

## 20. Open Questions

- Should the first explicit input live on `LocalExecutionRequest`, a narrower executor wrapper, or an internal helper?
- Should proposal/denial/skipping append be tied to a specific step or allowed as run-level disclosure?
- Should supplied `SideEffectId` values need a future store-backed existence check before append?
- Should denied/skipped events be required when policy denies a write-like capability, or remain explicitly supplied first?
- Should WorkReport discovery prefer workflow events or a future SideEffect store?
- Should generic audit `input_reference` remain the only SideEffect ID projection, or should later audit models add a typed side-effect reference field?
- Should SideEffect proposal disclosure become required before any future write-capable adapter can be called?

## 21. Final Recommendation

The explicit executor SideEffect proposed/denied/skipped event append path is implemented and accepted.

Recommended next phase: **attempted/completed/failed lifecycle event append planning**, now documented in [Executor SideEffect Lifecycle Event Append Plan](executor-side-effect-lifecycle-event-append-plan.md).

Future implementation must still not add runtime side-effect execution, write-capable adapters, provider mutations, automatic discovery, EvidenceReference side-effect attachment, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, rollback/compensation behavior, or release posture changes unless separately scoped and approved.

## 22. Dogfood Context

This planning phase used the self-governed dogfood helper as a governance wrapper.

- `npm run dogfood:benchmark -- validate --no-build` passed with expected experimental lifecycle warnings.
- `npm run dogfood:benchmark -- start --no-build --run-id run/side-effect-executor-append-plan` started the governed run and paused at `approval/run/side-effect-executor-append-plan/planning-approved`.
- The approval checkpoint advanced after an explicit approve attempt, but the helper then surfaced an idempotency-key length issue. A retry showed the run was no longer waiting for approval.
- `npm run dogfood:benchmark -- inspect run/side-effect-executor-append-plan` showed the run advanced to `Running` with approval granted and event history preserved.

Boundary: the kernel governed validation, run identity, event history, and approval state. Codex drafted this planning document outside the kernel. The run did not complete cleanly, and that limitation is disclosed rather than repaired by hand.
