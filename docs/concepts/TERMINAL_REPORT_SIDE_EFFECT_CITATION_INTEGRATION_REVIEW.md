# Terminal Report SideEffect Citation Integration Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted; proceed to executor SideEffect report input propagation planning.

The implementation adds a narrow, explicit terminal report helper path for supplied `SideEffectId` values. Generated terminal local `WorkReport` values now cite those IDs in the required side-effects section through existing `WorkReportCitation` construction, while preserving the previous none/skipped/unsupported side-effects text when no IDs are supplied.

No blockers were found. The phase stayed within terminal-helper-only scope and did not implement executor propagation, automatic discovery, side-effect persistence, workflow events, audit projections, EvidenceReference side-effect attachment, runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved terminal report helper propagation scope.

Verified in scope:

- `TerminalLocalWorkReportInput` accepts explicit `SideEffectId` values.
- Generated terminal local reports place supplied SideEffect citations only in `WorkReportSectionKind::SideEffects`.
- SideEffect citations are created through existing `WorkReportCitation::new(...)` validation.
- The existing no-SideEffect-input behavior is preserved.
- Citation summaries are bounded and generic.
- Tests and docs were updated.
- The end-of-phase report exists.

No accidental implementation was found for:

- executor SideEffect ID propagation;
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

## 3. Helper Input Assessment

The helper input change is appropriately small and typed.

`TerminalLocalWorkReportInput` now includes:

```rust
pub side_effect_ids: Vec<SideEffectId>
```

This is the right boundary for this phase. The helper accepts only already-validated stable IDs. It does not accept `SideEffectRecord`, raw target references, reason codes, outcomes, authority packets, idempotency details, provider payloads, command output, or side-effect redaction metadata.

The implementation therefore keeps `SideEffectRecord` lifecycle and payload state outside the terminal report helper, while still allowing reports to cite existing side-effect records when a caller can supply stable IDs.

## 4. Citation Construction Assessment

Citation construction is correct and bounded.

Verified:

- citations use `WorkReportCitationTarget::SideEffect { side_effect_id }`;
- citations are built through `WorkReportCitation::new(...)`;
- citation kind maps deterministically to `WorkReportCitationKind::SideEffect`;
- citation summaries use bounded generic text;
- existing report sensitivity and redaction metadata validation applies;
- citation ordering preserves input order deterministically;
- no `EvidenceReference` values are created implicitly;
- no `SideEffectRecord` values are created, resolved, mutated, persisted, or executed.

This is compatible with future executor propagation because the helper now has a clear explicit input field without requiring any automatic runtime discovery.

## 5. Section Placement Assessment

SideEffect citations are placed only in the side-effects section.

Verified:

- generated citations appear in `WorkReportSectionKind::SideEffects`;
- generated citations are not placed in evidence considered, policy gates, approvals, validation, handoff notes, or artifact metadata;
- absent SideEffect IDs preserve the prior summary:

```text
No write side effects are supported; side effects are none, skipped, or unsupported.
```

- supplied SideEffect IDs use the bounded summary:

```text
Side-effect records were supplied as stable references; no side-effect payloads are copied.
```

The summary does not claim that writes are supported, attempted, approved, completed, denied, or failed. The cited `SideEffectRecord` remains the source of truth for those facts.

## 6. Source-Of-Truth Assessment

The implementation preserves source-of-truth boundaries.

Verified:

- `SideEffectRecord` remains the source of truth for side-effect intent, authority, lifecycle state, idempotency, outcome references, and related references.
- `WorkReport` remains a governed handoff artifact that cites stable references.
- Workflow events remain the source of truth for run state.
- Audit records remain governance and operational projections.
- EvidenceReference remains the evidence citation substrate.

The report helper does not attempt to verify that a supplied `SideEffectId` resolves to an existing record. That is acceptable for this phase because resolution and persistence remain out of scope.

## 7. Runtime And State Boundary Assessment

Runtime and state boundaries remain clean.

Verified:

- the helper does not mutate `WorkflowRun`;
- the helper does not mutate `WorkflowRunSnapshot`;
- the helper does not append workflow events;
- the helper does not emit audit events;
- the helper does not emit observability events;
- the helper does not touch a `StateBackend`;
- the helper does not persist side effects;
- the helper does not persist reports;
- the helper does not write files;
- the helper does not create report artifacts;
- the helper does not expose CLI output;
- the helper does not invoke adapters;
- the helper does not execute provider mutations;
- the helper does not run local commands;
- workflow status and pass/fail semantics are unchanged.

Executor-integrated report-bearing execution currently forwards an empty SideEffect ID list. That is the correct boundary until executor report input propagation is separately planned and reviewed.

## 8. Validation And Error Handling Assessment

Validation relies on existing typed model gates.

Verified:

- `SideEffectId` validation happens before invalid IDs can be supplied as typed inputs.
- `WorkReportCitation::new(...)` validates citation summaries, redaction metadata, sensitivity, and target shape.
- `WorkReport::new(...)` preserves complete-report validation.
- Report construction remains atomic: citation construction failure prevents a report from being returned.
- Report-generation errors remain structured `WorkflowOsError` values and do not become user project diagnostics.
- The implementation does not introduce error paths that include side-effect payload fields.

No evidence was found that errors leak target references, side-effect summaries, reason codes, outcomes, command output, provider payloads, snippets, or secret-like values.

## 9. Privacy And Redaction Assessment

The privacy posture is acceptable for this phase.

Verified the helper does not copy:

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

`WorkReport` Debug output remains redaction-safe through existing report and citation Debug implementations. Serialization contains stable `SideEffectId` values when supplied, matching existing typed citation behavior, but it does not include `SideEffectRecord` payload fields.

## 10. Test Quality Assessment

The added tests are focused and meaningful.

Covered:

- generated reports cite supplied SideEffect IDs in the side-effects section;
- generated SideEffect citations use `WorkReportCitationTarget::SideEffect`;
- generated SideEffect citations use `WorkReportCitationKind::SideEffect`;
- SideEffect citation ordering is deterministic;
- absent SideEffect IDs preserve existing none/skipped/unsupported text and no citations;
- generated reports do not copy side-effect target, summary, reason, outcome, idempotency, provider payload, command output, spec, or secret-like markers;
- existing terminal report helper behavior continues to pass;
- existing workspace validation passes.

No shallow or placeholder tests were found.

Non-blocking test hardening opportunity: when executor propagation is planned, add executor-level tests proving executor-generated reports continue to omit SideEffect citations until explicit report inputs supply them, and then prove explicit propagation once implemented.

## 11. Documentation Review

Documentation now states:

- terminal report helper SideEffect citation propagation is implemented for explicit helper inputs;
- executor SideEffect ID propagation is not implemented;
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

The phase report accurately records the implementation boundary and recommends review before executor propagation.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Plan executor SideEffect report input propagation as a separate phase.
- Keep automatic SideEffect discovery deferred until workflow events, audit projections, or persistence provide a reviewed source of truth.
- Consider a later resolver/persistence phase that can validate whether a cited `SideEffectId` exists before persisted artifacts or public schemas expose reports.
- Keep EvidenceReference side-effect attachment separate from WorkReport SideEffect citations.
- Add executor-level non-regression coverage once executor propagation is implemented.

## 14. Recommended Next Phase

Recommended next phase: executor SideEffect report input propagation planning.

The terminal helper can now cite explicitly supplied SideEffect IDs safely. The next narrow step is to plan whether `LocalExecutionReportInputs` should accept SideEffect IDs and forward them through `LocalExecutor::execute_with_report(...)`, while preserving existing executor semantics and continuing to avoid automatic discovery, persistence, workflow events, audit projections, runtime side-effect execution, writes, schemas, CLI behavior, examples, reasoning lineage, and release posture changes.

## Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
