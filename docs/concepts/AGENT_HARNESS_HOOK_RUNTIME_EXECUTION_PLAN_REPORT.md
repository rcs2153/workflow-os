# Agent Harness Hook Runtime Execution Plan Report

Report date: 2026-06-16

## 1. Executive Summary

Created the Agent Harness Hook Runtime Execution Plan. The plan defines a conservative future implementation boundary for executing deterministic hook checkpoints through an explicit in-memory runtime helper.

The recommended first implementation should consume explicit inputs, call the existing `invoke_agent_harness_hook(...)` validation helper, build an in-memory `AgentHarnessHookAuditRecord`, and return a structured result. It must not automatically integrate with `LocalExecutor`, append workflow events, emit audit sink records, persist hook records, run local checks, invoke adapters, execute commands, write files, expose CLI behavior, add schema fields, model side effects, add writes, introduce recursive agents, introduce agent swarms, or change release posture.

## 2. Scope Completed

- Defined the runtime hook execution boundary.
- Recommended an explicit in-memory runtime service/helper as the first implementation.
- Defined required explicit input context.
- Defined stable hook invocation identity posture.
- Defined in-memory output/result expectations.
- Deferred automatic executor checkpoint integration.
- Defined candidate future executor checkpoints without authorizing them.
- Defined failure semantics.
- Defined audit/event posture.
- Defined policy, approval, local check, evidence, WorkReport, privacy, and redaction boundaries.
- Defined future tests and implementation sequence.

## 3. Scope Explicitly Not Completed

- No implementation.
- No runtime hook execution code.
- No executor hook invocation.
- No workflow event kinds.
- No workflow event append behavior.
- No audit sink emission.
- No hook persistence.
- No workflow schema fields.
- No CLI hook commands.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No `EvidenceReference` creation or attachment.
- No approval request or decision creation.
- No reasoning lineage implementation.
- No side-effect boundary implementation.
- No writes.
- No recursive agents.
- No agent swarms.
- No release posture change.

## 4. Planning Summary

The plan recommends a future explicit helper that:

1. Accepts a hook invocation ID or explicit deterministic ID seed.
2. Accepts a hook contract and explicit runtime context.
3. Validates the invocation through `invoke_agent_harness_hook(...)`.
4. Builds an in-memory `AgentHarnessHookAuditRecord`.
5. Returns the invocation ID, invocation result, and audit record.

Automatic executor checkpoints remain deferred until this helper is implemented and reviewed.

## 5. Validation Boundary Summary

The plan keeps hook execution fail-closed at the helper boundary for invalid contracts, hook kind mismatch, missing required inputs or outputs, invalid references, duplicate references, secret-like values, invalid redaction metadata, and side-effect requests.

Those failures should return structured non-leaking errors to the caller. They should not automatically fail, pause, cancel, or mutate workflow runs until executor checkpoint semantics are separately accepted.

## 6. Redaction And Privacy Summary

The plan forbids storing or copying raw prompts, raw spec contents, raw command output, raw provider payloads, raw CI logs, raw Jira/GitHub bodies, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded summaries, or hook execution transcripts.

Debug, serialization, and errors must remain bounded and redaction-safe.

## 7. Test Coverage Plan Summary

Future tests should prove:

- valid runtime hook execution returns in-memory result and audit record;
- caller-supplied invocation ID is preserved;
- hook kind mismatch and missing required context fail closed;
- side-effect requests are rejected;
- secret-like values do not leak;
- no workflow state mutation occurs;
- no workflow events are appended;
- no audit sink records are emitted;
- no `StateBackend` writes occur;
- no local checks, adapters, commands, files, CLI output, or raw payload copies occur;
- existing hook, WorkReport, executor, evidence, diagnostic, validation, adapter telemetry, runtime, and docs tests still pass.

## 8. Commands Run And Results

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 9. Remaining Known Limitations

- Runtime hook execution is not implemented.
- Executor hook checkpoints are not implemented.
- Hook workflow events are not implemented.
- Hook audit sink emission is not implemented.
- Hook records are not persisted.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.

## 10. Recommended Next Phase

Recommended next phase: **runtime hook execution service/helper implementation, in-memory only**.

That phase should implement only the explicit runtime helper boundary described by the plan. Executor automatic hook checkpoints, events, audit sink emission, persistence, CLI, schemas, side effects, writes, recursive agents, agent swarms, and release posture changes must remain separate future phases.
