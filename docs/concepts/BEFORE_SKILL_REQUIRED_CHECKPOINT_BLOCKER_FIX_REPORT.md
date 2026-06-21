# BeforeSkillInvocation Required Checkpoint Blocker Fix Report

## 1. Executive Summary

This phase fixes the blocker found in [BeforeSkillInvocation Required Checkpoint Enforcement Review](BEFORE_SKILL_REQUIRED_CHECKPOINT_ENFORCEMENT_REVIEW.md).

The original implementation allowed `LocalExecutionBeforeSkillInvocationCheckpointInputs::required_step_ids` to contain step IDs that were not present in the selected workflow. Because such steps are never reached, a typo or stale ID could silently enforce no checkpoint. That is unsafe for a required governance gate.

The fix validates required checkpoint step IDs against the loaded workflow step set during execution preparation, before `ExecutionPlan` creation and before any run events are appended.

## 2. Blocker Fixed

Fixed blocker:

- unknown required `BeforeSkillInvocation` step IDs now fail closed instead of silently doing nothing.

The stable error code is:

```text
executor.hook.before_skill_invocation.unknown_required_step
```

The error message is generic and does not include the raw step ID:

```text
before-skill-invocation required checkpoint policy references an unknown step
```

## 3. Implementation Approach

The fix adds a second validation pass for `LocalExecutionBeforeSkillInvocationCheckpointInputs` after the target workflow has been loaded and before step execution plans are built.

Validation now covers:

- duplicate required step IDs;
- required step IDs that are absent from the selected workflow.

Unknown required-step validation happens before run creation, so it appends no workflow events, invokes no local handler, creates no hook events, and performs no side effects.

## 4. Scope Completed

- Added unknown required-step validation for selected-step `BeforeSkillInvocation` checkpoint policy.
- Added a stable non-leaking error code.
- Added a focused regression test proving:
  - unknown required step ID fails closed;
  - no run events are appended;
  - no local handler is invoked;
  - the unknown step ID is not leaked through error message, debug output, or request debug output.

## 5. Scope Explicitly Not Completed

This blocker fix did not implement:

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

## 6. Privacy And Redaction Summary

The new error is stable and non-leaking.

It does not expose:

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

`LocalExecutionBeforeSkillInvocationCheckpointInputs` Debug output remains count-only.

## 7. Test Coverage Summary

Added regression coverage for unknown required checkpoint step IDs.

Existing coverage continues to verify:

- default no-hook execution remains unchanged;
- matching `Passed` hook continues execution;
- missing required hook fails before hook or skill events;
- mismatched hook target fails as missing required hook;
- matching `FailedClosed` hook preserves existing failed-closed behavior;
- duplicate required step IDs fail closed without leaking;
- request Debug output redacts required step IDs.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test local_executor` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- Required `BeforeSkillInvocation` enforcement remains limited to explicit selected step IDs.
- The first slice still uses the existing single optional hook input.
- Broad all-skill required hook policy is not implemented.
- Workflow-declared hook requirements are not implemented.
- Runtime hook configuration is not implemented.
- Warning/skipped/blocked status broadening is not implemented.
- Hook persistence and dedicated hook audit sink emission are not implemented.

## 10. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation required checkpoint blocker fix review**.

The fix closes the unsafe false-negative policy gap. A focused review should verify unknown-step fail-closed behavior, event non-mutation, non-leaking errors, and scope cleanliness before broadening the pre-skill checkpoint API.
