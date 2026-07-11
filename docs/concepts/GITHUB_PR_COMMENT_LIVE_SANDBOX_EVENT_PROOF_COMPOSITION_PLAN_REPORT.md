# GitHub PR Comment Live Sandbox Event-Proof Composition Plan Report

## 1. Executive Summary

The live sandbox event-proof composition planning phase is complete.

The plan defines the next explicit bridge after the live sandbox runtime
composition helper: how already-completed live sandbox provider outcomes may
be projected into durable completed/failed SideEffect workflow event proof
without recalling the provider, fabricating evidence, making writes automatic,
or changing default executor behavior.

This phase is planning only. It does not implement event append behavior,
default writes, CLI mutation commands, hidden auth loading, schemas, examples,
hosted behavior, broad adapters, automatic retry or repair, reasoning lineage,
or release posture changes.

## 2. Scope Completed

- Created [GitHub PR Comment Live Sandbox Event-Proof Composition Plan](../implementation-plans/github-pr-comment-live-sandbox-event-proof-composition-plan.md).
- Defined the recommended explicit helper boundary.
- Defined required inputs and outputs.
- Defined gate order.
- Defined failure semantics.
- Defined bounded event-proof status vocabulary.
- Defined relationship to report disclosure and artifact gates.
- Defined privacy/redaction posture.
- Defined future test plan.
- Updated roadmap and related planning docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- live sandbox event append helper code;
- default provider writes;
- automatic executor provider writes;
- implicit event append from ordinary executor execution;
- provider calls;
- provider lookup;
- retry, repair, or recovery mutation;
- report artifact writing;
- CLI mutation behavior;
- hidden auth loading;
- workflow schema fields;
- SDK changes;
- examples;
- hosted or distributed runtime behavior;
- broad adapter write support;
- reasoning lineage;
- release posture changes.

## 4. Planning Boundary Summary

The plan recommends a helper such as
`compose_github_pr_comment_live_sandbox_event_proof(...)`.

The helper should accept an already completed live sandbox runtime composition
result, explicit run/event context, a caller-supplied side-effect store, an
event append policy, and an explicit idempotency key. It should append only
eligible completed/failed SideEffect lifecycle workflow events through the
existing event append helper.

It must not invoke the provider again.

## 5. Gate Summary

Future implementation should validate:

- explicit request shape;
- run/event context;
- provider-call-performed posture;
- completed or classified failed provider outcome;
- attempted SideEffect record identity;
- run, workflow, step, spec, and correlation identity;
- event append policy;
- idempotency binding;
- duplicate/conflicting event posture.

Only after those gates pass should existing event append helpers be used.

## 6. Privacy And Redaction Summary

The plan forbids storing or copying provider auth tokens, authorization
headers, raw provider payloads, raw pull request bodies, issue/review comments,
repository file contents, CI logs, command output, raw spec contents,
environment values, browser/session state, approval-presentation payload text,
or secret-like values.

Errors and Debug output must remain bounded and non-leaking.

## 7. Validation

Validation commands for this planning phase:

```sh
npm run check:docs
git diff --check
```

Result: passed.

Dogfood planning run:

- workflow: `dg/d`
- run: `run-1783784628384240000-2`
- approval: `approval/run-1783784628384240000-2/planning-approved`
- approval-presentation proof: `presentation/c5a4116cfb27c184`
- outcome: approved and completed

## 8. Remaining Known Limitations

- No implementation exists yet.
- The recommended helper name and result vocabulary may be adjusted to match
  repository conventions during implementation.
- The first implementation must still decide whether to accept a full
  `WorkflowRun` or a narrower validated event append context.
- Failed provider outcome event-proof eligibility should remain explicit.

## 9. Recommended Next Phase

Recommended next phase: live sandbox event-proof composition helper
implementation.

The implementation should stay narrow: explicit helper only, no provider call,
no default writes, no CLI behavior, no hidden auth, no schemas, no examples,
no artifact writes, no automatic retries or repairs, no hosted behavior, no
reasoning lineage, and no release posture changes.
