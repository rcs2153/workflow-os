# Workflow-Declared High-Assurance Artifact Requirement Executor Integration Blocker Fix Report

## 1. Executive Summary

This blocker fix tightens the workflow-declared high-assurance report artifact executor integration.

The artifact-capable validation posture is now scoped to the selected workflow, and rehydrated artifact-path runs now fail closed if the current workflow spec no longer matches the immutable run identity captured when the run was created.

## 2. Blockers Fixed

Fixed blocker 1: artifact-capable validation was project-wide.

The validation capability now carries the selected workflow ID. Only that workflow may pass validation with workflow-declared report artifact requirements in the explicit artifact-capable path. Unselected workflows keep the conservative default validation behavior.

Fixed blocker 2: rehydrated artifact-path policy derivation could use changed project files.

For existing runs, the artifact path now rehydrates the run first, loads the selected workflow, and verifies the loaded workflow ID, schema version, workflow version, and spec content hash match `WorkflowRunIdentity`. If they do not match, the path fails closed before report artifact policy derivation or artifact write.

## 3. Implementation Approach

The fix is intentionally narrow:

- `ProjectValidationCapability::ReportArtifactCapable` now includes `workflow_id`.
- `validate_report_artifact_requirements(...)` relaxes only the matching selected workflow.
- `execute_for_report_artifact_path(...)` passes the selected workflow ID into artifact-capable validation.
- rehydrated artifact-path runs call a new identity comparison helper before deriving policy from the currently loaded workflow.
- mismatch failures use stable non-leaking error code `executor.report_artifact.workflow_identity_mismatch`.

No artifact persistence behavior, CLI behavior, schema behavior, side-effect execution, writes, or release posture changed.

## 4. Runtime Semantics

Fresh artifact-capable runs continue to:

- validate with the explicit artifact-capable path;
- derive workflow-declared high-assurance artifact policy from the selected workflow;
- compose caller and workflow policy by strictness;
- preserve run/report semantics when artifact gate failures occur.

Existing artifact-capable runs now:

- rehydrate the existing run;
- verify the current loaded workflow still matches run identity;
- fail closed on mismatch;
- avoid weakening or changing artifact policy because YAML changed after run creation.

## 5. Privacy And Redaction

The fix compares bounded identity fields and content hashes but does not expose their raw values in errors.

It does not copy raw workflow YAML, report text, provider payloads, command output, parser payloads, file contents, environment values, credentials, authorization headers, private keys, or token-like values.

The new mismatch error is stable and non-leaking.

## 6. Test Coverage Summary

Added tests cover:

- artifact-capable validation does not relax unselected workflows;
- rehydrated artifact-path runs fail closed when workflow spec content changes after run creation;
- the rehydrated mismatch path appends no events.

Existing local executor artifact-path tests continue to cover:

- default executor rejection;
- explicit artifact-path derivation;
- caller/workflow policy strictness composition;
- artifact gate failure preservation;
- artifact write success when disclosure satisfies the workflow-declared gate.

## 7. Commands Run And Results

- `cargo fmt --all` passed.
- `cargo fmt --all --check` passed.
- `cargo test -p workflow-core --test local_executor artifact_capable_validation_does_not_relax_unselected_workflows` passed.
- `cargo test -p workflow-core --test local_executor artifact_path_rehydrated_run_fails_closed_when_workflow_spec_changes` passed.
- `cargo test -p workflow-core --test local_executor` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

## 8. Scope Explicitly Not Completed

This phase did not implement:

- automatic report generation;
- automatic artifact writing from default executor paths;
- CLI artifact behavior;
- example updates;
- new workflow schema fields;
- TypeScript SDK changes;
- runtime config;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, quorum approval, or revocation enforcement;
- approval evidence attachment;
- workflow event or audit projection for artifact derivation;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage;
- release posture changes.

## 9. Remaining Known Limitations

- The artifact-capable validation posture is still an internal code path, not a CLI validation mode.
- Rehydrated artifact policy is still derived from currently loaded workflow YAML after identity matching; the derived policy is not yet durably captured at run creation.
- Default `workflow-os validate` continues to reject workflow-declared artifact enforcement postures.

## 10. Recommended Next Phase

Recommended next phase: workflow-declared high-assurance artifact requirement executor integration blocker fix review.

This fix changes a security-sensitive enforcement boundary and should be reviewed before any further high-assurance approval control expansion, artifact automation, CLI artifact behavior, or write-capable adapter readiness work.

## 11. Dogfood Governance

This blocker fix phase was governed by the local Workflow OS dogfood runner.

- workflow phase: blocker
- workflow ID: `dg/blocker`
- run ID: `run-1783143300469128000-2`
- approval ID: `approval/run-1783143300469128000-2/fix-approved`
- approval outcome: approved by the maintainer before blocker-fix work continued
- close status: completed
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`

The dogfood runner coordinated governance only. Code edits, tests, documentation updates, and validation commands were performed by the executor.
