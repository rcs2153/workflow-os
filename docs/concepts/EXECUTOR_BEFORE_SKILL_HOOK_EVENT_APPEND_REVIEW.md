# Executor Before-Skill Hook Event Append Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation delivers the approved narrow slice: one explicit `BeforeSkillInvocation` hook input on `LocalExecutionRequest`, bounded `HookInvocationRequested` and `HookInvocationEvaluated` workflow events before `SkillInvocationRequested`, generic audit projection through the existing append path, fail-closed behavior before skill invocation, and unchanged executor behavior when no explicit hook input is supplied.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved explicit executor hook event append scope.

Confirmed in scope:

- explicit `BeforeSkillInvocation` hook input for one targeted local skill invocation;
- hook input validation against active workflow/run/schema/spec/step/skill identity;
- existing in-memory hook helper reuse;
- bounded hook workflow event append through the existing executor append pipeline;
- generic audit projection through existing workflow event audit projection;
- focused tests and documentation/report updates.

No accidental implementation was found for:

- automatic hook invocation;
- broad executor hook checkpoint support;
- workflow-declared hook configuration;
- runtime hook configuration;
- post-terminal workflow events;
- conversion of `BeforeReport` into workflow events;
- dedicated hook audit sink emission;
- hook audit store or persistence;
- hook observability metrics;
- WorkReport hook event citation targets;
- CLI behavior;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- external provider calls;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- report artifact writes;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted/distributed runtime claims;
- release posture changes.

## 3. API Assessment

`LocalExecutionBeforeSkillInvocationHookInput` is explicit and narrow. It carries caller-supplied hook invocation identity, target step identity, target skill identity, target skill version, and the already validated hook invocation input.

`LocalExecutionRequest` now includes optional `before_skill_invocation_hook`. Existing call sites set this to `None`, so the no-hook path remains explicit in tests and CLI code. The new field is public API surface and should be watched before release hardening, but it is acceptable for the current preview-stage, Rust-owned core model.

The custom `Debug` implementation redacts hook invocation ID, step ID, skill ID, skill version, workflow ID, and run ID while exposing counts and hook kind. That matches the repository's redaction posture for caller-supplied governed context.

## 4. Event Ordering Assessment

The persisted successful event order is correct:

1. `StepScheduled`
2. `PolicyDecisionRecorded`
3. `HookInvocationRequested`
4. `HookInvocationEvaluated`
5. `SkillInvocationRequested`
6. `SkillInvocationStarted`
7. skill and terminal run events

The executor resolves the local skill handler before invoking the hook, which avoids emitting hook events for a skill that cannot be executed. This is a reasonable internal ordering choice because handler lookup is not a workflow event and does not perform the governed skill action.

The implementation executes the pure in-memory hook helper before appending the requested/evaluated events so invalid hook input can fail without leaving partial hook events. This is acceptable for the implemented helper because it is non-mutating and side-effect-free. Before warning, skipped, blocked, or side-effect-aware hook semantics are introduced, the requested/evaluated event semantics should be revisited explicitly.

## 5. Validation Boundary Assessment

Validation ensures:

- hook kind must be `BeforeSkillInvocation`;
- target skill ID and skill version must match the active skill;
- hook workflow ID, workflow version, run ID, schema version, spec hash, and step ID must match the active execution plan;
- hook workflow event payload construction reuses the existing bounded `AgentHarnessHookWorkflowEvent` validator;
- input/output reference counts are bounded and converted through stable non-leaking errors;
- unsupported hook statuses fail closed.

Validation errors use stable codes and do not include raw hook reference values.

## 6. Failure Behavior Assessment

Invalid hook identity fails the run before hook events or skill invocation events are appended.

Hook helper failure, such as a side-effect request, fails the run before `SkillInvocationRequested` and appends no partial hook events. That matches the implementation report and current tests.

Unsupported hook statuses currently fail closed with `executor.hook.before_skill_invocation.unsupported_status`. The runtime helper currently returns passed for valid non-side-effecting inputs, so deeper status handling remains future work.

No misleading user project diagnostic is created. Hook failures are runtime execution failures with stable codes.

## 7. Idempotency And Replay Assessment

Hook requested/evaluated idempotency keys are deterministic and derived from the skill invocation idempotency key with compact hook-specific suffixes.

Duplicate execution with the same run ID rehydrates the existing run and does not duplicate hook events. Replay remains deterministic because hook workflow events are state-preserving from `Running` and carry bounded payloads.

The implementation preserves existing step outputs, approvals, retries, and reports; hook events do not mutate those surfaces by themselves.

## 8. Audit And Observability Assessment

Hook workflow events are emitted through the existing executor append pipeline. Generic `AuditEvent` projection records `HookInvocationRequested` and `HookInvocationEvaluated` as workflow event audit entries.

The implementation does not emit dedicated `AgentHarnessHookAuditRecord` values through an audit sink, does not add a hook audit store, and does not add hook observability metrics.

Tests assert generic audit event projection and no adapter observability/report artifact side effects.

## 9. Privacy And Redaction Assessment

The workflow event payload stores bounded hook identity and reference-count metadata, not hook input payloads.

The implementation does not copy:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira/GitHub bodies;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded hook context;
- unbounded disclosures;
- evidence payloads.

Debug paths for the explicit executor hook input are redaction-safe. Runtime hook workflow event debug behavior was already redaction-safe and remains so.

## 10. Test Quality Assessment

The focused tests cover:

- successful explicit hook event append;
- event order relative to policy and skill invocation;
- hook event payload shape;
- generic audit projection;
- no report artifact creation;
- no hook events on the no-hook executor path;
- hook helper failure with no partial hook or skill events;
- identity mismatch non-leakage;
- idempotent replay without duplicate hook events;
- hook kind vocabulary.

Existing workspace tests also cover runtime hook event state transitions, audit projection, hook invocation validation, WorkReport hook citation vocabulary, executor report behavior, local check boundaries, adapter boundaries, evidence references, and CLI behavior.

Non-blocking test gaps:

- add a multi-step test where the explicit hook targets a later step rather than the first step;
- add an explicit test that a hook targeting a different step is ignored until the matching step;
- add an explicit missing-handler test proving hook events are not appended when the local skill handler is absent;
- add a debug/serialization regression test for `LocalExecutionRequest` containing secret-like hook references if this request type becomes serialized or externally exposed later.

## 11. Documentation Review

Documentation now states:

- the first explicit `BeforeSkillInvocation` executor hook event append path is implemented;
- automatic hook invocation is not implemented;
- workflow-declared hook configuration is not implemented;
- runtime hook configuration is not implemented;
- dedicated hook audit sink emission is not implemented;
- hook persistence is not implemented;
- `BeforeReport` remains report-path-only, in-memory-only, and non-mutating;
- CLI behavior is not implemented;
- workflow schema fields are not implemented;
- automatic local check execution is not implemented;
- side effects and writes remain unsupported;
- recursive agents, agent swarms, hosted behavior, and release posture changes are not introduced.

The fix-forward notes in older hook audit and vocabulary reports preserve historical review context while avoiding stale current-state claims.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add later-step targeted hook tests before broadening to multiple checkpoints.
- Add a missing-handler regression test proving no hook events are appended when the skill handler is absent.
- Clarify requested/evaluated event semantics before supporting warning, skipped, failed-closed, or blocked statuses.
- Consider an options/builder-style request API before release hardening so optional executor features do not repeatedly expand public struct literals.
- Decide whether hook invocation IDs should remain caller-supplied or become deterministic per step attempt before retry-aware hook semantics are introduced.

## 14. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation hook status and failure semantics planning**.

Fix-forward note: that planning is now documented in [BeforeSkillInvocation Hook Status And Failure Semantics Plan](../implementation-plans/before-skill-hook-status-failure-semantics-plan.md). The plan recommends a semantics-preserving boundary hardening test phase before any hook status broadening.

Reason: the first state-visible hook checkpoint is accepted for passed hook results. Before adding more checkpoints, automatic configuration, workflow schema fields, or dedicated hook audit records, the project should decide how warning, skipped-with-disclosure, failed-closed, blocked, retry, and escalation semantics should behave without creating fake evidence, hidden side effects, or ambiguous workflow state.

That next phase must not implement automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, post-terminal workflow events, dedicated hook audit sinks, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 15. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
