# GitHub PR Comment Proposed SideEffect Event Append Plan

Status: Planned, first helper implemented, helper review accepted with non-blocking follow-ups, executor-path proof implemented, executor-proof review accepted with non-blocking follow-ups, and follow-on report artifact citation planning documented in [GitHub PR Comment Report Artifact Citation Plan](github-pr-comment-report-artifact-citation-plan.md). This plan follows the accepted GitHub PR comment proposed SideEffect event helper review. It defines the smallest future executor integration that should accept a persisted proposed GitHub pull request comment `SideEffectRecord`, compose a validated `SideEffectProposed` workflow event from it, and append that event through the existing explicit local executor SideEffect event input path. The first implementation helper is documented in [GitHub PR Comment SideEffect Event Append Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_HELPER_REPORT.md), reviewed in [GitHub PR Comment SideEffect Event Append Helper Review](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_HELPER_REVIEW.md), proven through the executor append path in [GitHub PR Comment SideEffect Event Append Executor Proof Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_EXECUTOR_PROOF_REPORT.md), and reviewed in [GitHub PR Comment SideEffect Event Append Executor Proof Review](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_EXECUTOR_PROOF_REVIEW.md).

This plan does not authorize provider calls, GitHub mutation, live sandbox writes, attempted/completed/failed lifecycle behavior, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.

## 1. Executive Summary

Workflow OS can now:

- validate a GitHub pull request comment write request without provider calls;
- run write preflight;
- validate fixture/dry-run posture;
- compose a proposed `SideEffectRecord`;
- persist that proposed record through an explicit `SideEffectRecordStore`;
- compose a reference-only `SideEffectProposed` workflow event payload from the persisted proposed record.

The original runtime-composition gap was explicit event acceptance. A persisted proposed record is durable write intent, but it is not workflow event history until the executor accepts and appends a workflow event.

The accepted implementation adds a small explicit helper input that loads a persisted proposed GitHub PR comment record, composes the proposed event with the existing helper, and feeds that event into the existing `LocalExecutionSideEffectEventInput` path. The executor-path proof verifies this helper output through `LocalExecutionRequest.side_effect_events`. The path remains opt-in and local. It does not call GitHub, execute a side effect, create a comment, or append attempted/completed/failed lifecycle events.

## 2. Goals

- Accept persisted GitHub PR comment proposed records into local workflow history through an explicit opt-in executor path.
- Reuse `load_github_pr_comment_proposed_side_effect_event(...)`.
- Reuse `LocalExecutionSideEffectEventInput` and the existing executor append path.
- Preserve record-before-event ordering.
- Preserve generic audit projection from accepted workflow events.
- Require explicit target step, skill, and workflow/run identity.
- Preserve deterministic validation and replay behavior.
- Avoid raw provider payloads, generated comment bodies, command output, logs, diffs, file contents, or spec contents.
- Keep provider mutation blocked.
- Prepare for later report artifact citation and live sandbox planning without implementing either.

## 3. Non-Goals

Do not implement:

- this plan in the planning phase;
- GitHub provider calls;
- GitHub PR comment creation;
- live sandbox writes;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` behavior;
- automatic event append from default executor paths;
- automatic discovery from all persisted records;
- report artifact writes;
- automatic WorkReport mutation;
- CLI commands or rendering;
- workflow schema fields;
- examples;
- hosted or distributed behavior;
- production credential management;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Already implemented:

- `GitHubPullRequestCommentWriteRequest`;
- GitHub PR comment write preflight composition;
- fixture-only GitHub PR comment validation;
- proposed `SideEffectRecord` composition;
- explicit proposed-record persistence through `SideEffectRecordStore`;
- `GitHubPullRequestCommentSideEffectEventContext`;
- `compose_github_pr_comment_proposed_side_effect_event(...)`;
- `load_github_pr_comment_proposed_side_effect_event(...)`;
- `LocalExecutionSideEffectEventInput`;
- explicit local executor append support for `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped`;
- generic audit projection for accepted SideEffect workflow events.

Not yet implemented:

- GitHub-specific persisted-record to executor-input bridge;
- automatic append after proposed-record persistence;
- report artifact citation from the persisted proposed GitHub PR comment record;
- live GitHub writes.

## 5. Recommended First Implementation Target

Add a small explicit input/helper that produces `LocalExecutionSideEffectEventInput` from:

- a caller-supplied `SideEffectRecordStore`;
- an explicit `SideEffectId`;
- expected workflow/run identity;
- target `StepId`;
- target `SkillId`;
- target `SkillVersion`;
- expected correlation ID if available.

Possible API shape:

```text
GitHubPullRequestCommentSideEffectAppendInput {
    side_effect_id: SideEffectId,
    context: GitHubPullRequestCommentSideEffectEventContext,
    step_id: StepId,
    skill_id: SkillId,
    skill_version: SkillVersion,
    correlation_id: Option<CorrelationId>,
}

load_github_pr_comment_proposed_side_effect_event_input(
    store: &impl SideEffectRecordStore,
    input: GitHubPullRequestCommentSideEffectAppendInput,
) -> Result<LocalExecutionSideEffectEventInput, WorkflowOsError>
```

The helper should:

1. load the record by explicit ID;
2. compose a validated proposed event with `load_github_pr_comment_proposed_side_effect_event(...)`;
3. verify event step/skill/correlation identity matches the explicit target input;
4. return `LocalExecutionSideEffectEventInput`;
5. let `LocalExecutor::execute(...)` use its existing append path when the caller supplies that input.

Do not add a new executor method unless the helper cannot remain ergonomic. Prefer a helper over an executor method for the first slice.

## 6. Source-Of-Truth Boundary

Preserve the current boundaries:

| Surface | Boundary |
| --- | --- |
| `SideEffectRecordStore` | Durable proposed write-intent record source. |
| GitHub PR comment event helper | Reference-only event payload construction from a proposed persisted record. |
| `LocalExecutionSideEffectEventInput` | Explicit caller-supplied executor append input for one targeted local skill invocation. |
| `WorkflowRunEvent` | Accepted runtime history after executor append. |
| `AuditEvent` | Bounded projection from accepted workflow events. |
| `WorkReport` | Future citation surface; not the source of truth for write intent. |

A persisted record without an event means durable proposed intent exists but was not accepted into workflow history. An event append without a loaded and validated persisted record should not be introduced for this GitHub PR comment path.

## 7. Event Ordering Policy

The executor append path should preserve existing ordering:

1. run created/validated/started;
2. step scheduled;
3. policy decision recorded;
4. proposed SideEffect event appended;
5. skill invocation requested;
6. skill invocation started/succeeded/failed;
7. terminal run event.

The proposed event must appear before the targeted skill invocation request. It must not imply provider execution.

## 8. Identity And Idempotency Policy

The helper should validate:

- store-loaded record matches expected workflow ID;
- workflow version matches;
- schema version matches;
- spec hash matches;
- run ID matches;
- lifecycle is `Proposed`;
- capability is `GitHubWrite`;
- target is GitHub pull-request shaped;
- no outcome reference is present;
- event step ID, if present, matches target step;
- event skill ID/version, if present, matches target skill identity;
- event correlation ID, if present, matches the expected correlation ID.

Idempotency should continue to use the executor's existing side-effect event idempotency key logic for explicit side-effect event inputs. Do not add provider idempotency semantics in this phase.

## 9. Audit Projection Policy

The implementation should not add a dedicated GitHub PR comment audit sink.

When the proposed event is accepted into workflow history, existing generic audit projection should produce a bounded `AuditEvent` with:

- event identity;
- workflow/run identity;
- event type `SideEffectProposed`;
- side-effect input reference;
- reference-only redaction metadata;
- lifecycle vocabulary only.

The audit projection must not copy the persisted record, comment body, target details, provider payloads, or generated comment text.

## 10. Failure Behavior

Failure to load or validate the persisted record should fail the execution before the side-effect event append and before targeted skill invocation.

Recommended stable code families:

- `github_pr_comment_side_effect_event_input.record_missing`;
- `github_pr_comment_side_effect_event_input.store_read_failed`;
- `github_pr_comment_side_effect_event_input.record_invalid`;
- `github_pr_comment_side_effect_event_input.identity_mismatch`;
- `github_pr_comment_side_effect_event_input.target_mismatch`;
- `github_pr_comment_side_effect_event_input.correlation_mismatch`;
- `github_pr_comment_side_effect_event_input.append_unsupported`.

Errors must not leak:

- SideEffect IDs;
- run IDs;
- workflow IDs;
- spec hashes;
- repository names;
- pull request numbers;
- target references;
- summaries;
- comment bodies;
- provider references;
- redaction metadata values;
- tokens or secret-like values.

## 11. Privacy And Redaction

The path must remain reference-first and conservative.

It must not store, append, log, or project:

- raw GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw pull request bodies;
- raw diffs;
- raw generated comment bodies;
- raw CI logs;
- raw command output;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded prompts;
- secret-like values.

Debug output for any new input/helper type must redact side-effect IDs, run IDs, workflow identity, spec hash, target references, and redaction metadata values.

## 12. Test Plan

Future implementation tests should cover:

- persisted proposed GitHub PR comment record becomes a `LocalExecutionSideEffectEventInput`;
- executor appends `SideEffectProposed` before targeted skill invocation when supplied with that input;
- generic audit projection observes the accepted proposed event;
- missing record fails closed before skill invocation;
- store read failure maps to stable non-leaking error;
- non-proposed lifecycle is rejected;
- unsupported capability is rejected;
- unsupported target is rejected;
- outcome reference is rejected;
- workflow/run/schema/spec identity mismatch fails closed;
- step/skill mismatch fails closed;
- correlation mismatch fails closed if expected correlation is supplied;
- no provider mutation occurs;
- no attempted/completed/failed event is appended;
- no report artifact is written;
- no CLI output is introduced;
- Debug/serialization/error paths do not leak raw payloads or secret-like values;
- existing provider-write, executor side-effect append, audit projection, side-effect store, and report tests continue to pass.

## 13. Proposed Implementation Sequence

Completed:

1. Add a narrow `GitHubPullRequestCommentSideEffectAppendInput` helper input.
2. Add helper that loads the persisted proposed record and returns `LocalExecutionSideEffectEventInput`.
3. Add focused provider-write tests for success and failure paths.
4. Review the helper before broader executor integration.
5. Use the existing `LocalExecutionRequest.side_effect_events` field in a focused proof phase to prove end-to-end append with the GitHub-specific helper.
6. Verify generic audit projection through the existing local audit sink.
7. Review the executor-path proof before adding report artifact citation, automatic discovery, or live sandbox write planning.

Next:

1. Implement a validation-only report artifact citation helper for persisted proposed GitHub PR comment records and accepted proposed workflow events, following [GitHub PR Comment Report Artifact Citation Plan](github-pr-comment-report-artifact-citation-plan.md).

## 14. Deferred Work

- Automatic append after persistence.
- Automatic discovery from all persisted GitHub PR comment proposed records.
- Report artifact citation from persisted proposed records.
- Dedicated side-effect audit sink storage.
- `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed`.
- Live sandbox GitHub write.
- Provider mutation.
- CLI write command.
- Workflow schema fields.
- Examples.
- Hosted runtime behavior.
- Reasoning lineage.
- Release posture changes.

## 15. Final Recommendation

Proceed next to report artifact citation planning for persisted proposed GitHub PR comment records and accepted proposed workflow events.

Keep subsequent work local, opt-in, and reference-only. Do not implement provider calls, automatic event append, attempted/completed/failed lifecycle transitions, report artifact writes, CLI behavior, schemas, examples, hosted behavior, or release posture changes during citation planning.
