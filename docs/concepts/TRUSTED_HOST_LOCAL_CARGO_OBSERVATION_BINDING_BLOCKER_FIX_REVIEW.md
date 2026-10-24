# Trusted-Host Local Cargo Observation Binding Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to execution-provider suitability re-evaluation.**

The private pre-start coordinator can no longer receive execution-provider
observations that it does not canonically commit. It always routes explicit
observation absence through the Core assessment SPI and can therefore return
only the honest `DeniedBeforeStart` posture with `ObservationUnavailable`.
The original identity-collapse and reconciliation-loss paths are unreachable
from this local coordinator.

## 2. Scope Verification

The fix stayed within the approved blocker boundary. It removed one private
input, retained the future-facing Core SPI, added focused deterministic tests,
and updated documentation. It added no runner, Cargo process, live smoke,
provider adapter, OpenShell selection or wiring, admission change,
persistence, event or audit projection, report generation, schema, CLI or SDK
behavior, provider write, external SideEffect, hosted execution, authority
expansion, or release-posture change.

## 3. Original Blocker Restatement

The original private input accepted optional
`TrustedHostLiveExecutionSecurityObservations`, while the returned outcome and
composition fingerprint retained only the derived assessment. Different
provider observation sets capable of producing the same disposition and
reason vector could collapse to one identity. A possibly-started provider
posture could also be reduced to a generic execution-unavailable error.

## 4. Fix Approach Assessment

The selected fix is the smallest safe option. The private
`LocalCargoObservationExecutionInput` no longer has a provider-observation
field. `compose_pre_start` passes `None` to the Core SPI directly. This makes
the local coordinator's current contract explicit: it composes process-free
host input observation with proof that no accepted execution-provider
observation exists.

The provider-neutral Core SPI remains feature-gated and can still assess a
future exact provider observation set. That capability is not exposed through
the local coordinator and grants no execution or workflow authority.

## 5. Exact Binding Assessment

The coordinator continues to bind:

- the immutable local-check execution binding;
- the observer request and exact request fingerprint;
- the observer result and validated evidence-result commitment;
- the live-security requirements fingerprint;
- the closed security disposition; and
- canonical stable security-reason codes.

No provider claim enters this identity. The observer request remains distinct
from a future provider observation contract.

## 6. Denial And Reconciliation Assessment

Because the coordinator supplies explicit absence, the Core SPI returns
`DeniedBeforeStart` with exactly `ObservationUnavailable`. The input surface
contains no runner or provider capability, so the no-start posture is
structural.

The coordinator can no longer receive a possibly-started observation and
therefore cannot collapse `ReconciliationRequired`. Any future provider-owned
composition must canonically bind the complete observation set and preserve
reconciliation as a first-class outcome.

## 7. Fingerprint Assessment

The domain-separated composition fingerprint remains deterministic for
unchanged inputs. Focused coverage now proves identical replay equality and
changed static-request commitment sensitivity. Existing substitution tests
prove a mismatched execution/request binding fails before producing an
outcome.

The tests do not attempt exhaustive per-field mutation of every nested
commitment. That is non-blocking because each nested model already supplies a
validated canonical fingerprint and the composition hashes those identities.

## 8. Privacy And Error Assessment

The fix stores and exposes no paths, source or manifest contents, environment
values, Cargo metadata, command output, provider payloads, credentials, or
tokens. Debug output redacts the composition and evidence identities. Errors
remain stable and non-leaking, and no security failure becomes a user-project
diagnostic.

## 9. Test Quality Assessment

Focused tests now cover:

- the exact missing-observation denial and reason;
- inability to supply provider observations through the private input;
- conservative native-observer facts;
- substituted execution/request binding rejection;
- deterministic unchanged composition replay;
- changed-request fingerprint sensitivity;
- Debug redaction; and
- process-free ordinary execution.

The workspace suite also preserves existing Core, executor, hosted, report,
adapter, validation, and runtime behavior.

## 10. Documentation Assessment

The roadmap, composition plan, original review fix-forward note, and blocker
fix report consistently describe the current slice as private, process-free,
pre-start-only, and incapable of running Cargo. None claims provider custody,
admission, persistence, reporting, or runtime exposure.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Retain the feature-gated Core SPI until a provider contract can commit an
  exact observation set and preserve reconciliation.
- Keep direct wrong-command and missing-observation Core SPI coverage.
- Do not treat native host-input observation as containment attestation.
- Re-evaluate provider suitability against the already documented mandatory
  effective-control, lifecycle, cleanup, and reconciliation facts before any
  wiring.

## 13. Recommended Next Phase

**Trusted-host local Cargo execution-provider suitability re-evaluation.**

Re-evaluate the reviewed native and optional OpenShell boundaries against the
exact observation and attestation requirements. This is a bounded decision
phase, not provider implementation. OpenShell wiring remains blocked unless
authoritative upstream execution and cleanup attestation plus a live sandbox
smoke can satisfy the existing contract. A fork remains unjustified.

## 14. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --lib trusted_host_live_execution_security`:
  passed; 15 tests passed.
- `cargo test -p workflow-local-host --lib`: passed; 22 tests passed and one
  opt-in live smoke remained ignored.
- `cargo test --workspace`: passed; ordinary opt-in live and sandbox smokes
  remained ignored.
- `npm run check:docs`: passed under the repository-pinned Node 20 runtime.
- `git diff --check`: passed.

## 15. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792848956524112000-2`.
- Approval ID:
  `approval/run-1792848956524112000-2/review-scope-approved`.
- Approval presentation ID: `presentation/7e6ca3a7a52a24c2`.
- Approval presentation content hash:
  `7e6ca3a7a52a24c29d0dc93eeebba799dd2ed411f86ebdf73be957a90a1e9884`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval-presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: formatting, workspace Clippy, focused Core and
  local-host tests, the full workspace suite, docs checks, and diff hygiene
  passed. Final documentation validation and diff hygiene also passed after
  this review document and roadmap update were added.
- Out-of-kernel work: the delegated maintainer inspected the private
  coordinator, Core SPI, exact fingerprint inputs, focused tests, roadmap,
  plan, original review, and fix report; authored this review; and ran
  validation. The kernel governed scope and approval but did not inspect
  files, edit documentation, execute tests, or perform git and PR operations.
