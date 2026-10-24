# Trusted-Host Local Cargo Observation And Execution Composition Report

## 1. Executive Summary

The first trusted-host local Cargo observation/execution composition slice is
implemented as a private, in-memory, process-free boundary. It binds the exact
immutable local-check execution contract to the accepted local Cargo observer
request, validated evidence bundle, live-security requirements, and attempt,
then delegates security posture evaluation to Workflow Core.

The slice cannot run Cargo. Missing execution-provider observations produce a
stable `DeniedBeforeStart` assessment, and the coordinator accepts no runner,
process, provider, state, event, report-store, or workflow-mutation capability.

## 2. Scope Completed

- Added read-only accessors for the observer request's immutable run and
  attempt commitments.
- Added a narrow `trusted-host-application-spi`-gated Core assessment entry
  point for the exact local Cargo composition.
- Added one private local-host input and outcome binding immutable execution,
  observation, evidence, security requirements, and assessment.
- Added a domain-separated composition fingerprint.
- Added deterministic pre-start denial when provider observations are absent.
- Added process-free tests for valid denial, substituted binding rejection,
  conservative observer posture, and Debug redaction.

## 3. Scope Explicitly Not Completed

This phase added no standard runner, Cargo process, provider adapter,
OpenShell integration, live smoke, admission change, persistence, event or
audit projection, report generation, schema, CLI or SDK behavior, workflow
mutation, provider write, external SideEffect, nested harness behavior,
hosted execution, or release-posture change.

## 4. Core Assessment SPI

`assess_trusted_host_local_cargo_composition_security` is available only under
the existing unpublished trusted-host application feature. It validates the
immutable execution binding, requires the canonical Cargo-clippy command kind,
and verifies exact run-bundle, command-contract, security-requirement, and
attempt coherence before calling Core's canonical assessment.

The observer request and a future execution-provider request remain distinct
commitments. The SPI does not collapse those identities or treat adjacency as
custody. It returns no execution or workflow authority.

## 5. Private Composition Boundary

`LocalCargoHostObserver::compose_pre_start` runs the accepted injected
observer, revalidates its evidence bundle against the exact request and result,
and invokes the Core assessment SPI. The current boundary accepts only a
`DeniedBeforeStart` assessment. Any satisfied or reconciliation-required
fixture is rejected because no execution provider exists in this slice.

No runner is accepted by the input model. This makes zero process entry a
structural property of the current implementation rather than a caller
promise.

## 6. Pre-Start Denial

Absent provider-owned live-security observations produce
`DeniedBeforeStart` with `ObservationUnavailable`. Native observer facts remain
below `Observed`, and no root continuity is promoted. Successful preflight
observation therefore cannot unlock Cargo execution.

## 7. Privacy And Redaction

The composition stores commitments, closed postures, reasons, and validated
typed values only. It does not copy paths, source or manifest contents,
environment values, command output, Cargo metadata, provider payloads,
credentials, or tokens. Outcome Debug output redacts observation evidence and
composition commitments. Errors use stable generic codes without bound
identity values.

## 8. Test Coverage

Focused tests prove:

- exact current bindings return a deterministic pre-start denial;
- all nine native observer facts remain below `Observed`;
- substituted command/security bindings fail closed;
- the outcome fingerprint is present and Debug-redacted;
- no process is required or available in ordinary composition tests; and
- existing Core live-security and local-host observer tests remain passing.

## 9. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --lib trusted_host_live_execution_security`:
  passed; 15 focused tests.
- `cargo test -p workflow-local-host --lib`: passed; 21 tests passed and one
  opt-in live smoke remained ignored.
- `cargo test --workspace`: passed; ordinary live and sandbox smoke tests
  remained explicitly ignored.
- `npm run check:docs`: passed under the repository-pinned Node 20 runtime.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- The boundary returns only a pre-start denial and cannot execute Cargo.
- No execution provider owns admission-to-terminal custody.
- No provider observations, terminal result, attestation, or report citation
  are produced.
- The composition is private and unpublished.
- Outcomes are in memory and not projected to durable state or events.

## 11. Recommended Next Phase

Perform a focused maintainer and security review of this implementation. The
review should verify exact binding, the separation between observer and future
provider requests, Core ownership of security assessment, deterministic
pre-start denial, structural absence of process authority, redaction, and all
preserved non-goals before provider suitability is reconsidered.

## 12. Governed Phase Evidence

- Workflow ID: `dg/implement`.
- Run ID: `run-1792847157715678000-2`.
- Approval ID:
  `approval/run-1792847157715678000-2/implementation-approved`.
- Approval presentation ID: `presentation/58fe8f2aff392d44`.
- Approval presentation content hash:
  `58fe8f2aff392d4442acf42947a4a1250733afe60452c24a8d3de29288150864`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact private fail-closed composition scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval-presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: focused Core and local-host tests, workspace formatting,
  workspace clippy, workspace tests, documentation validation, and diff checks
  passed. No live or sandbox smoke was run because this phase is process-free.
- Out-of-kernel work: Codex implemented the private Core SPI, local-host
  composition boundary, tests, and documentation and ran validation. The
  kernel governed scope and approval but did not inspect files, edit code or
  documentation, execute tests, or perform git and PR operations.
