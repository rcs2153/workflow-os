# Explicit Projected Proof-Marker Artifact Composition Report

## 1. Executive Summary

This phase implements the explicit projected proof-marker artifact composition helper.

The new helper composes existing reviewed primitives:

- local terminal workflow execution;
- approval proof-marker projection persistence;
- terminal WorkReport generation;
- report artifact construction;
- side-effect integrity gates;
- approval-side-effect linkage gates;
- high-assurance approval disclosure gates;
- store-backed approval proof-marker artifact gates;
- local report artifact writing.

The helper is local, explicit, opt-in, and caller-supplied-store bounded. It does not change default executor behavior.

## 2. Scope Completed

- Added `LocalExecutionProjectedProofMarkerArtifactInputs`.
- Added `LocalExecutionWithProjectedProofMarkerArtifactResult`.
- Added `LocalExecutionWithProjectedProofMarkerArtifactParts`.
- Added `execute_with_report_artifact_and_projected_proof_markers(...)`.
- Reused existing terminal execution, report generation, projection persistence, artifact gate, and artifact write helpers.
- Added focused tests for successful projection plus artifact write.
- Added focused tests for projection failure before report/artifact write.
- Updated roadmap and planning documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- default executor projection persistence;
- automatic report generation;
- automatic report artifact writing;
- automatic projection persistence for all approvals;
- CLI artifact behavior;
- workflow schema changes;
- examples;
- runtime config;
- provider calls;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy changes;
- release posture changes.

## 4. Helper/API Summary

The new helper is:

```text
execute_with_report_artifact_and_projected_proof_markers(...)
```

It accepts:

- `LocalExecutor`;
- explicit `WorkReportArtifactStore`;
- explicit `SideEffectRecordStore`;
- explicit `LocalApprovalProofMarkerAuditProjectionStore`;
- explicit projection persistence policy;
- optional selected approval references;
- projection sensitivity and redaction metadata;
- explicit proof-marker artifact gate policy;
- existing `LocalExecutionWithReportArtifactRequest`.

It returns:

- existing `LocalExecutionWithReportArtifactResult`;
- optional `ApprovalProofMarkerProjectionPersistenceResult`;
- optional projection persistence error.

## 5. Composition Behavior

The helper executes or rehydrates the local workflow through the existing artifact-capable path. If the run is terminal, it persists approval proof-marker projections from the returned run before report generation and artifact gate evaluation.

If projection persistence succeeds, the helper generates a WorkReport, constructs a report artifact, evaluates existing artifact gates, and writes the artifact only if all requested gates pass.

If projection persistence fails after a run exists, the helper returns the run plus a projection error and does not generate a report or write an artifact.

## 6. Workflow Semantics Summary

The helper preserves workflow execution semantics:

- execution failure before a run exists returns the original execution error;
- non-terminal runs return report status posture and no projection persistence;
- projection failure does not change workflow pass/fail status;
- artifact failure does not change workflow pass/fail status;
- the helper does not append workflow events;
- the helper does not mutate workflow run history;
- the helper does not execute side effects;
- the helper does not call providers.

## 7. Redaction And Privacy Summary

The helper relies on existing validated model constructors and persistence helpers.

It does not copy or persist:

- approval presentation payloads;
- approval handoff text;
- approval reasons;
- command output;
- provider payloads;
- raw source or spec contents;
- environment values;
- credentials;
- tokens;
- authorization headers;
- private keys;
- secret-like values.

Debug output redacts stores, identifiers, redaction metadata, and payload-bearing fields.

## 8. Test Coverage Summary

Added focused tests proving:

- projected proof-marker artifact path persists a proof-marker projection and writes a report artifact;
- projection persistence result is exposed without leaking approval or presentation identifiers through debug output;
- projection persistence failure is returned as projection posture;
- projection failure writes no artifact and generates no report;
- workflow events remain unchanged after projection/artifact composition;
- existing hand-populated proof-marker artifact path behavior remains covered by existing tests.

## 9. Commands Run And Results

- `cargo fmt --all` applied formatting.
- `cargo fmt --all --check` passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_artifact_projected_proof_markers -- --nocapture` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

## 10. Governed Implementation Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1783688109126176000-2`
- Approval ID: `approval/run-1783688109126176000-2/implementation-approved`
- Approval outcome: granted
- Approval presentation ID: `presentation/1c8a59a69ae59988`
- Approval presentation hash: `1c8a59a69ae599887cbe3fc02c3a52a246fd36591e0e5e0c009f18a90ed5e30f`
- Event summary: 39 events; `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Out-of-kernel work: repository edits, validation commands, git/PR actions, and GitHub merge actions are performed by the maintainer/Codex execution layer outside the Workflow OS runtime.

## 11. Remaining Known Limitations

- The helper is explicit and caller-driven; default executor paths do not persist projections.
- Projection store selection remains caller-supplied.
- Workflow-declared projection-store configuration is not implemented.
- CLI artifact behavior is not implemented.
- Automatic artifact generation is not implemented.
- Approval-resume artifact/projection composition remains separate future work.

## 12. Recommended Next Phase

Recommended next phase: explicit projected proof-marker artifact composition review.

This is the right next step because the helper composes approval proof, report artifacts, and artifact gates in a security-sensitive runtime path. It should be reviewed before broadening any executor behavior, CLI behavior, or artifact automation.
