# Executor Hook Event Append Plan

Status: Planning complete. This plan follows the accepted hook workflow event vocabulary and accepted generic hook workflow event audit projection. It defines how a future executor implementation may append hook workflow events for a narrowly selected pre-terminal checkpoint. It does not implement executor event append behavior, automatic hook broadening, local check execution, command execution, adapter invocation, dedicated hook audit sink emission, hook persistence, observability metrics, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS now has:

- validated agent harness hook contracts;
- explicit in-memory hook invocation helpers;
- model-only hook audit records;
- model-only hook workflow event vocabulary;
- generic hook workflow event audit projection;
- an explicit non-mutating `BeforeReport` report-path hook.

Runtime paths still do not append hook workflow events. The next question is where an executor path should first make hook execution state-visible.

This plan recommends a future implementation that appends hook workflow events only for one pre-terminal, explicitly supplied checkpoint after policy has already allowed the scoped action and before the scoped action is performed. The first target should not be the existing post-terminal `BeforeReport` hook, because current terminal-state rules intentionally reject post-terminal workflow events.

This plan does not implement the append path.

## 2. Goals

- Define the first safe executor event append boundary for hook workflow events.
- Preserve `WorkflowRunEvent` as the runtime source of truth.
- Preserve `WorkflowRunSnapshot` replay determinism.
- Reuse existing hook workflow event vocabulary.
- Reuse existing generic audit projection.
- Preserve existing `execute(...)` and `execute_with_report(...)` semantics.
- Keep hook execution explicit, bounded, and non-ambient.
- Avoid raw payload copying.
- Avoid fake evidence, approvals, policy decisions, local check results, WorkReports, or side effects.
- Prepare a small implementation prompt with one selected checkpoint and focused tests.

## 3. Non-Goals

Do not implement in the future append phase:

- broad automatic hook invocation;
- workflow-declared hook configuration;
- runtime hook configuration;
- post-terminal workflow events;
- conversion of `BeforeReport` into a workflow event;
- dedicated hook audit sink emission;
- hook audit store or persistence;
- hook observability metrics;
- WorkReport hook event citation targets;
- CLI hook commands or rendering;
- workflow schema fields;
- automatic local check execution;
- default local check handler registration;
- command execution;
- adapter invocation;
- external provider calls;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- report artifact writes;
- reasoning lineage;
- side-effect boundary implementation;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Current executor behavior:

- `LocalExecutor::execute(...)` appends runtime events and returns a `WorkflowRun`.
- `LocalExecutor::execute_with_report(...)` wraps execution and may generate an in-memory report.
- The explicit `BeforeReport` hook runs only on the report-bearing path after a terminal run exists.
- The `BeforeReport` hook returns an in-memory invocation result and model-only hook audit record.
- Successful `BeforeReport` hook IDs may be cited by WorkReports.
- `BeforeReport` hook failure becomes `report_generation_error` and does not change the run.

Current hook event baseline:

- `HookInvocationRequested` and `HookInvocationEvaluated` exist as model-only workflow event vocabulary.
- Hook workflow events are state-preserving only from `Running`.
- Hook workflow events require idempotency keys.
- Hook workflow events are rejected before `Running` and after terminal states.
- Generic `AuditEvent` projection for hook workflow events is implemented.
- No executor path appends hook workflow events.

## 5. First Append Target Recommendation

Recommended first target: **one explicit pre-terminal `BeforeSkillInvocation` hook append path**, behind an explicit executor input supplied by the caller.

Rationale:

- It occurs while the run is `Running`, matching existing hook event transition rules.
- It has a concrete ordering point before a governed side-effect boundary.
- It can use the existing append pipeline, which already writes workflow events and emits generic audit projection.
- It can remain explicit and non-default.
- It avoids changing terminal-state behavior.

The existing `BeforeReport` hook should remain in-memory-only in the next implementation. Converting it into a workflow event would require a separate post-terminal event model and replay policy.

## 6. Required Ordering

For the first `BeforeSkillInvocation` append path, event ordering should be:

1. `StepScheduled`
2. policy decision for skill invocation
3. `HookInvocationRequested`
4. hook invocation helper executes from explicit inputs
5. `HookInvocationEvaluated`
6. if hook passes, `SkillInvocationRequested`
7. `SkillInvocationStarted`
8. skill success/failure events

The hook must not run before policy permits the skill invocation path.

The hook must not replace policy evaluation.

The hook must not append events after the skill invocation starts.

## 7. Explicit Input Boundary

The first implementation should accept explicit hook input from the caller. It should not read hidden global state or invent runtime configuration.

Candidate future input:

- optional `before_skill_invocation_hook` on `LocalExecutionRequest`, or a similarly named explicit executor-adjacent request field;
- hook invocation ID;
- hook invocation input;
- expected step ID;
- expected skill ID and skill version where available;
- stable input references only;
- stable output requirements only if the hook contract requires them;
- redaction metadata;
- sensitivity.

The input must match the current run identity:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- hook kind;
- step ID where applicable.

Identity mismatch must fail with a stable, non-leaking error.

## 8. Hook Status Semantics

The first event append implementation should support only the safest statuses:

- `Passed`: append requested/evaluated events and continue.
- `Warning`: append requested/evaluated events and continue only if the hook contract explicitly allows warning continuation and the warning is bounded/reportable.
- `SkippedWithDisclosure`: append requested/evaluated events and continue only if the skip is explicit and bounded.
- `FailedClosed`: append requested/evaluated events and fail or block the scoped skill invocation using an existing reviewed terminal/failure path.
- `Blocked`: defer until blocked/escalated runtime semantics are separately accepted.

Recommended first implementation subset: support `Passed` only, and reject or fail closed for all other statuses until failure/blocking semantics are separately reviewed.

## 9. Failure Behavior

Conservative future behavior:

- Hook construction failure before append: return a structured executor error and do not append partial hook events.
- `HookInvocationRequested` append succeeds but hook evaluation fails: append a bounded `HookInvocationEvaluated` event with `FailedClosed` only if the hook result can be constructed safely; otherwise fail the workflow with a stable internal hook failure code.
- Hook failed-closed before skill invocation: do not append `SkillInvocationRequested`.
- Hook failure must not become a misleading user project diagnostic.
- Hook failure must not create fake approvals, policy decisions, evidence, local check results, or WorkReports.
- Errors must use stable codes and must not leak hook IDs, raw context, paths, payloads, command output, provider output, parser output, tokens, or secret-like values.

## 10. Idempotency And Replay

The first implementation must define deterministic idempotency keys:

- hook requested key derived from the skill invocation idempotency key plus hook kind and checkpoint;
- hook evaluated key derived from the requested key plus evaluation marker.

Duplicate run behavior:

- if the requested/evaluated hook events already exist for the checkpoint, do not re-run the hook silently;
- rehydrate and continue from durable event state;
- do not append duplicate hook events.

Replay behavior:

- `WorkflowRunSnapshot` remains state-preserving while hook events replay from `Running`;
- hook events must not mutate current step outputs, approvals, retries, or reports by themselves.

## 11. Audit And Observability Behavior

Audit behavior:

- hook workflow events appended through the normal executor append path should project through `AuditEvent::from_workflow_event(...)`;
- no dedicated hook audit sink method should be called;
- no `AgentHarnessHookAuditRecord` should be emitted or persisted in the first append implementation.

Observability behavior:

- no hook observability metrics in the first append implementation;
- hook metrics require separate planning after hook execution semantics are proven.

## 12. Privacy And Redaction

Future appended hook events must not store:

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

Hook workflow event payloads should remain bounded and reference-first. Audit projection should continue to expose only status vocabulary and reference counts.

## 13. Relationship To BeforeReport

The existing `BeforeReport` hook remains:

- report-path-only;
- in-memory-only;
- non-mutating;
- not represented as workflow events;
- not emitted through the audit sink except through existing report/result behavior.

Reason: current terminal states reject post-terminal events, and changing that invariant would require a separate runtime event model decision.

## 14. Relationship To Policy, Approval, Local Checks, And Adapters

Hooks do not replace policy gates or approvals.

The future `BeforeSkillInvocation` append path must run only after policy allows the skill invocation path. It must not request, grant, deny, or infer approvals.

Hooks must not execute local checks, commands, adapters, or providers. They may reference stable IDs that already exist, but must not create them.

## 15. Test Plan For Future Implementation

Future implementation tests should cover:

- explicit `BeforeSkillInvocation` hook input appends `HookInvocationRequested`;
- explicit `BeforeSkillInvocation` hook input appends `HookInvocationEvaluated`;
- event ordering is policy decision, hook requested, hook evaluated, skill invocation requested;
- hook events project to generic audit events;
- no dedicated hook audit records are emitted;
- no hook observability metrics are emitted;
- successful hook does not change skill behavior except ordering;
- hook failure before append produces no partial hook events;
- failed-closed hook prevents skill invocation;
- duplicate run does not re-run or duplicate hook events;
- rehydration preserves run state and hook event ordering;
- `execute(...)` without explicit hook input remains unchanged;
- `execute_with_report(...)` `BeforeReport` remains in-memory-only and does not append hook workflow events;
- non-terminal/terminal state rules remain unchanged;
- identity mismatch fails without leaking IDs or payloads;
- raw provider/spec/command/parser payload markers are not copied;
- hook IDs, contract IDs, and phase IDs do not leak through generic audit projection;
- existing audit projection tests still pass;
- existing runtime, executor, WorkReport, EvidenceReference, adapter, local check, and CLI tests still pass.

## 16. Proposed Implementation Sequence

1. Add an explicit executor input for one `BeforeSkillInvocation` hook checkpoint.
2. Validate hook input against run, workflow, step, skill, schema, and spec identity.
3. Append `HookInvocationRequested` before skill invocation.
4. Execute existing in-memory hook helper.
5. Append `HookInvocationEvaluated` for supported `Passed` results.
6. Continue to `SkillInvocationRequested` only after successful hook evaluation.
7. Fail closed for unsupported statuses.
8. Add focused executor/replay/audit tests.
9. Review before supporting warning, skipped, blocked, or post-terminal hook events.

## 17. Deferred Work

- `BeforeReport` workflow event append behavior;
- post-terminal workflow event model;
- multiple hook checkpoints;
- workflow-declared hook configuration;
- runtime hook configuration;
- warning/skipped/blocked continuation policy;
- dedicated hook audit sink/store;
- hook observability metrics;
- hook WorkReport event citations;
- automatic local check execution;
- command execution;
- adapter invocation;
- approvals;
- EvidenceReference creation/attachment;
- report artifacts;
- CLI rendering;
- schema changes;
- side-effect boundary implementation;
- writes;
- reasoning lineage.

## 18. Open Questions

- Should the first `BeforeSkillInvocation` hook input live on `LocalExecutionRequest` or a new report-independent executor wrapper?
- Should a failed-closed hook fail the whole run immediately or schedule an escalation when escalation policy exists?
- Should warning continuation be allowed before WorkReport disclosure requirements are stronger?
- Should hook requested/evaluated event IDs be citably stable before WorkReport hook event citation targets are added?
- Should hook invocation IDs be deterministic per step attempt or caller-supplied only?
- How should hook events interact with retries: once per original step or once per retry attempt?
- Should hook events be emitted for skipped optional hooks, or should absence remain absence?
- When should `AgentHarnessHookAuditRecord` become durable, if ever?

## 19. Final Recommendation

Recommended next implementation phase: **executor `BeforeSkillInvocation` hook event append, explicit input only**.

That implementation should support the `Passed` status first, append requested/evaluated hook workflow events through the existing executor append pipeline, rely on generic audit projection, and preserve all existing executor behavior when no explicit hook input is supplied.

It must not implement automatic hook invocation, post-terminal hook events, dedicated hook audit sinks, persistence, observability metrics, local checks, commands, adapters, approvals, EvidenceReference attachment, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.
