# Workflow-Declared High-Assurance Artifact Requirement Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The phase stayed inside the approved internal model and policy-mapping boundary. It adds a small posture-only model for future workflow-declared high-assurance artifact requirements, maps supported postures to the already-reviewed explicit report artifact high-assurance disclosure gate, rejects unsupported future governance vocabulary fail-closed, and avoids workflow schema, runtime, CLI, example, side-effect, write, hosted, reasoning-lineage, or release-posture changes.

## 2. Scope Verification

The phase stayed within approved model-only scope.

Implemented:

- internal `WorkReportArtifactRequirement` model;
- construction definition for validated artifact requirements;
- supported high-assurance disclosure requirement posture vocabulary;
- unsupported future high-assurance requirement vocabulary that is rejected fail-closed;
- deterministic mapping to `WorkReportArtifactHighAssuranceDisclosurePolicy`;
- serde support through the validated model;
- redaction-safe `Debug` for the stored model;
- focused model and mapping tests;
- documentation and phase report.

No accidental implementation found for:

- workflow schema fields;
- workflow-declared artifact requirement parsing;
- runtime config;
- automatic report generation;
- automatic artifact writing;
- changes to existing executor methods;
- CLI behavior;
- examples;
- RBAC, IdP, SSO, SCIM, groups, teams, or directory integration;
- quorum or multi-party approval enforcement;
- role-bound authority enforcement;
- revocation enforcement;
- approval evidence attachment;
- stable high-assurance validation result records;
- workflow events or audit projection for artifact requirements;
- side-effect execution;
- provider mutation or write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Model Assessment

The model is domain-neutral, posture-oriented, and appropriately minimal for the approved slice.

`WorkReportArtifactRequirement` stores only the supported high-assurance approval disclosure posture. `WorkReportArtifactRequirementDefinition` is the construction boundary. `WorkReportArtifactHighAssuranceRequirement` expresses supported postures, and `WorkReportArtifactUnsupportedHighAssuranceRequirement` makes future unsupported governance vocabulary explicit without accepting it.

This is the right direction for avoiding decorative governance: the model can represent unsupported future intent only as a validation failure. It does not create YAML-facing false promises or allow workflow authors to declare capabilities that the runtime cannot enforce.

## 4. Policy Mapping Assessment

The mapping is deterministic and aligns with the existing artifact gate:

- `NotRequired` maps to disabled high-assurance disclosure policy;
- `DisclosureRequired` maps to `require_disclosure()`;
- `ValidatedDisclosureRequired` maps to `require_validated()`;
- `ValidatedFailClosedDisclosureRequired` maps to `require_validated_fail_closed()`.

The model does not infer high-assurance posture from workflow events, approval payloads, policy strings, side-effect records, report text, or runtime config. That preserves the explicit gate source-of-truth boundary.

## 5. Validation Assessment

Validation is small and fail-closed.

Verified behavior:

- default requirement is valid and maps to disabled policy;
- supported postures are valid and map to the expected gate policies;
- unsupported future requirement vocabulary is rejected;
- duplicate unsupported future vocabulary is rejected;
- deserialization routes through `WorkReportArtifactRequirement::new`;
- validation errors use stable codes;
- serialized invalid requirement data fails closed with a generic deserialization error.

Stable error codes:

- `work_report_artifact_requirement.high_assurance.unsupported`;
- `work_report_artifact_requirement.high_assurance.duplicate_unsupported`.

The validation errors do not include raw enum values, payloads, paths, actor IDs, report text, approval payloads, tokens, or secret-like values.

## 6. Serde And Compatibility Assessment

Valid requirements serialize and deserialize through the validated model shape. Invalid serialized requirements fail closed before returning a `WorkReportArtifactRequirement`.

The field names are sensible for a future schema-facing shape, but this phase does not expose workflow schema fields. Exporting the new Rust model from `workflow-core` is acceptable for the internal model slice, with the documented limitation that public schema compatibility and workflow parsing remain deferred.

## 7. Privacy And Redaction Assessment

The model stores posture vocabulary only.

It does not store:

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

`WorkReportArtifactRequirement` has bounded `Debug` output. Invalid deserialization errors are generic and non-leaking.

## 8. Relationship To Artifact Gates And High-Assurance Controls

The model composes correctly with the existing high-assurance approval and report artifact lanes.

It does not replace high-assurance approval controls. It models whether a terminal report artifact should require high-assurance approval disclosure before persistence. The intended chain remains:

```text
approval control validation -> report-safe disclosure -> artifact requirement gate
```

It also does not write artifacts. It only maps a requirement to the explicit gate policy that the governed artifact write path can already enforce.

## 9. Test Quality Assessment

The focused tests cover the approved model boundary:

- default requirement maps to disabled gate policy;
- supported postures map to the expected gate policies;
- unsupported future governance fails closed;
- duplicate unsupported future governance fails closed;
- serde round trip works for a valid requirement;
- invalid serialized requirement fails closed without leaking unsupported values.

Existing report, artifact, high-assurance approval, and workspace tests were rerun during the implementation phase and remain applicable.

Non-blocking gaps:

- no direct test composes `WorkReportArtifactRequirement::high_assurance_disclosure_policy()` into a governed artifact write call; this is acceptable for the model-only phase, but should be added when runtime wiring is planned;
- no schema-facing tests exist, because schema exposure is intentionally deferred;
- unsupported vocabulary tests check selected variants rather than every variant. The current coverage is sufficient, but a table-driven all-variant rejection test would make the future compatibility posture clearer.

## 10. Documentation Review

Documentation is honest about current state.

Verified docs say:

- workflow-declared high-assurance artifact requirements are planned;
- the first internal model/policy-mapping slice is implemented;
- workflow schema fields are not implemented;
- runtime derivation from workflow specs is not implemented;
- runtime config is not implemented;
- automatic report generation is not implemented;
- automatic artifact writing is not implemented;
- CLI behavior is not implemented;
- examples are not updated;
- RBAC/IdP, quorum, revocation, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, and release posture changes are not implemented.

## 11. Blockers

No blockers found.

## 12. Non-Blocking Follow-Ups

- Add a future runtime-wiring test that maps a validated artifact requirement to a governed artifact write gate and proves persistence succeeds or fails according to the policy.
- Add table-driven coverage proving every `WorkReportArtifactUnsupportedHighAssuranceRequirement` variant fails closed without leaking.
- Plan workflow schema exposure separately, including diagnostics for unsupported declared artifact requirements.
- Keep governance profile and enterprise steward policy mapping separate from this posture model until those concepts have accepted runtime boundaries.

## 13. Recommended Next Phase

Recommended next phase: workflow schema exposure planning.

The model slice is accepted, but Workflow OS should not jump directly into YAML fields. The next phase should decide the exact schema boundary, diagnostics, and validation behavior for workflow-authored artifact requirements while preserving the invariant that declared governance is either enforced or rejected. Runtime wiring from validated declarations into explicit artifact write paths should follow after schema exposure planning is reviewed.

## 14. Validation

Dogfood governance:

- workflow: `dg/review`;
- run ID: `run-1783127396710028000-2`;
- approval ID: `approval/run-1783127396710028000-2/review-scope-approved`;
- approval outcome: granted;
- final run status: completed.

Implementation validation reviewed from the phase report:

- `cargo fmt --all`: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test -p workflow-core --test work_report artifact_requirement -- --test-threads=1`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed.

Review validation after adding this review document:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.
