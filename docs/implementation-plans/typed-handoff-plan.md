# Typed Handoff Plan

Status: Typed handoff core model is implemented. Runtime handoff execution, automatic handoff generation, nested harness scheduling, workflow schema fields, CLI behavior, persistence, side-effect modeling, writes, domain packs, reasoning lineage, and release posture changes are not implemented.

## 1. Executive Summary

Composable Harness Contracts now have a reviewed core model. The next safe roadmap question is how bounded harnesses should transfer structured context, evidence, outputs, risks, and obligations without relying on unbounded natural-language summaries.

Typed handoffs should become the reference-first transfer object between governed workflow phases or future harnesses. The first implementation should be model-only.

## 2. Goals

- Define a domain-neutral typed handoff concept.
- Preserve Workflow OS as a governed work runtime, not a recursive-agent framework.
- Carry stable references instead of raw payloads.
- Preserve explicit source and target boundaries.
- Support evidence, validation, local check, audit, policy, approval, and report references.
- Represent incomplete work, known limitations, risks, and next obligations.
- Prepare for future harness execution without implementing it.

## 3. Non-Goals

This plan does not authorize:

- nested harness execution;
- runtime scheduling;
- automatic handoff generation;
- workflow spec schema fields;
- CLI rendering or export;
- persistence or artifact writing;
- reasoning lineage implementation;
- side-effect boundary implementation;
- write-capable adapters;
- domain packs;
- hosted or distributed runtime claims;
- recursive agents or agent swarms;
- Level 3 or Level 4 autonomy claims;
- release posture changes.

## 4. Current Baseline

Implemented foundations include:

- workflow and run identity;
- validation and source diagnostics;
- EvidenceReference;
- WorkReportContract and WorkReport;
- terminal report helpers and artifact APIs;
- local check result references;
- Composable Harness Contract core model.

Implemented:

- typed handoff contract/value model;
- source and target endpoint model;
- reference target vocabulary for evidence, validation, local checks, workflow/audit/policy/approval events, WorkReports, adapter telemetry, inputs, and outputs;
- bounded obligations, disclosures, risks, and notes;
- validation, serde, and redaction-safe `Debug`;
- focused Rust tests.

Not implemented:

- handoff runtime behavior;
- nested harness execution;
- schema exposure;
- CLI rendering;
- handoff persistence.

## 5. Why Typed Handoffs Belong In Workflow OS

Governed delegation needs typed transfer, not loose summaries. A future implementation should make handoffs auditable, bounded, and reference-first so downstream work receives explicit context, evidence, risks, limitations, and obligations.

This is not agent-to-agent messaging. It is a governed handoff contract between bounded execution envelopes.

## 6. Candidate Core Model

The first model-only implementation may introduce the smallest useful set of types, such as:

- `TypedHandoff`;
- `TypedHandoffId`;
- `TypedHandoffContract`;
- `TypedHandoffContractId`;
- `TypedHandoffContractVersion`;
- `TypedHandoffStatus`;
- `TypedHandoffInputReference`;
- `TypedHandoffOutputReference`;
- `TypedHandoffEvidenceReference`;
- `TypedHandoffValidationReference`;
- `TypedHandoffObligation`;
- `TypedHandoffDisclosure`;
- `TypedHandoffRisk`;
- `TypedHandoffFailureSemantics`.

The implementation should reuse existing vocabulary where possible, including harness contract IDs, `EvidenceReferenceId`, validation references, local check references, `EventId`, approval/policy references, `WorkReportSensitivity`, and redaction metadata.

## 7. Required Handoff Fields

A future typed handoff should capture:

- handoff ID;
- handoff contract ID/version;
- source workflow phase or harness reference;
- target workflow phase or harness reference;
- workflow ID and run ID where available;
- produced output references;
- required input references;
- evidence references;
- validation and local check references;
- audit, workflow event, policy, and approval references where available;
- incomplete or deferred work disclosures;
- known limitations;
- risks;
- next obligations;
- bounded operator notes;
- sensitivity;
- redaction metadata.

## 8. Validation Rules

Validation should ensure:

- IDs and versions are valid;
- source and target are present and distinct where required;
- reference lists are bounded;
- duplicate references are rejected;
- bounded text fields reject secret-like values;
- required reference groups are present when declared;
- invalid serialized handoffs fail closed;
- validation errors use stable codes and do not leak raw values.

## 9. Privacy And Redaction

Typed handoffs must not copy:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Natural-language notes may exist only as bounded, redaction-aware annotations. They must not become the source of truth for context transfer.

## 10. Relationship To Harness Contracts

Composable Harness Contracts describe the governed envelope. Typed handoffs describe the transfer across envelope boundaries.

The first typed handoff model should connect cleanly to `HarnessHandoffRequirement` without adding nested execution, scheduling, or schema behavior.

## 11. Relationship To Work Reports

Work reports are terminal handoff artifacts. Typed handoffs are intermediate transfer objects for future harness/workflow boundaries.

Future WorkReport sections may cite typed handoffs, but this plan does not change report generation or artifacts.

## 12. Relationship To EvidenceReference

Typed handoffs should cite `EvidenceReference` IDs and other stable references. They must not create evidence references implicitly and must not copy evidence payloads.

## 13. Runtime And State Boundary

The first implementation should be model-only. It must not:

- mutate workflow state;
- append events;
- schedule harnesses;
- persist handoffs;
- write files;
- expose CLI output.

## 14. Failure Semantics

Future handoff failure semantics may include blocked, rejected, incomplete, superseded, or accepted states. The first model should represent failure honestly without changing workflow execution semantics.

## 15. Test Plan

Future tests should cover:

- valid minimal typed handoff;
- invalid handoff ID/version;
- missing source or target;
- duplicate input/output/evidence/obligation references;
- secret-like notes, risks, limitations, and references rejected;
- bounded reference validation;
- serde round trip;
- invalid serialized handoff fails closed;
- debug output is redaction-safe;
- no raw command/provider/spec/file payload copying;
- no runtime events, persistence, artifacts, CLI, schemas, or writes introduced.

## 16. Proposed Implementation Sequence

1. Add typed handoff core model only. Completed.
2. Add validation and serde tests. Completed.
3. Review.
4. WorkReport typed handoff citation target vocabulary is implemented as documented in [WorkReport Typed Handoff Citation Plan](work-report-typed-handoff-citation-plan.md).
5. Terminal report helper typed handoff citation integration is implemented in [Terminal Report Typed Handoff Citation Integration Plan](terminal-report-typed-handoff-citation-integration-plan.md).
5. Only after later planning, consider runtime handoff production.

## 17. Open Questions

- Should handoff contract and handoff value be separate models in the first slice?
- Should source and target reference harness IDs, workflow step IDs, or both?
- Should typed handoffs be citeable from WorkReport immediately?
- Should missing references be represented as section text or typed missing reference records?
- How should handoffs relate to future reasoning lineage?
- What belongs in core versus future domain packs?

## 18. Final Recommendation

The next phase should be terminal report helper typed handoff citation integration review.

It must not build nested execution, runtime scheduling, workflow schema fields, CLI behavior, persistence, side-effect modeling, writes, domain packs, reasoning lineage, or release posture changes.
