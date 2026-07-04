# GitHub PR Comment Fixture Adapter Plan

Status: Implemented as fixture-only helper and accepted with non-blocking follow-ups. This plan follows the accepted GitHub PR comment preflight composition helper review and defines the fixture-backed adapter step. The implementation report is [GitHub PR Comment Fixture Adapter Helper Report](../concepts/GITHUB_PR_COMMENT_FIXTURE_ADAPTER_HELPER_REPORT.md), and the review recommended proposed `SideEffectRecord` composition planning before live sandbox write planning. That follow-up is documented in [GitHub PR Comment Proposed SideEffectRecord Composition Plan](github-pr-comment-side-effect-record-composition-plan.md), and the first in-memory composition helper is implemented in [GitHub PR Comment Proposed SideEffectRecord Composition Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_COMPOSITION_HELPER_REPORT.md). It does not implement live GitHub provider calls, provider mutation, proposed record persistence, runtime side-effect execution, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has a validated GitHub pull request comment request model and a preflight composition helper:

```text
GitHubPullRequestCommentPreflightedWrite::new(request)
```

That helper executes the adapter-neutral write preflight and returns a composed value that still has no provider-call authority.

The next safe step is a fixture-backed adapter plan. The future implementation should accept only a `GitHubPullRequestCommentPreflightedWrite`, validate deterministic fixture inputs, and return a bounded `GitHubPullRequestCommentWriteResponse` for fixture or dry-run validation. It must not call GitHub or mutate provider state.

The fixture-only helper is implemented. It does not implement live provider mutation.

## 2. Goals

- Define the fixture-only adapter boundary for GitHub pull request comments.
- Require `GitHubPullRequestCommentPreflightedWrite` rather than raw request models.
- Preserve explicit preflight-before-execution sequencing.
- Prove request/response wiring without provider credentials.
- Return deterministic fixture or dry-run responses.
- Keep fixture output bounded, redaction-safe, and non-authoritative.
- Preserve no provider call, no workflow event append, no SideEffect lifecycle transition, and no report artifact write posture.
- Prepare a small implementation prompt for the next phase.

## 3. Non-Goals

This plan does not authorize:

- implementation in this phase;
- GitHub provider calls;
- live sandbox writes;
- pull request comment creation;
- provider credentials;
- OAuth app behavior or webhook ingestion;
- runtime side-effect execution;
- SideEffect attempted/completed/failed lifecycle transitions;
- workflow event appends;
- audit event emission;
- report artifact writes;
- automatic executor integration;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented before this plan:

- `AdapterWritePreflightRequest`;
- `AdapterWritePreflightDecision`;
- `AdapterWriteReadinessPolicy`;
- `preflight_adapter_write(...)`;
- `GitHubPullRequestCommentWriteRequest`;
- `GitHubPullRequestCommentWriteResponse`;
- `github_pr_comment_preflight_definition(...)`;
- `GitHubPullRequestCommentPreflightedWrite`;
- tests proving preflight execution, live sandbox rejection, no-authority posture, and redaction-safe debug output.

Not implemented:

- fixture-backed GitHub PR comment adapter execution;
- live provider mutation;
- persisted SideEffect lifecycle transitions;
- workflow event or audit projection for write attempts;
- report artifact disclosure for write attempts;
- CLI or schema surfaces.

## 5. Fixture Adapter Boundary

The future fixture helper should be explicit and in-memory:

```text
validate_github_pr_comment_fixture_write(
    preflighted: GitHubPullRequestCommentPreflightedWrite,
    fixture: GitHubPullRequestCommentFixture,
) -> Result<GitHubPullRequestCommentWriteResponse, WorkflowOsError>
```

Exact names should follow repository conventions during implementation.

The helper should:

- accept a preflighted write value, not a raw request;
- accept explicit fixture inputs;
- verify fixture mode or dry-run mode;
- verify the fixture target matches the preflighted request target;
- verify the fixture SideEffect ID and idempotency key match the preflighted request;
- return a `GitHubPullRequestCommentWriteResponse` with `FixtureValidated` or `DryRunValidated`;
- preserve the current no-execution posture.

The helper must not:

- call GitHub;
- read credentials;
- read hidden runtime configuration;
- append workflow events;
- emit audit events;
- transition SideEffect lifecycle state;
- write report artifacts;
- write files;
- expose CLI output;
- fabricate provider IDs.

## 6. Required Fixture Inputs

The fixture input should be minimal and bounded:

- expected repository owner/name or stable target reference;
- expected pull request number;
- expected `SideEffectId`;
- expected idempotency key;
- fixture provider reference for a simulated comment, if needed;
- fixture mode: fixture or dry-run;
- bounded fixture summary;
- sensitivity and redaction metadata if the response needs them.

Fixture input must not include:

- GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw PR bodies;
- raw diffs;
- raw CI logs;
- raw command output;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded prompt text;
- secret-like values.

## 7. Response Policy

The fixture helper should return a validated `GitHubPullRequestCommentWriteResponse`.

Allowed fixture outcomes:

- `FixtureValidated`;
- `DryRunValidated`.

Rejected fixture outcomes:

- `ProviderSucceeded`;
- `ProviderFailed`, unless a separate provider-failure fixture plan explicitly needs it without provider calls.

The response must be bounded and redaction-safe. It must not claim that a provider comment was created. Any simulated provider reference must be clearly fixture-only and must not be usable as evidence of external mutation.

## 8. Preflight And Governance Requirements

Fixture execution must require:

- ready preflight decision from `GitHubPullRequestCommentPreflightedWrite`;
- capability `GitHubPullRequestComment`;
- SideEffect ID alignment;
- idempotency key alignment;
- policy posture preserved from preflight;
- approval/high-assurance posture preserved from preflight when required;
- no provider-call authority;
- no event-append authority;
- no SideEffect lifecycle-transition authority;
- no report-artifact-write authority.

The fixture helper must fail closed if the preflighted value or fixture input is inconsistent.

## 9. Error Handling

Errors must use stable, non-leaking codes.

Recommended codes:

- `github_pr_comment_fixture.mode.unsupported`;
- `github_pr_comment_fixture.target.mismatch`;
- `github_pr_comment_fixture.side_effect.mismatch`;
- `github_pr_comment_fixture.idempotency.mismatch`;
- `github_pr_comment_fixture.response.invalid`;
- `github_pr_comment_fixture.provider_call_forbidden`.

Errors must not include raw owner/repository names, pull request bodies, comment text, fixture payloads, paths, tokens, command output, provider payloads, redaction metadata, or secret-like values.

## 10. Privacy And Redaction

Rules:

- no raw provider payloads;
- no raw PR descriptions;
- no raw diffs;
- no raw logs;
- no command output;
- no credentials, tokens, authorization headers, private keys, or environment values;
- summaries must be bounded;
- fixture IDs must be bounded and non-secret-like;
- Debug output must redact preflighted request internals, fixture target details, comment text, SideEffect ID, and idempotency key;
- serde should be avoided unless implementation needs an explicit fixture file model.

If serde is added later, fixture serialization must be treated as internal test data and must fail closed or redact secret-like values.

## 11. Test Plan

Future implementation tests should cover:

- valid preflighted write produces `FixtureValidated` response;
- valid dry-run input produces `DryRunValidated` response;
- raw `GitHubPullRequestCommentWriteRequest` is not accepted by the fixture helper API;
- live sandbox mode remains rejected before fixture execution;
- target mismatch fails closed;
- SideEffect ID mismatch fails closed;
- idempotency key mismatch fails closed;
- denied policy cannot reach fixture response construction through the preflighted path;
- missing required approval cannot reach fixture response construction through the preflighted path;
- provider-success outcome is rejected in fixture mode;
- no provider call authority is exposed;
- no workflow event append occurs;
- no SideEffect lifecycle transition occurs;
- no report artifact write occurs;
- no files are written;
- no CLI output is produced;
- raw provider/spec/command/parser payload markers are rejected or absent;
- secret-like fixture summaries are rejected or redacted;
- Debug output does not leak target, comment text, SideEffect ID, idempotency key, fixture payloads, tokens, or secret-like values;
- existing provider write tests still pass;
- existing write preflight tests still pass;
- docs check passes.

## 12. Implemented Sequence

1. Implemented a small fixture input model.
2. Implemented a fixture-only helper that accepts `GitHubPullRequestCommentPreflightedWrite`.
3. Returned validated `GitHubPullRequestCommentWriteResponse` values only for fixture/dry-run outcomes.
4. Added focused tests for alignment, no-execution posture, and redaction.
5. Updated docs and created an implementation report.
6. Maintainer review remains required before any live sandbox planning.

Do not combine this with live provider mutation.

## 13. Deferred Work

Deferred:

- proposed `SideEffectRecord` persistence;
- SideEffect attempted/completed/failed lifecycle transitions;
- workflow event or audit projection for write attempts;
- report artifact write disclosure for write attempts;
- live sandbox GitHub comment smoke;
- provider credential handling;
- CLI write commands;
- workflow-declared write support;
- duplicate comment detection;
- GitHub branch, PR, merge, status, label, or check writes;
- Jira write candidates;
- examples;
- schemas;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 14. Open Questions

- Should fixture responses carry a synthetic provider reference, or should fixture validation avoid provider-shaped references entirely?
- Should fixture inputs live only in Rust tests, or should fixture file models be planned later?
- Should provider failure response fixtures be supported before any live provider smoke?
- Should persisted proposed `SideEffectRecord` composition come before or after the fixture helper implementation?
- Should the fixture helper remain internal until live sandbox planning is accepted?

## 15. Final Recommendation

Proceed next to GitHub PR comment proposed `SideEffectRecord` composition plan review.

Any follow-on planning must continue to forbid provider calls, live writes, SideEffect lifecycle transitions, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes unless separately approved and reviewed.
