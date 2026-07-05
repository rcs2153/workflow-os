# Write-Adapter Orchestration Plan

Status: Planning complete and accepted in [Write-Adapter Orchestration Plan Review](../concepts/WRITE_ADAPTER_ORCHESTRATION_PLAN_REVIEW.md). The first no-provider-call implementation slice is documented in [Write-Adapter Orchestration Helper Report](../concepts/WRITE_ADAPTER_ORCHESTRATION_HELPER_REPORT.md) and accepted in [Write-Adapter Orchestration Helper Review](../concepts/WRITE_ADAPTER_ORCHESTRATION_HELPER_REVIEW.md): it composes GitHub PR comment preflight, proposed `SideEffectRecord` persistence, approval-side-effect linkage, and store-backed attempted lifecycle transition without calling providers or appending workflow events. Follow-on completed/failed local outcome planning is documented in [Write-Adapter No-Provider Outcome Orchestration Plan](write-adapter-no-provider-outcome-orchestration-plan.md). This plan follows the accepted [Executor SideEffect Lifecycle Event Append Review](../concepts/EXECUTOR_SIDE_EFFECT_LIFECYCLE_EVENT_APPEND_REVIEW.md). It defines the no-provider-call orchestration boundary for composing existing write-readiness primitives. It does not implement provider writes, runtime side-effect execution, CLI mutation behavior, workflow schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, auth material loading, or release posture changes.

## 1. Executive Summary

Workflow OS now has separate reviewed primitives for the first write-candidate lane:

- adapter write preflight validation;
- GitHub pull request comment request/response models;
- fixture-backed adapter validation with no provider calls;
- proposed `SideEffectRecord` composition and persistence;
- proposed SideEffect workflow event construction and executor append;
- approval-side-effect linkage;
- store-backed attempted/completed/failed lifecycle transition helpers;
- executor attempted/completed/failed lifecycle event append;
- WorkReport and report artifact SideEffect citation/integrity gates.

Those pieces are intentionally separate. The next gap is not a provider call. The next gap is an explicit local orchestration helper that proves the pieces can be sequenced without hiding authority, fabricating evidence, or mutating providers.

The first helper implementation is now available for the GitHub PR comment candidate and stops at attempted-state orchestration. It accepts explicit inputs, composes already-implemented primitives, and returns a bounded orchestration result. It does not call GitHub, Jira, CI, shell commands, local write tools, or any external provider.

## 2. Goals

- Compose existing write-readiness primitives into one explicit no-provider-call orchestration boundary.
- Keep provider writes denied by default.
- Preserve `SideEffectRecord` as the source of truth for side-effect intent and lifecycle state.
- Preserve `WorkflowRunEvent` as the accepted run-local event history.
- Require explicit policy, approval, idempotency, SideEffect, lifecycle, and report/artifact references.
- Prove the ordering that a later live write path must follow before any provider call is proposed.
- Keep failures deterministic, structured, and redaction-safe.
- Avoid automatic runtime behavior, hidden state reads, or default executor changes.
- Produce an implementation-ready plan for a small helper/service slice.

## 3. Non-Goals

This plan does not authorize:

- implementation in this phase;
- live GitHub pull request comment creation;
- any provider mutation;
- runtime side-effect execution;
- automatic side-effect attempts or completions;
- automatic writes from `LocalExecutor::execute(...)`;
- automatic writes from `execute_with_report(...)`;
- automatic report artifact writing from default executor paths;
- CLI mutation commands, flags, rendering, or export;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- auth material loading or credential management;
- OAuth app behavior, webhook ingestion, or production adapter management;
- RBAC, IdP, enterprise admin controls, quorum approval, or revocation enforcement;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Primitive Baseline

The orchestration helper should reuse these existing boundaries instead of creating parallel models:

- `preflight_adapter_write(...)` for write-readiness validation.
- GitHub PR comment request/response validation for the first write candidate.
- Fixture-backed GitHub PR comment adapter validation for no-provider-call request/response checks.
- GitHub PR comment proposed `SideEffectRecord` composition.
- `SideEffectRecordStore` persistence for proposed records and lifecycle transitions.
- Proposed SideEffect event construction and explicit executor append input helpers.
- `SideEffectApprovalLinkage` validation helpers for approval authority references.
- Store-backed lifecycle transition helpers for attempted/completed/failed records.
- Explicit executor lifecycle event append for attempted/completed/failed transition results.
- WorkReport/report artifact SideEffect citation and referential integrity helpers.
- Report artifact approval-linkage and high-assurance disclosure gates where explicitly supplied.

The helper should not create new canonical source-of-truth models unless a gap is proven during implementation.

## 5. Orchestration Boundary

The orchestration boundary should be a local, explicit helper or service. A future implementation may choose names such as:

- `orchestrate_write_candidate_without_provider_call(...)`
- `compose_write_candidate_governance(...)`
- `WriteAdapterOrchestrationHelper`

The helper should be adapter-neutral where practical, but it may start with the GitHub PR comment candidate if that is the smallest reviewable shape.

It should accept explicit inputs for:

- workflow/run identity;
- actor or system actor;
- adapter identity and requested capability;
- bounded target reference;
- policy decision reference;
- approval or high-assurance approval references when required;
- idempotency binding;
- proposed `SideEffectRecord` or proposed record construction input;
- `SideEffectRecordStore`;
- optional accepted proposed SideEffect event proof;
- optional report/report artifact citation policy;
- redaction metadata and sensitivity;
- dry-run/fixture/provider-candidate mode.

It must not:

- read hidden global state;
- construct a `LocalStateBackend` internally;
- call providers;
- load auth material;
- mutate `WorkflowRun` outside explicit executor append paths;
- append events unless the caller provided an explicit executor/event append input path;
- write report artifacts unless a separately reviewed explicit artifact helper is supplied;
- emit CLI output.

## 6. Recommended First Implementation Slice

Implemented first slice: **write-adapter orchestration helper, no provider calls**.

The first helper should prove one complete local sequence:

1. Validate adapter write preflight.
2. Validate or persist a proposed `SideEffectRecord`.
3. Construct or validate accepted `SideEffectProposed` event proof.
4. Validate approval-side-effect linkage when approval is required.
5. Transition the stored record to `Attempted` through the store-backed lifecycle helper.
6. Optionally append the corresponding `SideEffectAttempted` workflow event through explicit executor input.
7. Produce a provider-call-ready boundary result that states provider invocation is still not performed.
8. Optionally simulate fixture-only success/failure classification without using it as provider proof.
9. Transition to `Completed` or `Failed` only when the input supplies an explicit non-provider outcome reference.
10. Optionally append the completed/failed lifecycle event through explicit executor input.
11. Return bounded report/artifact citation requirements and unresolved gates.

The implementation stops after attempted-state orchestration. Completed/failed no-provider outcome orchestration remains deferred.

## 7. Required Sequence Semantics

The helper should make sequencing visible in its output.

Recommended sequence states:

- `preflight_validated`;
- `proposed_record_persisted`;
- `proposed_event_proven`;
- `approval_linkage_validated`;
- `attempted_record_persisted`;
- `attempted_event_appended` when explicitly requested;
- `provider_call_deferred`;
- `completed_or_failed_record_persisted` only when explicitly supplied as no-provider outcome;
- `completed_or_failed_event_appended` only when explicitly requested;
- `report_citation_requirements_returned`.

Each state should be represented by bounded counts and stable IDs, not raw payloads.

## 8. Approval And High-Assurance Policy

The orchestration helper should treat approval as authority context, not as a side-effect lifecycle state.

Rules:

- Missing required approval fails before attempted transition.
- Denied approval fails before attempted transition.
- Approval-side-effect identity mismatch fails before attempted transition.
- High-assurance approval validation is used only when explicitly supplied or required by the caller's policy.
- Failure must not append attempted/completed events or create report artifact claims.

This plan does not add RBAC, enterprise stewardship, quorum approval, revocation enforcement, or automatic approval attachment.

## 9. Event And Store Ordering

The first implementation should keep store writes and workflow event appends explicit.

Recommended ordering:

1. Persist proposed record.
2. Append or validate accepted proposed event proof.
3. Persist attempted transition.
4. Append attempted event only through explicit executor inputs.
5. Persist completed/failed transition only from explicit no-provider outcome input.
6. Append completed/failed event only through explicit executor inputs.

If a store transition succeeds and event append fails, the helper must return a structured partial-orchestration error or result that discloses the mismatch. It must not hide the mismatch, fabricate event proof, or roll back by mutating store files directly.

## 10. Report And Artifact Citation Policy

The helper should return citation obligations rather than write artifacts by default.

Required citations for a completed orchestration result:

- proposed `SideEffectRecord` ID;
- accepted proposed SideEffect event ID when available;
- attempted `SideEffectRecord` ID when attempted;
- accepted attempted event ID when appended;
- completed or failed `SideEffectRecord` ID when supplied;
- accepted completed or failed event ID when appended;
- policy decision reference;
- approval or high-assurance approval reference when required;
- report artifact integrity policy used, if any.

Report/artifact writes remain separate explicit helpers. This orchestration plan does not change default artifact behavior.

## 11. Error Handling

Errors must use stable codes and avoid raw values.

Candidate error codes:

- `write_orchestration.preflight_failed`;
- `write_orchestration.proposed_record_failed`;
- `write_orchestration.proposed_event_missing`;
- `write_orchestration.approval_linkage_failed`;
- `write_orchestration.lifecycle_transition_failed`;
- `write_orchestration.event_append_failed`;
- `write_orchestration.report_citation_failed`;
- `write_orchestration.provider_call_not_supported`;
- `write_orchestration.partial_state_detected`.

Errors must not include provider payloads, request bodies, comment bodies, URLs, paths, raw spec contents, command output, auth material, headers, private keys, token-like values, or secret-like strings.

## 12. Privacy And Redaction

The helper must stay reference-first:

- cite stable IDs instead of copying payloads;
- use bounded summaries;
- preserve conservative sensitivity;
- require redaction metadata;
- reject secret-like inputs through existing constructors;
- keep Debug and serialization output bounded and redaction-safe.

The helper must not store or copy raw provider payloads, raw GitHub pull request content, raw diffs, raw issue/comment bodies, raw command output, raw CI logs, raw spec contents, environment variable values, auth material, credentials, authorization headers, private keys, or token-like values.

## 13. Test Plan

Future implementation tests should cover:

- valid no-provider orchestration reaches attempted-state boundary;
- proposed record is persisted before attempted transition;
- accepted proposed event proof is required when policy requires it;
- approval-side-effect linkage is validated before attempted transition;
- missing approval fails before attempted transition;
- denied approval fails before attempted transition;
- identity mismatch fails before attempted transition;
- attempted lifecycle record is persisted through store-backed helper;
- attempted workflow event is appended only through explicit executor input;
- completed/failed transitions are not fabricated from fixture output;
- explicit no-provider completed outcome can be persisted and appended when supplied;
- explicit no-provider failed outcome can be persisted and appended when supplied;
- partial state is disclosed if store transition succeeds but event append fails;
- report citation obligations include stable IDs only;
- no provider call is made;
- no auth material is loaded;
- no CLI output, schemas, examples, hosted behavior, or release posture changes are introduced;
- Debug and serialization do not leak raw payloads or secret-like values;
- existing SideEffect, approval linkage, report artifact, WorkReport, adapter preflight, fixture adapter, and executor tests still pass.

## 14. Proposed Implementation Sequence

Recommended small phases:

1. Implement a pure orchestration input/result model with validation only.
2. Compose preflight, proposed record persistence, proposed-event proof, and approval linkage.
3. Compose store-backed attempted transition and optional explicit executor attempted-event append.
4. Add report citation obligation output without writing artifacts.
5. Review.
6. Only after review, consider explicit completed/failed no-provider outcome orchestration.
7. Only after another review, reconsider live sandbox provider-call planning.

The first code phase should stop at attempted-state orchestration unless the implementation remains obviously small.

## 15. Deferred Work

Deferred until separately planned and reviewed:

- live provider calls;
- sandbox smoke writes;
- production credential posture;
- CLI mutation commands;
- workflow schema fields;
- workflow-declared write orchestration;
- automatic runtime side-effect execution;
- automatic report artifact writing from default executor paths;
- automatic evidence attachment for write outcomes;
- explicit missing-citation records;
- hosted/distributed runtime;
- RBAC/IdP/enterprise stewardship;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy.

## 16. Open Questions

- Should the first orchestration helper be adapter-neutral or GitHub PR comment specific?
- Should proposed-event proof be mandatory in v1 or policy-controlled?
- Should attempted-event append happen inside the helper or be returned as an explicit next action?
- How should partial store/event mismatch be represented in a result type without implying rollback?
- Should completed/failed no-provider outcome orchestration be included in the first helper or deferred?
- Which report artifact citation gates are required before any future live write?
- Should orchestration output become a WorkReport citation target later?
- What minimal sandbox policy must exist before a live GitHub PR comment smoke is even planned?

## 17. Final Recommendation

Proceed next to **write-adapter orchestration helper review**, keeping the implementation limited to attempted-state orchestration only.

Do not build provider writes, runtime side-effect execution, CLI mutation behavior, workflow schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, auth material loading, or release posture changes.

## 18. Dogfood Governance

This planning phase was governed by the repo-local self-governance workflow:

- workflow: `dg/d`;
- run: `run-1783272407967559000-2`;
- approval: `approval/run-1783272407967559000-2/planning-approved`;
- approval actor: `user/delegated-maintainer`;
- scope: planning/docs only;
- required validation: `npm run check:docs`.
