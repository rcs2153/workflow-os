# GitHub PR Comment SideEffect Event Append Helper Report

## 1. Executive Summary

Workflow OS now has an explicit helper that loads a persisted proposed GitHub pull request comment `SideEffectRecord`, composes the existing reference-only `SideEffectProposed` workflow event payload, validates the requested local executor target, and returns `LocalExecutionSideEffectEventInput`.

The helper does not append workflow events by itself. It does not call GitHub, mutate providers, execute side effects, emit audit records, write report artifacts, expose CLI behavior, add schemas, update examples, or change release posture.

## 2. Scope Completed

- Added `GitHubPullRequestCommentSideEffectAppendInput`.
- Added `load_github_pr_comment_proposed_side_effect_event_input(...)`.
- Reused `load_github_pr_comment_proposed_side_effect_event(...)`.
- Reused existing `LocalExecutionSideEffectEventInput`.
- Added stable non-leaking error mapping for the executor-input helper boundary.
- Added focused tests for success, missing records, step mismatch, correlation mismatch, and redaction-safe Debug behavior.
- Updated roadmap and integration documentation.

## 3. Scope Explicitly Not Completed

- No GitHub provider calls.
- No GitHub PR comment mutation.
- No live sandbox write.
- No runtime side-effect execution.
- No `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` behavior.
- No automatic event append after persistence.
- No new executor method.
- No report artifact writes.
- No CLI behavior.
- No workflow schema fields.
- No examples.
- No hosted behavior.
- No reasoning lineage.
- No autonomy expansion.
- No release posture change.

## 4. Helper API Summary

The helper input:

```text
GitHubPullRequestCommentSideEffectAppendInput {
    side_effect_id,
    context,
    step_id,
    skill_id,
    skill_version,
    correlation_id,
}
```

The helper:

```text
load_github_pr_comment_proposed_side_effect_event_input(
    store: &impl SideEffectRecordStore,
    input: GitHubPullRequestCommentSideEffectAppendInput,
) -> Result<LocalExecutionSideEffectEventInput, WorkflowOsError>
```

It returns an input that callers may pass through the existing `LocalExecutionRequest.side_effect_events` path.

## 5. Validation Boundary Summary

The helper:

- loads by explicit `SideEffectId`;
- verifies the persisted proposed record through the existing GitHub proposed-event helper;
- maps store and record validation failures into stable executor-input error codes;
- verifies event step/skill identity when present;
- verifies correlation ID when an expected correlation ID is supplied;
- returns only a validated `LocalExecutionSideEffectEventInput`.

It does not bypass the existing local executor side-effect event validation. The executor remains the append boundary.

## 6. Error Handling Summary

New stable error codes include:

- `github_pr_comment_side_effect_event_input.record_missing`;
- `github_pr_comment_side_effect_event_input.store_read_failed`;
- `github_pr_comment_side_effect_event_input.identity_mismatch`;
- `github_pr_comment_side_effect_event_input.record_invalid`;
- `github_pr_comment_side_effect_event_input.target_mismatch`;
- `github_pr_comment_side_effect_event_input.correlation_mismatch`.

Errors do not include raw side-effect IDs, run IDs, workflow IDs, target references, repository names, pull request numbers, comment summaries, comment bodies, spec hashes, redaction metadata values, provider references, or secret-like values.

## 7. Privacy And Redaction Summary

The helper remains reference-only. It does not copy:

- raw provider payloads;
- generated comment bodies;
- PR bodies or diffs;
- command output;
- CI logs;
- file contents;
- spec contents;
- environment values;
- credentials;
- authorization headers;
- token-like values.

`GitHubPullRequestCommentSideEffectAppendInput` has redaction-safe `Debug` behavior, and the returned executor input uses the existing redaction-safe `LocalExecutionSideEffectEventInput` `Debug` implementation.

## 8. Dogfood Governance Summary

- Dogfood workflow: `dg/implement`.
- Governed run ID: `run-1783214351115022000-2`.
- Approval ID: `approval/run-1783214351115022000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer.
- Terminal status: completed.

The kernel governed scope and approval. Codex performed repository edits and validation outside the kernel as executor.

## 9. Test Coverage Summary

Focused tests cover:

- persisted record loads into `LocalExecutionSideEffectEventInput`;
- returned input preserves explicit executor target step/skill identity;
- returned event remains `SideEffectProposed`;
- missing store record maps to a non-leaking executor-input error;
- step mismatch fails without leaking step IDs;
- correlation mismatch fails without leaking correlation IDs;
- Debug output does not leak GitHub PR comment IDs or target values;
- existing provider-write tests still pass.

## 10. Commands Run And Results

Validation commands for this implementation phase:

- `cargo fmt --all`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test -p workflow-core --test provider_write`;
- `cargo test -p workflow-core --test local_executor`;
- `npm run check:docs`;
- `git diff --check`.

## 11. Remaining Known Limitations

- The helper does not append events by itself.
- Automatic append after proposed-record persistence remains deferred.
- Report artifact citation from persisted proposed records remains deferred.
- Attempted/completed/failed lifecycle behavior remains deferred.
- Live sandbox GitHub writes remain blocked.

## 12. Recommended Next Phase

Recommended next phase: focused maintainer review of the GitHub PR comment SideEffect event append helper.

Do not proceed to automatic append, report artifact citation, or live sandbox write planning before that review is accepted.
