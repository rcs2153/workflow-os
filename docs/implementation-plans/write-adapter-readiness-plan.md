# Write-Capable Adapter Readiness Plan

Status: Planning complete, first preflight-only helper implemented, first model-only provider write request/response boundary implemented, preflight composition implemented as model/helper-only, fixture-backed adapter validation implemented as fixture-only helper, in-memory proposed `SideEffectRecord` composition implemented, explicit proposed record persistence through `SideEffectRecordStore` implemented, and workflow event/audit projection for persisted proposed GitHub PR comment records planned. Proposed `SideEffectRecord` composition for GitHub PR comment writes is documented in [GitHub PR Comment Proposed SideEffectRecord Composition Plan](github-pr-comment-side-effect-record-composition-plan.md) and [GitHub PR Comment Proposed SideEffectRecord Composition Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_COMPOSITION_HELPER_REPORT.md). Proposed record persistence is documented in [GitHub PR Comment Proposed SideEffectRecord Persistence Plan](github-pr-comment-side-effect-record-persistence-plan.md) and implemented in [GitHub PR Comment Proposed SideEffectRecord Persistence Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_PERSISTENCE_HELPER_REPORT.md). Projection planning is documented in [GitHub PR Comment Proposed SideEffect Event/Audit Projection Plan](github-pr-comment-side-effect-event-audit-projection-plan.md). Planning completion is documented in [Write-Adapter Readiness Plan Report](../concepts/WRITE_ADAPTER_READINESS_PLAN_REPORT.md), and the first helper slice is documented in [Write Adapter Preflight Helper Report](../concepts/WRITE_ADAPTER_PREFLIGHT_HELPER_REPORT.md). The first provider write candidate is planned in [First Provider Write Candidate Plan](first-provider-write-candidate-plan.md): GitHub pull request comment is the first candidate, and its model-only request/response boundary is implemented without provider calls. Preflight composition is documented in [GitHub PR Comment Preflight Composition Plan](github-pr-comment-preflight-composition-plan.md): it executes the existing preflight helper against the GitHub PR comment request model before fixture-backed adapter execution. Fixture-backed adapter validation is documented in [GitHub PR Comment Fixture Adapter Plan](github-pr-comment-fixture-adapter-plan.md) and implemented in [GitHub PR Comment Fixture Adapter Helper Report](../concepts/GITHUB_PR_COMMENT_FIXTURE_ADAPTER_HELPER_REPORT.md), still with no provider calls. This plan defines the readiness boundary for future write-capable adapters after the SideEffect, high-assurance approval, evidence, local check, WorkReport, and report artifact foundations have advanced. It does not implement write-capable adapters, provider mutation, runtime side-effect execution, CLI behavior, workflow schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has enough governance primitives to begin planning the transition from read-only adapter proving toward write-capable adapter readiness.

That does not mean writes should be implemented immediately. The current kernel can model and cite side effects, preserve approval context, generate and persist report artifacts through explicit paths, enforce selected policy effects, and run selected explicit local checks. The next question is how to turn those foundations into a safe adapter write boundary without collapsing into a one-off GitHub/Jira automation tool.

The first code-bearing phase is now implemented as a write-adapter readiness helper/model that proves a future write request can be classified, policy-gated, approval-linked, side-effect-recorded by reference, idempotency-bound, and reportable before any adapter is allowed to call a provider write API. It remains preflight-only and does not call providers.

## 2. Goals

- Define the minimum governance gates required before any provider write.
- Preserve Workflow OS as a governed workflow runtime, not a one-off provider automation bot.
- Reuse existing SideEffect, approval, evidence, WorkReport, report artifact, policy, and adapter telemetry foundations.
- Require writes to be explicit, opt-in, policy-gated, approval-aware, idempotency-bound, auditable, and redaction-safe.
- Preserve current read-only adapter behavior.
- Keep default runtime paths write-denied.
- Identify the smallest future code slice that reduces the docs/runtime gap without mutating providers.
- Define what must be reviewed before the first real provider write adapter.

## 3. Non-Goals

This plan does not authorize:

- implementation in this phase;
- GitHub branch creation, pull request creation, comments, merges, status writes, workflow dispatch, or CI reruns;
- Jira issue updates, comments, status changes, assignment, labels, or transitions;
- generic provider mutation;
- runtime side-effect execution;
- automatic side-effect attempts or completions;
- automatic report generation;
- automatic report artifact writing from default executor paths;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- OAuth app behavior, webhook ingestion, or production credential management;
- RBAC, IdP, SSO, SCIM, teams, groups, or enterprise admin controls;
- quorum approval, revocation enforcement, or safety-critical certification;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented foundations relevant to write readiness:

- read-only GitHub, Jira, and GitHub Actions adapter preview posture;
- typed policy-effect enforcement for supported v0 effects;
- governed multi-step local execution;
- durable local run state and event history;
- approval requests and decisions;
- high-assurance approval model, validation helper, explicit executor decision path, and report disclosure propagation;
- workflow-declared high-assurance artifact requirement schema, derivation, and explicit artifact-path integration;
- SideEffect core model, persistence, discovery, event/audit projection model, report citation vocabulary, referential integrity validation, and approval linkage;
- WorkReport, terminal report helper, executor report result path, local report artifact store, and explicit artifact gates;
- local check model, explicit DocsCheck handler, opt-in live DocsCheck smoke, and local check report citations;
- EvidenceReference core model and selected attachment paths.

Still not implemented:

- provider writes;
- runtime side-effect execution;
- write-capable adapter request/response path;
- provider mutation idempotency execution;
- live write credentials;
- write dry-run/plan semantics;
- CLI write behavior;
- hosted execution;
- production adapter management;
- enterprise authority administration.

## 5. Readiness Thesis

Workflow OS should move toward writes only when a write request is a governed object, not an incidental adapter method call.

The write boundary should be:

```text
declared capability
  -> policy precheck
  -> SideEffect proposal
  -> required approval/high-assurance validation when applicable
  -> idempotency binding
  -> adapter request with redacted summaries
  -> attempted/completed/failed/denied SideEffect state
  -> audit/event/report/artifact disclosure
```

Credentials must never be treated as authority. Possessing a token means the adapter can technically call a provider. It does not mean Workflow OS has authorized the write.

## 6. Required Readiness Gates

Before any provider write implementation, Workflow OS should have reviewed answers for these gates:

1. **Capability gate**: the requested provider operation maps to a supported capability.
2. **Policy gate**: unsupported, unknown, or denied capabilities fail closed before adapter invocation.
3. **SideEffect proposal gate**: every future write has a `proposed` SideEffect record before attempt.
4. **Authority gate**: sensitive or ambiguous writes require approval or high-assurance approval posture before attempt.
5. **Idempotency gate**: write attempts require a deterministic idempotency binding.
6. **Adapter preflight gate**: adapter requests must carry policy/approval/side-effect/idempotency references.
7. **Redaction gate**: request and response summaries must avoid raw provider payloads, credentials, tokens, raw logs, raw source contents, and secret-like values.
8. **Attempt/completion gate**: attempted, completed, failed, denied, and skipped write outcomes must update SideEffect lifecycle through explicit reviewed paths.
9. **Audit/report gate**: write posture must be visible through workflow events/audit projections and WorkReport/report artifact disclosures.
10. **Failure semantics gate**: provider write failure must not produce ambiguous workflow state or partial artifact claims.

If any gate is missing for a provider operation, that operation remains unsupported.

## 7. First Code-Bearing Readiness Slice

Implemented first slice after this plan was reviewed:

**write adapter preflight model/helper, no provider calls**

The helper should accept explicit inputs and return a deterministic preflight decision for a future write request.

Candidate model concepts:

- `AdapterWritePreflightRequest`
- `AdapterWritePreflightDecision`
- `AdapterWriteCapability`
- `AdapterWriteTarget`
- `AdapterWriteAuthorityContext`
- `AdapterWriteReadinessPolicy`
- `AdapterWriteReadinessError`

The helper should:

- classify the requested capability;
- reject unsupported write capabilities fail-closed;
- require a proposed SideEffect reference or record;
- require idempotency binding;
- require policy decision references;
- require approval/high-assurance references when the capability is sensitive;
- validate target references and summaries are bounded/redaction-safe;
- return stable non-leaking error codes;
- produce no provider calls, workflow events, SideEffect lifecycle transitions, report artifacts, CLI output, schemas, examples, or writes.

This helper is the bridge between governance primitives and future provider adapter implementations. It does not create SideEffect lifecycle transitions, append workflow events, write report artifacts, expose CLI behavior, or mutate providers.

## 8. First Provider Write Candidate

The first real provider write should be selected only after the preflight helper is implemented and reviewed.

Preferred candidates for later planning:

- GitHub pull request comment in a dedicated test repository or sandbox branch.
- Jira comment in a dedicated sandbox project.

Planning result: [First Provider Write Candidate Plan](first-provider-write-candidate-plan.md) recommends GitHub pull request comment as the first candidate, and the model-only request/response boundary is implemented before any provider mutation. Fixture-backed adapter validation is implemented as no-provider-call helper and documented in [GitHub PR Comment Fixture Adapter Plan](github-pr-comment-fixture-adapter-plan.md).

Avoid first:

- branch creation;
- pull request creation;
- merge operations;
- Jira status transitions;
- CI rerun/dispatch/cancel operations;
- repository file writes;
- destructive provider actions;
- anything requiring elevated enterprise authority.

Why comments are likely first:

- reversible at the human/process level;
- easy to scope to sandbox targets;
- useful for governed review/report workflows;
- lower blast radius than branch, merge, status, or issue-transition operations.

Even a comment is still a write and must remain blocked until the preflight, side-effect, approval, idempotency, audit, and report boundaries are reviewed.

## 9. Adapter Contract Requirements

A future write-capable adapter request must include:

- adapter identity and version;
- integration identity if available;
- correlation ID;
- workflow ID and version;
- schema version;
- spec hash;
- run ID;
- step ID if available;
- actor/system actor;
- requested capability;
- target reference;
- policy decision reference;
- approval/high-assurance reference when applicable;
- SideEffect ID;
- idempotency binding;
- bounded purpose/summary;
- sensitivity and redaction metadata.

It must not include:

- raw provider payloads;
- credentials;
- tokens;
- authorization headers;
- raw command output;
- raw CI logs;
- raw issue/comment bodies beyond bounded summaries;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded natural-language prompts.

## 10. Runtime Integration Boundary

The first write-capable runtime path should be explicit and additive.

Allowed later:

- an explicit executor-adjacent write preflight path;
- an explicit adapter write attempt path after preflight review;
- an explicit sandbox/live opt-in test path;
- explicit SideEffect lifecycle transition helpers;
- explicit WorkReport/report artifact disclosure of attempted or completed writes.

Rejected for first implementation:

- changing `LocalExecutor::execute(...)`;
- changing default report-bearing execution to write automatically;
- automatic provider mutation from workflow YAML;
- automatic report artifact writing from default executor paths;
- CLI commands that mutate providers;
- hidden runtime config;
- ambient credentials;
- workflow-declared write support without enforcement;
- provider calls from validation or first-run commands.

## 11. Policy And Approval Posture

Write support must preserve the invariant:

```text
declared governance is enforced or rejected
```

Policy requirements:

- unknown write capabilities fail closed;
- unsupported effects fail closed during validation or preflight;
- missing policy decisions fail closed for writes;
- policy allow alone is not enough for sensitive actions when approval is required;
- policy deny cannot be bypassed by adapter credentials.

Approval requirements:

- approval context must include target, capability, SideEffect ID, policy posture, idempotency posture, and bounded impact summary;
- high-assurance approval must be required for sensitive or irreversible operations once that mapping is designed;
- agent-provided evidence may satisfy observe/report-only profiles, but enterprise-gated profiles require steward/admin policy before broad use;
- denial must prevent provider invocation.

## 12. Idempotency And Retry Posture

Write-capable adapters must be restart-safe and duplicate-safe.

Rules:

- every write attempt needs an idempotency key before provider invocation;
- retries must not create duplicate provider mutations;
- duplicate provider responses must be classifiable without raw payload storage;
- idempotency binding must be cited in SideEffect records;
- retry policy must be explicit and bounded;
- provider-specific idempotency limitations must be documented before implementation.

## 13. SideEffect Lifecycle Posture

SideEffect state is the durable governed record for external mutation intent and outcome.

Rules:

- `proposed` before provider call;
- `denied` when policy/capability/approval blocks the write;
- `attempted` only when provider invocation begins;
- `completed` only after bounded provider success classification;
- `failed` after bounded provider failure classification;
- `skipped` only for explicit no-op or unsupported/postponed posture.

Do not treat approval as execution. Do not treat provider success as audit completeness. Do not treat report citation as proof of external provider state.

## 14. Evidence, Audit, And Report Posture

Future write paths must be reportable without copying payloads.

Rules:

- WorkReports cite SideEffect IDs, approval references, policy references, adapter telemetry, audit/workflow events, and EvidenceReference IDs where available;
- reports disclose attempted/completed/failed/denied/skipped side effects;
- report artifacts that cite SideEffects should use referential integrity gates where required;
- command-output evidence remains deferred unless separately planned;
- provider payload evidence must remain reference-first and redaction-safe;
- missing citations must be explicit and must not fabricate evidence.

## 15. Credential And Secret Posture

Future write adapters must not load credentials from specs or prompts.

Allowed later:

- documented environment variable references;
- documented local secret references;
- provider-specific credential loaders with redaction-safe errors.

Required:

- no credentials in specs, reports, events, audit records, diagnostics, WorkReports, SideEffect records, or adapter telemetry;
- authentication failures are classified without leaking credential material;
- permission failures are classified without leaking private provider payloads;
- live tests are skipped by default and require explicit environment opt-in.

## 16. Test Plan For Future Implementation

The write preflight model/helper should add tests for:

- valid low-risk preflight request;
- unsupported capability rejected;
- unknown capability rejected;
- missing SideEffect reference rejected;
- missing idempotency binding rejected;
- missing policy decision rejected;
- sensitive capability without approval rejected;
- high-assurance-required capability without validation rejected;
- bounded target validation;
- secret-like target rejected;
- secret-like summary rejected;
- redaction-safe Debug;
- redaction-safe serde where exposed;
- stable non-leaking error codes;
- no provider call;
- no SideEffect lifecycle transition;
- no workflow event append;
- no report artifact write;
- no CLI output;
- existing read-only adapter tests still pass;
- existing SideEffect, approval, WorkReport, local check, and policy tests still pass.

Later provider-specific write tests must be fixture-first, sandbox-only for live tests, and skipped by default.

## 17. Proposed Implementation Sequence

Recommended phases:

1. Write-adapter readiness plan review.
2. Write preflight model/helper implementation, no provider calls.
3. Write preflight model/helper review.
4. First provider write candidate planning, likely GitHub PR comment or Jira sandbox comment.
5. Provider-specific write adapter request/response model, no live call.
6. Provider-specific sandbox fixture tests.
7. Explicit opt-in live sandbox smoke.
8. Executor-adjacent opt-in write path planning.
9. Executor-adjacent opt-in write path implementation after review.

Do not skip directly from this plan to provider mutation.

## 18. Open Questions

- Should the first real provider write be GitHub PR comment or Jira sandbox comment?
- Should write preflight live in adapter-neutral core or an adapter-readiness module?
- Which capabilities require high-assurance approval by default?
- How should enterprise strictness profiles later override local observe/report-only posture?
- How should provider-specific idempotency limitations be represented?
- Should report artifacts be mandatory for write attempts in local preview?
- Which audit/event projections are required before first live write?
- How should failed provider writes be retried without duplicate mutation?
- What is the smallest sandbox target that proves value without production risk?

## 19. Final Recommendation

Proceed next to **write-adapter readiness plan review**.

If accepted, implement only the **write adapter preflight model/helper, no provider calls**.

Still do not build provider writes, runtime side-effect execution, CLI mutation commands, workflow-declared write schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.
