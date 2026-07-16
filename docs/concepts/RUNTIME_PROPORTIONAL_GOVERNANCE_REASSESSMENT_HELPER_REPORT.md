# Runtime Proportional-Governance Reassessment Helper Report

## 1. Executive Summary

Workflow OS now has a pure helper that derives ordered proportional-governance
assessments from a validated `StoredImmutableRunBundle` and exactly matched
explicit runtime facts. The result carries a deterministic aggregate
fingerprint and remains assessed-only, in memory, and non-enforcing.

## 2. Scope Completed

- Added `StepGovernanceRuntimeFacts` for exact step-bound facts.
- Added explicit runtime escalation that composes monotonically with static
  escalation declarations.
- Added `ImmutableBundleGovernanceAssessmentRequest`.
- Added ordered per-step and aggregate assessment result types.
- Added `assess_immutable_bundle_governance`.
- Reused the accepted canonical workflow-step derivation and workload selector.
- Added fixed-width domain-separated aggregate fingerprinting.
- Added stable bounded failure codes and redaction-safe `Debug` behavior.
- Added focused behavioral and framing tests.

## 3. Scope Explicitly Not Completed

No executor integration, durable binding, event vocabulary, persistence change,
schema, CLI, UI, provider call, provider write, automatic approval, enterprise
administration, or default behavior was added.

## 4. Source And Fact Boundary

Static workflow, skill, and policy definitions are resolved only from a
validated stored immutable bundle. Mutable project files are not read. The
request must contain exactly one matching runtime-fact record for each ordered
workflow step; missing, duplicate, extra, or mismatched records fail closed.

The helper fingerprints supplied facts but does not independently prove their
freshness. Trusted references, validity windows, durable binding, and
time-of-use reassessment remain future phases.

## 5. Derivation And Assessment Boundary

The existing project-based derivation now delegates to a shared internal
resolved-definition function. The immutable-bundle path uses that same
function, preventing a parallel action, sensitivity, policy-minimum, or
escalation classifier from emerging.

Every step is assessed through the accepted workload selector. Results retain
the existing independent execution and disclosure axes.

## 6. Fingerprint Boundary

The aggregate fingerprint binds a new versioned domain, immutable bundle root,
workflow and run identity, workflow-ordered step identities, assessment
algorithm identities, and per-step input fingerprints. Encoding uses fixed
`u64` length framing. Tests include a stable known vector and a
delimiter-collision regression.

## 7. Privacy And Error Posture

`Debug` output redacts workflow, run, step, and fingerprint values. Validation
errors use stable codes and a fixed bounded message without echoing definition
content, identifiers, hashes, paths, runtime facts, or payloads. The model does
not carry raw evidence, check output, provider payloads, source content,
commands, parser output, environment values, or credentials.

## 8. Test Coverage

Focused tests prove:

- deterministic workflow ordering and repeatability;
- a stable aggregate fingerprint vector;
- decision-relevant runtime-fact invalidation;
- quiet, visible, approval, and denial runtime-escalation composition;
- relevant immutable definitions invalidate while unreferenced definitions do
  not churn the result;
- mutable project files cannot alter stored-bundle assessment;
- missing, duplicate, and extra facts fail closed;
- fixed-width framing prevents delimiter ambiguity;
- `Debug`, errors, and serialization do not copy forbidden payload markers.

## 9. Validation Commands

- Focused immutable-bundle reassessment tests: passed.
- Fixed-width framing unit test: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live integration tests remained
  intentionally ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- Supplied runtime facts have no trusted freshness proof.
- Assessment fingerprints are not durably bound to run identity.
- The executor does not consume or enforce these results.
- Retry and approval resume do not yet reassess proportional governance.
- Visible disclosure has no runtime event or UI projection from this helper.

## 11. Recommended Next Phase

Perform a focused maintainer review of the pure helper. If accepted, proceed to
the separately scoped durable assessment-binding model and event vocabulary.

The initial review found a missing runtime-escalation fact and two missing
definition-boundary tests. The narrow blocker fix is complete and awaits
focused re-review.

## 12. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1784183176746242000-2`
- Approval ID: `approval/run-1784183176746242000-2/implementation-approved`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust and documentation edits, local validation commands,
  test execution, diff inspection, and report drafting
