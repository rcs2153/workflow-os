# Terminal Report SideEffect Citation Integration Report

Report date: 2026-06-17

## 1. Executive Summary

Terminal report SideEffect citation propagation is implemented for the in-memory terminal local WorkReport helper.

`TerminalLocalWorkReportInput` now accepts explicitly supplied `SideEffectId` values. Generated terminal reports cite those IDs through `WorkReportCitationTarget::SideEffect` in the required `SideEffects` section, using existing `WorkReportCitation` validation. When no SideEffect IDs are supplied, the helper preserves the existing explicit none/skipped/unsupported side-effects section text.

This phase does not implement executor SideEffect ID propagation, automatic discovery, SideEffect record creation or resolution, side-effect persistence, workflow events, audit projections, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added explicit `side_effect_ids: Vec<SideEffectId>` input to `TerminalLocalWorkReportInput`.
- Added terminal report helper citation construction for supplied SideEffect IDs.
- Placed SideEffect citations in `WorkReportSectionKind::SideEffects`.
- Preserved existing absent-reference side-effects section text.
- Used `WorkReportCitation::new(...)` for all SideEffect citations.
- Kept citation summaries bounded and generic.
- Added focused tests for generated SideEffect citations, deterministic ordering, absent-reference behavior, and non-copying of side-effect payload markers.
- Updated roadmap, planning, and concept docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- executor report input propagation for SideEffect IDs;
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

## 4. Helper API Summary

The in-memory terminal local report helper input now includes:

```rust
pub side_effect_ids: Vec<SideEffectId>
```

Callers may supply already-validated `SideEffectId` values. The helper does not accept `SideEffectRecord`, raw side-effect target references, reason codes, outcomes, authority packets, idempotency bindings, or side-effect redaction metadata.

## 5. Citation Construction Summary

For each supplied ID, the helper constructs:

```rust
WorkReportCitationTarget::SideEffect {
    side_effect_id,
}
```

The citation is built through `WorkReportCitation::new(...)` with generic summary text:

```text
Side-effect record reference considered.
```

The helper does not create, resolve, mutate, persist, or execute `SideEffectRecord` values.

## 6. Section Population Summary

SideEffect citations are placed only in:

```rust
WorkReportSectionKind::SideEffects
```

When no IDs are supplied, the section summary remains:

```text
No write side effects are supported; side effects are none, skipped, or unsupported.
```

When IDs are supplied, the section summary is:

```text
Side-effect records were supplied as stable references; no side-effect payloads are copied.
```

The report does not claim that writes are supported, attempted, approved, completed, denied, or failed. The cited `SideEffectRecord` remains the source of truth for lifecycle state.

## 7. Workflow Semantics Summary

The helper still borrows an already-terminal `WorkflowRun` and does not mutate it.

This phase does not:

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

Executor-integrated report-bearing execution passes an empty SideEffect ID list for now. Explicit executor propagation remains deferred.

## 8. Redaction And Privacy Summary

The helper stores only stable `SideEffectId` citation targets. It does not copy:

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

`Debug` output remains redaction-safe through existing WorkReport and citation debug implementations. Serialization includes valid `SideEffectId` values as stable references, matching existing typed citation behavior, but does not include `SideEffectRecord` payload fields.

## 9. Test Coverage Summary

Added focused tests covering:

- generated reports cite supplied SideEffect IDs in the side-effects section;
- generated SideEffect citations use `WorkReportCitationTarget::SideEffect`;
- generated SideEffect citations use `WorkReportCitationKind::SideEffect`;
- generated SideEffect citation ordering is deterministic;
- absent SideEffect IDs preserve existing none/skipped/unsupported text and no citations;
- generated reports do not copy side-effect target, summary, reason, outcome, idempotency, provider payload, command output, spec, or secret-like markers;
- existing WorkReport terminal helper tests continue to pass.

The focused `workflow-core` WorkReport test suite passed with 107 tests.

## 10. Commands Run And Results

- `cargo test -p workflow-core --test work_report` - passed.
- `cargo fmt --all` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 11. Remaining Known Limitations

- Executor report inputs do not accept or forward SideEffect IDs yet.
- The helper does not discover SideEffect IDs from workflow events, audit records, stores, adapter telemetry, local checks, hooks, disclosures, or side-effect persistence.
- SideEffect records are not persisted.
- SideEffect workflow events and audit projections are not implemented.
- EvidenceReference side-effect attachment is not implemented.
- Runtime side-effect execution and write-capable adapters are not implemented.
- Schemas, CLI behavior, examples, hosted behavior, and release posture are unchanged.

## 12. Recommended Next Phase

Recommended next phase: terminal report SideEffect citation integration review.

Review this implementation before planning or implementing executor SideEffect ID propagation. Do not implement executor propagation, automatic discovery, side-effect persistence, workflow events, audit projections, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes until separately scoped and reviewed.
