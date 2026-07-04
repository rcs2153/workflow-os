# Report Artifact High-Assurance Disclosure Gate Report

## 1. Executive Summary

This phase implements an explicit opt-in high-assurance approval disclosure gate for governed `WorkReport` artifact writes.

The gate validates the bounded `WorkReportHighAssuranceApprovalDisclosure` carried by the report artifact before persistence. When the caller requires disclosure and the report lacks it, or the disclosure does not satisfy the requested posture, artifact writing fails without changing the workflow run, generated report, approval decision, event history, or existing executor semantics.

This is a narrow artifact-write gate only. It does not make report generation automatic, does not make artifact writing automatic from existing executor paths, does not add CLI behavior, schemas, examples, RBAC/IdP, quorum approval, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added `WorkReportArtifactHighAssuranceDisclosurePolicy`.
- Added `WorkReportArtifactHighAssuranceDisclosureGateResult`.
- Added optional structured high-assurance approval disclosure retention to `WorkReport`.
- Preserved backward-compatible report deserialization when disclosure is absent.
- Added high-assurance disclosure policy to `WorkReportArtifactGovernedWriteInput`.
- Added high-assurance disclosure gate posture to `WorkReportArtifactGovernedWriteResult`.
- Threaded the policy/result through `LocalExecutionReportArtifactInputs` and `LocalExecutionWithReportArtifactResult`.
- Validated required disclosure before artifact persistence.
- Preserved existing artifact behavior when the policy is disabled.
- Added focused executor and report tests.
- Updated roadmap and implementation-plan status language.

## 3. Scope Explicitly Not Completed

- No automatic report generation.
- No automatic report artifact writing from existing executor paths.
- No change to `LocalExecutor::execute(...)`.
- No change to `LocalExecutor::execute_with_report(...)`.
- No change to approval decision APIs.
- No workflow-declared high-assurance controls.
- No workflow event scanning or inference.
- No new workflow event types.
- No audit projection for high-assurance disclosure.
- No stable high-assurance validation result records.
- No `EvidenceReference` creation.
- No approval evidence attachment.
- No workflow schema fields.
- No runtime config.
- No CLI behavior.
- No examples.
- No RBAC, IdP, SSO, SCIM, teams, groups, or directory integration.
- No quorum or multi-party approval enforcement.
- No revocation enforcement.
- No side-effect execution.
- No provider mutation or write-capable adapters.
- No hosted/distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. API Summary

New model types:

```rust
WorkReportArtifactHighAssuranceDisclosurePolicy
WorkReportArtifactHighAssuranceDisclosureGateResult
```

`WorkReportArtifactGovernedWriteInput` now accepts:

```rust
high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy
```

`WorkReportArtifactGovernedWriteResult` now exposes:

```rust
high_assurance_disclosure()
```

`LocalExecutionReportArtifactInputs` now accepts the same explicit high-assurance disclosure policy, and `LocalExecutionWithReportArtifactResult` exposes the bounded gate result when it ran.

## 5. Gate Behavior Summary

The gate is disabled by default. Disabled policy preserves existing artifact writes.

When enabled, the gate validates the structured high-assurance disclosure carried by the `WorkReport`:

- disclosure must be present when required;
- validation-used posture must be true when required;
- validation-passed posture must be true when required;
- fail-closed denial behavior posture must be true when required.

Failure occurs before `WorkReportArtifactStore::write_work_report_artifact(...)` is called.

## 6. Workflow Semantics Summary

The implementation preserves workflow semantics:

- workflow pass/fail status is unchanged by gate failure;
- approval decisions are unchanged by gate failure;
- generated in-memory reports remain available when artifact gating fails;
- no workflow events are appended for gate success or failure;
- no snapshots are mutated;
- no SideEffect records are created or changed;
- no providers or adapters are called;
- existing non-artifact executor paths remain unchanged.

## 7. Privacy And Redaction Summary

The gate validates only bounded structured report disclosure.

It does not inspect, copy, or leak:

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

Errors use stable codes and bounded messages.

## 8. Test Coverage Summary

Added/updated focused tests for:

- disabled high-assurance disclosure gate remains inert;
- enabled gate writes an artifact when disclosure is present and satisfies policy;
- enabled gate fails before persistence when disclosure is missing;
- gate failure preserves the run and generated report;
- gate failure appends no events and writes no artifact;
- generated `WorkReport` retains structured high-assurance disclosure;
- existing high-assurance approval disclosure report tests still pass;
- existing artifact SideEffect integrity behavior remains covered.

## 9. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test local_executor high_assurance -- --test-threads=1`: passed.
- `cargo test -p workflow-core --test work_report high_assurance -- --test-threads=1`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Dogfood governance:

- workflow: `dg/implement`;
- run ID: `run-1783123359504219000-2`;
- approval ID: `approval/run-1783123359504219000-2/implementation-approved`;
- approval outcome: granted.

## 10. Remaining Known Limitations

- Gate policy is explicit caller input only.
- Workflow-declared high-assurance artifact requirements are not implemented.
- High-assurance validation result IDs are not durable records.
- The gate validates report disclosure posture, not enterprise identity authority.
- RBAC/IdP, quorum approval, revocation enforcement, and role-bound authority remain future work.
- Automatic report generation and automatic artifact writes remain unsupported.

## 11. Recommended Next Phase

Recommended next phase: report artifact high-assurance disclosure gate review.

This phase touches artifact persistence semantics and WorkReport serialization shape. It should be reviewed before adding workflow-declared controls, CLI artifact inspection, automatic artifact writes, or write-capable adapter readiness work.
