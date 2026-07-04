# GitHub PR Comment Fixture Adapter Plan Report

## 1. Executive Summary

The fixture-backed GitHub pull request comment adapter phase is planned.

The plan defines a fixture-only, no-provider-call boundary that should accept a `GitHubPullRequestCommentPreflightedWrite`, validate deterministic fixture inputs, and return a bounded `GitHubPullRequestCommentWriteResponse` for fixture or dry-run validation. It keeps provider mutation, live sandbox writes, runtime side-effect execution, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes out of scope.

## 2. Scope Completed

Completed:

- created `docs/implementation-plans/github-pr-comment-fixture-adapter-plan.md`;
- defined the future fixture adapter boundary;
- required fixture execution to accept `GitHubPullRequestCommentPreflightedWrite`;
- defined fixture input posture;
- defined fixture response posture;
- documented preflight and governance requirements;
- documented error handling, privacy, and redaction requirements;
- documented a future implementation sequence and test plan;
- updated roadmap and write-readiness docs to link this plan.

## 3. Scope Explicitly Not Completed

Not implemented:

- fixture-backed adapter execution;
- GitHub provider calls;
- live sandbox writes;
- pull request comment creation;
- provider credentials;
- runtime side-effect execution;
- SideEffect attempted/completed/failed lifecycle transitions;
- workflow event appends;
- audit event emission;
- report artifact writes;
- automatic executor integration;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Planning Boundary Summary

The plan recommends a small future helper equivalent to:

```text
validate_github_pr_comment_fixture_write(
    preflighted: GitHubPullRequestCommentPreflightedWrite,
    fixture: GitHubPullRequestCommentFixture,
) -> Result<GitHubPullRequestCommentWriteResponse, WorkflowOsError>
```

The exact names remain implementation details for the next phase.

The future helper should return only `FixtureValidated` or `DryRunValidated` responses and must not claim that a provider comment was created.

## 5. Governance Summary

The plan requires fixture execution to preserve:

- ready preflight decision;
- capability alignment;
- target alignment;
- SideEffect ID alignment;
- idempotency key alignment;
- policy posture;
- approval/high-assurance posture when required;
- no provider-call authority;
- no event-append authority;
- no SideEffect lifecycle-transition authority;
- no report-artifact-write authority.

## 6. Redaction And Privacy Summary

The plan forbids:

- raw provider payloads;
- raw PR descriptions;
- raw diffs;
- raw logs;
- raw command output;
- credentials, tokens, authorization headers, private keys, or environment values;
- raw file contents;
- raw spec contents;
- unbounded prompt text;
- secret-like fixture values.

It also requires stable non-leaking errors and redaction-safe Debug behavior.

## 7. Test Coverage Plan

Future tests should cover:

- valid preflighted fixture response;
- valid dry-run response;
- raw request not accepted by fixture helper API;
- live sandbox rejection;
- target, SideEffect ID, and idempotency mismatches;
- denied policy and missing approval blocked before fixture response construction;
- provider-success outcome rejected in fixture mode;
- no provider calls, events, SideEffect lifecycle transitions, report artifacts, files, or CLI output;
- raw payload and secret-like value non-leakage;
- Debug redaction;
- existing provider write and preflight regressions.

## 8. Documentation Updates

Updated:

- `ROADMAP.md`;
- `docs/implementation-plans/write-adapter-readiness-plan.md`;
- `docs/implementation-plans/first-provider-write-candidate-plan.md`;
- `docs/implementation-plans/github-pr-comment-preflight-composition-plan.md`;
- `docs/integrations/github-future.md`.

Docs continue to state that provider writes and live sandbox writes are not implemented.

## 9. Commands Run And Results

Validation commands:

- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Dogfood Governance Summary

This planning phase is governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/d`
- Run ID: `run-1783200459323356000-2`
- Approval ID: `approval/run-1783200459323356000-2/planning-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Terminal: true
- Events total: 39
- Approvals: 1
- Retries: 0
- Escalations: 0
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: repository inspection, planning documentation, documentation updates, validation commands, and phase-close inspection are performed by the agent outside kernel execution. The kernel coordinates governance only.

## 11. Remaining Known Limitations

- Fixture adapter execution remains unimplemented.
- Provider writes remain unsupported.
- Persisted SideEffect lifecycle composition remains future work.
- Workflow event/audit projection remains future work.
- Report artifact disclosure for write attempts remains future work.
- Live sandbox planning remains future work.

## 12. Recommended Next Phase

Recommended next phase: fixture-backed GitHub PR comment adapter implementation, fixture-only.

The implementation must remain no-provider-call, no-live-write, explicit-input-only, in-memory, and preflighted-value-only.
