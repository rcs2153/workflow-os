# GitHub PR Comment Report Artifact Executor Integration Plan

Status: Implemented as an explicit local helper in
[GitHub PR Comment Report Artifact Executor Integration Helper Report](../concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_EXECUTOR_INTEGRATION_HELPER_REPORT.md).
This plan does not implement runtime behavior.

## 1. Executive Summary

The GitHub PR comment write-candidate lane now has a no-provider-write chain of primitives:

- write preflight;
- fixture-backed request validation;
- proposed `SideEffectRecord` composition and persistence;
- proposed `SideEffectProposed` event construction and executor append proof;
- report artifact citation validation;
- explicit local report artifact write composition with generic artifact gates;
- focused hardening coverage and accepted hardening review.

The next question is how a future executor-adjacent path should compose those primitives into one explicit local integration boundary. The first implementation should remain opt-in, local, fixture-first, and no-provider-write. It should not make live GitHub comments, provider mutation, automatic artifact writes, CLI mutation commands, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Goals

- Define the next explicit integration boundary after composition-helper hardening.
- Connect existing proposed GitHub PR comment SideEffect records, accepted proposed events, generated reports, and report artifacts.
- Preserve workflow pass/fail semantics.
- Keep report artifact writing explicit and caller-supplied.
- Require stable citations before artifact write.
- Reuse existing `WorkReportArtifactStore` and `SideEffectRecordStore` boundaries.
- Reuse existing report artifact SideEffect integrity, approval linkage, and high-assurance disclosure gates.
- Avoid raw provider payloads, generated comment bodies, pull request bodies, diffs, CI logs, command output, local paths, credentials, tokens, and secret-like values.
- Prepare a small implementation prompt that proves end-to-end local composition without provider mutation.

## 3. Non-Goals

Do not implement:

- live GitHub PR comments;
- provider mutation;
- runtime side-effect execution;
- attempted/completed/failed lifecycle events;
- automatic artifact writes from default executor paths;
- automatic report generation for every run;
- CLI mutation behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Already implemented:

- GitHub PR comment request/response model boundary.
- Write preflight helper and preflight composition.
- Fixture-backed adapter validation.
- Proposed GitHub PR comment `SideEffectRecord` composition.
- Proposed record persistence through `SideEffectRecordStore`.
- Proposed `SideEffectProposed` event construction.
- Persisted-record-to-executor-input bridge.
- Executor append proof for explicit proposed SideEffect events.
- GitHub PR comment report artifact citation helper.
- Explicit local composition helper that validates GitHub citation integrity before artifact write.
- Generic report artifact SideEffect referential integrity.
- Approval-linkage gates for SideEffect records.
- High-assurance disclosure artifact gate.
- Explicit executor artifact path with generic SideEffect gates.

The missing piece is a narrow executor-adjacent integration that takes the already-produced proposed GitHub PR comment SideEffect context and the generated report artifact context, then calls the reviewed GitHub-specific artifact write composition helper.

That missing piece is now implemented as
`write_github_pr_comment_report_artifact_from_explicit_context(...)`. The
helper remains explicit, local, no-provider-write, and not wired into default
executor behavior.

## 5. Proposed Integration Boundary

Add an explicit local helper or executor-adjacent API such as:

```text
execute_with_github_pr_comment_report_artifact(...)
```

or:

```text
write_github_pr_comment_report_artifact_from_execution_result(...)
```

The exact name should follow existing executor/report conventions.

The helper should accept explicit inputs:

- terminal `WorkflowRun` or existing executor report/artifact result;
- generated `WorkReport` or validated `WorkReportArtifactRecord`;
- expected GitHub PR comment `SideEffectId`;
- explicit `SideEffectRecordStore`;
- explicit `WorkReportArtifactStore`;
- optional caller-supplied workflow events;
- explicit citation policy;
- explicit generic artifact gate policy;
- explicit approval-linkage policy;
- explicit high-assurance disclosure policy.

It must not read hidden runtime state, infer runtime config, call providers, generate comments, append events, or create artifacts unless the caller supplies the artifact store and the gates pass.

## 6. Recommended First Implementation Shape

Preferred first implementation:

1. Accept an already-terminal local execution result plus explicit report artifact inputs.
2. Build or accept a validated `WorkReportArtifactRecord`.
3. Load or accept the expected proposed GitHub PR comment SideEffect record by stable ID.
4. Validate the optional accepted proposed-event evidence when caller supplies workflow events and enables the policy.
5. Call `write_github_pr_comment_report_artifact_with_citations(...)`.
6. Return a structured result containing:
   - original run;
   - report or artifact;
   - GitHub citation validation result if successful;
   - artifact write result if successful;
   - optional bounded artifact write error;
   - no provider response.

This should be implemented as a helper first, not by changing default executor methods.

## 7. Runtime Semantics

The helper must preserve existing workflow semantics.

Rules:

- execution failure before a run exists remains `Err`;
- report or artifact failure after a run exists must not change the run result;
- artifact write failure must not mutate the run or append events;
- provider mutation remains unsupported;
- accepted proposed-event evidence proves only that the proposed SideEffect event was accepted into workflow history, not that a GitHub comment was created;
- no post-terminal workflow events are appended by the helper;
- no audit or observability events are emitted by the helper in the first slice.

## 8. Citation And Event Policy

The integration should make citation requirements explicit.

Recommended policy fields:

- require expected SideEffect citation in the report artifact;
- require persisted proposed GitHub PR comment SideEffect record;
- require accepted `SideEffectProposed` event when workflow events are supplied and the caller opts in;
- require approval linkage when the SideEffect authority says approval is required;
- require high-assurance disclosure only through existing artifact gate policy.

Missing stable IDs must fail closed when required. They must not be fabricated.

## 9. Artifact Write Policy

Artifact writes remain explicit.

Rules:

- write only through caller-supplied `WorkReportArtifactStore`;
- do not write arbitrary files;
- do not make default executor paths write artifacts;
- do not make artifact writing automatic for all terminal runs;
- do not store generated GitHub comment body text;
- do not store provider payloads;
- preserve existing duplicate artifact handling;
- return bounded errors for write failure.

## 10. Error Handling

Errors should use stable, non-leaking codes.

The integration can reuse existing composition codes where possible:

- `github_pr_comment_report_artifact_write.invalid_artifact`;
- `github_pr_comment_report_artifact_write.identity_mismatch`;
- `github_pr_comment_report_artifact_write.citation_invalid`;
- `github_pr_comment_report_artifact_write.approval_linkage_invalid`;
- `github_pr_comment_report_artifact_write.artifact_write_failed`.

If a new executor-adjacent wrapper is introduced, wrapper errors should preserve these inner codes or map to a bounded namespace without exposing raw IDs, target references, paths, provider payloads, report text, command output, credentials, tokens, or secret-like values.

## 11. Privacy And Redaction

The integration remains reference-only.

It must not copy:

- generated comment bodies;
- GitHub provider payloads;
- pull request bodies;
- diffs;
- CI logs;
- command output;
- raw SideEffect record JSON;
- raw report sections into errors;
- local paths;
- environment values;
- credentials;
- authorization headers;
- tokens or private keys.

Debug output should expose only bounded counts, booleans, and stable posture. Reports and artifacts remain sensitive even when all citations are read-only references.

## 12. Test Plan

Future implementation tests should cover:

- valid terminal result plus GitHub PR comment SideEffect record writes artifact through the explicit store;
- accepted proposed-event requirement succeeds when matching event exists;
- missing accepted event fails before artifact write;
- missing SideEffect citation fails before artifact write;
- identity mismatch maps to stable non-leaking error;
- approval-linkage failure prevents artifact write;
- high-assurance disclosure failure prevents artifact write when policy requires it;
- artifact store failure preserves run/report and returns bounded error;
- helper does not mutate `WorkflowRun`;
- helper does not append events;
- helper does not call providers;
- helper does not create files directly outside `WorkReportArtifactStore`;
- helper emits no CLI output;
- raw provider/spec/command/parser payload markers are not copied;
- Debug and serialization remain non-leaking;
- existing generic artifact gate, GitHub citation, provider write, local executor, and WorkReport tests still pass.

## 13. Proposed Implementation Sequence

1. Add a small executor/report-adjacent integration input and result type.
2. Accept explicit terminal run, report artifact, expected SideEffect ID, stores, events, and policies.
3. Call the reviewed composition helper.
4. Preserve run/report on artifact failure.
5. Add focused tests for success, no-write-on-failure, non-mutation, and redaction.
6. Create an implementation report.
7. Review before any live-provider or lifecycle-expansion phase.

## 14. Deferred Work

- Live GitHub PR comments.
- Provider mutation.
- Attempted/completed/failed lifecycle modeling.
- Automatic artifact writing from executor paths.
- CLI rendering or export.
- Workflow schema fields.
- Examples.
- Hosted runtime behavior.
- Reasoning lineage.
- Release posture changes.

## 15. Open Questions

- Should the first helper start from an existing `WorkReportArtifactRecord`, or should it accept a report-bearing executor result and construct the artifact internally?
- Should accepted-event validation require a specific step or skill ID in the first integration slice, or remain event-presence only until targeted context is modeled?
- Should duplicate artifact writes remain hard failures, or should byte-identical idempotent replay be allowed later?
- Should the result preserve inner composition result types directly, or wrap them in executor-adjacent result vocabulary?

## 16. Dogfood Governance

- Workflow: `dg/d`.
- Run ID: `run-1783222411477337000-2`.
- Approval ID: `approval/run-1783222411477337000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer.
- Event summary: 39 events; `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Retries: 0.
- Escalations: 0.
- Validation: `npm run check:docs` passed.
- Out-of-kernel work disclosed: docs edits, validation command, git/PR actions, and this planning update.

## 17. Final Recommendation

Recommended next implementation phase: **explicit GitHub PR comment artifact integration helper, local only**.

Start with a helper that accepts a terminal run or report-bearing result, a validated `WorkReportArtifactRecord`, explicit stores, expected GitHub PR comment `SideEffectId`, optional workflow events, and gate policies. It should return structured in-memory results and bounded errors.

Still do not build provider mutation, live comments, automatic artifact writes, CLI mutation behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.
