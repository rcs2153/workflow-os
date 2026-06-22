# Scaffold Field Operationalization Plan

Status: In progress. The first implementation slice, first-run governance field posture output, is implemented in [First-Run Governance Field Posture Report](../concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REPORT.md) and accepted with non-blocking follow-ups in [First-Run Governance Field Posture Review](../concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REVIEW.md). The ownership/escalation check slice is implemented in [Ownership And Escalation Check Report](../concepts/OWNERSHIP_ESCALATION_CHECK_REPORT.md) and accepted with non-blocking follow-ups in [Ownership And Escalation Check Review](../concepts/OWNERSHIP_ESCALATION_CHECK_REVIEW.md). The first-run spec-field coverage check is implemented in [Spec Field Coverage Check Report](../concepts/SPEC_FIELD_COVERAGE_CHECK_REPORT.md), following [Spec Field Coverage Check Plan](spec-field-coverage-check-plan.md). Workflow discovery integration and catalog/store planning remain future work.

## 1. Executive Summary

Workflow OS scaffolds and specs already contain rich governance fields: ownership, lifecycle, autonomy, triggers, steps, mappings, policy requirements, approvals, retry/escalation behavior, audit, observability, evidence posture, side-effect posture, checks, handoffs, and reports.

Those fields cannot remain decorative as Workflow OS moves toward more automation. A seamless user experience depends on the kernel progressively turning scaffolded YAML into governed behavior, while being honest about what is currently enforced, what is validated, what is disclosed in reports, and what remains advisory or deferred.

This plan defines the next product lane:

```text
Scaffolded fields become governed obligations.
```

The first implementation should not add broad automation. It should make the existing scaffold and first-run path explain and check the governance posture of the fields users are already seeing.

## 2. Goals

- Ensure every important scaffold/spec field has an explicit operational posture.
- Make first-run onboarding feel guided and opinionated rather than like raw YAML authoring.
- Surface ownership, escalation, approval, evidence, check, side-effect, handoff, and report expectations immediately.
- Preserve the current boundary that agents/humans execute and Workflow OS governs.
- Avoid letting rich scaffold fields become ignored metadata.
- Create workflow/check lanes that help users maintain workflow definitions over time.
- Keep missing or advisory fields visible in first-run reports and future checks.
- Prepare for future workflow catalog/store governance without requiring it now.

## 3. Non-Goals

Do not implement in this lane:

- automatic workflow generation or registration;
- automatic command execution;
- automatic local check execution;
- write-capable adapters;
- provider mutations;
- workflow schema changes without separate planning;
- hosted collaboration registry;
- RBAC, IdP, org-directory sync, paging, or notification integrations;
- production escalation routing;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- replacement of deterministic validation with model self-review.

## 4. Field Operationalization Principle

Every scaffolded field should be classified as one of:

| Posture | Meaning |
| --- | --- |
| Enforced | Runtime or validation blocks unsafe or incomplete use. |
| Validated | Deterministic validation checks shape/reference/consistency but does not execute behavior. |
| Disclosed | First-run, WorkReport, or inspection output explicitly states the field's current meaning and gaps. |
| Advisory | The field guides humans/agents but does not affect validation or runtime behavior yet. |
| Deferred | The field is intentionally reserved for a future accepted phase. |

No rich field should be silently ignored. If the kernel cannot enforce it yet, the user should see that in validation, first-run output, or the final report.

## 5. Current Field Inventory

| Field Area | Examples | Current Posture | Target Near-Term Posture |
| --- | --- | --- | --- |
| Project identity | `project.id`, `project.name`, schema version | Validated | Keep enforced by validation. |
| Ownership | `owner.owning_team`, `owner.maintainer`, `owner.escalation_contact` | Parsed; lifecycle warns | Add ownership/escalation posture checks and first-run disclosure. |
| Lifecycle | `experimental`, `stable`, `deprecated` | Warning for experimental/deprecated | Add stale/orphan workflow checks later. |
| Autonomy | `autonomy_level`, `disabled_by_default` | Validated for Level 3/4 safety | Keep validation; disclose autonomy posture in first-run. |
| Triggers | manual/file/schedule/external_event | Required by validation; not executed | Disclose non-executed trigger posture. |
| State model | inline/reference states | Parsed; future runtime use | Disclose advisory state-model posture until runtime binding exists. |
| Steps | ordered steps, skill refs | Validated and sequential executor uses ordered steps | Continue runtime enforcement for supported local paths. |
| Input/output mappings | field/literal/config refs | Parsed; partially used by local executor paths | Add mapping coverage checks before broader automation. |
| Policy requirements | workflow and step policy refs | Validated and used by local executor policy checks where supported | Expand report/first-run disclosure of policy obligations. |
| Approval requirements | workflow requirements, step approval policy, sensitivity | Validated and runtime approval pause exists | Add owner/approver/escalation consistency checks. |
| Retry/escalation | retry policy refs, escalation policy refs, terminal behavior | Validated; sequential retry/escalation paths exist | Disclose configured escalation owner/contact posture. |
| Timeout/cancellation | timeout, timeout policy, cancellation behavior | Validated in selected cases | Add explicit first-run/posture reporting. |
| Audit/observability | required events, metrics, tracing | Required by workflow validation; local events exist | Add gap disclosure between declared requirements and implemented sinks. |
| Skill contracts | input/output fields, required fields | Validated | Add agent setup guidance to use contracts as work boundaries. |
| Sensitivity/redaction | sensitive fields, redaction behavior | Validated for skill contracts | Keep validation; add first-run "no raw payload" posture. |
| Capabilities/adapters | allowed capabilities, adapter requirements | Validated for shape and known v0 adapter boundary | Add authority/side-effect posture check before writes. |
| Failure/evaluation | failure modes, evaluation criteria | Validated for selected fields | Add maintenance checks for missing evaluation evidence. |
| Tests | `tests/*.test.yml` declarations | Loaded and reference-shaped | Add check coverage posture before real test execution. |

## 6. Immediate User Experience Target

After:

```sh
workflow-os init-repo-governance
workflow-os first-run
```

the user should understand:

- who owns the scaffolded workflow;
- who is the escalation contact;
- whether those values are placeholders;
- what policy and approval gates exist;
- what evidence/checks are expected but not yet supplied;
- which side effects are unsupported or explicitly skipped;
- which fields are enforced now and which are advisory;
- what the agent should do next;
- which workflows/checkpoints should be created or hardened next.

The goal is not more YAML. The goal is a guided first-run ledger that says:

```text
Here is the governance contract I found.
Here is what I can enforce today.
Here is what is missing.
Here is what should become a real workflow/check next.
```

## 7. Proposed Workflow OS Workflows

These workflows should be treated as candidate governed workflows or workflow templates. Dogfood variants may live under `dogfood/`; portable user-facing variants should live under future `scaffolds/` or examples only after review.

| Workflow | Purpose | First Use |
| --- | --- | --- |
| `dg/spec-field-operationalization` | Govern Workflow OS work that turns scaffold/spec fields into validation, first-run disclosure, checks, or runtime behavior. | Dogfood roadmap execution. |
| `local/governance-onboarding-review` | Review a newly scaffolded repo for ownership, escalation, approval, checks, side effects, evidence, and report posture. | Existing-repo onboarding. |
| `local/workflow-ownership-review` | Identify workflows/skills with missing, placeholder, stale, or conflicting ownership and escalation metadata. | Single-user and team maintenance. |
| `local/workflow-check-coverage-review` | Compare declared validation/check obligations against available local check handlers, skipped checks, and evidence. | Before trusting generated reports. |
| `local/workflow-authority-review` | Review autonomy, capabilities, adapter requirements, policy gates, approvals, and side-effect posture before increasing authority. | Before write-capable or higher-autonomy work. |
| `local/workflow-catalog-stewardship` | Review recommendations for new/changed/retired workflows, ownership assignment, lifecycle transitions, and conflicts. | Future catalog/store lane. |

## 8. Proposed Checks

The first checks should be deterministic and local. They should inspect specs and report posture; they should not execute arbitrary commands or mutate workflow files.

| Check | Behavior | First Output |
| --- | --- | --- |
| Governance field posture check | Classifies important fields as enforced, validated, disclosed, advisory, or deferred. | Report-ready field posture table. |
| Ownership/escalation check | Detects missing or placeholder owner, maintainer, escalation contact, and lifecycle values. | Warnings first; future validation policy may fail selected active workflows. |
| Approval authority consistency check | Flags approval-sensitive skills or high-risk capabilities without approval policy or clear approver context. | Validation/report disclosure. |
| Evidence/check coverage check | Flags workflows whose reports/check obligations cannot yet cite evidence or local check references. | First-run and WorkReport disclosure. |
| Side-effect authority check | Flags adapter/capability declarations and future side-effect posture that lack policy, approval, timeout, idempotency, or report linkage. | Fail-closed before writes. |
| Handoff/report closure check | Ensures workflows intended for agent work include final report and handoff posture. | Report disclosure and workflow recommendation. |
| Catalog conflict check | Detects duplicate or overlapping workflow ownership, authority, resources, side effects, or lifecycle states. | Future catalog recommendation, not auto-mutation. |

## 9. Governance Strictness Profiles

Field posture should eventually be interpreted through an explicit governance profile. A field that is advisory in `observe_and_report` may become blocking in a stricter enterprise profile.

Initial planned profiles are documented in [Governance Strictness Profiles And Stewardship Plan](governance-strictness-profiles-and-stewardship-plan.md):

- `observe_and_report`: do not require approval by default; standardize evidence/report/audit posture.
- `agent_assisted_gated`: allow agents to provide required detail/evidence for selected gates when policy allows.
- `human_approval_gated`: pause at selected human approval checkpoints.
- `strict_enterprise`: future steward/admin-managed profile for organization-level governance.

The first implementation should disclose the selected/default profile without changing executor approval behavior.

## 10. Implementation Sequence

### Phase 1: Field Posture Planning And Review

Create this plan and run maintainer review.

No runtime behavior changes.

### Phase 2: First-Run Governance Field Posture Output

Status: Implemented in [First-Run Governance Field Posture Report](../concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REPORT.md) and accepted with non-blocking follow-ups in [First-Run Governance Field Posture Review](../concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REVIEW.md).

Extend `workflow-os first-run` to emit a bounded governance field posture summary:

- ownership configured / placeholder / missing;
- escalation configured / placeholder / missing;
- approval posture detected;
- policy requirements detected;
- evidence and check posture;
- audit/observability declarations;
- side-effect/capability posture;
- advisory/deferred field disclosure.

This should reuse existing validated constructors and redaction-safe output. It must not read raw repo contents or execute commands.

### Phase 3: Ownership And Escalation Check

Status: Implemented in [Ownership And Escalation Check Report](../concepts/OWNERSHIP_ESCALATION_CHECK_REPORT.md) and accepted with non-blocking follow-ups in [Ownership And Escalation Check Review](../concepts/OWNERSHIP_ESCALATION_CHECK_REVIEW.md).

Add a deterministic local helper/check that inspects loaded workflow and skill definitions for:

- missing owner metadata;
- scaffold placeholder values;
- missing escalation contact;
- deprecated or experimental lifecycle posture;
- workflows with approvals or side-effecting capabilities but unclear responsible human/team.

Initial behavior should be warning/reporting, not a hard schema break.

### Phase 4: Spec Field Coverage Check

Status: Implemented in [Spec Field Coverage Check Report](../concepts/SPEC_FIELD_COVERAGE_CHECK_REPORT.md).

The implemented helper/check inventories rich fields during `workflow-os first-run` and reports whether each field is:

- enforced by runtime;
- validated;
- disclosed in reports;
- advisory;
- deferred.

This gives users the "magic" map without pretending every field is automated. It remains warning-only and does not change validation pass/fail behavior, run workflows, execute checks, call providers, generate workflows, add RBAC/escalation routing, or enable writes.

### Phase 5: Workflow Discovery Integration

Update workflow discovery recommendations so proposed workflow changes include:

- suggested owner/steward;
- escalation contact;
- required evidence/checks;
- approval posture;
- side-effect posture;
- lifecycle state;
- report obligations;
- conflicts with existing ownership or authority.

Recommendations remain review-only.

### Phase 6: Catalog/Store Planning

Only after local checks are reviewed, plan catalog/store support for persistent ownership, lifecycle, stewardship decisions, and promotion/review state.

## 11. Runtime And Automation Boundaries

This lane should make the system more useful without overstepping:

- Do not silently execute commands because a check field exists.
- Do not silently call providers because an adapter requirement exists.
- Do not treat owner strings as RBAC.
- Do not page or notify escalation contacts.
- Do not auto-promote recommended workflows.
- Do not treat advisory fields as enforced behavior.
- Do not let report prose override deterministic validation.

## 12. Documentation Updates Needed

Future implementation phases should update:

- `docs/specs/workflows.md` with per-field operational posture notes;
- `docs/specs/skills.md` with per-field operational posture notes;
- `docs/user-guide/agent-harness-quickstart.md` with first-run ownership/escalation guidance;
- `docs/user-guide/workbook.md` with field posture and ownership maintenance worksheets;
- `docs/implementation-plans/existing-repo-governance-onboarding-plan.md` with this lane;
- `ROADMAP.md` with the P0 field-operationalization roadmap.

## 13. Test Plan For Future Implementation

Future implementation should test:

- first-run output includes governance field posture;
- missing owner is disclosed without leaking paths or payloads;
- placeholder owner values are detected;
- missing escalation contact is disclosed;
- approval-sensitive workflow without clear approval/owner posture is flagged;
- adapter/capability declarations produce side-effect/authority posture disclosure;
- audit/observability declarations are disclosed as declared versus implemented;
- advisory/deferred fields are not represented as enforced;
- JSON output remains bounded and redaction-safe;
- existing scaffold, validation, runtime, report, and docs tests continue to pass.

## 14. Final Recommendation

The next implementation prompt should be:

```text
First-run spec field coverage check.
```

That is the smallest high-leverage move after ownership/escalation checking. It makes the existing scaffold and specs feel guided and honest by showing which rich fields are enforced, validated, disclosed, advisory, or deferred, while preserving the current local/no-write/no-automatic-command boundary.

Do not start with catalog/store, RBAC, workflow schema changes, automatic workflow generation, or runtime command execution.
