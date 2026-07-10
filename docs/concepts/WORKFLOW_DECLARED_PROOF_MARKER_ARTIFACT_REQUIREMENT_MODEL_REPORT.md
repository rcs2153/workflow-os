# Workflow-Declared Proof-Marker Artifact Requirement Model Report

## 1. Executive Summary

The first internal model slice for workflow-declared proof-marker artifact requirements is implemented.

Workflow OS now has posture-only internal vocabulary for future terminal report artifact approval proof-marker requirements and deterministic mapping to the existing `WorkReportArtifactApprovalProofMarkerGatePolicy`.

This remains model and mapping only. No workflow schema fields, TypeScript SDK changes, CLI behavior, executor default enforcement, automatic report generation, automatic artifact writing, automatic projection persistence, provider writes, examples, hosted behavior, reasoning lineage, or release posture changes were added.

## 2. Scope Completed

- Added internal approval proof-marker artifact requirement posture vocabulary.
- Added future unsupported approval proof-marker requirement vocabulary.
- Added validated constructor handling for the new proof-marker requirement fields.
- Added deterministic mapping from proof-marker requirement posture to explicit artifact proof-marker gate policy.
- Added redaction-safe `Debug`/serde behavior through the existing validated artifact requirement model.
- Added focused tests for mapping, fail-closed unsupported vocabulary, duplicate unsupported vocabulary, serde round trip, and invalid serde rejection.
- Hardened approval proof-marker projection store tests to use process-scoped temporary roots after full-suite validation exposed stale temp-state collision risk.
- Updated the planning document status.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- workflow schema fields;
- TypeScript SDK changes;
- project validation diagnostics for authored proof-marker requirements;
- runtime derivation from workflow specs;
- executor default proof-marker enforcement;
- automatic report generation;
- automatic report artifact writing;
- automatic approval proof-marker projection persistence;
- CLI rendering, artifact export, or artifact write commands;
- examples;
- public approval cards;
- provider writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Model Types Added

Added:

- `WorkReportArtifactApprovalProofMarkerRequirement`
- `WorkReportArtifactUnsupportedApprovalProofMarkerRequirement`

Extended:

- `WorkReportArtifactRequirement`
- `WorkReportArtifactRequirementDefinition`

The new supported postures are:

- `not_required`
- `projection_required`
- `marker_required`

## 5. Policy Mapping Summary

The mapping is deterministic:

- `not_required` maps to no proof-marker artifact gate policy.
- `projection_required` maps to `WorkReportArtifactApprovalProofMarkerGatePolicy::allow_marker_free()`.
- `marker_required` maps to `WorkReportArtifactApprovalProofMarkerGatePolicy::require_present_markers()`.

The model does not infer stores, persist projections, generate reports, write artifacts, append events, or change runtime semantics.

## 6. Validation Boundary Summary

Unsupported future proof-marker requirement vocabulary fails closed through the existing `WorkReportArtifactRequirement::new(...)` constructor path.

Stable error codes:

- `work_report_artifact_requirement.approval_proof_marker.unsupported`
- `work_report_artifact_requirement.approval_proof_marker.duplicate_unsupported`

Serde deserialization uses the validated constructor and fails closed with a bounded generic error when unsupported vocabulary is supplied.

## 7. Redaction And Privacy Summary

The model is posture-only. It does not store approval IDs, event IDs, projection IDs, presentation IDs, content hashes, local paths, approval reasons, report text, provider payloads, command output, CI logs, source/spec contents, environment variable values, credentials, authorization headers, private keys, token-like values, or secret-like metadata.

Tests assert that unsupported future vocabulary does not leak through errors.

## 8. Test Coverage Summary

Focused tests cover:

- default `not_required` proof-marker posture maps to no gate policy;
- `projection_required` maps to marker-free-allowed projection policy;
- `marker_required` maps to present-marker-required policy;
- unsupported future proof-marker vocabulary fails closed without leaking values;
- duplicate unsupported future proof-marker vocabulary fails closed without leaking values;
- serde round trip preserves validated high-assurance and proof-marker posture;
- invalid serialized artifact requirement fails closed without leaking unsupported values.

Existing high-assurance artifact requirement tests remain intact.

Full-suite validation also exercised existing approval proof-marker projection store tests after their temporary test root was made process-scoped, preventing stale local test records from causing duplicate-record failures on rerun.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test work_report artifact_requirement -- --nocapture` - passed.
- `cargo test -p workflow-core --test work_report approval_proof_marker_audit_projection_store -- --nocapture` - passed after test-root hardening.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783669636074933000-2 --phase implementation` - passed; governed phase closed with proof-enforced approval presentation.

## 10. Remaining Known Limitations

- The requirement is not wired to workflow YAML.
- No JSON schema or TypeScript SDK surface exists yet.
- No runtime derivation from workflow specs exists yet.
- No executor artifact path derives this requirement from a workflow declaration.
- Projection stores remain caller-supplied.
- Automatic projection persistence remains unimplemented.
- Automatic artifact writing remains unimplemented.
- Default executor behavior remains unchanged.

## 11. Recommended Next Phase

Recommended next phase: **workflow-declared proof-marker artifact requirement model review**.

This model is write-adjacent because it will eventually select artifact proof-marker gates. It should be reviewed before schema planning, runtime derivation, executor integration, automatic projection persistence, or write-capable adapter readiness.
