# High-Assurance Approval Controls Plan

Status: Core model implemented; blocker fix accepted; first pure runtime validation helper implemented; first opt-in executor enforcement slice implemented; WorkReport disclosure, disclosure discovery, executor/report disclosure integration, explicit report artifact disclosure gating, and the first internal workflow artifact requirement model/policy-mapping slice implemented. Workflow-declared high-assurance artifact requirement planning is documented in [Workflow-Declared High-Assurance Artifact Requirement Plan](workflow-declared-high-assurance-artifact-requirement-plan.md), with the model slice reported in [Workflow-Declared High-Assurance Artifact Requirement Model Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_MODEL_REPORT.md). Automatic high-assurance enforcement, workflow-declared controls, schemas, CLI behavior, write-capable adapters, RBAC/IdP, quorum approval, hosted behavior, side-effect execution, and release posture changes are not implemented.

## 1. Executive Summary

Workflow OS already has local event-sourced approval requests and decisions. A step can pause before skill invocation, a grant resumes execution, a denial fails closed, and approval request projections can be rebuilt from workflow events.

Policy-effect enforcement now closes the P0 false-governance gap for supported policy effects. SideEffect approval linkage can prove that selected SideEffect records cite matching approval events. The next safety question is how approval should mature before Workflow OS permits write-capable adapters or sensitive external actions.

High-assurance approval controls define that future approval posture. The model should make sensitive actions look less like a single "yes" button and more like a governed authority checkpoint with explicit requester, approver, evidence context, policy context, expiration/revocation semantics, auditability, and report disclosure.

This plan led to the core implementation documented in [High-Assurance Approval Control Core Model Report](../concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_REPORT.md). The model blocker fix is accepted in [High-Assurance Approval Control Core Model Blocker Fix Review](../concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_BLOCKER_FIX_REVIEW.md), the opt-in runtime enforcement boundary is planned in [High-Assurance Approval Runtime Enforcement Plan](high-assurance-approval-runtime-enforcement-plan.md), and the first pure runtime validation helper is implemented in [High-Assurance Approval Runtime Validation Helper Report](../concepts/HIGH_ASSURANCE_APPROVAL_RUNTIME_VALIDATION_HELPER_REPORT.md). The implemented slices add domain-neutral model types, stable validation errors, serde support, redaction-safe `Debug` behavior, and a pure in-memory helper for explicit decision validation. They do not add executor-integrated high-assurance approval enforcement, write-capable adapters, RBAC, IdP integration, quorum approval, schemas, CLI behavior, hosted behavior, side-effect execution, or release posture changes.

## 2. Goals

- Define the high-assurance approval control boundary before write-capable adapters.
- Preserve current local approval semantics.
- Preserve event-log source-of-truth behavior.
- Make approval authority explicit and auditable.
- Require enough approval context for later review.
- Prepare for separation of requester and approver.
- Prepare for approval expiration and revocation semantics.
- Prepare for evidence-required approval packets.
- Preserve side-effect authority/lifecycle separation.
- Preserve final WorkReport disclosure of approvals requested, granted, denied, expired, revoked, skipped, or deferred.
- Keep errors stable and non-leaking.
- Keep the first implementation path small and local.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- write-capable adapters;
- provider mutations;
- runtime side-effect execution;
- broad policy DSLs;
- RBAC, IdP, SSO, SCIM, groups, teams, or enterprise directory integration;
- quorum approval implementation;
- role-bound authority enforcement implementation;
- external approval systems;
- approval UI;
- background timers;
- hosted or distributed runtime behavior;
- workflow schema changes;
- CLI behavior changes;
- examples;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- safety-critical certification claims;
- release posture changes.

## 4. Current Baseline

Implemented today:

- approval policy effects are typed and validated;
- approval-gated local steps pause before skill invocation;
- `ApprovalRequested` is written before approval projection storage;
- approval request events carry immutable run identity, step identity, skill identity, requester, correlation ID, reason, idempotency key, and optional expiration metadata;
- grants append `ApprovalGranted`, resume the run, and then allow invocation;
- denials append `ApprovalDenied` and fail closed;
- duplicate decisions and decisions after terminal states are rejected;
- approval projections are rebuildable from event history;
- WorkReports can cite approval decisions by stable vocabulary;
- SideEffect records can carry approval authority references;
- SideEffect approval linkage helpers can validate approval references against run events.

Not implemented today:

- requester/approver separation;
- self-approval prevention;
- role-bound or steward-bound authority checks;
- quorum or multi-party approval;
- evidence-required approval packets;
- approval context completeness validation;
- approval expiration enforcement;
- approval revocation;
- escalation on expired approval;
- approval evidence attachment;
- workflow-declared high-assurance approval controls;
- enterprise stewardship/admin enforcement;
- write-capable adapter use of approval controls.

## 5. Problem Statement

A single local approval decision is enough for current v0 local workflows, but it is not enough for sensitive or irreversible actions.

Before external writes, Workflow OS needs a stronger answer to:

- who requested approval;
- who approved it;
- whether the same actor can approve their own request;
- what policy required the approval;
- what evidence was available to the approver;
- what side effect or capability the approval authorized;
- whether the approval expired before use;
- whether the approval was revoked before use;
- whether the final report disclosed the approval posture honestly.

Without this boundary, future write-capable adapters could cite an approval ID while leaving authority, context, evidence sufficiency, and decision freshness ambiguous.

## 6. Source-Of-Truth Boundaries

Workflow events remain the source of truth for approval requests and approval decisions.

Approval projections remain rebuildable lookup caches. They must not authorize decisions unless backed by matching workflow events.

Policy decisions define why approval is required. They do not replace approval decisions.

SideEffect records define side-effect intent, authority, lifecycle, target, capability, idempotency, references, and outcomes. Approval is authority context, not side-effect lifecycle.

EvidenceReference values cite supporting material. They must not store approval packets or raw payloads.

WorkReports disclose approval posture and cite approval references. They must not become the source of truth for approval decisions.

Audit events are operational projections. They help review and inspection, but approval event history remains authoritative.

## 7. High-Assurance Approval Contract

A future high-assurance approval control should define:

- control ID;
- control version;
- protected action or capability class;
- protected side-effect class where applicable;
- requester identity requirements;
- approver identity requirements;
- whether requester and approver must differ;
- required approval count;
- required evidence references;
- required policy decision references;
- required SideEffect references when authorizing a side effect;
- expiration policy;
- revocation policy;
- escalation policy;
- denial behavior;
- report disclosure requirements;
- sensitivity and redaction policy.

The first model should remain domain-neutral. GitHub, Jira, CI, filesystem, deployment, security, or finance-specific controls belong in adapters, examples, domain packs, or future templates.

## 8. Approval Packet Requirements

High-assurance approval should eventually operate on an approval packet rather than only a reason string.

Candidate packet fields:

- approval request ID;
- workflow/run identity;
- step and skill identity;
- requested action/capability;
- requester actor or system actor;
- approval policy reference;
- policy decision references;
- side-effect references, if approval authorizes side effects;
- evidence reference IDs;
- validation diagnostic references;
- local check result references;
- known limitations;
- risks;
- incomplete/deferred work disclosures;
- bounded operator note;
- sensitivity and redaction metadata.

Rules:

- packets cite references instead of copying payloads;
- raw provider payloads are forbidden;
- raw command output is forbidden;
- raw spec contents are forbidden;
- secret-like values are forbidden;
- missing evidence must be explicit;
- model-generated opinions cannot replace deterministic required evidence.

## 9. Requester And Approver Separation

Requester/approver separation is the first practical high-assurance control to plan after the current local approval model.

Future behavior should support:

- same-actor approval allowed for low-risk local profiles;
- same-actor approval rejected for high-assurance controls;
- system actor request with human approver;
- human requester with different human approver;
- agent requester with human approver;
- explicit disclosure when identity assurance is local-only and not IdP-backed.

The first implementation should not claim strong identity. In v0, actor IDs are auditable local identifiers, not proof of external identity.

## 10. Expiration And Revocation

Current approval requests store expiration metadata but do not run background timers.

High-assurance planning should define:

- whether an approval must be used before expiration;
- whether expired approval fails closed, cancels, or escalates;
- whether approval can be revoked before use;
- who can revoke an approval;
- whether revocation after use is report-only or triggers compensating work;
- what event types are required for expiration and revocation;
- how WorkReports disclose expired or revoked approvals.

No silent expiration is allowed. Expiration and revocation must be explicit events if implemented.

## 11. Relationship To Policy Effects

Policy effects now follow the invariant:

```text
Declared policy effects must be enforced or rejected.
```

High-assurance approval controls must preserve that invariant.

Future policy effects related to approval should be rejected until the runtime implements them. Examples that should not be silently accepted today:

- `require_separate_approver`;
- `require_quorum`;
- `require_role`;
- `require_steward`;
- `require_approval_evidence`;
- `approval_expires`;
- `approval_revocable`;
- `nuclear_key`.

Those may become supported vocabulary only when validation and runtime behavior are implemented together.

## 12. Relationship To Governance Profiles

Governance profiles are the product-level separation point:

- `observe_and_report` may avoid approval pauses while still standardizing evidence, skipped-check disclosure, side-effect disclosure, audit posture, and reports.
- `agent_assisted_gated` may allow an agent to provide required detail or evidence where deterministic validation can check it.
- `human_approval_gated` pauses selected checkpoints for human decisions.
- `strict_enterprise` should eventually require steward-defined gates, human authority, evidence, policy, approval, audit, and report closure.

High-assurance approvals belong primarily to strict or sensitive profiles. They should not make the single-user local fast path feel unnecessarily heavy unless the user opts into that posture.

## 13. Relationship To Side Effects And Writes

High-assurance approvals must land before serious write-capable adapter work.

For future writes:

- credentials must not imply authority;
- approval must authorize a proposed side effect or protected capability;
- approval must not imply the side effect completed;
- side-effect lifecycle remains separate from approval authority;
- approval linkage must prove references against workflow events;
- idempotency must prevent duplicate mutation attempts;
- WorkReports must disclose side effects and approvals together.

Write-capable adapters should remain blocked until a reviewed implementation composes policy effects, approval controls, SideEffect records, idempotency, audit, evidence, and reports.

## 14. Relationship To WorkReports

WorkReports should eventually disclose:

- approvals requested;
- approvals granted;
- approvals denied;
- approvals expired;
- approvals revoked;
- approvals skipped or not applicable;
- approval evidence considered;
- approval limitations;
- side effects authorized or denied by approval.

Reports should cite approval IDs, policy decision IDs, SideEffect IDs, EvidenceReference IDs, validation references, and local check references. Reports must not copy raw approval payloads, raw provider data, raw command output, raw specs, or secrets.

## 15. Recommended First Implementation Sequence

Recommended small phases:

1. **High-assurance approval control core model.**
   Implemented as model types and validation only. No runtime behavior.
2. **Approval packet model.**
   Add bounded reference-only packet model for approval context. No evidence fetching or attachment.
3. **Local requester/approver separation helper.**
   Add explicit validation helper over existing approval request/decision events. No IdP or RBAC.
4. **Approval expiration/revocation event planning.**
   Plan event vocabulary and failure semantics before implementation.
5. **Approval WorkReport disclosure integration.**
   Ensure reports can cite high-assurance approval posture without copying payloads.
6. **Write-adapter readiness review.**
   Reassess whether the approval, side-effect, policy, idempotency, evidence, audit, and report layers are ready for a first write adapter.

Do not start with runtime side-effect execution.

## 16. Future Test Plan

Future implementation tests should cover:

- valid high-assurance approval control model;
- invalid control IDs and versions;
- required protected capability/action fields;
- requester and approver separation requirement;
- local-only identity disclosure;
- required evidence references;
- missing required evidence fails closed;
- approval packet forbids raw payloads;
- approval packet citations are bounded;
- approval packet Debug/serialization are redaction-safe;
- same-actor approval accepted only when policy allows it;
- same-actor approval rejected for high-assurance controls;
- expired approval cannot authorize protected action when expiration is enforced;
- revoked approval cannot authorize protected action when revocation is enforced;
- SideEffect approval linkage still separates authority from lifecycle;
- WorkReport disclosure remains reference-only;
- validation errors use stable codes and do not leak IDs, reasons, snippets, paths, provider payloads, command output, tokens, or secrets;
- existing approval, policy, SideEffect, WorkReport, and executor tests continue to pass.

## 17. Open Questions

- What is the smallest useful control model before RBAC exists?
- Should requester/approver separation be model-only first or helper-first?
- Should expiration be evaluated at approval decision time, protected action time, or both?
- Should revocation after side-effect attempt be report-only or block downstream steps?
- How much approval packet completeness should be enforced before write-capable adapters?
- Should high-assurance controls be tied to governance profiles or workflow-local policy references first?
- What is the first sensitive action class that needs this boundary?
- Should the first implementation use existing approval IDs or introduce stronger approval decision reference IDs?
- How should local-only actor identity be disclosed in reports?
- What exact readiness gate must pass before first provider write planning?

## 18. Final Recommendation

The model-only implementation is complete. The next prompt should be: **High-assurance approval control core model review**.

That review should verify the model remains domain-neutral, redaction-safe, and scope-clean before any approval packet, requester/approver helper, expiration/revocation, or write-readiness work begins.

It must still not build:

- write-capable adapters;
- provider mutations;
- runtime side-effect execution;
- RBAC or IdP integration;
- quorum approval;
- approval UI;
- schemas;
- CLI behavior;
- examples;
- hosted behavior;
- Level 3/4 autonomy.
