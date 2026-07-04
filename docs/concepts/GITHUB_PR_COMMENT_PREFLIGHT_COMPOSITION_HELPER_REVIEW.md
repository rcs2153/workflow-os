# GitHub PR Comment Preflight Composition Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The GitHub PR comment preflight composition helper implements the intended model/helper-only bridge between the validated GitHub PR comment request model and the adapter-neutral write preflight helper. It executes `preflight_adapter_write(...)`, rejects live sandbox mode, preserves the no-execution boundary, and adds focused tests.

It is safe to proceed to fixture-backed GitHub PR comment adapter planning after this review, with no live provider mutation.

## 2. Scope Verification

The phase stayed within approved model/helper-only scope.

No accidental implementation was found for:

- GitHub provider calls;
- pull request comment creation;
- fixture-backed adapter execution;
- live sandbox writes;
- runtime side-effect execution;
- SideEffect attempted/completed/failed lifecycle transitions;
- workflow event appends;
- audit event emission;
- report artifact writes;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credentials;
- OAuth app behavior or webhook ingestion;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

The composed value continues to expose false no-authority accessors for provider calls, workflow event appends, SideEffect lifecycle transitions, and report artifact writes.

## 3. Helper API Assessment

The implemented API is appropriately small:

```text
GitHubPullRequestCommentPreflightedWrite::new(
    request: GitHubPullRequestCommentWriteRequest,
) -> Result<GitHubPullRequestCommentPreflightedWrite, WorkflowOsError>
```

The type owns:

- a validated `GitHubPullRequestCommentWriteRequest`;
- an executed `AdapterWritePreflightDecision`.

The API is explicit, in-memory, and testable. It does not read hidden global state, runtime config, credentials, or provider state.

The implementation uses the readiness policy embedded in the already validated preflight request. That is acceptable for this phase because the preflight request is already part of the validated GitHub PR comment request boundary and avoids a second parallel policy parameter.

## 4. Preflight Execution Assessment

The helper calls `preflight_adapter_write(request.preflight())` instead of trusting the embedded preflight request shape.

That closes the intended gap from the prior phase:

- request construction still validates preflight alignment;
- composition now executes the adapter-neutral preflight helper;
- denied policy and missing required approval fail through the existing preflight helper;
- the composed value carries the executed decision for future fixture paths.

This is the right sequencing before any fixture-backed adapter work.

## 5. Validation Assessment

Validation ensures:

- the request is validated before composition;
- the embedded preflight request is validated by the preflight helper;
- `LiveSandbox` mode is rejected before provider work;
- denied policy fails closed;
- missing required approval fails closed;
- returned decision capability is `GitHubPullRequestComment`;
- returned decision SideEffect ID matches the request;
- returned decision idempotency key matches the request;
- the returned decision does not authorize provider calls, event appends, SideEffect lifecycle transitions, or report artifact writes.

Validation errors use stable codes and do not echo raw comment bodies, target values, tokens, provider payloads, command output, parser payloads, spec contents, redaction metadata, or secret-like values.

## 6. No-Execution Boundary Assessment

The helper correctly preserves the no-execution boundary:

- no GitHub call;
- no fixture adapter invocation;
- no provider credentials;
- no workflow events;
- no audit events;
- no SideEffect lifecycle transition;
- no report artifact write;
- no CLI output.

The explicit accessors return false for:

- `provider_call_allowed()`;
- `workflow_event_append_allowed()`;
- `side_effect_lifecycle_transition_allowed()`;
- `report_artifact_write_allowed()`.

This keeps the composed value useful as a future fixture prerequisite without making it an execution authority.

## 7. Privacy And Redaction Assessment

The implementation preserves the existing privacy posture:

- no raw provider payloads are introduced;
- no raw PR body, diff, log, command output, parser payload, or spec content is introduced;
- no credentials, tokens, authorization headers, private keys, or environment values are introduced;
- `Debug` redacts the full request and uses the existing redaction-safe preflight decision debug output;
- tests verify that debug output does not leak target owner/repository, comment body, run ID, SideEffect ID, or idempotency key.

The composed value intentionally does not implement serde in this phase, which is a good conservative choice. Any future serialization should be treated as sensitive because valid bounded comment text may be present inside the owned request.

## 8. Test Quality Assessment

Tests cover the important behavior:

- valid request composes with executed preflight;
- denied policy fails composition;
- missing required approval fails composition;
- live sandbox mode is rejected before provider work;
- debug output is redaction-safe;
- no provider-call, workflow-event-append, SideEffect-lifecycle-transition, or report-artifact-write authority is exposed;
- existing provider write request/response behavior still passes;
- existing write preflight tests still pass via workspace validation.

Non-blocking test follow-ups:

- add an explicit high-assurance-required failure test for composition;
- add a direct missing-policy-reference composition test;
- add a direct unsupported-readiness-policy composition test;
- add fixture-planning tests that require `GitHubPullRequestCommentPreflightedWrite` rather than raw request definitions once fixture work begins.

The current tests are sufficient for this phase.

## 9. Documentation Review

Docs accurately state:

- preflight composition is implemented as model/helper-only;
- provider writes are not implemented;
- fixture-backed adapter execution is not implemented;
- live sandbox writes are not implemented;
- runtime side-effect execution is not implemented;
- CLI write commands are not implemented;
- schemas and examples are not updated;
- hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes are not implemented.

A small wording correction was made during review to clarify that the listed primitives are baseline primitives, not all newly added by this phase.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add high-assurance-required composition coverage.
- Add missing-policy-reference composition coverage.
- Add unsupported-readiness-policy composition coverage.
- During fixture planning, require fixture execution to accept a preflighted value rather than raw request definitions.
- Keep live sandbox mode rejected until a separate live-sandbox plan is reviewed.

## 12. Recommended Next Phase

Recommended next phase: fixture-backed GitHub PR comment adapter planning.

The planning phase should remain explicit, local, fixture-only, and no-provider-call. It should specify that fixture execution requires `GitHubPullRequestCommentPreflightedWrite` and must not introduce live provider mutation, CLI behavior, schemas, examples, runtime SideEffect lifecycle transitions, workflow events, report artifacts, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 13. Validation

Implementation phase validation recorded in `docs/concepts/GITHUB_PR_COMMENT_PREFLIGHT_COMPOSITION_HELPER_REPORT.md`:

- `cargo test -p workflow-core --test provider_write` - passed
- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

Review phase validation:

- `npm run check:docs` - passed
- `git diff --check` - passed

## 14. Dogfood Governance Summary

This review phase is governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1783200226018723000-2`
- Approval ID: `approval/run-1783200226018723000-2/review-scope-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Terminal: true
- Events total: 39
- Approvals: 1
- Retries: 0
- Escalations: 0
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: repository inspection, maintainer review drafting, tiny documentation correction, validation commands, and phase-close inspection are performed by the agent outside kernel execution. The kernel coordinates governance only.
