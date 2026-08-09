# Proportional-Governance Approval-Resume Runtime-Fact Source Consumer Report

## 1. Executive Summary

Workflow OS now has one explicit local approval-resume API that consumes fresh
facts from the registered runtime-fact source committed when the run began. A
grant freezes the approved resume context, validates the durable V3 source
commitment, resolves one fresh payload-free snapshot, and requires the new
assessment to reproduce durable governance state before any grant, resume,
policy, or skill event is appended.

Denial remains source-free. The phase does not activate proportional governance
by default or add providers, SideEffect execution, writes, schemas, CLI/UI
behavior, report persistence, or reusable authority.

## 2. Scope Completed

- Added an explicit source-backed approval-decision request and result.
- Added one opt-in local approval-resume source consumer.
- Reused immutable run bundles, registered source validation, V3 snapshot
  commitments, and existing approval application semantics.
- Froze and validated the approved resume plan before source invocation.
- Required fresh grant-time facts to reproduce the durable assessment.
- Preserved the initial durable snapshot commitment as provenance.
- Kept denial available without source health.
- Added focused ordering, privacy, compatibility, and regression tests.

## 3. Scope Explicitly Not Completed

The phase did not change default approval APIs, automatically activate
proportional governance, grant approvals, persist raw facts, replace the initial
snapshot commitment, add presentation-proof composition, create report
citations, change schemas or CLI/UI behavior, invoke providers, integrate
OpenShell, execute SideEffects, add writes, expand hosted behavior, add
enterprise identity, or change release posture.

## 4. API Summary

`LocalCurrentRuntimeFactsGovernanceApprovalDecisionRequest` carries the existing
approval request, explicit governance profile, registered source commitment,
decision-time evaluation timestamp, and optional expected aggregate
fingerprint.

`decide_approval_with_current_runtime_facts_governance_reassessment(...)`
accepts the executor, immutable-bundle store, injected source, and explicit
request. A granted result returns the run, unchanged durable assessment binding,
and accepted decision-time snapshot. A denied result returns the terminal run
without a fabricated binding or snapshot.

## 5. Grant Ordering And Atomicity

The implementation prepares the approval, reconstructs and freezes the exact
approved resume plan, validates the stored immutable bundle and durable V3
assessment, compares source-registration commitments, resolves and assesses
fresh facts, and only then appends approval/resume events and executes the
frozen plan.

Failure at any precondition leaves the run waiting with exactly equal event
history and no skill invocation. This ordering closes both fact freshness and
resolved-context time-of-check/time-of-use gaps for the new path.

## 6. Denial Semantics

A denied decision cannot resume or expand authority. It therefore follows the
existing fail-closed denial path without reading the immutable-bundle store for
source authority or invoking the source. The result includes no decision-time
snapshot.

## 7. Provenance And Compatibility

Only V3 durable assessment bindings with a runtime-fact snapshot commitment are
accepted. V1 and V2 remain valid for their existing caller-fact paths but are
not treated as source-backed approval authority.

The decision-time snapshot may have a new snapshot identity and observation
time, but it must preserve the registered source, exact bundle, complete fact
coverage, and durable assessment result. It does not overwrite the initial
snapshot commitment or become reusable authority.

## 8. Privacy And Error Posture

Request and result Debug output omit paths, source IDs, snapshot IDs, bundle
IDs, timestamps, approval details, and fact values. Source-local failures remain
wrapped by the stable Core-owned source error. New executor errors use bounded
codes and do not include caller values. No raw runtime facts are appended to
events or persisted by this API.

## 9. Test Coverage

Focused tests cover matching grant-time reassessment, a fresh snapshot result,
unchanged initial provenance, changed-fact rejection with exact event equality,
registration mismatch before source access, source-free denial, changed
workflow context before source access, non-leaking source failure, V1/V2
compatibility rejection, no skill invocation on failure, and redacted Debug
output. Existing workspace tests remain the regression boundary.

## 10. Validation Commands And Results

- Focused approval-resume runtime-fact tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed in the required PR Rust job; the equivalent
  local run was stopped after passing CLI unit and integration sets because
  macOS process startup made the 64-binary run impractical.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Limitations

- The API is explicit and opt-in rather than a default executor path.
- Presentation-proof enforcement is not composed into this source consumer.
- Changed assessments fail closed rather than route to deterministic escalation.
- Decision-time snapshots are returned but not persisted or cited in reports.
- Registration is an embedding trust decision, not authenticated remote
  attestation.
- Production time authority remains separately scoped.

## 12. Recommended Next Phase

Add a proof-enforced approval wrapper that reuses this private grant
precondition and frozen-plan boundary. Do not broaden proportional-governance
defaults or provider mutation families first.

## 13. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786284384652726000-2`
- Approval ID: `approval/run-1786284384652726000-2/composition-approved`
- Presentation ID: `presentation/ec421e5cdf4c2feb`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR work
- Missing coverage: proof-enforced composition, default activation, report
  citation, providers, OpenShell, SideEffects, and writes were disclosed and
  not simulated
