# Proportional Governance Read-Only Projection Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to read-only projection implementation.**

The plan is narrow, implementation-ready, and preserves the accepted selector
as the only decision source. Two planning defects were corrected during review:
the interaction-mode table now uses the existing `visible_disclosure` contract,
and validated custom deserialization is required rather than left open.

## 2. Scope Verification

The plan remains within a read-only model/helper boundary. It does not authorize
executor behavior, quiet-success activation, approval-default changes,
automatic approval, persistence, workflow events, CLI output, schemas, reports,
provider calls, writes, mutation expansion, hosted behavior, RBAC, examples, or
release changes.

The roadmap update adds immutable run-bundle hardening as a future phase. It
does not claim that the bundle exists or implement it in this phase.

## 3. Source-Of-Truth Assessment

The plan correctly requires the helper to consume an already validated
`ProportionalGovernanceDecision`. It does not accept original selector inputs,
recompute risk, reinterpret reasons, or generate rationale.

This keeps `select_proportional_governance` as the sole policy-selection
boundary and prevents the projection from becoming a second decision engine.

## 4. Projection Model Assessment

The candidate model is appropriately minimal:

- selected interaction mode;
- corresponding risk class;
- stable ordered reason codes;
- deterministic operator action requirement;
- explicit decision posture;
- explicit persistence posture.

The proposed closed values `assessed_not_enforced` and `not_persisted` are
important capability guards. The deliberate exclusion of workflow, run, actor,
evidence, approval, event, report, provider, path, and timestamp fields avoids
unstable or fabricated context.

## 5. Deterministic Mapping Assessment

The exhaustive mapping is suitable:

- `quiet_capture` to `none`;
- `visible_disclosure` to `review`;
- `blocking_approval` to `approval`;
- `denied` to `denied`.

Callers cannot override the mapping. The review corrected the initial plan's
`visible_non_blocking_disclosure` spelling because it did not match the accepted
Rust/serde contract `visible_disclosure`.

## 6. Validation And Serde Assessment

The plan now requires custom validated deserialization. This is necessary
because independently serialized mode, risk class, action requirement, reasons,
and posture fields could otherwise form an inconsistent projection.

Implementation should reuse a bounded internal decision validator if that can
be done without widening unrelated public API. It must not duplicate the
selector or infer missing reasons. Stable non-leaking errors are required.

## 7. Privacy And Redaction Assessment

The enum-only projection is safe by design and has no free-form summary field.
The plan explicitly excludes prompts, reasoning, raw policy expressions,
payloads, paths, actor details, approval reasons, environment values, and
credentials. Required `Debug`, serde, and error tests are proportionate.

## 8. Compatibility Assessment

The projection is additive, in memory, and has no current automatic consumer.
Existing decision, executor, approval, policy, WorkReport, SideEffect, and
provider behavior remains unchanged. No schema or CLI contract is created.

## 9. Immutable Run-Bundle Assessment

The roadmap distinction is accurate. Current runtime identity binds workflow,
version, schema version, and spec content hash and rejects mismatches. That does
not by itself preserve a self-contained set of resolved workflow, policy, skill,
governance, and configuration inputs for later inspection or safe replay.

Positioning immutable run-bundle planning after projection review and before
additional provider mutation expansion is justified by external dogfood
feedback and the engineering standard's immutability requirements.

## 10. Test Plan Assessment

The future tests cover:

- all interaction modes and action mappings;
- exact source-decision preservation;
- deterministic reason ordering;
- non-mutation;
- explicit non-enforcement and non-persistence;
- valid serde round trip;
- inconsistent serialized state rejection;
- safe errors, `Debug`, and serialization;
- absence of unstable contextual IDs and payloads;
- existing model and runtime regression suites.

No material test gap remains for this model-only slice.

## 11. Planning Blockers

None after the two plan corrections made during review.

## 12. Non-Blocking Follow-Ups

- Prefer a shared private decision validator if it avoids duplicated
  consistency rules.
- During implementation review, verify the public serialization shape does not
  create an accidental schema commitment beyond the preview model contract.
- Define immutable run-bundle contents in its own subsequent planning phase.

## 13. Recommended Next Phase

Implement the proportional-governance read-only projection model and pure
helper only. Require focused tests, full workspace validation, an implementation
report, and maintainer review.

Do not activate quiet success, integrate the executor, persist decisions, append
events, expose CLI output, or broaden provider mutations.

## 14. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783820758524437000-2`.
- Approval ID:
  `approval/run-1783820758524437000-2/review-scope-approved`.
- Presentation ID: `presentation/1a6404b98e51f3ff`.
- Approval outcome: granted through the proof-enforced approval path.
- Final status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.
- Validation: `npm run check:docs` and `git diff --check` passed. Rust checks
  were not run because this review changed documentation only.
- Out-of-kernel work: Codex inspected the plan, accepted model, tests, roadmap,
  and current immutable identity checks; corrected two planning defects; and
  authored this review. The kernel governed scope and approval but did not edit
  files or perform the review.
- Report posture: this document is the review record. No runtime WorkReport or
  report artifact was generated or persisted.
