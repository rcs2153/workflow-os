# GitHub PR Comment Report Artifact Write Composition Plan

## 1. Executive Summary

The GitHub PR comment write-candidate lane now has a reviewed validation-only helper for report artifact citations. The helper can prove that a local `WorkReportArtifactRecord` cites an expected proposed GitHub PR comment `SideEffectId`, that the persisted record is shaped like a proposed GitHub pull-request-comment write, and that an accepted `SideEffectProposed` workflow event exists when caller-supplied events are provided.

The next implementation question is how to compose that helper with the existing explicit local report artifact write path. This plan does not implement artifact writes, provider mutation, live GitHub comments, runtime side-effect execution, attempted/completed/failed lifecycle events, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Goals

- Compose the reviewed GitHub PR comment citation helper with the existing explicit artifact-capable report write path.
- Keep composition opt-in, local, explicit, and validation-first.
- Validate proposed GitHub PR comment SideEffect citations before artifact write.
- Reuse existing generic artifact validation, SideEffect referential integrity, and approval-linkage gates.
- Preserve the distinction between proposed write intent, accepted workflow history, and report artifact handoff.
- Avoid copying provider payloads, generated comment bodies, pull request bodies, diffs, logs, command output, or credentials.
- Preserve workflow pass/fail semantics and runtime state.
- Prepare for a small implementation that returns bounded structured errors.

## 3. Non-Goals

Do not implement:

- provider mutation;
- GitHub PR comment creation;
- live sandbox writes;
- runtime side-effect execution;
- attempted/completed/failed lifecycle behavior;
- automatic event append;
- automatic SideEffect discovery;
- automatic report artifact writing from default executor paths;
- CLI behavior;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Already implemented and reviewed:

- GitHub PR comment request/response model boundary;
- write preflight helper and composition;
- fixture-backed adapter validation;
- proposed GitHub PR comment `SideEffectRecord` composition;
- proposed record persistence through `SideEffectRecordStore`;
- proposed event construction from a persisted record;
- persisted-record-to-executor-input helper;
- explicit executor append proof for `SideEffectProposed`;
- generic report artifact SideEffect referential integrity;
- report artifact high-assurance disclosure gate;
- approval-side-effect linkage helpers;
- explicit local artifact write helper;
- GitHub PR comment report artifact citation helper.

The missing piece is a small composition helper that runs the GitHub-specific citation validation before calling the existing explicit artifact write helper.

## 5. Proposed Composition Boundary

Add a helper such as:

```text
write_github_pr_comment_report_artifact_with_citations(
    artifact_store,
    side_effect_store,
    approval_records,
    input,
) -> Result<WorkReportArtifactRecord, WorkflowOsError>
```

The exact name should follow local conventions. The helper should accept explicit inputs only:

- terminal `WorkflowRun`;
- validated `WorkReportArtifactRecord` or `WorkReport`;
- expected GitHub PR comment `SideEffectId`;
- optional caller-supplied workflow events;
- report artifact gate policy;
- approval linkage policy;
- artifact store;
- SideEffect record store;
- approval records or existing approval-linkage input.

It should not read hidden runtime state or derive report content implicitly.

## 6. Validation Order

Recommended order:

1. Validate the artifact and immutable run identity against the supplied terminal run.
2. Validate GitHub PR comment citation integrity using `validate_github_pr_comment_report_artifact_citations(...)`.
3. Validate generic SideEffect referential integrity through the existing artifact write helper or preflight it once if the write helper requires it.
4. Validate approval linkage through the existing approval-linkage gate when policy requires it.
5. Write the artifact through the existing explicit local artifact write helper.
6. Return the stored artifact record.

The helper must not attach events, mutate the run, mutate the report after artifact creation, or create provider-visible side effects.

## 7. Accepted Event Policy

The first implementation should make accepted-event validation explicit.

Recommended fields:

- `require_github_pr_comment_record: bool`;
- `require_accepted_side_effect_event: bool`;
- `workflow_events: Option<&[WorkflowRunEvent]>`.

If `require_accepted_side_effect_event` is true, workflow events must be supplied and the GitHub citation helper must find a matching `SideEffectProposed` event.

If workflow events are not supplied, the helper may still validate proposed-record citation integrity, but it must not claim accepted-event integrity.

Targeted ordering against the matching `SkillInvocationRequested` event remains deferred until the helper accepts explicit target step/skill context.

## 8. Report Artifact Write Policy

The composition helper may write only through the existing explicit artifact store path.

It must not:

- make default executor paths write artifacts;
- persist reports unless the explicit artifact store is supplied;
- write files directly;
- append workflow events;
- emit audit/observability events;
- render CLI output;
- create or mutate GitHub provider resources.

Artifact write failure must not alter the already-completed workflow run.

## 9. Error Handling

Errors must be stable and non-leaking.

Candidate codes:

- `github_pr_comment_report_artifact_write.invalid_artifact`;
- `github_pr_comment_report_artifact_write.identity_mismatch`;
- `github_pr_comment_report_artifact_write.citation_invalid`;
- `github_pr_comment_report_artifact_write.approval_linkage_invalid`;
- `github_pr_comment_report_artifact_write.artifact_write_failed`.

Errors must not include:

- `SideEffectId` values;
- workflow/run IDs;
- spec hashes;
- repository names;
- pull request numbers;
- generated comment bodies;
- target references;
- raw report sections;
- provider payloads;
- raw record JSON;
- local paths;
- command output;
- credentials, tokens, or secret-like values.

## 10. Privacy And Redaction

The helper remains reference-only.

It should preserve:

- bounded `Debug` output;
- stable structured errors;
- no raw provider payloads;
- no generated comment bodies;
- no raw pull request content;
- no diffs or CI logs;
- no command output;
- no raw spec contents;
- no environment values;
- no credentials or authorization headers.

Reports and artifacts may remain sensitive even when citations are read-only.

## 11. Test Plan

Future implementation tests should cover:

- valid artifact write with cited proposed GitHub PR comment record;
- valid artifact write requiring accepted `SideEffectProposed` event;
- missing GitHub citation fails before artifact write;
- missing required record fails before artifact write;
- invalid GitHub record shape fails before artifact write;
- missing accepted event fails before artifact write when required;
- approval-linkage failure prevents artifact write;
- artifact identity mismatch fails without leaking IDs;
- artifact write store failure maps to stable non-leaking error;
- helper does not mutate `WorkflowRun`;
- helper does not append workflow events;
- helper does not call providers;
- helper does not create files directly;
- helper does not emit CLI output;
- no generated comment body, provider payload, diff, log, command output, or secret-like value is copied;
- bounded `Debug` output;
- existing report artifact, SideEffect, provider write, local executor, and WorkReport tests still pass.

## 12. Proposed Implementation Sequence

1. Add the explicit composition helper input/result types.
2. Call the reviewed GitHub PR comment citation helper first.
3. Compose with the existing explicit report artifact write helper.
4. Add focused tests for success, fail-closed citation validation, approval-linkage failure, and non-mutation.
5. Create an end-of-phase report.
6. Review before any live-write-adjacent phase.

## 13. Deferred Work

- Live GitHub PR comments.
- Attempted/completed/failed lifecycle events.
- Provider execution.
- Automatic event append.
- Automatic report artifact writing from default executor paths.
- Runtime config.
- CLI commands.
- Workflow schema fields.
- Examples.
- Hosted behavior.
- Reasoning lineage.
- Release posture changes.

## 14. Final Recommendation

Proceed next to a small implementation of the explicit GitHub PR comment report artifact write composition helper.

The implementation must remain local, explicit, validation-first, and no-provider-write. It should prove that the reviewed citation helper can guard the explicit artifact write path without making report artifacts automatic or implying that a GitHub comment was created.
