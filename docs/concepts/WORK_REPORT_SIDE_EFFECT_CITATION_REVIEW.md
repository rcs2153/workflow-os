# WorkReport SideEffect Citation Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted; proceed to terminal report SideEffect citation propagation planning.

The implementation delivers the approved model-only WorkReport citation vocabulary: `WorkReportCitationKind::SideEffect` and `WorkReportCitationTarget::SideEffect { side_effect_id: SideEffectId }`. It reuses the existing `WorkReportCitation` validation and serde boundary, keeps `Debug` redaction-safe, adds focused tests, updates documentation, and creates the end-of-phase report.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved model-only scope.

No accidental implementation was found for:

- terminal report helper SideEffect ID propagation;
- executor SideEffect ID propagation;
- report artifact behavior changes;
- automatic discovery from workflow events, audit events, stores, adapter telemetry, or side-effect persistence;
- side-effect workflow events;
- side-effect audit projections;
- side-effect persistence;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- rollback or compensation behavior;
- schemas;
- CLI behavior;
- examples;
- reasoning lineage;
- hosted or distributed behavior;
- release posture changes.

## 3. Model Assessment

The model addition is minimal and domain-neutral.

Implemented:

- `WorkReportCitationKind::SideEffect`;
- `WorkReportCitationTarget::SideEffect { side_effect_id: SideEffectId }`;
- deterministic target-to-kind mapping through `WorkReportCitationTarget::citation_kind()`.

The implementation uses the canonical `SideEffectId` instead of a generic string reference. That is appropriate because `SideEffectId` is already a validated core identifier and avoids introducing another loosely typed report reference shape.

No `SideEffectRecord` is embedded in `WorkReportCitation`, `WorkReportSection`, `WorkReport`, `WorkReportGenerationContext`, report artifact metadata, terminal report inputs, or executor report inputs.

## 4. Source-Of-Truth Assessment

The source-of-truth boundary is preserved.

- `SideEffectRecord` remains the source of truth for side-effect intent, authority, lifecycle state, idempotency, outcome references, and related references.
- `WorkReportCitation` points to a `SideEffectId`; it does not recreate, resolve, summarize, or validate a side-effect record.
- `WorkReport` remains a governed handoff artifact, not a side-effect store.
- Absence of a SideEffect citation still does not prove that no side effect existed.

This keeps report prose from becoming the authoritative side-effect record.

## 5. Section Placement Assessment

The implementation supports manual SideEffect citations in `WorkReportSectionKind::SideEffects`.

It does not automatically place SideEffect citations in:

- evidence considered;
- policy gates evaluated;
- approvals;
- validation and quality checks;
- terminal helper generated reports;
- executor-integrated generated reports.

That is the correct boundary for this phase. Automatic or helper-driven placement should be planned separately.

## 6. Validation And Serde Assessment

Validation is deterministic and reuse-oriented.

Verified:

- `SideEffectId` validates before a target can be constructed.
- `WorkReportCitation::new(...)` remains the validation boundary for summary, redaction metadata, missing flag, and sensitivity.
- `citation_kind()` maps SideEffect targets to `WorkReportCitationKind::SideEffect`.
- valid SideEffect citations serialize and deserialize.
- invalid serialized SideEffect IDs fail closed.
- secret-like SideEffect IDs fail without leaking rejected values.
- existing WorkReport serde shape remains consistent with the existing tagged citation target style.

No new validation path copies side-effect payload data.

## 7. Privacy And Redaction Assessment

The privacy posture is acceptable.

The citation target stores only a stable `SideEffectId`. It does not store or copy:

- side-effect target references;
- side-effect summaries;
- side-effect reason codes;
- side-effect outcome references;
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

`WorkReportCitationTarget` `Debug` output redacts the reference value and exposes only the citation kind. Serialization includes the stable `SideEffectId`, matching existing typed citation behavior for evidence references, typed handoffs, hook invocations, and approvals.

## 8. Runtime Boundary Assessment

The implementation does not touch runtime behavior.

No code path was added that:

- mutates `WorkflowRun`;
- mutates `WorkflowRunSnapshot`;
- appends workflow events;
- emits audit events;
- emits observability events;
- writes state backend records;
- persists side-effect records;
- persists report artifacts differently;
- invokes adapters;
- calls external systems;
- writes files;
- emits CLI output.

Terminal report generation and executor-integrated report-bearing execution remain unchanged.

## 9. Relationship To SideEffect

The relationship to SideEffect is correctly reference-first.

The WorkReport model imports `SideEffectId`, not `SideEffectRecord`. This prevents report citations from becoming a side-effect payload carrier while still allowing future report sections to cite an authoritative side-effect record.

This is compatible with later side-effect workflow events, audit projections, persistence, and runtime execution because those phases can make `SideEffectId` resolvable without changing the WorkReport citation target shape.

## 10. Test Quality Assessment

The focused tests cover the expected model boundary:

- SideEffect citation target validation;
- citation kind mapping;
- serde round trip;
- secret-like ID rejection;
- invalid serialized SideEffect citation failure;
- deserialization error non-leakage;
- redaction-safe `Debug`;
- serialization without side-effect record payload fields;
- manual SideEffects section construction with a SideEffect citation;
- existing side-effects section behavior without write support;
- existing WorkReport, WorkReportContract, SideEffect, runtime, adapter, validation, local check, hook, and executor tests through `cargo test --workspace`.

No blocker-level test gaps were found.

Non-blocking gap: a future terminal helper propagation phase should add tests proving generated reports can place supplied SideEffect IDs into the SideEffects section without changing existing absent-reference text when none are supplied.

## 11. Documentation Review

Documentation accurately states:

- WorkReport SideEffect citation vocabulary is implemented;
- terminal report helper SideEffect ID propagation is not implemented;
- executor SideEffect ID propagation is not implemented;
- side-effect persistence is not implemented;
- side-effect workflow events and audit projections are not implemented;
- EvidenceReference side-effect attachment is not implemented;
- runtime side-effect execution is not implemented;
- writes and write-capable adapters remain unsupported;
- schemas, CLI behavior, examples, hosted behavior, and release posture are unchanged.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Plan terminal report helper propagation for explicitly supplied SideEffect IDs.
- Plan executor report input propagation only after terminal helper propagation is accepted.
- Keep side-effect workflow events, audit projection, persistence, approval linkage, and EvidenceReference attachment separate from report citation vocabulary.
- Consider adding a contract-level citation requirement test if future contracts require SideEffect citations.

## 14. Recommended Next Phase

Recommended next phase: terminal report SideEffect citation propagation planning.

That phase should decide how `TerminalLocalWorkReportInput` may accept explicitly supplied `SideEffectId` values and place them in the SideEffects section through existing `WorkReportCitation` constructors. It must not add executor propagation, automatic discovery, side-effect persistence, workflow events, audit projection, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, or release posture changes.

## 15. Validation

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`
