# Workflow-Declared High-Assurance Artifact Requirement Schema Plan

Status: Implemented as a schema/parser/SDK/validation slice in [Workflow-Declared High-Assurance Artifact Requirement Schema Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md). Workflow specs may declare `report_artifact_requirements.high_assurance_approval: not_required`; stronger enforcement postures are parsed and schema-known but rejected by semantic validation until runtime artifact derivation exists. Runtime derivation from workflow specs, automatic report generation, automatic artifact writing, CLI behavior, examples, RBAC/IdP, quorum approval, revocation enforcement, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, and release posture changes are not implemented.

## 1. Executive Summary

Workflow OS now has an internal `WorkReportArtifactRequirement` model that can map high-assurance approval disclosure requirements to the explicit `WorkReportArtifactHighAssuranceDisclosurePolicy` used by the governed report artifact write path.

The next question is how to expose that requirement in authored workflow specs without creating false governance. This plan defines the smallest future schema-facing boundary. The recommended field is posture-only, maps directly to the reviewed model, and must fail validation when the runtime path cannot enforce it.

This plan does not implement schema exposure. It prepares the next implementation prompt.

## 2. Goals

- Let workflow authors eventually declare terminal report artifact high-assurance disclosure requirements.
- Preserve the invariant that declared governance must be enforced or rejected.
- Keep the first schema field small, posture-only, deterministic, and artifact-scoped.
- Map authored declarations to the existing `WorkReportArtifactRequirement` model.
- Map validated requirements to the existing explicit artifact gate policy.
- Keep workflow execution pass/fail semantics separate from artifact persistence semantics.
- Avoid raw approval payloads, actor IDs, evidence payloads, policy payloads, provider payloads, command output, local paths, and secret-like strings.
- Preserve schema compatibility discipline across Rust, checked-in JSON schema, TypeScript SDK contracts, docs, and tests.
- Prepare for future runtime wiring without implementing it in the schema slice.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic report generation;
- automatic report artifact writing;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- changing explicit artifact write semantics;
- CLI rendering, export, or artifact write commands;
- example updates in the first schema implementation unless required by contract tests;
- broad policy DSLs;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, SSO, SCIM, groups, teams, or directory integration;
- quorum or multi-party approval enforcement;
- role-bound authority enforcement;
- revocation enforcement;
- approval evidence attachment;
- new workflow events;
- audit projection for artifact requirements;
- side-effect execution;
- provider mutation or write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented:

- explicit report artifact high-assurance disclosure gate;
- internal `WorkReportArtifactRequirement` model;
- deterministic mapping from internal requirement posture to `WorkReportArtifactHighAssuranceDisclosurePolicy`;
- fail-closed validation for unsupported future requirement vocabulary;
- serde support for the internal model;
- model review accepted with non-blocking follow-ups.

Not implemented:

- workflow spec field;
- JSON schema field;
- TypeScript SDK field;
- project validation diagnostics for declared artifact requirements;
- runtime derivation from workflow specs;
- artifact write request construction from workflow declarations.

## 5. Compatibility Surfaces

Future schema exposure must update these surfaces together:

- Rust `WorkflowDefinition` in `workflow-core`;
- Rust parser and project loader tests;
- Rust semantic validation diagnostics;
- checked-in `schemas/v0/workflow.schema.json`;
- TypeScript SDK workflow input/output types;
- TypeScript contract fixtures and generated project tests;
- `docs/specs/workflows.md`;
- roadmap and adjacent high-assurance/report artifact planning docs;
- `npm run check:contracts`.

Because v0 schemas are manually checked in, schema exposure must not be treated as a Rust-only change.

## 6. Recommended Field Placement

Recommended first schema-facing field:

```yaml
report_artifact_requirements:
  high_assurance_approval: validated_fail_closed_disclosure_required
```

Why this placement:

- it is workflow-level because the requirement applies to terminal report artifacts for the workflow;
- it is artifact-scoped and does not imply approval enforcement for every step;
- it maps directly to the internal `WorkReportArtifactRequirementDefinition`;
- it avoids overloading `approval_requirements`, which governs approval intent rather than artifact persistence;
- it keeps report artifact requirements separate from `audit_requirements` and `observability_requirements`.

Rejected placements:

- `approval_requirements`: conflates approval decision posture with report artifact persistence posture;
- `audit_requirements`: report artifacts are governed handoff artifacts, not audit events;
- `observability_requirements`: artifact disclosure is not telemetry;
- step-level field: first requirement is terminal artifact scoped, not per-step;
- policy rule effects: this is not a broad policy DSL.

## 7. Recommended Field Shape

Recommended Rust-facing model:

```rust
pub report_artifact_requirements: WorkReportArtifactRequirementDefinition
```

Recommended schema enum values:

- `not_required`
- `disclosure_required`
- `validated_disclosure_required`
- `validated_fail_closed_disclosure_required`

The schema-facing shape should not expose `unsupported_high_assurance_requirements`. That field is useful for internal fail-closed model tests, but a workflow author should not be invited to declare future governance vocabulary that the runtime cannot enforce.

Unknown fields and unknown enum values must fail closed through `deny_unknown_fields`, JSON schema `additionalProperties: false`, and semantic validation diagnostics where possible.

## 8. Validation Semantics

Future implementation must validate:

- authored `report_artifact_requirements` deserializes through validated Rust types;
- unsupported or unknown high-assurance artifact requirement values fail closed;
- the requirement does not include raw payloads, local paths, actor IDs, approval IDs, evidence payloads, policy payloads, provider payloads, command output, or secret-like strings;
- workflow diagnostics use stable non-leaking codes;
- diagnostic ordering remains deterministic;
- validation does not generate reports, write artifacts, inspect runtime events, call adapters, evaluate live policy, or read state backends.

Recommended diagnostic codes:

- `validation.workflow.report_artifact_requirement.invalid`
- `validation.workflow.report_artifact_requirement.unsupported`
- `validation.workflow.report_artifact_requirement.runtime_not_enforced`

The first schema implementation should decide whether declarations are accepted as schema-valid but warned as runtime-not-enforced, or rejected until runtime wiring exists. The safer recommendation is to reject any declaration that cannot be enforced by the currently approved explicit artifact path used by the caller.

## 9. Runtime Semantics

Schema exposure alone must not change runtime behavior.

Rules:

- `LocalExecutor::execute(...)` remains unchanged;
- `LocalExecutor::execute_with_report(...)` remains unchanged;
- explicit artifact write APIs remain explicit;
- no automatic artifact write occurs because a workflow declares a requirement;
- if a runtime path later reads the declaration, it must map the declaration to `WorkReportArtifactHighAssuranceDisclosurePolicy` before artifact persistence;
- if a runtime path cannot enforce the declaration, artifact persistence must fail before write or workflow validation must reject the workflow;
- artifact gate failure must not mutate workflow status, events, snapshots, approvals, SideEffect records, or generated reports.

## 10. Schema And Versioning Posture

This is a v0 schema change and must be treated as a public compatibility surface.

Rules:

- update `schemas/v0/workflow.schema.json` in the same implementation as Rust parsing;
- keep `schema_version: workflowos.dev/v0`;
- update TypeScript SDK types/builders in the same implementation;
- update contract tests so generated workflow specs can include the field and still pass Rust validation when enforcement posture is accepted;
- add invalid schema fixture coverage for unknown enum values and unknown nested fields;
- document that this is local preview schema behavior, not production artifact automation.

## 11. TypeScript SDK Posture

The TypeScript SDK should expose the same posture enum vocabulary as Rust.

Recommended TS additions:

- `ReportArtifactHighAssuranceApprovalRequirement`;
- `ReportArtifactRequirements`;
- optional `report_artifact_requirements` on workflow input/output.

The SDK must not add helper behavior that implies runtime report generation, artifact writing, or high-assurance approval enforcement. It should only emit typed specs.

## 12. Documentation And Examples Posture

Docs should explain:

- the field declares artifact persistence requirements, not approval enforcement;
- the field does not create a report;
- the field does not write an artifact;
- the field does not make high-assurance validation happen by itself;
- runtime paths must explicitly enforce or reject declared requirements;
- write-capable adapters remain unsupported.

Examples should remain unchanged in the first schema implementation unless a contract fixture needs a minimal schema test. Public examples should not imply artifact automation until runtime wiring is accepted and reviewed.

## 13. Privacy And Redaction

The schema field must remain posture-only.

It must not store:

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

Errors must not echo untrusted payloads or secret-like values.

## 14. Test Plan

Future implementation tests should cover:

- workflow YAML with no `report_artifact_requirements` remains valid;
- workflow YAML with `high_assurance_approval: not_required` parses and validates according to accepted runtime posture;
- workflow YAML with `disclosure_required` parses and maps to `require_disclosure()`;
- workflow YAML with `validated_disclosure_required` parses and maps to `require_validated()`;
- workflow YAML with `validated_fail_closed_disclosure_required` parses and maps to `require_validated_fail_closed()`;
- unknown high-assurance posture fails closed without leaking value;
- unknown nested field fails schema/Rust parsing;
- schema allows only the documented field shape;
- TypeScript SDK can emit a valid workflow with the requirement field;
- TypeScript SDK invalid fixture fails Rust validation;
- `npm run check:contracts` passes;
- existing examples and dogfood workflows remain valid;
- no runtime artifact write occurs from validation or parsing;
- no CLI behavior is introduced.

## 15. Proposed Implementation Sequence

Recommended next implementation slice:

1. Add `report_artifact_requirements` to Rust `WorkflowDefinition`, defaulting to no requirement.
2. Reuse `WorkReportArtifactRequirementDefinition` or a schema-facing wrapper that validates into `WorkReportArtifactRequirement`.
3. Add semantic validation for unsupported or unenforceable declarations.
4. Update `schemas/v0/workflow.schema.json`.
5. Update TypeScript SDK workflow types/builders.
6. Add Rust parser/validation tests and TypeScript contract tests.
7. Update `docs/specs/workflows.md` and adjacent status docs.
8. Do not wire runtime artifact writes in this phase.
9. Review before any runtime derivation or artifact path integration.

## 16. Deferred Work

Deferred:

- runtime derivation from workflow specs;
- automatic report generation;
- automatic artifact writing;
- CLI artifact persistence/rendering;
- workflow-declared high-assurance approval controls;
- governance profile/steward overrides;
- RBAC/IdP;
- quorum approval;
- revocation enforcement;
- approval evidence attachment;
- stable high-assurance validation result records;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage.

## 17. Open Questions

- Should a workflow declaration be rejected until an artifact-writing runtime path explicitly opts into enforcing it, or should validation accept it while artifact persistence later fails if unenforced?
- Should `report_artifact_requirements` be optional with a default of `not_required`, or should authored workflows be encouraged to declare `not_required` explicitly for high-risk profiles?
- Should governance strictness profiles eventually require or override this field?
- Should the first runtime wiring read the declaration from the loaded workflow or require callers to pass the derived requirement explicitly?
- Should future schema versions split report generation requirements from artifact persistence requirements?

## 18. Final Recommendation

Next implementation phase: workflow-declared high-assurance artifact requirement schema field, parser, schema, SDK, docs, and validation tests only.

That phase should expose a small `report_artifact_requirements.high_assurance_approval` posture field and prove Rust, JSON schema, TypeScript SDK, and contract checks stay synchronized. It must not implement automatic report generation, automatic artifact writing, runtime config, CLI behavior, examples, approval evidence attachment, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.
