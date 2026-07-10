# Workflow-Declared Proof-Marker Artifact Requirements Plan

Status: internal proof-marker artifact requirement model and policy mapping implemented in [Workflow-Declared Proof-Marker Artifact Requirement Model Report](../concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_MODEL_REPORT.md) and accepted in [Workflow-Declared Proof-Marker Artifact Requirement Model Review](../concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_MODEL_REVIEW.md). Schema/parser/SDK vocabulary is implemented in [Workflow-Declared Proof-Marker Artifact Requirement Schema Report](../concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md), following [Workflow-Declared Proof-Marker Artifact Requirement Schema Plan](workflow-declared-proof-marker-artifact-requirement-schema-plan.md). This follows the accepted explicit executor artifact proof-marker gate integration in [Executor Artifact Proof-Marker Gate Integration Review](../concepts/EXECUTOR_ARTIFACT_PROOF_MARKER_GATE_INTEGRATION_REVIEW.md).

This plan does not implement Rust code, workflow schema fields, default executor behavior, automatic report generation, automatic artifact writing, automatic proof-marker projection persistence, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has an explicit artifact-capable executor path that can require store-backed approval proof-marker projection validation before writing a `WorkReportArtifactRecord`.

Today that requirement is caller-supplied. A caller can opt into proof-marker gates by invoking the explicit proof-marker artifact path and supplying a local approval proof-marker projection store plus gate policy.

The next design question is whether authored workflow specs should eventually declare that terminal report artifacts require approval proof-marker projection coverage before persistence. This plan answers that question without implementing it.

The recommendation is conservative: model the requirement first, expose schema later, derive gate policy only in explicit artifact-capable paths, and preserve the invariant that declared governance must be enforced or rejected.

## 2. Goals

- Define the future authored-workflow declaration boundary for proof-marker report artifact requirements.
- Preserve the invariant that declared governance must be enforced or rejected.
- Avoid decorative workflow metadata that looks enforceable but is ignored.
- Keep proof-marker requirements artifact-scoped, not workflow pass/fail semantics.
- Reuse existing approval proof-marker citation, projection, store-backed gate, and executor artifact gate primitives.
- Keep projection persistence caller-supplied and explicit.
- Preserve deterministic validation and redaction-safe errors.
- Prepare a small future implementation slice that can be reviewed before schema exposure.
- Keep write-capable adapter work blocked until proof-marker artifact requirements are composed and reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- workflow schema changes;
- public schema publication;
- TypeScript SDK changes;
- runtime config;
- automatic report generation;
- automatic report artifact writing;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- default proof-marker enforcement for existing executor paths;
- automatic approval proof-marker projection persistence;
- CLI rendering, artifact export, or artifact write commands;
- example updates;
- public approval cards;
- approval evidence attachment;
- dedicated proof-marker audit sink records;
- new workflow event types;
- side-effect execution;
- provider mutation or write-capable adapters;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- approval decision proof markers for opt-in proof-enforced approvals;
- bounded approval proof-marker inspect/projection behavior;
- WorkReport approval proof-marker citation derivation;
- terminal report opt-in proof-marker citation integration;
- executor report input propagation for proof-marker citation policy;
- approval proof-marker audit projection helpers;
- local approval proof-marker projection persistence helper;
- pure in-memory report artifact approval proof-marker gate helper;
- store-backed report artifact approval proof-marker gate helper;
- helper-level artifact write composition with SideEffect, approval-linkage, high-assurance disclosure, and proof-marker gates;
- opt-in executor artifact path proof-marker gate integration.

Not implemented:

- workflow-declared proof-marker artifact requirements;
- schema fields for proof-marker artifact requirements;
- runtime derivation from workflow specs;
- default executor proof-marker enforcement;
- automatic report artifact writing;
- automatic projection persistence;
- CLI artifact behavior.

## 5. Problem Statement

The explicit proof-marker artifact path proves that a caller can require approval proof-marker projection coverage before persisting a terminal report artifact.

That is not yet an authored workflow contract. If a workflow represents sensitive or high-impact work, the workflow should eventually be able to declare:

```text
terminal report artifacts for this workflow must prove approval proof-marker posture before they may be persisted
```

Without a workflow declaration, every caller must remember to supply the right proof-marker artifact gate policy. That is acceptable for the first explicit API, but it is not a durable governance contract.

The opposite failure is worse: adding YAML fields that imply proof-marker artifact enforcement before the runtime can enforce them. Workflow OS must not create false governance. A declared requirement must either be enforced in an explicit artifact-capable path or rejected.

## 6. Source-Of-Truth Boundaries

The declaration must not make report artifacts or projection records the source of truth for approvals.

Boundaries:

- workflow spec: authored intent and artifact persistence requirements;
- workflow validation: rejects unsupported or unenforceable requirements;
- workflow event log: source of truth for approval decision events;
- approval proof marker: bounded proof that the approval presentation was tied to the decision;
- proof-marker projection store: caller-supplied durable projection cache;
- `WorkReport`: governed handoff artifact carrying bounded approval citations;
- report artifact gate: explicit pre-persistence enforcement;
- report artifact store: durable artifact storage after all selected gates pass.

The future declaration should select proof-marker artifact gate policy. It should not claim enterprise identity proof, quorum approval, approval revocation enforcement, side-effect completion, provider write authorization, or hosted audit guarantees.

## 7. Candidate Declaration Shape

A future workflow-authored requirement should be small, posture-based, and terminal-artifact scoped.

Conceptual shape:

```yaml
report_artifact_requirements:
  approval_proof_markers: marker_required
```

Candidate posture vocabulary:

- `not_required`
- `projection_required`
- `marker_required`

Meaning:

- `not_required`: do not require proof-marker projection coverage before artifact persistence.
- `projection_required`: every approval citation in the artifact must resolve to one projection record, but marker-free projections may be accepted when the projection explicitly says proof markers were not required.
- `marker_required`: every approval citation in the artifact must resolve to one projection record with a present proof marker.

This is conceptual only. No schema fields are implemented by this plan.

Rejected declaration shapes:

- free-form policy rules;
- embedded approval IDs;
- embedded presentation IDs or hashes;
- raw approval payloads;
- raw projection payloads;
- per-step proof-marker artifact declarations in the first slice;
- workflow execution approval requirements disguised as artifact requirements.

## 8. Relationship To Existing High-Assurance Artifact Requirements

Workflow OS already has `report_artifact_requirements.high_assurance_approval`.

Proof-marker artifact requirements should be sibling posture under the same terminal artifact requirement boundary, not a replacement for high-assurance approval disclosure.

Conceptual future shape:

```yaml
report_artifact_requirements:
  high_assurance_approval: validated_fail_closed_disclosure_required
  approval_proof_markers: marker_required
```

Relationship:

```text
approval decision -> proof marker -> audit projection -> WorkReport citation -> artifact proof-marker gate
```

High-assurance artifact requirements prove the report discloses high-assurance validation posture. Proof-marker artifact requirements prove cited approvals have durable proof-marker projection coverage. They are separate gates that should compose by strictness when both are declared.

## 9. Recommended First Implementation Boundary

Recommended first implementation: internal model and policy mapping only.

Why:

- keeps the authored requirement shape reviewable before schema compatibility promises attach;
- lets tests prove deterministic mapping to `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- preserves the invariant that unsupported governance is rejected;
- avoids widening CLI, schemas, TypeScript SDK, examples, or executor behavior too early.

Candidate first slice:

1. Add an internal `WorkReportArtifactApprovalProofMarkerRequirement` posture enum.
2. Add an internal requirement definition field separate from schema-facing workflow YAML.
3. Map requirement posture to `WorkReportArtifactApprovalProofMarkerGatePolicy`.
4. Reject unsupported future proof-marker requirement vocabulary.
5. Add deterministic validation and redaction-safe serialization tests.
6. Do not wire the model to workflow YAML until a separate schema plan is reviewed.

## 10. Future Schema Boundary

Schema exposure must be a separate phase.

Future schema exposure must update these surfaces together:

- Rust workflow definition and parser model;
- semantic validation diagnostics;
- checked-in `schemas/v0/workflow.schema.json`;
- TypeScript SDK workflow types/builders;
- TypeScript contract fixtures;
- `docs/specs/workflows.md`;
- roadmap and adjacent artifact/proof-marker planning docs;
- `npm run check:contracts`.

The first schema-facing implementation should decide whether stronger postures are:

- accepted only by explicit artifact-capable validation; or
- rejected by default validation with a stable `runtime_not_enforced` diagnostic until executor integration exists.

The safer default is to reject any declaration that cannot be enforced by the current validation/execution path.

## 11. Runtime Integration Boundary

The future runtime integration point should remain the explicit artifact-capable executor path, not the base executor path.

Allowed future integration:

- artifact-capable validation may accept proof-marker artifact requirements for the selected workflow;
- the executor artifact path may derive workflow-declared proof-marker policy from the selected workflow;
- derived policy may compose with caller-supplied proof-marker policy by strictness;
- artifact persistence must fail before write when the effective proof-marker policy is not satisfied.

Rejected for first runtime integration:

- automatic artifact writing after every terminal run;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- appending workflow events for proof-marker derivation;
- mutating workflow snapshots;
- adding CLI artifact commands;
- inferring proof-marker projection stores from runtime state;
- creating projection records automatically.

## 12. Effective Policy Composition

The effective proof-marker artifact gate policy should be the stricter of workflow declaration and caller-supplied policy.

Recommended strictness order:

1. disabled or `not_required`;
2. `projection_required`;
3. `marker_required`.

Mapping:

- `not_required` maps to no proof-marker gate.
- `projection_required` maps to `WorkReportArtifactApprovalProofMarkerGatePolicy::allow_marker_free()`.
- `marker_required` maps to `WorkReportArtifactApprovalProofMarkerGatePolicy::require_present_markers()`.

Rules:

- callers may request a stricter policy than the workflow declares;
- callers must not weaken a workflow-declared policy;
- disabled caller policy must not erase a workflow declaration;
- absent or `not_required` workflow declaration must preserve existing explicit caller behavior;
- composition must be deterministic and tested.

## 13. Validation Behavior

Validation must fail closed when a declared requirement cannot be enforced.

Rules:

- unsupported proof-marker artifact requirements must be rejected;
- unknown requirement keys must be rejected when schema exposure exists;
- declarations must not include raw payloads, local paths, approval IDs, event IDs, projection IDs, hashes, evidence payloads, provider payloads, command output, or secret-like strings;
- declarations must not imply identity, quorum, revocation, side-effect, provider write, or hosted audit guarantees that do not exist;
- default validation must reject stronger proof-marker artifact requirements until an explicit artifact-capable validation posture can enforce them;
- validation errors must use stable non-leaking codes;
- diagnostic ordering must remain deterministic.

Candidate diagnostic codes:

- `validation.workflow.report_artifact_requirement.approval_proof_marker.invalid`
- `validation.workflow.report_artifact_requirement.approval_proof_marker.unsupported`
- `validation.workflow.report_artifact_requirement.approval_proof_marker.runtime_not_enforced`

## 14. Workflow Semantics And Failure Behavior

Workflow execution semantics must remain unchanged.

Rules:

- if a workflow declares a proof-marker artifact requirement and the runtime path cannot enforce it, validation should reject the workflow or artifact persistence should fail before write;
- proof-marker artifact gate failure must not change workflow pass/fail status;
- gate failure must not append workflow events;
- gate failure must not mutate the run, snapshot, approval decision, SideEffect records, projection records, or generated report;
- generated in-memory reports remain available when artifact persistence fails;
- no partial artifact may be written when the gate fails;
- errors must be stable and non-leaking.

This preserves the separation:

```text
workflow execution result != report artifact persistence result
```

## 15. Store And Persistence Boundary

All proof-marker projection stores must remain caller-supplied in the first runtime integration.

The implementation must not:

- infer projection store roots from state directories;
- create projection stores automatically;
- persist projection records automatically;
- repair or backfill projection records;
- read hidden stores;
- write artifacts unless the caller chose an explicit artifact-capable API;
- append workflow or audit events.

Automatic proof-marker projection persistence from executor paths remains separate work.

## 16. Privacy And Redaction

Declared artifact requirements must be posture-only.

They must not store, copy, or expose:

- approval presentation text;
- approval reasons;
- approval IDs;
- event IDs;
- projection IDs;
- presentation IDs;
- content hashes;
- local paths;
- report text;
- provider payloads;
- command output;
- CI logs;
- Jira or GitHub bodies;
- raw spec or source contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, token-like values, or secret-like metadata.

Errors and `Debug` output must remain bounded and non-leaking.

## 17. Relationship To Side Effects And Writes

This remains a prerequisite path before write-capable adapters, not write support itself.

Future write-capable adapter work should require:

- capability-gated write intent;
- proposed `SideEffect` record;
- policy decision;
- approval authority when needed;
- proof-marker-backed approval presentation for sensitive actions;
- high-assurance approval disclosure where required;
- WorkReport disclosure;
- proof-marker artifact gate where declared;
- idempotency;
- auditability;
- redaction-safe errors.

This plan does not authorize provider mutation, side-effect execution, or write-capable adapters.

## 18. Test Plan

Future implementation tests should cover:

- internal `not_required` posture maps to no proof-marker artifact gate;
- internal `projection_required` posture maps to marker-free-allowed projection policy;
- internal `marker_required` posture maps to present-marker-required policy;
- caller stricter than workflow declaration wins;
- caller weaker than workflow declaration cannot weaken the gate;
- unsupported future proof-marker requirement vocabulary fails closed;
- serialization/deserialization remains bounded and posture-only;
- invalid serialized requirement fails closed without leaking values;
- default validation rejects enforcement postures until artifact-capable validation exists;
- artifact-capable validation accepts only the selected workflow's enforceable proof-marker requirement;
- explicit executor artifact path derives workflow-declared proof-marker policy;
- missing required projection returns artifact error and writes no artifact;
- marker-free projection fails when `marker_required` is effective;
- successful persisted proof-marker projection allows artifact write when all other gates pass;
- artifact gate failure preserves run/report status and event history;
- no projection records are created or repaired;
- no hidden store roots are inferred;
- no CLI behavior, schema behavior, examples, provider calls, or writes are introduced in internal-model phases.

## 19. Proposed Implementation Sequence

Recommended small phases:

1. Internal proof-marker artifact requirement model and policy mapping.
2. Maintainer review of the internal model.
3. Schema/parser/SDK planning for `report_artifact_requirements.approval_proof_markers`.
4. Schema/parser/SDK implementation that rejects unenforceable postures outside explicit artifact-capable validation.
5. Pure runtime derivation helper from selected workflow to proof-marker artifact gate policy.
6. Maintainer review of derivation helper.
7. Explicit executor artifact-path integration that composes workflow-declared and caller-supplied proof-marker policy by strictness.
8. Review before considering any default enforcement, automatic projection persistence, or write-capable adapter readiness.

## 20. Deferred Work

Deferred:

- workflow schema exposure in this planning phase;
- automatic approval proof-marker projection persistence;
- automatic report artifact writing;
- default executor proof-marker enforcement;
- CLI artifact inspection or export;
- public approval cards;
- examples;
- hosted/distributed artifact stores;
- provider writes;
- runtime side-effect execution;
- reasoning lineage;
- release posture changes.

## 21. Final Recommendation

Proceed next with **workflow-declared proof-marker artifact requirement internal model implementation**.

The first implementation should add only internal posture vocabulary and deterministic mapping to existing proof-marker artifact gate policy. It must not add schema fields, CLI behavior, examples, automatic artifact writing, automatic projection persistence, default executor enforcement, provider writes, hosted behavior, reasoning lineage, or release posture changes.
