# GitHub PR Comment SideEffect Event/Audit Projection Planning Report

## 1. Executive Summary

The GitHub PR comment write-candidate lane now has a planning document for projecting persisted proposed `SideEffectRecord` values into future workflow event and audit semantics.

The plan keeps the next boundary narrow: a future helper should construct a reference-only `SideEffectProposed` workflow event from an already persisted proposed GitHub PR comment record, then rely on existing bounded generic audit projection. This planning phase does not implement event append behavior, audit sink emission, runtime side-effect execution, provider mutation, report artifact writing, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.

## 2. Scope Completed

- Added [GitHub PR Comment Proposed SideEffect Event/Audit Projection Plan](../implementation-plans/github-pr-comment-side-effect-event-audit-projection-plan.md).
- Linked the plan from `ROADMAP.md`.
- Updated write-adapter readiness status.
- Updated the proposed-record persistence plan to point to projection planning while keeping implementation deferred.
- Updated the GitHub future integration posture.

## 3. Scope Explicitly Not Completed

- No GitHub provider calls.
- No GitHub PR comment mutation.
- No live sandbox write.
- No runtime side-effect execution.
- No `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` behavior.
- No workflow event append implementation.
- No audit sink implementation.
- No report artifact write.
- No automatic executor integration.
- No CLI behavior.
- No workflow schema fields.
- No examples.
- No hosted behavior.
- No reasoning lineage.
- No autonomy expansion or release posture change.

## 4. Planning Boundary Summary

The plan defines a record-before-event posture:

1. validate and preflight the GitHub PR comment write candidate;
2. compose and persist the proposed `SideEffectRecord`;
3. construct a reference-only `SideEffectProposed` event from that persisted record in a future phase;
4. project accepted workflow events through existing generic audit projection.

The persisted record remains durable write intent. A workflow event is needed before the run history can claim that the proposed side effect was accepted into runtime history.

## 5. Dogfood Governance Summary

- Dogfood workflow: `dg/d`.
- Governed run ID: `run-1783211740959387000-2`.
- Approval ID: `approval/run-1783211740959387000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer.
- Terminal status: completed.
- Event summary: 39 events, including one approval request, one approval grant, eight policy decisions, six scheduled steps, six skill invocation requests, six skill starts, six skill successes, and run completion.

The kernel governed scope and approval. Repository edits and validation commands were performed outside the kernel by Codex as executor.

## 6. Validation Summary

- `npm run check:docs` - passed using the repo-local bundled Node path.
- `git diff --check` - passed.

## 7. Remaining Known Limitations

- The plan is documentation only.
- The future event construction helper is not implemented.
- Event append behavior is not implemented for this provider-specific path.
- Dedicated audit sink storage is not implemented.
- Report artifact citation from the persisted proposed record remains future work.
- Live sandbox GitHub writes remain blocked.

## 8. Recommended Next Phase

Recommended next phase: GitHub PR comment proposed SideEffect event construction helper implementation.

That phase should remain pure/model-helper only: construct a validated `SideEffectProposed` event from an already persisted proposed record, add focused tests, and still avoid provider mutation, runtime side-effect execution, automatic executor behavior, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, and release posture changes.
