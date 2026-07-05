# GitHub PR Comment Live Provider Call Plan

Status: Accepted plan; first provider-call trait/input model implemented; injected-provider orchestration helper implemented; concrete injected-transport provider client implemented in [GitHub PR Comment Provider Client/Auth Loading Implementation Report](../concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REPORT.md) and accepted with non-blocking follow-ups in [GitHub PR Comment Provider Client/Auth Loading Implementation Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REVIEW.md). Provider write reconciliation planning is documented in [GitHub PR Comment Provider Write Reconciliation Plan](github-pr-comment-provider-write-reconciliation-plan.md). This follows the accepted [Write-Adapter No-Provider Outcome Orchestration Review](../concepts/WRITE_ADAPTER_NO_PROVIDER_OUTCOME_ORCHESTRATION_REVIEW.md) and [GitHub PR Comment Live Provider Call Plan Review](../concepts/GITHUB_PR_COMMENT_LIVE_PROVIDER_CALL_PLAN_REVIEW.md). It defines the controlled, opt-in live provider-call boundary for the GitHub pull request comment write-adapter candidate. The current implementation adds the injected provider trait, explicit auth wrapper, validated provider-call request model, a narrow helper that invokes only a caller-supplied provider trait before transitioning the attempted `SideEffectRecord` to completed or failed from a classified provider response, and a concrete client that uses only caller-supplied auth plus injected transport. It does not implement hidden auth loading, executor-integrated writes, automatic event append, report artifact writes, CLI mutation behavior, workflow schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has a no-provider GitHub PR comment write lane that can validate write readiness, persist a proposed `SideEffectRecord`, validate approval linkage, transition to attempted, and close the attempted record as completed or failed from explicit local fixture/dry-run outcomes.

The next question is not how to make writes automatic. The next question is how a future implementation should perform one live provider call without breaking the governance boundary.

This plan defined the live provider-call boundary. The implemented slices now support an injected-provider helper and a concrete injected-transport GitHub PR comment provider client. Hidden auth loading, executor integration, CLI mutation behavior, schemas, examples, hosted behavior, and production credential management remain future work.

## 2. Goals

- Define the smallest safe live GitHub PR comment provider-call boundary.
- Preserve explicit opt-in behavior.
- Preserve provider writes denied by default.
- Require preflight, proposed record, approval linkage, attempted lifecycle state, and idempotency before provider call.
- Classify provider responses into completed or failed outcomes without copying raw payloads.
- Keep workflow event append and report artifact writes as separate explicit caller boundaries.
- Keep errors stable and non-leaking.
- Prepare a narrow future implementation prompt.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic provider writes;
- default executor write behavior;
- automatic side-effect execution;
- automatic workflow event append;
- automatic report artifact writing;
- CLI mutation commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- auth material loading in this phase;
- production credential management;
- OAuth app behavior or webhook ingestion;
- RBAC, IdP, enterprise stewardship, quorum approval, or revocation enforcement;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed or staged for review:

- write adapter preflight model/helper;
- GitHub PR comment request/response models;
- fixture/dry-run validation without provider calls;
- proposed `SideEffectRecord` composition and persistence;
- proposed SideEffect event construction and explicit executor append;
- approval-side-effect linkage;
- store-backed attempted/completed/failed lifecycle transitions;
- executor attempted/completed/failed lifecycle event append;
- no-provider attempted orchestration helper;
- no-provider completed/failed outcome orchestration helper.

Still missing:

- live provider-call helper;
- provider success/failure lifecycle orchestration boundary;
- provider idempotency behavior beyond local idempotency binding;
- live-write response redaction policy;
- explicit live sandbox test strategy;
- review of whether the first live call should be disabled behind compile/runtime/test-only gates.

Implemented as the first model-only slice:

- explicit auth input wrapper for caller-supplied provider auth;
- validated provider-call request model;
- injected provider-call trait returning the existing validated response model;
- focused tests for pre-call gate failures and redaction-safe Debug behavior.

## 5. Required Pre-Call Gates

A future live provider-call helper must require all of the following before it calls GitHub:

1. Existing stored `SideEffectRecord`.
2. Stored record is `Attempted`.
3. Capability is `GitHubWrite`.
4. Target is a GitHub pull request comment adapter target.
5. Policy references are present and valid.
6. Approval references are present when authority requires approval.
7. Approval linkage is valid when required.
8. Idempotency key is present and bound to the side-effect.
9. Write mode explicitly permits live sandbox/provider call.
10. Auth input is supplied explicitly by the caller.
11. Auth input passes redaction-safe validation.
12. Provider call is explicitly enabled for the call site.

If any gate is missing, the helper must fail before provider invocation.

## 6. Auth Posture

The first live provider-call helper should accept explicit auth material through a narrow caller-provided boundary. It should not read environment variables, config files, credential stores, git remotes, GitHub CLI state, keychains, or hidden global state.

Auth input rules:

- token value must not be `Debug` formatted;
- token value must not be serialized;
- token value must not appear in errors;
- token value must not be stored in `SideEffectRecord`, workflow events, audit records, adapter telemetry, WorkReport, report artifacts, or logs;
- auth scope summary may be represented as bounded metadata if supplied separately;
- missing auth fails before provider invocation.

Production auth loading remains future work.

## 7. Provider Call Boundary

The future helper should be explicit and narrow, likely shaped around:

- an attempted `SideEffectId`;
- `SideEffectRecordStore`;
- explicit auth input;
- explicit provider client or injected provider call trait;
- expected request identity from the validated write request;
- idempotency metadata;
- transition timestamp;
- optional bounded transition summary;
- references/evidence counts.

The helper must not construct a `LocalStateBackend`, discover hidden runtime state, append workflow events, write report artifacts, emit CLI output, or mutate `WorkflowRun`.

## 8. Provider Client Boundary

The first implementation should prefer an injected provider-call trait so tests can prove behavior without live credentials.

The trait should be narrow:

- one method for creating a pull request comment;
- input contains bounded target and comment body from the validated request model;
- output contains only a stable provider comment reference or classified provider failure;
- raw provider response body is not exposed;
- raw HTTP request/response payloads are not stored.

This keeps provider-call logic testable and prevents the orchestration helper from becoming an HTTP client kitchen sink.

## 9. Idempotency Policy

Local idempotency binding is required before call.

Future provider-call planning should decide whether GitHub PR comment creation can use provider-native idempotency. If provider-native idempotency is unavailable, Workflow OS must rely on local idempotency state and must document duplicate-call limitations.

The first implementation should:

- reject missing idempotency binding;
- not retry provider writes automatically;
- not silently create a second comment when a completed outcome already exists;
- classify duplicate/prior-completed cases through explicit local state rather than provider guessing.

## 10. Success Classification

Provider success should transition the attempted side-effect to `Completed` only when the provider returns a bounded stable provider comment reference.

Rules:

- provider outcome reference kind should be `Outcome`;
- provider reference must be bounded and non-secret-like;
- provider reference should use a distinct provider prefix, not the no-provider `fixture/`, `dry-run/`, or `local/` prefixes;
- raw provider response must not be copied;
- returned transition result remains reference-only;
- workflow event append remains explicit and separate.

## 11. Failure Classification

Provider failure should transition the attempted side-effect to `Failed` only when the failure is classified into a stable reason code and optional bounded failure reference.

Initial provider failure vocabulary should include:

- `provider.auth_failed`;
- `provider.permission_denied`;
- `provider.not_found`;
- `provider.rate_limited`;
- `provider.validation_failed`;
- `provider.network_failed`;
- `provider.timeout`;
- `provider.unknown_failed`.

Rules:

- raw provider error payload is not copied;
- HTTP headers are not copied;
- token/auth values are never copied;
- stack traces are not copied;
- unclassified errors fail closed without provider payload leakage.

## 12. Workflow Event Boundary

The live provider-call helper may return completed/failed lifecycle transition event payloads, but it must not append them.

Appending lifecycle events remains the responsibility of the explicit executor append helper or a later reviewed executor integration path.

## 13. Report And Artifact Boundary

The helper must not write WorkReports or report artifacts.

It may return:

- side-effect ID;
- lifecycle state;
- outcome reference count;
- evidence reference count;
- stable provider outcome/failure reference;
- report citation obligations.

Report artifact writing remains a separate explicit path with SideEffect referential integrity and approval-linkage gates.

## 14. Runtime Semantics

Provider-call failure must not silently rewrite workflow pass/fail semantics unless a separately reviewed executor integration path explicitly defines that behavior.

For the first helper:

- provider call success/failure updates only the `SideEffectRecord` lifecycle through reviewed store transition helpers;
- no workflow run mutation occurs;
- no event is appended;
- no audit record is emitted;
- no report artifact is written;
- caller decides how to reflect the result in a workflow run through existing explicit boundaries.

## 15. Privacy And Redaction

The live provider-call implementation must not store or output:

- raw provider payloads;
- raw GitHub responses;
- raw HTTP headers;
- raw command output;
- raw CI logs;
- raw file contents;
- raw spec contents;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

`Debug`, serialization, errors, audit candidates, and report candidates must remain redaction-safe.

## 16. Test Plan

Future implementation tests should cover:

- pre-call rejection when no attempted record exists;
- pre-call rejection when record is not attempted;
- pre-call rejection when approval linkage is required but invalid;
- pre-call rejection when auth input is missing;
- pre-call rejection when live call is not explicitly enabled;
- successful provider-call path uses injected provider client;
- provider success transitions to completed with provider outcome reference;
- provider failure transitions to failed with stable reason code;
- provider-shaped outcome references are distinct from no-provider references;
- no raw provider payload is copied;
- token/auth values do not appear in `Debug`, serialization, or errors;
- helper does not append workflow events;
- helper does not mutate `WorkflowRun`;
- helper does not write report artifacts;
- helper does not create CLI output;
- retries do not happen automatically;
- existing no-provider tests still pass;
- workspace validation remains green.

## 17. Proposed Implementation Sequence

Recommended small phases:

1. Live provider-call plan review.
2. Provider-call trait/input model only, no network implementation.
3. Injected-client orchestration helper with success/failure classification, no live network tests.
4. Maintainer review.
5. Provider write reconciliation plan review.
6. Model/helper-only reconciliation candidate implementation.
7. Optional live sandbox smoke planning with explicit credentials and test repository constraints.
8. Live sandbox smoke helper, opt-in only.
9. Executor integration planning only after the helper is reviewed.

## 18. Open Questions

- Should the first provider-call helper be compiled into normal builds or test-only until reviewed again?
- What exact provider reference prefix should distinguish live provider outcomes?
- Should local idempotency prevent duplicate comments strongly enough without provider-native idempotency?
- Should provider auth be represented by a trait object, token wrapper, or caller-supplied client?
- Should provider failures ever avoid lifecycle transition if classification fails?
- Should provider success require immediate report artifact citation, or only return citation obligations?
- When should live sandbox smoke tests be allowed in CI, if ever?

## 19. Final Recommendation

Proceed next to **injected-provider orchestration helper review**.

The implemented helper remains intentionally narrow: caller-supplied provider trait only, explicit auth input, explicit lifecycle transition, no concrete network client, no hidden auth loading, no default executor writes, no CLI commands, no schemas, no examples, no hosted behavior, no automatic event append, no automatic report artifacts, no reasoning lineage, no recursive agents, no agent swarms, no Level 3/4 autonomy, and no release posture changes.
