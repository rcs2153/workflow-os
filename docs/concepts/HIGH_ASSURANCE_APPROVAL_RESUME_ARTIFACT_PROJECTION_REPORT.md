# High-Assurance Approval-Resume Artifact Projection Report

## 1. Executive Summary

The high-assurance approval-resume artifact projection slice is implemented as
an explicit local helper. The helper composes durable approval-presentation
proof, high-assurance approval validation, proof-marker projection persistence,
terminal WorkReport generation, high-assurance disclosure gates, and local
report artifact writing.

This is an opt-in runtime-composition path. It does not change default approval
behavior, automatic report generation, CLI behavior, workflow schemas, examples,
provider writes, side-effect execution, hosted behavior, reasoning lineage, or
release posture.

## 2. Scope Completed

- Added `LocalHighAssuranceApprovalResumeWithProjectedProofMarkerArtifactRequest`.
- Added `decide_approval_with_high_assurance_report_artifact_and_projected_proof_markers(...)`.
- Required durable approval-presentation proof before approval mutation.
- Required high-assurance approval validation before approval mutation.
- Attached the resulting approval proof marker to the approval decision for projection.
- Carried report-safe high-assurance approval disclosure into WorkReport generation.
- Reused existing proof-marker projection persistence and report artifact finish paths.
- Preserved separate projection, report-generation, artifact-write, and workflow-run result posture.
- Added focused success and fail-closed tests.

## 3. Scope Explicitly Not Completed

- No default approval behavior changes.
- No automatic high-assurance approval enforcement.
- No automatic report generation.
- No automatic projection persistence.
- No automatic artifact writing.
- No CLI behavior.
- No workflow schema changes.
- No examples.
- No runtime configuration.
- No provider calls or provider writes.
- No side-effect execution.
- No approval evidence attachment.
- No RBAC, IdP, SSO, SCIM, teams, groups, quorum, or external directory integration.
- No role-bound approval authority or revocation enforcement.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No recursive agents, agent swarms, or Level 3/4 autonomy changes.
- No release posture changes.

## 4. Helper/API Summary

The new helper accepts:

- an explicit `LocalExecutor`;
- an explicit `WorkReportArtifactStore`;
- an explicit `SideEffectRecordStore`;
- explicit projection inputs containing a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore`;
- an explicit `LocalHighAssuranceApprovalDecisionRequest`;
- durable approval presentation proof and optional maximum presentation age;
- explicit report inputs;
- optional side-effect discovery inputs;
- explicit artifact inputs.

It returns the existing `LocalExecutionWithProjectedProofMarkerArtifactResult`.

## 5. Composition Behavior

The helper rehydrates and validates the waiting approval run, validates durable
approval-presentation proof, validates high-assurance controls, creates bounded
high-assurance disclosure, applies one approval decision carrying a proof marker,
persists approval proof-marker projections, generates a terminal WorkReport with
the disclosure attached, evaluates existing artifact gates, and writes the local
artifact only when all requested gates pass.

High-assurance validation failure occurs before approval decision events are
appended. Projection failure after approval resume is returned as projection
posture and writes no artifact.

## 6. Workflow Semantics Summary

The helper preserves existing workflow semantics:

- approval validation failure appends no decision events;
- approval grant resumes exactly once;
- projection/report/artifact failures after a run exists do not rewrite run status;
- no extra workflow events are appended for projection, report, or artifact work;
- no hidden stores are discovered;
- no provider calls or side-effect execution are performed.

## 7. Redaction/Privacy Summary

The helper uses existing report, high-assurance disclosure, projection, and
artifact constructors. It does not store raw high-assurance control payloads,
approval presentation payloads, approval reasons, command output, provider
payloads, raw source/spec contents, environment values, credentials,
authorization headers, private keys, token-like values, or secret-like values.

Debug output redacts approval, presentation, store, report, artifact, and
high-assurance disclosure payloads.

## 8. Test Coverage Summary

Added focused tests for:

- high-assurance approval grant writes a proof-marker-projected report artifact with high-assurance disclosure;
- missing required high-assurance references fail before approval mutation;
- failed high-assurance validation writes no projection records or artifacts;
- debug output does not leak approval, presentation, or evidence reference identifiers.

Existing high-assurance, approval presentation, WorkReport, artifact, projection,
and local executor tests remain in place.

## 9. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test local_executor high_assurance_approval_resume -- --nocapture` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783695577846757000-2 --phase implementation` - passed.

## 10. Governed Implementation Record

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783695577846757000-2`.
- Approval ID: `approval/run-1783695577846757000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after proof-enforced handoff.
- Approval presentation ID: `presentation/1b9a5f39666d4e30`.
- Approval presentation hash: `1b9a5f39666d4e3050d1b60808bcefde68f6982e43cc272f3e6a04d99b401d8c`.
- Event summary: 39 events, including one approval request, one approval grant, six scheduled skill steps, six skill invocation successes, and completion.
- Approval presentation enforcement: proof-enforced with approval event proof marker present.

## 11. Remaining Known Limitations

- Denial-result artifact support is not first-class in this slice.
- Workflow-declared high-assurance approval controls remain unimplemented.
- RBAC, IdP, quorum, revocation enforcement, role-bound authority, and hosted approval administration remain unimplemented.
- CLI exposure remains unimplemented.
- Automatic runtime generation and automatic artifact writing remain unimplemented.
- Provider writes and side-effect execution remain unsupported.

## 12. Recommended Next Phase

Recommended next phase: high-assurance approval-resume artifact/projection
composition review.
