# Proportional Governance Decision Axes And Workload Inference Plan

Status: Planning accepted. The two-axis core decision model and read-only
projection correction are implemented and accepted after one focused blocker
fix. The model-only deterministic workload assessment and fingerprint helper
are implemented and accepted after focused fixes for fingerprint framing and
reason provenance. The model
remains assessed, in-memory, not persisted, and not runtime-enforced. The first
pure workflow-declaration derivation helper is implemented. Focused review found
an incomplete workflow-level policy invalidation root; the narrow fix is
implemented and accepted after focused re-review. The first read-only onboarding
assessment path is implemented in `workflow-os first-run --verbose` and preview
JSON. It remains review-only, assessed-not-enforced, and not persisted; the
default concise first-run output remains quiet. Runtime enforcement remains
unimplemented.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Proportional Governance Read-Only Projection Plan](proportional-governance-read-only-projection-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)
- [Existing Repo Governance Onboarding Plan](existing-repo-governance-onboarding-plan.md)

## 1. Executive Summary

External dogfood feedback identified two valid gaps in the accepted
proportional-governance model:

1. `VisibleDisclosure` is modeled as a stricter execution mode than
   `QuietCapture`, although both permit work to proceed and differ primarily in
   what an operator sees.
2. `ProportionalGovernanceDecisionInput` assumes a caller has already converted
   workload facts into governance requirements. That leaves the most important
   onboarding work outside the model and makes policy configuration too manual.

The corrected boundary separates execution disposition from disclosure. Audit,
evidence, and report capture remain invariants. A deterministic workload
assessment may recommend governance from validated repository, workflow,
capability, authority, check, evidence, sensitivity, and SideEffect metadata,
but it may never weaken an explicit workflow, policy, profile, or steward
minimum.

This plan does not implement runtime enforcement, UI, schemas, provider
mutations, persistence, hosted administration, or probabilistic model authority.

## 2. Product Decision

`VisibleDisclosure` should not remain an execution-disposition category.

The model should express independent answers to independent questions:

- **Execution disposition:** may work proceed, must it await approval, or is it
  denied?
- **Disclosure obligation:** may presentation remain quiet, or must bounded
  operator-visible disclosure occur?
- **Record obligation:** what evidence, audit, check, SideEffect, limitation,
  and report posture must be retained?

Record obligations are always present according to the governing contract.
Operator UI may render quiet decisions live without changing their execution
disposition. Policy may separately require visible disclosure even when no
approval is required.

## 3. Goals

- Separate interruption semantics from presentation semantics.
- Preserve deterministic monotonic escalation and fail-closed denial.
- Make evidence and audit capture invariant rather than optional modes.
- Derive useful governance recommendations from validated workload metadata.
- Let existing-repository onboarding configure most common posture without
  requiring users to hand-author every decision input.
- Keep explicit policy, authority, profile, and steward requirements
  authoritative.
- Reassess governance when relevant validated inputs change.
- Produce stable reason codes and a payload-free input fingerprint.
- Keep unknown or unsupported workload facts explicit and conservative.
- Prepare future local UI, CLI, and enterprise policy projection without
  implementing them now.

## 4. Non-Goals

This phase does not authorize:

- executor or workflow-state changes;
- automatic approvals or model self-approval;
- probabilistic inference as an enforcement source of truth;
- arbitrary source-code or prompt inspection;
- workflow schema or policy schema exposure;
- provider calls, PR creation, Jira issue creation, or other mutation families;
- durable decision storage, events, audit records, or report artifacts;
- local or hosted UI;
- RBAC, IdP integration, remote policy sync, or enterprise administration;
- hidden gates, silent downgrades, or ambient authority;
- recursive agents, agent swarms, or autonomy-level claims;
- release-posture changes.

## 5. Corrected Core Model

The smallest future model should replace the ordered interaction mode with
orthogonal vocabulary, likely:

- `GovernanceExecutionDisposition`
  - `Proceed`
  - `RequireApproval`
  - `Deny`
- `GovernanceDisclosureRequirement`
  - `Quiet`
  - `Visible`
- `GovernanceExecutionRequirement`
  - `Proceed`
  - `RequireApproval`
  - `Deny`
- `GovernanceDisclosureObligation`
  - `QuietAllowed`
  - `VisibleRequired`

Names may be refined during implementation. The separation is the invariant.

`GovernanceRiskClass` should describe assessed risk, not presentation style.
An allowed action can be review-worthy while remaining `Proceed`; the separate
disclosure obligation determines whether the operator must be notified.

## 6. Deterministic Selection

Selection should compute each axis independently:

```text
execution_disposition = max(
  profile_execution_minimum,
  workflow_execution_requirement,
  policy_execution_requirement,
  authority_execution_requirement,
  evidence_check_execution_requirement,
  sensitivity_execution_requirement,
  side_effect_execution_requirement,
  runtime_execution_escalation,
  prior_execution_disposition
)

disclosure_requirement = max(
  profile_disclosure_minimum,
  workflow_disclosure_obligation,
  policy_disclosure_obligation,
  authority_disclosure_obligation,
  evidence_check_disclosure_obligation,
  sensitivity_disclosure_obligation,
  side_effect_disclosure_obligation,
  runtime_disclosure_escalation,
  prior_disclosure_requirement
)
```

`Deny` is terminal. `RequireApproval` cannot be downgraded to `Proceed` during
the same governed action. `Visible` cannot be downgraded to `Quiet` when an
explicit requirement or prior accepted decision requires visibility.

## 7. Capture And Presentation Boundary

Quiet capture means eligible work proceeds without interruption and without a
required immediate disclosure. It does not mean missing records.

Visible disclosure means eligible work proceeds while a bounded disclosure is
required. A local UI, CLI stream, dashboard, or notification system may project
that disclosure later. The presentation surface is not the policy engine.

An operator may choose to display quiet decisions live. That preference does
not mutate governance. Conversely, hiding a policy-required visible disclosure
does not satisfy the obligation.

Every disposition must preserve the applicable evidence, audit, check,
SideEffect, limitation, risk, and report posture.

## 8. Workload Assessment Boundary

The framework should get most existing workflows to a useful recommendation
without requiring manual per-decision configuration. The first assessment
boundary should consume only validated, bounded facts such as:

- workflow and step identity and definition hashes;
- governance profile and declared policy results;
- action and capability class;
- read-only or mutation posture;
- SideEffect kind, scope, lifecycle, and reversibility;
- resource count or bounded affected scope;
- external visibility;
- sensitivity classification;
- actor and delegated-authority posture;
- required evidence and checks plus availability/results;
- retry, ambiguity, and reconciliation posture;
- repository metadata already accepted by onboarding;
- prior decision and validated runtime changes.

Raw prompts, chain-of-thought, provider payloads, command output, source
contents, environment values, credentials, and unbounded natural language are
not assessment inputs.

## 9. Recommendation Versus Authority

Pure inference must not become hidden policy authority.

The assessment layer may produce:

- a recommended execution disposition;
- a recommended disclosure requirement;
- stable reasons;
- facts that remain unknown;
- a confidence posture based on deterministic completeness, not model belief;
- an input fingerprint.

The enforced result is the strictest composition of inferred recommendation
and explicit validated minima. Inference may escalate. It may not downgrade:

- workflow requirements;
- policy decisions;
- authority boundaries;
- governance-profile minima;
- steward minima;
- required evidence or checks;
- SideEffect constraints;
- prior accepted posture for the active action.

Unknown facts that affect safety must cause explicit escalation or denial, not
optimistic inference.

## 10. Configuration Posture

Users should not have to construct `ProportionalGovernanceDecisionInput`
manually for ordinary onboarding.

Future configuration should declare constraints and overrides, not reproduce
the entire classifier. Likely sources are:

- built-in deterministic defaults;
- validated workflow and policy declarations;
- governance profile;
- repository-level local configuration;
- future steward minimums.

Workflow schema changes remain deferred. Before schema exposure, the core
model and assessment helper should accept explicit typed inputs in memory.

The onboarding target is to infer or derive roughly 90 percent of common
posture from safe metadata, then ask the user only about unresolved authority,
sensitivity, approval, write, and evidence/check choices.

## 11. Input Fingerprint And Invalidation

The build-cache analogy is useful. A decision should be bound to a stable,
payload-free fingerprint over its validated inputs. Relevant changes invalidate
the prior assessment and require deterministic reevaluation.

Fingerprint inputs should eventually include:

- immutable workflow, skill, and policy definition references;
- governance profile and explicit requirements;
- capability and authority references;
- evidence/check requirement posture;
- sensitivity and SideEffect posture;
- relevant runtime escalation facts;
- versioned assessment algorithm identity.

The fingerprint is not an authorization grant and is not executable replay.
The immutable run-bundle boundary should provide the canonical definition roots
before runtime enforcement relies on this mechanism.

## 12. Existing-Repository Onboarding

Future first-run onboarding should:

1. inspect safe repository and workflow metadata;
2. classify known actions and validation obligations;
3. derive a review-only governance recommendation;
4. identify unresolved facts explicitly;
5. propose profile, policy, approval, evidence/check, and disclosure defaults;
6. let the user accept or tighten the recommendation;
7. never silently weaken existing declarations;
8. rerun assessment when relevant metadata changes.

Natural-language agent recommendations may explain or suggest posture, but the
kernel must make the enforceable decision from validated typed facts.

## 13. Provider Mutation Sequencing

The GitHub PR-comment sandbox is the first narrow provider-write proof. PR
creation and Jira issue creation are plausible later mutation families, not
automatic extensions of the comment capability.

Each mutation family needs its own reviewed capability scope, authority,
idempotency, reconciliation, evidence, approval, SideEffect, report, and sandbox
proof. This correction does not authorize those mutations.

## 14. Compatibility And Migration

No runtime caller consumes the current model, so this is the least expensive
point to correct it. The preview API may make a deliberate breaking model
change, but serde and projection compatibility must be reviewed explicitly.

The implementation should remove or deprecate `VisibleDisclosure` as an
execution mode rather than retain two competing sources of truth. It should
update the read-only projection in the same bounded phase so accepted decision
and projection cannot disagree.

## 15. Test Plan

Future tests must prove:

- proceed, approval, and denial are representable independently of disclosure;
- quiet and visible disclosure are representable for permitted work;
- visible disclosure does not itself imply blocking approval;
- a local presentation preference cannot weaken required visibility;
- record obligations remain independent of both axes;
- strictest explicit execution requirement wins;
- strictest explicit disclosure obligation wins;
- prior posture prevents runtime downgrade;
- unsupported or contradictory inputs fail closed;
- identical validated inputs produce identical decisions and fingerprints;
- relevant input changes alter the fingerprint and force reassessment;
- irrelevant presentation preferences do not alter execution disposition;
- inferred recommendations cannot downgrade explicit minima;
- unknown safety-relevant facts escalate or deny;
- Debug, serde, and errors remain bounded and non-leaking;
- current executor behavior remains unchanged before integration;
- existing policy, approval, evidence, SideEffect, WorkReport, and provider
  tests continue to pass.

## 16. Proposed Implementation Sequence

1. **Implemented.** Correct the core decision model and pure selector to
   separate execution and disclosure axes.
2. **Implemented.** Correct the read-only projection and its validated serde
   boundary.
3. **Implemented.** Add a model-only workload assessment input,
   recommendation, unresolved-fact/completeness posture, and versioned stable
   fingerprint helper. The helper consumes only bounded typed facts and the
   immutable definition root, then composes inferred posture with explicit
   minima through the accepted selector.
4. Review the combined model before any runtime integration. Initial review
   found architecture-dependent length framing and inferred-action posture
   mislabeled as workflow-declared provenance. Fixed-width framing, a known v1
   vector, and a distinct workload-assessment selector source/reason now fix
   those blockers; focused re-review accepts both corrections.
5. Complete immutable run-bundle construction and use its canonical roots in a
   later reassessment boundary.
6. **Implemented and accepted.** Add deterministic
   derivation from one already-loaded, validated workflow step, resolved skill,
   referenced policies, and explicit runtime-only facts into the accepted
   assessment input. The helper now derives static declaration facts and a
   relevant-definition root without filesystem scanning, persistence, schema
   changes, or enforcement. Workflow-level retry and escalation policy
   definitions now participate in invalidation after the initial focused review
   found that omission.
7. **Implemented and accepted with non-blocking follow-ups.** Project one assessment per validated
   workflow step through `workflow-os first-run --verbose` and preview JSON.
   The projection keeps execution and disclosure separate, exposes unknown
   facts, completeness, algorithm identity, and input fingerprint, and labels
   every result review-only, assessed-not-enforced, and not persisted. It adds
   no YAML, runtime enforcement, UI server, authority inference, or provider
   mutation. Focused review confirms the boundary remains read-only and
   recommends broader CLI ordering/invalidation coverage only when the output
   or persistence contract next expands.
8. Only then plan runtime enforcement and presentation surfaces.

The first implementation prompt should cover steps 1 and 2 only. Step 3 should
remain a separate reviewed phase because inference provenance and invalidation
are security-relevant.

## 17. Open Questions

- Should risk class remain a decision output or become an assessment input?
- Which disclosure levels beyond quiet and visible are genuinely enforceable
  obligations rather than UI preferences?
- Which repository metadata is sufficiently stable and safe for initial
  assessment?
- Which unknown facts require approval versus denial?
- How should future policy YAML express minima without exposing internal model
  mechanics?
- Which algorithm/version identifier belongs in the fingerprint?
- When should a changed input invalidate an approval as well as a governance
  decision?
- How should local UI preferences compose with enterprise-required
  notifications?

## 18. Final Recommendation

Treat the feedback as a P0 model correction before proportional-governance
runtime integration. Implement the two-axis core model and projection together,
then separately implement deterministic workload assessment and fingerprinting.

Do not make pure inference the authority source. Do not require users to
hand-author every decision input. Do not broaden provider mutations while the
decision and invalidation boundaries are unsettled.

## 19. Governed Planning Evidence

- Dogfood workflow: `dg/d`.
- Run ID: `run-1783920086133151000-2`.
- Approval ID: `approval/run-1783920086133151000-2/planning-approved`.
- Approval presentation: `presentation/603d8cc264525044`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Validation required: `npm run check:docs` and `git diff --check`.
- Out-of-kernel work: Codex inspected the accepted model and authored this
  planning change; the kernel governed scope and approval but did not edit files.
- Report posture: this document carries the planning record; no WorkReport
  artifact was generated or persisted.
