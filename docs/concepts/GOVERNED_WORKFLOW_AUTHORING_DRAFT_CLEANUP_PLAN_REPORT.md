# Governed Workflow Authoring Draft Cleanup Plan Report

## 1. Executive Summary

This planning phase defines the next authoring boundary after active workflow
promotion: how Workflow OS should treat inactive drafts after they have been
used to create active workflow files.

The plan recommends a first non-mutating draft status command before archive,
delete, persisted approval, or catalog work.

## 2. Scope Completed

- Created [Governed Workflow Authoring Draft Cleanup And Supersession Plan](../implementation-plans/governed-workflow-authoring-draft-cleanup-plan.md).
- Defined draft state vocabulary.
- Recommended first behavior: non-mutating draft status inspection.
- Compared preserve, mark-superseded, archive, and delete options.
- Defined safety, privacy, error handling, and test requirements.
- Kept cleanup implementation, archive movement, deletion, catalog state, and
  persisted approvals out of scope.

## 3. Scope Explicitly Not Completed

- No cleanup command implemented.
- No automatic cleanup after promotion.
- No draft movement, edit, archive, or deletion.
- No persisted steward approval records.
- No workflow catalog persistence.
- No runtime state creation.
- No workflow run creation.
- No command execution.
- No local check execution.
- No provider calls.
- No report artifacts.
- No schema changes.
- No examples.
- No hosted behavior.
- No external writes.
- No release posture changes.

## 4. Recommended Next Phase

Recommended next phase: non-mutating draft status implementation.

Reason: active promotion intentionally preserves drafts. A status command gives
maintainers and agents a safe way to see whether a draft is still a candidate,
already promoted, stale, or superseded before any archive/delete semantics are
introduced.

## 5. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783478667598068000-2`.
- Approval ID: `approval/run-1783478667598068000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer.
- Final status: `Completed`.
- Event summary: 39 events total, including 1 approval request, 1 approval
  grant, 8 policy decisions, 6 step schedules, 6 skill invocation request/start
  pairs, 6 skill successes, and run completion.
- Approved scope: docs-only draft cleanup/supersession planning.
- Strict non-goals: no implementation, runtime state, commands, providers,
  schemas, examples, external writes, or release posture changes.

## 6. Validation

- `npm run check:docs` - passed.
