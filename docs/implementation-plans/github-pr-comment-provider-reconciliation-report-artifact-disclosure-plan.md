# GitHub PR Comment Provider Reconciliation Report Artifact Disclosure Plan

Status: Planned. This follows the accepted [GitHub PR Comment Provider Write Event Append Helper Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REVIEW.md). It defines how a future explicit report/artifact path should disclose provider-write reconciliation posture and workflow event append posture without implementing report artifact writing in this phase.

## 1. Executive Summary

Workflow OS now has an explicit GitHub pull request comment provider-write helper that can call a caller-supplied provider, persist local SideEffect lifecycle transitions, classify provider/local reconciliation posture, and append completed/failed SideEffect workflow events for eligible reconciled outcomes.

The next question is how report and report-artifact paths should represent that provider-write posture. A completed provider write, a failed provider write, an ambiguous provider response, a local transition mismatch, or an event append failure must not be flattened into generic success or generic failure.

This plan does not implement report generation, report artifact writing, provider writes, retries, auth loading, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Goals

- Define a reconciliation-aware disclosure boundary for future WorkReport and report artifact paths.
- Preserve the distinction between workflow execution status, provider-write status, SideEffect lifecycle state, workflow event append posture, and report/artifact posture.
- Cite stable SideEffect records and workflow events rather than copying provider payloads.
- Disclose ambiguous or split-brain provider/local states explicitly.
- Disclose event append failures explicitly instead of pretending the event trail is complete.
- Keep artifact writes fail-closed when a future strict artifact policy claims completed/failed provider-write evidence without matching local lifecycle and event evidence.
- Preserve default executor behavior and existing explicit helper boundaries.
- Keep errors stable, bounded, and redaction-safe.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- default provider writes;
- automatic provider calls;
- hidden auth loading from environment, keychain, GitHub CLI, git remotes, config files, OAuth, or secret managers;
- automatic retries or provider lookup/query reconciliation;
- new provider mutations beyond the existing explicit GitHub PR comment helper;
- report artifact writing in this phase;
- automatic report generation for every run;
- report persistence changes;
- CLI mutation behavior, rendering, or export;
- workflow schema fields;
- example updates;
- hosted or distributed runtime behavior;
- enterprise RBAC, IdP, quorum approval, or revocation enforcement;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- GitHub PR comment provider-call model and injected-provider helper;
- GitHub PR comment provider-write reconciliation model;
- executor-integrated live provider-write helper;
- blocker fix preserving classified provider response context when local transition fails;
- explicit completed/failed SideEffect event append for eligible reconciled provider outcomes;
- terminal runtime rehydration for completed/failed SideEffect outcome projections;
- WorkReport SideEffect citation vocabulary;
- report artifact SideEffect referential integrity validation;
- approval-linkage and high-assurance disclosure gates for explicit artifact paths;
- no-provider-write GitHub PR comment report artifact integration helper.

Still missing:

- reconciliation-aware WorkReport disclosure for provider-write outcomes;
- reconciliation-aware report artifact policy for provider-write outcomes;
- artifact gate treatment of missing completed/failed provider-write workflow events;
- operator recovery workflow after ambiguous provider/local states;
- provider lookup/query reconciliation;
- automatic retry handling.

## 5. Disclosure Boundary

The future disclosure helper should accept explicit provider-write result context. It should not call providers, read hidden state, load auth, append events, write artifacts, or mutate SideEffect records.

Preferred first shape:

```text
derive_github_pr_comment_provider_write_report_disclosures(...)
```

or an equivalent executor-adjacent helper that consumes the existing explicit provider-write result and returns bounded report/artifact disclosure inputs.

Inputs should be explicit:

- terminal `WorkflowRun`;
- `LocalExecutionWithGitHubPrCommentProviderWriteResult` or equivalent provider-write posture;
- optional expected `SideEffectId`;
- optional completed/failed workflow event IDs when already appended;
- optional report/artifact policy;
- optional store-backed SideEffect record lookup only if supplied by the caller.

The helper should produce structured disclosure data only. It must not write a `WorkReport`, write a report artifact, or create missing evidence.

## 6. Eligible Disclosure Inputs

Allowed inputs:

- workflow ID, workflow version, run ID, and terminal status;
- SideEffect ID and lifecycle state;
- provider-write reconciliation status;
- provider call performed flag;
- retry blocked flag;
- operator action required flag;
- workflow event append attempted/appended flag;
- stable workflow event IDs when available;
- stable artifact/report IDs when a caller already has them;
- bounded redaction metadata already validated by model constructors.

Forbidden inputs:

- raw provider payloads;
- GitHub comment bodies;
- GitHub PR bodies, diffs, review threads, or file contents;
- authorization headers, tokens, private keys, environment variables, or credential material;
- raw command output, CI logs, parser payloads, or raw spec contents;
- unbounded caller-supplied prose;
- local filesystem paths unless separately redacted and explicitly needed.

## 7. Reconciliation Posture Taxonomy

Future disclosure should preserve these postures:

- `provider_not_called`: no provider call occurred because a pre-call gate failed or provider invocation was disabled.
- `provider_succeeded_local_completed_event_appended`: provider success, local completed transition, reconciliation, and completed event append agree.
- `provider_failed_local_failed_event_appended`: provider failure, local failed transition, reconciliation, and failed event append agree.
- `provider_succeeded_local_completed_event_missing`: provider/local state agree, but completed event append was not performed or failed.
- `provider_failed_local_failed_event_missing`: provider/local failure state agrees, but failed event append was not performed or failed.
- `provider_response_ambiguous`: provider call outcome cannot safely prove success or failure.
- `provider_succeeded_local_transition_failed`: provider success exists, but local completed transition failed.
- `provider_failed_local_transition_failed`: provider failure exists, but local failed transition failed.
- `reconciliation_construction_failed`: provider/local inputs could not produce a valid reconciliation candidate.

The taxonomy should be represented with stable enum vocabulary or bounded string codes in the future implementation. It should not be inferred from natural-language report text.

## 8. WorkReport Disclosure Policy

WorkReports may disclose provider-write posture through bounded section text and stable citations.

Recommended section treatment:

- `work performed`: mention that the local workflow reached its terminal status, not that provider mutation succeeded unless the provider-write posture proves it.
- `evidence considered`: cite existing EvidenceReference and SideEffect IDs when provided; do not recreate evidence.
- `decisions made`: cite approval or policy references when supplied.
- `validation and quality checks`: cite validation references when supplied.
- `side effects`: disclose provider-write posture, lifecycle state, retry posture, operator-action posture, and event append posture.
- `known limitations`: include ambiguous provider/local or missing-event states as bounded limitations.
- `operator handoff notes`: require operator action text when retry is blocked or reconciliation is ambiguous.

Report summaries must not copy provider responses, comment bodies, diagnostic messages, auth context, raw errors, or filesystem paths.

## 9. Report Artifact Gate Policy

A future artifact-writing path should distinguish disclosure-only reports from strict artifact claims.

Recommended v1 strict policy:

- If an artifact claims a GitHub PR comment SideEffect completed, the SideEffect record must resolve to the same run and completed lifecycle state.
- If an artifact claims a GitHub PR comment SideEffect failed, the SideEffect record must resolve to the same run and failed lifecycle state.
- If a completed/failed provider-write workflow event is required by policy, the matching event ID must be present and must reference the same SideEffect and run.
- If provider/local reconciliation is ambiguous, strict artifact write must fail closed rather than writing a completed/failed claim.
- If event append failed or is missing while strict event proof is required, artifact write must fail closed.
- A disclosure-only report may still be generated in memory to explain the mismatch, but it must not claim completed/failed provider-write evidence as durable proof.

This phase does not implement artifact writing or new artifact gates.

## 10. Event Append Posture Policy

Completed/failed provider-write workflow events are evidence of agreed provider/local/reconciliation outcome projection, not the provider outcome by themselves.

Future report/artifact disclosure should include whether:

- event append was eligible;
- event append was attempted;
- event append succeeded;
- event append was skipped because the outcome was ineligible;
- event append failed after an eligible outcome;
- an existing idempotent event was reused.

Event append failure must not trigger another provider call. It should require explicit operator review if strict event proof is needed.

## 11. Failure Behavior

Future implementation errors must be stable and non-leaking.

Recommended error categories:

- `github_pr_comment_provider_disclosure.invalid_input`;
- `github_pr_comment_provider_disclosure.invalid_reconciliation`;
- `github_pr_comment_provider_disclosure.side_effect_mismatch`;
- `github_pr_comment_provider_disclosure.event_missing`;
- `github_pr_comment_provider_disclosure.event_mismatch`;
- `github_pr_comment_provider_disclosure.strict_artifact_gate_failed`.

Errors must not include SideEffect IDs, provider references, PR numbers, comment URLs, comment bodies, auth context, tokens, command output, file paths, raw provider errors, or secret-like metadata.

## 12. Privacy And Redaction

The future implementation must use existing validated constructors for WorkReport, WorkReportCitation, WorkReportArtifactRecord, SideEffectRecord, and reconciliation model values.

It must not store or copy:

- raw provider payloads;
- raw GitHub comment bodies;
- raw GitHub PR bodies, diffs, or file contents;
- raw CI logs or command output;
- raw spec contents or parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values;
- unbounded operator notes.

Debug output should expose bounded posture codes and counts only.

## 13. Test Plan

Future implementation should add focused tests for:

- provider-not-called posture produces disclosure without fake evidence;
- provider success plus local completed plus appended event produces completed disclosure;
- provider failure plus local failed plus appended event produces failed disclosure;
- provider success plus completed transition but missing event produces missing-event disclosure;
- provider ambiguity produces operator-action-required disclosure;
- local transition failure after provider response produces split-brain disclosure;
- strict artifact gate fails when completed/failed claims lack matching event proof;
- disclosure-only report can describe mismatch without writing an artifact;
- no provider call occurs during disclosure derivation;
- no event append occurs during disclosure derivation;
- no artifact write occurs during disclosure derivation;
- no raw provider payload, comment body, auth value, path, command output, parser payload, or secret-like string is copied;
- Debug and serialization remain redaction-safe;
- existing provider-write, report, artifact, SideEffect, approval-linkage, executor, runtime, adapter, and docs tests still pass.

## 14. Proposed Implementation Sequence

1. Add a small reconciliation disclosure model/helper that derives bounded disclosure posture from explicit provider-write result context.
2. Add WorkReport side-effect section integration using existing report constructors only.
3. Add strict report artifact gate input for provider-write event proof, still no automatic artifact write.
4. Add focused tests for success, failure, ambiguity, missing event, split-brain, and non-leakage.
5. Review before any artifact-writing composition.
6. Only after review, consider composing the helper into the explicit artifact-capable executor path.

## 15. Deferred Work

- Provider lookup/query reconciliation.
- Automatic retry or retry scheduling.
- Operator recovery workflow.
- Hidden auth loading.
- CLI mutation or display.
- Workflow schema support.
- Example updates.
- Hosted/distributed runtime behavior.
- Broader provider writes.
- Reasoning lineage.
- Recursive agents or agent swarms.
- Level 3/4 autonomy expansion.
- Release posture changes.

## 16. Final Recommendation

The next implementation prompt should be **reconciliation disclosure model/helper only**.

It should accept explicit provider-write posture, derive bounded report/artifact disclosure inputs, and add tests. It must still not write artifacts, call providers, append events, load auth, retry, expose CLI behavior, add schemas/examples, broaden write support, implement hosted behavior, or change release posture.

## 17. Governed Dogfood Summary

This planning phase was governed by the dogfood workflow `dg/d` with run `run-1783295782531926000-2`.

Approval:

- approval ID: `approval/run-1783295782531926000-2/planning-approved`
- approval reason: `delegated-maintainer-approved-provider-reconciliation-report-artifact-planning`

The governed run validated the dogfood project, paused for approval, resumed after delegated maintainer approval, and completed before documentation edits began.

Phase close:

- status: `Completed`
- events total: 39
- approvals: 1
- retries: 0
- escalations: 0
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`

Validation:

- `npm run check:docs` - passed.
- `git diff --check` - passed.

Out-of-kernel work:

- Documentation edits were made by Codex in the repository worktree.
- No runtime code, provider calls, event appends, artifact writes, persistence changes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes were implemented.
