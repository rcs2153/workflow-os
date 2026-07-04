# Workflow-Declared High-Assurance Artifact Requirement Model Report

## 1. Executive Summary

This phase implements the first internal model-only slice for workflow-declared high-assurance artifact requirements.

The new model lets Workflow OS represent a terminal report artifact high-assurance approval disclosure requirement and map it to the existing explicit `WorkReportArtifactHighAssuranceDisclosurePolicy`. It is intentionally not wired to workflow YAML, schemas, runtime config, automatic report generation, automatic artifact writing, CLI behavior, examples, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added `WorkReportArtifactRequirement`.
- Added `WorkReportArtifactRequirementDefinition`.
- Added `WorkReportArtifactHighAssuranceRequirement`.
- Added `WorkReportArtifactUnsupportedHighAssuranceRequirement`.
- Added deterministic mapping from internal high-assurance artifact requirement posture to existing artifact disclosure gate policy.
- Added fail-closed validation for unsupported future high-assurance requirements.
- Added duplicate unsupported-requirement rejection.
- Added serde support for the validated model.
- Added redaction-safe Debug behavior through bounded enum/count-only surfaces.
- Exported the model from `workflow-core`.
- Added focused tests.

## 3. Scope Explicitly Not Completed

- No workflow schema fields.
- No workflow-declared artifact requirement parsing.
- No runtime config.
- No automatic report generation.
- No automatic artifact writing from existing executor paths.
- No changes to `LocalExecutor::execute(...)`.
- No changes to `LocalExecutor::execute_with_report(...)`.
- No CLI behavior.
- No examples.
- No RBAC, IdP, SSO, SCIM, groups, teams, or directory integration.
- No quorum or multi-party approval enforcement.
- No role-bound authority enforcement.
- No revocation enforcement.
- No approval evidence attachment.
- No stable high-assurance validation result records.
- No workflow events or audit projection for artifact requirements.
- No side-effect execution.
- No provider mutation or write-capable adapters.
- No hosted/distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Model Types Added

Added:

```rust
WorkReportArtifactRequirement
WorkReportArtifactRequirementDefinition
WorkReportArtifactHighAssuranceRequirement
WorkReportArtifactUnsupportedHighAssuranceRequirement
```

Supported high-assurance requirement postures:

- `NotRequired`
- `DisclosureRequired`
- `ValidatedDisclosureRequired`
- `ValidatedFailClosedDisclosureRequired`

Unsupported future requirement vocabulary is representable only to fail closed:

- quorum approval;
- role-bound authority;
- revocation enforcement;
- external identity;
- automatic artifact writing;
- side-effect execution.

## 5. Policy Mapping Summary

The internal model maps directly to the already-reviewed artifact gate policy:

- `NotRequired` -> `WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()`
- `DisclosureRequired` -> `require_disclosure()`
- `ValidatedDisclosureRequired` -> `require_validated()`
- `ValidatedFailClosedDisclosureRequired` -> `require_validated_fail_closed()`

The model does not infer posture from workflow events, approval payloads, report text, policy strings, side-effect records, or runtime configuration.

## 6. Validation Boundary Summary

Validation is deterministic and fail-closed.

Rules:

- supported high-assurance artifact requirement posture maps to a gate policy;
- unsupported future requirement vocabulary is rejected;
- duplicate unsupported requirement vocabulary is rejected;
- deserialization validates before returning a model value;
- validation errors use stable codes and do not include raw enum values, payloads, paths, IDs, tokens, or secret-like strings.

Stable error codes:

- `work_report_artifact_requirement.high_assurance.unsupported`
- `work_report_artifact_requirement.high_assurance.duplicate_unsupported`

## 7. Redaction And Privacy Summary

The model stores posture vocabulary only. It does not store:

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

Debug and serialization surfaces are bounded to enum posture, and invalid deserialization errors are intentionally generic.

## 8. Test Coverage Summary

Added focused tests for:

- default requirement maps to disabled gate policy;
- disclosure-required posture maps to `require_disclosure()`;
- validated-disclosure posture maps to `require_validated()`;
- validated fail-closed posture maps to `require_validated_fail_closed()`;
- unsupported future high-assurance requirement vocabulary fails closed;
- duplicate unsupported vocabulary fails closed;
- serde round trip for a valid requirement;
- invalid serialized requirement fails closed without leaking unsupported vocabulary.

Existing report, artifact, high-assurance approval, and executor tests remain applicable.

## 9. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test work_report artifact_requirement -- --test-threads=1`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

Dogfood governance:

- workflow: `dg/implement`;
- run ID: `run-1783126683116846000-2`;
- approval ID: `approval/run-1783126683116846000-2/implementation-approved`;
- approval outcome: granted.

## 10. Remaining Known Limitations

- The model is not wired to workflow YAML.
- Workflow schema exposure is not implemented.
- Runtime artifact paths do not derive requirements from workflow specs.
- Automatic artifact writing remains unsupported.
- Governance profiles do not yet set artifact requirement posture.
- Enterprise steward/admin controls remain future work.

## 11. Recommended Next Phase

Recommended next phase: **workflow-declared high-assurance artifact requirement model review**.

This phase introduces a new exported model type that maps future authored requirements to artifact gate policy. It should be reviewed before schema planning, workflow validation integration, runtime artifact wiring, CLI exposure, or write-capable adapter readiness work.
