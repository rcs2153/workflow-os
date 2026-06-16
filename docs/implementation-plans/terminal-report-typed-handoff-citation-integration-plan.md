# Terminal Report Typed Handoff Citation Integration Plan

Status: Terminal report helper support for supplied typed handoff references is implemented. Executor-integrated typed handoff report input propagation is implemented in [Executor Typed Handoff Report Input Propagation Plan](executor-typed-handoff-report-input-plan.md). Runtime handoff generation, automatic typed handoff citation, nested harness execution, typed handoff persistence, workflow schema fields, CLI behavior, report artifact behavior changes, side-effect modeling, writes, domain packs, reasoning lineage, and release posture changes are not implemented.

## 1. Executive Summary

WorkReport typed handoff citation target vocabulary is implemented and reviewed. `WorkReportCitationTarget::TypedHandoff` can now cite a `TypedHandoffId` without embedding or resolving a full typed handoff value.

The next question is whether the terminal local report generation helper should accept explicitly supplied typed handoff IDs and include them as citations in generated in-memory reports.

This plan recommended a narrow implementation: add explicit typed handoff ID inputs to the in-memory terminal report helper and cite those IDs in an existing report section without creating handoffs, generating handoffs, persisting handoffs, changing executor semantics, changing report artifact behavior, or adding automatic runtime citation.

## 2. Goals

- Allow terminal report helper callers to supply existing `TypedHandoffId` values.
- Cite supplied typed handoff IDs through existing `WorkReportCitation` constructors.
- Preserve terminal report generation as explicit, local, deterministic, and in-memory.
- Preserve current workflow pass/fail semantics.
- Avoid copying typed handoff payload fields into report sections.
- Keep source-of-truth boundaries clear between WorkReport and TypedHandoff.
- Prepare for future typed handoff review and helper integration without enabling nested execution.

## 3. Non-Goals

This plan does not authorize:

- runtime handoff generation;
- automatic typed handoff citation discovery;
- nested harness execution;
- runtime scheduling;
- typed handoff persistence;
- report artifact behavior changes;
- workflow schema fields;
- CLI rendering or export;
- example updates;
- EvidenceReference creation;
- approval evidence attachment;
- reasoning lineage implementation;
- side-effect boundary implementation;
- write-capable adapters;
- domain packs;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- `TypedHandoffId`;
- typed handoff core model;
- WorkReport core model;
- WorkReport citation kind and target vocabulary for typed handoffs;
- terminal local report generation helper;
- terminal report helper typed handoff input;
- in-memory runtime result exposure helper;
- executor-integrated report-bearing execution;
- explicit local report artifact store;
- local check result report helper integration.

Not implemented:

- automatic typed handoff citation;
- automatic runtime discovery of typed handoff IDs for executor report inputs;
- typed handoff persistence;
- runtime handoff generation;
- nested harness execution.

## 5. Proposed Helper Input

The implementation adds one explicit optional input field to `TerminalLocalWorkReportInput`:

```rust
pub typed_handoff_ids: Vec<TypedHandoffId>
```

Rules:

- accept only already-constructed `TypedHandoffId` values;
- do not accept raw typed handoff payloads;
- do not accept `TypedHandoff` values;
- do not read typed handoffs from storage;
- do not validate typed handoff existence;
- do not infer typed handoff IDs from workflow events or report notes;
- do not fabricate missing typed handoff IDs.

Using `TypedHandoffId` keeps validation at the existing ID boundary and avoids generic string references.

## 6. Citation Construction Policy

Helper integration constructs citations with:

```rust
WorkReportCitationTarget::TypedHandoff {
    typed_handoff_id,
}
```

Rules:

- use `WorkReportCitation::new(...)`;
- use bounded, generic summary text such as `Typed handoff reference considered.`;
- do not copy handoff obligations, disclosures, risks, notes, endpoints, reference lists, or payload fields;
- use the report helper's existing sensitivity and redaction metadata;
- fail safely if citation construction fails;
- return structured non-leaking report-generation errors;
- do not create `EvidenceReference` values;
- do not create or mutate `TypedHandoff` values.

## 7. Section Placement Recommendation

Recommended first placement: `OperatorHandoffNotes`.

Rationale:

- typed handoffs are governed transfer objects for downstream operators, workflow phases, or future harnesses;
- `OperatorHandoffNotes` already represents downstream handoff context;
- placing typed handoff citations there avoids implying that handoffs are evidence payloads;
- it avoids overloading `EvidenceConsidered` before evidence-ledger semantics for handoffs are designed.

Alternative placements:

| Section | Assessment |
| --- | --- |
| `EvidenceConsidered` | Useful when a typed handoff is reviewed as evidence, but risks confusing handoff with evidence payload. Defer. |
| `IncompleteOrDeferredWork` | Useful for blocked or deferred handoffs, but requires status-aware handoff semantics. Defer. |
| `Risks` | Useful for risk-bearing handoffs, but requires status/context interpretation. Defer. |
| New dedicated section | Not supported by v1 required section vocabulary; defer until domain-specific/report extension planning. |

## 8. Missing And Unavailable References

If no typed handoff IDs are supplied:

- keep existing section text behavior;
- do not add missing typed handoff citations by default;
- do not imply a typed handoff was expected.

If future contract requirements make typed handoff citations mandatory, missing citation semantics should be handled in a separate report/audit/missing-citation phase. This plan does not change missing-citation semantics.

## 9. Error Handling

The implementation should continue to:

- return existing structured `WorkflowOsError` values on citation construction failure;
- avoid leaking typed handoff IDs, raw paths, notes, risks, payloads, tokens, or secret-like values;
- not convert helper citation failures into workflow diagnostics;
- not mutate workflow runs or append events when helper citation fails;
- not change workflow pass/fail results.

Since inputs are already `TypedHandoffId`, secret-like or invalid IDs should fail before report generation. Deserialized helper-input tests may still verify fail-closed behavior if a future serializable request model is added.

## 10. Workflow Semantics Boundary

The helper integration must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit or observability events;
- touch a `StateBackend`;
- persist typed handoffs;
- persist reports;
- create filesystem artifacts;
- expose CLI output;
- change executor behavior.

Executor-integrated report-bearing execution may accept typed handoff IDs only after this helper integration is implemented and reviewed separately.

## 11. Privacy And Redaction

The helper integration must not copy:

- typed handoff obligations;
- typed handoff disclosures;
- typed handoff risks;
- typed handoff notes;
- typed handoff endpoint names;
- typed handoff reference names;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Debug output must remain redaction-safe through existing WorkReport and citation `Debug` implementations. Serialization may include typed handoff IDs as stable references, but not typed handoff payloads.

## 12. Test Plan

Implemented and future tests should cover:

- terminal report helper accepts supplied typed handoff IDs;
- generated report includes typed handoff citations;
- typed handoff citations use `WorkReportCitationTarget::TypedHandoff`;
- typed handoff citations map to `WorkReportCitationKind::TypedHandoff`;
- typed handoff citations are placed in `OperatorHandoffNotes`;
- absence of typed handoff IDs preserves existing not-available/none section behavior;
- helper does not create or mutate `TypedHandoff` values;
- helper does not recreate `EvidenceReference` values;
- helper does not copy typed handoff obligations, disclosures, risks, notes, or endpoints;
- helper errors do not leak secret-like values;
- Debug output does not leak typed handoff IDs;
- serialization does not copy typed handoff payload fields;
- existing terminal report helper tests still pass;
- existing executor-integrated report tests still pass;
- existing WorkReport, TypedHandoff, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests still pass.

## 13. Proposed Implementation Sequence

1. Add `typed_handoff_ids: Vec<TypedHandoffId>` to `TerminalLocalWorkReportInput`. Completed.
2. Add typed handoff citation construction inside terminal report citation grouping. Completed.
3. Attach typed handoff citations to `OperatorHandoffNotes`. Completed.
4. Add focused helper tests. Completed.
5. Update docs and phase report. Completed.
6. Review.
7. Only after review, plan executor-integrated report input propagation for typed handoff IDs. Completed in [Executor Typed Handoff Report Input Propagation Plan](executor-typed-handoff-report-input-plan.md).
8. Implement executor-integrated typed handoff report input propagation. Completed.

## 14. Open Questions

- Should typed handoff citations always appear in `OperatorHandoffNotes`, or should failed/canceled reports place them in `IncompleteOrDeferredWork`?
- Should future contract enforcement be able to require typed handoff citations?
- Should local report artifacts eventually verify that cited typed handoff IDs exist in a typed handoff store?
- Should explicit missing typed handoff citations wait for the report/audit/missing-citation semantics phase?
- Should executor-integrated report inputs accept typed handoff IDs directly after helper support exists?
- How should typed handoff citations relate to future reasoning lineage nodes?

## 15. Final Recommendation

The next phase should be executor typed handoff report input propagation review.

Executor propagation is implemented in [Executor Typed Handoff Report Input Propagation Plan](executor-typed-handoff-report-input-plan.md). It does not implement runtime handoff generation, automatic citation discovery, nested harness execution, typed handoff persistence, workflow schema fields, CLI behavior, report artifact behavior changes, side-effect modeling, writes, domain packs, reasoning lineage, or release posture changes.
