# Workflow-Declared High-Assurance Artifact Requirement Plan

Status: First internal model and policy-mapping slice implemented in [Workflow-Declared High-Assurance Artifact Requirement Model Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_MODEL_REPORT.md) and accepted in [Workflow-Declared High-Assurance Artifact Requirement Model Review](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_MODEL_REVIEW.md). Workflow schema exposure is implemented as a narrow schema/parser/SDK/validation slice in [Workflow-Declared High-Assurance Artifact Requirement Schema Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md), following [Workflow-Declared High-Assurance Artifact Requirement Schema Plan](workflow-declared-high-assurance-artifact-requirement-schema-plan.md). Runtime derivation from workflow specs, runtime config, automatic report generation, automatic artifact writing, CLI behavior, examples, RBAC/IdP, quorum approval, revocation enforcement, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, and release posture changes are not implemented.

## 1. Executive Summary

Workflow OS now has an explicit opt-in report artifact gate that can require a generated `WorkReport` to carry bounded high-assurance approval disclosure before the report artifact is persisted.

Today that gate is caller-supplied. A caller can explicitly request `require_disclosure`, `require_validated`, or stricter fail-closed disclosure posture when using the governed artifact write path.

The next design question is how authored workflows should eventually express that terminal report artifacts require high-assurance approval disclosure. This plan answers that question without implementing it. The core recommendation is to keep the first workflow-declared requirement narrow, local, deterministic, and artifact-scoped, and to route it through already-reviewed report/artifact gates rather than inventing a broad policy DSL.

## 2. Goals

- Define the future authored-workflow declaration boundary for high-assurance report artifact requirements.
- Preserve the invariant that declared governance must be enforced or rejected.
- Avoid decorative workflow metadata that looks enforceable but is ignored.
- Keep report artifact requirements separate from workflow execution pass/fail semantics.
- Reuse existing `WorkReport` high-assurance approval disclosure.
- Reuse the explicit report artifact high-assurance disclosure gate.
- Preserve local deterministic validation.
- Preserve redaction-safe report and artifact behavior.
- Prepare a small future implementation slice that can be reviewed before schema exposure.
- Keep write-capable adapters blocked until governance prerequisites remain composed and reviewed.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- workflow schema changes;
- public schema publication;
- runtime config;
- automatic report generation;
- automatic artifact writing from existing executor paths;
- changes to `LocalExecutor::execute(...)`;
- changes to `LocalExecutor::execute_with_report(...)`;
- CLI rendering or export commands;
- example updates;
- RBAC, IdP, SSO, SCIM, teams, groups, or directory integration;
- quorum or multi-party approval enforcement;
- role-bound approval authority;
- revocation enforcement;
- protected-use expiration checkpoints;
- high-assurance validation result persistence;
- approval evidence attachment;
- new workflow event types;
- audit projection for high-assurance artifact requirements;
- side-effect execution;
- provider mutation or write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented today:

- `HighAssuranceApprovalControl` model and validation.
- Pure high-assurance approval decision validation helper.
- Explicit `LocalExecutor::decide_approval_with_high_assurance(...)`.
- Explicit `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`.
- `WorkReportHighAssuranceApprovalDisclosure`.
- Terminal report generation support for explicit high-assurance disclosure input.
- High-assurance disclosure discovery from explicit validation results.
- `WorkReportArtifactRecord` and local `WorkReportArtifactStore`.
- Explicit governed artifact write helper with SideEffect referential integrity and approval-linkage gates.
- Explicit opt-in high-assurance approval disclosure gate for report artifact writes.

Not implemented today:

- workflow-declared high-assurance approval controls;
- workflow-declared report artifact requirements;
- workflow-declared artifact persistence;
- automatic executor report artifact writes;
- workflow schema fields for report artifacts or high-assurance disclosure;
- runtime configuration for artifact gates;
- CLI report artifact persistence or rendering.

## 5. Problem Statement

The explicit artifact gate proves that a caller can require high-assurance approval disclosure before persisting a report artifact.

That is not enough for authored governance at workflow scale. If a workflow represents sensitive or high-impact work, the authored workflow should eventually be able to declare:

```text
terminal report artifacts for this workflow must disclose high-assurance approval posture before they may be persisted
```

Without a declared requirement, the burden remains on every caller to remember the right artifact gate policy. That is workable for a narrow explicit API, but it is not a durable governance contract.

The risk is false governance in the opposite direction: adding YAML fields that look like artifact requirements before runtime validation and artifact-write enforcement actually exist. This plan keeps the next step explicit: any declaration must either be enforced through the reviewed gate or rejected.

## 6. Source-Of-Truth Boundaries

The future declaration should not make `WorkReport` or artifact storage the source of truth for approval decisions.

Boundaries:

- workflow spec: authored intent and requirements;
- workflow validation: rejects unsupported or unenforceable requirements;
- workflow event log: source of truth for workflow execution state and approval events;
- approval projection: rebuildable lookup cache;
- high-assurance validation helper: explicit in-memory decision validation;
- `WorkReport`: governed handoff artifact carrying bounded disclosure;
- report artifact gate: explicit pre-persistence enforcement;
- report artifact store: durable artifact storage after validation and gates pass.

The declaration should select an artifact gate policy. It should not claim enterprise identity proof, quorum approval, revocation enforcement, provider write authorization, or side-effect completion.

## 7. Candidate Declaration Shape

A future workflow-authored requirement should be small and posture-based.

Candidate conceptual shape:

```yaml
report_artifact:
  high_assurance_approval:
    required: true
    validation: required
    denial_behavior: fail_closed
```

This is conceptual only. No schema fields are implemented by this plan.

The first real implementation should avoid broad policy expressions. It should map to the already-implemented gate policies:

- disabled;
- require disclosure present;
- require validation used;
- require validation passed;
- require validation passed with fail-closed denial posture.

The declaration should not embed raw approval payloads, actor IDs, evidence payloads, policy payloads, local paths, provider payloads, command output, or secret-like values.

## 8. Recommended First Implementation Boundary

Recommended first implementation: model and validate an internal workflow artifact requirement value before public schema exposure.

Why:

- keeps the authored requirement shape reviewable before schema compatibility promises attach;
- lets tests prove enforcement/rejection behavior without adding YAML schema fields;
- preserves the invariant that declared governance is either enforced or rejected;
- avoids widening the CLI or examples too early.

Candidate first slice:

1. Add a small internal model for terminal report artifact requirements.
2. Map the high-assurance approval disclosure requirement to `WorkReportArtifactHighAssuranceDisclosurePolicy`.
3. Validate unsupported requirement combinations fail closed.
4. Add tests proving the mapping is deterministic and redaction-safe.
5. Do not wire the model to workflow YAML until a separate schema plan is reviewed.

Only after that model is reviewed should Workflow OS consider workflow spec fields.

## 9. Runtime Integration Boundary

The future runtime integration point should remain the explicit artifact-bearing executor path, not the base executor path.

Allowed future integration:

- a report artifact input path may accept a validated artifact requirement derived from workflow declaration;
- the artifact write helper maps that requirement to existing gate policies;
- report generation remains explicit;
- artifact writing remains explicit until a separate automatic artifact-write phase is accepted.

Rejected for first implementation:

- automatic artifact writing after every terminal run;
- changing default `execute(...)`;
- changing default `execute_with_report(...)`;
- appending events for report artifact gate evaluation;
- mutating snapshots;
- adding CLI artifact write behavior;
- using runtime config to silently opt workflows into gates.

## 10. Validation Behavior

Validation must fail closed when a declared requirement cannot be enforced.

Rules:

- unsupported high-assurance artifact requirements must be rejected;
- unknown requirement keys must be rejected when schema exposure exists;
- declaration must not accept raw payloads or secret-like strings;
- declaration must not imply identity guarantees that do not exist;
- declaration must not imply quorum or revocation enforcement before those are implemented;
- declaration must not imply side-effect execution or write authorization;
- validation errors must use stable non-leaking codes;
- validation must preserve deterministic diagnostic ordering.

The future schema-facing implementation should include explicit diagnostics that tell the author whether the field is supported, unsupported, or deferred.

## 11. Workflow Semantics And Failure Behavior

Workflow execution semantics must remain unchanged.

Rules:

- if a workflow declares a high-assurance artifact requirement and the runtime path cannot enforce it, validation should reject the workflow or the artifact write request should fail before persistence;
- artifact gate failure must not change workflow pass/fail status;
- artifact gate failure must not append workflow events;
- artifact gate failure must not mutate the run, snapshot, approval decision, or SideEffect records;
- generated in-memory reports remain available when artifact persistence fails;
- no partial artifact may be written when the gate fails;
- error output must be stable and non-leaking.

This preserves the current separation:

```text
workflow execution result != report artifact persistence result
```

## 12. Relationship To Governance Profiles

Governance profiles should eventually decide whether high-assurance artifact requirements are mandatory, optional, or disallowed.

Likely posture:

- `observe_and_report`: may disclose missing high-assurance posture without requiring artifact gate success;
- `agent_assisted_gated`: may require agent-supplied report disclosure when deterministic validation supports it;
- `human_approval_gated`: may require high-assurance disclosure for selected sensitive workflows;
- `strict_enterprise`: may require steward-managed high-assurance artifact requirements before persistence.

This plan does not implement governance profiles. It only preserves the separation point.

## 13. Relationship To High-Assurance Approval Controls

Workflow-declared artifact requirements should not replace high-assurance approval controls.

High-assurance approval controls govern approval decision posture. Artifact requirements govern whether a terminal `WorkReport` artifact may be persisted unless it discloses that posture.

The relationship should be:

```text
approval control validation -> report-safe disclosure -> artifact requirement gate
```

The artifact requirement should not infer that validation happened. It should require the disclosure already carried by the report.

## 14. Relationship To Side Effects And Writes

This remains a prerequisite path before write-capable adapters, not write support itself.

Future write-capable adapter work should require:

- policy-gated capability;
- proposed SideEffect record;
- approval authority where needed;
- high-assurance approval validation for sensitive actions;
- WorkReport disclosure;
- report artifact gate, where required;
- idempotency;
- auditability;
- redaction-safe errors.

This plan does not authorize provider mutation, side-effect execution, or write-capable adapters.

## 15. Privacy And Redaction

Declared artifact requirements must be posture-only.

They must not store, copy, or expose:

- raw approval request payloads;
- raw approval decision payloads;
- actor IDs;
- high-assurance control payload bodies;
- evidence payloads;
- policy payloads;
- provider payloads;
- command output;
- CI logs;
- Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- local paths;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values;
- secret-like metadata.

Debug, Display, validation diagnostics, and serialized requirement output must remain redaction-safe.

## 16. Test Plan For Future Implementation

Future implementation tests should cover:

- valid internal artifact requirement maps to disabled policy;
- valid internal artifact requirement maps to require-disclosure policy;
- valid internal artifact requirement maps to require-validated policy;
- valid internal artifact requirement maps to fail-closed validated policy;
- unsupported quorum/revocation/role-bound declarations fail closed;
- unsupported schema-facing fields fail validation when schema exposure exists;
- declaration errors do not leak raw values;
- artifact write succeeds when declared requirement maps to a gate and report disclosure satisfies it;
- artifact write fails before persistence when declared requirement maps to a gate and report disclosure is absent;
- artifact gate failure preserves workflow run status, generated report, approvals, events, and snapshots;
- no artifact is written on gate failure;
- no CLI, schema, examples, runtime config, side-effect execution, or provider writes are introduced in the first internal-model slice.

## 17. Proposed Implementation Sequence

Recommended sequence:

1. Implement an internal terminal report artifact requirement model, no schema exposure.
2. Map the high-assurance disclosure requirement to existing artifact gate policy.
3. Add model/mapping tests and non-leakage tests.
4. Review.
5. Plan workflow schema exposure separately.
6. Implement workflow validation support only after schema posture is accepted.
7. Plan explicit runtime artifact path wiring for validated declarations.
8. Defer automatic artifact writing, CLI rendering, examples, hosted behavior, and write-capable adapters.

This keeps the next implementation code-bearing while avoiding the false-governance trap of adding decorative YAML first.

## 18. Deferred Work

Deferred:

- public workflow schema fields;
- workflow-declared high-assurance approval controls;
- workflow-declared report artifact persistence;
- automatic terminal report generation;
- automatic artifact writes;
- CLI artifact inspection/rendering;
- examples;
- RBAC/IdP;
- quorum approval;
- approval revocation;
- approval evidence attachment;
- stable high-assurance validation result records;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage.

## 19. Open Questions

- Should the first schema-facing field live under report requirements, artifact requirements, or governance profiles?
- Should a workflow be invalid if it declares artifact requirements but execution uses a non-artifact path?
- Should observe/report-only profiles downgrade artifact requirement failure to disclosure-only posture?
- How should enterprise steward policies override or require workflow-local artifact requirements?
- Should artifact requirement evaluation ever produce an audit projection, or remain report/artifact result only?
- When should CLI expose artifact gate results, and in what redaction-safe format?
- What is the smallest useful schema contract that avoids future compatibility churn?

## 20. Final Recommendation

Next implementation phase: **internal workflow artifact requirement model and policy mapping, model-only**.

That phase should add the smallest internal model that can express terminal report artifact high-assurance disclosure requirements and map them to the existing `WorkReportArtifactHighAssuranceDisclosurePolicy`.

It must still not add workflow schema fields, runtime config, automatic artifact writing, CLI behavior, examples, RBAC/IdP, quorum, revocation, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.
