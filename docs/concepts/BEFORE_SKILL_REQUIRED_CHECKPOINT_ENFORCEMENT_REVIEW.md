# BeforeSkillInvocation Required Checkpoint Enforcement Review

## 1. Executive Verdict

Needs blocker fixes.

The phase correctly implements the first opt-in local executor enforcement path for selected-step `BeforeSkillInvocation` required checkpoints. Missing and mismatched hook input for an active required step fails closed before hook or skill invocation events, matching `Passed` and `FailedClosed` hooks preserve existing semantics, default no-hook behavior remains unchanged, and validation passed.

One blocker remains: `required_step_ids` are not validated against the workflow step set. A typo or stale step ID in the required checkpoint policy can silently enforce nothing because the step is never reached. For a governance gate, that is an unsafe false-negative configuration.

## 2. Scope Verification

The phase stayed within approved scope:

- local-executor-only;
- request-input-driven;
- explicit selected step IDs only;
- no default hook registration;
- no broad automatic hook invocation;
- no workflow-declared hook configuration;
- no runtime hook configuration;
- no schema changes;
- no CLI commands or rendering;
- no hook persistence;
- no dedicated hook audit sink emission;
- no local check execution;
- no command execution;
- no adapter invocation;
- no side-effect execution;
- no writes;
- no hosted or distributed runtime behavior;
- no recursive agents or agent swarms;
- no release posture changes.

The only CLI changes are compile-required default request field population, not a behavior or command surface expansion.

## 3. Enforcement Boundary Assessment

The implemented boundary is appropriately narrow. `LocalExecutionRequest` now carries `LocalExecutionBeforeSkillInvocationCheckpointInputs`, and the default value preserves existing `execute(...)` behavior.

For the active step, the executor checks whether the step is required before appending `SkillInvocationRequested`. If a required step has no matching explicit hook input, or if the single supplied hook targets another step, the executor returns a stable required-hook error and fails the run through the existing failure path.

That is the right enforcement location for this slice: policy and handler availability still run first, explicit SideEffect proposed/denied/skipped event append remains in its previously planned position, and skill invocation only starts after the required checkpoint passes.

The blocker is configuration validation: the required checkpoint policy accepts step IDs that are not part of the workflow. Because those IDs are never reached, enforcement never fires. Required checkpoint policy should fail closed during execution preparation when any configured required step ID is not in the loaded workflow step set.

## 4. API Assessment

The new API is small and idiomatic:

```rust
pub struct LocalExecutionBeforeSkillInvocationCheckpointInputs {
    pub required_step_ids: Vec<StepId>,
}
```

`Debug` exposes only `required_step_count`, which is appropriate for redaction. The type is exported from `workflow-core` consistently with existing local executor request types.

The API does allow multiple required step IDs while the executor still accepts only one `before_skill_invocation_hook` input. That is acceptable as a fail-closed first slice, but it is awkward: callers can require multiple steps but can only satisfy one in a single request. This should be resolved by either adding a bounded hook input collection or narrowing the first public policy surface to one required step until multi-hook input is available.

## 5. Runtime Behavior Assessment

Verified behavior:

- no required policy preserves existing optional no-hook execution;
- matching `Passed` hook appends hook requested/evaluated events before `SkillInvocationRequested`;
- missing required hook fails the run before hook events and before skill invocation events;
- mismatched required hook target is treated as missing required input;
- matching `FailedClosed` hook preserves existing requested/evaluated event behavior and fails before skill invocation;
- duplicate required step IDs are rejected with a stable non-leaking error.

The behavior remains deterministic and local. It does not create hook evidence, approvals, reports, side effects, local check results, or provider calls.

## 6. Ordering And Replay Assessment

The local step ordering remains:

1. schedule step;
2. approval request/resume where applicable;
3. invoke-policy evaluation;
4. handler availability check;
5. explicit SideEffect proposed/denied/skipped event append where supplied;
6. required `BeforeSkillInvocation` checkpoint enforcement;
7. `SkillInvocationRequested`;
8. local handler invocation.

This preserves policy-before-work and approval-before-work ordering. Required hook failure does not consume skill retry attempts because the skill invocation never starts.

Replay posture is consistent with existing executor behavior: duplicate run IDs rehydrate durable state before rerunning hook logic. The focused tests cover existing hook replay/idempotency behavior, but they do not add a specific duplicate-run test for the new required checkpoint policy. That is a useful follow-up, not a blocker.

## 7. Privacy And Redaction Assessment

The new failure errors use stable generic codes and messages:

- `executor.hook.before_skill_invocation.required`;
- `executor.hook.before_skill_invocation.duplicate_required_step`.

Errors and Debug output do not include step IDs, run IDs, hook IDs, paths, command output, provider payloads, parser payloads, disclosures, tokens, or secret-like values.

The implementation does not copy raw hook context, local check output, evidence payloads, WorkReport content, approval content, or SideEffect payloads beyond the already validated explicit event paths.

## 8. Test Quality Assessment

Tests cover the important first-slice behavior:

- default no-hook execution remains unchanged;
- required hook with `Passed` continues execution;
- missing required hook fails before hook and skill events;
- mismatched target fails as missing required hook;
- `FailedClosed` keeps existing failed-closed semantics;
- duplicate required step IDs fail closed without leaking the step ID;
- request Debug output redacts required step IDs.

Missing or shallow tests:

- unknown required step ID should fail closed during preparation;
- multi-step required policy with only one hook input should document and test the fail-closed limitation;
- required checkpoint plus approval-required step ordering should be explicitly tested;
- required checkpoint plus policy denial ordering should be explicitly tested;
- duplicate run replay with required checkpoint policy should be explicitly tested.

The unknown-step test is tied to the blocker. The rest are non-blocking hardening follow-ups.

## 9. Documentation Review

Docs correctly state that:

- first explicit selected-step required `BeforeSkillInvocation` enforcement is implemented;
- broad required `BeforeSkillInvocation` runtime enforcement is not implemented;
- broader automatic executor hook invocation is not implemented;
- workflow-declared hook configuration is not implemented;
- runtime hook configuration is not implemented;
- warning/skipped/blocked status broadening is not implemented;
- discovery from workflow events or audit projections remains deferred;
- dedicated hook audit sink emission is not implemented;
- hook persistence is not implemented;
- CLI behavior and workflow schema fields are not implemented;
- automatic local checks, command execution, adapter invocation, side-effect execution, writes, hosted behavior, recursive agents, agent swarms, and release posture changes remain unsupported.

The implementation report is accurate except that it does not call out the unknown required-step false-negative risk. That should be added in the blocker-fix report rather than rewriting this review finding.

## 10. Blockers

- Validate `LocalExecutionBeforeSkillInvocationCheckpointInputs::required_step_ids` against the loaded workflow step IDs during execution preparation. If any required step ID is absent from the workflow, fail closed before run creation or before any run events with a stable non-leaking error, such as `executor.hook.before_skill_invocation.unknown_required_step`. The error must not include the raw step ID.

## 11. Non-Blocking Follow-Ups

- Add a bounded vector of `BeforeSkillInvocation` hook inputs, or narrow the first policy surface so callers cannot configure multiple required steps without a way to satisfy them.
- Add focused replay/idempotency tests for required checkpoint policy.
- Add explicit approval-ordering and policy-denial-ordering tests for required checkpoint policy.
- Add a multi-step required checkpoint test that documents current single-hook limitations.
- Consider whether duplicate required step IDs should be rejected before project load or after workflow lookup alongside unknown-step validation for one coherent policy validation pass.

## 12. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation required checkpoint blocker fix**.

The implementation is close, but required governance policy must not silently miss because of a typo or stale step ID. Fixing unknown required-step validation should happen before broadening hook input collections, all-skill policies, workflow-declared configuration, runtime config, or warning/skipped continuation.

## 13. Validation

Review validation commands run:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
