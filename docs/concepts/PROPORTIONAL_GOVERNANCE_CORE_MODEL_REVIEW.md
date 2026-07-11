# Proportional Governance Core Model Review

## 1. Executive Verdict

Needs blocker fixes.

The implementation is tightly scoped, deterministic in its direct helper path,
payload-free, and well tested. Two public model boundaries can nevertheless
produce claims that are inconsistent with the plan or with the decision's own
fields.

## 2. Scope Verification

The phase stayed within the approved model-only scope. It added no executor or
CLI integration, automatic approval, schema fields, persistence, workflow
events, metrics collection, provider mutations, hosted administration,
examples, or release changes.

## 3. Model Assessment

The ordered interaction modes, risk classes, requirement vocabulary, reason
codes, input, result, and pure selector are appropriately small and domain
neutral. The maximum-selection approach is deterministic and prevents one
requirement source from weakening another.

## 4. Blocker Findings

### Blocker 1: decision deserialization does not validate derived consistency

`ProportionalGovernanceDecision` derives `Deserialize` while storing both
`mode` and derived `risk_class`. A serialized value can therefore pair
`quiet_capture` with `denied` or another inconsistent risk class and deserialize
successfully. Callers could then observe contradictory validated-looking model
state.

Required fix: deserialize through a validated wire boundary, reject mismatched
mode/risk-class values with a stable non-leaking error, and add invalid-wire
tests.

### Blocker 2: profile mapping invents stricter product behavior

The selector maps `agent_assisted_gated` to visible disclosure and
`strict_enterprise` to blocking approval solely from the profile enum. The
accepted plan does not define those unconditional minima:

- agent-assisted work may remain quiet when required validated evidence and
  checks satisfy eligible gates;
- strict-enterprise posture depends on a steward-defined minimum that this
  first model does not yet carry.

This mapping violates the least-interruptive invariant and risks making maximum
ceremony the default. It also overstates the existing profile model, whose API
explicitly says profiles are not currently enforced.

Required fix: do not infer disclosure or approval from profiles whose complete
minimum cannot be derived by this model. Either add an explicit typed profile
minimum supplied by a future owning boundary or fail closed for unsupported
profile evaluation. Preserve human-approval-gated behavior only where the
profile contract explicitly requires configured human checkpoints, and test the
chosen compatibility policy.

## 5. Determinism And Escalation Assessment

The direct selector is pure. Ordered enum maximum selection is deterministic,
and `prior_mode` correctly prevents downgrade. Explicit denial wins. Unsupported
requirements return a stable payload-free error.

## 6. Privacy And Redaction Assessment

The model stores enum vocabulary only. Debug and serialization cannot include
raw prompts, paths, evidence payloads, command output, provider data, approval
reasons, or credentials. The unsupported error is stable and non-leaking.

## 7. Test Assessment

Tests cover the direct happy paths, strictest requirement, denial, monotonic
escalation, serde round trip, and unsupported failure. Missing blocker coverage:

- inconsistent serialized decision rejection;
- agent-assisted quiet eligibility when evidence/check requirements are quiet;
- explicit handling of strict-enterprise posture without a steward minimum.

## 8. Documentation Assessment

Documentation accurately states that the model is not runtime-integrated and
does not activate quiet success. The implementation report should retain that
boundary. Its model-acceptance language must not be strengthened until the two
blockers are fixed and re-reviewed.

## 9. Non-Blocking Follow-Ups

- Consider stable labels for modes, risk classes, and reasons when a read-only
  projection is planned.
- Decide later whether reason codes should include all contributing sources or
  only sources equal to the selected maximum.
- Keep event/audit projection and metrics semantics deferred.

## 10. Recommended Next Phase

Proportional governance core model blocker fix.

Fix validated deserialization and profile-minimum semantics only. Do not add
runtime integration, projection, CLI behavior, schema fields, automatic
approval, persistence, events, metrics, or provider writes.

## 11. Validation

- Full implementation-phase format, clippy, workspace tests, and docs check
  passed before review.
- Review used source inspection and the focused model test results.
- No live or opt-in provider tests were required for this model-only review.

## 12. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1783806708997132000-2`.
- Approval ID: `approval/run-1783806708997132000-2/review-scope-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/baf2e966e86888d4`.
- Out-of-kernel work: Codex inspected code/tests and authored this review; the
  kernel governed scope and approval but did not perform the review.
- Event and terminal summary: completed with 39 ordered events, one approval,
  zero retries, and zero escalations; presentation proof enforcement and the
  approval event marker were present.

## 13. Fix-Forward Note

The two blockers were subsequently addressed without widening scope. Decision
deserialization now validates derived consistency, agent-assisted posture may
remain quiet when concrete requirements permit it, and strict-enterprise
evaluation requires an explicit typed steward minimum. The original findings
remain the review record pending focused blocker-fix re-review.
