# BeforeSkillInvocation Required Checkpoint Plan

Status: Planning complete, first explicit selected-step implementation slice complete, and required-step validation blocker fixed. This plan follows the accepted deterministic `BeforeReport` required-checkpoint enforcement review and the existing explicit `BeforeSkillInvocation` event append, failed-closed, unsupported-status, and disclosure-planning work. The first implementation adds opt-in local executor required `BeforeSkillInvocation` enforcement for explicitly selected step IDs only, as documented in [BeforeSkillInvocation Required Checkpoint Enforcement Report](../concepts/BEFORE_SKILL_REQUIRED_CHECKPOINT_ENFORCEMENT_REPORT.md), and the unknown required-step blocker is fixed in [BeforeSkillInvocation Required Checkpoint Blocker Fix Report](../concepts/BEFORE_SKILL_REQUIRED_CHECKPOINT_BLOCKER_FIX_REPORT.md). It does not add automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, schemas, CLI behavior, persistence, local check execution, command execution, adapter invocation, side effects, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 1. Executive Summary

Workflow OS now has two separate hook enforcement foundations:

- explicit `BeforeReport` report-path hooks can be required before report generation;
- explicit `BeforeSkillInvocation` hooks can append requested/evaluated workflow events before `SkillInvocationRequested`, continue only on `Passed`, and fail closed on explicit `FailedClosed`.

The next question is whether a local executor caller can require a `BeforeSkillInvocation` hook before selected local skill invocations. This is a higher-authority checkpoint than `BeforeReport` because it gates work before the skill handler runs. It must therefore stay explicit, deterministic, and narrowly targeted.

This plan recommends the smallest future implementation: an explicit request-level required-step policy for `BeforeSkillInvocation`, starting with one or more caller-specified step IDs. Missing, mismatched, unsafe, or unsupported required hook input should fail closed before `SkillInvocationRequested` with stable non-leaking errors. The plan does not authorize broad automatic hooks, schema-declared hooks, runtime config, warning/skipped continuation, local check execution, adapter invocation, side effects, or writes.

## 2. Goals

- Require `BeforeSkillInvocation` hooks only for explicitly selected local step IDs.
- Preserve existing no-hook executor behavior by default.
- Preserve policy-before-hook and policy-before-side-effect ordering.
- Preserve approval behavior.
- Preserve deterministic event ordering and replay.
- Prevent skill invocation when a required pre-skill hook is missing, mismatched, invalid, unsupported, or failed closed.
- Avoid fake hook events, fake evidence, fake approvals, fake local check results, fake side effects, and fake WorkReports.
- Use stable, non-leaking errors.
- Keep implementation small enough for one reviewable future phase.

## 3. Non-Goals

Do not implement in the next phase:

- automatic hook invocation for every executor path;
- workflow-declared hook configuration;
- runtime hook configuration;
- default hook registration;
- broad `AllSkillInvocations` policy;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- post-terminal workflow events;
- dedicated hook audit sink emission;
- hook persistence;
- hook observability metrics;
- WorkReport hook event citation targets beyond existing explicit supplied IDs;
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
- runtime side-effect execution;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Implementation Inventory

Current local executor behavior:

- `LocalExecutionRequest` can carry one explicit `LocalExecutionBeforeSkillInvocationHookInput`.
- The hook input targets one step ID, skill ID, and skill version.
- The executor ignores the hook input until the active step ID matches.
- Policy evaluation for the scoped skill invocation happens before side-effect events and before hook append.
- Local skill handler lookup happens before side-effect events and before hook append.
- Explicit SideEffect proposed/denied/skipped events are appended before the pre-skill hook path.
- `Passed` appends `HookInvocationRequested` and `HookInvocationEvaluated(Passed)`, then continues to `SkillInvocationRequested`.
- `FailedClosed` appends `HookInvocationRequested` and `HookInvocationEvaluated(FailedClosed)`, then fails the run before `SkillInvocationRequested`.
- `Warning`, `SkippedWithDisclosure`, and `Blocked` are unsupported and fail closed before hook or skill events are appended.
- Duplicate run execution rehydrates existing durable run state and does not duplicate hook events.
- Generic audit projection for hook workflow events exists as projection-only behavior.

Current `BeforeReport` behavior:

- explicit report-bearing callers can require a `BeforeReport` hook before report generation;
- missing required `BeforeReport` hook input produces report-generation error code `executor.hook.before_report.required`;
- the workflow run and event history remain unchanged because `BeforeReport` is report-path-only and post-terminal.

## 5. Why Pre-Skill Is Higher Authority

`BeforeSkillInvocation` is not just a report completeness gate. It sits before local work is performed.

That means a required pre-skill hook can affect:

- whether the local skill handler is invoked;
- whether a run fails before work happens;
- whether later steps execute;
- whether retry behavior is reached;
- whether final reports cite completed or incomplete work;
- whether side-effect-adjacent disclosures are visible before execution.

Because this checkpoint can block work, it must not depend on the agent remembering prose instructions. It must be represented as explicit executor input and deterministic runtime behavior.

## 6. Proposed Enforcement Boundary

Recommended future boundary: **explicit required-step policy**.

Add a small request-level policy for local execution that can say:

```text
BeforeSkillInvocation is required for these step IDs.
```

The first implementation should support:

- a bounded list of required step IDs;
- one explicit supplied `BeforeSkillInvocation` hook input per required step;
- fail-closed behavior when a required step reaches local skill invocation without a matching safe hook input.

Do not implement an ambient "all skill invocations require hooks" mode in the first slice. That mode becomes safer after the executor can accept a bounded vector of hook inputs and after schema/runtime config posture is planned.

## 7. Required Hook Input Shape

The current single optional hook input is enough for some one-step tests, but it is too narrow for general multi-step required checkpoint enforcement.

Recommended future implementation shape:

- preserve the existing `before_skill_invocation_hook: Option<...>` compatibility path if possible;
- add a bounded explicit collection, such as `before_skill_invocation_hooks: Vec<...>`, or add a small wrapper that can normalize the existing single input plus future collection input;
- reject duplicate hook inputs for the same target step ID;
- require every configured required step ID to have at most one matching hook input when that step is reached;
- keep hook inputs targeted by step ID, skill ID, and skill version.

If compatibility makes adding a vector too large for the next phase, the implementation may start with a single required step ID and the existing single hook input. In that case, documentation and tests must state that multi-step broad requirement is deferred.

## 8. Missing Required Hook Behavior

When a step is marked as requiring `BeforeSkillInvocation` and no matching hook input exists:

- append no hook workflow events;
- append no `SkillInvocationRequested`;
- do not invoke the local skill handler;
- fail the run through the existing failed-step path;
- return stable error code:

```text
executor.hook.before_skill_invocation.required
```

Recommended stable message:

```text
before-skill-invocation hook is required before skill invocation
```

The error must not include workflow IDs, run IDs, step IDs, skill IDs, hook IDs, paths, raw context, payloads, command output, provider output, parser output, disclosures, tokens, or secret-like values.

## 9. Matching And Validation Policy

A supplied hook should satisfy a required step only when all of the following match the active invocation:

- hook kind is `BeforeSkillInvocation`;
- step ID;
- skill ID;
- skill version;
- workflow ID;
- workflow version;
- run ID;
- schema version;
- spec hash;
- correlation ID where required by the existing invocation model.

Existing validation errors should remain stable and non-leaking:

- `executor.hook.before_skill_invocation.kind_mismatch`;
- `executor.hook.before_skill_invocation.skill_mismatch`;
- `executor.hook.before_skill_invocation.identity_mismatch`;
- `executor.hook.before_skill_invocation.unsupported_status`;
- `executor.hook.before_skill_invocation.failed_closed`.

Add only the new missing-required error unless duplicate-target validation needs a separate code such as:

```text
executor.hook.before_skill_invocation.duplicate_target
```

## 10. Policy, Approval, And SideEffect Ordering

Preserve current ordering for the first required-checkpoint implementation:

1. schedule step;
2. request approval if the step requires approval;
3. resume only after approval is granted;
4. evaluate invoke policy;
5. verify local handler availability;
6. append explicit SideEffect proposed/denied/skipped events where supplied;
7. enforce required `BeforeSkillInvocation` checkpoint;
8. append `SkillInvocationRequested` only if the required checkpoint passes;
9. invoke the local handler.

Rules:

- policy denial must still prevent hook events;
- approval requirements must not be bypassed by hook status;
- hooks must not create approval requests or approval decisions;
- hooks must not create policy decisions;
- side-effect event append must remain explicit and non-mutating;
- required hook failure must not consume a skill retry attempt because the skill invocation never starts.

If a future phase wants hooks before SideEffect event append, that must be planned separately because it changes side-effect disclosure ordering.

## 11. Event Ordering And Replay

For a matching required hook with `Passed`:

1. append `HookInvocationRequested`;
2. append `HookInvocationEvaluated(Passed)`;
3. append `SkillInvocationRequested`;
4. continue execution.

For a matching required hook with `FailedClosed`:

1. append `HookInvocationRequested`;
2. append `HookInvocationEvaluated(FailedClosed)`;
3. append `RunFailed`;
4. append no `SkillInvocationRequested`.

For a missing required hook:

1. append no hook events;
2. append no `SkillInvocationRequested`;
3. append `RunFailed` through existing failure handling.

Replay requirements:

- duplicate execution with the same run ID must rehydrate existing run state;
- required hook enforcement must not append duplicate hook events;
- changing required hook inputs after a run has already reached terminal state must not rewrite history;
- failed required hook enforcement must remain deterministic from the immutable run identity and explicit request inputs.

## 12. Failure Semantics

Failure behavior should remain fail-closed.

| Case | Behavior |
| --- | --- |
| Required step has no matching hook input | fail run before hook and skill events |
| Required step has mismatched hook input | fail run before hook and skill events |
| Required step has invalid hook input | fail run before hook and skill events |
| Required step has unsupported status | fail run before hook and skill events |
| Required step has `FailedClosed` status | append requested/evaluated, then fail before skill invocation |
| Required step has `Passed` status | append requested/evaluated, then continue |
| Non-required step has no hook input | current behavior, continue |
| Non-required step has later-step hook input | current behavior, ignore until target step |

Do not introduce warning-only continuation, skipped continuation, escalation, pause-for-approval, or blocked runtime state as part of required checkpoint enforcement.

## 13. Privacy And Redaction

Required checkpoint enforcement must not store, emit, or copy:

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
- evidence payloads;
- local check output;
- approval content;
- WorkReport content.

Errors must use stable codes and generic messages. Debug output for new request/policy types must redact caller-supplied IDs and text consistently with existing executor request types.

## 14. Test Plan

Future implementation tests should cover:

- existing `execute(...)` behavior remains unchanged when no required checkpoint policy is supplied;
- required one-step `BeforeSkillInvocation` hook with `Passed` continues execution;
- required one-step `BeforeSkillInvocation` hook with `FailedClosed` fails before skill invocation;
- missing required hook input fails with `executor.hook.before_skill_invocation.required`;
- required hook kind mismatch fails before hook or skill events;
- required hook skill mismatch fails before hook or skill events;
- required hook identity mismatch fails before hook or skill events;
- unsupported warning/skipped/blocked status fails before hook or skill events;
- missing required hook does not call the local handler;
- missing required hook appends no `HookInvocationRequested`;
- missing required hook appends no `HookInvocationEvaluated`;
- missing required hook appends no `SkillInvocationRequested`;
- policy denial still prevents hook events;
- approval-required steps still wait for approval before hook enforcement;
- duplicate run execution rehydrates without duplicate hook events;
- later-step required hook remains inactive until the target step;
- duplicate required hook targets are rejected if vector input is implemented;
- errors do not leak IDs, paths, raw context, disclosures, command output, provider output, parser output, or token-like values;
- Debug output for new input/policy types is redaction-safe;
- existing hook, SideEffect, WorkReport, EvidenceReference, Diagnostic, validation, adapter telemetry, local check, runtime, CLI, and docs tests still pass.

## 15. Proposed Implementation Sequence

Recommended small future sequence:

1. Add request-level required `BeforeSkillInvocation` checkpoint policy for explicit step IDs.
2. Add bounded hook-input collection or a single-required-step compatibility slice.
3. Enforce missing required hook failure before `SkillInvocationRequested`.
4. Preserve existing `Passed` and `FailedClosed` behavior for matching supplied hooks.
5. Add focused event-ordering, non-leakage, no-handler-call, policy-denial, approval-ordering, and replay tests.
6. Review before broadening to all skill invocations, runtime config, workflow schemas, warning/skipped continuation, or hook persistence.

## 16. Deferred Work

Explicitly deferred:

- broad automatic executor hook invocation;
- all-skill required hook policy;
- workflow-declared hook configuration;
- runtime hook configuration;
- schema exposure;
- CLI rendering or commands;
- hook persistence;
- dedicated hook audit sink emission;
- warning/skipped continuation;
- blocked runtime status;
- local check execution;
- command execution;
- adapter invocation;
- side-effect execution;
- writes;
- approval evidence attachment;
- WorkReport hook event citation targets beyond explicit supplied IDs;
- hosted/distributed execution;
- recursive agents and agent swarms;
- release posture changes.

## 17. Open Questions

- Should the first code slice add a bounded vector of pre-skill hook inputs, or should it start with one required step and the existing single hook input?
- Should required pre-skill checkpoint policy live directly on `LocalExecutionRequest`, or in a small `LocalExecutionHookCheckpointInputs` type shared by report and execution paths?
- Should duplicate supplied hook inputs for the same step be rejected at request validation time or only when the step is reached?
- Should the required checkpoint policy identify step IDs only, or step ID plus skill ID/version?
- Should `BeforeSkillInvocation` ever run before explicit SideEffect proposed/denied/skipped event append, or is the current side-effect-before-hook order the correct disclosure posture?
- When should hook workflow event IDs become WorkReport citation targets?

## 18. Final Recommendation

Proceed next to **BeforeSkillInvocation required checkpoint enforcement review**.

The first implementation adds the opt-in local executor request policy requiring `BeforeSkillInvocation` for explicitly selected step IDs, enforces missing/mismatched hook input before `SkillInvocationRequested`, preserves current `Passed` and `FailedClosed` semantics, and keeps warning/skipped/blocked, workflow schemas, runtime config, persistence, CLI behavior, local check execution, adapter invocation, side effects, writes, hosted behavior, recursive agents, agent swarms, and release posture out of scope. A focused maintainer review should verify event ordering, fail-closed behavior, replay posture, and non-leaking errors before any broader checkpoint policy is added.
