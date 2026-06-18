# Terminal Report SideEffect Citation Integration Plan Review

Review date: 2026-06-17

## 1. Executive Verdict

Plan accepted; proceed to terminal report SideEffect citation propagation implementation.

The plan defines a narrow, reviewable terminal-helper-only implementation: accept explicitly supplied `SideEffectId` values, construct `WorkReportCitationTarget::SideEffect` citations through existing `WorkReportCitation` validation, and place them in the required `SideEffects` section. It preserves the current explicit none/skipped/unsupported behavior when no IDs are supplied.

No planning blocker was found.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- helper implementation in the planning phase;
- executor report input propagation;
- automatic SideEffect discovery;
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

The plan explicitly keeps executor propagation as a later phase after terminal helper propagation is implemented and reviewed.

## 3. Helper Input Assessment

The proposed future input shape is appropriate:

```rust
pub side_effect_ids: Vec<SideEffectId>
```

This matches existing terminal report helper patterns for explicit typed inputs such as validation references, local check references, typed handoff IDs, hook invocation IDs, and hook disclosure IDs. It also avoids introducing a generic string reference or accepting a full `SideEffectRecord`.

The plan correctly says the helper should not accept side-effect target references, summaries, reason codes, outcomes, authority packets, idempotency bindings, or redaction metadata.

## 4. Citation Construction Assessment

The citation construction policy is sound.

The plan requires:

- `WorkReportCitationTarget::SideEffect { side_effect_id }`;
- `WorkReportCitation::new(...)`;
- bounded generic summary text;
- existing report sensitivity and redaction metadata;
- no `EvidenceReference` creation;
- no `SideEffectRecord` creation, mutation, resolution, persistence, or execution.

This keeps the new helper behavior aligned with the already-accepted WorkReport SideEffect citation vocabulary and avoids creating a side-effect payload channel through report text.

## 5. Section Placement Assessment

The plan places SideEffect citations only in `WorkReportSectionKind::SideEffects`.

That is the correct first placement. Side-effect references should not be mixed into evidence, policy, approval, validation, or handoff sections until separate phases define those relationships.

The proposed section summary behavior is also appropriate:

- no supplied IDs preserves existing none/skipped/unsupported text;
- supplied IDs use bounded reference-only text;
- summary text must not imply writes are supported, attempted, approved, or completed.

## 6. Missing Reference Assessment

The plan handles absence conservatively.

If no SideEffect IDs are supplied, the helper should:

- keep existing section text;
- add no missing SideEffect citation by default;
- avoid implying a SideEffect was expected;
- avoid treating absence of citations as proof no side effect existed.

This is consistent with the report/audit/missing-citation semantics hardening already accepted for WorkReport.

## 7. Source-Of-Truth Assessment

The plan preserves source-of-truth boundaries:

- `SideEffectRecord` remains authoritative for side-effect intent, authority, lifecycle state, idempotency, outcomes, and related references.
- WorkReport remains a governed terminal handoff artifact.
- Workflow events remain authoritative for run state.
- Audit events remain operational/governance projections.
- EvidenceReference remains the evidence citation substrate.

The plan correctly avoids making report prose or report citations into the durable side-effect ledger.

## 8. Runtime And State Boundary Assessment

The runtime boundary is clean.

The future implementation is explicitly forbidden from:

- mutating `WorkflowRun`;
- mutating `WorkflowRunSnapshot`;
- appending workflow events;
- emitting audit or observability events;
- touching `StateBackend`;
- persisting side effects;
- persisting reports;
- writing files;
- creating report artifacts;
- exposing CLI output;
- invoking adapters;
- executing provider mutations;
- running local commands;
- changing workflow status or pass/fail semantics.

That is the right boundary for a terminal helper propagation phase.

## 9. Validation And Error-Handling Assessment

The plan uses existing validation boundaries:

- `SideEffectId` constructor-backed validation;
- `WorkReportCitation::new(...)`;
- existing WorkReport summary, sensitivity, and redaction validation;
- existing terminal report helper error behavior.

The planned atomic failure behavior is also appropriate: if any SideEffect citation cannot be constructed, report generation should fail rather than return a partial report. Report-generation failure must remain separate from workflow execution state and must not become a user project diagnostic.

The plan requires stable, non-leaking errors and forbids raw IDs, target references, side-effect summaries, reasons, outcomes, paths, snippets, provider payloads, command output, and secret-like values in errors.

## 10. Privacy And Redaction Assessment

The privacy posture is acceptable.

The plan forbids copying:

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

Serialization may include valid `SideEffectId` values as stable references, matching existing typed citation behavior. The plan correctly requires that serialized reports must not include `SideEffectRecord` payload fields.

## 11. Relationship To Executor Propagation

The plan correctly defers executor propagation.

It does not change:

- `LocalExecutionReportInputs`;
- `LocalExecutionWithReportRequest`;
- `LocalExecutor::execute_with_report(...)`;
- executor hook checkpoint behavior;
- runtime result exposure helper behavior;
- local report artifact store behavior.

That keeps the next implementation small: terminal helper input plus section citation population only. Executor report input propagation can follow after helper behavior is tested and reviewed.

## 12. Relationship To EvidenceReference

The plan correctly avoids EvidenceReference attachment.

SideEffect citations are WorkReport citations, not evidence records. EvidenceReference side-effect attachment should remain a separate phase because it needs a policy for citing side-effect intent, authority, outcomes, and provider context without copying unsafe payloads.

## 13. Test Plan Assessment

The proposed test plan covers the important future implementation behaviors:

- helper accepts supplied `SideEffectId` values;
- generated reports cite SideEffect IDs in `SideEffects`;
- citation target and kind are correct;
- absent IDs preserve existing none/skipped/unsupported behavior;
- supplied IDs affect only the side-effects section citation and bounded summary;
- helper does not create, resolve, mutate, persist, or execute `SideEffectRecord` values;
- helper does not create `EvidenceReference` values;
- helper does not copy side-effect payload fields;
- errors do not leak secret-like values;
- Debug and serialization remain redaction-safe;
- existing terminal report, executor, WorkReport, SideEffect, EvidenceReference, Diagnostic, validation, adapter telemetry, local check, hook, typed handoff, and runtime tests continue to pass.

Non-blocking addition for the implementation prompt: include an ordering assertion if more than one SideEffect ID is supplied so citation ordering remains deterministic.

## 14. Documentation Review

The plan and related docs state:

- WorkReport SideEffect citation vocabulary is implemented;
- terminal report helper SideEffect citation propagation is planned, not implemented;
- executor SideEffect ID propagation is not implemented;
- side-effect persistence is not implemented;
- side-effect workflow events and audit projections are not implemented;
- EvidenceReference side-effect attachment is not implemented;
- runtime side-effect execution is not implemented;
- writes and write-capable adapters remain unsupported;
- schemas, CLI behavior, examples, hosted behavior, and release posture are unchanged.

## 15. Planning Blockers

No planning blockers.

## 16. Non-Blocking Follow-Ups

- Add deterministic ordering coverage when implementing multi-ID SideEffect citation propagation.
- Keep executor report input propagation as a separate phase after terminal helper propagation review.
- Keep SideEffect workflow events, audit projection, persistence, approval linkage, and EvidenceReference attachment separate from this helper path.
- Revisit explicit missing SideEffect citation records only if future WorkReportContract enforcement requires them.

## 17. Recommended Next Phase

Recommended next phase: terminal report SideEffect citation propagation implementation.

Implement only the terminal helper propagation: add explicit supplied `SideEffectId` input to `TerminalLocalWorkReportInput`, build `WorkReportCitationTarget::SideEffect` citations through existing constructors, place them in `SideEffects`, preserve absent-reference behavior, and add focused tests.

Do not implement executor propagation, automatic discovery, side-effect persistence, workflow events, audit projections, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, provider mutations, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 18. Validation

Validation commands run for this review:

- `npm run check:docs`
- `git diff --check`
