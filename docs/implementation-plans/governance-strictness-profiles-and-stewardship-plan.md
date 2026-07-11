# Governance Strictness Profiles And Stewardship Plan

Status: first local disclosure model implemented in [Governance Strictness Profile Disclosure Model Report](../concepts/GOVERNANCE_STRICTNESS_PROFILE_DISCLOSURE_MODEL_REPORT.md).

The implemented slice adds typed local profile vocabulary and wires the
existing first-run `observe_and_report` disclosure through that vocabulary.
Executor approval behavior, admin controls, RBAC, IdP integration, hosted
policy enforcement, workflow schema fields, write-capable adapters, and
enterprise stewardship remain unimplemented.

## 1. Executive Summary

Workflow OS needs a clear separation between local automation preferences and enterprise governance stewardship.

Many single local users will want the fastest possible mode: let the agent execute freely, avoid approval pauses, and use Workflow OS primarily to standardize evidence, skipped-check disclosure, side-effect disclosure, audit posture, and final reporting. That is a valid product mode.

Enterprise teams will need a different layer: admins or workflow stewards define which workflows may run observe-only, which gates are required, which gates may be satisfied by agent-provided evidence, which gates require human approval, and which actions are blocked or escalated at the department or company level.

This plan does not implement profiles, admin controls, RBAC, IdP integration, notifications, write-capable adapters, hosted behavior, or policy enforcement changes. It records the roadmap separation point.

## 2. Product Thesis

```text
Local users choose speed.
Organizations assign stewardship.
Workflow OS records which mode governed the work.
```

No-approval operation does not mean no governance. It means the governance posture is report/evidence/audit-first rather than approval-gated.

At the same time, enterprise adoption cannot rely on every user deciding their own governance posture forever. A future team or company deployment needs governed stewardship over profiles, gates, approvals, agents, evidence requirements, side effects, reports, and workflow promotion.

## 3. Initial Governance Profiles

Future onboarding and workflow creation should support profiles like:

| Profile | Intended Use | Prompt/Block Behavior |
| --- | --- | --- |
| `observe_and_report` | Single-user or early adoption where speed matters most. | Do not require approval by default; disclose evidence/checks/side effects/risks/final report posture. |
| `agent_assisted_gated` | Agent may provide required detail/evidence to satisfy selected gates. | Block only when required evidence/check/report details are missing or invalid. |
| `human_approval_gated` | Sensitive local/team workflows where a human must approve selected checkpoints. | Pause at configured approval gates. |
| `strict_enterprise` | Future organization-managed governance posture. | Enforce steward-defined gates, human authority, required evidence, policy, approval, audit, and report closure. |

The names may change during implementation. The separation should not.

## 4. Single-User Local Mode

Single-user local Workflow OS should make the fast path legitimate:

- agent executes work;
- Workflow OS validates project/spec posture;
- Workflow OS records evidence/check posture where available;
- Workflow OS discloses skipped checks and unsupported side effects;
- Workflow OS produces report-ready context or WorkReports where implemented;
- approvals may be disabled or advisory when the user explicitly chooses a non-blocking profile.

This mode must still fail closed for malformed specs, secret-like values in unsafe fields, unsupported Level 3/4 claims, invalid references, and other deterministic safety violations.

## 5. Enterprise Stewardship Mode

Enterprise Workflow OS should eventually add an admin/steward layer that can define:

- allowed governance profiles by repo, workflow, team, department, or risk class;
- who owns a workflow and who can approve changes to it;
- which gates require human approval;
- which gates may accept agent-provided evidence;
- whether agents can self-report or must cite external evidence/checks;
- which workflows require final WorkReports;
- which side effects require approval before attempted execution;
- which agents/tools/harnesses may operate under which scoped authority;
- escalation owners and review obligations;
- workflow promotion, deprecation, and retirement rules.

This is a future capability and should not be claimed by the local preview.

## 6. Relationship To Existing Roadmap

- Existing-repo onboarding should expose the profile choice without making users hand-author YAML.
- Scaffold field operationalization should disclose which fields are enforced, validated, disclosed, advisory, or deferred under the selected profile.
- Workflow discovery/catalog governance should recommend profile changes when observed work crosses risk or authority boundaries.
- High-assurance approval controls should provide the future machinery for strict or sensitive enterprise profiles.
- Side-effect boundaries should determine when report-only disclosure is insufficient and approval/policy gates become mandatory.
- Future store/catalog work should persist stewardship decisions, ownership, profile assignments, promotion status, and lifecycle history.

## 7. Non-Goals

- No current RBAC or IdP integration.
- No hosted admin console.
- No department/company-wide policy enforcement in v0.
- No automatic workflow promotion or mutation.
- No write-capable adapter authorization.
- No agent swarm or recursive-agent framing.
- No Level 3/4 autonomy enablement.
- No claim that observe-only mode is enterprise control.

## 8. First Implementation

The first implementation is local and explicit:

1. Add a typed governance profile concept in `workflow-core`.
2. Default first-run posture to the conservative local `observe_and_report`
   profile.
3. Disclose the active profile and posture through existing first-run human
   and JSON output.
4. Do not change executor approval behavior.
5. Add focused tests that profile labels and disclosure posture are stable and
   do not overclaim enforcement.

The first implementation does not introduce admin controls. It creates the
vocabulary and disclosure surface needed before enterprise stewardship can be
designed safely.
