# WorkReport SideEffect Citation Report

Report date: 2026-06-17

## 1. Executive Summary

The WorkReport SideEffect citation target phase is implemented as model-only vocabulary.

`WorkReportCitation` can now cite a governed `SideEffectRecord` by stable `SideEffectId` through `WorkReportCitationTarget::SideEffect`. The citation target maps deterministically to `WorkReportCitationKind::SideEffect`, serializes and deserializes through the existing WorkReport citation model, and keeps `Debug` redaction behavior intact.

This phase does not propagate SideEffect IDs through terminal report helper inputs, executor report inputs, report artifact behavior, workflow events, audit projections, persistence, CLI behavior, schemas, examples, runtime side-effect execution, provider mutations, or writes.

## 2. Scope Completed

- Added `WorkReportCitationKind::SideEffect`.
- Added `WorkReportCitationTarget::SideEffect { side_effect_id: SideEffectId }`.
- Mapped the new target to the new citation kind.
- Reused existing `WorkReportCitation::new(...)` validation.
- Preserved redaction-safe `Debug` output for citation targets.
- Preserved existing serde tag style.
- Added focused WorkReport tests for SideEffect citation behavior.
- Updated roadmap and concept docs to state the implemented vocabulary and remaining boundaries.

## 3. Scope Explicitly Not Completed

This phase did not implement:

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

## 4. Model Changes

The implementation adds one citation kind and one citation target:

```rust
WorkReportCitationKind::SideEffect

WorkReportCitationTarget::SideEffect {
    side_effect_id: SideEffectId,
}
```

The target stores only the validated `SideEffectId`. It does not embed a `SideEffectRecord` or copy target references, summaries, reason codes, outcome references, authority details, idempotency bindings, references, redaction metadata, provider payloads, command output, logs, spec contents, parser payloads, environment values, credentials, or token-like values.

## 5. Validation Boundary Summary

SideEffect citation validation relies on:

- `SideEffectId` constructor-backed validation;
- existing `WorkReportCitation::new(...)` summary validation;
- existing WorkReport redaction metadata validation;
- existing WorkReport citation sensitivity handling;
- constructor-backed deserialization for invalid serialized citation payloads.

Invalid or secret-like SideEffect IDs fail closed before a citation can be stored. Errors use stable existing SideEffect validation codes and do not include rejected raw values.

## 6. Redaction And Privacy Summary

`WorkReportCitationTarget` `Debug` output continues to show only the citation kind and a redacted reference marker. SideEffect citation serialization includes only the stable SideEffect ID, matching existing typed citation target behavior. Serialization does not include SideEffect record payload fields.

This phase does not create SideEffect records, resolve SideEffect records, attach evidence to SideEffect records, read provider payloads, read command output, or copy raw side-effect context into reports.

## 7. Test Coverage Summary

Added focused WorkReport tests covering:

- SideEffect citation target validation;
- citation kind mapping to `WorkReportCitationKind::SideEffect`;
- serde round trip for valid SideEffect citation;
- secret-like SideEffect ID rejection without leaking the value;
- invalid serialized SideEffect citation failure without leaking the value;
- redaction-safe citation `Debug`;
- serialization without SideEffect record payload fields;
- manual SideEffects section construction with a SideEffect citation;
- continued side-effect section behavior without write support.

The focused `workflow-core` WorkReport test suite passed with 104 tests.

## 8. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test work_report` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 9. Remaining Known Limitations

- Reports can cite SideEffect IDs only when a caller manually constructs citations.
- Terminal report helper inputs do not accept SideEffect IDs yet.
- Executor report inputs do not propagate SideEffect IDs yet.
- SideEffect records are not persisted.
- SideEffect workflow events and audit projections are not implemented.
- EvidenceReference side-effect attachment is not implemented.
- Runtime side-effect execution and write-capable adapters are not implemented.
- Schemas, CLI behavior, examples, hosted behavior, and release posture are unchanged.

## 10. Recommended Next Phase

Recommended next phase: terminal report SideEffect citation propagation planning.

The citation vocabulary is accepted in [WorkReport SideEffect Citation Review](WORK_REPORT_SIDE_EFFECT_CITATION_REVIEW.md). Plan terminal report helper propagation for explicitly supplied SideEffect IDs. Do not implement executor propagation, artifact behavior changes, side-effect persistence, side-effect workflow events, audit projection, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, or release posture changes until separately scoped and reviewed.
