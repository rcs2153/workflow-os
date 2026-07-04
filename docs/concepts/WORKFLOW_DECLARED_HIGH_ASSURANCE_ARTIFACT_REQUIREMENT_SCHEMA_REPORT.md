# Workflow-Declared High-Assurance Artifact Requirement Schema Report

## 1. Executive Summary

The workflow-declared high-assurance artifact requirement schema slice is implemented.

Workflow specs can now declare the schema-facing `report_artifact_requirements.high_assurance_approval` field. The no-op posture `not_required` parses and validates. Stronger high-assurance artifact enforcement postures are recognized by the parser, JSON schema, Rust model, and TypeScript SDK, but semantic validation rejects them with a stable fail-closed diagnostic until runtime artifact derivation is implemented.

This keeps authored YAML honest: a workflow cannot appear to require high-assurance report artifact disclosure unless the runtime path can actually enforce that requirement.

## 2. Scope Completed

- Added `ReportArtifactRequirements` to the Rust workflow definition model.
- Added `report_artifact_requirements.high_assurance_approval` to checked-in v0 workflow JSON schema.
- Exposed the same vocabulary through the TypeScript SDK workflow types.
- Reused the existing `WorkReportArtifactHighAssuranceRequirement` posture vocabulary.
- Added semantic validation that rejects enforcement postures before runtime derivation exists.
- Preserved existing workflow validation behavior when the field is absent.
- Added parser tests for valid posture parsing and fail-closed unknown field/value handling.
- Added validation tests for `not_required` and runtime-not-enforced rejection.
- Added SDK contract coverage for valid and rejected generated workflow specs.
- Updated workflow spec and adjacent roadmap/planning docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- runtime derivation from workflow specs;
- automatic report generation;
- automatic report artifact writing;
- executor artifact request construction from workflow declarations;
- CLI artifact behavior;
- example updates;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, quorum approval, or revocation enforcement;
- approval evidence attachment;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Schema Field Summary

The new workflow field is:

```yaml
report_artifact_requirements:
  high_assurance_approval: not_required
```

Recognized posture values are:

- `not_required`
- `disclosure_required`
- `validated_disclosure_required`
- `validated_fail_closed_disclosure_required`

Only `not_required` validates successfully today. The enforcement values are intentionally schema-known but semantically rejected until a runtime path can enforce them.

## 5. Validation Boundary Summary

Validation preserves the invariant that declared governance must be enforced or rejected.

Current behavior:

- missing `report_artifact_requirements` defaults to no requirement;
- `high_assurance_approval: not_required` passes;
- `disclosure_required`, `validated_disclosure_required`, and `validated_fail_closed_disclosure_required` fail with `validation.workflow.report_artifact_requirement.runtime_not_enforced`;
- unknown nested fields fail Rust parsing and JSON schema validation;
- unknown posture values fail Rust parsing and JSON schema validation.

Validation does not generate reports, write artifacts, inspect runtime events, call adapters, read state backends, or mutate workflow state.

## 6. Compatibility Surface Summary

Updated compatibility surfaces:

- Rust `WorkflowDefinition`;
- Rust parser and project validation tests;
- checked-in `schemas/v0/workflow.schema.json`;
- TypeScript SDK source types;
- TypeScript SDK source types and generated-output build validation;
- SDK contract fixtures;
- contract check coverage;
- workflow spec docs;
- roadmap and adjacent implementation-plan status docs.

The field keeps `schema_version: workflowos.dev/v0`.

## 7. Redaction And Privacy Summary

The field is posture-only. It stores no:

- approval payloads;
- actor IDs;
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
- credentials, authorization headers, private keys, or token-like values.

Validation errors use stable codes and do not require echoing untrusted payloads.

## 8. Test Coverage Summary

Added or updated tests for:

- workflow YAML parsing of the requirement field;
- mapping parsed posture vocabulary to artifact gate policy vocabulary;
- unknown nested field rejection;
- unknown posture value rejection;
- semantic validation pass for `not_required`;
- semantic validation fail-closed behavior for enforcement posture before runtime derivation;
- TypeScript SDK generated valid `not_required` project;
- TypeScript SDK generated rejected enforcement posture project;
- checked-in schema and contract synchronization.

Existing WorkReport artifact requirement tests remain unchanged.

## 9. Commands Run And Results

- `npm run build --workspace @workflow-os/sdk-typescript` - passed.
- `cargo fmt --all` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:contracts` - passed after rebuilding the local CLI binary through Rust validation; the first attempt used a stale local `target/debug/workflow-os`.
- `npm run check:ts` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Dogfood Governance

This implementation phase was governed by the local Workflow OS dogfood runner.

- workflow phase: implementation
- run ID: `run-1783131104787600000-2`
- approval ID: `approval/run-1783131104787600000-2/implementation-approved`
- approval outcome: approved by the maintainer before implementation work continued
- close status: completed
- event summary: 39 events total; 1 approval; 0 retries; 0 escalations

The dogfood runner coordinates governance only. Repo edits, tests, and reporting were performed by the executor.

## 11. Remaining Known Limitations

- Enforcement postures cannot be used in valid workflow specs yet.
- Runtime artifact write paths do not read workflow declarations.
- The executor does not derive artifact gate policy from workflow specs.
- Missing runtime derivation means this is a schema compatibility step, not a product automation step.
- Public examples remain unchanged to avoid implying artifact automation.

## 12. Recommended Next Phase

Recommended next phase: workflow-declared high-assurance artifact requirement schema review.

The next review should verify that the schema slice is honest, deterministic, synchronized across Rust/schema/SDK/docs, and does not imply runtime artifact enforcement before that path exists.
