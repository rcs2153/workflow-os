# Report Artifact High-Assurance Disclosure Gate Review

Review date: 2026-07-04

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds an explicit opt-in high-assurance approval disclosure gate to the governed `WorkReport` artifact write path. The gate validates bounded `WorkReportHighAssuranceApprovalDisclosure` already carried by the report artifact, fails before artifact persistence when required disclosure is missing or insufficient, and preserves workflow run state, generated reports, approval decisions, event history, and existing executor semantics.

No blocker was found. The most important follow-up is additional direct regression coverage for each individual gate failure branch before this surface becomes workflow-declared, schema-backed, or broader than the current explicit artifact-write path.

## 2. Scope Verification

The phase stayed within the approved explicit artifact-gate scope.

Implemented:

- `WorkReportArtifactHighAssuranceDisclosurePolicy`;
- `WorkReportArtifactHighAssuranceDisclosureGateResult`;
- optional bounded high-assurance approval disclosure retention on `WorkReport`;
- high-assurance disclosure validation in `WorkReport::validate`;
- high-assurance disclosure policy input on `WorkReportArtifactGovernedWriteInput`;
- high-assurance disclosure gate result exposure on `WorkReportArtifactGovernedWriteResult`;
- policy/result propagation through `LocalExecutionReportArtifactInputs` and `LocalExecutionWithReportArtifactResult`;
- pre-write high-assurance disclosure validation in the existing governed artifact write helper;
- focused executor/report tests;
- roadmap and implementation-plan updates;
- end-of-phase report.

No accidental implementation was found for:

- automatic report generation;
- automatic report artifact writing from existing executor paths;
- changes to `LocalExecutor::execute(...)`;
- changes to `LocalExecutor::execute_with_report(...)`;
- changes to approval decision APIs;
- workflow-declared high-assurance controls;
- workflow event scanning or inference;
- new workflow event types;
- audit projection for high-assurance disclosure;
- stable high-assurance validation-result records;
- `EvidenceReference` creation;
- approval evidence attachment;
- workflow schema fields;
- runtime config;
- CLI behavior;
- examples;
- RBAC, IdP, SSO, SCIM, teams, groups, or directory integration;
- quorum or multi-party approval enforcement;
- revocation enforcement;
- side-effect execution;
- provider mutation or write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. API Assessment

The API is explicit, narrow, and compatible with the current governed artifact-write boundary.

`WorkReportArtifactHighAssuranceDisclosurePolicy` defaults to disabled, preserving existing artifact behavior. Public constructors provide the intended policy postures:

- disabled;
- require disclosure;
- require validated disclosure;
- require validated disclosure with fail-closed denial behavior.

`WorkReportArtifactHighAssuranceDisclosureGateResult` exposes booleans only and does not expose report IDs, approval IDs, actor IDs, reference values, paths, high-assurance control payloads, or report text.

The executor-adjacent API remains opt-in through `LocalExecutionReportArtifactInputs`. Existing non-artifact executor APIs are unchanged.

## 4. Gate Behavior Assessment

The gate is composed in the right order inside `write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(...)`:

1. validate the report artifact;
2. verify artifact/report identity against the supplied terminal run;
3. validate cited `SideEffect` referential integrity;
4. validate approval linkage for cited `SideEffect` records;
5. validate high-assurance approval disclosure when requested;
6. write the artifact only after all requested gates pass.

This order is correct. High-assurance disclosure failure occurs before `WorkReportArtifactStore::write_work_report_artifact(...)`, so no partial artifact is written when the disclosure gate fails.

The gate does not infer posture from workflow events, approval payloads, policy strings, side-effect records, or report text. It validates only the explicit bounded disclosure carried by the `WorkReport`.

## 5. Workflow Semantics Assessment

Workflow semantics are preserved.

Verified behavior:

- workflow pass/fail status is unchanged by gate failure;
- generated in-memory reports remain available when artifact gating fails;
- approval decisions are not changed by artifact gating;
- event history is not mutated;
- no workflow events are appended for gate success or failure;
- no snapshots are mutated;
- no `SideEffect` records are created or changed;
- providers and adapters are not called;
- existing non-artifact executor paths remain unchanged.

This preserves the established posture that report/artifact failures after a run exists are separate from workflow execution success or failure.

## 6. Privacy And Redaction Assessment

The privacy boundary is sound.

The gate validates structured booleans/enums/counts only. It does not inspect, copy, persist, or leak:

- approval request payloads;
- approval decision payloads;
- actor IDs;
- high-assurance control payloads;
- required reference values;
- evidence payloads;
- policy payloads;
- provider payloads;
- command output;
- CI logs;
- Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- source snippets;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values;
- local paths;
- secret-like metadata.

Errors use stable codes and bounded messages. Debug output for the new result surfaces exposes presence booleans and gate booleans rather than sensitive values.

## 7. Serialization And Compatibility Assessment

`WorkReport` now serializes the optional `high_assurance_approval` field when present and deserializes reports without the field through `#[serde(default)]`. That preserves backward-compatible deserialization for existing report payloads.

The added field on `WorkReportDefinition` is a Rust construction-surface change. This is acceptable for the current preview-stage core model because the field is needed to retain disclosure for artifact validation and all local constructors/tests were updated. Before stronger public compatibility promises attach, maintainers should consider whether builder-style construction or `#[non_exhaustive]` input shapes would reduce future field-addition churn.

No workflow spec schema changes were introduced.

## 8. Relationship To High-Assurance Approval Controls

The implementation correctly does not treat artifact disclosure as enterprise authority.

The gate validates report-safe disclosure posture only. It does not implement RBAC/IdP, quorum approval, revocation enforcement, role-bound authority, protected-use expiration checkpoints, or workflow-declared controls. Those remain future governance layers.

This is the right relationship: high-assurance approval enforcement can produce bounded disclosure; report generation can carry it; artifact writing can require it explicitly. The gate does not claim more than that.

## 9. Relationship To SideEffect And Artifact Gates

The high-assurance disclosure gate composes cleanly with the existing `SideEffect` referential integrity and approval-linkage gates.

Positive behavior:

- the base artifact store remains a storage boundary, not a governance-policy engine;
- the governed write helper remains the explicit composition point;
- side-effect integrity and approval-linkage semantics are unchanged;
- high-assurance disclosure is optional and disabled by default;
- successful gate results are bounded to booleans.

No runtime side-effect execution or provider mutation is introduced.

## 10. Test Quality Assessment

Tests cover the critical first slice:

- existing artifact writes remain unchanged when the high-assurance disclosure policy is disabled;
- strict high-assurance disclosure policy writes an artifact when a valid disclosure is supplied;
- missing disclosure fails before artifact persistence;
- gate failure preserves the run and generated report;
- gate failure writes no artifact;
- gate failure appends no events;
- generated reports retain structured high-assurance disclosure;
- WorkReport disclosure validation rejects inconsistent validation posture;
- invalid serialized high-assurance disclosure fails closed;
- existing report, artifact, executor, SideEffect, approval, validation, adapter, and runtime tests pass.

No shallow or fake tests were found.

Non-blocking test gaps:

- add direct tests for `require_disclosure()` passing with presence-only disclosure;
- add direct tests for `require_validated()` failing when validation was not used;
- add direct tests for `require_validated()` failing when validation did not pass;
- add direct tests for `require_validated_fail_closed()` failing when denial behavior is not fail-closed;
- add direct governed-write helper tests, independent of the executor wrapper, for all high-assurance disclosure gate codes;
- add a redaction assertion for Debug output of `WorkReportArtifactGovernedWriteResult` and `LocalExecutionWithReportArtifactResult` when the high-assurance gate result is present.

These are not blockers because the success and pre-write missing-disclosure failure boundaries are covered, the gate branches are simple and non-mutating, and full workspace validation passes.

## 11. Documentation Review

Documentation is aligned with the implementation.

Verified docs state:

- explicit report artifact high-assurance approval disclosure gating is implemented;
- the gate is opt-in and artifact-scoped;
- automatic report generation is not implemented;
- automatic artifact writing from existing executor paths is not implemented;
- workflow-declared high-assurance controls are not implemented;
- workflow schema fields are not implemented;
- CLI behavior is not implemented;
- examples are not updated;
- RBAC/IdP, quorum approval, revocation enforcement, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

The end-of-phase report accurately records scope, non-scope, validation, and the recommended review.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add direct coverage for each high-assurance disclosure gate failure branch and stable error code.
- Add presence-only policy coverage for `require_disclosure()`.
- Add direct governed-write helper tests independent of executor wrapping.
- Add Debug non-leakage coverage for high-assurance disclosure gate result exposure in governed write and executor result wrappers.
- Consider builder/non-exhaustive input ergonomics for `WorkReportDefinition` before broader public compatibility expectations attach.
- Link this review from the roadmap after it is accepted.

## 14. Recommended Next Phase

Recommended next phase: **workflow-declared high-assurance artifact requirement planning**.

The explicit artifact gate is implemented and accepted for opt-in caller use. The next useful step is not write-capable adapters yet. The next gap is deciding how, and whether, authored workflows can declare that certain terminal reports must carry high-assurance approval disclosure before artifacts are persisted.

That next phase should remain planning-only and must not add schemas, runtime config, automatic artifact writes, CLI behavior, RBAC/IdP, quorum approval, side-effect execution, provider writes, or release posture changes.

## 15. Validation

Commands run:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

Dogfood governance:

- workflow: `dg/review`
- run ID: `run-1783124674564723000-2`
- approval ID: `approval/run-1783124674564723000-2/review-scope-approved`
- approval outcome: granted
