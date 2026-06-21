# BeforeSkillInvocation Required Checkpoint Blocker Fix Review

## 1. Executive Verdict

Blocker fixed with non-blocking follow-ups.

The blocker identified in [BeforeSkillInvocation Required Checkpoint Enforcement Review](BEFORE_SKILL_REQUIRED_CHECKPOINT_ENFORCEMENT_REVIEW.md) is fixed. `LocalExecutionBeforeSkillInvocationCheckpointInputs::required_step_ids` are now validated against the selected workflow's actual step IDs after project load and workflow lookup, before `ExecutionPlan` creation and before run creation events. Unknown required step IDs fail closed with a stable non-leaking error and do not silently enforce nothing.

The implementation remains local-executor-only, request-input-driven, and scoped to explicit selected step IDs. It does not broaden hook execution, schema behavior, CLI behavior, persistence, local checks, command execution, adapter invocation, side-effect execution, writes, hosted behavior, recursive agents, agent swarms, or release posture.

## 2. Scope Verification

The blocker fix stayed within the approved blocker-fix scope.

Implemented:

- fail-closed validation for required `BeforeSkillInvocation` checkpoint step IDs that are absent from the selected workflow;
- stable error code `executor.hook.before_skill_invocation.unknown_required_step`;
- non-leaking generic error message;
- regression coverage proving no run events are appended and no local handler is invoked;
- documentation and blocker-fix report updates.

Not introduced:

- broad automatic hook invocation;
- all-skill required hook policy;
- bounded multi-hook input collection;
- workflow-declared hook configuration;
- runtime hook configuration;
- schema changes;
- CLI behavior;
- hook persistence;
- dedicated hook audit sink emission;
- automatic local check execution;
- command execution;
- adapter invocation;
- side-effect execution;
- writes;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- release posture changes.

## 3. Original Blocker Restatement

The original blocker was a false-negative governance gap.

The first selected-step required checkpoint implementation allowed callers to configure required `BeforeSkillInvocation` checkpoint step IDs that were not present in the loaded workflow. Because those steps would never execute, the required checkpoint would never be enforced. A typo or stale step ID could therefore make a required governance gate look configured while enforcing nothing.

For a pre-skill checkpoint, that behavior is unsafe because `BeforeSkillInvocation` sits before local work is performed.

## 4. Fix Approach Assessment

The fix validates required checkpoint step IDs after the workflow is selected and before step execution plans are built.

This is the right boundary:

- duplicate required step ID validation can remain request-local;
- unknown required step ID validation needs the loaded workflow step set;
- validation happens before `RunCreated`, `RunValidated`, `RunStarted`, hook events, side-effect events, or skill invocation events;
- failure does not mutate durable run state;
- the implementation avoids changing existing no-hook behavior.

The approach is minimal and idiomatic for the current executor architecture. It does not introduce a new public contract shape, runtime config, or schema field.

## 5. Validation Boundary Assessment

Validated behavior:

- required checkpoint step IDs are compared to the selected workflow's `StepDefinition` IDs;
- any unknown required step ID fails closed;
- the stable error code is `executor.hook.before_skill_invocation.unknown_required_step`;
- the error message is generic;
- the raw unknown step ID is not included in the error message or Debug output;
- validation occurs before run creation and event append.

The existing duplicate-step validation remains intact with `executor.hook.before_skill_invocation.duplicate_required_step`.

The validation boundary is deterministic and local. It does not depend on model judgment, external systems, adapter calls, local command execution, or mutable runtime config.

## 6. Runtime Semantics Assessment

The fix preserves current runtime semantics:

- default local execution without required checkpoint policy is unchanged;
- matching `Passed` `BeforeSkillInvocation` hooks still continue execution;
- matching `FailedClosed` hooks still append requested/evaluated hook events and fail before `SkillInvocationRequested`;
- missing or mismatched required hook input still fails before hook or skill invocation events;
- unknown required step IDs now fail earlier, before run creation;
- handler invocation is not reached on unknown required step ID;
- event history remains empty for the unknown-step failure case.

This closes the blocker without changing workflow pass/fail semantics for valid requests.

## 7. Privacy And Redaction Assessment

The fix is redaction-safe.

The unknown-step error does not expose:

- unknown step IDs;
- workflow IDs;
- run IDs;
- skill IDs;
- hook IDs;
- file paths;
- command output;
- provider payloads;
- parser payloads;
- disclosures;
- credentials;
- tokens;
- secret-like values.

`LocalExecutionBeforeSkillInvocationCheckpointInputs` Debug output remains count-only, so request Debug output does not reveal required step IDs.

## 8. Test Quality Assessment

The new regression test covers the blocker directly:

- an unknown required checkpoint step ID fails closed;
- the error code is stable;
- the error message is generic;
- the raw step ID is absent from error message, error Debug, and request Debug output;
- request Debug still exposes only the required step count;
- the local handler is not invoked;
- no run events are appended.

Existing tests continue to cover:

- default no-hook execution;
- required `Passed` hook behavior;
- required missing-hook behavior;
- mismatched hook target behavior;
- `FailedClosed` hook behavior;
- duplicate required step ID rejection;
- request Debug redaction.

Remaining useful non-blocking test additions:

- required checkpoint plus approval-required step ordering;
- required checkpoint plus policy-denial ordering;
- duplicate run replay with required checkpoint policy;
- multi-step required policy that documents the current single-hook input limitation.

## 9. Documentation Review

Docs now accurately state that:

- the first explicit selected-step required `BeforeSkillInvocation` enforcement slice is implemented;
- the unknown required-step blocker is fixed;
- unknown required step IDs fail closed before run creation;
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

The blocker-fix report records the implementation approach, validation boundary, privacy posture, commands run, remaining limitations, and recommended next phase.

## 10. Blockers

None.

The original blocker is fixed.

## 11. Non-Blocking Follow-Ups

- Add a bounded vector of `BeforeSkillInvocation` hook inputs so callers can satisfy multiple required step IDs in one request.
- Add focused ordering tests for required checkpoints combined with approval-required steps.
- Add focused ordering tests for required checkpoints combined with policy-denied steps.
- Add duplicate-run replay coverage for required checkpoint policy.
- Consider moving duplicate required-step validation and unknown required-step validation into a single named policy-validation helper once multi-hook input collection exists.

## 12. Recommended Next Phase

Recommended next phase: **bounded BeforeSkillInvocation hook input collection planning**.

The blocker is fixed, and the current selected-step enforcement path is now safe for one explicit hook input. The remaining architectural pressure is that callers can require multiple step IDs but can only supply one `before_skill_invocation_hook` input today. The next phase should plan a bounded, explicit multi-hook input collection before implementation, without adding workflow schema fields, runtime hook configuration, automatic local checks, command execution, adapter invocation, side effects, writes, hosted execution, recursive agents, agent swarms, or release posture changes.

## 13. Validation

Review validation commands run:

- `cargo test -p workflow-core --test local_executor` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
