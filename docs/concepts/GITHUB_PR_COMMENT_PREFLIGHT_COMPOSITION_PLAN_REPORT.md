# GitHub PR Comment Preflight Composition Plan Report

## 1. Executive Summary

GitHub PR comment preflight composition planning is complete.

The new plan defines the next narrow bridge between the existing adapter-neutral write preflight helper and the model-only GitHub PR comment write request boundary. The future implementation should execute `preflight_adapter_write(...)` against a validated `GitHubPullRequestCommentWriteRequest` and return a composed, redaction-safe, no-execution value before any fixture-backed adapter path is implemented.

This phase does not implement provider writes, fixture-backed adapter execution, live sandbox calls, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Scope Completed

Completed:

- created [GitHub PR Comment Preflight Composition Plan](../implementation-plans/github-pr-comment-preflight-composition-plan.md);
- linked the plan from `ROADMAP.md`;
- updated [Write-Capable Adapter Readiness Plan](../implementation-plans/write-adapter-readiness-plan.md);
- updated [First Provider Write Candidate Plan](../implementation-plans/first-provider-write-candidate-plan.md);
- updated [GitHub Adapter Posture](../integrations/github-future.md).

## 3. Scope Explicitly Not Completed

Not implemented:

- Rust preflight composition helper;
- fixture-backed GitHub PR comment adapter execution;
- GitHub provider calls;
- pull request comment creation;
- runtime side-effect execution;
- SideEffect lifecycle transitions;
- workflow event or audit event appends;
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

## 4. Planning Summary

The plan establishes that the next implementation should:

- accept a validated `GitHubPullRequestCommentWriteRequest`;
- accept an explicit `AdapterWriteReadinessPolicy`;
- execute `preflight_adapter_write(...)`;
- require a ready preflight decision;
- verify decision/request alignment for capability, target, SideEffect ID, idempotency key, policy/approval posture, sensitivity, and redaction;
- return a composed value that still forbids provider calls, workflow event appends, SideEffect lifecycle transitions, and report artifact writes.

## 5. Validation Boundary Summary

The future helper should fail closed unless:

- the request is valid;
- the preflight request is valid;
- preflight execution returns ready posture;
- the decision matches GitHub PR comment capability;
- the decision target matches the request target reference;
- the decision SideEffect ID matches the request;
- the decision idempotency key matches the request;
- required policy, approval, and high-assurance references are present;
- redaction metadata is valid;
- no execution authority is exposed.

Errors must use stable non-leaking codes.

## 6. Redaction And Privacy Summary

The plan preserves the existing privacy posture:

- no raw provider payloads;
- no raw PR bodies, diffs, logs, command output, parser payloads, or spec contents;
- no credentials, tokens, authorization headers, private keys, or environment variable values;
- bounded summaries only;
- redaction metadata validated at construction;
- Debug output must redact target, body, summary, SideEffect ID, idempotency key, preflight details, and decision details.

The plan also calls out that any future serialized composed value should be treated as sensitive because valid bounded comment text may be present.

## 7. Test Coverage Planned

The future implementation should test:

- valid composition;
- actual preflight helper execution;
- denied policy;
- unsupported capability;
- target mismatch;
- SideEffect mismatch;
- idempotency mismatch;
- missing policy references;
- missing approval references when required;
- missing high-assurance references when required;
- secret-like input rejection;
- Debug non-leakage;
- no provider-call authority;
- no event-append authority;
- no SideEffect lifecycle-transition authority;
- existing `write_adapter_preflight` and `provider_write` regressions.

## 8. Commands Run And Results

Validation commands:

- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 9. Dogfood Governance Summary

This planning phase was governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/d`
- Run ID: `run-1783198825361305000-2`
- Approval ID: `approval/run-1783198825361305000-2/planning-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Terminal: true
- Events total: 39
- Approvals: 1
- Retries: 0
- Escalations: 0
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: repository inspection, planning document creation, roadmap/posture doc updates, validation commands, and phase-close inspection are performed by the agent outside kernel execution. The kernel coordinates governance only.

## 10. Remaining Known Limitations

- Preflight composition is planned but not implemented.
- The existing GitHub PR comment request validates embedded preflight alignment but does not execute preflight.
- Fixture-backed adapter execution remains future work.
- Provider writes remain unsupported.
- Persisted SideEffect linkage before live writes remains future work.

## 11. Recommended Next Phase

Recommended next phase: GitHub PR comment preflight composition helper implementation, model/helper only.

That implementation should remain no-provider-call, no-fixture-execution, no-CLI, no-schema, no-example, no-hosted-behavior, and no-release-posture-change.
