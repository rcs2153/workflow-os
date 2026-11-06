# Governed Obligation Plan Phase 3 Authoritative Oracle Correction Report

## Status

The focused Phase 3 differential-oracle blocker is corrected. A second focused
maintainer/security review is required before Phase 3 can be accepted or Phase
4 runtime composition can be authorized.

## Corrected Boundary

The private comparator no longer defines or accepts a parallel
`GovernedObligationCanonicalOutcome` vocabulary. Its observed side now carries
existing Core decision types directly:

- `PolicyDecision`, produced by `ConservativePolicyEngine`; and
- `ProportionalGovernanceWorkloadAssessment`, produced by
  `assess_proportional_governance_workload`.

Unsupported explicit governance requirements are represented by the exact
`WorkflowOsError` returned from the existing selector. The comparator accepts
only the bounded obligation-to-decision pairings covered by this phase. Any
other kind, posture, evidence class, decision type, denial reason, or error code
fails closed.

The comparator remains private, non-authorizing, and disconnected from runtime
control flow. It does not recompute policy or governance selection. It checks
whether independently supplied fixed obligations agree with decisions already
returned by existing Core paths.

## Representative Evidence

The approval observation is produced by the existing conservative policy
engine from a typed `require_approval` policy effect. The accepted decision is
allowed but requires approval.

The local-check observation is produced by the existing proportional-
governance workload assessment with required evidence/checks unavailable. The
accepted decision is denied and carries the existing
`EvidenceOrCheckRequirement` reason.

The omitted-gate negative fixture removes the local-check obligation from the
derived plan while independently proving that:

- the policy decision still requires approval; and
- the evidence/check decision still denies execution.

The complete independent source-closure and inventory commitments detect the
omission before a permissive result can be selected.

Unsupported local-check posture is tested against the exact existing
`governance.proportional.requirement.unsupported` error with `Unsupported`
kind. Runtime `SideEffect` observations remain non-authorizing facts and cannot
manufacture an obligation.

## Added Fail-Closed Coverage

Focused regressions now prove rejection of:

- duplicate independently authored expectations;
- duplicate authoritative Core decisions;
- authoritative decisions outside the derived plan; and
- a policy decision supplied where an evidence/check decision is required.

The existing stale-closure, inventory-omission, unsupported-disclosure,
ordering, redaction, and `SideEffect` non-inference tests remain in place.

## Validation

- `cargo test -p workflow-core --lib governed_obligation_plan`: 21 passed;
  zero failed.
- `cargo test -p workflow-core`: passed, including 769 library tests and all
  integration and documentation test targets; zero failures.
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`:
  passed.
- `cargo fmt --all --check`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## Governed Phase

- workflow: `dg/blocker`
- run: `run-1793962509423845000-2`
- approval: `approval/run-1793962509423845000-2/fix-approved`
- presentation: `presentation/0893da21575ddd2d`
- approval outcome: granted with persisted presentation proof

Workflow OS governed the correction scope and approval. Repository edits, test
commands, documentation, Git, PR, and merge work occur outside the kernel and
remain disclosed. No runtime composition, provider action, independent
third-party review, or release behavior is simulated.

## Residual Boundary

The evidence uses existing deterministic Core decision functions, not a full
`LocalExecutor` run. That is sufficient for this private differential slice
only if focused review accepts the exact type and decision-reason bindings.
Phase 4 remains responsible for one separately reviewed opt-in runtime
composition after acceptance.

The model still has no canonical source extractor or reviewed runtime
cardinality limits. It remains crate-private and non-serializing. No schema,
SDK, CLI, persistence, provider, dependency, Salsa, release, or publication
behavior is added.

## Next Authorized Phase

Perform a second focused maintainer/security review of this correction. Review
must verify that authoritative decision provenance is credible, exact decision
type and reason matching fails closed, unsupported errors cannot be widened,
duplicate and unexpected observations are rejected, and no parallel evaluator
or runtime authority has been introduced.
