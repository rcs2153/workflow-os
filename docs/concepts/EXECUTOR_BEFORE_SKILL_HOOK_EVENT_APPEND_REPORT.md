# Executor Before-Skill Hook Event Append Report

## 1. Executive Summary

The first state-visible agent harness hook checkpoint is implemented for the local executor. `LocalExecutionRequest` now accepts one explicit `BeforeSkillInvocation` hook input, and `LocalExecutor::execute(...)` can append bounded `HookInvocationRequested` and `HookInvocationEvaluated` workflow events before `SkillInvocationRequested`.

The implementation remains narrow and opt-in. It does not add automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sink emission, hook persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 2. Scope Completed

- Added `AgentHarnessHookKind::BeforeSkillInvocation`.
- Added `LocalExecutionBeforeSkillInvocationHookInput`.
- Added optional explicit `before_skill_invocation_hook` input on `LocalExecutionRequest`.
- Validated hook input against active workflow, run, schema, spec hash, step, skill ID, and skill version.
- Executed the existing in-memory hook helper from explicit input.
- Appended `HookInvocationRequested` and `HookInvocationEvaluated` workflow events for supported passed hook results.
- Preserved existing `execute(...)` behavior when no explicit hook input is supplied.
- Preserved the existing `BeforeReport` report-path hook as in-memory-only and non-mutating.
- Added focused executor tests for ordering, audit projection, idempotent replay, failure behavior, and non-leakage.
- Updated roadmap, concept docs, and hook append planning docs.

## 3. Scope Explicitly Not Completed

- No automatic hook invocation.
- No broad executor hook checkpoint support.
- No workflow-declared hook configuration.
- No runtime hook configuration.
- No post-terminal workflow event model.
- No conversion of `BeforeReport` into workflow events.
- No dedicated hook audit sink emission.
- No hook audit store or persistence.
- No hook observability metrics.
- No WorkReport hook event citation target.
- No CLI behavior.
- No workflow schema fields.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No external provider calls.
- No `EvidenceReference` creation or attachment.
- No approval request or approval decision creation.
- No approval evidence attachment.
- No report artifact writes.
- No reasoning lineage.
- No side-effect boundary implementation.
- No writes.
- No recursive agents or agent swarms.
- No hosted/distributed runtime claims.
- No release posture changes.

## 4. API Summary

The new explicit input type is `LocalExecutionBeforeSkillInvocationHookInput`.

It carries:

- hook invocation ID;
- target step ID;
- target skill ID;
- target skill version;
- validated hook invocation input.

`LocalExecutionRequest` now includes optional `before_skill_invocation_hook`. When omitted, existing local executor behavior is unchanged.

## 5. Event Ordering

For the implemented path, the successful event order is:

1. `StepScheduled`
2. `PolicyDecisionRecorded`
3. `HookInvocationRequested`
4. `HookInvocationEvaluated`
5. `SkillInvocationRequested`
6. `SkillInvocationStarted`
7. terminal skill and run events

The hook is evaluated only after the policy decision allows the skill invocation path and only after the local skill handler exists.

## 6. Audit And Observability Behavior

Hook workflow events flow through the existing executor append path and existing generic `AuditEvent::from_workflow_event(...)` projection.

The implementation does not emit dedicated `AgentHarnessHookAuditRecord` values to an audit sink, does not add a hook audit store, and does not add hook observability metrics.

## 7. Idempotency And Replay

Hook requested and evaluated events use deterministic idempotency keys derived from the skill invocation idempotency key with compact hook-specific suffixes.

Duplicate execution with the same run ID rehydrates the existing run and does not duplicate hook events.

## 8. Failure Behavior

Invalid hook identity fails closed before any hook event or skill invocation event is appended.

Hook helper failure fails the run before skill invocation and does not append partial hook events.

Unsupported hook statuses are rejected in this phase. The implementation supports passed hook results only.

## 9. Privacy And Redaction Summary

The implementation stores bounded workflow event payloads with hook IDs, contract identity, hook kind, status, optional step/phase identity, reference counts, redaction metadata, and sensitivity.

It does not copy raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw spec contents, raw parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded hook context, unbounded disclosures, or evidence payloads.

Executor errors use stable codes and avoid leaking hook reference IDs, paths, payloads, tokens, or secret-like values.

## 10. Test Coverage Summary

Focused tests cover:

- explicit `BeforeSkillInvocation` hook events append in the expected order;
- hook events project through the generic audit path;
- no hook observability metrics or report artifacts are created;
- no explicit hook input preserves the existing event shape;
- hook helper failure appends no partial hook or skill invocation events;
- identity mismatch fails without leaking mismatched values;
- duplicate execution does not duplicate hook events;
- hook kind vocabulary includes `BeforeSkillInvocation`.

## 11. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test local_executor before_skill_hook`: passed.
- `cargo test -p workflow-core --test agent_harness_hook_contract hook_kinds_are_model_vocabulary_only`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 12. Remaining Known Limitations

- Only one explicit `BeforeSkillInvocation` hook input is supported.
- Warning, skipped, failed-closed event payloads, blocked semantics, and escalation behavior remain deferred.
- `BeforeReport` remains report-path-only and in-memory-only.
- Dedicated hook audit sink/store semantics remain deferred.
- Hook observability metrics remain deferred.
- Workflow-declared hook configuration remains deferred.
- Runtime hook configuration remains deferred.
- WorkReport hook event citation targets remain deferred.

## 13. Recommended Next Phase

Recommended next phase: **executor BeforeSkillInvocation hook event append review**.

The review should verify ordering, failure semantics, replay/idempotency, audit projection, privacy, and scope boundaries before expanding hook checkpoints or status semantics.
