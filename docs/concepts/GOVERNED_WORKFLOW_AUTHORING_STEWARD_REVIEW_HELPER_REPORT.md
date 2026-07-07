# Governed Workflow Authoring Steward Review Helper Report

## 1. Executive Summary

This phase implements the first pure steward-review helper for inactive workflow draft promotion.

The helper accepts explicit review inputs, validates that a preflight-passing draft is unchanged, produces a bounded review card, and returns whether the steward decision authorizes a future separately implemented promotion step.

This phase does not implement a CLI command, active workflow promotion, workflow registration, file movement, persisted approvals, runtime state, command execution, provider calls, report artifacts, schemas, examples, hosted behavior, writes, or release posture changes.

## 2. Scope Completed

- Added `workflow_authoring` core module.
- Added `WorkflowDraftPromotionPreflightStatus`.
- Added `WorkflowDraftStewardReviewDecision`.
- Added `WorkflowDraftStewardReviewAuthorization`.
- Added `WorkflowDraftStewardReviewInput`.
- Added `WorkflowDraftStewardReviewCard`.
- Added `WorkflowDraftStewardReviewBoundary`.
- Added `WorkflowDraftStewardReviewResult`.
- Added `review_workflow_draft_for_promotion`.
- Exported the helper and model types from `workflow-core`.
- Added focused `workflow-core` tests.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- steward-review CLI command;
- active workflow promotion;
- workflow registration;
- file movement from `workflows/drafts/` to `workflows/`;
- persisted steward approval records;
- runtime state creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- workflow-declared steward configuration;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, admin UI, paging, or notifications;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Helper API Summary

Primary helper:

```rust
review_workflow_draft_for_promotion(input: WorkflowDraftStewardReviewInput)
    -> Result<WorkflowDraftStewardReviewResult, WorkflowOsError>
```

The helper requires explicit inputs for:

- draft path;
- candidate workflow id;
- preflight draft content hash;
- current draft content hash;
- preflight status;
- preflight blocker and warning codes;
- owner, escalation, policy, evidence/report, and side-effect summaries;
- active workflow conflict posture;
- reviewer actor;
- steward decision;
- bounded approval reason.

## 5. Validation Boundary Summary

The helper fails closed when:

- draft path is unsafe or outside `workflows/drafts/`;
- candidate workflow id remains in the `draft/` namespace;
- current content hash differs from the preflight hash;
- preflight status is blocked;
- preflight blockers are present;
- active workflow conflict is present;
- summaries or approval reason are empty, too long, or secret-like;
- preflight blocker/warning codes are duplicated, too many, too long, or malformed.

## 6. Authorization Semantics

`ApprovedForPromotion` returns `AuthorizedForPromotion` only for the exact unchanged draft and only for a separately implemented future promotion step.

`Denied`, `NeedsChanges`, and `Deferred` return `NotAuthorized`.

Authorization does not move files, register workflows, run commands, call providers, persist approval state, create runtime state, or approve future changes to the draft.

## 7. Non-Mutation Boundary

The result includes a boundary record where all mutation flags are false:

- `files_written`;
- `workflow_registered`;
- `workflow_promoted`;
- `approval_persisted`;
- `runtime_state_created`;
- `commands_executed`;
- `providers_called`.

## 8. Redaction And Privacy Summary

The helper validates bounded text and rejects secret-like values. Errors use stable codes and avoid echoing raw input values.

Debug output redacts approval reason and bounded review summary text. The helper does not copy raw draft YAML, source contents, package scripts, CI logs, command output, provider payloads, parser payloads, environment values, credentials, authorization headers, private keys, token-like strings, or private absolute paths.

## 9. Test Coverage Summary

Focused tests cover:

- approved preflight-passing draft authorizes future promotion without mutation;
- denied, needs-changes, and deferred decisions do not authorize promotion;
- preflight blockers fail closed without leaking blocker payloads;
- stale preflight hash fails closed;
- active workflow conflict fails closed;
- unsafe or secret-like inputs are rejected without leakage;
- debug output redacts bounded review text.

## 10. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783408516977950000-2`.
- Approval ID: `approval/run-1783408516977950000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Approved scope: add explicit-input steward-review helper/model with bounded approval-card result and focused tests, docs, and report.
- Strict non-goals: no CLI promotion command, active promotion, workflow registration, file movement, persisted approvals, runtime state, commands, providers, artifacts, schemas, examples, writes, hosted behavior, or release posture changes.
- Phase-close: completed with 39 events, 1 approval, 0 retries, and 0 escalations.
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Out-of-kernel work disclosed: repository edits, shell validation commands, git/PR actions, and report updates remain agent actions outside the kernel.

## 11. Validation Commands

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test workflow_authoring`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783408516977950000-2 --phase implementation`: passed.

## 12. Remaining Known Limitations

- No CLI surface exists for steward review yet.
- No active promotion command exists.
- No file movement or workflow registration exists.
- No persisted steward approval record exists.
- No workflow-declared steward configuration exists.
- No enterprise steward/RBAC/IdP integration exists.
- Approval-presentation proof remains a separate open P0 hardening gap.

## 13. Recommended Next Phase

Recommended next phase: governed workflow authoring steward-review helper review.

The review should verify that the helper is pure, explicit-input, redaction-safe, non-mutating, and compatible with future active promotion before any CLI or file-movement implementation is planned.
