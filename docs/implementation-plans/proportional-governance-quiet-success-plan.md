# Proportional Governance And Quiet Success Plan

Status: Core decision model and focused blocker fixes accepted. Runtime
integration, schemas, CLI, automatic approval, and
approval-default behavior remain unimplemented.

Related foundations:

- [Governance Strictness Profiles And Stewardship Plan](governance-strictness-profiles-and-stewardship-plan.md)
- [Approval Gate Presentation Default Enforcement Plan](approval-gate-presentation-default-enforcement-plan.md)
- [Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md)
- [Governed Work Pattern](../concepts/governed-work-pattern.md)

## 1. Executive Summary

Workflow OS should apply the least ceremony necessary to satisfy declared
governance and the deterministically assessed risk of the work. Evidence,
disclosure, audit posture, and final reporting remain present even when no human
approval is required.

The current foundations define governance profiles, approval gates,
approval-presentation proof, evidence, SideEffect posture, policy decisions, and
WorkReports. They do not yet define one deterministic decision boundary that
maps those inputs to the operator experience. Without that boundary, the
repo-local dogfood ceremony could become the accidental product default, or
callers could make inconsistent decisions about when work should remain quiet,
be disclosed, or block for approval.

This plan introduces the future product invariant:

```text
Use the least interruptive governance mode that satisfies declared policy,
authority, evidence, sensitivity, and side-effect requirements.
Escalate when risk changes. Never weaken an explicit requirement silently.
```

This plan does not implement the model, runtime policy, CLI behavior, automatic
approval, provider writes, hosted administration, or schema changes.

## 2. Goals

- Define deterministic inputs for selecting governance interaction posture.
- Separate evidence capture from human interruption.
- Support quiet successful completion for eligible low-risk work.
- Support visible, non-blocking disclosure when operator awareness is useful.
- Require blocking approval when policy, authority, sensitivity, ambiguity, or
  SideEffect posture requires it.
- Escalate safely when observed execution posture becomes riskier than the
  posture assessed before execution.
- Preserve complete evidence, audit, check, SideEffect, limitation, risk, and
  report disclosure across every interaction mode.
- Make the selected posture and the reasons for it inspectable and stable.
- Give local users a legitimate automation-first path without weakening
  deterministic safety controls.
- Prepare enterprise stewards to set minimum posture by repository, workflow,
  team, department, risk class, or capability without implementing hosted
  administration now.
- Measure governance quality, including unnecessary interruption and missing
  evidence, rather than treating the number of approval gates as success.

## 3. Non-Goals

This phase does not authorize:

- implementation in this planning phase;
- automatic approval or model self-approval;
- bypass of declared approval, policy, authority, evidence, or check rules;
- probabilistic risk scoring as an enforcement source of truth;
- arbitrary model judgment deciding whether a gate applies;
- global changes to current approval defaults;
- workflow schema changes;
- CLI behavior changes;
- public or hosted approval-card UI;
- RBAC, IdP integration, organization administration, or remote policy sync;
- provider writes or new mutation families;
- automatic local command execution;
- weakening high-assurance approval controls;
- recursive agents, agent swarms, or Level 3/4 autonomy defaults;
- examples or release posture changes.

## 4. Product Boundary

Proportional governance is not reduced governance. It separates three concerns:

1. what must be recorded;
2. what must be shown immediately;
3. what must interrupt execution.

Evidence and reporting obligations can remain mandatory while human approval is
unnecessary. Conversely, concise output must not hide a blocking requirement or
erase the durable record.

The kernel owns deterministic classification and enforcement. Agents and tools
may provide bounded facts and evidence, but they must not decide unilaterally
that an explicit gate no longer applies.

## 5. Relationship To Governance Profiles

Governance profiles establish the minimum posture desired by the user or
steward. Proportional governance refines behavior within that allowed envelope;
it does not override the profile.

| Profile | Minimum future posture |
| --- | --- |
| `observe_and_report` | Prefer quiet capture or visible disclosure. Block only for deterministic safety violations or an explicitly required gate. |
| `agent_assisted_gated` | Allow eligible gates to be satisfied by validated evidence; block when required evidence or checks are missing or invalid. |
| `human_approval_gated` | Block at declared human checkpoints and any higher-risk escalation boundary. |
| `strict_enterprise` | Enforce steward-defined minimum posture and authority rules; local callers cannot downgrade it. |

The final selected posture must be at least as strict as the active profile,
workflow declaration, policy result, capability boundary, and steward rule.

## 6. Deterministic Decision Inputs

The future decision model should accept typed, bounded inputs already validated
by their owning model boundaries:

- active governance profile;
- workflow and step identity;
- declared approval and policy requirements;
- actor and delegated authority posture;
- action/capability class;
- sensitivity;
- SideEffect kind and lifecycle posture;
- reversibility and externally visible impact;
- affected resource scope;
- evidence requirements and evidence availability;
- required check posture and results;
- ambiguity or unsupported-behavior flags;
- retry, escalation, and timeout posture;
- workflow ownership and steward minimums;
- prior decision posture for the same run/step;
- runtime change signals that may require escalation.

Raw prompts, model chain-of-thought, provider payloads, command output, tokens,
credentials, and unbounded source contents must not become decision inputs.

## 7. Initial Risk Classes

The first implementation should use a small ordered enum, not a free-form score:

| Class | Meaning | Typical posture |
| --- | --- | --- |
| `bounded_observation` | No external mutation; required evidence/check/report posture is available or explicitly disclosed. | Quiet capture |
| `review_worthy` | No mandatory human gate, but missing optional evidence, notable limitations, elevated sensitivity, or meaningful operator context exists. | Visible non-blocking disclosure |
| `approval_required` | A declared gate, authority boundary, sensitive action, externally visible mutation, ambiguity, or steward rule requires approval. | Blocking approval |
| `denied` | Unsupported, malformed, unsafe, prohibited, or unenforceable posture. | Fail closed |

The names may change after model review. The ordering and fail-closed semantics
must remain explicit.

## 8. Interaction Modes

### Quiet Capture

Eligible work proceeds without an approval pause or verbose success dump. The
kernel still records the selected posture, reasons, evidence/check state,
SideEffect disclosure, limitations, and report/audit references.

Human output should be one bounded completion line or equivalent machine result
with an inspect command/reference. Quiet capture must never mean no record.

### Visible Non-Blocking Disclosure

Work may proceed, but the operator receives a concise bounded disclosure of the
reason attention may be useful. Examples include skipped optional checks,
partial evidence, elevated but permitted sensitivity, or a known limitation.

The disclosure must identify whether action is required. It must not look like
an approval request when execution is not blocked.

### Blocking Approval

Execution pauses before the governed action. The existing approval-presentation
requirements apply: concrete scope, strict non-goals, affected surfaces,
validation/evidence context, reason for the gate, and next action must be
presented and, where configured, durably proven.

### Denial

Execution fails closed when requirements are invalid, prohibited, ambiguous,
unsupported, or not enforceable by the active runtime path. Denial is not an
interaction optimization and cannot be downgraded to disclosure.

## 9. Deterministic Selection Rules

The selection helper should be pure and monotonic:

1. Validate all inputs.
2. Derive the minimum profile posture.
3. Derive workflow, policy, authority, evidence, check, sensitivity, and
   SideEffect posture independently.
4. Select the strictest resulting posture.
5. Return bounded reason codes and required obligations.
6. Reject unsupported or contradictory combinations.

Suggested invariant:

```text
selected_posture = max(
  profile_minimum,
  workflow_requirement,
  policy_requirement,
  authority_requirement,
  evidence_check_requirement,
  sensitivity_requirement,
  side_effect_requirement,
  runtime_escalation_requirement
)
```

The ordering is deterministic, not probabilistic. A model may recommend a
stricter posture as review-only input, but it cannot downgrade the result.

## 10. Runtime Escalation

The kernel should reassess posture at explicit checkpoints when validated facts
change. Escalation triggers may include:

- a read-only action becomes a proposed external mutation;
- affected resource scope expands;
- sensitivity increases;
- required evidence or checks become unavailable or fail;
- the actor lacks required authority;
- provider outcome becomes ambiguous;
- retry would risk duplicating an external effect;
- workflow execution reaches a declared sensitive checkpoint;
- a steward minimum changes before the governed action begins.

Posture may move from quiet to disclosure, approval, or denial. It must not move
to a less strict posture during an active governed action without a new explicit
decision and auditable reason. Already completed side effects cannot be made safe
retroactively by a later approval.

## 11. Quiet Success UX

Default human output for eligible successful work should answer only:

- what completed;
- which governance mode applied;
- whether anything needs attention;
- how to inspect the complete record.

Example vocabulary, not a committed CLI contract:

```text
Completed under observe-and-report. Evidence and report posture recorded.
Inspect: workflow-os inspect <run-id>
```

Detailed policy decisions, field posture, evidence references, skipped checks,
and event history remain available through explicit detail/verbose/JSON or
inspect surfaces. Compact output must be a projection of the durable record, not
a separate source of truth.

## 12. Evidence, Reporting, And Audit Invariants

Every interaction mode should record:

- selected posture and stable reason codes;
- profile and policy inputs used;
- actor/authority posture where available;
- evidence and check availability/results;
- SideEffect posture, including none/skipped/unsupported;
- approval identity and presentation proof when approval occurred;
- incomplete work, limitations, and risks;
- escalation transitions;
- report/audit references where the current runtime supports them.

Missing evidence must remain explicit. Quiet success must not fabricate
evidence, hide skipped checks, or claim a WorkReport/artifact was produced when
only report-ready context exists.

## 13. Failure And Compatibility Behavior

- Invalid or contradictory decision inputs fail closed with stable,
  non-leaking error codes.
- A declared gate that the runtime cannot enforce is rejected, not treated as
  advisory.
- Absence of the new policy must preserve existing behavior during staged
  adoption.
- Callers cannot request a posture below the computed minimum.
- Rendering failure must not erase the durable decision result.
- Evidence/report projection failure must be disclosed without rewriting an
  already completed workflow result unless the governing contract explicitly
  required successful report closure.
- Existing proof-enforced approval paths remain unchanged until separately
  integrated and reviewed.

## 14. Metrics And Evaluation

Future runtime telemetry should support bounded aggregate metrics without raw
payload capture:

- approvals requested by risk class and profile;
- unnecessary approval rate, based on reviewed outcomes or configured labels;
- approval wait time;
- disclosure-to-escalation rate;
- quiet-completion rate;
- gate override and denial rate;
- missing or invalid evidence rate;
- skipped required/optional check rate;
- post-start escalation rate;
- report/evidence completeness rate;
- ambiguous SideEffect/provider outcome rate.

These metrics evaluate whether governance improves outcomes with acceptable
friction. A high approval count is not inherently a success metric.

## 15. Proposed Implementation Sequence

1. **Core decision model only: implemented.** Typed risk class, interaction
   mode, reason codes, bounded decision input/result, and pure monotonic
   selection helper now exist in `workflow-core`.
2. **Focused model review.** Verify deterministic ordering, fail-closed
   behavior, redaction safety, and compatibility with governance profiles.
3. **Read-only projection.** Expose the selected posture and reason codes through
   an explicit in-memory result or inspect projection without changing executor
   behavior.
4. **One low-risk quiet path.** Integrate quiet capture into one explicit local,
   read-only, no-SideEffect path while preserving complete event/report posture.
5. **One escalation path.** Prove deterministic promotion from disclosure to
   blocking approval when a validated runtime fact changes.
6. **Operator UX hardening.** Add compact human output with explicit drill-down
   and retain bounded verbose/JSON detail.
7. **Profile integration.** Compose with existing governance profile vocabulary
   and reject attempted downgrades.
8. **Steward policy integration later.** Add organization minimums only after a
   separate admin/authority design and shared durable state exist.

Each implementation slice requires its own plan or accepted prompt, focused
tests, implementation report, and maintainer review.

## 16. Future Test Plan

- each risk class and interaction mode is representable;
- identical inputs always produce the same result;
- input ordering does not change the decision;
- the strictest applicable requirement wins;
- callers cannot downgrade an explicit approval requirement;
- unsupported declared requirements fail closed;
- `observe_and_report` permits quiet capture for eligible bounded observation;
- quiet capture still records evidence/check/SideEffect/report posture;
- missing optional evidence can produce non-blocking disclosure;
- missing required evidence blocks or denies according to contract;
- an external mutation cannot select quiet capture;
- sensitive or ambiguous action selects approval or denial;
- runtime fact changes escalate monotonically;
- approval presentation proof remains required where configured;
- concise output includes an inspect reference and no raw payloads;
- verbose/JSON output remains bounded and redaction-safe;
- errors do not leak paths, tokens, credentials, provider payloads, source
  contents, command output, or approval reasons;
- current executor and approval behavior remains unchanged before explicit
  integration;
- existing governance profile, approval, evidence, WorkReport, SideEffect, and
  runtime tests continue to pass.

## 17. Documentation Requirements

Future implementation must document:

- the active profile and selected interaction mode;
- which inputs are enforced, validated, disclosed, advisory, or deferred;
- the exact quiet-success product contract;
- escalation and denial behavior;
- compatibility/default behavior;
- metrics semantics and privacy posture;
- the difference between evidence capture, operator disclosure, and approval;
- current local-only and enterprise-stewardship boundaries.

## 18. Open Questions

- Which existing action/capability taxonomy should seed initial risk classes?
- Should `review_worthy` be one mode or separate informational and warning
  modes?
- Which optional missing evidence warrants disclosure rather than quiet capture?
- How should a caller request stricter behavior without creating incompatible
  local policy?
- Which event should durably record initial selection and later escalation?
- Should successful quiet capture produce no stdout by default, or one bounded
  completion line?
- How should enterprise steward minimums compose with local workflow-specific
  requirements once shared state exists?
- What reviewed signal can label an approval as unnecessary without relying on
  model opinion?

## 19. Final Recommendation

Proceed next with the core decision model only after the current complete
provider-write sandbox proof is accepted. The model should be implemented before
public approval defaults or broad workflow automation are expanded, so those
surfaces do not encode maximum dogfood ceremony as the permanent user
experience.

Do not build CLI behavior, automatic approvals, provider writes, hosted
stewardship, schema fields, or enterprise administration as part of the first
implementation slice.

## 20. Governed Planning Evidence

- Dogfood workflow: `dg/d`.
- Run ID: `run-1783795205937611000-2`.
- Approval ID: `approval/run-1783795205937611000-2/planning-approved`.
- Approval outcome: granted with persisted approval-presentation proof.
- Phase status: completed.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, eight policy decisions, six scheduled steps, six successful
  skill invocations, and one completed run; no retries or escalations.
- Validation: `npm run check:docs` passed.
- Out-of-kernel work: the side-conversation planning agent authored the plan
  and roadmap handoff. The kernel governed scope and approval but did not edit
  files or implement runtime behavior.
- Report posture: this plan carries the governed planning record; no runtime
  WorkReport artifact was generated or persisted.
