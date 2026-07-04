# GitHub PR Comment Preflight Composition Plan

Status: Implemented as model/helper-only preflight composition. Fixture-backed adapter execution is planned separately in [GitHub PR Comment Fixture Adapter Plan](github-pr-comment-fixture-adapter-plan.md). This plan follows the accepted GitHub PR comment write boundary review and defines the narrow runtime-composition step before any fixture-backed adapter or provider write work.

## 1. Executive Summary

Workflow OS now has two separate write-readiness primitives:

- an adapter-neutral `preflight_adapter_write(...)` helper that classifies future write readiness without execution;
- a model-only GitHub pull request comment write request/response boundary.

The gap is composition. A future GitHub PR comment path must not merely contain a matching preflight request; it must execute the preflight helper and require a ready decision before any fixture-backed adapter or future provider invocation.

This plan defines that composition boundary, and the model/helper-only composition slice is implemented. It does not implement provider writes, fixture-backed adapter execution, live sandbox calls, CLI commands, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Goals

- Compose `GitHubPullRequestCommentWriteRequest` with `preflight_adapter_write(...)`.
- Require an explicit ready preflight decision before any future fixture or provider execution path.
- Preserve the current no-execution boundary.
- Keep preflight composition deterministic, local, and explicit-input-only.
- Reuse existing request constructors, preflight constructors, policy references, approval references, SideEffect IDs, idempotency keys, sensitivity, and redaction metadata.
- Return structured non-leaking composition errors.
- Make fixture-backed adapter implementation safer by forcing preflight execution first.

## 3. Non-Goals

This plan does not authorize:

- implementation in this phase;
- GitHub provider calls;
- pull request comment creation;
- fixture-backed adapter execution;
- live sandbox writes;
- runtime side-effect execution;
- SideEffect attempted/completed/failed lifecycle transitions;
- automatic workflow event appends;
- automatic report generation or report artifact writing;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- OAuth app behavior or webhook ingestion;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented baseline primitives:

- `AdapterWritePreflightRequest`;
- `AdapterWritePreflightDecision`;
- `AdapterWriteReadinessPolicy`;
- `preflight_adapter_write(...)`;
- `GitHubPullRequestCommentWriteRequest`;
- `GitHubPullRequestCommentWriteResponse`;
- `github_pr_comment_preflight_definition(...)`;
- request-side validation that embedded preflight capability, target reference, SideEffect ID, and idempotency key match the GitHub PR comment request.

Implemented:

- a helper that executes `preflight_adapter_write(...)` for a GitHub PR comment write request;
- a composed result type that carries both request and ready preflight decision.

Not implemented:

- fixture-backed adapter execution;
- live provider mutation;
- SideEffect lifecycle transitions;
- workflow event/audit projection;
- report artifact write disclosure;
- CLI or schema surfaces.

## 5. Composition Boundary

The implementation adds a small explicit helper in `workflow-core`:

```text
GitHubPullRequestCommentPreflightedWrite::new(request)
  -> Result<GitHubPullRequestCommentPreflightedWrite, WorkflowOsError>
```

It:

- accepts an already validated `GitHubPullRequestCommentWriteRequest`;
- calls `preflight_adapter_write(request.preflight())`;
- verifies the returned decision is ready;
- verifies the decision capability, SideEffect ID, idempotency key, and no-execution posture still align with the request;
- returns a redaction-safe composed value.

The helper must not:

- call GitHub;
- call a fixture adapter;
- transition SideEffect lifecycle state;
- append workflow events;
- emit audit events;
- write report artifacts;
- read hidden runtime config;
- read credentials;
- create CLI output.

## 6. Proposed Model Concepts

The implemented model set is:

- `GitHubPullRequestCommentPreflightedWrite`

`GitHubPullRequestCommentPreflightedWrite` contains:

- the validated GitHub PR comment write request;
- the validated `AdapterWritePreflightDecision`;
- read-only accessors;
- no provider response;
- no SideEffect lifecycle state change;
- no workflow event or audit event.

It should provide explicit no-execution accessors equivalent to the underlying request/decision:

- `provider_call_allowed() == false`
- `workflow_event_append_allowed() == false`
- `side_effect_lifecycle_transition_allowed() == false`
- `report_artifact_write_allowed() == false` if exposed by the underlying decision

## 7. Validation Rules

The helper must fail closed unless:

1. the request is already valid;
2. the preflight request is already valid;
3. `preflight_adapter_write(...)` returns a ready decision;
4. the decision capability is `GitHubPullRequestComment`;
5. the embedded preflight target reference matches `GitHubPullRequestCommentTarget::reference()`;
6. the decision SideEffect ID matches the request SideEffect ID;
7. the decision idempotency key matches the request idempotency key;
8. policy decision references remain present and valid through preflight execution;
9. approval references remain present when required by policy or sensitivity through preflight execution;
10. high-assurance references remain present when required through preflight execution;
11. redaction metadata is valid and non-secret-like;
12. the composed value still reports no provider-call authority.

Validation errors must use stable codes and must not include raw comment bodies, target names, paths, tokens, provider payloads, command output, parser payloads, spec contents, redaction metadata, or secret-like values.

## 8. Policy And Approval Posture

Preflight composition must preserve the invariant:

```text
declared governance is enforced or rejected
```

The helper uses the `AdapterWriteReadinessPolicy` embedded in the validated preflight request rather than hidden global state. This keeps the path explicit and testable while avoiding another parallel policy argument.

Recommended first behavior:

- local preview policy may classify fixture/dry-run comment-shaped requests as ready when policy and required references are present;
- denied policy fails closed;
- missing policy references fail closed;
- missing approval references fail closed when the request or policy requires approval;
- missing high-assurance references fail closed when the readiness policy requires high-assurance posture;
- live sandbox mode should remain rejected or unusable until a separately reviewed live-sandbox plan exists.

## 9. Fixture Adapter Implications

The fixture-backed adapter phase should depend on this composition helper.

Future fixture execution should require:

- `GitHubPullRequestCommentPreflightedWrite`, not a raw request definition;
- fixture mode or dry-run mode only;
- no provider credentials;
- no provider calls;
- no SideEffect lifecycle transition unless separately implemented;
- no event append unless separately implemented;
- no report artifact write unless separately implemented.

This prevents a fixture adapter from becoming the accidental authority boundary.

## 10. Privacy And Redaction

Preflight composition must preserve existing request and preflight redaction behavior:

- no raw provider payloads;
- no raw PR bodies, diffs, logs, command output, parser payloads, or spec contents;
- no credentials, tokens, authorization headers, private keys, or environment variable values;
- bounded summaries only;
- redaction metadata validated at construction;
- Debug output redacts target owner/repository, comment body, summary, SideEffect ID, idempotency key, preflight details, and decision details;
- serialization, if implemented, must be considered sensitive because valid bounded comment text may be present.

If a composed value is serializable, docs and tests must treat serialization as an internal/sensitive model surface, not user-facing output.

## 11. Test Plan

Future implementation tests should cover:

- valid GitHub PR comment request composes with ready preflight decision;
- preflight helper is actually called;
- denied policy fails composition;
- unsupported capability fails composition;
- mismatched target fails composition;
- mismatched SideEffect ID fails composition;
- mismatched idempotency key fails composition;
- missing policy references fail composition;
- missing approval reference fails composition when required;
- missing high-assurance reference fails composition when required;
- secret-like comment body remains rejected before composition;
- secret-like preflight summary/redaction remains rejected before composition;
- composed Debug output does not leak target, comment body, summary, SideEffect ID, idempotency key, preflight details, or decision details;
- no provider call authority is exposed;
- no workflow event append authority is exposed;
- no SideEffect lifecycle transition authority is exposed;
- existing `write_adapter_preflight` tests still pass;
- existing `provider_write` tests still pass;
- workspace docs check passes.

## 12. Implemented Sequence

1. Added a focused preflight composition helper and composed value type.
2. Used existing constructors and `preflight_adapter_write(...)`.
3. Added focused Rust tests for success, policy/approval failures, live-sandbox rejection, no-execution posture, and redaction.
4. Updated write-readiness and GitHub future docs.
5. Created an implementation report.
6. Review remains required before fixture-backed adapter work.

Do not combine this with fixture adapter execution or live provider mutation.

## 13. Documentation Updates

Implementation updates:

- `docs/implementation-plans/write-adapter-readiness-plan.md`;
- `docs/implementation-plans/first-provider-write-candidate-plan.md`;
- `docs/integrations/github-future.md`;
- `ROADMAP.md`;
- a new phase report under `docs/concepts/`.

Docs continue to say:

- provider writes are not implemented;
- fixture-backed adapter execution is not implemented unless separately added;
- live sandbox writes are not implemented;
- runtime side-effect execution is not implemented;
- CLI write commands are not implemented;
- schemas and examples are not updated;
- hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes are not implemented.

## 14. Remaining Open Questions

- Should the composed value implement serde, or should it remain an in-memory-only internal type until fixture execution is reviewed?
- Should fixture execution require a composed preflighted value at the type level?
- Should a future live-sandbox plan allow composition for `LiveSandbox`, or keep live mode rejected until immediately before provider invocation?

## 15. Final Recommendation

Next phase: fixture-backed GitHub PR comment adapter implementation, fixture-only, after planning review.

The project must still not build provider writes, live sandbox calls, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes in this phase.
