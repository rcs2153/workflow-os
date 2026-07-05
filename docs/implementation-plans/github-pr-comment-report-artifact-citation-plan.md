# GitHub PR Comment Report Artifact Citation Plan

Status: Planned; first validation-only helper implemented in [GitHub PR Comment Report Artifact Citation Helper Report](../concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_CITATION_HELPER_REPORT.md) and accepted with non-blocking follow-ups in [GitHub PR Comment Report Artifact Citation Helper Review](../concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_CITATION_HELPER_REVIEW.md). This plan follows the accepted [GitHub PR Comment SideEffect Event Append Executor Proof Review](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_EXECUTOR_PROOF_REVIEW.md). It defines how a future explicit report/artifact path should cite a persisted proposed GitHub pull request comment `SideEffectRecord` and the accepted `SideEffectProposed` workflow event without copying provider payloads or implying that a write executed.

This plan does not implement report artifact writes, live GitHub writes, provider mutation, runtime side-effect execution, automatic append, automatic discovery, attempted/completed/failed lifecycle behavior, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS can now model the first provider write candidate, validate it without provider calls, compose and persist a proposed `SideEffectRecord`, construct a reference-only `SideEffectProposed` workflow event, and explicitly append that event through the local executor.

The next safe step is citation planning. A future report artifact may need to show that a GitHub PR comment write was proposed and accepted into workflow history as a governed side-effect disclosure. That artifact should cite stable references only:

- the persisted proposed `SideEffectRecord`;
- the accepted `SideEffectProposed` workflow event;
- generic audit projection where available.

It must not copy the generated comment body, target repository details, provider payloads, pull request contents, diffs, logs, command output, credentials, or raw record JSON. It must also avoid implying that the side effect was attempted, completed, failed, approved, or externally visible.

## 2. Goals

- Define how report artifacts should cite persisted proposed GitHub PR comment records.
- Define how report artifacts should cite accepted `SideEffectProposed` workflow events.
- Preserve `SideEffectRecord` as durable proposed write-intent record.
- Preserve workflow events as accepted runtime history.
- Preserve WorkReport artifacts as governed handoff records, not audit logs or side-effect ledgers.
- Keep citations stable, typed, bounded, and redaction-safe.
- Reuse existing `WorkReportCitationTarget::SideEffect` vocabulary where sufficient.
- Reuse existing report artifact SideEffect referential integrity validation where sufficient.
- Identify any missing helper behavior before implementation.
- Prepare for a small implementation prompt that remains fixture-first and local.

## 3. Non-Goals

Do not implement:

- this plan in the planning phase;
- provider mutation;
- GitHub PR comment creation;
- live sandbox writes;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed`;
- automatic event append after record persistence;
- automatic discovery from all persisted proposed records;
- automatic report artifact writing from existing executor paths;
- WorkReport mutation after artifact creation;
- new public CLI behavior;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- GitHub PR comment write request/response model boundary;
- write preflight composition;
- fixture-backed adapter validation without provider calls;
- proposed `SideEffectRecord` composition;
- explicit proposed-record persistence through `SideEffectRecordStore`;
- `GitHubPullRequestCommentSideEffectEventContext`;
- proposed-event construction from a persisted record;
- persisted-record-to-`LocalExecutionSideEffectEventInput` helper;
- local executor append proof through `LocalExecutionRequest.side_effect_events`;
- generic audit projection from the accepted `SideEffectProposed` workflow event;
- WorkReport SideEffect citation vocabulary;
- terminal and executor report input propagation for explicit `SideEffectId` values;
- SideEffect discovery helpers;
- report artifact SideEffect referential integrity helper;
- explicit executor report artifact writing with generic SideEffect integrity and approval-linkage gates.

Not implemented:

- GitHub-specific report artifact citation helper;
- automatic artifact citation from GitHub proposed records;
- automatic artifact writes from default executor paths;
- attempted/completed/failed side-effect lifecycle events;
- live provider writes.

## 5. Source-Of-Truth Boundary

The report artifact citation path must keep sources distinct.

| Surface | Meaning |
| --- | --- |
| `SideEffectRecord` | Durable proposed side-effect intent, authority, lifecycle, target class, idempotency, and references. |
| `WorkflowRunEvent::SideEffectProposed` | Accepted runtime history that the proposed side-effect disclosure was attached before the targeted skill invocation. |
| `AuditEvent` | Bounded projection from accepted workflow history. |
| `WorkReport` | Governed handoff artifact that cites stable references. |
| `WorkReportArtifactRecord` | Durable validated report artifact, separate from workflow state. |

A persisted proposed record alone means proposed intent exists. An accepted workflow event means the executor accepted the disclosure into run history. A report artifact may cite both, but it must not upgrade either into proof of provider mutation.

## 6. Citation Model

Implemented first helper:

- cite the proposed GitHub PR comment by `WorkReportCitationTarget::SideEffect { side_effect_id }`;
- place the citation in the `SideEffects` report section;
- use bounded generic summary text such as `Proposed GitHub pull request comment side effect`;
- cite the accepted workflow event separately only if existing `WorkReportCitationTarget::WorkflowEvent` vocabulary is already available and suitable;
- otherwise disclose the accepted-event relationship through bounded section text and keep workflow-event citation vocabulary deferred.

The implementation should not add a GitHub-specific WorkReport citation target unless the generic SideEffect target proves insufficient.

## 7. Artifact Citation Policy

A future explicit artifact path should validate:

- the report cites the persisted proposed `SideEffectId`;
- the cited `SideEffectRecord` exists when strict integrity is requested;
- the cited record matches the report's immutable run identity;
- the accepted `SideEffectProposed` workflow event exists when accepted-event citation is requested;
- the accepted event references the same `SideEffectId`;
- the accepted event appears before the targeted `SkillInvocationRequested` event;
- no attempted/completed/failed lifecycle event is required or implied.

If accepted-event citation is not implemented yet, the artifact may still cite the `SideEffectId` after generic SideEffect referential integrity passes, but documentation and report text must avoid claiming accepted runtime history unless the event relationship was validated.

## 8. Proposed Helper Shape

Implemented first helper:

```text
GitHubPullRequestCommentReportArtifactCitationInput {
    artifact: WorkReportArtifactRecord,
    side_effect_id: SideEffectId,
    require_record: bool,
    require_accepted_event: bool,
}

validate_github_pr_comment_report_artifact_citations(
    side_effect_store: &impl SideEffectRecordStore,
    input: GitHubPullRequestCommentReportArtifactCitationInput,
) -> Result<GitHubPullRequestCommentReportArtifactCitationResult, WorkflowOsError>
```

The exact API should follow local naming conventions, but the first implementation should remain validation-only unless a separate phase explicitly approves a combined artifact write helper.

The helper should:

1. validate the artifact itself;
2. extract SideEffect citations from the contained report;
3. verify the expected `SideEffectId` is cited;
4. call the existing report artifact SideEffect referential integrity helper;
5. optionally inspect the report's associated run events only if the caller supplies them explicitly;
6. return bounded counts and booleans;
7. never mutate the artifact, report, run, workflow events, SideEffect records, or stores.

## 9. Workflow Event Relationship

Accepted-event validation should be explicit and caller-supplied.

The helper should not read hidden runtime state. If accepted-event validation is required, the caller should supply the relevant `WorkflowRun` or event slice. The helper should then verify:

- an event with kind `SideEffectProposed` references the expected `SideEffectId`;
- the event belongs to the same workflow/run identity as the artifact report;
- the event appears before the targeted skill invocation if target step/skill context is supplied;
- no attempted/completed/failed event is required for this proposed-only phase.

If workflow events are not supplied, the helper may validate record/artifact integrity but must not claim accepted-event integrity.

## 10. Failure Behavior

Failures must be stable and non-leaking.

Candidate error codes:

- `github_pr_comment_report_artifact_citation.side_effect_missing`;
- `github_pr_comment_report_artifact_citation.record_missing`;
- `github_pr_comment_report_artifact_citation.record_invalid`;
- `github_pr_comment_report_artifact_citation.identity_mismatch`;
- `github_pr_comment_report_artifact_citation.event_missing`;
- `github_pr_comment_report_artifact_citation.event_mismatch`;
- `github_pr_comment_report_artifact_citation.ordering_invalid`;
- `github_pr_comment_report_artifact_citation.integrity_failed`.

Errors must not include:

- SideEffect IDs;
- workflow IDs;
- run IDs;
- spec hashes;
- repository names;
- pull request numbers;
- target references;
- generated comment text;
- record JSON;
- report section text;
- provider payloads;
- command output;
- local paths;
- credentials;
- tokens or secret-like values.

## 11. Privacy And Redaction

The citation path must remain reference-only.

It must not copy, serialize, debug-print, or include in report summaries:

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
- secret-like values;
- untrusted repository or pull request strings beyond validated stable references.

Debug output for new input/result types should expose booleans and counts only.

## 12. Relationship To Existing Generic Integrity

This plan should reuse the generic report artifact SideEffect referential integrity helper.

The GitHub-specific helper should add only the extra meaning that:

- the cited SideEffect is the GitHub PR comment write candidate;
- the record lifecycle is `Proposed`;
- the record capability/target class is appropriate;
- an accepted `SideEffectProposed` workflow event can be validated when event history is supplied.

Do not fork generic SideEffect integrity logic.

## 13. Relationship To Approvals

Approval-side-effect linkage remains separate.

This citation plan may validate that a proposed GitHub PR comment record is cited. It should not decide whether the proposal had enough approval, whether the approver was authorized, or whether high-assurance approval policy was satisfied. Those checks belong to existing approval-linkage and high-assurance gates.

Future artifact paths may compose:

1. report artifact validation;
2. SideEffect referential integrity;
3. GitHub PR comment proposed-record/event citation validation;
4. approval-side-effect linkage;
5. high-assurance disclosure gates.

Each gate should remain explicit and caller-visible.

## 14. Relationship To Live Writes

This plan deliberately stays before live writes.

Citation of proposed intent and accepted disclosure does not mean:

- the comment was posted;
- the provider request was attempted;
- a provider response exists;
- rollback or compensation is available;
- write-capable adapters are production-ready.

Attempted/completed/failed lifecycle planning must happen before live provider mutation.

## 15. Test Plan

Future implementation tests should cover:

- report artifact citing a persisted proposed GitHub PR comment SideEffect passes strict citation validation;
- report artifact missing the expected SideEffect citation fails closed;
- missing SideEffect record fails closed when record is required;
- mismatched immutable record identity fails closed;
- non-proposed lifecycle is rejected;
- unsupported capability or target class is rejected;
- accepted `SideEffectProposed` event validates when supplied;
- missing accepted event fails when required;
- event with mismatched SideEffect ID fails closed;
- accepted event after targeted skill invocation fails if ordering validation is requested;
- attempted/completed/failed events are not required or implied;
- helper returns bounded counts only;
- Debug output does not leak IDs, target references, comment text, or report text;
- error output does not leak raw payloads or secret-like values;
- no provider mutation occurs;
- no report artifact is written by validation-only helper;
- existing report artifact, SideEffect integrity, approval linkage, provider-write, and local executor tests still pass.

## 16. Proposed Implementation Sequence

Completed:

1. Add a validation-only GitHub PR comment report artifact citation helper.
2. Reuse generic report artifact SideEffect referential integrity.
3. Add focused report artifact tests.

Next:

1. Plan composing the reviewed helper into an explicit artifact-capable executor path.
2. Keep live provider mutation and attempted/completed/failed lifecycle planning separate.

## 17. Deferred Work

- Automatic report artifact citation discovery.
- Automatic report artifact writing from default executor paths.
- Workflow-event citation target expansion if existing vocabulary is insufficient.
- Attempted/completed/failed side-effect lifecycle events.
- Live sandbox GitHub writes.
- Provider mutation.
- CLI commands.
- Workflow schema fields.
- Examples.
- Hosted runtime behavior.
- Reasoning lineage.
- Release posture changes.

## 18. Final Recommendation

Proceed next to GitHub PR comment report artifact citation helper-to-artifact-write composition planning.

The reviewed helper proves that a report artifact cites the persisted proposed GitHub PR comment `SideEffectRecord`, and when caller-supplied events are available, that the same SideEffect was accepted as `SideEffectProposed` workflow history. The next planning phase should define how that helper composes with the explicit artifact-capable executor path. Do not build live writes, attempted/completed/failed lifecycle events, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.
