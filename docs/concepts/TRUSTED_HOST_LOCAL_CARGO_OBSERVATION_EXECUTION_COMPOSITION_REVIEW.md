# Trusted-Host Local Cargo Observation And Execution Composition Review

## 1. Executive Verdict

**Needs blocker fixes.**

The process-free composition slice correctly binds immutable execution,
observer request and result, evidence, live-security requirements, and attempt;
uses Core-owned security assessment; returns a stable pre-start denial when
provider observations are absent; and exposes no runner or process capability.

One exact-binding blocker remains. The private input accepts optional
`TrustedHostLiveExecutionSecurityObservations`, but the composition fingerprint
and returned outcome retain only the derived assessment. Distinct provider
observation sets that produce the same disposition and reason vector can
therefore collapse to one composition identity. Provider suitability work must
not begin until this input is removed from the pre-start-only slice or is
canonically committed and retained.

## 2. Scope Verification

The phase stayed inside its approved private, in-memory, process-free scope.
It introduced no standard runner, Cargo process, live smoke, OpenShell wiring,
admission change, persistence, events, audit projection, report generation,
schema, CLI or SDK behavior, provider write, external SideEffect, hosted
execution, authority broadening, or release-posture change.

## 3. Core SPI Assessment

The feature-gated Core SPI is appropriately narrow and pure. It validates the
immutable execution binding, restricts the command to the canonical Cargo
clippy profile, and binds run bundle, command contract, security requirements,
and attempt to the observer request before using Core's canonical security
assessment.

The implementation correctly keeps the observer request and future provider
request as distinct commitments. A provider request is not falsely equated to
the observer request fingerprint, and the returned assessment grants no
execution or workflow authority.

## 4. Observer And Evidence Binding Assessment

The local coordinator runs the accepted observer, validates its exact evidence
bundle, and includes the observer request, result, and validated result
commitment in the composition fingerprint. Evidence entries are fully checked
against the result's root and fact references, producer, revision, and
commitments, so the result commitment is sufficient for this first private
observer bundle.

All nine native facts remain below `Observed`. No root continuity or effective
control is promoted, and successful preflight observation cannot unlock Cargo.

## 5. Live-Security Observation Blocker

`LocalCargoObservationExecutionInput` accepts
`Option<&TrustedHostLiveExecutionSecurityObservations>`. Core correctly
assesses a supplied set, but `local_cargo_composition_fingerprint` receives
only the resulting assessment. `LocalCargoObservationExecutionOutcome` also
stores only that assessment.

This loses the exact provider-observation identity. Two different valid
observation sets can yield the same disposition and reasons, producing the
same composition fingerprint and indistinguishable outcome. The plan requires
the complete attempt and future provider observations to be exactly bound.

The same path converts any non-`DeniedBeforeStart` assessment into a generic
`composition.execution_unavailable` error. That is fail-closed, but it does
not preserve a possibly-started `ReconciliationRequired` posture if this input
surface is retained for future-provider fixtures.

Required blocker fix, choose the smallest safe option:

1. remove provider observations from this pre-start-only coordinator and
   always invoke the Core SPI with absence, deferring supplied observations to
   a later provider-owned contract; or
2. add a canonical Core-owned observation commitment, bind it into the
   composition fingerprint and outcome, and preserve reconciliation posture.

The first option is recommended because this slice has no provider or process
authority and is intended to prove absence-driven pre-start denial only.

## 6. Pre-Start Denial And Process Boundary

Absent provider observations deterministically return `DeniedBeforeStart`
with `ObservationUnavailable`. The coordinator input contains no runner,
process, provider, state-store, event, report-store, or workflow-mutation
capability. Zero process entry is therefore structural in this slice rather
than dependent on a caller-supplied spy.

Rejecting `Satisfied` at this boundary is correct while no execution provider
exists. Any future possibly-started path must preserve reconciliation instead
of mapping it to an ordinary pre-start failure.

## 7. Fingerprint Assessment

The composition fingerprint is domain-separated and binds immutable execution,
observer request, observer result, validated evidence result commitment,
security requirements, disposition, and canonical stable reason codes. It no
longer depends on Rust Debug names.

Focused coverage proves the fingerprint is present and redacted, but does not
yet prove deterministic unchanged replay or changed-input sensitivity. Those
tests are required in the blocker fix, especially for provider-observation
presence or substitution if that input remains.

## 8. Privacy And Error Assessment

The implementation remains payload-free. It does not copy paths, source or
manifest contents, Cargo metadata, environment values, command output,
provider payloads, credentials, or tokens. Outcome Debug redacts evidence and
commitments. Stable errors do not echo bound identities.

No security failure is converted into a compiler or user-project diagnostic.

## 9. Test Quality Assessment

Existing focused tests cover:

- deterministic pre-start disposition and reason for missing observations;
- conservative native observer posture;
- substituted execution/request binding rejection;
- Debug redaction; and
- process-free local-host behavior.

Missing blocker-level coverage:

- exact supplied provider-observation commitment or explicit rejection;
- distinction between absent and supplied observation identity;
- unchanged composition fingerprint replay;
- changed-input fingerprint sensitivity; and
- preservation of reconciliation if supplied observations remain accepted.

## 10. Documentation Assessment

The plan, roadmap, and implementation report correctly state that the slice is
private, process-free, in memory, and incapable of Cargo execution. They do not
claim provider custody, admission, persistence, reporting, or runtime exposure.

The roadmap is updated to make the observation-binding blocker the only active
next phase and to keep provider suitability work deferred.

## 11. Blockers

1. **Exact provider-observation identity is not bound.** Optional live-security
   observations affect the assessment but not the composition fingerprint or
   returned outcome. Remove that input from this slice or add a canonical
   Core-owned observation commitment and preserve its exact posture.

## 12. Non-Blocking Follow-Ups

- Add direct focused tests for the Core SPI's wrong-command and missing-
  observation paths.
- Keep the SPI feature-gated and unpublished.
- Retain explicit stable reason codes in the composition fingerprint.
- Continue to treat the absence of a runner as the strongest no-start proof
  for this slice.

## 13. Recommended Next Phase

**Local Cargo composition observation-binding blocker fix.**

Prefer removing supplied provider observations from the pre-start-only private
coordinator, while retaining the Core SPI's future optional-observation
assessment boundary. Add deterministic and substitution tests, then perform a
focused blocker-fix review before any provider suitability re-evaluation.

## 14. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --lib trusted_host_live_execution_security`:
  passed; 15 focused tests.
- `cargo test -p workflow-local-host --lib`: passed; 21 tests passed and one
  opt-in live smoke remained ignored.
- `cargo test --workspace`: passed; ordinary opt-in live and sandbox smoke
  tests remained ignored.
- `npm run check:docs`: passed under the repository-pinned Node 20 runtime.
- `git diff --check`: passed.

## 15. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792848268959691000-2`.
- Approval ID:
  `approval/run-1792848268959691000-2/review-scope-approved`.
- Approval presentation ID: `presentation/9ac058ea1c39a0dd`.
- Approval presentation content hash:
  `9ac058ea1c39a0dde01d7e4d569fe51a0295f3dd79a22d95a8b473cc2a43ae8e`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval-presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: focused Core and local-host tests, workspace formatting,
  workspace clippy, workspace tests, documentation validation, and diff checks
  passed. No opt-in live or sandbox smoke was run.
- Out-of-kernel work: the delegated maintainer inspected the Core SPI, private
  local-host composition, tests, plan, implementation report, and roadmap;
  authored this review and roadmap queue update; and ran validation. The kernel
  governed scope and approval but did not inspect files, edit documentation,
  execute validation, or perform git and PR operations.
