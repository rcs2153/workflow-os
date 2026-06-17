# BeforeSkillInvocation Hook Boundary Hardening Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The boundary hardening phase added focused regression coverage for the explicit `BeforeSkillInvocation` executor hook path without changing runtime semantics. The new tests confirm later-step targeting, missing-handler behavior, policy-denial ordering, and hook request debug redaction.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved semantics-preserving hardening scope.

Confirmed in scope:

- test helper expansion for targeted hook inputs;
- later-step hook targeting coverage;
- non-matching current-step coverage through the later-step test;
- missing local skill handler coverage;
- policy-denial coverage;
- request debug redaction coverage;
- documentation/report updates.

No accidental implementation was found for:

- automatic hook invocation;
- workflow-declared hook configuration;
- runtime hook configuration;
- hook status broadening;
- warning continuation;
- skipped-with-disclosure continuation;
- failed-closed evaluated event support;
- blocked runtime status support;
- post-terminal workflow events;
- dedicated hook audit sink emission;
- hook persistence;
- hook observability metrics;
- CLI behavior;
- schema changes;
- automatic local check execution;
- command execution;
- adapter invocation;
- approval evidence attachment;
- `EvidenceReference` creation or attachment;
- side-effect boundary implementation;
- writes;
- reasoning lineage;
- recursive agents or agent swarms;
- hosted/distributed runtime behavior;
- release posture changes.

## 3. Runtime Semantics Assessment

The runtime behavior remains unchanged.

`LocalExecutor::invoke_local_skill(...)` still performs policy evaluation before handler lookup, handler lookup before `append_before_skill_invocation_hook(...)`, and hook event append before `SkillInvocationRequested`.

`append_before_skill_invocation_hook(...)` still:

- returns immediately when no explicit hook input is supplied;
- ignores non-matching step IDs;
- validates hook identity for the matching step;
- executes the existing side-effect-free runtime hook helper;
- continues only for `AgentHarnessHookInvocationStatus::Passed`;
- appends `HookInvocationRequested` and `HookInvocationEvaluated` before skill invocation.

No workflow state mutation, report artifact behavior, CLI behavior, local check execution, adapter execution, side effects, or writes were introduced.

## 4. Multi-Step Targeting Assessment

The new later-step test is valuable and correctly shaped.

It proves:

- a hook targeting `echo-2` does not fire during `echo-1`;
- `echo-1` still executes normally;
- the hook fires only after `echo-2` is scheduled;
- `HookInvocationRequested` precedes `HookInvocationEvaluated`;
- both hook events precede `SkillInvocationRequested` for `echo-2`;
- exactly two hook workflow events are emitted.

This closes the prior review gap around explicit hooks targeting later steps in multi-step workflows.

## 5. Missing Handler And Policy Denial Assessment

The missing-handler regression test proves that absent local skill handlers fail the run before any hook or skill invocation event is appended. This matches the executor ordering and prevents hook events from implying that an invocation checkpoint ran for an unexecutable local skill.

The policy-denial regression test targets the denied adapter-backed later step and proves:

- the policy denial is recorded;
- the first local step still runs;
- no hook events are appended for the denied step;
- no denied-step `SkillInvocationRequested` event is appended.

This preserves the policy-before-side-effect and policy-before-hook boundary documented in the planning phase.

## 6. Redaction And Privacy Assessment

The added debug regression test confirms `LocalExecutionRequest` debug output does not expose hook invocation ID, target skill ID, input evidence reference, or output evidence reference through the optional hook field.

The test intentionally does not require top-level request fields such as the explicit run ID to be hidden, because those are already part of the surrounding request surface. The covered hook-specific values are the relevant caller-supplied governed hook context for this phase.

No raw provider payloads, command output, CI logs, Jira/GitHub bodies, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded hook context, or evidence payloads were copied into hook events.

## 7. Test Quality Assessment

The focused tests now cover the prior non-blocking gaps:

- later-step targeted hook behavior;
- ignored non-matching current-step behavior;
- missing handler with no hook event append;
- policy denial with no hook event append;
- request debug redaction for hook-specific context.

Existing tests continue to cover:

- successful explicit hook event append;
- event ordering relative to policy and skill invocation;
- no-hook event shape;
- helper failure without partial hook or skill events;
- identity mismatch non-leakage;
- duplicate run replay without duplicate hook events;
- runtime hook event state transitions;
- generic hook audit projection;
- WorkReport hook citation vocabulary.

One non-blocking gap remains: unsupported hook status behavior is not directly injected through this executor path because the current side-effect-free helper returns `Passed` for valid inputs and fails before producing a safe non-passed result for invalid inputs. This should stay deferred until a safe structured `FailedClosed` or explicit test seam is designed.

## 8. Documentation Review

Documentation now states:

- BeforeSkillInvocation boundary hardening tests are implemented;
- `Passed` remains the only continuing hook status;
- automatic hook invocation is not implemented;
- workflow-declared hook configuration is not implemented;
- runtime hook configuration is not implemented;
- status broadening is not implemented;
- dedicated hook audit sink emission is not implemented;
- hook persistence is not implemented;
- CLI behavior is not implemented;
- workflow schema fields are not implemented;
- automatic local check execution is not implemented;
- side effects and writes remain unsupported;
- recursive agents, agent swarms, hosted behavior, and release posture changes are not introduced.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Design a safe structured `FailedClosed` hook result path before adding failed-closed evaluated event support.
- Add direct unsupported-status regression coverage only if a future helper seam can model it without fake evidence, fake side effects, or unsafe event semantics.
- Consider an options/builder-style request API before release hardening if optional executor feature fields continue to grow.
- Decide whether hook invocation IDs should remain caller-supplied or become deterministic per step attempt before retry-aware hook semantics are introduced.

## 11. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation failed-closed result path planning**.

Reason: the explicit `Passed` checkpoint is accepted and now has the boundary coverage requested by review. The next safe step is planning how a non-passed hook result, especially `FailedClosed`, could become durable without partial events, fake evidence, misleading diagnostics, hidden side effects, or ambiguous replay semantics.

The next phase must not implement warning continuation, skipped-with-disclosure continuation, blocked runtime behavior, automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sinks, persistence, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
