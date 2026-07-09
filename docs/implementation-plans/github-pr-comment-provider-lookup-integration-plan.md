# GitHub PR Comment Provider Lookup Integration Plan

Status: Planning only. This follows the accepted [GitHub PR Comment Provider Lookup HTTP Client Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_HTTP_CLIENT_REVIEW.md).

## 1. Executive Summary

Workflow OS now has:

- a provider write reconciliation model;
- executor-integrated live provider write helper;
- provider write workflow-event append helper;
- provider reconciliation disclosure/report composition;
- strict report artifact event-proof gates;
- provider event-proof recovery classification;
- provider lookup reconciliation model/helper;
- concrete injected-transport GitHub PR comment lookup HTTP client.

The next question is where lookup should be integrated so provider-side observations can help operators understand ambiguous provider-write outcomes without weakening event-proof gates.

This plan does not implement integration. It does not add automatic lookup, hidden auth, provider writes, retries, event append, state repair, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 2. Goals

- Define the smallest safe integration point for the concrete lookup HTTP client.
- Preserve explicit opt-in behavior.
- Preserve caller-supplied auth only.
- Keep lookup as bounded provider-side observation, not durable event proof.
- Preserve strict artifact gates that require accepted workflow event proof.
- Support future operator recovery workflows for ambiguous provider write outcomes.
- Avoid raw provider payloads, comment bodies, PR bodies, diffs, review threads, CI logs, command output, source files, tokens, and credentials.
- Keep failures stable and non-leaking.
- Keep default executor behavior unchanged.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- automatic provider lookup;
- hidden auth loading;
- background polling;
- automatic retries/backoff;
- provider writes;
- workflow event append from lookup;
- side-effect state repair;
- manual repair execution;
- report artifact writes based on lookup alone;
- CLI lookup/recovery command;
- workflow schema changes;
- examples;
- hosted/distributed behavior;
- broad provider mutation support;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- approval-presentation enforcement;
- release posture changes.

## 4. Current Capability Inventory

Implemented boundaries that lookup integration may compose:

- `GitHubPullRequestCommentProviderLookupClient` trait.
- `GitHubPullRequestCommentLookupHttpClient<T>`.
- `GitHubPullRequestCommentProviderLookupRequest`.
- `GitHubPullRequestCommentProviderLookupResponse`.
- `reconcile_github_pr_comment_provider_lookup`.
- provider event-proof recovery classification.
- provider reconciliation disclosure helper.
- report artifact event-proof gates.
- executor-integrated live provider write result model.

Still missing:

- executor-adjacent lookup request/result helper;
- recovery workflow/helper that combines provider disclosure posture with lookup observation;
- operator-facing lookup summary model;
- CLI or runtime exposure;
- live opt-in smoke test;
- manual repair helper;
- durable approval-presentation proof.

## 5. Recommended First Integration Target

Recommended first implementation target:

**Add an explicit executor-adjacent provider lookup recovery helper, in memory only.**

The helper should accept:

- a completed local provider-write result or explicit provider-write reconciliation disclosure;
- a caller-supplied lookup client;
- explicit lookup auth;
- expected provider reference or managed marker;
- side-effect identity and run identity;
- sensitivity and redaction metadata.

The helper should return:

- the existing lookup reconciliation result;
- provider recovery posture;
- bounded operator next action;
- retry-blocked/artifact-write-blocked flags;
- optional report disclosure input, if already supported by existing report models.

It should not mutate runtime state, append events, write side-effect records, write report artifacts, or perform repair.

## 6. Why This Target

This target closes the biggest current gap without overstepping:

- provider-write ambiguity already exists when remote/provider behavior and local lifecycle transition disagree;
- lookup can answer whether a managed remote comment appears to exist;
- operators need a bounded next-action classification;
- report artifact gates must remain strict and require durable event proof;
- the executor should not start doing hidden network recovery by default.

This is more useful than adding a CLI command first because it establishes the kernel composition boundary before exposing UX.

## 7. Explicit Input Policy

Allowed inputs:

- `WorkflowRunId`;
- `SideEffectId`;
- side-effect lifecycle/reconciliation references;
- provider-write disclosure posture;
- expected provider comment reference;
- expected managed marker;
- caller-supplied `GitHubPullRequestCommentProviderAuth`;
- injected lookup client;
- bounded correlation/idempotency metadata;
- sensitivity;
- redaction metadata.

Forbidden inputs:

- raw GitHub JSON;
- raw comment body;
- PR body;
- diffs;
- review-thread payloads;
- source file contents;
- CI logs;
- command output;
- environment variable values;
- credentials, tokens, authorization headers, private keys;
- natural-language proof that is not backed by stable references.

## 8. Lookup Invocation Policy

Lookup must be explicit.

Allowed:

- caller passes a lookup client;
- caller passes auth explicitly;
- caller passes expected stable remote reference or managed marker;
- helper invokes lookup once through the trait;
- helper maps response through existing reconciliation and recovery classifiers.

Forbidden:

- reading GitHub token from environment/keychain/git config/GitHub CLI;
- inferring repository or PR from git remotes;
- broad repository search;
- unbounded pagination;
- background polling;
- automatic retry;
- treating lookup observation as event proof;
- fabricating provider references or managed markers.

## 9. Artifact Gate Policy

Provider lookup observation must not satisfy strict report artifact event-proof gates.

Rules:

- remote observed can improve operator posture and next action;
- remote absent can inform retry planning;
- remote ambiguous can block retry and require manual resolution;
- unauthorized/unavailable/rate-limited/untrusted lookup cannot justify artifact writes;
- missing workflow event proof must continue to block strict artifact writes;
- only accepted workflow events may satisfy event-proof gates.

## 10. Error Handling

Integration errors must:

- use stable codes;
- avoid raw provider payloads;
- avoid raw paths;
- avoid raw metadata values;
- avoid token-like values;
- avoid source snippets, command output, parser output, or CI logs.

Recommended behavior:

- lookup request construction failure returns a structured lookup integration error;
- auth mismatch returns before transport invocation;
- lookup transport failure maps to bounded lookup-unavailable posture when possible;
- invalid response shape returns response-untrusted or stable construction error;
- helper failure does not mutate run status or side-effect records.

## 11. Privacy And Redaction

The integration helper must:

- use existing validated constructors;
- keep summaries bounded;
- redact Debug output;
- serialize only bounded model values;
- never copy provider payloads;
- never include authorization headers in models or errors;
- treat provider references and managed markers as potentially sensitive;
- preserve redaction metadata validation.

## 12. Test Plan

Future implementation tests should cover:

- explicit lookup integration returns remote observed posture;
- remote absent maps to retry-eligibility review without artifact write permission;
- remote ambiguous blocks retry and requires manual resolution;
- unauthorized lookup maps to provide-authorized-lookup next action;
- unavailable/rate-limited lookup maps to retry-lookup-later next action;
- response-untrusted maps to fix-lookup-input next action;
- auth mismatch rejects before transport invocation;
- helper uses injected client only;
- helper does not mutate `WorkflowRun`;
- helper does not append workflow events;
- helper does not write side-effect records;
- helper does not write report artifacts;
- lookup observation does not satisfy strict event-proof gates;
- raw provider/spec/command/parser/CI payloads are not copied;
- Debug and serialization do not leak secret-like values;
- existing provider write, lookup reconciliation, report artifact, executor, and side-effect tests continue to pass.

## 13. Proposed Implementation Sequence

1. Add an explicit provider lookup recovery integration helper, in memory only.
2. Add focused tests for observed, absent, ambiguous, unauthorized, unavailable, rate-limited, and untrusted lookup outcomes.
3. Add tests proving no events, records, artifacts, CLI output, hidden auth, or provider writes.
4. Review.
5. Only after review, plan CLI/operator exposure.
6. Only after separate review, plan manual repair.
7. Keep automatic lookup and hidden auth deferred.

## 14. Deferred Work

- CLI lookup/recovery command.
- Live opt-in GitHub lookup smoke test.
- Executor automatic lookup.
- Report artifact writing based on lookup.
- State repair.
- Workflow event append from recovery.
- Hidden/ambient auth loading.
- Automatic retry/backoff.
- Provider search across pages.
- Hosted lookup service.
- Examples.
- Schemas.
- Reasoning lineage.
- Approval-presentation enforcement.

## 15. Open Questions

- Should the first helper accept an executor provider-write result directly, or only explicit disclosure/reconciliation inputs?
- Should remote observed posture always block retry, or only when the expected marker/reference matches exactly?
- Should single-page lookup returning no observations be enough to classify remote absent, or should pagination uncertainty produce ambiguity?
- What is the smallest operator-facing summary that is useful without adding CLI behavior?
- Should live lookup smoke testing wait until CLI/operator exposure exists?
- Should future manual repair require high-assurance approval controls?

## 16. Final Recommendation

Proceed next to **provider lookup recovery integration helper, in memory only**.

The first implementation should compose the existing lookup client, lookup reconciliation helper, and recovery classifier through explicit caller-supplied inputs. It must not implement automatic lookup, hidden auth, provider writes, retries, event append, state repair, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 17. Governed Dogfood Run

- workflow_id: `dg/d`
- run_id: `run-1783566977020648000-2`
- approval_id: `approval/run-1783566977020648000-2/planning-approved`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-provider-lookup-integration-planning-scope`
