# Trusted-Host Local Cargo Observation Binding Blocker Fix Report

## 1. Executive Summary

The local Cargo composition observation-binding blocker is fixed. The private
pre-start-only coordinator no longer accepts execution-provider observations.
It always passes explicit observation absence to the Core-owned assessment SPI
and therefore returns the honest current `DeniedBeforeStart` posture without
collapsing any uncommitted provider claim into the composition identity.

## 2. Blocker Fixed

Focused review found that optional provider observations affected the security
assessment but were absent from both the returned outcome and composition
fingerprint. Distinct provider claims with the same assessment could therefore
share one identity.

The blocker fix removes that optional input from
`LocalCargoObservationExecutionInput`. The local coordinator cannot receive a
provider observation set until a later provider-owned contract defines its
canonical commitment and reconciliation behavior.

## 3. Implementation Approach

`LocalCargoHostObserver::compose_pre_start` retains the same exact immutable
execution, observer request, evidence, security-requirement, and attempt
bindings. It calls the Core SPI with `None` for provider observations. The Core
SPI remains feature-gated and capable of future assessment, but the current
local composition exposes only the absence-driven branch.

No runner or process capability was added. A future provider cannot enter this
input surface by supplying an unbound claim.

## 4. Composition Identity

The existing domain-separated fingerprint continues to bind immutable local-
check execution, observer request and result, validated evidence result,
security requirements, disposition, and stable reason codes.

New focused coverage proves identical input replay returns the same
composition fingerprint and a changed bound static-request commitment changes
the fingerprint.

## 5. Security And Reconciliation Boundary

This slice can now represent only current pre-start denial. It cannot accept a
provider observation claiming start, terminality, effective controls, or
cleanup, so it cannot erase a reconciliation posture. Provider-owned
observations and possibly-started semantics remain deferred to a separately
reviewed provider composition.

The Core SPI's assessment of future provider observations grants no execution,
workflow, policy, approval, or capability authority.

## 6. Privacy And Redaction

The fix adds no payload or identity exposure. Paths, source and manifest
contents, environment values, Cargo metadata, command output, provider
payloads, credentials, and tokens remain absent. Debug output and stable errors
remain non-leaking.

## 7. Scope Explicitly Not Completed

This fix added no runner, Cargo process, live smoke, provider adapter,
OpenShell integration, admission change, persistence, event or audit
projection, report generation, schema, CLI or SDK behavior, provider write,
external SideEffect, hosted execution, authority expansion, or release change.

## 8. Test Coverage

Focused tests cover:

- explicit missing-provider-observation pre-start denial;
- conservative native observer posture;
- substituted execution/request binding rejection;
- deterministic unchanged composition replay;
- changed-request fingerprint sensitivity;
- Debug redaction; and
- process-free ordinary execution.

## 9. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --lib trusted_host_live_execution_security`:
  passed; 15 tests passed.
- `cargo test -p workflow-local-host --lib`: passed; 22 tests passed and one
  opt-in live smoke remained ignored.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- The composition cannot run Cargo and returns only a pre-start denial.
- No provider owns admission-to-terminal custody.
- Provider observations have no canonical composition commitment yet.
- No terminal result, attestation, report citation, persistence, or event is
  produced.
- The boundary remains private and unpublished.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Verify the
provider-observation input is absent, the Core SPI remains future-facing and
non-authorizing, fingerprints are deterministic and input-sensitive, the
pre-start denial is exact, and every execution non-goal remains intact before
provider suitability is reconsidered.

That review is complete and accepted in
[Trusted-Host Local Cargo Observation Binding Blocker Fix Review](TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_BINDING_BLOCKER_FIX_REVIEW.md).

## 12. Governed Phase Evidence

- Workflow ID: `dg/blocker`.
- Run ID: `run-1792848564957590000-2`.
- Approval ID: `approval/run-1792848564957590000-2/fix-approved`.
- Approval presentation ID: `presentation/07b39ff904b0c6fd`.
- Approval presentation content hash:
  `07b39ff904b0c6fd9d140d5004d0e754d3913445b8d63da38d811d082ad90b82`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact observation-binding blocker fix.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval, with zero retries and zero
  escalations. Approval presentation enforcement was proof-enforced and the
  presentation marker is present in the event trail.
- Validation summary: formatting, workspace Clippy, focused Core and local-host
  tests, the full workspace test suite, documentation checks, and diff hygiene
  all passed.
- Out-of-kernel work: Codex removed the unbound provider-observation input,
  added focused deterministic tests, updated documentation, and ran validation.
  The kernel governed scope and approval but did not inspect files, edit code
  or documentation, execute tests, or perform git and PR operations.
