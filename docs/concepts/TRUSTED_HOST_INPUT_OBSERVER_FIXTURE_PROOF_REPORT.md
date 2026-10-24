# Trusted-Host Input Observer Fixture Proof Report

## 1. Executive Summary

The first fixture-backed in-memory trusted-host input observer proof is
implemented. An explicit synthetic fixture source can now produce a complete,
exact-request-bound `TrustedHostInputObservationResult` through the accepted
Core constructors without reading the host or running a process.

The proof is non-authorizing and does not establish any fact about a live
machine.

## 2. Scope Completed

- Added `TrustedHostInputObservationFixture` as an explicit synthetic fact
  source.
- Added `observe_trusted_host_inputs_from_fixture` as a pure constructor-backed
  proof helper.
- Restricted the fixture path to `Fixture` producers with `FixtureOnly`
  assurance.
- Derived request fingerprint, repository tree, and ordering authority from
  the validated request rather than accepting duplicate caller bindings.
- Reused `TrustedHostInputObservationResult::new` and
  `validate_for_request` as the completeness, coherence, ordering, freshness,
  and substitution enforcement boundary.
- Added public-boundary integration tests.

## 3. Scope Explicitly Not Completed

This phase added no live host inspection, filesystem or environment access,
process execution, provider selection or integration, runtime admission,
persistence, workflow events, schemas, CLI behavior, examples, writes,
capability grants, or release changes.

## 4. Helper API Summary

`TrustedHostInputObservationFixture::new` accepts one validated fixture
producer, ordered revisions, one validated repository-root observation, and
the explicit synthetic fact set. It rejects non-fixture producers with a
stable non-leaking error.

`observe_trusted_host_inputs_from_fixture` derives the exact request bindings,
constructs the result through the existing Core constructor, and validates the
result against the exact request before returning it.

## 5. Determinism And Binding Summary

Repeated observation of the same request and fixture produces equal results
and identical canonical fingerprints. Result construction sorts the closed
nine-obligation set canonically. Producer substitution, stale ordering,
incomplete facts, root mismatch, and every existing result-coherence failure
remain fail closed.

The helper does not create a reusable authority or weaken the request/result
binding model.

## 6. Privacy And Redaction Summary

The fixture stores validated payload-free model values only. Debug output
reports producer posture, ordered revisions, root posture, and fact count
without printing fact commitments or references. Tests prove raw input labels
used to derive commitments are not copied into Debug or serialized results.

Errors use stable generic codes and do not echo rejected producer or fact
values.

## 7. Test Coverage Summary

Focused tests prove:

- the same explicit source produces the same complete result twice;
- all nine obligations are present and canonically ordered;
- exact request/producer substitution fails closed;
- stale fixture revisions fail closed;
- incomplete facts fail through the Core result constructor;
- non-fixture producers cannot enter the fixture path; and
- Debug and serialized results do not copy raw payload markers.

## 8. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live provider tests remained ignored
  behind their existing environment gates.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Focused fixture proof tests: six passed.

## 9. Remaining Known Limitations

- The helper proves model composition only; its facts are synthetic.
- No live producer, ordering authority, host observer, or provider is trusted
  or operated.
- No observation is persisted or consumed by runtime admission.
- Static-plus-host assessment and live provider capability/evidence mapping
  remain separate future phases.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of the fixture proof. Verify the
pure boundary, fixture-only producer restriction, constructor reuse,
determinism, substitution rejection, privacy, and preservation of the
non-authorizing posture before planning one live provider implementation.

## 11. Governed Phase Evidence

- Workflow ID: `dg/implement`.
- Run ID: `run-1792839582366740000-2`.
- Approval ID:
  `approval/run-1792839582366740000-2/implementation-approved`.
- Approval presentation ID: `presentation/18c9e40df01dc51f`.
- Approval presentation content hash:
  `18c9e40df01dc51f6fdd2aa396e0e5edd3ebf3d0fc0c397c50306004b1dfb227`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact fixture-proof implementation scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Out-of-kernel work: Codex implemented the helper and tests, authored this
  report, and ran validation. The kernel governed scope and approval but did
  not edit files or execute validation commands.
