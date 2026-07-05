# GitHub PR Comment SideEffect Event Helper Report

## 1. Executive Summary

The GitHub PR comment write-candidate lane now has a pure helper for constructing a reference-only `SideEffectProposed` workflow event payload from an already persisted proposed GitHub PR comment `SideEffectRecord`.

The helper validates that the record is proposed, GitHub-write scoped, GitHub pull-request targeted, outcome-free, and matched to the expected workflow/run identity. It can also load the record through a caller-supplied `SideEffectRecordStore` by stable `SideEffectId`. It does not append workflow events, emit audit records, call GitHub, execute side effects, transition lifecycle, write report artifacts, expose CLI behavior, add schemas, update examples, or change release posture.

## 2. Scope Completed

- Added `GitHubPullRequestCommentSideEffectEventContext`.
- Added `compose_github_pr_comment_proposed_side_effect_event(...)`.
- Added `load_github_pr_comment_proposed_side_effect_event(...)`.
- Added read-only `SideEffectRecord` accessors needed for reference-only event construction.
- Exported the new helper/context APIs from `workflow-core`.
- Added focused provider-write tests for event construction, store-backed loading, missing records, unsupported lifecycle, identity mismatch, and non-leakage.
- Updated roadmap and integration documentation.

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

## 4. Helper API Summary

The loaded-record helper:

```text
compose_github_pr_comment_proposed_side_effect_event(
    record: &SideEffectRecord,
    context: &GitHubPullRequestCommentSideEffectEventContext,
) -> Result<SideEffectWorkflowEvent, WorkflowOsError>
```

The store-backed helper:

```text
load_github_pr_comment_proposed_side_effect_event(
    store: &impl SideEffectRecordStore,
    side_effect_id: &SideEffectId,
    context: &GitHubPullRequestCommentSideEffectEventContext,
) -> Result<SideEffectWorkflowEvent, WorkflowOsError>
```

Both helpers return only a validated in-memory event payload. They do not append it to a run.

## 5. Validation Boundary Summary

The helper fails closed unless:

- the record lifecycle is `Proposed`;
- the capability is `GitHubWrite`;
- the target is an adapter resource shaped like a GitHub pull request target;
- no outcome reference is present;
- workflow ID matches expected context;
- workflow version matches expected context;
- schema version matches expected context;
- spec hash matches expected context;
- run ID matches expected context.

Errors use stable non-leaking codes and do not include raw record IDs, target references, summaries, run IDs, spec hashes, or secret-like values.

## 6. Event Construction Summary

The event payload cites:

- `SideEffectId`;
- optional step ID;
- optional skill ID/version;
- optional correlation ID;
- stable references already present on the record;
- evidence reference count;
- zero outcome reference count;
- sensitivity;
- redaction metadata.

It does not copy the full `SideEffectRecord`, raw GitHub payloads, PR bodies, diffs, generated comment bodies, command output, CI logs, file contents, spec contents, credentials, authorization headers, or environment variable values.

## 7. Dogfood Governance Summary

- Dogfood workflow: `dg/implement`.
- Governed run ID: `run-1783212294884663000-2`.
- Approval ID: `approval/run-1783212294884663000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer.
- Terminal status: completed.

The kernel governed scope and approval. Codex performed repository edits and validation outside the kernel as executor.

## 8. Test Coverage Summary

Focused tests cover:

- event construction from a persisted proposed record;
- store-backed event loading by stable ID;
- missing store record failure without leaking the missing ID;
- non-proposed record rejection;
- workflow/run identity mismatch rejection without leaking IDs or target details;
- Debug and serialization non-leakage for raw provider/spec/comment markers;
- existing provider-write request, fixture, response, composition, and persistence behavior.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test provider_write` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Remaining Known Limitations

- The helper constructs an event payload only; it does not append it.
- Generic audit projection is not invoked by the helper.
- Dedicated audit sink storage is not implemented.
- Report artifact citation from the persisted proposed record remains future work.
- Live sandbox GitHub writes remain blocked.

## 11. Recommended Next Phase

Recommended next phase: GitHub PR comment SideEffect event helper review.

This is write-readiness-adjacent and creates event payloads from durable proposed records, so it should receive a focused maintainer review before executor append integration, audit/report integration, or live sandbox write planning.
