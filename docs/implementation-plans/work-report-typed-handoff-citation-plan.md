# WorkReport Typed Handoff Citation Plan

Status: WorkReport typed-handoff citation target vocabulary is implemented. Report generation changes, runtime handoff execution, nested harness scheduling, workflow schema fields, CLI behavior, persistence changes, artifact writing changes, side-effect modeling, writes, domain packs, reasoning lineage, and release posture changes are not implemented.

## 1. Executive Summary

Typed handoff core model is implemented and reviewed. WorkReport already provides the governed terminal handoff artifact, and typed handoffs now provide a model-only transfer object for future bounded workflow or harness boundaries.

The next question is whether WorkReports should be able to cite typed handoffs directly.

This plan recommended adding WorkReport citation vocabulary for typed handoffs in a model-only implementation. The implementation allows report sections to cite a stable typed handoff reference without creating handoffs, generating handoffs, persisting handoffs, or changing report generation behavior.

## 2. Goals

- Allow future WorkReports to cite typed handoff records by stable reference.
- Preserve WorkReport as a terminal governed handoff artifact.
- Preserve TypedHandoff as a model-only, reference-first transfer object.
- Avoid copying typed handoff payloads into report sections.
- Keep report generation behavior unchanged.
- Keep persistence, CLI, schemas, artifacts, and runtime execution unchanged.
- Prepare for future harness and handoff review without enabling nested execution.

## 3. Non-Goals

This plan does not authorize:

- runtime handoff generation;
- automatic WorkReport citation of typed handoffs;
- nested harness execution;
- runtime scheduling;
- workflow schema fields;
- CLI rendering or export;
- report artifact behavior changes;
- persistence changes;
- handoff persistence;
- EvidenceReference creation;
- reasoning lineage implementation;
- side-effect boundary implementation;
- write-capable adapters;
- domain packs;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- `WorkReport` core model;
- `WorkReportCitation`;
- `WorkReportCitationTarget`;
- `WorkReportStableReference`;
- terminal local report generation helper;
- runtime result report exposure helper;
- executor-integrated report-bearing local execution;
- explicit local report artifact store;
- local check citation target vocabulary;
- WorkReport citation target vocabulary for typed handoffs;
- typed handoff core model.

Not implemented:

- automatic typed handoff citation from report helpers;
- typed handoff persistence;
- runtime handoff generation;
- nested harness execution.

## 5. Citation Boundary

WorkReport should cite typed handoffs by stable reference only.

The citation should not:

- embed a full `TypedHandoff`;
- copy obligations, disclosures, risks, notes, or reference lists;
- create a `TypedHandoff`;
- validate a typed handoff against a contract;
- resolve handoff storage;
- imply that a runtime handoff occurred.

The likely first model shape is a new `WorkReportCitationTarget` variant such as:

```rust
TypedHandoff {
    reference: WorkReportStableReference,
}
```

or, if a stable `TypedHandoffId` should become the canonical public identifier for reports:

```rust
TypedHandoff {
    handoff_id: TypedHandoffId,
}
```

The implementation uses `TypedHandoffId` directly. That keeps the report citation target unambiguous without embedding or resolving a full typed handoff value.

## 6. Source-Of-Truth Rules

- TypedHandoff remains the source model for handoff content.
- WorkReport remains the terminal report artifact.
- WorkReport citations point to handoffs; they do not reproduce handoff payloads.
- Audit and workflow events remain run-state and operational history sources.
- EvidenceReference remains citation substrate for evidence, not the handoff itself.

## 7. Privacy And Redaction

Typed handoff citations must not copy:

- typed handoff notes;
- typed handoff risks;
- typed handoff disclosures;
- typed handoff obligations;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Citation summaries, if used, must be bounded and redaction-safe through existing `WorkReportCitation` validation. The recommended first implementation should not auto-populate summaries from typed handoff fields.

## 8. Validation Rules

The implementation ensures:

- typed handoff citation target is representable;
- stable reference or handoff ID is validated;
- citation kind maps deterministically;
- invalid serialized citation target fails closed;
- secret-like citation references fail without leaking values;
- report serialization does not copy typed handoff payloads;
- debug output remains redaction-safe.

## 9. Report Generation Boundary

The first implementation is citation vocabulary only.

It does not change:

- `generate_terminal_local_work_report(...)`;
- `expose_terminal_local_work_report_result(...)`;
- `LocalExecutor::execute_with_report(...)`;
- report artifact stores;
- CLI behavior.

Report helpers may learn to accept supplied typed handoff references only after this model vocabulary is reviewed.

## 10. Artifact And Persistence Boundary

Report artifacts should not change in the first citation-target implementation except to serialize/deserialize reports that contain the new citation target.

Typed handoffs are not persisted by this phase.

Artifact writing from executor paths remains deferred.

## 11. Test Plan

Implemented and future tests should cover:

- typed handoff citation target validates;
- typed handoff citation target serializes and deserializes;
- typed handoff citation target maps to a stable citation kind;
- invalid typed handoff ID/reference fails closed;
- secret-like typed handoff reference fails without leaking values;
- WorkReport citation debug output does not leak typed handoff IDs or summaries;
- WorkReport serialization does not copy typed handoff payload fields;
- existing WorkReport tests still pass;
- existing typed handoff tests still pass;
- no report generation helper behavior changes;
- no report artifact behavior changes;
- no CLI, schema, persistence, runtime, nested execution, side-effect, or write behavior is introduced.

## 12. Proposed Implementation Sequence

1. WorkReport typed handoff citation target vocabulary only is implemented.
2. Focused WorkReport citation tests are added.
3. Docs are updated.
4. Review should follow.
5. Only after review, plan terminal report helper support for supplied typed handoff references.

## 13. Open Questions

- Should WorkReport cite `TypedHandoffId` directly or use `WorkReportStableReference`?
- Should `WorkReportCitationKind` gain `TypedHandoff`, or should typed handoffs be represented as a stable report artifact reference?
- Should report helpers later accept supplied typed handoff IDs?
- Should local report artifacts later validate that typed handoff citations refer to known persisted handoffs?
- How should typed handoff citations relate to future reasoning lineage nodes?
- Should WorkReport sections cite typed handoffs in `EvidenceConsidered`, `IncompleteOrDeferredWork`, `Risks`, `OperatorHandoffNotes`, or a future dedicated section?

## 14. Final Recommendation

The next phase should be WorkReport typed handoff citation target review.

Later implementation may plan terminal report helper support for supplied typed handoff references, but it must not implement runtime handoff generation, automatic report citation, nested harness execution, workflow schema fields, CLI behavior, persistence changes, artifact writing changes, side-effect modeling, writes, domain packs, reasoning lineage, or release posture changes without a separate accepted phase.
