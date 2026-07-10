# Workflow-Declared Proof-Marker Artifact Requirement Schema Plan

Status: schema/parser/SDK vocabulary implemented in [Workflow-Declared Proof-Marker Artifact Requirement Schema Report](../concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md). The internal proof-marker artifact requirement model and policy mapping are implemented and accepted in [Workflow-Declared Proof-Marker Artifact Requirement Model Review](../concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_MODEL_REVIEW.md). Runtime derivation, executor integration, automatic artifacts, projection persistence, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, and release behavior remain unimplemented.

## 1. Executive Summary

Workflow OS now has internal posture vocabulary for terminal report artifact approval proof-marker requirements:

- `not_required`
- `projection_required`
- `marker_required`

The next question is how, when, and under what validation posture workflow YAML should be allowed to declare that terminal report artifact persistence requires approval proof-marker projection coverage.

This phase is planning only. It does not add `approval_proof_markers` to workflow specs, checked-in JSON schema, TypeScript SDK types, CLI output, examples, executor integration, artifact writing, automatic report generation, automatic projection persistence, provider writes, hosted behavior, or reasoning lineage.

## 2. Goals

- Define the future schema-facing boundary for `report_artifact_requirements.approval_proof_markers`.
- Preserve the invariant that declared governance must be enforced or rejected.
- Avoid decorative YAML fields that look enforceable but are ignored.
- Keep proof-marker requirements terminal-report-artifact scoped, not workflow pass/fail scoped.
- Define parser, schema, SDK, validation, and docs surfaces that must move together.
- Define how default semantic validation should fail closed for unenforceable postures.
- Preserve existing default executor behavior.
- Preserve explicit artifact-capable executor paths as the only initial enforcement path.
- Preserve redaction-safe diagnostics and deterministic validation ordering.
- Prepare a small implementation prompt that can be reviewed before runtime derivation changes.

## 3. Non-Goals

This plan does not authorize:

- implementation in this phase;
- checked-in JSON schema changes;
- Rust parser/model changes;
- TypeScript SDK changes;
- contract fixture changes;
- CLI rendering or commands;
- automatic report generation;
- automatic report artifact writing;
- automatic approval proof-marker projection persistence;
- default executor proof-marker enforcement;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- examples;
- public approval cards;
- approval evidence attachment;
- new workflow event types;
- provider writes;
- side-effect execution;
- hosted or distributed runtime;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented:

- approval decision proof markers for opt-in proof-enforced approvals;
- bounded proof-marker inspect/projection behavior;
- WorkReport approval proof-marker citation helpers;
- terminal report opt-in proof-marker citation integration;
- executor report input propagation for proof-marker citation policy;
- approval proof-marker audit projection helpers;
- local durable approval proof-marker projection persistence helper;
- in-memory and store-backed report artifact proof-marker gate helpers;
- helper-level artifact write composition with proof-marker gates;
- explicit executor artifact path proof-marker gate integration;
- internal `WorkReportArtifactApprovalProofMarkerRequirement` model and deterministic mapping.

Not implemented:

- workflow YAML field `report_artifact_requirements.approval_proof_markers`;
- checked-in schema support;
- TypeScript SDK support;
- semantic validation diagnostics for authored proof-marker artifact requirements;
- runtime derivation from workflow specs;
- automatic report artifact writing;
- automatic projection persistence.

## 5. Candidate Schema Shape

Future conceptual YAML:

```yaml
report_artifact_requirements:
  approval_proof_markers: marker_required
```

Allowed values:

- `not_required`
- `projection_required`
- `marker_required`

Meaning:

- `not_required`: do not require approval proof-marker projection coverage before terminal report artifact persistence.
- `projection_required`: every approval citation in the artifact must resolve to one durable projection record; marker-free projections are acceptable only when the projection explicitly says proof markers were not required.
- `marker_required`: every approval citation in the artifact must resolve to a projection record with a present approval decision proof marker.

The field must remain posture-only. It must not accept IDs, paths, hashes, payloads, reasons, actor names, approval text, policy expressions, store roots, or provider data.

## 6. Public Contract Surfaces

A future schema implementation must update these surfaces together:

- `crates/workflow-core/src/definitions.rs`
- `crates/workflow-core/src/validation.rs`
- `crates/workflow-core/tests/project_specs.rs`
- `crates/workflow-core/tests/project_validation.rs`
- `crates/workflow-core/tests/local_executor.rs` only if artifact-capable validation/runtime behavior is included in that phase
- `schemas/v0/workflow.schema.json`
- `packages/sdk-typescript/src/index.ts`
- `packages/sdk-typescript/dist/index.d.ts`
- `packages/sdk-typescript/test/contract-fixtures.mjs`
- `packages/sdk-typescript/test/spec-generation.test.mjs` if generated workflow fixtures include artifact requirements
- `docs/specs/workflows.md`
- related implementation plans and concept reports

The implementation must run `npm run check:contracts` if schema/SDK contract surfaces change.

## 7. Validation Strategy

Validation must prevent false governance.

Recommended default validation behavior:

- `not_required` passes default semantic validation.
- `projection_required` is parsed but rejected by default semantic validation with a stable runtime-not-enforced diagnostic unless the validation path is explicitly artifact-capable.
- `marker_required` is parsed but rejected by default semantic validation with the same runtime-not-enforced posture unless the validation path is explicitly artifact-capable.
- unknown enum values fail parser/schema validation.
- unknown keys under `report_artifact_requirements` fail schema/parser validation.

Candidate diagnostic codes:

- `validation.workflow.report_artifact_requirement.approval_proof_marker.invalid`
- `validation.workflow.report_artifact_requirement.approval_proof_marker.runtime_not_enforced`
- `validation.workflow.report_artifact_requirement.approval_proof_marker.unsupported`

The existing high-assurance artifact requirement behavior is the precedent: enforcement postures may be schema-known while default validation rejects them until an explicit artifact-capable path can enforce them.

## 8. Artifact-Capable Validation Boundary

The future implementation should define or reuse an explicit artifact-capable validation posture that can accept enforceable proof-marker declarations only when the selected runtime path can satisfy them.

Rules:

- default `workflow-os validate` must not accept enforceable proof-marker artifact requirements unless default runtime behavior can enforce them;
- explicit artifact-capable validation may accept the selected workflow's proof-marker requirement;
- validation must be scoped to the selected workflow, not all workflows in the project;
- validation must not infer projection stores;
- validation must not write projection records or artifacts;
- validation must not append workflow events;
- diagnostics must remain deterministic and non-leaking.

## 9. Parser And Schema Requirements

Parser/schema requirements:

- parse `approval_proof_markers` as a bounded enum;
- default absent field to `not_required`;
- reject unknown values;
- reject object/string payloads that are not exact enum values;
- reject unknown fields under `report_artifact_requirements`;
- keep `approval_proof_markers` sibling to `high_assurance_approval`;
- preserve backward compatibility for workflows that omit the field;
- avoid accepting aliases in the first public slice unless compatibility requires them.

Schema examples must stay minimal and must not imply automatic artifact writing.

## 10. TypeScript SDK Requirements

TypeScript SDK updates should mirror the schema exactly.

Required future changes:

- add `approval_proof_markers?: ReportArtifactApprovalProofMarkerRequirement`;
- define `ReportArtifactApprovalProofMarkerRequirement` as a string union;
- preserve optional field behavior;
- update generated/contract fixtures;
- ensure helper APIs cannot emit unsupported values;
- ensure fixtures demonstrate no-op posture first unless enforcement posture tests are explicitly covered.

The SDK must not introduce runtime behavior, CLI behavior, artifact writing, projection persistence, provider writes, or hosted assumptions.

## 11. Runtime Relationship

Schema exposure must not itself change runtime semantics.

Future runtime derivation should be a separate implementation phase:

- derive `WorkReportArtifactApprovalProofMarkerRequirement` from the selected workflow;
- map it to `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- compose it with caller-supplied policy by strictness;
- preserve caller ability to request stricter policy than the workflow declares;
- prevent callers from weakening a workflow-declared policy;
- fail artifact persistence before write when the effective gate is not satisfied.

This plan does not implement that runtime derivation.

## 12. Privacy And Redaction

The field must remain posture-only.

It must not store, copy, or expose:

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

Errors and debug output must remain bounded and must not echo invalid raw payloads.

## 13. Compatibility And Migration

Backward compatibility:

- workflows without `report_artifact_requirements` remain valid;
- workflows with only `high_assurance_approval` keep existing behavior;
- workflows with `approval_proof_markers: not_required` remain behaviorally equivalent to absence;
- no default executor behavior changes.

Forward compatibility:

- schema-known enforcement postures are allowed only if validation can fail closed when runtime enforcement is unavailable;
- release notes must call the field experimental or preview-scoped if exposed before full artifact-path integration is complete.

## 14. Test Plan

Future schema implementation tests should cover:

- parser accepts absence of `approval_proof_markers`;
- parser accepts `not_required`;
- parser accepts known enforcement posture values while semantic validation decides whether they are enforceable;
- parser rejects unknown proof-marker values;
- parser rejects unknown requirement keys;
- schema accepts the same valid shapes as Rust parser;
- schema rejects unknown values and object payloads;
- TypeScript SDK string union matches schema values;
- TypeScript fixtures round-trip through Rust validation;
- default semantic validation accepts `not_required`;
- default semantic validation rejects `projection_required` and `marker_required` with stable runtime-not-enforced diagnostics;
- artifact-capable validation, if included, accepts selected workflow enforcement postures;
- validation diagnostics preserve ordering;
- diagnostics do not leak raw invalid payloads;
- existing high-assurance artifact requirement tests continue to pass;
- existing WorkReport, executor artifact path, projection store, and proof-marker gate tests continue to pass.

Required commands for future implementation:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:contracts`
- `npm run check:docs`

## 15. Proposed Implementation Sequence

Recommended small phases:

1. Implement schema/parser/SDK vocabulary and default semantic validation rejection for enforceable postures.
2. Review schema/parser/SDK phase.
3. Implement pure runtime derivation helper from selected workflow to proof-marker artifact gate policy.
4. Review derivation helper.
5. Integrate explicit executor artifact path with workflow-declared proof-marker policy composition.
6. Review executor artifact-path integration.
7. Only after review, consider automatic projection persistence planning or default artifact behavior.

The first implementation phase should not combine schema exposure with executor artifact-path integration. Keeping them separate makes the false-governance boundary easier to review.

## 16. Deferred Work

Deferred:

- runtime derivation from workflow specs;
- executor artifact-path integration;
- automatic projection persistence;
- automatic report artifact writing;
- automatic report generation;
- default executor enforcement;
- CLI artifact commands;
- examples;
- public approval cards;
- hosted/distributed runtime;
- provider writes;
- side-effect execution;
- reasoning lineage;
- release posture changes.

## 17. Final Recommendation

Proceed next with schema/parser/SDK vocabulary implementation only, with default semantic validation rejecting `projection_required` and `marker_required` until an explicit artifact-capable path is selected.

Do not implement runtime derivation, executor integration, automatic artifact writing, automatic projection persistence, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes in that implementation phase.
