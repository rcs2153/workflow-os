# Approval-Resume Artifact Projection Composition Report

## 1. Executive Summary

This phase implements the explicit approval-resume artifact/projection
composition helper for the proof-enforced approval-presentation path.

The helper applies an approval decision through existing durable presentation
proof enforcement, then composes existing reviewed primitives:

- approval proof-marker projection persistence;
- terminal WorkReport generation;
- report artifact construction;
- side-effect integrity gates;
- approval-side-effect linkage gates;
- high-assurance approval disclosure gates;
- store-backed approval proof-marker artifact gates;
- local report artifact writing.

The helper is local, explicit, opt-in, and caller-supplied-store bounded. It
does not change default approval behavior.

## 2. Scope Completed

- Added `decide_approval_with_report_artifact_and_projected_proof_markers(...)`.
- Added `LocalApprovalResumeWithProjectedProofMarkerArtifactRequest`.
- Reused `LocalApprovalPresentationDecisionRequest` for the proof-enforced
  approval-resume path.
- Reused `LocalExecutionProjectedProofMarkerArtifactInputs`.
- Reused `LocalExecutionWithProjectedProofMarkerArtifactResult`.
- Preserved `LocalExecutor::decide_approval(...)`.
- Preserved `LocalExecutor::decide_approval_with_presentation(...)`.
- Reused existing terminal report generation, projection persistence, artifact
  gate, and artifact write helpers.
- Added focused tests for successful proof-enforced approval resume,
  projection persistence, and artifact write.
- Added focused tests for projection failure after approval resume with no
  report generation and no artifact write.
- Updated roadmap and planning documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- default approval behavior changes;
- automatic report generation;
- automatic projection persistence;
- automatic artifact writing;
- CLI approval-resume artifact behavior;
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
decide_approval_with_report_artifact_and_projected_proof_markers(...)
```

It accepts:

- `LocalExecutor`;
- explicit `WorkReportArtifactStore`;
- explicit `SideEffectRecordStore`;
- `LocalApprovalResumeWithProjectedProofMarkerArtifactRequest`;

The request contains:

- explicit `LocalApprovalProofMarkerAuditProjectionStore`;
- explicit projection persistence policy;
- selected approval references;
- projection sensitivity and redaction metadata;
- explicit proof-marker artifact gate policy;
- `LocalApprovalPresentationDecisionRequest`;
- existing `LocalExecutionReportInputs`;
- optional `LocalExecutionSideEffectDiscoveryInputs`;
- existing `LocalExecutionReportArtifactInputs`.

It returns the existing `LocalExecutionWithProjectedProofMarkerArtifactResult`,
which exposes:

- resumed `WorkflowRun`;
- optional `WorkReport`;
- optional report-generation error;
- optional `WorkReportArtifactRecord`;
- optional artifact-write error;
- optional `ApprovalProofMarkerProjectionPersistenceResult`;
- optional projection persistence error;
- artifact gate posture fields where available.

## 5. Composition Behavior

The helper first applies the approval decision through
`LocalExecutor::decide_approval_with_presentation(...)`. Approval-presentation
proof validation still runs before approval decision events are appended.

After a resumed run exists, the helper derives workflow report artifact
requirements from the immutable workflow/run identity, preserving
workflow-declared proof-marker artifact requirements. If the resumed run is
terminal, it persists approval proof-marker projections from the resumed run
before report generation and artifact gate evaluation.

If projection persistence succeeds, the helper generates a WorkReport,
constructs a report artifact, evaluates existing artifact gates, and writes the
artifact only if all requested gates pass.

If projection persistence fails after a run exists, the helper returns the run
plus a projection error and does not generate a report or write an artifact.

## 6. Workflow Semantics Summary

The helper preserves approval and workflow execution semantics:

- approval proof validation failure returns the original approval error;
- approval failure before a resumed run exists appends no invalid events;
- approval grant resumes exactly through the existing approval path;
- approval denial still follows existing fail-closed terminal behavior;
- non-terminal resumed runs return report status posture and no projection
  persistence;
- projection failure does not change workflow pass/fail status;
- artifact failure does not change workflow pass/fail status;
- report/projection/artifact post-processing does not append workflow events;
- the helper does not execute side effects;
- the helper does not call providers.

## 7. Redaction And Privacy Summary

The helper relies on existing validated model constructors and persistence
helpers.

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

Debug output redacts stores, identifiers, redaction metadata, and
payload-bearing fields.

## 8. Test Coverage Summary

Added focused tests proving:

- proof-enforced approval resume persists a proof-marker projection and writes a
  report artifact;
- projection persistence result is exposed without leaking approval or
  presentation identifiers through debug output;
- projection persistence failure is returned as projection posture;
- projection failure writes no artifact and generates no report;
- workflow events remain unchanged after projection/artifact composition;
- existing execution-time projected proof-marker artifact behavior remains
  covered by existing tests.

## 9. Commands Run And Results

- `cargo fmt --all` applied formatting.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test -p workflow-core --test local_executor decide_approval_with_report_artifact_projected_proof_markers -- --nocapture` passed.
- `cargo test -p workflow-core --test local_executor` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

## 10. Governed Implementation Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1783691439526707000-2`
- Approval ID: `approval/run-1783691439526707000-2/implementation-approved`
- Approval outcome: granted by delegated maintainer
- Approval presentation ID: `presentation/6c82033e960e788b`
- Approval presentation hash: `6c82033e960e788b0a3970bec5e76fc9d88f8132a4cf65cde00413a350e12ffc`
- Event summary: 39 events; `ApprovalGranted:1`, `ApprovalRequested:1`,
  `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`,
  `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Out-of-kernel work: repository edits, validation commands, git/PR actions,
  and GitHub merge actions are performed by the maintainer/Codex execution
  layer outside the Workflow OS runtime.

## 11. Remaining Known Limitations

- The helper is explicit and caller-driven; default approval paths do not
  generate reports or write artifacts.
- Projection store selection remains caller-supplied.
- Workflow-declared projection-store configuration is not implemented.
- CLI approval-resume artifact behavior is not implemented.
- Automatic artifact generation is not implemented.
- High-assurance approval-resume artifact/projection composition remains future
  scoped work.

## 12. Recommended Next Phase

Recommended next phase: approval-resume artifact/projection composition review.

This is the right next step because the helper composes approval proof, report
artifacts, and artifact gates in a security-sensitive runtime path. It should be
reviewed before broadening default executor behavior, CLI behavior, artifact
automation, or provider-write integration.
