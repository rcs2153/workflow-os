# GitHub PR Comment Live Sandbox Event-Proof Composition Plan

Status: Helper implemented.

The first narrow helper implementation is documented in
[GitHub PR Comment Live Sandbox Event-Proof Composition Helper Report](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_HELPER_REPORT.md).
The maintainer review is documented in
[GitHub PR Comment Live Sandbox Event-Proof Composition Helper Review](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_HELPER_REVIEW.md)
with a `Needs blocker fixes` verdict for deterministic event-key binding and
correlation identity binding.
The helper remains explicit and opt-in. Default provider writes, automatic
executor writes, CLI mutation behavior, hidden auth loading, schemas, examples,
hosted behavior, broad adapters, automatic retry or repair, reasoning lineage,
report artifacts, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has an explicit GitHub PR comment live sandbox runtime
composition helper. That helper can invoke an injected provider after sandbox
target proof, sandbox readiness, explicit auth posture, provider-call input,
and attempted SideEffect lifecycle gates pass.

The next question is how a live sandbox provider outcome should become durable
workflow event proof without making provider writes automatic or fabricating
evidence. This plan defines the smallest future bridge from a successful or
classified failed live sandbox composition result to existing completed/failed
SideEffect workflow event append helpers.

This plan originally did not implement event append behavior. The first helper
implementation now adds an explicit event-proof composition helper that accepts
an already-composed live sandbox runtime result and appends completed/failed
workflow event proof only when the caller supplies a terminal run, append
policy, idempotency key, actor, and correlation context. It does not add
default writes, CLI mutation behavior, hidden auth loading, schemas, examples,
hosted behavior, broad adapters, automatic retry or repair, reasoning lineage,
report artifacts, or release posture changes.

## 2. Goals

- Define an explicit event-proof composition boundary for live sandbox
  provider outcomes.
- Reuse existing SideEffect lifecycle workflow event append helpers.
- Preserve default executor behavior.
- Preserve explicit caller opt-in.
- Preserve sandbox target proof and readiness as preconditions before any
  provider transport.
- Preserve provider-call, lifecycle transition, reconciliation, event-proof,
  report-disclosure, recovery, and artifact posture as separate concepts.
- Ensure completed/failed event proof is appended only for eligible provider
  outcomes.
- Keep missing event proof visible and bounded.
- Prepare a small implementation prompt without authorizing broader writes.

## 3. Non-Goals

Do not implement in the next phase unless separately approved:

- default provider writes;
- automatic executor provider writes;
- implicit event append from ordinary executor execution;
- CLI mutation commands;
- hidden auth loading;
- auth loading from environment, keychain, GitHub CLI, git credentials,
  browser sessions, config files, OAuth state, or secret managers;
- automatic retry;
- automatic repair or recovery mutation;
- automatic provider lookup;
- automatic report generation;
- automatic report artifact writing;
- workflow schema fields;
- SDK changes;
- examples;
- hosted or distributed runtime behavior;
- write-capable Jira, CI, filesystem, HTTP, or arbitrary adapters;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Implemented Boundary

The live sandbox runtime composition helper is implemented as:

- `GitHubPrCommentLiveSandboxRuntimeCompositionRequest`;
- `GitHubPrCommentLiveSandboxRuntimeCompositionResult`;
- `compose_github_pr_comment_live_sandbox_runtime(...)`.

It returns:

- sandbox readiness posture;
- provider-call orchestration result;
- provider-call-performed posture;
- `workflow_event_appended = false`;
- `report_artifact_written = false`.

The existing provider-write event append helper can append completed/failed
SideEffect lifecycle workflow events for eligible reconciled provider outcomes.

The first safe composition boundary is implemented as
`compose_github_pr_comment_live_sandbox_event_proof(...)`. It states when a
live sandbox runtime result is eligible to use the existing event append path
without calling the provider again.

## 5. Composition Problem

Provider success or classified provider failure is not, by itself, durable
workflow event proof.

For an event to become proof, the runtime needs to bind:

- the original run identity;
- workflow identity and spec hash;
- step identity;
- attempted SideEffect record identity;
- provider outcome;
- lifecycle transition result;
- idempotency key;
- correlation id;
- sandbox target proof;
- sandbox readiness result;
- approval-presentation proof when configured;
- event append policy.

Without this bridge, downstream report artifact gates should continue to see
provider outcomes as missing durable workflow event proof.

## 6. Recommended First Implementation Boundary

Add a new explicit helper-level composition path. Do not change
`LocalExecutor::execute(...)`.

Candidate helper:

```rust
compose_github_pr_comment_live_sandbox_event_proof(...)
```

Candidate request/result names:

```rust
GitHubPrCommentLiveSandboxEventProofCompositionRequest
GitHubPrCommentLiveSandboxEventProofCompositionResult
```

The helper should accept:

- a caller-supplied `WorkflowRun` or validated terminal/local run context;
- the live sandbox runtime composition result;
- a caller-supplied `SideEffectRecordStore`;
- explicit event append policy;
- explicit idempotency key;
- optional approval-presentation proof posture already validated elsewhere;
- optional report-disclosure inputs only as bounded posture;
- no provider implementation and no auth.

The helper should return:

- the original run or append result posture;
- completed/failed workflow event proof when appended;
- event-proof status;
- provider outcome status;
- report-disclosure eligibility posture;
- artifact-gate eligibility posture only if it can be derived without writing;
- stable non-leaking errors.

## 7. Gate Order

Recommended deterministic order:

1. Validate explicit event-proof composition request shape.
2. Validate the supplied run context and terminal/current event posture.
3. Validate that the live sandbox runtime result performed a provider call.
4. Validate that the provider outcome is completed or classified failed.
5. Validate attempted SideEffect record identity against the store.
6. Validate run/workflow/step/spec/correlation identity.
7. Validate event append policy.
8. Validate idempotency binding.
9. Append a completed/failed SideEffect workflow event only through the
   existing event append helper.
10. Return bounded event-proof posture.

Provider transport must already be complete before this helper is called. The
event-proof helper must not call the provider again.

## 8. Failure Semantics

Fail closed before event append when:

- live sandbox result is missing;
- provider call was not performed;
- provider outcome is unclassified or ambiguous;
- attempted SideEffect record is missing;
- run identity, workflow identity, step identity, spec hash, or correlation id
  mismatches;
- lifecycle transition does not match completed/failed event kind;
- idempotency key is missing or mismatched;
- event append policy is disabled;
- approval-presentation/event policy requires unavailable proof;
- event append would duplicate or conflict with an existing event.

After failure:

- do not call the provider;
- do not retry;
- do not repair provider/local ambiguity;
- do not fabricate workflow event proof;
- do not write report artifacts;
- preserve bounded recovery posture for the operator.

## 9. Event-Proof Status Model

The result should expose bounded vocabulary such as:

- `NotRequested`;
- `NotEligible`;
- `Blocked`;
- `Appended`;
- `AlreadyPresent`;
- `Conflict`;
- `Failed`.

The status should make it obvious whether a report/artifact path can cite
durable workflow event proof or must disclose missing proof.

## 10. Relationship To Report Disclosure And Artifacts

Report disclosure may describe provider outcome and event-proof posture, but
it must not imply durable proof when no workflow event was appended.

Artifact writing remains a separate helper path. A future implementation may
return artifact eligibility posture, but it must not write artifacts unless a
separately reviewed artifact-gated helper is explicitly invoked.

## 11. Privacy And Redaction

The event-proof helper must not store or copy:

- provider auth tokens;
- authorization headers;
- raw provider payloads;
- raw pull request bodies;
- raw issue or review comments;
- repository file contents;
- CI logs;
- command output;
- raw spec contents;
- environment variable values;
- browser/session state;
- approval-presentation payload text;
- secret-like values.

Errors must use stable codes and bounded messages. Debug output must be
redaction-safe.

## 12. Test Plan

Future implementation tests should cover:

- provider success event proof appends a completed SideEffect lifecycle event;
- classified provider failure appends a failed SideEffect lifecycle event when
  policy allows failed outcome proof;
- disabled event append policy returns no event and bounded posture;
- missing attempted record blocks event append;
- run identity mismatch blocks event append;
- step identity mismatch blocks event append;
- idempotency mismatch blocks event append;
- duplicate matching event is handled deterministically;
- conflicting existing event fails closed;
- unclassified/ambiguous provider outcome blocks event append;
- helper does not call the provider;
- helper does not write report artifacts;
- helper does not expose CLI output;
- Debug and error output do not leak provider payloads, auth, comment bodies,
  target strings, approval-presentation text, or secret-like values;
- existing live sandbox, provider-write, SideEffect, WorkReport, approval, and
  runtime tests still pass.

## 13. Documentation Updates

The first implementation updated:

- this plan;
- [Roadmap](../../ROADMAP.md);
- the live sandbox runtime composition plan;
- an end-of-phase report under `docs/concepts/`;

A maintainer review under `docs/concepts/` remains the recommended next phase.

Docs state that event proof is explicit and opt-in, not automatic. They also
state that default writes, CLI mutation commands, hidden auth loading, schemas,
examples, hosted behavior, broad adapters, automatic retry or repair, reasoning
lineage, report artifacts, and release posture changes remain unimplemented.

## 14. Open Questions

- Should the first helper accept a full `WorkflowRun`, or a narrower validated
  event append context?
- Should matching duplicate completed/failed events return `AlreadyPresent` or
  reuse the existing event append helper's duplicate semantics directly?
- Should failed provider outcomes be event-proof eligible by default, or only
  under explicit policy?
- Should the result include artifact eligibility posture, or leave that
  entirely to artifact-gated composition?
- What is the smallest useful test fixture that proves no provider recall
  occurs?

## 15. Final Recommendation

Proceed next with the bounded blocker fix identified by the maintainer review:
canonical event idempotency-key binding and correlation identity binding.

The implementation should append completed/failed SideEffect workflow events
only through existing eligibility rules and only from already completed live
sandbox runtime composition results. It must not call providers, broaden
writes, add CLI mutation behavior, load hidden auth, add schemas or examples,
write artifacts, retry, repair, implement reasoning lineage, or change release
posture.
