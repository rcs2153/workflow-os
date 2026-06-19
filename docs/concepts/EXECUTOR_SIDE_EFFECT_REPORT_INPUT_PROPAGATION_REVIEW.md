# Executor SideEffect Report Input Propagation Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted; proceed to side-effect workflow event and audit projection planning.

The implementation is narrow, additive, and consistent with the accepted plan. Executor-integrated report-bearing execution now accepts explicitly supplied `SideEffectId` values and forwards them into terminal local WorkReports without discovering, creating, resolving, persisting, auditing, executing, or mutating side effects.

No blockers were found. The phase stayed within explicit executor report input propagation scope and did not implement automatic discovery, side-effect workflow events, audit projections, persistence, EvidenceReference side-effect attachment, runtime side-effect execution, writes, provider mutations, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Verification

The phase stayed within approved executor input propagation scope.

Verified in scope:

- `LocalExecutionReportInputs` accepts `side_effect_ids: Vec<SideEffectId>`.
- `LocalExecutionReportInputs` `Debug` exposes only `side_effect_count`.
- `terminal_report_input_for_run(...)` forwards supplied SideEffect IDs into `TerminalLocalWorkReportInput`.
- Generated reports cite supplied SideEffect IDs through existing terminal report helper and `WorkReportCitation` construction.
- Existing executor behavior is preserved when no SideEffect IDs are supplied.
- Focused tests and documentation/report updates were added.

No accidental implementation was found for:

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

## 3. API Assessment

The API addition is the smallest useful executor-adjacent change:

```rust
pub side_effect_ids: Vec<SideEffectId>
```

This is the right boundary for this phase. The executor accepts only already-validated stable IDs. It does not accept raw strings, `SideEffectRecord` values, side-effect target references, summaries, reason codes, authority packets, lifecycle states, outcomes, idempotency details, provider payloads, command output, or side-effect redaction metadata.

The field is additive to `LocalExecutionReportInputs`; existing `execute(...)`, `decide_approval(...)`, and `cancel_run(...)` APIs remain unchanged.

## 4. Propagation Assessment

The propagation implementation is intentionally direct:

```rust
side_effect_ids: report.side_effect_ids.clone()
```

That keeps the executor as an explicit input bridge between `LocalExecutionReportInputs` and `TerminalLocalWorkReportInput`. It does not introduce hidden global state, runtime config, storage reads, side-effect existence checks, automatic discovery from workflow events, audit events, adapter telemetry, local check results, hook disclosures, typed handoffs, report notes, artifacts, or persistence.

This is compatible with later automatic discovery work because the current code creates a clean explicit-input path without pretending that a durable side-effect source of truth already exists.

## 5. Report Behavior Assessment

Generated reports use the existing WorkReport citation model:

- citation target: `WorkReportCitationTarget::SideEffect`;
- citation kind: `WorkReportCitationKind::SideEffect`;
- section placement: `WorkReportSectionKind::SideEffects`;
- summary behavior: bounded and generic through the terminal report helper.

The implementation is useful for future governed handoffs because executor callers can now surface explicit SideEffect references in terminal reports. It does not make reports authoritative for side-effect state. `SideEffectRecord` remains the future source of truth for intent, authority, lifecycle state, idempotency, outcomes, and related references.

When no SideEffect IDs are supplied, reports preserve existing none/skipped/unsupported side-effects section behavior. The implementation does not fabricate missing IDs or missing-citation records.

## 6. Workflow Semantics Assessment

Executor semantics remain unchanged.

Verified:

- `execute_with_report(...)` still calls `execute(&request.execution)` first.
- Execution errors are returned unchanged.
- Report generation failure after execution preserves the run.
- Report generation errors remain separate from workflow execution errors.
- The implementation does not mutate `WorkflowRun`.
- The implementation does not mutate `WorkflowRunSnapshot`.
- The implementation does not append workflow events for SideEffect input propagation.
- The implementation does not emit audit or observability events for SideEffect input propagation.
- The implementation does not create, resolve, persist, or execute SideEffect records.
- The implementation does not touch `StateBackend` beyond existing executor execution behavior.
- The implementation does not write report artifacts automatically.
- The implementation does not expose CLI output.
- The implementation does not change workflow pass/fail behavior.

Focused tests compare backend event history to the returned run events and verify no report artifacts are written automatically.

## 7. Validation And Error Handling Assessment

Validation relies on existing typed gates.

Verified:

- invalid raw SideEffect IDs fail before executor propagation because callers must construct `SideEffectId` values;
- `WorkReportCitation::new(...)` remains the citation validation boundary;
- `WorkReport::new(...)` remains the report validation boundary;
- report generation remains atomic;
- report-generation failures preserve the workflow run through the existing report-bearing result path;
- report-generation errors do not become user project diagnostics;
- no new error path includes SideEffect IDs, target references, summaries, reason codes, outcomes, authority context, idempotency details, notes, paths, tokens, raw payloads, command output, parser output, or provider data.

No leaking error surface was found.

## 8. Privacy And Redaction Assessment

The executor report input remains reference-only.

No evidence was found that the implementation copies:

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

`LocalExecutionReportInputs` `Debug` exposes only a count. `LocalExecutionWithReportResult` `Debug` remains redaction-safe. Generated WorkReport serialization includes stable SideEffect IDs when supplied, consistent with existing typed citation behavior, but does not include `SideEffectRecord` payload fields.

## 9. Relationship To Existing Concepts

The implementation remains aligned with current Workflow OS boundaries:

- Governed Work Pattern: explicit references and bounded report handoff without ambient authority.
- SideEffect: stable IDs can be cited, while SideEffect records remain separate from report input propagation.
- WorkReport: generated reports remain derived handoff artifacts, not audit logs or side-effect stores.
- EvidenceReference: no evidence references are created or attached implicitly.
- Audit and workflow events: no fabricated event or audit history is introduced.
- Adapters and writes: no runtime side-effect execution or provider mutation is enabled.

This is the last safe reference-forwarding step before the project needs a reviewed source-of-truth plan for side-effect workflow events, audit projections, and persistence.

## 10. Test Quality Assessment

Test coverage is focused and meaningful.

Covered:

- `execute_with_report(...)` forwards supplied SideEffect IDs into generated report citations;
- generated citation target is `WorkReportCitationTarget::SideEffect`;
- generated citation kind is `WorkReportCitationKind::SideEffect`;
- SideEffect citations appear only in `WorkReportSectionKind::SideEffects`;
- no SideEffect IDs leak through executor report input Debug;
- no SideEffect IDs leak through report-bearing result Debug;
- generated report serialization includes stable SideEffect citation IDs but not side-effect payload markers;
- no workflow event mutation;
- no automatic report artifact writes;
- existing WorkReport, SideEffect, executor, evidence, diagnostic, validation, adapter, hook, local check, and runtime tests through workspace validation.

No blocker-level test gaps were found.

Non-blocking test follow-ups:

- Add referential-integrity tests after a SideEffect store exists.
- Add event/audit projection tests after side-effect workflow event and audit semantics are designed.
- Add automatic discovery tests only after a durable source of truth exists.

## 11. Documentation Review

Documentation now states:

- executor SideEffect report input propagation is implemented for explicitly supplied `SideEffectId` values;
- automatic SideEffect citation discovery is not implemented;
- SideEffect persistence is not implemented;
- side-effect workflow events are not implemented;
- side-effect audit projections are not implemented;
- EvidenceReference side-effect attachment is not implemented;
- runtime side-effect execution is not implemented;
- write-capable adapters are not implemented;
- provider mutations are not implemented;
- report artifact behavior changes are not implemented;
- schemas are not implemented;
- CLI behavior is not implemented;
- examples are not updated;
- hosted or distributed runtime behavior is not claimed;
- reasoning lineage is not implemented;
- release posture is unchanged.

The end-of-phase report accurately records implementation scope, validation results, limitations, and the recommendation to review before broader side-effect phases.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Plan side-effect workflow events and audit projections before automatic SideEffect discovery.
- Plan SideEffect persistence and referential integrity before persisted/public report artifacts rely on SideEffect citation resolution.
- Keep EvidenceReference side-effect attachment separate from WorkReport SideEffect citations.
- Keep approval-side-effect linkage separate from this executor input propagation path.
- Revisit whether approval-resume and cancellation report-bearing APIs should accept SideEffect references after those report-bearing APIs exist.

## 14. Recommended Next Phase

Recommended next phase: side-effect workflow event and audit projection planning.

The executor can now forward explicit SideEffect IDs into reports, but automatic discovery would be premature without reviewed event and audit semantics. The next plan should define how SideEffect intent, approval, attempt, completion, denial, failure, skip, and compensation vocabulary should appear in workflow events and audit projections, while continuing to defer persistence, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, and release posture changes.

## 15. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
