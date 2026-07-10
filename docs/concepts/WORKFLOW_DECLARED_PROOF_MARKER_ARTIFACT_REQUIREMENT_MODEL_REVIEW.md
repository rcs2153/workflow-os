# Workflow-Declared Proof-Marker Artifact Requirement Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation stays within the approved model-and-policy-mapping boundary. It adds internal proof-marker artifact requirement vocabulary, validates unsupported future vocabulary fail-closed, maps supported postures to the existing artifact proof-marker gate policy, and preserves the current runtime/schema/CLI boundaries.

The next phase should be workflow-declared proof-marker artifact requirement schema planning, not direct schema implementation.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented scope:

- internal `WorkReportArtifactApprovalProofMarkerRequirement` posture vocabulary;
- internal `WorkReportArtifactUnsupportedApprovalProofMarkerRequirement` future vocabulary;
- `WorkReportArtifactRequirement` and definition extensions;
- deterministic mapping to `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- validated constructor and serde-deserialization path;
- focused tests and documentation.

No accidental implementation was found for:

- workflow schema fields;
- TypeScript SDK changes;
- project validation diagnostics for authored proof-marker requirements;
- runtime derivation from workflow specs;
- executor default proof-marker enforcement;
- automatic report generation;
- automatic report artifact writing;
- automatic proof-marker projection persistence;
- CLI rendering, artifact export, or artifact write commands;
- examples;
- public approval cards;
- provider writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Model Assessment

The model is appropriately minimal and domain-specific to terminal report artifact requirements.

The supported posture enum is small:

- `NotRequired`;
- `ProjectionRequired`;
- `MarkerRequired`.

The unsupported future vocabulary is explicit and useful because it lets internal callers reject future governance claims rather than silently accepting decorative metadata. The selected unsupported variants correctly keep automatic projection persistence, public approval cards, quorum proof, external identity, hosted audit, and side-effect execution outside this phase.

The `WorkReportArtifactRequirement` type remains posture-only. It does not store approval IDs, event IDs, projection IDs, presentation IDs, hashes, file paths, approval reasons, raw report text, or provider payloads.

## 4. Policy Mapping Assessment

The mapping is deterministic and matches the plan:

- `NotRequired` maps to no proof-marker artifact gate policy.
- `ProjectionRequired` maps to `WorkReportArtifactApprovalProofMarkerGatePolicy::allow_marker_free()`.
- `MarkerRequired` maps to `WorkReportArtifactApprovalProofMarkerGatePolicy::require_present_markers()`.

This is the right first boundary. It connects authored-intent vocabulary to the already-reviewed explicit gate policy without adding runtime behavior.

One important boundary remains: `derive_workflow_report_artifact_gate_policy(...)` still derives only the existing high-assurance artifact policy. That is not a blocker because workflow-declared proof-marker schema/runtime derivation is explicitly out of scope. It should become part of the next schema/runtime derivation design rather than being patched into this model review.

## 5. Validation Assessment

Validation is deterministic and fail-closed.

Verified behavior:

- defaults are valid and preserve existing artifact requirement behavior;
- supported proof-marker postures construct successfully;
- unsupported future proof-marker vocabulary is rejected;
- duplicate unsupported future proof-marker vocabulary is rejected;
- stable error codes are used:
  - `work_report_artifact_requirement.approval_proof_marker.unsupported`;
  - `work_report_artifact_requirement.approval_proof_marker.duplicate_unsupported`;
- serde deserialization flows through the validated constructor and fails closed with a bounded generic serde error.

Validation errors do not include raw unsupported enum values in tested debug/error paths.

## 6. Privacy And Redaction Assessment

The privacy posture is sound for the implemented scope.

The model stores only enum posture values. It does not store:

- raw provider payloads;
- command output;
- CI logs;
- source or spec contents;
- approval presentation text;
- approval reasons;
- approval IDs or projection IDs;
- local paths;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

`Debug` output for `WorkReportArtifactRequirement` exposes only posture values. That is acceptable because the values are bounded internal vocabulary, not caller-supplied strings.

## 7. Serde And Compatibility Assessment

Serde support is suitable for internal model use.

Valid requirements serialize and deserialize with stable snake-case posture values. Invalid serialized requirements that include unsupported future vocabulary fail closed through the constructor-backed `Deserialize` implementation.

The shape is appropriate for future schema planning, but it is not itself a schema commitment. No workflow schema, checked-in JSON schema, SDK type, or example contract was changed.

## 8. Relationship To Workflow-Declared Artifact Requirements

The implementation prepares the internal model required for future workflow-declared artifact requirements without exposing authored YAML yet.

This is the right sequence because the next schema phase must decide how to avoid false governance. A future workflow declaration must either be enforceable in an explicit artifact-capable path or rejected with a stable validation diagnostic.

The implementation does not create report artifacts, persist projections, derive policy from workflow specs, or change executor behavior. It remains compatible with future workflow-declared requirements.

## 9. Test Quality Assessment

The focused tests are good for the approved scope.

Covered:

- default `NotRequired` maps to no proof-marker gate policy;
- `ProjectionRequired` maps to marker-free projection policy;
- `MarkerRequired` maps to present-marker-required policy;
- unsupported future proof-marker vocabulary fails closed without leaking values;
- duplicate unsupported future proof-marker vocabulary fails closed without leaking values;
- serde round trip preserves proof-marker posture;
- invalid serialized artifact requirement fails closed without leaking unsupported values;
- existing high-assurance artifact requirement tests remain intact.

The temporary-root hardening for existing projection store tests was appropriate because it addressed full-suite repeatability without widening product scope.

Missing but non-blocking for this phase:

- no workflow parser/schema tests, because schema exposure is deferred;
- no workflow derivation tests for proof-marker posture, because runtime derivation is deferred;
- no executor artifact-path tests for workflow-declared proof-marker policy, because executor integration is deferred.

## 10. Documentation Review

Documentation is honest about the current boundary.

Verified documentation says the internal model and mapping are implemented while the following remain unimplemented:

- workflow schema fields;
- TypeScript SDK changes;
- project validation diagnostics for authored proof-marker requirements;
- runtime derivation from workflow specs;
- executor default proof-marker enforcement;
- automatic report generation;
- automatic report artifact writing;
- automatic proof-marker projection persistence;
- CLI rendering, artifact export, or artifact write commands;
- examples;
- public approval cards;
- provider writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

The phase report and roadmap accurately position this as model and policy mapping only.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Schema planning should define whether authored `approval_proof_markers` declarations are rejected until enforceable artifact-capable execution is selected.
- Future runtime derivation should extend the workflow artifact gate derivation shape to include proof-marker artifact policy once schema support exists.
- Future tests should cover schema fixtures, parser behavior, SDK contract compatibility, validation diagnostics, and explicit artifact-path enforcement after those phases are approved.

## 13. Recommended Next Phase

Recommended next phase: workflow-declared proof-marker artifact requirement schema planning.

Reason: the model is now available, but exposing the requirement in workflow YAML is a public contract decision. The next phase should plan parser/schema/SDK/validation behavior and define how Workflow OS avoids false governance before any schema implementation begins.
