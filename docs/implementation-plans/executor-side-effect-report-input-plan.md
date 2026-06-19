# Executor SideEffect Report Input Propagation Plan

Status: Implemented and reviewed. Terminal report helper SideEffect citation propagation is implemented and accepted for explicitly supplied `SideEffectId` values, and executor-integrated report-bearing execution now accepts and forwards caller-supplied SideEffect IDs into terminal reports. This plan defined the narrow executor-integrated input propagation phase and is accepted in [Executor SideEffect Report Input Propagation Review](../concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REVIEW.md). SideEffect workflow event and audit projection planning is documented in [SideEffect Workflow Event And Audit Projection Plan](side-effect-workflow-event-audit-projection-plan.md). It does not implement automatic SideEffect discovery, SideEffect persistence, side-effect workflow events, audit projections, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

The SideEffect core model is implemented and accepted. WorkReport side-effect citation vocabulary is implemented and accepted. The terminal local report helper can now cite explicitly supplied `SideEffectId` values in generated in-memory reports.

The executor gap was narrow: `LocalExecutionReportInputs` did not accept SideEffect IDs, and the executor report path passed an empty SideEffect ID list into `TerminalLocalWorkReportInput`.

This plan defined the smallest safe implementation for explicit executor-integrated SideEffect report input propagation. The implementation adds one typed field to executor report inputs and forwards it into the existing terminal report helper. It does not discover, create, resolve, persist, execute, audit, or attach side effects automatically.

## 2. Goals

- Let callers of `LocalExecutor::execute_with_report(...)` supply existing `SideEffectId` values.
- Forward those IDs from `LocalExecutionReportInputs` to `TerminalLocalWorkReportInput`.
- Preserve executor semantics and existing `execute(...)` behavior.
- Keep SideEffect citations explicit, local, deterministic, and in-memory.
- Use existing `SideEffectId`, `WorkReportCitation`, and terminal report helper validation boundaries.
- Avoid copying full `SideEffectRecord` values or side-effect payload fields into executor inputs, report sections, citations, debug output, or serialization.
- Preserve current behavior when no SideEffect IDs are supplied.
- Prepare for later side-effect event, audit, persistence, and EvidenceReference phases without implementing them here.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic SideEffect citation discovery;
- SideEffect record creation;
- SideEffect record resolution;
- SideEffect persistence;
- side-effect workflow events;
- side-effect audit projections;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- rollback or compensation behavior;
- report artifact behavior changes;
- workflow schema fields;
- CLI rendering or export;
- example updates;
- hosted or distributed runtime claims;
- reasoning lineage implementation;
- release posture changes.

## 4. Current Baseline

Implemented:

- `SideEffectId`;
- `SideEffectRecord`;
- `SideEffectLifecycleState`;
- `SideEffectAuthority`;
- `SideEffectIdempotencyBinding`;
- `SideEffectReference`;
- `WorkReportCitationKind::SideEffect`;
- `WorkReportCitationTarget::SideEffect { side_effect_id: SideEffectId }`;
- terminal local WorkReport generation helper;
- terminal report helper support for supplied `side_effect_ids`;
- executor-integrated report-bearing execution through `LocalExecutor::execute_with_report(...)`.

Closed gap:

- `LocalExecutionReportInputs` now has a `side_effect_ids` field.
- `terminal_report_input_for_run(...)` now forwards `report.side_effect_ids.clone()`.
- Therefore executor-integrated report-bearing execution can forward caller-supplied SideEffect references into generated reports.

The gap is input propagation only. It is not runtime side-effect execution.

## 5. Proposed API Change

The implementation added one field to `LocalExecutionReportInputs`:

```rust
pub side_effect_ids: Vec<SideEffectId>
```

Rules:

- accept only already-validated `SideEffectId` values;
- do not accept raw strings;
- do not accept `SideEffectRecord` values;
- do not accept side-effect target references, summaries, reason codes, authority packets, lifecycle states, outcomes, idempotency details, or redaction metadata;
- do not read side effects from storage;
- do not validate SideEffect record existence;
- do not infer IDs from workflow events, audit events, adapter telemetry, local check results, hook disclosures, typed handoffs, report notes, or artifacts;
- do not fabricate missing SideEffect IDs.

This is an additive local Rust API change. It must preserve existing executor methods and should update tests and call sites that construct `LocalExecutionReportInputs`.

## 6. Propagation Boundary

The implementation updated `terminal_report_input_for_run(...)` so:

```rust
side_effect_ids: report.side_effect_ids.clone()
```

This should be the only intended behavior change.

The executor should still:

- call `execute(&request.execution)` first;
- return execution errors unchanged;
- preserve `run` when report generation fails after execution;
- never mutate workflow state for report generation;
- never append events for report generation;
- never create SideEffect records;
- never resolve SideEffect records;
- never persist SideEffect records;
- never execute side effects;
- never write report artifacts automatically;
- never expose CLI output.

## 7. Debug And Redaction Policy

The implementation updated `LocalExecutionReportInputs` `Debug` to include only a count:

```rust
.field("side_effect_count", &self.side_effect_ids.len())
```

Debug output must not include SideEffect IDs, report IDs, paths, tokens, notes, limitations, risks, handoff text, side-effect target references, reason codes, outcomes, authority context, idempotency details, or payload-like strings.

Serialization is not currently the primary surface for executor report inputs. If serialization is added later, SideEffect IDs must be treated as potentially sensitive stable references and must not be mixed with side-effect payload fields.

## 8. Report Behavior

When SideEffect IDs are supplied:

- generated reports should include SideEffect citations in `WorkReportSectionKind::SideEffects`;
- citation target should be `WorkReportCitationTarget::SideEffect`;
- citation kind should be `WorkReportCitationKind::SideEffect`;
- citation summary should remain bounded and generic;
- the side-effects section summary should remain the existing terminal helper text for supplied SideEffect IDs;
- no `SideEffectRecord` value should be created, resolved, summarized, or copied.

When no SideEffect IDs are supplied:

- generated reports should preserve current behavior;
- no SideEffect citation should be added;
- no missing SideEffect citation should be invented by default;
- absence of SideEffect citations must not be treated as proof that no side effect existed.

## 9. Workflow Semantics Boundary

The implementation must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- touch `StateBackend` beyond existing executor behavior;
- write report artifacts;
- persist side effects;
- read side effects from storage;
- execute provider mutations;
- invoke adapters because a SideEffect ID was supplied;
- expose CLI output;
- change workflow pass/fail behavior;
- change existing `execute(...)`, `decide_approval(...)`, or `cancel_run(...)` behavior.

Report generation failure must remain separate from workflow execution failure in `LocalExecutionWithReportResult`.

## 10. Error Handling

Because the new input field should use `SideEffectId`, invalid raw ID values fail before the executor propagation boundary.

If report generation fails after SideEffect IDs are supplied:

- preserve the run;
- return `work_report: None`;
- return `report_generation_error: Some(...)`;
- avoid leaking SideEffect IDs, target references, summaries, reason codes, outcomes, authority context, idempotency details, notes, paths, tokens, raw payloads, command output, parser output, or provider data in errors;
- do not convert report-generation errors into workflow diagnostics;
- do not append events or audit records for the report-generation failure.

## 11. Privacy And Payload Policy

Executor report inputs must remain reference-only.

The implementation must not copy:

- side-effect target references;
- side-effect summaries;
- side-effect reason codes;
- side-effect authority context;
- side-effect lifecycle payloads;
- side-effect outcome references;
- side-effect idempotency details;
- side-effect redaction metadata;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

SideEffect IDs are stable references. Future artifact, schema, CLI, SDK, or hosted exposure must treat them as sensitive enough to redact in `Debug` and review before public contract exposure.

## 12. Relationship To Runtime Side-Effect Execution

This propagation phase must not execute side effects.

The executor should consume IDs supplied by a caller. It should not:

- evaluate side-effect authority;
- transition side-effect lifecycle state;
- create `SideEffectRecord` values;
- attempt provider writes;
- invoke write-capable adapters;
- check external side-effect status;
- read local files because a SideEffect ID exists;
- call external systems;
- enable any default write or side-effect profile.

That keeps mutation authority separate from report construction.

## 13. Relationship To Audit And Workflow Events

SideEffect IDs should not be modeled as `AuditEvent` or `WorkflowEvent` citations in this phase.

Reasons:

- side-effect workflow events are not implemented;
- side-effect audit projections are not implemented;
- side-effect persistence is not implemented;
- using audit or workflow event citations for SideEffect IDs would fabricate runtime history.

Future side-effect event phases should separately define event kinds, ordering, idempotency, state projection, audit sink behavior, and referential integrity.

## 14. Relationship To EvidenceReference

This propagation phase must not create or attach `EvidenceReference` values.

SideEffect citations are WorkReport citations, not evidence records. EvidenceReference side-effect attachment remains deferred until a separate phase decides how side-effect records can be cited as evidence without copying unsafe payloads.

## 15. Relationship To Report Artifacts

This propagation phase must not change report artifact behavior.

Generated reports may contain SideEffect citations when explicit IDs are supplied, but executor paths must not automatically write artifacts. Artifact storage, referential integrity checks, and artifact exposure remain separate phases.

## 16. Test Plan

Implementation tests cover:

- `LocalExecutionReportInputs` accepts `side_effect_ids`;
- `execute_with_report(...)` forwards supplied SideEffect IDs into generated report citations;
- generated citation target is `WorkReportCitationTarget::SideEffect`;
- generated citation kind is `WorkReportCitationKind::SideEffect`;
- SideEffect citations appear only in `WorkReportSectionKind::SideEffects`;
- absent SideEffect IDs preserve current none/skipped/unsupported behavior;
- `LocalExecutionReportInputs` Debug reports only count and does not leak SideEffect IDs;
- `LocalExecutionWithReportRequest` Debug remains redaction-safe;
- report-generation failure preserves run and event history;
- no runtime events are appended for SideEffect input propagation;
- no `StateBackend` write beyond existing executor execution behavior;
- no report artifacts are written automatically;
- no `SideEffectRecord` value is created, resolved, or copied;
- no side-effect payload markers appear in Debug or serialized generated reports;
- no runtime side-effect execution, adapter invocation, provider mutation, or local command execution occurs;
- existing WorkReport, SideEffect, executor, artifact, evidence, diagnostic, validation, adapter telemetry, hook, local check, and runtime tests still pass.

## 17. Proposed Implementation Sequence

1. Add `side_effect_ids: Vec<SideEffectId>` to `LocalExecutionReportInputs`.
2. Import/export `SideEffectId` in executor code as needed.
3. Update `LocalExecutionReportInputs` Debug with `side_effect_count`.
4. Forward `report.side_effect_ids.clone()` in `terminal_report_input_for_run(...)`.
5. Update existing test helpers that construct `LocalExecutionReportInputs`.
6. Add focused executor tests for propagation, redaction-safe Debug, absence behavior, and non-mutation.
7. Update docs and create an end-of-phase report.
8. Run full validation.

## 18. Open Questions

- Should executor-integrated SideEffect propagation eventually validate that supplied IDs exist in a future SideEffect store?
- Should missing SideEffect references ever become explicit `missing=true` citations, or remain section text unless contract requirements change?
- Should future report artifacts verify referential integrity for SideEffect citations?
- Should SideEffect IDs appear in CLI JSON only after schema and redaction posture are separately reviewed?
- Should approval-resume and cancellation report-bearing APIs receive the same SideEffect input later?
- Should future side-effect workflow events or audit projections be the preferred source for automatic citation discovery?

## 19. Final Recommendation

Recommended next phase after review: side-effect workflow event and audit projection planning.

The implementation added only the explicit input field and forwarding behavior described here. The review accepted this phase and recommends planning side-effect workflow event and audit projection semantics before automatic discovery, SideEffect creation, SideEffect resolution, SideEffect persistence, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, provider mutations, report artifact changes, CLI rendering, workflow schema changes, reasoning lineage, hosted behavior, or release posture changes.

## 20. Validation

For implementation, run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

For this planning phase, run:

- `npm run check:docs`.
