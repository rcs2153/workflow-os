# GitHub PR Comment Provider Write Readiness Plan

Status: Planning complete. This plan follows the accepted executor provider-candidate report artifact integration review. It defines the remaining readiness boundary before any future live GitHub pull request comment mutation may be implemented or proposed. It does not implement live GitHub provider writes, runtime side-effect execution, CLI mutation commands, workflow schema changes, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS can now model a GitHub pull request comment write candidate without writing to GitHub.

Implemented foundations include:

- adapter-neutral write preflight validation;
- a GitHub pull request comment request/response model;
- fixture-backed adapter validation with no provider calls;
- proposed `SideEffectRecord` composition and persistence;
- proposed side-effect workflow event construction;
- explicit persisted-record-to-executor-input bridging;
- report artifact citation and artifact-write gates;
- explicit executor provider-candidate report artifact inputs.

That stack proves write intent can be governed, cited, audited, and reported before any external mutation occurs.

The next question is what must be true before Workflow OS is allowed to cross from provider-candidate validation into a live GitHub pull request comment call. This plan answers that question. It is a readiness plan only.

## 2. Goals

- Define the final readiness gates before any live GitHub PR comment provider call.
- Preserve Workflow OS as a governed runtime, not a GitHub automation bot.
- Require explicit opt-in for any future live mutation path.
- Require policy, SideEffect, approval, idempotency, audit, report, and redaction posture before provider invocation.
- Preserve default write-denied behavior.
- Keep the first future live path sandbox-only and low blast radius.
- Define failure semantics for provider write attempts without creating ambiguous workflow state.
- Define required tests and review gates for any future implementation prompt.
- Keep provider write readiness separate from provider write implementation.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- live GitHub PR comment creation;
- any GitHub provider mutation;
- runtime side-effect execution;
- automatic side-effect attempted/completed/failed transitions;
- automatic executor writes;
- CLI mutation commands or flags;
- workflow schema changes;
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

Implemented no-write foundations:

- `preflight_adapter_write(...)` validates write-readiness posture without provider calls.
- GitHub pull request comment request/response models validate bounded target, body, policy, approval, SideEffect, idempotency, and redaction posture.
- Fixture-backed GitHub PR comment adapter validation proves request/response behavior without live credentials.
- Proposed GitHub PR comment `SideEffectRecord` composition creates governed write intent without attempting execution.
- Proposed record persistence writes through explicit `SideEffectRecordStore`.
- Proposed side-effect workflow event helpers create accepted proposed-event proof from persisted records.
- The explicit executor append input path can append proposed SideEffect events from persisted records.
- WorkReport/report artifact citation helpers validate that reports cite the persisted record and accepted proposed event.
- Explicit artifact-capable executor inputs can validate GitHub PR comment provider-candidate context before report artifact write.

Still not implemented:

- live GitHub PR comment provider call;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` runtime transitions for provider writes;
- live credential posture beyond read-only adapter previews;
- live write sandbox smoke;
- CLI write command;
- workflow-declared write configuration;
- automatic runtime write execution.

## 5. Readiness Principle

The next live write must be treated as a governed side effect, not as an adapter convenience method.

The required shape is:

```text
explicit caller request
  -> policy effect enforcement
  -> adapter write preflight
  -> proposed SideEffectRecord persisted
  -> accepted SideEffectProposed workflow event
  -> approval or high-assurance approval where required
  -> idempotency guard
  -> live sandbox credential boundary
  -> provider attempt
  -> attempted/completed/failed SideEffect transition
  -> audit/report disclosure
```

If any stage is missing, the provider call remains unsupported.

## 6. Required Readiness Gates

Before live GitHub PR comment mutation, a future implementation must satisfy these gates:

1. **Explicit API gate**: live write is reachable only through a new explicit API, never default `execute(...)`, default report generation, validation, first-run, or scaffold commands.
2. **Mode gate**: fixture/dry-run remains the default; live sandbox mode must be separately opted in.
3. **Capability gate**: capability must be exactly GitHub pull request comment creation.
4. **Target gate**: repository and pull request target must be bounded, non-secret-like, and sandbox-eligible.
5. **Policy gate**: policy must explicitly allow the write candidate; unsupported or unknown effects fail closed.
6. **SideEffect proposal gate**: a persisted proposed `SideEffectRecord` must exist before provider invocation.
7. **Event proof gate**: an accepted `SideEffectProposed` workflow event must exist before provider invocation when the write is tied to a workflow run.
8. **Approval gate**: live mode must require approval; high-assurance mode must require stronger validation when configured.
9. **Idempotency gate**: the live call must have deterministic idempotency posture before provider invocation.
10. **Credential gate**: credentials must come from explicit local environment or secret reference, never specs or report text.
11. **Redaction gate**: request, response, Debug, serialization, and errors must not leak bodies, tokens, paths, provider payloads, or secret-like values.
12. **Lifecycle gate**: provider attempt and outcome must map to reviewed SideEffect lifecycle transitions.
13. **Audit/report gate**: attempted/completed/failed posture must be visible through workflow events, audit projection, WorkReport, and report artifact citations.
14. **Failure semantics gate**: provider failures must not fabricate completion, lose the proposed record, or make the workflow result ambiguous.

## 7. Future API Boundary

The first live-capable implementation should add a new explicit helper or executor-adjacent service, not broaden existing default paths.

Allowed future shape:

```rust
execute_github_pr_comment_write_candidate(...)
```

or a similarly explicit provider-write helper that accepts:

- persisted proposed `SideEffectRecord` identity;
- accepted proposed workflow event proof;
- policy decision reference;
- approval or high-assurance approval reference;
- idempotency binding;
- bounded GitHub PR comment request;
- explicit sandbox/live mode;
- explicit credential reference;
- redaction metadata.

Rejected future shape:

- provider writes from `LocalExecutor::execute(...)`;
- automatic writes from `execute_with_report(...)`;
- automatic writes from report artifact creation;
- writes triggered by workflow validation;
- writes triggered by first-run or scaffolding;
- writes inferred from YAML without runtime enforcement.

## 8. Credential And Sandbox Posture

The first live smoke must be sandbox-only.

Requirements:

- token supplied through environment or a documented secret reference;
- token never appears in specs, events, reports, errors, debug output, serialized payloads, or tests;
- target repository and pull request must be explicitly supplied and marked sandbox-eligible;
- live smoke must be opt-in through test-only or explicit local configuration;
- no broad production repository target should be accepted in the first live slice;
- no hosted credential storage is introduced.

The first implementation should prefer a disabled-by-default opt-in smoke test over product-facing runtime exposure.

## 9. Approval And High-Assurance Posture

Live GitHub PR comment writes require approval because they mutate an external system and may notify humans.

Approval context must include:

- requested capability;
- GitHub target reference;
- proposed `SideEffectId`;
- policy decision reference;
- idempotency posture;
- bounded comment purpose;
- redaction posture;
- sandbox/live mode.

High-assurance approval is not required for every low-risk sandbox comment by default, but future policy should be able to require it. When high-assurance posture is configured, the live write path must validate it before provider invocation.

Denial, missing approval, approval identity mismatch, or high-assurance validation failure must prevent provider invocation.

## 10. Idempotency And Duplicate Prevention

GitHub does not provide natural idempotency for pull request comments.

The first live implementation must choose and document a local duplicate-prevention strategy before provider invocation. Acceptable first options:

- require a local proposed record and refuse duplicate completion for the same `SideEffectId`;
- require a caller-supplied idempotency key and store attempted/completed outcome locally;
- include a bounded non-secret marker in the comment body only if separately reviewed.

The first live slice should not implement provider comment lookup unless separately scoped.

## 11. SideEffect Lifecycle Semantics

Provider invocation must be surrounded by explicit lifecycle transitions:

- `Proposed` exists before call.
- `Attempted` records that a provider call was attempted.
- `Completed` records a provider reference after successful comment creation.
- `Failed` records a classified non-leaking provider failure.
- `Denied` remains available when policy or approval blocks before attempt.
- `Skipped` remains available for explicit non-execution posture.

The future implementation must decide whether the first live slice appends lifecycle events itself or returns lifecycle transition records for a caller to append. Either choice must be reviewed before implementation.

No attempted/completed/failed transition may be fabricated from fixture results.

## 12. Audit And Report Posture

Live write posture must be inspectable without copying provider payloads.

Required references:

- proposed `SideEffectRecord`;
- accepted proposed workflow event;
- attempted/completed/failed SideEffect lifecycle event when implemented;
- policy decision;
- approval or high-assurance decision when applicable;
- provider reference for successful comment creation;
- WorkReport/report artifact citation.

Reports may include bounded summaries, but must not copy:

- raw comment body beyond the reviewed bounded body model;
- raw GitHub API response;
- raw pull request body;
- raw diff;
- command output;
- CI logs;
- credentials or token-like values.

## 13. Failure Semantics

Future provider write failures must be deterministic and non-leaking.

Failure cases:

- missing or denied policy;
- missing approval;
- high-assurance validation failure;
- missing proposed record;
- missing proposed event proof;
- idempotency conflict;
- invalid sandbox target;
- missing or invalid credential reference;
- provider authentication failure;
- provider permission failure;
- provider rate limit;
- provider not found;
- provider validation failure;
- network failure;
- unknown provider failure.

Rules:

- failures before provider invocation must not create attempted/completed events;
- failures after provider invocation must not claim completion unless the provider reference is validated;
- errors must use stable codes;
- errors must not include target URLs, token values, comment bodies, provider payloads, raw paths, command output, or secret-like values;
- workflow result semantics must be explicit before integrating with executor paths.

## 14. Test Plan For Future Implementation

Future implementation tests must cover:

- default paths still cannot write;
- explicit live-capable helper is required;
- fixture/dry-run remains no-provider-call;
- live mode is opt-in and test-gated;
- unsupported capability fails closed;
- non-sandbox target fails closed for first live smoke;
- missing proposed record fails before provider call;
- missing proposed event proof fails before provider call when required;
- missing policy decision fails before provider call;
- denied policy fails before provider call;
- missing approval fails before provider call;
- approval mismatch fails before provider call;
- high-assurance validation failure fails before provider call when configured;
- missing idempotency binding fails before provider call;
- duplicate idempotency fails safely;
- missing credential reference fails without leaking;
- provider auth, permission, rate limit, not found, and network errors are classified;
- successful sandbox comment stores provider reference only;
- attempted/completed/failed lifecycle transitions are deterministic;
- audit/report citations reference stable IDs only;
- Debug and serialization do not leak comment body, target, tokens, IDs treated as sensitive, or provider payloads;
- no CLI command writes to GitHub;
- existing read-only GitHub adapter tests still pass;
- existing provider-candidate report artifact tests still pass.

## 15. Proposed Implementation Sequence

Recommended future phases:

1. **Provider write readiness review**: review this plan before code.
2. **SideEffect lifecycle transition helper planning**: define attempted/completed/failed transition mechanics for provider writes.
3. **GitHub PR comment live sandbox helper implementation**: explicit helper only, opt-in smoke only, no executor default behavior.
4. **Live sandbox helper review**: security and non-leakage review before broader exposure.
5. **Executor-adjacent explicit provider write planning**: only after live helper review, define how an executor path may call it.
6. **CLI and schema planning later**: do not expose user-facing mutation commands until runtime semantics are reviewed.

Do not skip directly to executor writes or CLI mutation.

## 16. Open Questions

- Should live sandbox comments require high-assurance approval by default, or only when caller policy asks for it?
- Should provider writes return lifecycle transition records or append events through an executor path?
- Should idempotency use only local SideEffect state, or also include a bounded provider-visible marker?
- Should the first live smoke target a private sandbox repository, a public test PR, or a maintainer-owned fork?
- How should provider reference permanence be represented if a comment is deleted externally?
- Should successful live smoke be allowed in CI, or only in local opt-in tests?
- How should branch protection and GitHub app/token permission scopes be documented?
- What is the smallest useful provider credential abstraction before hosted runtime exists?

## 17. Final Recommendation

Proceed next to a focused maintainer review of this provider write readiness plan.

Do not implement live GitHub PR comment mutation yet. The next code-bearing phase should only be considered after this plan is accepted and should remain explicit, sandbox-only, opt-in, and no-CLI.

## 18. Planning Validation

- `npm run check:docs`: passed.
- Code checks were not run because this phase is documentation-only planning and did not change Rust or TypeScript code.
- Governed phase closeout: passed.

Governed planning:

- workflow: `dg/d`;
- run: `run-1783261325630452000-2`;
- approval: `approval/run-1783261325630452000-2/planning-approved`;
- approval outcome: granted by delegated maintainer;
- phase closeout: completed;
- events: 39 total, 1 approval, 0 retries, 0 escalations.
