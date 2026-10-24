# Trusted-Host Input Observer Fixture Proof Review

## 1. Executive Verdict

Phase accepted; proceed to separate planning for one live provider
implementation and a provider capability/evidence matrix.

The fixture proof is a narrow, deterministic composition proof. It exercises
the accepted trusted-host observation request/result boundary without reading
the host, executing a process, selecting a provider, or granting runtime
authority.

## 2. Scope Verification

The phase stayed within the approved fixture-only scope. It added one explicit
synthetic fixture type, one pure construction helper, public-boundary tests,
documentation, and exports.

It did not add live host inspection, filesystem or environment access,
process execution, provider selection or integration, runtime admission,
persistence, events, schemas, CLI behavior, writes, capability grants, or
release-posture changes.

## 3. Helper And API Assessment

`TrustedHostInputObservationFixture` is appropriately small. It stores an
already-validated producer, revisions, repository-root observation, and the
closed fact collection. Its constructor rejects any producer that is not both
`Fixture` and `FixtureOnly`.

`observe_trusted_host_inputs_from_fixture` derives the request fingerprint,
repository-tree commitment, and ordering-authority commitment from the exact
validated request. It does not accept caller duplicates of those authoritative
bindings. The helper delegates completeness, coherence, canonical ordering,
freshness, and request matching to `TrustedHostInputObservationResult::new`
and `validate_for_request` rather than creating a parallel enforcement path.

## 4. Fixture-Only Producer Assessment

The fixture boundary fails closed for a live `HostObserver` producer and for
any assurance posture other than `FixtureOnly`. The existing Core coherence
rules also require observed facts from a fixture producer to use the fixture
observation method.

This distinction is visible in the type vocabulary and documentation. The
fixture cannot be mistaken for independent observation evidence and cannot
support runtime admission.

## 5. Request Binding And Freshness Assessment

The result is bound to the exact request fingerprint, repository tree,
ordering authority, expected producer identity/version, and ordered revision
domain. Producer substitution and a start revision that is not later than the
static-discovery completion revision fail closed.

The helper does not weaken the accepted shared ordering-authority fix. It
passes the fixture revisions into the Core result constructor and relies on
exact request validation for freshness and substitution enforcement.

## 6. Determinism And Failure Assessment

Repeated observation of the same request and fixture produces equal results
and identical fingerprints. The accepted result constructor canonicalizes the
closed nine-obligation set. Incomplete facts fail through the Core binding
validator, while stale and substituted inputs fail through exact request
validation.

Errors use stable generic codes and do not echo producer identifiers, fact
values, commitments, references, or raw input labels.

## 7. Privacy And Redaction Assessment

The fixture stores only validated payload-free model values. Its custom
`Debug` implementation exposes producer posture, revisions, redacted root
posture, and fact count; it does not format facts. The root model redacts its
commitment and reference. Result serialization contains commitments and stable
references but does not copy the raw labels used to derive commitments.

No raw command output, environment value, filesystem content, provider
response, credential, token, authorization header, private key, or secret-like
payload is introduced.

## 8. Test Quality Assessment

The public integration tests cover:

- deterministic repeated construction;
- all nine obligations and canonical result order;
- exact producer/request substitution rejection;
- stale revision rejection;
- incomplete fact rejection through the Core constructor;
- rejection of non-fixture producers; and
- Debug and serialized-result non-copying of raw payload markers.

The tests exercise the exported public helper rather than private
implementation details. Existing workspace validation passed, preserving the
accepted observer, security, executor, persistence, adapter, and report
boundaries.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add one fixture-level regression proving that two semantically identical
  fact sets supplied in different input orders produce the same result and
  fingerprint. Existing Core tests already cover canonical ordering, so this
  is defense in depth rather than an acceptance blocker.
- Keep fixture construction separate from any future live-provider adapter so
  fixture assurance can never be promoted by configuration.
- Require the future provider plan to identify which producer and ordering
  authority can establish every mandatory fact, and to leave unsupported or
  unavailable facts explicit rather than weakening the contract.

## 11. Recommended Next Phase

Plan one live trusted-host input observation provider and its
capability/evidence matrix.

That planning phase should select exactly one provider candidate, map each of
the nine obligations and repository-root proof to an authoritative provider
surface, define freshness and continuity evidence, and fail closed where the
provider cannot prove a mandatory fact. It must not implement live host access,
process execution, runtime admission, persistence, writes, or additional
provider families.

## 12. Validation Reviewed

The review inspected commit `ba78ab55` and the recorded implementation
validation:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live tests remained ignored behind
  their existing environment gates.
- focused fixture-proof tests: six passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1792840378292338000-2`.
- Approval ID:
  `approval/run-1792840378292338000-2/review-scope-approved`.
- Approval presentation ID: `presentation/8987ad2ae1f92a15`.
- Approval presentation content hash:
  `8987ad2ae1f92a156b4311a066b43b3274a79fbc0a99245e866759d4342b2e18`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact focused review scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected the implementation,
  tests, accepted Core enforcement paths, report, plan, and roadmap and wrote
  this review. The kernel governed scope and approval but did not inspect code,
  edit files, or run validation.
