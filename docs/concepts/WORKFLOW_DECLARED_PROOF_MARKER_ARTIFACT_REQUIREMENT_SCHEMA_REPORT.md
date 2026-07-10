# Workflow-Declared Proof-Marker Artifact Requirement Schema Report

## 1. Executive Summary

The workflow-declared proof-marker artifact requirement schema/parser/SDK vocabulary slice is implemented.

Workflow specs can now express `report_artifact_requirements.approval_proof_markers` as bounded posture vocabulary. The no-op posture `not_required` passes default semantic validation. Enforceable postures `projection_required` and `marker_required` are schema-known but rejected by default semantic validation until an explicit artifact-capable runtime path can derive and enforce them.

This preserves the core invariant: declared governance must be enforced or rejected.

## 2. Scope Completed

- Added schema-facing Rust workflow definition field `report_artifact_requirements.approval_proof_markers`.
- Added checked-in v0 JSON schema vocabulary for proof-marker artifact requirements.
- Added TypeScript SDK string-union and interface support.
- Added parser tests for valid and invalid proof-marker requirement values.
- Added semantic validation tests proving `not_required` passes while enforceable proof-marker postures fail closed by default.
- Added TypeScript fixture/test coverage for valid no-op posture and rejected enforcement posture.
- Updated workflow spec docs, roadmap, and proof-marker artifact requirement planning docs.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- runtime derivation from workflow specs;
- executor artifact-path integration;
- automatic report generation;
- automatic report artifact writing;
- automatic approval proof-marker projection persistence;
- default executor proof-marker enforcement;
- CLI rendering or artifact commands;
- examples;
- public approval cards;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Schema And Parser Summary

The schema-facing workflow field is:

```yaml
report_artifact_requirements:
  approval_proof_markers: not_required
```

Supported values:

- `not_required`
- `projection_required`
- `marker_required`

The field is posture-only. It does not accept approval IDs, event IDs, projection IDs, presentation IDs, content hashes, paths, reasons, raw approval text, provider payloads, command output, or store configuration.

## 5. Validation Boundary Summary

Default semantic validation behavior:

- `not_required` passes;
- absent field behaves like `not_required`;
- `projection_required` fails with `validation.workflow.report_artifact_requirement.approval_proof_marker.runtime_not_enforced`;
- `marker_required` fails with `validation.workflow.report_artifact_requirement.approval_proof_marker.runtime_not_enforced`;
- unknown values fail parser/schema validation.

This mirrors the existing high-assurance artifact requirement posture: the schema can know future enforcement vocabulary without letting default validation imply unsupported runtime guarantees.

## 6. SDK And Contract Summary

The TypeScript SDK now includes:

- `ReportArtifactApprovalProofMarkerRequirement`
- `ReportArtifactRequirements.approval_proof_markers`

SDK contract fixtures exercise:

- no-op posture passing Rust validation;
- enforcement posture failing default Rust validation with the stable proof-marker runtime-not-enforced diagnostic.

## 7. Privacy And Redaction Summary

The implemented field is bounded enum vocabulary only.

It does not store or copy:

- approval presentation payloads;
- approval reasons;
- approval IDs;
- projection IDs;
- event IDs;
- content hashes;
- local paths;
- raw report text;
- provider payloads;
- command output;
- source/spec contents;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like metadata.

## 8. Test Coverage Summary

Added or updated coverage for:

- Rust parser accepts `approval_proof_markers: marker_required`;
- Rust parser rejects unknown proof-marker artifact requirement values;
- default project validation accepts `approval_proof_markers: not_required`;
- default project validation rejects `approval_proof_markers: marker_required`;
- checked-in JSON schema knows the bounded enum;
- TypeScript SDK exposes the bounded enum;
- TypeScript generated project with `not_required` passes Rust validation;
- TypeScript generated project with `marker_required` fails default Rust validation with the stable diagnostic.

Existing high-assurance artifact requirement tests remain intact.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `npm run build --workspace @workflow-os/sdk-typescript` - passed.
- `npm run check:contracts` - initially failed against a stale local CLI binary that did not yet include the new schema field; passed after rebuilding `workflow-os`.
- `cargo test -p workflow-core --test project_specs --test project_validation` - passed.
- `cargo build --locked -p workflow-cli --bin workflow-os` - passed.
- `npm run check:contracts` - passed after CLI rebuild.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783673552622872000-2 --phase implementation` - passed.

## 10. Remaining Known Limitations

- Runtime derivation from workflow declarations is not implemented.
- The explicit executor artifact path does not yet derive proof-marker policy from workflow declarations.
- Caller-supplied proof-marker artifact policy remains the only implemented artifact-path enforcement source.
- Automatic projection persistence remains unimplemented.
- Automatic artifact writing remains unimplemented.
- Default executor behavior remains unchanged.

## 11. Recommended Next Phase

Recommended next phase: workflow-declared proof-marker artifact requirement schema implementation review.

This phase touched public schema and SDK contract surfaces. It should be reviewed before runtime derivation or executor artifact-path integration begins.
