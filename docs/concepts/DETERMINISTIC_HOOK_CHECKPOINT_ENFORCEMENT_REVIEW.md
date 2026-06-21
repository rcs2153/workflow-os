# Deterministic Hook Checkpoint Enforcement Review

Review date: 2026-06-21

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation delivers the intended narrow enforcement expansion: explicit report-bearing callers can require a `BeforeReport` hook checkpoint before report generation. The behavior remains opt-in, local, in-memory, report-path-only, and non-mutating. Missing required hook input fails report generation closed with a stable non-leaking error while preserving the workflow run, snapshot, event history, audit posture, artifact posture, CLI posture, schema posture, and existing `execute(...)` behavior.

## 2. Scope Verification

The phase stayed within approved deterministic hook checkpoint enforcement scope.

Implemented scope:

- added `LocalExecutionHookCheckpointInputs`;
- added `LocalExecutionReportInputs::hook_checkpoints`;
- added explicit `require_before_report` policy;
- enforced the required `BeforeReport` checkpoint in explicit report-bearing paths;
- reused existing `BeforeReport` hook validation and execution;
- shared the report-path hook checkpoint handling between `execute_with_report(...)` and `execute_with_report_and_side_effect_discovery(...)`;
- added tests and documentation.

No accidental behavior was introduced for:

- automatic hook invocation across all executor paths;
- workflow-declared hook configuration;
- runtime hook configuration;
- broad required `BeforeSkillInvocation` policy;
- warning/skipped/blocked hook continuation;
- hook persistence;
- dedicated hook audit sink emission;
- discovery from workflow events or audit projections;
- post-terminal hook workflow events;
- local check execution;
- command execution;
- adapter invocation;
- side-effect modeling or runtime side-effect execution;
- provider writes;
- CLI behavior;
- workflow schema changes;
- examples;
- hosted/distributed behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 3. API Assessment

The API is appropriately small:

```rust
pub struct LocalExecutionHookCheckpointInputs {
    pub require_before_report: bool,
}
```

Adding this policy to `LocalExecutionReportInputs` keeps enforcement colocated with report generation inputs. Default behavior remains optional because `LocalExecutionHookCheckpointInputs::default()` sets `require_before_report` to `false`.

The type is exported consistently with the surrounding local executor/report API. Debug output for `LocalExecutionReportInputs` exposes only the boolean policy and still redacts report identity, actor identity, redaction metadata, and hook context.

Non-blocking compatibility note: `LocalExecutionReportInputs` is a public struct and adding a field requires downstream struct-literal callers to update. This is acceptable in the current preview-stage branch, but a future public API hardening pass should consider builders, constructors, or `#[non_exhaustive]`-style posture before stronger compatibility promises.

## 4. Runtime Semantics Assessment

Runtime semantics are preserved.

`LocalExecutor::execute_with_report(...)` still:

- calls existing `execute(...)`;
- returns execution failures unchanged when no run is produced;
- only considers `BeforeReport` after a terminal run exists;
- returns report-generation errors beside the run;
- does not alter the workflow pass/fail result.

`execute_with_report_and_side_effect_discovery(...)` now uses the same required checkpoint handling as `execute_with_report(...)`, removing duplicated hook handling while preserving its explicit side-effect discovery behavior.

`LocalExecutor::execute(...)`, approval decisions, cancellation, retry, escalation, local check registration, side-effect event append behavior, artifact writing, and CLI behavior remain unchanged.

## 5. Required Checkpoint Assessment

The required checkpoint behavior is deterministic and fail-closed.

When `require_before_report` is `false`:

- no `BeforeReport` hook is required;
- supplied `BeforeReport` hooks still run as before;
- existing report-bearing behavior remains intact.

When `require_before_report` is `true` and no hook input is supplied:

- no report is generated;
- the returned result preserves the run;
- the error code is stable: `executor.hook.before_report.required`;
- no hook invocation IDs, workflow IDs, run IDs, evidence IDs, paths, or raw payload values are leaked.

When `require_before_report` is `true` and a valid hook input is supplied:

- the existing `BeforeReport` validation path runs;
- the hook invocation ID is cited through existing WorkReport hook citation behavior;
- discovered hook disclosure IDs are merged deterministically as before.

## 6. Event, Audit, And Artifact Assessment

The phase correctly avoids state mutation for the `BeforeReport` checkpoint.

The implementation does not:

- append `HookInvocationRequested` or `HookInvocationEvaluated` for `BeforeReport`;
- append post-terminal workflow events;
- mutate `WorkflowRun` or `WorkflowRunSnapshot`;
- emit dedicated hook audit sink records;
- emit hook observability events;
- persist hook results;
- write report artifacts automatically;
- create filesystem artifacts or CLI output.

Tests verify event history remains equal to the returned run after required checkpoint failure and successful required checkpoint execution.

## 7. Privacy And Redaction Assessment

Privacy posture is preserved.

The implementation does not copy:

- raw hook payloads;
- raw specs;
- command output;
- parser payloads;
- provider payloads;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- hook audit records;
- hook transcripts.

Errors are stable and non-leaking. Debug output remains redaction-safe. Serialization behavior for generated reports is unchanged.

## 8. Test Quality Assessment

Focused tests cover:

- required `BeforeReport` missing on `execute_with_report(...)`;
- required `BeforeReport` supplied on `execute_with_report(...)`;
- required `BeforeReport` missing on `execute_with_report_and_side_effect_discovery(...)`;
- report-only failure behavior;
- run status preservation;
- event history preservation;
- absence of `BeforeReport` workflow events;
- absence of report artifacts;
- hook citation when the required checkpoint is supplied;
- existing optional `BeforeReport` behavior;
- broader local executor regression coverage through the full local executor suite.

The full workspace test suite also passed, covering existing WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter, local check, SideEffect, approval, runtime, and CLI behavior.

Non-blocking test follow-up: add a small test that the default `LocalExecutionHookCheckpointInputs` value is permissive if a future public API hardening phase adds constructors or builders.

## 9. Documentation Review

Documentation accurately states that:

- deterministic required-checkpoint enforcement is implemented for explicit `BeforeReport` report paths;
- the behavior is opt-in;
- the behavior is report-path-only, in-memory-only, and non-mutating;
- automatic executor hook invocation is not implemented;
- workflow-declared and runtime hook configuration are not implemented;
- hook persistence is not implemented;
- dedicated hook audit sink emission is not implemented;
- CLI behavior is not implemented;
- schemas are not changed;
- local checks, adapter invocation, side effects, writes, hosted behavior, recursive agents, agent swarms, and release posture changes remain unsupported.

The phase report is honest about limitations and recommends review before expanding to pre-skill required checkpoints.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Consider public API ergonomics for `LocalExecutionReportInputs` before stronger compatibility commitments.
- Add a tiny default-policy regression test if constructors/builders are introduced.
- Plan required `BeforeSkillInvocation` checkpoint behavior separately before implementation.
- Keep `BeforeReport` event/audit persistence deferred unless a future phase explicitly designs post-terminal semantics.

## 12. Recommended Next Phase

Recommended next phase: **BeforeSkillInvocation required checkpoint planning**.

Why: `BeforeReport` required enforcement proves the opt-in fail-closed pattern on the safest report-side boundary. The next meaningful runtime enforcement gap is pre-skill checkpoint policy, but that location is higher authority because it gates work before local skill invocation. It needs a dedicated plan covering policy order, event order, missing required hook behavior, idempotency, failed-closed semantics, replay, audit projection, local check boundaries, side-effect boundaries, and non-leaking errors before implementation.
