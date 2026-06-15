# WorkReport Typed Handoff Citation Plan

Status: Planning only. WorkReport typed-handoff citation support is not implemented. This plan does not authorize report generation changes, runtime handoff execution, nested harness scheduling, workflow schema fields, CLI behavior, persistence changes, artifact writing changes, side-effect modeling, writes, domain packs, reasoning lineage, or release posture changes.

## 1. Executive Summary

Typed handoff core model is implemented and reviewed. WorkReport already provides the governed terminal handoff artifact, and typed handoffs now provide a model-only transfer object for future bounded workflow or harness boundaries.

The next question is whether WorkReports should be able to cite typed handoffs directly.

This plan recommends adding WorkReport citation vocabulary for typed handoffs in a future model-only implementation. The first implementation should allow report sections to cite a stable typed handoff reference without creating handoffs, generating handoffs, persisting handoffs, or changing report generation behavior.

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

- implementation in this prompt;
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
- typed handoff core model.

Not implemented:

- WorkReport citation target for typed handoffs;
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

The plan recommends using `TypedHandoffId` if the dependency is acceptable inside `work_report.rs`, because it prevents generic string references from becoming ambiguous. If that creates coupling concerns, use `WorkReportStableReference` and document the reference namespace.

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

Future implementation should ensure:

- typed handoff citation target is representable;
- stable reference or handoff ID is validated;
- citation kind maps deterministically;
- invalid serialized citation target fails closed;
- secret-like citation references fail without leaking values;
- report serialization does not copy typed handoff payloads;
- debug output remains redaction-safe.

## 9. Report Generation Boundary

The first implementation should be citation vocabulary only.

Do not change:

- `generate_terminal_local_work_report(...)`;
- `expose_terminal_local_work_report_result(...)`;
- `LocalExecutor::execute_with_report(...)`;
- report artifact stores;
- CLI behavior.

Report helpers may learn to accept supplied typed handoff references only after this model vocabulary is reviewed.

## 10. Artifact And Persistence Boundary

Report artifacts should not change in the first citation-target implementation except to serialize/deserialize reports that contain the new citation target.

Typed handoffs should not be persisted by this phase.

Artifact writing from executor paths remains deferred.

## 11. Test Plan

Future tests should cover:

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

1. Add WorkReport typed handoff citation target vocabulary only.
2. Add focused WorkReport citation tests.
3. Update docs.
4. Review.
5. Only after review, plan terminal report helper support for supplied typed handoff references.

## 13. Open Questions

- Should WorkReport cite `TypedHandoffId` directly or use `WorkReportStableReference`?
- Should `WorkReportCitationKind` gain `TypedHandoff`, or should typed handoffs be represented as a stable report artifact reference?
- Should report helpers later accept supplied typed handoff IDs?
- Should local report artifacts later validate that typed handoff citations refer to known persisted handoffs?
- How should typed handoff citations relate to future reasoning lineage nodes?
- Should WorkReport sections cite typed handoffs in `EvidenceConsidered`, `IncompleteOrDeferredWork`, `Risks`, `OperatorHandoffNotes`, or a future dedicated section?

## 14. Final Recommendation

The next implementation phase should be WorkReport typed handoff citation target vocabulary only.

It must not implement runtime handoff generation, automatic report citation, nested harness execution, workflow schema fields, CLI behavior, persistence changes, artifact writing changes, side-effect modeling, writes, domain packs, reasoning lineage, or release posture changes.
