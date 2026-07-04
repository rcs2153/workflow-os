# First Provider Write Candidate Plan

Status: Planning complete, first model-only request/response boundary implemented, preflight composition implemented as model/helper-only, fixture-backed adapter validation implemented as fixture-only helper, and in-memory proposed `SideEffectRecord` composition implemented. Proposed `SideEffectRecord` composition is documented in [GitHub PR Comment Proposed SideEffectRecord Composition Plan](github-pr-comment-side-effect-record-composition-plan.md) and [GitHub PR Comment Proposed SideEffectRecord Composition Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_COMPOSITION_HELPER_REPORT.md). This plan chooses the first low-risk provider write candidate after the adapter-neutral write preflight helper was implemented and reviewed. The model-only GitHub PR comment write request/response boundary now exists, [GitHub PR Comment Preflight Composition Plan](github-pr-comment-preflight-composition-plan.md) implements the helper-only bridge before fixture-backed adapter work, and [GitHub PR Comment Fixture Adapter Plan](github-pr-comment-fixture-adapter-plan.md) documents the no-provider-call fixture helper. This does not implement provider mutation, write-capable adapters, proposed record persistence, runtime side-effect execution, CLI write commands, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has a preflight-only adapter write helper that can validate whether a future write request has enough governed posture to be considered ready for a later write path.

The next question is which provider operation should be planned first.

This plan recommends **GitHub pull request comment** as the first provider write candidate. The first implementation is now model-only: it defines bounded request/response types, validates matching preflight posture, and remains sandbox-only, fixture-first, explicit-input-only, and opt-in. Future provider-call-capable work must route through the existing write preflight helper before any provider call and must not become automatic runtime behavior.

Jira sandbox comments remain a close second candidate, but GitHub pull request comments are better aligned with the current dogfood loop, public repo workflow, and PR review/report use cases.

## 2. Goals

- Choose a single first provider write candidate.
- Preserve the write-adapter readiness boundary.
- Keep the first candidate low-risk, reviewable, and sandboxable.
- Require adapter write preflight before provider invocation.
- Require a proposed `SideEffect` identity before provider invocation.
- Require policy and approval posture appropriate to the selected write.
- Require idempotency posture before provider invocation.
- Keep raw provider payloads out of reports, events, debug output, and errors.
- Define fixture-first tests before any live write smoke.
- Prepare a future implementation prompt without authorizing implementation in this phase.

## 3. Non-Goals

This plan does not authorize:

- provider mutation;
- write-capable adapter implementation;
- GitHub comments, reviews, branch creation, commits, pull requests, labels, merges, closes, check updates, workflow dispatch, or reruns;
- Jira comments, issue updates, transitions, assignment, labels, links, or status changes;
- runtime side-effect execution;
- `SideEffect` attempted/completed/failed lifecycle transition implementation;
- automatic workflow event appends for write attempts;
- automatic report generation or artifact writing;
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

## 4. Candidate Assessment

### GitHub Pull Request Comment

Assessment: recommended first candidate.

Why it fits:

- It is useful to Workflow OS dogfooding and PR hygiene.
- It is naturally scoped to a pull request, not a broad repository mutation.
- It can be fixture-tested without live credentials.
- It can be sandboxed to a dedicated test repository or a maintainer-created test PR.
- It aligns with existing GitHub read-only adapter knowledge.
- It produces a clear external provider reference if later implemented.
- It is easier to explain than branch creation, merge, status updates, or CI reruns.

Risks:

- It is still an external write.
- It can notify people or create public noise if pointed at the wrong PR.
- It may include sensitive summary text if not bounded.
- It could be mistaken for approval or final review unless explicitly labeled.
- It requires GitHub token handling and permission scoping.

Mitigations:

- sandbox target only;
- explicit request model;
- preflight-required;
- proposed `SideEffectId` required;
- idempotency key required;
- policy decision reference required;
- approval reference required for live mode;
- bounded comment body or template, no raw payloads;
- dry-run/fixture mode before live smoke;
- no default executor integration;
- no CLI command in first implementation.

### Jira Sandbox Comment

Assessment: defer.

Why it is attractive:

- Jira comments are low-risk compared with transitions, assignment, labels, or issue edits.
- Jira is a common enterprise workflow system.
- It proves Workflow OS is not GitHub-only.

Why it should not be first:

- Jira auth and site/project configuration are harder for contributors to reproduce.
- A sandbox project must exist and be documented carefully.
- Jira issue/comment visibility can be surprisingly sensitive.
- GitHub PR comments are closer to the current repo dogfood loop.

### Rejected First Candidates

Reject as first provider write candidates:

- GitHub branch creation;
- GitHub pull request creation;
- GitHub merge, close, label, review request, or status write;
- GitHub Actions rerun, dispatch, or cancel;
- Jira issue field update;
- Jira status transition;
- Jira assignment or label change;
- repository file writes;
- generic HTTP write;
- any destructive, irreversible, or broad-permission operation.

## 5. Recommendation

The first provider write candidate should be:

```text
GitHub pull request comment, sandbox-only, fixture-first, explicit-input-only.
```

The first implementation should not post comments by default. It should start with a provider write request/response model and a fixture-backed adapter path that can prove request construction, preflight composition, redaction, idempotency posture, and no-default-write behavior.

Only after that is reviewed should a separately scoped opt-in live sandbox smoke be considered.

## 6. Required Governance Gates

A future GitHub PR comment write attempt must pass these gates before provider invocation:

1. **Capability gate**: requested capability is `GitHubPullRequestComment`.
2. **Target gate**: target identifies a bounded pull request reference without raw URL secrets or token-like values.
3. **Policy gate**: policy decision is allowed and referenced.
4. **SideEffect proposal gate**: a proposed `SideEffectId` exists before attempt.
5. **Approval gate**: live write mode requires approval reference; high-assurance mode can require stronger evidence later.
6. **Idempotency gate**: deterministic idempotency key is present.
7. **Redaction gate**: comment body, summary, target, errors, and debug output are bounded and secret-safe.
8. **Preflight gate**: `preflight_adapter_write(...)` returns ready posture.
9. **Credential gate**: token is supplied only through documented local environment/secret reference, never through specs.
10. **Mode gate**: fixture/dry-run mode must be the default; live sandbox mode must be opt-in.

## 7. Request And Response Boundary

A future request model should include:

- adapter identity and version;
- correlation ID;
- workflow ID and version;
- schema version;
- spec hash;
- run ID;
- step ID if available;
- actor/system actor;
- capability: `GitHubPullRequestComment`;
- repository owner/name or stable repository reference;
- pull request number or stable PR reference;
- bounded comment summary/body reference;
- policy decision reference;
- approval/high-assurance reference when required;
- proposed `SideEffectId`;
- idempotency key;
- sensitivity;
- redaction metadata.

It must not include:

- raw GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw pull request body;
- raw diff contents;
- raw check logs;
- raw command output;
- environment variable values;
- unbounded prompt text;
- secret-like values.

A future response model should include:

- provider reference to the created comment if live mode succeeds;
- bounded redacted summary;
- classified provider error if failed;
- no raw response payload by default;
- no workflow state mutation by itself.

## 8. SideEffect Lifecycle Boundary

This plan does not implement attempted/completed/failed lifecycle transitions.

Future implementation must decide whether the first GitHub PR comment slice:

- requires an already persisted proposed `SideEffectRecord`; or
- accepts a validated proposed `SideEffectId` and defers persistence to a later executor composition path.

Recommendation: require a proposed `SideEffectId` for the first model/fixture slice, and require proposed `SideEffectRecord` composition before any live provider write is allowed. That follow-up is documented in [GitHub PR Comment Proposed SideEffectRecord Composition Plan](github-pr-comment-side-effect-record-composition-plan.md), and the first in-memory composition helper is implemented. Proposed record persistence remains future work.

## 9. Policy And Approval Posture

For fixture-only request construction, approval may be supplied as a stable reference or omitted only when the policy explicitly marks the run as non-live/non-provider-calling.

For any live sandbox smoke:

- approval reference is required;
- approval context must include target, capability, proposed `SideEffectId`, idempotency key posture, and bounded comment purpose;
- denial prevents provider invocation;
- policy denial prevents provider invocation;
- missing approval fails closed;
- missing policy reference fails closed.

The first implementation should not add enterprise RBAC, IdP, quorum approval, role-bound approver authority, revocation enforcement, or production approval workflows.

## 10. Idempotency Posture

Future write request construction must require an idempotency key before adapter invocation.

For a GitHub PR comment, idempotency should initially be local-kernel-scoped rather than provider-enforced. The adapter should not assume GitHub de-duplicates comments.

Future live implementation should decide whether duplicate protection is:

- local SideEffect store lookup;
- provider comment marker lookup;
- deterministic comment body marker;
- or a combination.

Do not implement duplicate comment lookup in the first planning-to-model slice unless separately scoped.

## 11. Privacy And Redaction

Comment text can leak sensitive information even when the operation is low-risk.

Rules:

- no raw provider payloads;
- no raw PR descriptions;
- no raw diffs;
- no raw logs;
- no command output;
- no credentials, tokens, authorization headers, or private keys;
- no environment variable values;
- summaries must be bounded;
- errors must use stable non-leaking codes;
- debug output must redact target/body/reference details;
- live smoke resources must be non-sensitive and explicitly approved.

## 12. Testing Plan

Future tests should cover:

- request model accepts valid GitHub PR comment fixture input;
- unsupported write capability fails closed;
- missing proposed `SideEffectId` fails closed;
- missing idempotency key fails closed;
- missing policy reference fails closed;
- denied policy fails closed;
- missing approval fails closed for live mode;
- preflight helper is called before provider invocation;
- fixture mode performs no provider call;
- default runtime path performs no provider write;
- comment body/summary is bounded;
- secret-like target/body/error values are rejected or redacted;
- debug output does not leak target, comment text, idempotency key, tokens, or secret-like values;
- serialization does not leak forbidden raw payload markers;
- no workflow event append occurs unless separately scoped;
- no SideEffect lifecycle transition occurs unless separately scoped;
- existing read-only adapter tests still pass;
- existing write preflight tests still pass;
- docs checks pass.

## 13. Proposed Implementation Sequence

1. Model-only GitHub PR comment write request/response plan review.
2. Provider write request/response model implementation, no provider call.
3. Explicit preflight composition with request model.
4. Review.
5. Fixture-backed GitHub PR comment adapter planning, no live call.
6. Fixture-backed GitHub PR comment adapter implementation, no live call.
7. Review.
8. Persisted proposed `SideEffectRecord` composition planning.
9. Review.
10. Opt-in live sandbox smoke planning.
11. Only after review, consider live sandbox comment implementation.

The next implementation should start at step 6 only after fixture-backed adapter planning is reviewed.

## 14. Deferred Work

Deferred:

- live provider mutation;
- CLI write command;
- automatic executor write integration;
- workflow-declared write support;
- SideEffect attempted/completed/failed transition implementation;
- provider duplicate detection;
- report artifact write automation;
- webhook ingestion;
- OAuth app behavior;
- hosted runtime;
- production credentials;
- Jira write candidate implementation;
- GitHub branch/PR/merge/status/check writes;
- enterprise RBAC/IdP/quorum approval;
- reasoning lineage;
- examples;
- schemas;
- release posture changes.

## 15. Open Questions

- Should a live PR comment require a persisted proposed `SideEffectRecord` before provider invocation?
- Should duplicate comment prevention use local idempotency state, provider comment markers, or both?
- Should comment text be templated rather than free-form bounded text?
- Should the first live smoke target a private sandbox repo or a public test PR?
- Should approval be mandatory even for sandbox live writes?
- Should the first write request model include provider-specific response references or a generic write outcome reference?
- Should GitHub PR comments be categorized as low-risk by default, or sensitive until enterprise stewardship exists?
- What is the minimum acceptable evidence for a successful sandbox write?

## 16. Final Recommendation

Proceed next to a review of this planning document.

The model-only GitHub PR comment write request/response boundary is implemented with no provider call. Provider mutation should remain deferred until the request model is reviewed, fixture-backed adapter composition is separately planned, and proposed SideEffect persistence posture, approval posture, idempotency posture, and fixture tests are reviewed.
