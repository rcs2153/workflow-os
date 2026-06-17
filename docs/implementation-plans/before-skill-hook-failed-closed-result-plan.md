# BeforeSkillInvocation Failed-Closed Result Path Plan

Status: Implementation complete for the first narrow `BeforeSkillInvocation` failed-closed result path. This plan follows the accepted explicit `BeforeSkillInvocation` executor hook event append path and the accepted boundary hardening review. The implementation adds an explicit failed-closed in-memory hook result helper and executor handling for caller-supplied `FailedClosed` status. It does not implement automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, post-terminal workflow events, dedicated hook audit sink emission, hook persistence, observability metrics, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

The explicit `BeforeSkillInvocation` checkpoint is implemented for caller-supplied hook input and has boundary coverage for later-step targeting, missing handlers, policy denial, replay, and redaction.

The next narrow design question is how a future failed-closed hook result should be represented without creating partial events, fake evidence, misleading diagnostics, hidden side effects, or ambiguous replay behavior.

This plan recommends a conservative future implementation:

- keep `Passed` as the only continuing status;
- add a safe, explicit way to construct a validated `FailedClosed` hook invocation result from bounded inputs;
- append `HookInvocationRequested` and `HookInvocationEvaluated(FailedClosed)` together only after the failed-closed result can be constructed safely;
- fail the scoped run before `SkillInvocationRequested`;
- do not introduce warning, skipped, or blocked continuation.

The first narrow implementation now exists. It keeps `Passed` as the only continuing status and treats explicit `FailedClosed` as a pre-skill run failure after requested/evaluated hook events are safely constructed and appended.

## 2. Goals

- Define the first safe failed-closed hook result path.
- Preserve deterministic workflow execution.
- Preserve policy-before-hook and policy-before-side-effect ordering.
- Preserve existing `Passed` hook behavior.
- Preserve existing no-hook executor behavior.
- Prevent partial hook events.
- Prevent fake evidence, fake approvals, fake policy decisions, fake local check results, and fake WorkReports.
- Ensure failed-closed hook outcomes prevent the scoped skill invocation.
- Use stable, non-leaking error codes.
- Keep event replay deterministic and idempotent.
- Prepare a small future implementation prompt.

## 3. Non-Goals

Do not implement beyond this phase:

- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- automatic hook invocation;
- broad executor hook checkpoints;
- workflow-declared hook configuration;
- runtime hook configuration;
- post-terminal workflow events;
- conversion of `BeforeReport` into workflow events;
- dedicated hook audit sink emission;
- hook persistence;
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

- `LocalExecutionRequest` can carry one explicit `BeforeSkillInvocation` hook input.
- The executor considers that input only when the active step ID matches.
- Policy is evaluated before hook append.
- Local handler lookup happens before hook append.
- The existing runtime hook helper returns `Passed` for valid, side-effect-free inputs.
- Unsafe hook helper input fails closed as an error before hook events are appended.
- Unsupported statuses fail closed if encountered.
- `HookInvocationRequested` and `HookInvocationEvaluated` are appended only for passed hook results today.

Current vocabulary already includes:

- `Passed`;
- `FailedClosed`;
- `Warning`;
- `SkippedWithDisclosure`;
- `Blocked`.

Only `Passed` is executable today.

## 5. Recommended Failed-Closed Model

Recommended model: safe result before append.

Future implementation should construct a validated `AgentHarnessHookInvocationResult` with status `FailedClosed` before appending any hook workflow event. Only after that result is safe should the executor append:

1. `HookInvocationRequested`;
2. `HookInvocationEvaluated` with `FailedClosed`;
3. `RunFailed` with a stable hook failure code.

The executor must not append `SkillInvocationRequested` for that step.

Recommended stable runtime failure code:

```text
executor.hook.before_skill_invocation.failed_closed
```

The failure message should be stable and generic:

```text
before-skill-invocation hook failed closed before skill invocation
```

The message must not include hook IDs, reference names, evidence IDs, source paths, payloads, command output, provider output, parser output, tokens, disclosures, or secret-like values.

## 6. Why Requested And Evaluated Should Append Together

The current implementation evaluates the side-effect-free helper before appending hook events. That posture avoids partial hook events when hook construction fails.

For failed-closed support, keep this posture:

- prevalidate hook identity;
- construct a safe failed-closed invocation result;
- construct bounded workflow event payloads;
- append requested/evaluated events together;
- fail the run before skill invocation.

Do not append `HookInvocationRequested` before the failed-closed result can be safely constructed. A requested-without-evaluated state would require its own recovery and replay policy, which is outside this phase.

## 7. Failed-Closed Result Construction

Future implementation should add the smallest explicit helper or constructor path needed to create a failed-closed result.

Possible shape:

```text
AgentHarnessHookInvocationResult::failed_closed(...)
```

or a narrow helper:

```text
failed_closed_agent_harness_hook_result(input, disclosure)
```

The helper must:

- accept explicit input only;
- reuse existing validation where possible;
- require hook kind and contract identity to match;
- require workflow/run/schema/spec identity;
- require target step identity for `BeforeSkillInvocation`;
- reject side-effect requests;
- reject secret-like disclosures;
- reject unbounded disclosures;
- preserve reference-only input and output metadata;
- not create evidence references;
- not create local check results;
- not create approval decisions;
- not call adapters;
- not run commands;
- return `Result<AgentHarnessHookInvocationResult, WorkflowOsError>`.

## 8. Disclosure Policy

Failed-closed results may need a bounded disclosure, but the first implementation should keep it minimal.

Recommended initial policy:

- require either no disclosure or one bounded generic disclosure supplied by the caller;
- disclosure text must be validated and redaction-safe;
- disclosure text must not be copied from raw provider output, command output, parser output, diagnostic messages, spec contents, environment values, or secrets;
- failed-closed executor errors must not include the disclosure text.

If disclosure semantics are unclear, the first implementation should allow `FailedClosed` with no disclosure and rely on stable event status plus stable runtime failure code. Rich disclosure can follow after WorkReport disclosure semantics are planned.

## 9. Event Semantics

Future failed-closed executor behavior should be:

1. Policy allows the scoped action.
2. Handler lookup succeeds.
3. Hook input matches the active step.
4. Hook identity validates.
5. Failed-closed result validates.
6. `HookInvocationRequested` is appended.
7. `HookInvocationEvaluated(FailedClosed)` is appended.
8. `RunFailed` is appended.
9. No `SkillInvocationRequested` event is appended for the scoped step.

If any validation before step 6 fails:

- append no hook events;
- append `RunFailed` using the stable validation error code;
- do not append `SkillInvocationRequested`.

If appending requested succeeds but appending evaluated fails, the executor would have a partial event problem. The first implementation should avoid this by constructing both event payloads before appending either. If append failure still occurs due to backend write failure, existing backend error behavior applies and must not be hidden.

## 10. Idempotency And Replay

Failed-closed hook events should reuse the existing deterministic hook idempotency-key pattern:

- requested key derived from the skill invocation key;
- evaluated key derived from the skill invocation key;
- run failure remains the terminal event.

Duplicate execution with the same run ID should rehydrate the existing failed run and must not append duplicate hook events.

Failed-closed hook failure should not consume a skill retry attempt because `SkillInvocationRequested` and skill attempts never occur.

Retry-aware hook execution remains deferred.

## 11. Policy And Approval Relationship

Failed-closed hooks must remain downstream of policy for the scoped action and must not replace policy or approval gates.

Rules:

- policy denial still occurs before hook append;
- failed-closed hooks do not create policy decisions;
- failed-closed hooks do not request approvals;
- failed-closed hooks do not grant or deny approvals;
- failed-closed hooks do not escalate to approval unless a separate approval/escalation model is accepted later;
- failed-closed hooks do not bypass existing approval gates.

## 12. Audit And Report Relationship

Generic audit projection of `HookInvocationRequested`, `HookInvocationEvaluated`, and `RunFailed` is enough for the first failed-closed implementation.

Do not add:

- dedicated hook audit sink emission;
- hook audit persistence;
- WorkReport hook event citation targets;
- automatic report generation;
- report artifacts;
- CLI rendering.

Future WorkReport disclosure can be planned after failed-closed runtime semantics are reviewed.

## 13. Privacy And Redaction

Failed-closed support must not store or emit:

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

Errors must use stable codes and generic messages. Debug output must remain redaction-safe.

## 14. Implemented Test Coverage

The failed-closed implementation added focused tests covering:

- passed hook behavior remains unchanged;
- failed-closed hook appends requested/evaluated events before `RunFailed`;
- failed-closed evaluated payload has status `FailedClosed`;
- failed-closed prevents `SkillInvocationRequested`;
- failed-closed does not consume skill retry attempts;
- failed-closed does not invoke the skill handler;
- failed-closed does not create evidence;
- failed-closed does not create WorkReports or report artifacts;
- failed-closed duplicate run does not duplicate hook events;
- failed-closed errors do not leak hook IDs, references, disclosures, paths, payloads, or secret-like values;
- existing runtime, executor, audit, WorkReport, EvidenceReference, adapter, local check, and CLI tests still pass.

The earlier boundary hardening tests continue to cover no-hook behavior, policy denial, missing handlers, later-step targeting, duplicate run behavior, and debug redaction for the explicit hook input path.

## 15. Completed Implementation Sequence

1. Added a narrow failed-closed result construction helper.
2. Added model tests for valid and invalid failed-closed result construction.
3. Added executor support only for explicit `BeforeSkillInvocation` failed-closed results.
4. Constructed requested/evaluated event payloads before appending either event.
5. Failed the run with `executor.hook.before_skill_invocation.failed_closed`.
6. Added focused executor tests for failed-closed event order, no skill invocation, replay, no report artifacts, and unsupported warning status behavior.
7. Created an implementation report for phase review.
8. Kept warning/skipped disclosure semantics deferred.

## 16. Deferred Work

- Warning continuation.
- Skipped-with-disclosure continuation.
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

- Should failed-closed require a bounded disclosure in v1, or should status plus stable failure code be enough?
- Should failed-closed results be constructed by a generic invocation-result constructor or a dedicated helper?
- Should failed-closed eventually support escalation instead of immediate run failure?
- Should hook invocation IDs remain caller-supplied or become deterministic per step attempt?
- Should hook event IDs become WorkReport citation targets after failed-closed behavior is implemented?
- Should dedicated hook audit records remain model-only or become emitted after failed-closed behavior exists?

## 18. Final Recommendation

Recommended next phase at implementation close: **BeforeSkillInvocation failed-closed result path review**.

The review should verify that the implementation remains narrow: it constructs a safe failed-closed result, appends requested/evaluated hook events only after validation, fails the run before `SkillInvocationRequested`, and keeps warning, skipped, blocked, automatic configuration, persistence, CLI, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, hosted behavior, and release posture changes out of scope.

Fix-forward note: that review is complete, and warning/skipped disclosure semantics planning is now documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](before-skill-hook-warning-skipped-disclosure-plan.md).
