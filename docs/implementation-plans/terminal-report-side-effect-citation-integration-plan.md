# Terminal Report SideEffect Citation Integration Plan

Status: Implemented and accepted for terminal report helper propagation. Planning was accepted in [Terminal Report SideEffect Citation Integration Plan Review](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_PLAN_REVIEW.md), implementation is documented in [Terminal Report SideEffect Citation Integration Report](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REPORT.md), and review is documented in [Terminal Report SideEffect Citation Integration Review](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md). Executor propagation is implemented in [Executor SideEffect Report Input Propagation Report](../concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md). WorkReport SideEffect citation vocabulary is implemented and accepted, and `TerminalLocalWorkReportInput` now accepts explicit `SideEffectId` values. This does not implement automatic SideEffect discovery, report artifact behavior changes, persistence, schemas, CLI behavior, examples, runtime side-effect execution, write-capable adapters, provider mutations, or release posture changes.

## 1. Executive Summary

The SideEffect core model is implemented and accepted as a domain-neutral boundary for future governed mutation intent, authority, lifecycle state, idempotency, and report citation. WorkReport can now cite a side-effect record by stable `SideEffectId` through `WorkReportCitationTarget::SideEffect`.

The remaining terminal report helper gap is narrow: `TerminalLocalWorkReportInput` does not yet accept explicitly supplied `SideEffectId` values, and generated terminal reports always populate the side-effects section with explicit none/skipped/unsupported text and no SideEffect citations.

The implemented helper integration allows callers to pass already-existing `SideEffectId` values to the in-memory terminal local report helper and places those citations in the `SideEffects` section through existing `WorkReportCitation` constructors.

This plan and implementation do not implement executor propagation, automatic discovery, side-effect record creation, persistence, runtime side-effect execution, writes, CLI behavior, schemas, examples, hosted behavior, or release posture changes.

## 2. Goals

- Allow terminal report helper callers to supply existing `SideEffectId` values.
- Cite supplied SideEffect IDs through existing `WorkReportCitation` constructors.
- Place SideEffect citations in `WorkReportSectionKind::SideEffects`.
- Preserve current terminal report behavior when no SideEffect IDs are supplied.
- Preserve local, deterministic, in-memory report generation.
- Preserve workflow pass/fail semantics and existing executor result behavior.
- Avoid copying side-effect payload fields into report sections.
- Keep `SideEffectRecord` as the source of truth for side-effect lifecycle, authority, idempotency, and outcome references.
- Prepare for later executor report input propagation after helper integration is implemented and reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- executor report input propagation;
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
- required WorkReport side-effects section;
- in-memory terminal local report generation helper;
- runtime result report exposure helper;
- executor-integrated report-bearing execution;
- explicit local report artifact store.

Implemented after this plan:

- `TerminalLocalWorkReportInput` SideEffect ID input;
- generated terminal report SideEffect citations in `SideEffects`;

Not implemented:

- `LocalExecutionReportInputs` SideEffect ID propagation;
- automatic discovery from workflow events, audit records, stores, adapter telemetry, local checks, hooks, or side-effect persistence;
- side-effect persistence, workflow events, audit projections, runtime execution, provider mutations, or writes.

## 5. Proposed Helper Input Shape

The implementation adds one explicit input field to `TerminalLocalWorkReportInput`:

```rust
pub side_effect_ids: Vec<SideEffectId>
```

Rules:

- accept only already-constructed `SideEffectId` values;
- do not accept `SideEffectRecord` values;
- do not accept raw side-effect target references;
- do not accept side-effect summaries, reason codes, outcomes, authority packets, or idempotency bindings;
- do not read side effects from storage;
- do not validate that a cited SideEffect record exists;
- do not infer SideEffect IDs from workflow events, audit events, report notes, local check results, hook disclosures, or adapter telemetry;
- do not fabricate missing SideEffect IDs.

Using `SideEffectId` keeps validation at the existing typed ID boundary and avoids a generic string reference.

## 6. Citation Construction Policy

Helper integration constructs citations with:

```rust
WorkReportCitationTarget::SideEffect {
    side_effect_id,
}
```

Rules:

- use `WorkReportCitation::new(...)`;
- use bounded, generic summary text only, such as `Side-effect record reference considered.`;
- use the report helper's existing sensitivity and redaction metadata;
- fail safely if citation construction fails;
- return structured, non-leaking report-generation errors;
- do not create `EvidenceReference` values;
- do not create, resolve, mutate, persist, or execute `SideEffectRecord` values;
- do not copy target references, outcome references, authority details, reason codes, idempotency details, provider payloads, command output, logs, specs, parser payloads, environment values, credentials, or token-like values.

## 7. Section Placement Policy

SideEffect citations are placed only in:

```rust
WorkReportSectionKind::SideEffects
```

Rationale:

- the required side-effects section already exists for proposed, approved, attempted, completed, skipped, denied, failed, unsupported, or not-available side-effect disclosures;
- placing SideEffect citations there avoids confusing side effects with evidence payloads, policy decisions, approvals, or validation checks;
- future approval, policy, evidence, and audit phases can cite their own source records separately.

Do not place SideEffect citations in:

- `EvidenceConsidered`;
- `PolicyGatesEvaluated`;
- `Approvals`;
- `ValidationAndQualityChecks`;
- `OperatorHandoffNotes`;
- report artifact metadata.

## 8. Section Summary Behavior

When no SideEffect IDs are supplied, preserve the existing explicit side-effects section text:

```text
No write side effects are supported; side effects are none, skipped, or unsupported.
```

When SideEffect IDs are supplied, the generated side-effects section uses bounded, non-payload text:

```text
Side-effect records were supplied as stable references; no side-effect payloads are copied.
```

That text must not imply that writes are supported, attempted, approved, or completed. The cited `SideEffectRecord` remains the source of truth for lifecycle state.

## 9. Missing And Unavailable References

If no SideEffect IDs are supplied:

- keep existing section text behavior;
- do not add missing SideEffect citations by default;
- do not imply a SideEffect was expected;
- do not treat absence of citations as proof that no side effects existed.

If a caller expected a SideEffect citation but cannot supply a stable ID:

- record the limitation in bounded incomplete-work, known-limitation, risk, or handoff-note inputs if appropriate;
- do not fabricate a SideEffect ID;
- do not resolve from stores;
- do not create an explicit missing citation unless a later missing-citation phase adds that requirement.

## 10. Source-Of-Truth Rules

- `SideEffectRecord` remains the source of truth for side-effect intent, authority, lifecycle state, idempotency, outcome references, and related references.
- WorkReport cites side effects; it does not recreate or validate side-effect facts.
- Workflow events remain the source of truth for run state.
- Audit events remain governance and operational projections.
- EvidenceReference remains the evidence citation substrate.
- Report text remains a governed handoff summary, not a durable side-effect ledger.

## 11. Runtime And State Boundary

The helper integration must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- touch a `StateBackend`;
- persist side effects;
- persist reports;
- write files;
- create report artifacts;
- expose CLI output;
- invoke adapters;
- execute provider mutations;
- run local commands;
- change workflow status or pass/fail semantics.

Executor propagation should remain a separate phase after terminal helper propagation is implemented and reviewed.

## 12. Validation And Error Handling

Validation should rely on:

- `SideEffectId` constructor-backed validation;
- existing `WorkReportCitation::new(...)` validation;
- existing WorkReport summary, sensitivity, and redaction validation;
- existing terminal report helper error behavior.

Rules:

- invalid or secret-like `SideEffectId` values fail before or during report construction;
- citation construction failure returns a structured non-leaking report-generation error;
- errors must use stable codes and must not include raw SideEffect IDs, target references, side-effect summaries, reason codes, outcome references, paths, snippets, command output, provider payloads, or secret-like values;
- report generation should remain atomic: if any SideEffect citation fails construction, no partial report should be returned;
- report generation failure must not become a user project diagnostic and must not change workflow run status.

## 13. Privacy And Redaction

The helper integration must not copy:

- side-effect target references;
- side-effect summaries;
- side-effect reason codes;
- side-effect authority context;
- side-effect outcome references;
- side-effect idempotency details;
- side-effect redaction metadata;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue/comment bodies;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

Debug output must remain redaction-safe through existing `WorkReport` and citation `Debug` implementations. Serialization may include valid `SideEffectId` values as stable references, matching existing typed citation targets, but must not include `SideEffectRecord` payload fields.

## 14. Relationship To Executor Propagation

This plan covers terminal report helper propagation only.

Do not change:

- `LocalExecutionReportInputs`;
- `LocalExecutionWithReportRequest`;
- `LocalExecutor::execute_with_report(...)`;
- executor hook checkpoint behavior;
- runtime result exposure helper behavior;
- local report artifact store behavior.

After helper integration is implemented and reviewed, a separate executor propagation phase may add explicit `side_effect_ids` to executor report inputs and forward them into `TerminalLocalWorkReportInput`.

## 15. Relationship To EvidenceReference

This integration must not attach EvidenceReference values.

SideEffect citations are WorkReport citations, not evidence records. EvidenceReference side-effect attachment remains deferred until a separate phase decides how side effects should be represented as evidence without copying unsafe payloads.

## 16. Test Plan For Future Implementation

Implementation tests cover:

- terminal report helper accepts supplied `SideEffectId` values;
- generated report cites SideEffect IDs in `SideEffects`;
- SideEffect citations use `WorkReportCitationTarget::SideEffect`;
- SideEffect citations map to `WorkReportCitationKind::SideEffect`;
- absence of SideEffect IDs preserves existing none/skipped/unsupported side-effects section text;
- supplied SideEffect IDs change only the side-effects section citations and bounded summary;
- helper does not create, resolve, mutate, persist, or execute `SideEffectRecord` values;
- helper does not recreate `EvidenceReference` values;
- helper does not copy side-effect target references, reason codes, summaries, outcomes, authority details, idempotency details, or redaction metadata;
- helper errors do not leak secret-like values;
- Debug output does not leak SideEffect IDs;
- serialization does not copy SideEffect record payload fields;
- existing terminal report helper tests still pass;
- existing executor-integrated report tests still pass;
- existing WorkReport, SideEffect, EvidenceReference, Diagnostic, validation, adapter telemetry, local check, hook, typed handoff, and runtime tests still pass.

## 17. Proposed Implementation Sequence

1. Add `side_effect_ids: Vec<SideEffectId>` to `TerminalLocalWorkReportInput`. Completed.
2. Add bounded SideEffect citation construction alongside existing evidence, validation, local check, hook, disclosure, typed handoff, policy, and approval citation helpers. Completed.
3. Attach SideEffect citations to `WorkReportSectionKind::SideEffects`. Completed.
4. Preserve existing side-effects section text when no IDs are supplied. Completed.
5. Add focused helper tests. Completed.
6. Update docs and create an end-of-phase report. Completed.
7. Review.
8. Only after review, plan executor-integrated report input propagation for SideEffect IDs.

## 18. Deferred Work

Deferred:

- implementation in this planning phase;
- executor report input propagation;
- report artifact behavior changes;
- automatic discovery from workflow events, audit events, stores, adapter telemetry, local checks, hooks, disclosures, or side-effect persistence;
- side-effect workflow events;
- side-effect audit projections;
- side-effect persistence;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- rollback and compensation behavior;
- schemas;
- CLI behavior;
- examples;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 19. Final Recommendation

Recommended next phase: executor SideEffect report input propagation review.

The implementation is accepted in [Terminal Report SideEffect Citation Integration Review](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md). Executor propagation is implemented in [Executor SideEffect Report Input Propagation Report](../concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md): the executor report input now accepts explicit supplied `SideEffectId` values and forwards them into the terminal helper while preserving existing absent-reference behavior.

Do not implement automatic discovery, side-effect persistence, workflow events, audit projections, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, provider mutations, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes until separately scoped and reviewed.

## 20. Validation

For implementation, run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`.
- `npm run check:docs`;
- `git diff --check`.

## 21. Acceptance Criteria

- The plan identifies a narrow terminal-helper-only SideEffect citation propagation boundary.
- The plan uses explicit `SideEffectId` inputs only.
- The plan places SideEffect citations in the side-effects section.
- The plan preserves absent-reference behavior.
- The plan avoids SideEffect payload copying.
- The plan does not authorize executor propagation, automatic discovery, persistence, CLI, schemas, examples, runtime side-effect execution, writes, or release posture changes.
- The next phase is clearly identified.
