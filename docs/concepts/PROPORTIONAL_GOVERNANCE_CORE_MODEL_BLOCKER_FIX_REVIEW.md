# Proportional Governance Core Model Blocker Fix Review

## 1. Executive Verdict

Blockers fixed; proportional-governance core model accepted.

Proceed to read-only decision projection planning. Do not integrate the model
into executor decisions until that projection boundary is separately planned,
implemented, and reviewed.

## 2. Scope Verification

The fix remained within the two approved model blockers. It added no runtime or
executor behavior, CLI, schemas, persistence, workflow events, automatic
approval, metrics collection, provider mutations, examples, hosted behavior, or
release changes.

## 3. Deserialization Fix Assessment

`ProportionalGovernanceDecision` now deserializes through a private wire shape.
It rejects a mode/risk-class mismatch and rejects results without the required
profile-minimum reason. The fixed error text is bounded and does not echo the
serialized payload.

Blocker 1 is fixed.

## 4. Profile Semantics Fix Assessment

- `observe_and_report` and `agent_assisted_gated` permit quiet capture when all
  concrete requirements permit it.
- `human_approval_gated` retains blocking approval as its explicit profile
  minimum.
- `strict_enterprise` requires an explicit typed steward minimum and fails
  closed when it is absent.
- A supplied steward minimum participates in the same strictest-posture maximum
  and receives a stable reason code.

The model therefore no longer invents unconditional disclosure or approval
behavior and preserves the accepted least-interruptive invariant.

Blocker 2 is fixed.

## 5. Determinism And Monotonicity

Selection remains a pure ordered maximum. Profile, workflow, policy, authority,
evidence/check, sensitivity, SideEffect, runtime-escalation, steward, and prior
decision requirements cannot weaken one another. Prior decisions remain a
floor, including denial.

## 6. Privacy And Error Safety

All new data remains enum-only. No raw prompts, paths, evidence, source content,
command output, provider payloads, approval reasons, or credentials can be
stored. Both unsupported-requirement and missing-steward errors are stable and
non-leaking. Invalid serde errors do not reproduce input.

## 7. Test Assessment

The focused suite now has 14 passing tests and covers both blockers directly.
Full format, clippy, workspace tests, and docs validation passed. Existing
opt-in live tests remained skipped by their established environment gates; no
live provider test is relevant to this model-only fix.

## 8. Remaining Limitations

- No runtime path derives these typed requirements.
- No read-only inspect or result projection exists.
- No decision or escalation event is persisted.
- No quiet-success runtime behavior or CLI output exists.
- No metrics are collected.
- Enterprise steward state and administration remain future work.

These are explicit deferred phases, not blockers to the core model.

## 9. Recommended Next Phase

Plan a read-only proportional-governance decision projection. It should expose
the selected mode, risk class, and stable reasons through an explicit in-memory
result without changing executor decisions, approval defaults, persistence,
events, CLI output, or provider behavior.

## 10. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1783808155460259000-2`.
- Approval ID: `approval/run-1783808155460259000-2/review-scope-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/675355860574d805`.
- Validation: focused tests, format, clippy, full workspace tests, docs check,
  and diff check passed during the blocker phase.
- Out-of-kernel work: Codex inspected the fix and authored this review; the
  kernel governed scope and approval but did not perform the review.
- Event and terminal summary: completed with 39 ordered events, one approval,
  zero retries, and zero escalations; presentation proof enforcement and the
  approval event marker were present.
