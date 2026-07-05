# GitHub PR Comment SideEffect Event Append Plan Report

## 1. Executive Summary

The GitHub PR comment write-candidate lane now has a planning document for the next runtime-composition step: explicitly accepting a persisted proposed GitHub PR comment `SideEffectRecord` into workflow history as a `SideEffectProposed` event through the existing local executor side-effect event input path.

The plan keeps the boundary narrow. It recommends a helper that loads a persisted proposed record, composes the already implemented proposed event payload, validates target step/skill/correlation identity, and returns `LocalExecutionSideEffectEventInput`. That first helper is now implemented in [GitHub PR Comment SideEffect Event Append Helper Report](GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_HELPER_REPORT.md).

## 2. Scope Completed

- Inspected existing local executor SideEffect event append behavior.
- Inspected generic SideEffect audit projection behavior.
- Inspected the GitHub PR comment proposed event helper.
- Added `docs/implementation-plans/github-pr-comment-side-effect-event-append-plan.md`.
- Defined source-of-truth, ordering, identity, idempotency, audit, privacy, failure, and test boundaries.

## 3. Scope Explicitly Not Completed

- No runtime code changes.
- No GitHub provider calls.
- No GitHub PR comment mutation.
- No live sandbox write.
- No runtime side-effect execution.
- No attempted/completed/failed lifecycle support.
- No automatic event append.
- No report artifact writes.
- No CLI behavior.
- No schemas.
- No examples.
- No hosted behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Current Baseline Summary

Existing Workflow OS primitives already include:

- proposed GitHub PR comment `SideEffectRecord` persistence;
- pure proposed-event construction from a persisted record;
- explicit `LocalExecutionSideEffectEventInput`;
- local executor append support for proposed/denied/skipped SideEffect events;
- generic bounded audit projection from accepted SideEffect workflow events.

The missing bridge is GitHub-specific persisted record loading into the explicit executor input shape.

## 5. Recommended Helper Summary

The plan recommends a helper that:

1. accepts a caller-supplied `SideEffectRecordStore`;
2. accepts an explicit `SideEffectId`;
3. accepts expected workflow/run identity;
4. accepts target step and skill identity;
5. loads and validates the persisted proposed record;
6. composes the proposed event using the existing helper;
7. returns `LocalExecutionSideEffectEventInput`.

The helper should not append events itself. The existing executor remains the append boundary.

## 6. Privacy And Redaction Summary

The plan preserves a reference-only posture. It forbids copying raw provider payloads, generated comment bodies, PR bodies, diffs, logs, command output, file contents, spec contents, environment values, credentials, authorization headers, or secret-like values.

Errors and Debug output must remain stable and non-leaking.

## 7. Dogfood Governance Summary

- Dogfood workflow: `dg/d`.
- Governed run ID: `run-1783213967558142000-2`.
- Approval ID: `approval/run-1783213967558142000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer.
- Terminal status: completed.

The kernel governed scope and approval. Codex performed repository edits and validation outside the kernel as executor.

## 8. Validation Summary

Validation commands for this planning phase:

- `npm run check:docs`;
- `git diff --check`.

## 9. Remaining Known Limitations

- The plan does not implement the helper.
- Automatic append after proposed-record persistence remains deferred.
- Report artifact citation from persisted proposed records remains deferred.
- Live sandbox GitHub writes remain blocked.

## 10. Recommended Next Phase

Recommended next phase: focused maintainer review of the explicit persisted-record to `LocalExecutionSideEffectEventInput` helper for GitHub PR comment proposed events.

Do not implement provider mutation, automatic append, attempted/completed/failed lifecycle transitions, report artifacts, CLI behavior, schemas, examples, hosted behavior, or release posture changes.
