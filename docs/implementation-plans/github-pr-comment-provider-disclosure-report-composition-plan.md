# GitHub PR Comment Provider Disclosure Report Composition Plan

Status: Planned. This follows the accepted [GitHub PR Comment Provider Reconciliation Disclosure Helper Blocker Fix Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_RECONCILIATION_DISCLOSURE_HELPER_BLOCKER_FIX_REVIEW.md).

This plan defines how the accepted GitHub pull request comment provider reconciliation disclosure should be composed into WorkReport and report artifact paths without overclaiming event proof, calling providers, appending events, writing artifacts implicitly, or changing workflow semantics.

This plan does not implement anything.

## 1. Executive Summary

Workflow OS now has an explicit executor-adjacent GitHub PR comment provider-write path and a bounded disclosure helper that classifies provider/local/event-proof posture.

The next question is how that disclosure should be represented in terminal WorkReports and later report artifacts. The report path should disclose the provider/local reconciliation posture as bounded report content and citations. It must not turn disclosure into durable event proof, retry authority, provider-call authority, report artifact write authority, or a claim that a GitHub comment is externally visible when the event proof is missing.

The recommended first implementation is a small explicit WorkReport input/section composition slice only. Strict report artifact event-proof gates should remain a separate later implementation after the WorkReport disclosure path is reviewed.

## 2. Goals

- Add a bounded path for explicit provider reconciliation disclosure to appear in generated WorkReports.
- Preserve the provider-write result as the source of provider/local/event-proof posture.
- Preserve WorkReport as a governed handoff artifact, not an audit log or side-effect ledger.
- Preserve workflow events as the durable event-proof source.
- Preserve report artifacts as explicit durable report records, not automatic runtime output.
- Cite stable references only.
- Avoid copying provider payloads, PR bodies, comment bodies, diffs, command output, raw specs, paths, credentials, or token-like values.
- Keep missing event proof visible.
- Keep provider/local agreement distinct from workflow event proof.
- Prepare a later report artifact gate phase that can require event proof before artifact writes.

## 3. Non-Goals

Do not implement:

- implementation in this planning phase;
- provider calls;
- GitHub comment creation;
- provider lookup/query reconciliation;
- automatic retries;
- workflow event appends;
- audit sink emission;
- observability emission;
- automatic report generation;
- automatic report artifact writing;
- CLI behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime behavior;
- broader write-capable adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- GitHub PR comment request/response model boundary.
- Write preflight and side-effect record composition/persistence.
- Store-backed attempted/completed/failed lifecycle transition helpers.
- Provider-call trait/input model.
- Concrete injected-transport GitHub PR comment provider client.
- Provider write reconciliation candidate model/helper.
- Explicit executor-integrated provider-write request/result/helper.
- Provider write event append helper for eligible reconciled outcomes.
- Bounded provider reconciliation disclosure helper.
- Focused missing-event disclosure tests.
- WorkReport model and terminal report helper.
- Explicit executor report-bearing result path.
- Explicit local report artifact store.
- Generic report artifact side-effect integrity helper.
- GitHub PR comment report artifact citation helper.
- Explicit provider-candidate report artifact integration helper.

Not implemented:

- WorkReport section population from provider reconciliation disclosure.
- Report artifact gate policy based directly on provider reconciliation disclosure posture.
- Operator recovery workflow for missing event proof.
- Provider lookup/query reconciliation.
- Default executor automatic report/artifact behavior.
- Live write adapter readiness.

## 5. Source-Of-Truth Boundaries

The composition must keep sources distinct.

| Surface | Source-of-truth role |
| --- | --- |
| `LocalExecutionWithGitHubPrCommentProviderWriteResult` | Explicit in-memory result containing provider response/error, local lifecycle transition, reconciliation candidate, and workflow-event-appended posture. |
| `GitHubPullRequestCommentProviderWriteReportDisclosure` | Bounded projection of provider/local/event-proof posture. |
| `SideEffectRecordStore` | Local side-effect lifecycle source of truth. |
| `WorkflowRunEvent` | Durable runtime event proof. |
| `WorkReport` | Governed handoff artifact that can disclose posture and cite stable references. |
| `WorkReportArtifactRecord` | Durable validated report artifact, separate from workflow state. |

A WorkReport may disclose that provider/local reconciliation succeeded while event proof is missing. It must not upgrade that disclosure into event proof.

## 6. Recommended First Implementation Target

Implement only explicit WorkReport disclosure composition.

Recommended shape:

- Add an optional explicit provider disclosure input to the terminal report helper or executor report input boundary.
- Accept a precomputed `GitHubPullRequestCommentProviderWriteReportDisclosure` or a small report-safe wrapper around it.
- Populate the `SideEffects` report section with bounded text that reflects the disclosure posture.
- Add stable citations only when stable IDs already exist in existing input fields.
- Do not create new `EvidenceReference`, `SideEffectRecord`, workflow event, audit event, or report artifact values.

This first slice should not write artifacts. It should only return an in-memory WorkReport through existing explicit report-generation paths.

## 7. Report Section Policy

Provider disclosure belongs in the `side effects` section.

Recommended bounded section text by posture:

| Disclosure posture | Report posture text |
| --- | --- |
| `ProviderNotCalled` | Provider call not performed. |
| `ProviderSucceededLocalCompletedEventAppended` | Provider success, local completed state, and workflow event proof are present. |
| `ProviderSucceededLocalCompletedEventMissing` | Provider success and local completed state are present, but workflow event proof is missing. |
| `ProviderFailedLocalFailedEventAppended` | Provider failure, local failed state, and workflow event proof are present. |
| `ProviderFailedLocalFailedEventMissing` | Provider failure and local failed state are present, but workflow event proof is missing. |
| `ProviderResponseAmbiguous` | Provider response is ambiguous; reconciliation is required before retry. |
| `ProviderSucceededLocalTransitionFailed` | Provider success was observed, but local completed transition failed. |
| `ProviderFailedLocalTransitionFailed` | Provider failure was observed, but local failed transition failed. |
| `LocalStateAmbiguous` | Local lifecycle state is ambiguous; reconciliation is required. |
| `ReconciliationRequired` | Reconciliation is required before further provider action. |
| `ReconciliationUnavailable` | Reconciliation candidate is unavailable. |

The exact strings may be refined in implementation, but they must be bounded, deterministic, non-marketing, and redaction-safe.

## 8. Citation Policy

The WorkReport path should cite only stable references already supplied by existing explicit inputs:

- `SideEffectId` through existing SideEffect citation vocabulary;
- workflow event ID when the caller supplies durable event proof;
- audit event ID when available;
- adapter/provider telemetry ID only if stable and already modeled;
- approval/high-assurance disclosure IDs only through existing explicit input paths.

Rules:

- Do not create `EvidenceReference` values.
- Do not create SideEffect records.
- Do not fabricate event IDs.
- Do not infer event proof from provider/local agreement.
- Do not copy raw provider responses.
- Do not copy GitHub comment body, PR body, diffs, review threads, or file contents.
- Missing event proof should remain section text unless and until explicit missing-citation records are implemented.

## 9. Report Artifact Policy

Strict report artifact behavior should be deferred to a separate implementation phase.

The later artifact phase should decide how to use the disclosure posture before artifact write:

- allow artifact write when disclosure posture includes event proof and other gates pass;
- fail artifact write or record explicit artifact error when policy requires event proof and disclosure says event proof is missing;
- preserve run/report result semantics when artifact write is denied;
- never append events or call providers from the artifact path;
- keep provider/local agreement distinct from durable event proof.

This planning phase should not authorize automatic artifact writes or default executor artifact behavior.

## 10. Failure Behavior

WorkReport disclosure composition failures should be report-generation errors only when existing report-generation architecture supports that boundary.

Recommended behavior:

- invalid disclosure input fails before report creation;
- report generation failure after a workflow run exists does not change workflow pass/fail result;
- missing event proof is not an error by itself unless a future caller policy explicitly requires event proof;
- errors use stable non-leaking codes;
- errors do not include provider payloads, comment text, repository identity, PR numbers, paths, tokens, raw IDs where sensitive, command output, raw specs, or secret-like values.

Candidate error codes:

- `github_pr_comment_provider_disclosure_report.invalid_input`;
- `github_pr_comment_provider_disclosure_report.unsupported_posture`;
- `github_pr_comment_provider_disclosure_report.unbounded_summary`;
- `github_pr_comment_provider_disclosure_report.secret_like_value`.

## 11. Privacy And Redaction

The first implementation must remain reference-only and bounded.

It must not store, serialize, debug-print, or include in report text:

- raw provider payloads;
- GitHub comment bodies;
- GitHub PR bodies;
- diffs or file contents;
- review thread bodies;
- CI logs;
- command output;
- parser payloads;
- raw specs;
- environment variable values;
- credentials, authorization headers, private keys, or tokens;
- unbounded targets, summaries, reasons, or operator notes.

Debug output for any new input or wrapper type must be bounded and redaction-safe.

## 12. Test Plan

Future implementation tests should cover:

- provider-not-called disclosure populates side-effects report section;
- provider-success/local-completed/event-appended disclosure is represented without copying payloads;
- provider-success/local-completed/event-missing disclosure explicitly says event proof is missing;
- provider-failure/local-failed/event-appended disclosure is represented without copying payloads;
- provider-failure/local-failed/event-missing disclosure explicitly says event proof is missing;
- ambiguous/transition-failed/reconciliation-unavailable postures are represented distinctly;
- disclosure composition cites supplied SideEffect IDs without creating new SideEffect records;
- supplied workflow event IDs are cited only when provided;
- missing event IDs do not fabricate citations;
- no raw provider/comment/PR/diff/command/spec/parser payloads are copied;
- secret-like disclosure text is rejected or safely handled;
- report-generation failure preserves workflow result semantics;
- no workflow events are appended;
- no provider calls occur;
- no report artifacts are written;
- existing WorkReport, report artifact, provider-write, SideEffect, executor, validation, and runtime tests continue to pass.

## 13. Proposed Implementation Sequence

1. Add a small explicit provider disclosure input type or reuse the accepted disclosure type directly if that fits existing report input conventions.
2. Map disclosure posture to bounded side-effects section text.
3. Forward stable SideEffect/event citations only when supplied through explicit inputs.
4. Add focused WorkReport helper tests for appended and missing event proof postures.
5. Add redaction/non-leakage tests.
6. Review before any report artifact gate integration.
7. Plan strict artifact event-proof gates separately.

## 14. Open Questions

- Should the report helper accept the existing disclosure type directly, or should it accept a report-specific wrapper?
- Should missing event proof be represented only as section text, or should it later become a typed missing-citation record?
- Should provider disclosure live only in the `side effects` section, or also in `known limitations` when reconciliation is required?
- Should event-missing postures ever block report generation, or only artifact writes under strict policy?
- Should artifact gates require event proof by default, or only when configured?

## 15. Final Recommendation

Next implementation phase: **WorkReport provider reconciliation disclosure composition, in-memory only**.

The implementation should add explicit provider disclosure input to the in-memory report path and populate bounded side-effects report content. It must not write artifacts, append events, call providers, retry, load auth, expose CLI behavior, add schemas/examples, broaden writes, implement hosted behavior, implement reasoning lineage, or change release posture.

## 16. Governed Planning Summary

This planning phase was governed by the dogfood workflow `dg/d` with run `run-1783301085979762000-2`.

Approval:

- approval ID: `approval/run-1783301085979762000-2/planning-approved`
- approval reason: `delegated-maintainer-approved-provider-disclosure-composition-planning`

Validation:

- `npm run check:docs` - passed.
- `git diff --check` - passed.

Phase close:

- status: `Completed`
- terminal: `true`
- events total: 39
- approvals: 1
- retries: 0
- escalations: 0
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work:

- this planning document and roadmap pointer were written by Codex in the repository worktree;
- validation commands were run outside the kernel and are listed above;
- no implementation, provider calls, workflow event appends, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, broad writes, or release posture changes were performed.
