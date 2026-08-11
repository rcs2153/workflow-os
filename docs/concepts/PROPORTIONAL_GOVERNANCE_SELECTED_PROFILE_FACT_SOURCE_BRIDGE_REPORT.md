# Proportional-Governance Selected-Profile Fact-Source Bridge Report

## 1. Executive Summary

Workflow OS now has a private Core-owned runtime-fact source bridge for the
selected authoritative local project-validation profile. The bridge executes
the canonical local check exactly once, derives its evidence/check posture
inside Core, resolves the complete current fact set through a fixed registered
source, and returns a payload-free source-backed governance binding.

No executor or CLI path consumes the bridge yet.

## 2. Scope Completed

- Added a private selected-profile bridge beside the accepted authoritative
  local-check reassessment.
- Fixed source identity, contract version, configuration commitment, and
  freshness policy inside Core.
- Derived selected evidence/check posture only from the actual same-call check.
- Reused the generic registered-source freshness, coverage, assessment, and
  snapshot boundary.
- Failed closed if the source-backed assessment differs from the accepted
  authoritative reassessment.
- Added a test-only semantic equivalence matrix.

## 3. Scope Explicitly Not Completed

The phase did not add a selected-consumer product API, executor integration,
CLI behavior, automatic approval, persistence, caller-supplied authority,
multi-step expansion, providers, OpenShell, SideEffect execution, writes,
schemas, examples, hosted behavior, or release changes.

## 4. Bridge API Summary

`compose_authoritative_local_check_runtime_fact_source_bridge` accepts the
existing private reassessment input plus one Core-selected evaluation time. It
returns local-check results, a `GovernanceAssessmentBinding` carrying a current
runtime-fact snapshot commitment, and the validated payload-free snapshot.

The bridge and its input/output types are crate-private. Callers cannot provide
or override the source registration.

## 5. Fact And Source Boundary

The bridge first completes existing reassessment preflight and runs the
canonical check once. It injects only the derived evidence/check posture into
the copied exact fact set. Its private source observes that set against the
exact immutable bundle at the generic consumer's evaluation time.

The fixed source uses a one-second maximum age and a snapshot identity derived
from the accepted authoritative reassessment fingerprint. The snapshot remains
payload-free and does not become reusable authority.

## 6. Equivalence Boundary

The bridge compares the complete source-backed assessment set with the existing
authoritative reassessment set. A mismatch returns the stable
`local_check_attestation.runtime_fact_source_bridge.equivalence_mismatch`
error and no bridge outcome.

The test matrix proves equivalent aggregate fingerprint, execution,
disclosure, completeness, result count, and process-call count for:

- required check satisfied;
- optional check omitted;
- required check omitted;
- satisfied check with unavailable authority; and
- executed optional check failure.

## 7. Privacy And Failure Posture

Debug output redacts source identity, source version, snapshot identity, bundle
binding, results, commitments, and runtime facts. Errors use stable Core-owned
codes and fixed messages. Caller-supplied evidence/check posture fails before
clock or process access. Source-backed assessment or binding failure returns no
partial bridge value.

## 8. Test Coverage

Focused tests cover one-call execution, source-backed binding, accepted
snapshot posture, fixed registration across changing facts, the equivalence
matrix, caller-posture rejection before execution, and Debug non-leakage.
Existing local-check, executor, runtime-fact, report, adapter, hosted, and
workspace tests remain green.

## 9. Validation

- Focused local-check runtime tests: 37 passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; intentional opt-in live tests remained
  ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- No product path invokes the bridge.
- Approval resume, trusted receipt derivation, and report-artifact persistence
  are not composed through this bridge yet.
- The existing authoritative product path remains unchanged and available.
- Multi-step authoritative governance and arbitrary local checks remain out of
  scope.

## 11. Recommended Next Phase

Implement and review one additive selected-consumer composition API that owns
this private bridge and reuses the accepted approval-to-authority-receipt-to-
report-artifact closure. Keep CLI adoption separate.

## 12. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1786439617074633000-2`
- Approval ID:
  `approval/run-1786439617074633000-2/implementation-approved`
- Presentation ID: `presentation/96c7cf6e060f8607`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Presentation enforcement: proof-enforced with event marker present
- Out-of-kernel work: Rust implementation, tests, documentation, and review
