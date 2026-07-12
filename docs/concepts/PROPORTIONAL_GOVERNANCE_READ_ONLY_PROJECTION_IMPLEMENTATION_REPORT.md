# Proportional Governance Read-Only Projection Implementation Report

## 1. Executive Summary

The first proportional-governance read-only projection is implemented in
`workflow-core`. It converts an already validated
`ProportionalGovernanceDecision` into a bounded in-memory projection with stable
operator-action, decision-posture, and persistence-posture vocabulary.

The projection is explicitly assessed but not enforced and not persisted. It
does not change executor, approval, policy, report, event, CLI, schema, or
provider behavior.

## 2. Scope Completed

- Added closed operator-action vocabulary.
- Added closed non-enforcement and non-persistence posture vocabulary.
- Added a private-field projection model with read-only accessors.
- Added one pure helper that consumes an accepted decision.
- Added exhaustive mode-to-action mapping.
- Reused one bounded internal consistency validator for decision and projection
  deserialization.
- Added custom validated projection deserialization.
- Exported the additive model/helper from `workflow-core`.
- Added focused mapping, non-mutation, serde, consistency, and privacy tests.

## 3. Scope Explicitly Not Completed

- No selector or risk-input changes.
- No executor integration or quiet-success activation.
- No automatic disclosure, approval, denial, or approval-default change.
- No persistence, workflow event, audit record, report, artifact, or inspect
  integration.
- No CLI, schema, example, provider call, provider write, mutation expansion,
  hosted, RBAC, reasoning-lineage, or release change.

## 4. Model And Helper Summary

The implementation adds:

- `GovernanceActionRequirement`
- `GovernanceDecisionPosture`
- `GovernancePersistencePosture`
- `ProportionalGovernanceDecisionProjection`
- `project_proportional_governance_decision`

The helper copies only accepted enum vocabulary and stable reasons. It does not
accept original selector inputs, perform risk selection, or add free-form
rationale.

## 5. Deterministic Mapping

- `QuietCapture` maps to `None`.
- `VisibleDisclosure` maps to `Review`.
- `BlockingApproval` maps to `Approval`.
- `Denied` maps to `Denied`.

Every projection reports `AssessedNotEnforced` and `NotPersisted`. Callers cannot
override these values through the constructor.

## 6. Validation Boundary

Custom deserialization rejects:

- a risk class inconsistent with the mode;
- a decision without the required profile-minimum reason;
- an action requirement inconsistent with the mode;
- unsupported decision or persistence posture values.

Errors use a fixed non-leaking message and do not echo serialized input.

## 7. Privacy And Redaction

The projection stores enum vocabulary and a bounded set of stable reason codes
only. It has no prompt, source, path, actor, evidence, approval, event, report,
provider, timestamp, command-output, or credential field. Derived `Debug` and
serialization therefore expose no caller-supplied free-form values.

## 8. Tests Added

Focused tests prove:

- every interaction mode maps to the expected operator action;
- mode, risk class, and reasons match the accepted source decision;
- projection does not mutate the source decision;
- valid serde round trip preserves posture;
- inconsistent action and decision combinations fail closed;
- `Debug` and serialization remain payload-free;
- existing selector tests continue to pass.

## 9. Governed Phase Evidence

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783820962492676000-2`.
- Approval ID:
  `approval/run-1783820962492676000-2/implementation-approved`.
- Presentation ID: `presentation/de9ee71b80d6009a`.
- Approval outcome: granted through the proof-enforced approval path.
- Phase status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.

## 10. Validation

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test proportional_governance`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Out-Of-Kernel Work

Codex edited the Rust model, exports, tests, roadmap, plans, and this report and
ran the validation commands. The kernel governed phase scope and approval but
did not edit files, run tests, persist a WorkReport, or create an artifact.

No git commit, push, pull request, provider call, or external write occurred in
this implementation phase before validation.

## 12. Remaining Limitations

- Focused implementation review subsequently found the invalid-input error
  redaction blocker documented in
  [Proportional Governance Read-Only Projection Implementation Review](PROPORTIONAL_GOVERNANCE_READ_ONLY_PROJECTION_IMPLEMENTATION_REVIEW.md).
  The blocker is fixed and accepted in the later blocker-fix report and review.
- No runtime consumer exists.
- The projection is not a durable decision record.
- Quiet success, disclosure, approval, and denial are not activated by it.
- Immutable run-bundle hardening remains future work.

## 13. Recommended Next Phase

Historical recommendation at phase close: perform focused maintainer review.
That review, blocker fix, and re-review are complete. The current next phase is
immutable run-bundle hardening planning before additional provider mutation
expansion.
