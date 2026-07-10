# Workflow-Declared Proof-Marker Artifact Requirement Schema Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The schema/parser/SDK vocabulary implementation stayed within the approved narrow scope. Workflow YAML can now declare `report_artifact_requirements.approval_proof_markers`, the no-op posture validates, and enforceable postures fail closed by default until a runtime artifact path can derive and enforce them.

## 2. Scope Verification

The phase stayed within schema/parser/SDK vocabulary scope.

Implemented:

- Rust workflow definition field for `report_artifact_requirements.approval_proof_markers`;
- checked-in v0 JSON schema enum values;
- TypeScript SDK type and fixture coverage;
- parser tests for valid and invalid proof-marker posture values;
- default semantic validation accepting `not_required`;
- default semantic validation rejecting `projection_required` and `marker_required` with a stable runtime-not-enforced diagnostic;
- docs and phase report updates.

No accidental implementation was found for:

- runtime derivation from workflow declarations;
- executor artifact-path integration;
- automatic report generation;
- automatic report artifact writing;
- automatic approval proof-marker projection persistence;
- default executor proof-marker enforcement;
- CLI rendering or artifact commands;
- examples;
- provider writes;
- hosted or distributed behavior;
- reasoning lineage;
- release posture changes.

## 3. Contract Surface Assessment

The public contract surfaces moved together appropriately.

Rust defines `ReportArtifactRequirements.approval_proof_markers` using the existing `WorkReportArtifactApprovalProofMarkerRequirement` enum. The checked-in workflow schema exposes the same bounded values: `not_required`, `projection_required`, and `marker_required`. The TypeScript SDK mirrors that vocabulary with `ReportArtifactApprovalProofMarkerRequirement` and an optional `approval_proof_markers` field.

This is appropriately minimal and avoids creating a second TypeScript-side model.

## 4. Validation Assessment

Default validation preserves the false-governance boundary:

- absent field remains compatible with existing workflows;
- `not_required` passes;
- `projection_required` and `marker_required` fail default semantic validation;
- the stable diagnostic is `validation.workflow.report_artifact_requirement.approval_proof_marker.runtime_not_enforced`;
- the diagnostic points at `$.report_artifact_requirements.approval_proof_markers`;
- unknown values fail parsing/schema validation rather than becoming ignored strings.

This is the right behavior because the default runtime path still does not enforce workflow-declared proof-marker artifact requirements.

## 5. Runtime Boundary Assessment

No runtime behavior was added.

The implementation does not derive artifact proof-marker gate policy from workflow declarations, does not write report artifacts, does not persist approval proof-marker projections, and does not change executor behavior. That is correct for this phase. Runtime derivation should remain a separate phase so the gate composition rules can be reviewed independently.

## 6. Privacy And Redaction Assessment

The new workflow field is posture-only bounded vocabulary.

It does not store or copy:

- approval presentation payloads;
- approval reasons;
- approval IDs;
- event IDs;
- projection IDs;
- presentation IDs;
- content hashes;
- local paths;
- raw report text;
- provider payloads;
- command output;
- source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like metadata.

The validation diagnostic does not echo raw invalid payloads or secret-like values.

## 7. Test Quality Assessment

Coverage is adequate for the phase.

Reviewed coverage includes:

- parser acceptance for `approval_proof_markers: marker_required`;
- parser rejection for unknown proof-marker posture values;
- default validation acceptance for `approval_proof_markers: not_required`;
- default validation rejection for `approval_proof_markers: marker_required`;
- assertion of the stable proof-marker runtime-not-enforced diagnostic;
- TypeScript fixture generation for no-op and enforcement postures;
- SDK contract test proving no-op posture passes Rust validation;
- SDK contract test proving enforcement posture fails Rust validation.

Existing WorkReport artifact requirement tests, high-assurance artifact requirement tests, and workspace tests continued to pass.

## 8. Documentation Review

Docs accurately state:

- schema/parser/SDK vocabulary is implemented;
- `not_required` is accepted;
- `projection_required` and `marker_required` are known but rejected by default validation;
- runtime derivation is not implemented;
- executor artifact-path integration is not implemented;
- automatic artifact writing is not implemented;
- automatic projection persistence is not implemented;
- CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

Non-blocking documentation note: the planning document still preserves original planning-language sections that say the plan itself does not implement the field. That is acceptable because the status line now points to the implementation report, but future docs cleanup could make the transition from historical plan to implemented status easier to scan.

## 9. Blockers

No blockers.

## 10. Non-Blocking Follow-Ups

- Add a small docs cleanup pass later to make historical planning docs less visually ambiguous after implementation.
- Consider a future artifact-capable validation posture only when runtime derivation exists.
- Keep the next implementation phase focused on pure derivation helper logic before executor integration.

## 11. Validation

Implementation phase validation reviewed:

- `cargo fmt --all` - passed.
- `npm run build --workspace @workflow-os/sdk-typescript` - passed.
- `npm run check:contracts` - initially failed against a stale local CLI binary, then passed after rebuilding `workflow-os`.
- `cargo test -p workflow-core --test project_specs --test project_validation` - passed.
- `cargo build --locked -p workflow-cli --bin workflow-os` - passed.
- `npm run check:contracts` - passed after CLI rebuild.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783673552622872000-2 --phase implementation` - passed.

Review phase validation:

- `npm run check:docs` - passed.

## 12. Recommended Next Phase

Recommended next phase: workflow-declared proof-marker artifact requirement runtime derivation helper planning.

Why: the schema and SDK can now express the posture safely, but runtime derivation remains intentionally absent. The next useful step is to plan the pure helper that derives effective proof-marker artifact gate policy from a selected workflow without writing artifacts, persisting projections, changing default executor behavior, or weakening caller-supplied policy.
