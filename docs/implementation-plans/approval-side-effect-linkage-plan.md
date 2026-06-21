# Approval SideEffect Linkage Plan

Status: Validation-only helper implemented in [SideEffect Approval Linkage Report](../concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md). This plan follows the accepted SideEffect core model, explicit proposed/denied/skipped executor event append path, SideEffect persistence/discovery helpers, executor SideEffect discovery opt-in, and report artifact SideEffect referential integrity review. It does not implement runtime side-effect execution, write-capable adapters, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS can now model, persist, discover, cite, and artifact-check SideEffect records without executing writes. It also has local approval request and decision events that can pause, resume, or fail a governed run.

The next safety question is not how to write. The next question is how future SideEffect records should prove their authority relationship to approval requests and decisions when human approval is required.

Approval must remain authority context, not lifecycle state. A granted approval may permit a future side-effect attempt, but it does not prove that a provider mutation happened. A denied approval may explain why a SideEffect was denied, but it does not replace the SideEffect record.

This plan defines the future linkage boundary between approval decisions and SideEffect records. The first implementation should be a validation-only helper that checks already-existing SideEffect records against already-existing workflow approval events. It should not create approvals, create SideEffect records, mutate workflow state, append events, write artifacts, run adapters, or execute side effects.

## 2. Goals

- Define how SideEffect authority references should link to approval requests and decisions.
- Preserve the separation between approval authority and SideEffect lifecycle.
- Preserve deterministic workflow execution and replay.
- Validate linkage using existing workflow events and SideEffect records.
- Fail closed when a SideEffect claims approval authority that cannot be proven.
- Keep errors stable and non-leaking.
- Keep reports and artifacts citation-based.
- Prepare for high-assurance approval controls before write-capable adapters.
- Preserve current approval behavior and current SideEffect behavior.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable adapters;
- provider mutation;
- approval evidence attachment;
- creating, mutating, or deleting approval requests;
- changing `LocalExecutor::decide_approval(...)`;
- changing approval pause/resume semantics;
- multi-party approval, quorum approval, or role-based approval enforcement;
- external identity provider integration;
- approval revocation or expiration enforcement changes;
- automatic SideEffect discovery;
- automatic report artifact writing;
- workflow schema fields;
- CLI rendering or commands;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented approval baseline:

- workflow events can request approval;
- workflow events can grant or deny approval;
- approval requests include immutable run identity, step ID, skill ID, skill version, requester, correlation ID, idempotency key where available, reason, timestamp, and optional expiration metadata;
- approval decisions include approval ID, actor, timestamp, decision kind, reason, and correlation ID;
- approved runs can resume;
- denied approvals fail closed;
- approval decisions after terminal state are rejected;
- duplicate approval decisions are rejected;
- approval projection mismatch fails closed.

Implemented SideEffect baseline:

- `SideEffectRecord` models lifecycle, authority, target, capability, idempotency, references, and immutable run identity;
- `SideEffectAuthority` can represent `RequiresApproval`, `ApprovedByHuman`, and `DeniedByApproval`;
- `SideEffectAuthority` contains approval references as stable `SideEffectReference` values;
- proposed, denied, and skipped SideEffect events can be explicitly appended before local skill invocation;
- SideEffect records can be explicitly persisted and discovered;
- WorkReports can cite SideEffect IDs;
- report artifacts can explicitly validate SideEffect citation referential integrity.

Current gap:

- a SideEffect record can carry an approval reference, but there is no helper that proves the referenced approval request or decision exists in the run event history and matches the SideEffect authority decision.

## 5. Source-Of-Truth Boundaries

Workflow events are the source of truth for approval requests and decisions.

SideEffect records are the source of truth for side-effect intent, authority, lifecycle, target, capability, idempotency, references, and outcome.

Audit events are projections. They may help humans inspect governance, but they should not be the authoritative source for linkage validation.

WorkReports and report artifacts are governed handoff artifacts. They cite approvals and SideEffects; they do not prove linkage by themselves.

EvidenceReference remains a citation pointer. It should not be used as approval payload storage or SideEffect record storage.

## 6. Linkage Problem Statement

For future writes, a SideEffect record that says it was approved by a human must be backed by a concrete approval decision from the same workflow run. Without this linkage, a report or artifact could cite a valid SideEffect record and a valid approval reference separately, while leaving the critical relationship unproven.

The linkage check should answer only:

```text
Do this SideEffect record's approval authority references resolve to approval events in the same immutable workflow run, and do the referenced approval decisions match the SideEffect authority decision?
```

It must not claim:

- the SideEffect was executed;
- provider state changed;
- the approver had role-based authority beyond the current local approval model;
- evidence was sufficient for approval;
- the approval packet was complete;
- all relevant SideEffects were discovered;
- a WorkReport artifact was written;
- a write-capable adapter is safe to enable.

## 7. Recommended First Boundary

Recommended first implementation: an explicit validation-only helper.

Candidate shape:

```rust
pub struct SideEffectApprovalLinkageInput<'a> {
    pub run: &'a WorkflowRun,
    pub side_effect_records: &'a [SideEffectRecord],
    pub require_approval_references_for_requires_approval: bool,
    pub require_decision_for_approved_or_denied: bool,
}

pub struct SideEffectApprovalLinkageResult {
    // bounded counts only
}

pub fn validate_side_effect_approval_linkage(
    input: SideEffectApprovalLinkageInput<'_>,
) -> Result<SideEffectApprovalLinkageResult, WorkflowOsError>
```

Alternative future shape:

```rust
pub fn validate_side_effect_approval_linkage_from_store(
    side_effect_store: &impl SideEffectRecordStore,
    run: &WorkflowRun,
    policy: SideEffectApprovalLinkagePolicy,
) -> Result<SideEffectApprovalLinkageResult, WorkflowOsError>
```

The first implementation should prefer already-loaded records and a borrowed run. Store-backed linkage can follow after review if a caller needs it.

## 8. Linkage Policy

The first helper should validate only `SideEffectRecord.authority().approval_references`.

Recommended policy:

- `ApprovedByHuman` requires at least one approval decision reference.
- `DeniedByApproval` requires at least one approval decision reference.
- `RequiresApproval` should allow either an approval request reference or explicit no-decision-yet posture, depending on caller policy.
- `AllowedByPolicy`, `DeniedByPolicy`, `DeniedByCapability`, `DeniedByKillSwitch`, `DeniedByValidation`, `Unsupported`, and `NotEvaluated` should not require approval references.
- Approval references must resolve to approval events in the supplied run.
- A reference used for `ApprovedByHuman` must resolve to a granted approval decision.
- A reference used for `DeniedByApproval` must resolve to a denied approval decision.
- A reference used for `RequiresApproval` must resolve to an approval request, and the helper should not treat that request as granted.
- Duplicate references should de-duplicate deterministically while retaining bounded duplicate counts.

This policy should be caller-visible and testable. It should not be hidden inside report generation, artifact writing, or executor execution paths in the first slice.

## 9. Identity Validation

The helper must validate full immutable run identity.

Each SideEffect record must match the supplied run:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID.

Approval events already carry workflow/run identity through the event envelope and approval request payload. The helper must not trust approval ID strings alone.

Recommended additional checks:

- If a SideEffect record is step-scoped, its step ID should match the referenced approval request step ID when the approval request is step-scoped.
- If a SideEffect record is skill-scoped, its skill ID and skill version should match the referenced approval request skill identity when present.
- Correlation ID may be counted or surfaced as bounded metadata later, but should not be required in the first helper unless existing runtime semantics already guarantee it.

## 10. Approval Reference Shape

The current generic reference type stores `SideEffectReferenceKind::ApprovalDecision` plus a bounded reference string. The first helper should define accepted reference text as an approval ID that can be matched against workflow approval events.

This is intentionally narrower than a future approval-decision object ID. It fits the current local runtime, where approval request and decision events share `approval_id`.

Do not fabricate approval IDs. Do not infer approval IDs from prose, report sections, audit summaries, policy reason text, or provider payloads.

Future phases may introduce a stronger approval decision reference type if needed.

## 11. Lifecycle And Authority Rules

Approval linkage validates authority context. It does not change lifecycle.

Recommended lifecycle interactions:

- `Proposed` with `RequiresApproval` may be valid if the approval request exists and no grant has been claimed.
- `Denied` with `DeniedByApproval` must reference a denied approval decision.
- `Skipped` with approval authority should be allowed only when the skip reason and authority posture are compatible; first helper may validate references without enforcing skip semantics.
- `Attempted`, `Completed`, and `Failed` remain model vocabulary and future execution semantics. If records with those lifecycle states are supplied, the helper should validate any approval linkage but must not enable execution.

Do not introduce an `Approved` lifecycle state.

## 12. Error Handling

Errors must use stable non-leaking codes.

Candidate codes:

- `side_effect_approval_linkage.identity_mismatch`
- `side_effect_approval_linkage.approval_missing`
- `side_effect_approval_linkage.decision_missing`
- `side_effect_approval_linkage.decision_kind_mismatch`
- `side_effect_approval_linkage.step_mismatch`
- `side_effect_approval_linkage.skill_mismatch`
- `side_effect_approval_linkage.record_invalid`
- `side_effect_approval_linkage.run_invalid`

Errors must not leak:

- SideEffect IDs;
- approval IDs;
- workflow IDs;
- run IDs;
- step IDs;
- skill IDs;
- spec hashes;
- target references;
- summaries;
- reasons;
- redaction metadata;
- provider payloads;
- command output;
- parser payloads;
- paths;
- snippets;
- credentials;
- tokens;
- authorization headers;
- private keys.

## 13. Privacy And Redaction

The linkage helper must inspect only typed IDs, bounded references, lifecycle/authority enums, and immutable identity fields.

It must not copy:

- approval reasons;
- SideEffect summaries;
- SideEffect target references;
- raw provider payloads;
- raw CI logs;
- raw Jira bodies or comments;
- raw GitHub file contents;
- raw command output;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

Input and result `Debug` output should expose only bounded counts and booleans.

## 14. Report And Artifact Relationship

WorkReports may cite both approval decisions and SideEffect records. That is useful, but citation co-presence is not proof of linkage.

The first linkage helper should remain separate from:

- terminal report generation;
- executor report-bearing execution;
- SideEffect discovery;
- report artifact writing;
- report artifact SideEffect referential integrity validation.

Future composition may run:

1. SideEffect discovery.
2. Approval-side-effect linkage validation.
3. Report generation.
4. Report artifact SideEffect referential integrity validation.
5. Explicit artifact write.

That composition must be planned separately before it becomes automatic.

## 15. Runtime Integration Boundary

Do not change executor behavior in the first implementation.

The helper should not:

- call `LocalExecutor::decide_approval(...)`;
- append workflow events;
- emit audit events;
- emit observability events;
- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- mutate approval projections;
- create or update SideEffect records;
- read or write provider state;
- write report artifacts;
- expose CLI output.

Approval linkage failure should be surfaced as a structured helper error. A future report path may translate that into a report-generation error, but it must not become a misleading user project diagnostic.

## 16. Test Plan

Future implementation should add focused tests for:

- SideEffect `ApprovedByHuman` with matching granted approval succeeds.
- SideEffect `DeniedByApproval` with matching denied approval succeeds.
- SideEffect `RequiresApproval` with matching approval request succeeds under request-only policy.
- missing approval reference fails closed.
- missing approval decision fails closed when decision is required.
- granted/denied decision mismatch fails closed.
- SideEffect/run identity mismatch fails without leaking values.
- approval/run identity mismatch fails without leaking values if constructible.
- step mismatch fails without leaking values.
- skill or skill version mismatch fails without leaking values.
- duplicate approval references de-duplicate deterministically.
- unrelated approval events do not satisfy linkage.
- approval request without grant does not satisfy `ApprovedByHuman`.
- denied approval does not satisfy `ApprovedByHuman`.
- granted approval does not satisfy `DeniedByApproval`.
- records without approval authority remain accepted when policy allows.
- errors do not leak approval IDs, SideEffect IDs, reasons, target references, paths, or token-like values.
- Debug output exposes counts only.
- existing approval, SideEffect, WorkReport, artifact, executor, runtime, evidence, adapter, hook, local-check, and docs tests still pass.

## 17. Proposed Implementation Sequence

Recommended small phases:

1. Add validation-only `SideEffectApprovalLinkageInput`, policy, result, and helper over a borrowed `WorkflowRun` and already-loaded `SideEffectRecord` values.
2. Add focused linkage tests and redaction tests.
3. Review the helper.
4. Plan store-backed linkage only if a concrete caller needs it.
5. Plan report/artifact composition only after helper review.
6. Continue to defer runtime side-effect execution and write-capable adapters until approval linkage and policy-gated write planning are reviewed.

## 18. Deferred Work

Deferred:

- approval evidence attachment;
- approval packet model;
- multi-party approval;
- quorum approval;
- role-based approval authority;
- requester/approver separation enforcement;
- approval revocation;
- approval expiration enforcement changes;
- external identity provider integration;
- store-backed linkage;
- automatic report/artifact linkage validation;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters;
- provider mutation;
- schemas;
- CLI behavior;
- examples;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 19. Final Recommendation

Implemented phase: **approval-side-effect linkage validation helper, model-only/reference-only**.

The implementation started with a borrowed-run and already-loaded-record helper. It is the smallest useful safety boundary before any future write-capable adapter planning because it proves that SideEffect records claiming human approval authority are tied to concrete approval events from the same immutable run.

The validation helper is accepted with non-blocking follow-ups in [SideEffect Approval Linkage Review](../concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REVIEW.md). Composition planning is documented in [Approval SideEffect Linkage Composition Plan](approval-side-effect-linkage-composition-plan.md).

Recommended next phase: **store-backed SideEffect approval linkage helper review**.

The store-backed helper is implemented in [SideEffect Approval Linkage Store-Backed Report](../concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REPORT.md). Do not build runtime side-effect execution, write-capable adapters, automatic report artifact writing, automatic report/artifact linkage validation, approval evidence attachment, workflow schema fields, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes before that helper is reviewed.
