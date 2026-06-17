# BeforeSkillInvocation Hook Status And Failure Semantics Plan

Status: Planning complete; first boundary hardening tests implemented; first narrow failed-closed result path implemented; unsupported-status hardening tests implemented. This plan follows the accepted explicit `BeforeSkillInvocation` executor hook event append path and defines how future hook statuses and failure semantics should be handled before any runtime broadening. It does not implement automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, post-terminal workflow events, dedicated hook audit sink emission, hook persistence, observability metrics, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS now has the first state-visible hook checkpoint:

- `LocalExecutionRequest` can accept one explicit `BeforeSkillInvocation` hook input.
- `LocalExecutor::execute(...)` can append bounded `HookInvocationRequested` and `HookInvocationEvaluated` events before `SkillInvocationRequested`.
- The implemented executor path supports passed hook results only.
- Invalid hook construction or execution fails closed before skill invocation and appends no partial hook events.

The next question is how non-passed hook statuses should behave. This plan keeps the implementation conservative: `Passed` remains the only status that may continue execution until warning, skipped, failed-closed, blocked, retry, escalation, report disclosure, and audit semantics are separately implemented and reviewed.

This plan does not implement status broadening.

## 2. Goals

- Define future semantics for hook statuses before runtime broadening.
- Preserve deterministic workflow execution.
- Preserve policy-before-side-effect ordering.
- Preserve event-log replay correctness.
- Preserve existing `execute(...)` behavior when no explicit hook input is supplied.
- Avoid hidden hook state.
- Avoid fake evidence, fake approvals, fake policy decisions, fake local check results, fake WorkReports, and fake side effects.
- Ensure non-passed statuses do not silently continue without durable disclosure.
- Define failure behavior that is stable, non-leaking, and auditable.
- Prepare a small future implementation sequence.

## 3. Non-Goals

Do not implement in this phase:

- runtime code changes;
- automatic hook invocation;
- broad executor hook checkpoints;
- workflow-declared hook configuration;
- runtime hook configuration;
- post-terminal workflow events;
- conversion of `BeforeReport` into workflow events;
- dedicated hook audit sink emission;
- hook audit store or persistence;
- hook observability metrics;
- WorkReport hook event citation targets;
- CLI hook commands or rendering;
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
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Current implemented behavior:

- `BeforeSkillInvocation` is explicit caller-supplied input only.
- The executor validates the hook input against workflow, run, schema, spec hash, step, skill ID, and skill version.
- The executor invokes the existing in-memory hook helper.
- If the helper returns `Passed`, the executor appends `HookInvocationRequested`, appends `HookInvocationEvaluated`, then continues to `SkillInvocationRequested`.
- If hook input validation or helper execution fails, the run fails before hook events or skill invocation events are appended.
- Unsupported statuses fail closed.
- Duplicate run execution rehydrates existing events and does not duplicate hook events.
- Generic audit projection records hook workflow events.
- No dedicated hook audit records are emitted through audit sinks.

Current modeled status vocabulary:

- `Passed`
- `Warning`
- `FailedClosed`
- `SkippedWithDisclosure`
- `Blocked`

Only `Passed` is executable in the current local executor append path.

## 5. Status Semantics Principles

Future status support must follow these principles:

- Status semantics must be deterministic and replayable.
- Status semantics must be visible in workflow events if they affect execution.
- Status semantics must not be hidden in logs, reports, or in-memory helper results only.
- Non-passed statuses must never silently continue without durable disclosure.
- Hooks must not replace policy gates or approvals.
- Hooks must not infer user approval.
- Hooks must not create evidence references.
- Hooks must not execute local checks, commands, adapters, or providers.
- Hook failure must not become a misleading user project diagnostic.
- Hook errors must use stable codes and avoid raw payloads, paths, IDs, tokens, snippets, and secret-like values.

## 6. Status-by-Status Policy

### Passed

Policy: continue execution.

Allowed behavior:

- append `HookInvocationRequested`;
- append `HookInvocationEvaluated` with `Passed`;
- continue to `SkillInvocationRequested`.

This is the only status implemented today.

### Warning

Policy: defer runtime continuation support.

Future warning continuation may be allowed only when all of the following exist:

- warning text is bounded and redaction-safe;
- warning is durably recorded;
- report disclosure requirements are defined;
- policy can decide whether warning continuation is allowed;
- tests prove warning does not bypass approval, validation, local checks, or policy gates.

Until then, a warning status should fail closed or be rejected rather than silently continue.

### SkippedWithDisclosure

Policy: defer runtime continuation support.

Future skip continuation may be safe only for optional hooks. It requires:

- an explicit hook optionality model;
- a bounded disclosure reason;
- durable event representation;
- report disclosure behavior;
- policy for when skip is allowed;
- tests proving the skip does not fabricate a passed hook.

Until optional hook semantics exist, skipped-with-disclosure should fail closed or be rejected.

### FailedClosed

Policy: block the scoped skill invocation.

Future failed-closed support should append a durable evaluated hook event with `FailedClosed` only when the failure status itself can be constructed safely without leaking raw context. After that event, the executor should append the reviewed failure or escalation path.

Open design choice:

- fail the whole run immediately with `RunFailed`; or
- escalate/pause only if a separately accepted escalation or blocked-status model exists.

Recommended initial implementation after planning: fail the run before `SkillInvocationRequested` using a stable hook failure code. Escalation should remain deferred until escalation semantics are explicitly planned.

### Blocked

Policy: defer runtime support.

`Blocked` implies a new operational state or a mapping to existing failure/escalation behavior. Workflow OS should not invent a blocked runtime status inside the hook path.

Before `Blocked` can be implemented, the runtime needs a separate accepted status model or a policy-approved mapping to existing `Failed`, `WaitingForApproval`, or escalation behavior.

## 7. Requested/Evaluated Event Semantics

The current implementation prevalidates and executes the side-effect-free helper before appending hook events. This prevents partial hook events when invalid input fails before a durable hook result exists.

That posture remains acceptable for the current `Passed`-only implementation.

For future non-passed statuses, the executor must choose one of two explicit models.

### Option A: Safe Result Before Append

Evaluate the side-effect-free hook helper first. If it returns a safe structured result, append requested/evaluated together. If construction fails, append no hook events and fail with a stable executor error.

Benefits:

- no partial hook events;
- simple replay behavior;
- matches current implementation.

Risks:

- `HookInvocationRequested` is not a literal pre-execution event;
- failed construction is represented as run failure, not hook evaluated failure.

### Option B: Requested Before Evaluation

Append `HookInvocationRequested`, run the helper, then append `HookInvocationEvaluated`.

Benefits:

- event names match execution lifecycle more literally;
- failed-closed hook evaluation can be durably visible.

Risks:

- partial requested events can exist if helper evaluation cannot produce a safe evaluated event;
- requires a durable policy for requested-without-evaluated states;
- complicates replay and duplicate behavior.

Recommended posture: keep Option A until a safe, bounded `FailedClosed` invocation result can always be constructed for helper-level failures. Do not implement Option B without a separate requested-without-evaluated recovery policy.

## 8. Failure Mapping Policy

Future hook failures must map to stable runtime behavior:

| Failure source | Recommended behavior |
| --- | --- |
| identity mismatch | fail run before hook events |
| invalid hook kind | fail run before hook events |
| skill mismatch | fail run before hook events |
| invalid redaction metadata | fail run before hook events |
| secret-like hook context | fail run before hook events |
| helper side-effect request | fail run before hook events today; future may emit `FailedClosed` only if safe result construction exists |
| unsupported status | fail run before skill invocation |
| warning without disclosure policy | fail closed |
| skipped without optionality policy | fail closed |
| blocked without status model | fail closed or defer implementation |

Errors must not leak raw hook IDs, input references, output references, phase IDs, source paths, payloads, command output, provider output, parser output, tokens, or secret-like values.

## 9. Retry And Multi-Step Policy

Hook status behavior must be explicit across retries and multiple steps.

Recommended defaults:

- `BeforeSkillInvocation` should execute once per step attempt only if retry-aware hook semantics are explicitly implemented.
- Current implementation effectively runs before the skill invocation request for the step, before attempts.
- Do not move hooks inside the retry loop without planning.
- A failed hook should prevent the scoped skill invocation and should not consume a skill retry attempt.
- Later-step hooks should be matched by target step identity and ignored for other steps until the matching step is active.
- Duplicate runs should not re-run hooks or duplicate events.

Before retry-aware hook execution is implemented, add tests proving later-step targeting and missing-handler behavior.

## 10. Policy And Approval Relationship

Hooks must remain downstream of policy decisions for the scoped action.

Rules:

- policy must still run before the hook;
- hooks must not create policy decisions;
- hooks must not request approvals;
- hooks must not grant or deny approvals;
- hooks must not bypass approval gates;
- warning/skipped continuation must eventually be policy-controlled if it affects execution;
- failed-closed behavior must not be converted into approval without a separately accepted approval model.

## 11. Audit And Report Disclosure

Generic audit projection is sufficient for the current `Passed` status.

Before non-passed continuation is supported, the project must define:

- whether warning/skipped statuses become report disclosures;
- whether WorkReport can cite hook event IDs or should continue citing hook invocation IDs;
- whether failed-closed hook outcomes need dedicated hook audit records;
- whether `AgentHarnessHookAuditRecord` should remain model-only, be emitted through a sink, or be persisted;
- how audit and report summaries remain bounded and redaction-safe.

Do not add dedicated hook audit sink emission or hook persistence in the first status broadening implementation.

## 12. Privacy And Redaction

Future status support must not store or emit:

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

Warning and skip disclosures must be bounded, validated, redaction-aware, and non-secret-like before they can become durable event or report content.

## 13. Recommended First Follow-Up Implementation

Do not implement warning, skipped, failed-closed, or blocked continuation yet.

Implemented first code-bearing follow-up: **BeforeSkillInvocation boundary hardening tests**.

That implementation added focused tests for:

- hook input targeting a later step in a multi-step workflow;
- hook input targeting a different current step is ignored until matching step;
- missing local skill handler appends no hook events;
- no hook events are appended on policy denial;
- unsupported status remains fail-closed if a test-only helper path can model it safely;
- request debug output remains redaction-safe.

This hardening step did not change runtime semantics. The implementation report is [BeforeSkillInvocation Hook Boundary Hardening Report](../concepts/BEFORE_SKILL_HOOK_BOUNDARY_HARDENING_REPORT.md).

## 14. Future Implementation Sequence

1. Review BeforeSkillInvocation boundary hardening tests.
2. Define a safe structured `FailedClosed` hook result path, if needed.
3. Add event/report disclosure model for warnings and skips.
4. Review.
5. Consider `FailedClosed` event append support.
6. Review.
7. Consider warning/skipped continuation only after policy and report disclosure semantics exist.
8. Defer `Blocked` until runtime status or escalation semantics are accepted.

## 15. Test Plan

Future tests should cover:

- passed hook still continues execution;
- warning is rejected or fails closed until disclosure policy exists;
- skipped-with-disclosure is rejected or fails closed until optionality policy exists;
- failed-closed prevents `SkillInvocationRequested`;
- blocked is rejected or fails closed until status semantics exist;
- hook failure does not consume skill retry attempts;
- hook failure does not request or infer approval;
- hook failure does not create evidence;
- hook failure does not create local check results;
- hook failure does not create WorkReports or report artifacts;
- later-step targeting behaves deterministically;
- missing handler appends no hook events;
- policy denial appends no hook events;
- duplicate run does not duplicate hook events;
- errors do not leak hook IDs, references, paths, payloads, or secret-like values;
- existing runtime, executor, audit, WorkReport, EvidenceReference, adapter, local check, and CLI tests still pass.

## 16. Deferred Work

- Warning continuation.
- Skipped-with-disclosure continuation.
- Failed-closed evaluated event support.
- Blocked runtime status support.
- Retry-aware hook execution.
- Multiple hook checkpoints.
- Workflow-declared hook configuration.
- Runtime hook configuration.
- Dedicated hook audit sink/store.
- Hook observability metrics.
- WorkReport hook event citation targets.
- Post-terminal `BeforeReport` workflow event model.
- Automatic local check execution.
- Command execution.
- Adapter invocation.
- Approval evidence attachment.
- EvidenceReference creation/attachment.
- Report artifact auto-writing.
- CLI rendering.
- Schema changes.
- Side-effect boundary implementation.
- Writes.
- Reasoning lineage.

## 17. Open Questions

- Should failed-closed hook results become durable `HookInvocationEvaluated` events before `RunFailed`?
- Should warning/skipped statuses require WorkReport disclosure before continuation is allowed?
- Should warning/skipped continuation be policy-controlled per workflow, hook contract, or runtime request?
- Should hook execution occur once per step or once per retry attempt?
- Should hook invocation IDs remain caller-supplied or become deterministic per step attempt?
- Should a future blocked hook map to `WaitingForApproval`, `Failed`, escalation, or a new status?
- Should hook event IDs become WorkReport citation targets?
- Should `AgentHarnessHookAuditRecord` remain model-only or become a persisted/emitted audit surface?

## 18. Final Recommendation

Follow-on failed-closed planning is documented in [BeforeSkillInvocation Failed-Closed Result Path Plan](before-skill-hook-failed-closed-result-plan.md). The first narrow failed-closed implementation is complete and documented in [BeforeSkillInvocation Failed-Closed Result Path Implementation Report](../concepts/BEFORE_SKILL_HOOK_FAILED_CLOSED_RESULT_PATH_IMPLEMENTATION_REPORT.md).

Recommended next phase: **BeforeSkillInvocation unsupported status hardening review**.

Do not broaden runtime status behavior beyond passed continuation and failed-closed run failure semantics. Failed-closed review is complete, warning/skipped disclosure semantics planning is documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](before-skill-hook-warning-skipped-disclosure-plan.md), and unsupported-status hardening tests are documented in [BeforeSkillInvocation Unsupported Status Hardening Report](../concepts/BEFORE_SKILL_HOOK_UNSUPPORTED_STATUS_HARDENING_REPORT.md). The next phase should review that hardening without adding continuation. Warning, skipped, blocked, automatic configuration, persistence, CLI, schemas, local checks, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, hosted behavior, and release posture changes remain out of scope.
