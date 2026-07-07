# Governed Workflow Authoring Steward Review CLI Preview Plan Report

## 1. Executive Summary

This planning phase defines the next bounded user-facing surface after the accepted in-memory steward-review helper: a preview-only CLI command for steward review.

The plan recommends `workflow-os author workflow steward-review --draft ...` as a non-mutating command that derives fresh preflight context, calls the existing helper, and prints a bounded approval card and decision result.

This phase does not implement the command.

## 2. Scope Completed

- Created [Governed Workflow Authoring Steward Review CLI Preview Plan](../implementation-plans/governed-workflow-authoring-steward-review-cli-preview-plan.md).
- Updated the roadmap to link the accepted helper review to the CLI preview plan.
- Updated the governed workflow authoring umbrella plan.
- Updated the promotion/steward-review plan.

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

## 4. Plan Summary

The plan recommends a preview-only CLI command:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

The command should derive current draft/preflight context in-process, call `review_workflow_draft_for_promotion`, and print bounded text or JSON output.

## 5. Boundary Summary

The planned CLI preview must:

- require an explicit draft path;
- require explicit reviewer, decision, and reason;
- reuse current preflight behavior;
- fail closed on blocked preflight;
- produce bounded review-card output;
- report non-mutation flags;
- avoid raw draft content and raw payloads;
- avoid runtime state, events, artifacts, commands, providers, and file writes.

## 6. Test Plan Summary

Future implementation tests should cover:

- passing draft preview;
- blocked preflight;
- non-authorizing decisions;
- unknown decision;
- invalid reviewer;
- missing or secret-like reason;
- unsafe draft path;
- duplicate active workflow id;
- text and JSON output boundaries;
- no state, artifacts, active workflow files, command execution, or provider calls.

## 7. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783434751000850000-2`.
- Approval ID: `approval/run-1783434751000850000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Approved scope: create planning document for a non-mutating CLI preview that presents the steward-review card and decision result without promotion.
- Strict non-goals: no implementation, active promotion, registration, file movement, persisted approvals, runtime state, commands, providers, artifacts, schemas, examples, writes, hosted behavior, or release posture changes.
- Phase-close: completed with 39 events, 1 approval, 0 retries, and 0 escalations.
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.

## 8. Validation Commands

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783434751000850000-2 --phase planning`: passed.

## 9. Recommended Next Phase

Recommended next phase: governed workflow authoring steward-review CLI preview implementation.

The implementation should stay preview-only and non-mutating. Active promotion, file movement, workflow registration, persisted approvals, runtime state, commands, providers, artifacts, schemas, examples, writes, hosted behavior, and release posture changes must remain out of scope.
