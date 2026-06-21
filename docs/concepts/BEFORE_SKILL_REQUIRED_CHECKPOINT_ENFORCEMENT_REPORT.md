# BeforeSkillInvocation Required Checkpoint Enforcement Report

## 1. Executive Summary

This phase implements the first explicit required `BeforeSkillInvocation` checkpoint enforcement slice.

Local execution requests can now declare selected step IDs that require a matching explicit `BeforeSkillInvocation` hook before local skill invocation. If a required step reaches the pre-skill checkpoint without a matching safe hook input, the run fails closed before `SkillInvocationRequested`, appends no hook events, and does not invoke the local skill handler.

The implementation remains opt-in, local-executor-only, request-input-driven, and bounded to explicit selected step IDs. It does not implement broad automatic hook invocation, workflow-declared hook configuration, runtime hook configuration, schemas, CLI behavior, hook persistence, dedicated hook audit sink emission, local check execution, command execution, adapter invocation, side-effect execution, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Scope Completed

- Added `LocalExecutionBeforeSkillInvocationCheckpointInputs`.
- Added `LocalExecutionRequest::before_skill_invocation_checkpoints`.
- Added explicit selected-step required checkpoint policy through `required_step_ids`.
- Exported the new input type from `workflow-core`.
- Enforced missing required `BeforeSkillInvocation` hook input before `SkillInvocationRequested`.
- Enforced mismatched required hook target as missing required input.
- Preserved existing optional no-hook behavior by default.
- Preserved existing `Passed` hook behavior.
- Preserved existing explicit `FailedClosed` hook behavior.
- Added duplicate required-step validation for checkpoint policy inputs.
- Added focused local executor tests.
- Updated roadmap and concept documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic hook invocation for every executor path;
- workflow-declared hook configuration;
- runtime hook configuration;
- default hook registration;
- broad all-skill required hook policy;
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

## 4. API Summary

The new request policy type is:

```rust
pub struct LocalExecutionBeforeSkillInvocationCheckpointInputs {
    pub required_step_ids: Vec<StepId>,
}
```

`LocalExecutionRequest` now carries:

```rust
pub before_skill_invocation_checkpoints:
    LocalExecutionBeforeSkillInvocationCheckpointInputs
```

The default value has no required steps and preserves existing executor behavior.

The first implementation uses the existing `before_skill_invocation_hook: Option<LocalExecutionBeforeSkillInvocationHookInput>` compatibility path. A required step is satisfied only when that explicit hook targets the active step and passes existing hook validation. A future phase may add a bounded vector of pre-skill hook inputs before broad multi-step required hook policy is expanded.

## 5. Runtime Behavior Summary

When no required checkpoint policy is supplied:

- `LocalExecutor::execute(...)` behaves as before.
- Optional `BeforeSkillInvocation` hook input still runs only when its target step becomes active.

When a step ID is required and no matching hook input is supplied:

- no `HookInvocationRequested` event is appended;
- no `HookInvocationEvaluated` event is appended;
- no `SkillInvocationRequested` event is appended;
- the local skill handler is not invoked;
- the run fails closed through the existing run failure path.

The stable failure code is:

```text
executor.hook.before_skill_invocation.required
```

When a required matching hook is supplied with `Passed`, existing hook requested/evaluated events are appended and skill invocation continues.

When a required matching hook is supplied with `FailedClosed`, existing failed-closed behavior is preserved: requested/evaluated hook events are appended and the run fails before skill invocation.

## 6. Ordering Summary

The implementation preserves current local executor ordering:

1. schedule step;
2. request or resume approval when applicable;
3. evaluate invoke policy;
4. verify local handler availability;
5. append explicit SideEffect proposed/denied/skipped events when supplied;
6. enforce `BeforeSkillInvocation` checkpoint requirement;
7. append `SkillInvocationRequested` only after the required checkpoint passes;
8. invoke the local skill handler.

Policy denial and missing local handler behavior still occur before hook events. Approval behavior is unchanged. Required hook failure does not consume a skill retry attempt because skill invocation never starts.

## 7. Privacy And Redaction Summary

The required checkpoint errors use stable generic codes and messages.

Errors and Debug output do not expose:

- workflow IDs;
- run IDs;
- step IDs;
- skill IDs;
- hook invocation IDs;
- source paths;
- raw hook context;
- command output;
- provider output;
- parser payloads;
- disclosures;
- credentials;
- tokens;
- secret-like values.

The new checkpoint policy Debug output reports only the required step count.

## 8. Test Coverage Summary

Added focused tests covering:

- required `BeforeSkillInvocation` hook with `Passed` continues execution;
- missing required `BeforeSkillInvocation` hook fails before hook or skill events;
- mismatched required hook target fails as missing required hook;
- required `BeforeSkillInvocation` hook with `FailedClosed` preserves existing failed-closed semantics;
- duplicate required step IDs fail before run creation with a stable non-leaking error;
- request Debug output redacts required step IDs;
- existing optional no-hook behavior remains unchanged;
- existing optional `BeforeSkillInvocation` hook behavior remains intact.

Existing local executor tests continue to cover:

- later-step hook targeting;
- missing handler behavior;
- policy denial ordering;
- unsupported warning/skipped/blocked statuses;
- failed-closed replay/idempotency;
- WorkReport and `BeforeReport` hook paths;
- SideEffect event append and discovery paths;
- approval and cancellation behavior;
- local check and dogfood behavior.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test local_executor` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 10. Remaining Known Limitations

- Required `BeforeSkillInvocation` enforcement is limited to explicit selected step IDs.
- The first slice uses the existing single optional hook input.
- Broad all-skill required hook policy is not implemented.
- Workflow-declared hook requirements are not implemented.
- Runtime hook configuration is not implemented.
- Warning/skipped/blocked status broadening is not implemented.
- Hook persistence and dedicated hook audit sink emission are not implemented.
- Hook event WorkReport citation targets remain deferred beyond existing explicit supplied hook IDs.

## 11. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation required checkpoint enforcement review**.

This phase gates local skill invocation and is therefore higher authority than report-path hook enforcement. A focused maintainer review should verify fail-closed behavior, event ordering, replay posture, privacy/redaction behavior, approval/policy ordering, and scope cleanliness before broadening hook input collections, all-skill policies, workflow-declared configuration, runtime config, or warning/skipped continuation.
