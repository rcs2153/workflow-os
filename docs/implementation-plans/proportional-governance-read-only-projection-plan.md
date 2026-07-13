# Proportional Governance Read-Only Projection Plan

Status: The first read-only, in-memory projection and its deserialization error
redaction blocker fix are implemented and accepted. The P0 correction identified
by external dogfood feedback is implemented: operator disclosure is now
projected independently from execution disposition. Follow the
[Decision Axes And Workload Inference Plan](proportional-governance-decision-axis-and-inference-plan.md)
for focused review and later workload assessment. No executor behavior,
persistence, workflow event, CLI, schema, approval-default, or provider-write
behavior is implemented.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Proportional Governance Core Model Review](../concepts/PROPORTIONAL_GOVERNANCE_CORE_MODEL_REVIEW.md)
- [Proportional Governance Core Model Blocker Fix Review](../concepts/PROPORTIONAL_GOVERNANCE_CORE_MODEL_BLOCKER_FIX_REVIEW.md)
- [Governed Work Pattern](../concepts/governed-work-pattern.md)

## 1. Executive Summary

The accepted proportional-governance core model selects the strictest required
interaction mode from validated profile, policy, authority, evidence/check,
sensitivity, SideEffect, prior-decision, and runtime-escalation inputs. Nothing
outside that model currently exposes the accepted result through a deliberately
bounded operator- or machine-facing contract.

The next implementation should add one pure, read-only projection helper. It
should consume an already validated `ProportionalGovernanceDecision` and return
an in-memory projection that states the selected mode, risk class, stable reason
codes, required operator action, and the explicit facts that the result is
assessed but not enforced and not persisted.

This phase improves inspectability and machine readability. It does not make
the executor quiet, disclose automatically, pause for approval, deny work,
persist a decision, append an event, render CLI output, or authorize writes.

## 2. Goals

- Expose an accepted proportional-governance decision through one stable,
  bounded, domain-neutral projection.
- Keep the accepted decision as the only source of selection truth.
- Map each interaction mode to explicit operator-action vocabulary.
- Preserve stable reason codes without adding free-form explanations.
- State clearly that the projection is assessed, not enforced.
- State clearly that the projection is in memory, not persisted.
- Support redaction-safe `Debug`, serialization, and deserialization.
- Prepare a future inspect, report, event, or executor integration without
  authorizing any of those integrations now.
- Preserve all existing executor, approval, policy, report, SideEffect, and
  provider-write behavior.

## 3. Non-Goals

This phase must not add:

- executor or workflow state changes;
- automatic quiet success or non-blocking disclosure;
- automatic approval, model self-approval, or approval-default changes;
- denial enforcement outside the accepted core selector;
- durable state, workflow events, audit records, or report artifacts;
- CLI text, JSON commands, inspect output, or public schema changes;
- new policy semantics or governance-profile defaults;
- provider calls, provider writes, retries, recovery, or mutation families;
- evidence, checks, WorkReports, or SideEffects fabricated from the decision;
- hosted behavior, RBAC, IdP integration, enterprise administration, or remote
  policy sync;
- examples, release posture changes, reasoning lineage, recursive agents,
  agent swarms, or Level 3/4 autonomy defaults.

## 4. Source-Of-Truth Boundary

`select_proportional_governance` remains the only decision boundary in this
slice. The projection helper must accept an already constructed and validated
`ProportionalGovernanceDecision`; it must not accept the original decision
inputs or recompute risk.

The projection must not:

- reinterpret reason codes;
- drop reasons to produce a quieter result;
- add model-generated rationale;
- select a stricter or weaker mode;
- infer that evidence, checks, approvals, reports, or events exist;
- claim a decision was enforced or persisted.

This separation prevents a display contract from becoming an incompatible
parallel policy engine.

Correction note: the original projection mapped `VisibleDisclosure` from the
same ordered mode used for approval and denial. The corrected projection now
exposes accepted execution and disclosure axes independently and derives the
blocking operator action only from execution disposition.

## 5. Candidate Model

Add the smallest justified vocabulary, likely:

- `ProportionalGovernanceDecisionProjection`
- `GovernanceActionRequirement`
- `GovernanceDecisionPosture`
- `GovernancePersistencePosture`
- `project_proportional_governance_decision`

The implementation should reuse the existing:

- `GovernanceInteractionMode`
- `GovernanceRiskClass`
- `GovernanceDecisionReason`
- `ProportionalGovernanceDecision`

Do not duplicate those enums or introduce free-form status strings.

## 6. Required Projection Fields

The projection should contain only:

- selected interaction mode;
- selected risk class;
- stable ordered decision reasons;
- operator action requirement;
- decision posture;
- persistence posture.

Initial posture vocabulary should be closed and explicit:

```text
decision_posture: assessed_not_enforced
persistence_posture: not_persisted
```

The projection should not include workflow ID, run ID, step ID, actor ID,
evidence IDs, approval IDs, event IDs, report IDs, provider references, paths,
or timestamps. Those values require stable caller context or durable records and
belong in later integration phases.

## 7. Operator Action Mapping

The helper should deterministically map the selected interaction mode:

| Interaction mode | Action requirement | Meaning in this phase |
| --- | --- | --- |
| `quiet_capture` | `none` | No operator action is indicated by the accepted decision; nothing proceeds automatically because this projection is not enforced. |
| `visible_disclosure` | `review` | Operator attention is useful but the projection does not pause or continue a run. |
| `blocking_approval` | `approval` | Approval would be required by a future enforcing integration; no approval is created here. |
| `denial` | `denied` | The accepted decision is denied; this helper does not alter a run or emit a user diagnostic. |

The mapping must be exhaustive and impossible for callers to override.

## 8. Reason Ordering And Completeness

The core decision's stable reason order must be preserved exactly. Projection
must not sort by display label, deduplicate differently, or collapse multiple
requirements into prose.

An empty reason list must remain impossible through the validated source model.
Deserialization of a projection must fail closed if its mode, risk class,
action requirement, or posture fields are inconsistent.

## 9. Validation And Fail-Closed Behavior

Projection construction should be infallible after receiving a validated
decision. The projection must use custom validated deserialization so serialized
input cannot bypass the constructor boundary. Deserialization must validate at
least:

- action requirement matches interaction mode;
- decision posture is the supported non-enforcing posture;
- persistence posture is the supported in-memory posture;
- risk class, mode, and reasons retain the accepted model's consistency rules;
- reason list is non-empty, bounded, unique, and deterministically ordered.

If direct reuse of the decision's private validator is not available, add one
bounded shared validation path rather than copying validation logic.

Errors must use stable codes and must not echo raw serialized values.

## 10. Privacy And Redaction

The projection stores enum vocabulary only. It must not store or render:

- prompts or model reasoning;
- raw policy expressions;
- raw evidence, check, provider, command, parser, or source payloads;
- paths or repository contents;
- actor details, approval reasons, or operator notes;
- environment values, credentials, authorization headers, private keys, or
  token-like values.

`Debug`, serialization, deserialization errors, and validation errors must
remain safe by construction. No custom summary field is needed in this phase.

## 11. Compatibility Boundary

- Existing decision construction and serde behavior remain unchanged.
- Existing executor and approval APIs remain unchanged.
- The projection is additive and in memory only.
- No current caller is migrated automatically.
- No public CLI or workflow schema depends on the projection.
- Future fields require explicit compatibility review; unknown enum values fail
  closed under the current preview contract.

## 12. Relationship To Future Runtime Enforcement

This projection is deliberately insufficient to enforce a run. A later runtime
phase must independently define:

- which executor checkpoint invokes selection;
- which validated facts form the input;
- how the decision is durably recorded;
- how quiet, disclosure, approval, and denial change execution;
- how posture escalates when facts change;
- how approval-presentation proof composes with blocking approval;
- how evidence, checks, SideEffects, and WorkReports cite the decision.

Until then, consumers must treat the projection as inspectable assessment only.

## 13. Relationship To Immutable Run Bundles

Current runs bind workflow ID, workflow version, schema version, and spec content
hash, and durable state rejects mismatched identity. That is an important
immutability invariant, but it is not a complete self-contained run bundle.

External dogfood feedback correctly identified the remaining boundary: a run
should eventually retain or durably reference the exact validated workflow,
policy, skill, governance, and relevant configuration inputs needed for later
inspection and safe replay. Planning that bundle is a separate runtime phase
after this projection and before additional provider mutation expansion.

This projection must not imply that such a bundle already exists.

## 14. Future Test Plan

- every accepted interaction mode projects successfully;
- every interaction mode maps to the correct action requirement;
- projected mode, risk class, and reasons exactly match the source decision;
- source reason ordering is preserved;
- projection does not mutate the source decision;
- projection states `assessed_not_enforced`;
- projection states `not_persisted`;
- valid projection serde round trip succeeds;
- mismatched action requirement fails deserialization;
- unsupported decision or persistence posture fails deserialization;
- malformed or empty reasons fail closed;
- errors do not echo serialized values;
- `Debug` and serialization contain no raw payload fields;
- no workflow, run, actor, evidence, approval, event, report, provider, path, or
  timestamp field is introduced;
- existing proportional-governance core model tests continue to pass;
- existing executor, approval, policy, WorkReport, SideEffect, provider, and
  runtime tests continue to pass;
- docs check passes.

## 15. Proposed Implementation Sequence

1. Add the projection and closed posture/action enums beside the existing
   proportional-governance core model.
2. Add one pure mapping helper that consumes an accepted decision.
3. Add validated custom deserialization so serialized input cannot admit
   inconsistent projections; reuse a shared bounded decision validator where
   possible rather than copying the selector.
4. Add focused model, mapping, serde, and redaction tests.
5. Update the proportional-governance plan and roadmap honestly.
6. Create an implementation report and perform a focused maintainer review.
7. Plan immutable run-bundle hardening before additional provider mutation
   expansion.
8. Consider one explicit low-risk quiet runtime path only after both reviews.

## 16. Documentation Requirements

Implementation docs must state:

- the projection is implemented in memory only;
- the accepted decision remains the source of truth;
- the projection is assessed, not enforced;
- the projection is not persisted or emitted as an event;
- no executor, approval, CLI, schema, report, or provider behavior changed;
- immutable run bundles remain future work;
- provider mutation expansion remains deferred.

## 17. Open Questions

- Can the existing decision validator be reused without widening unrelated
  public API?
- Should `review` later split into informational and warning actions, or should
  that distinction remain outside the core?
- Which durable event should eventually carry the selected decision?
- Should a future inspect projection cite an event ID or a WorkReport citation?
- What exact authored and resolved inputs belong in an immutable run bundle?

## 18. Final Recommendation

Implement the read-only projection as the next small code phase. Keep it pure,
in memory, additive, non-enforcing, and non-persistent.

After its focused review, plan immutable run-bundle hardening before adding
another provider mutation family. Do not use the projection to activate quiet
success, change approval defaults, or claim runtime enforcement.

## 19. Governed Planning Evidence

- Dogfood workflow: `dg/d`.
- Run ID: `run-1783820441319658000-2`.
- Approval ID:
  `approval/run-1783820441319658000-2/planning-approved`.
- Presentation ID: `presentation/3c979dea7f3e2e11`.
- Approval outcome: granted through the proof-enforced approval path.
- Phase status: completed.
- Event summary: 39 ordered events, including one approval request, one
  proof-marked approval grant, eight policy decisions, six scheduled steps,
  six successful mock skill invocations, and one completed run; no retries or
  escalations.
- Out-of-kernel work: Codex inspected the accepted model and current immutable
  identity checks, authored this plan, and updated roadmap sequencing. The
  kernel governed scope and approval but did not edit files.
- Report posture: this plan carries the planning record. No runtime WorkReport
  or report artifact was generated or persisted.
