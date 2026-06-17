# Executor Hook Report Input Propagation Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted; proceed to runtime hook execution planning.

The phase implemented the intended additive executor report input propagation for explicitly supplied `AgentHarnessHookInvocationId` values. The implementation adds one reference-only input field to `LocalExecutionReportInputs`, forwards it into `TerminalLocalWorkReportInput`, keeps Debug count-only, and verifies generated reports cite supplied hook IDs through the existing WorkReport citation path.

No runtime hook execution, hook event emission, audit sink emission, persistence, CLI behavior, schema changes, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes were introduced.

## 2. Scope Verification

The phase stayed within the approved executor input propagation scope.

Implemented:

- `LocalExecutionReportInputs::agent_harness_hook_invocation_ids`;
- count-only `agent_harness_hook_count` in `LocalExecutionReportInputs` Debug;
- forwarding from `LocalExecutionReportInputs` into `TerminalLocalWorkReportInput`;
- focused executor tests for generated report citation behavior, redaction-safe Debug, non-copying of hook payload markers, event-history preservation, and absence of automatic report artifacts;
- documentation updates and an implementation report.

No accidental scope expansion was found:

- no runtime hook execution;
- no executor-integrated hook invocation;
- no automatic hook citation discovery;
- no hook invocation ID creation;
- no hook invocation result creation;
- no hook audit record creation;
- no workflow event kinds;
- no workflow event append behavior;
- no audit sink emission;
- no hook audit record persistence;
- no report artifact behavior changes;
- no CLI behavior;
- no workflow schema fields;
- no workflow-declared hook configuration;
- no automatic local check execution;
- no default local check handler registration;
- no command-output evidence;
- no `EvidenceReference` creation or attachment;
- no approval evidence attachment;
- no reasoning lineage implementation;
- no side-effect boundary implementation;
- no writes;
- no recursive agents;
- no agent swarms;
- no release posture change.

## 3. API Assessment

The API change is minimal and appropriate.

`LocalExecutionReportInputs` now accepts `Vec<AgentHarnessHookInvocationId>`. This preserves typed validation at the ID boundary and avoids accepting raw strings, hook invocation results, hook audit records, hook contracts, hook disclosures, hook input/output references, output summaries, or hook payloads.

The change is additive to executor report input types. Existing executor methods remain intact:

- `execute(...) -> Result<WorkflowRun, WorkflowOsError>`;
- `execute_with_report(...) -> Result<LocalExecutionWithReportResult, WorkflowOsError>`;
- `decide_approval(...) -> Result<WorkflowRun, WorkflowOsError>`;
- `cancel_run(...) -> Result<WorkflowRun, WorkflowOsError>`.

## 4. Propagation Assessment

Propagation is correctly bounded.

`terminal_report_input_for_run(...)` now forwards:

```rust
agent_harness_hook_invocation_ids: report.agent_harness_hook_invocation_ids.clone()
```

Verified:

- executor-integrated report generation can now pass caller-supplied hook invocation IDs into the terminal report helper;
- the executor does not invent IDs;
- the executor does not discover hook records from storage;
- the executor does not validate hook record existence;
- the executor does not infer hook IDs from workflow events, audit events, report notes, local check results, or typed handoffs;
- absent hook IDs preserve current no-hook behavior.

## 5. Report Behavior Assessment

Report behavior is correct for this phase.

When hook IDs are supplied, generated reports include `WorkReportCitationTarget::AgentHarnessHook` citations in `ValidationAndQualityChecks`.

The implementation relies on the already-reviewed terminal report helper for citation construction. It does not create `EvidenceReference` values, hook invocation results, hook audit records, or missing hook citations.

When hook IDs are not supplied, generated reports retain explicit not-available section text rather than fabricating missing citations.

## 6. Workflow Semantics Assessment

Workflow semantics remain unchanged.

Verified:

- `execute(...)` still returns `WorkflowRun`;
- `execute_with_report(...)` still executes the workflow first;
- execution errors are not converted into report errors;
- report-generation errors remain separate from workflow execution results;
- hook input propagation does not mutate `WorkflowRun`;
- hook input propagation does not mutate `WorkflowRunSnapshot`;
- hook input propagation does not append workflow events;
- hook input propagation does not emit audit events;
- hook input propagation does not emit observability events;
- hook input propagation does not write report artifacts automatically;
- hook input propagation does not add `StateBackend` writes beyond existing executor behavior.

## 7. Privacy And Redaction Assessment

The privacy posture is acceptable.

Verified:

- `LocalExecutionReportInputs` Debug includes only `agent_harness_hook_count`;
- Debug output does not expose hook invocation IDs;
- generated report serialization includes the valid hook invocation ID as the stable citation reference, which is expected;
- generated report serialization does not include hook audit record fields;
- generated report serialization does not copy hook disclosures, hook input references, hook output references, supplemental references, workflow/run/actor context, hook output summaries, raw prompts, raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values.

## 8. Error Handling Assessment

Error handling remains aligned with existing report-bearing execution.

Because the API accepts `AgentHarnessHookInvocationId`, invalid raw hook ID values fail before executor propagation. If report generation fails after execution, the existing `LocalExecutionWithReportResult` pattern preserves the run and returns `report_generation_error`.

No evidence was found that hook-related values are included in error messages. The implementation does not convert report-generation failures into user project diagnostics or workflow execution failures.

## 9. Test Quality Assessment

Test coverage is focused and adequate for the phase.

Covered:

- `execute_with_report(...)` forwards supplied hook invocation IDs;
- generated citation target is `WorkReportCitationTarget::AgentHarnessHook`;
- generated citation kind is `WorkReportCitationKind::AgentHarnessHook`;
- hook citations appear in `ValidationAndQualityChecks`;
- absent hook IDs preserve current explicit not-available behavior;
- `LocalExecutionReportInputs` Debug reports only `agent_harness_hook_count`;
- `LocalExecutionWithReportResult` Debug does not leak hook IDs;
- serialized generated reports do not copy hook payload markers;
- event history is unchanged;
- report artifacts are not written automatically;
- existing WorkReport, hook, executor, artifact, evidence, diagnostic, validation, adapter telemetry, runtime, CLI, and docs tests pass through full validation.

Non-blocking test follow-up:

- A future runtime hook execution phase should add tests proving hook execution, event ordering, audit sink behavior, idempotency, and failure semantics before any executor invocation is enabled.

## 10. Documentation Review

Documentation is aligned with the implemented boundary.

Verified docs state:

- executor hook report input propagation is implemented;
- terminal report helper hook citation integration is implemented for explicit supplied IDs only;
- runtime hook execution is not implemented;
- hook workflow events are not implemented;
- audit sink emission is not implemented;
- hook persistence is not implemented;
- CLI hook behavior is not implemented;
- workflow schema fields are not implemented;
- side effects and writes are not implemented;
- recursive agents and agent swarms are not introduced.

One small documentation correction was made during this review: the implemented plan's executive summary still described the executor gap in present tense. It now describes that gap as historical.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Plan runtime hook execution before adding any executor hook invocation.
- Keep hook event ordering, idempotency, audit sink behavior, and failure semantics explicit before implementation.
- Keep hook citation section routing in `ValidationAndQualityChecks` until hook kind/status context is represented.
- Revisit public compatibility before exposing hook report input propagation through schemas, CLI, SDKs, or stable machine-readable outputs.

## 13. Recommended Next Phase

Recommended next phase: **runtime hook execution planning**.

The executor can now forward supplied hook invocation IDs into reports, but it still does not invoke hooks. The next bounded question is whether and how Workflow OS should execute deterministic hook checkpoints in a governed way, including event ordering, idempotency, audit records, failure semantics, policy boundaries, and no-write guarantees.

## 14. Validation

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.
