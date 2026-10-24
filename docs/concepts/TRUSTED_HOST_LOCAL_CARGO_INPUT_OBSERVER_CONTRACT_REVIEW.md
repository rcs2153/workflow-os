# Trusted-Host Local Cargo Input Observer Contract Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups. Proceed to the reviewed fixed
local Cargo source and opt-in `aarch64-apple-darwin` smoke implementation.

The private contract slice preserves the intended boundary: it can assemble
complete, exact-request-bound observations from injected sources, but it
cannot inspect the host, start a process, authorize admission, or claim
`Observed` continuity.

## 2. Scope Verification

The phase stayed within the accepted private-contract scope. It added one
private local-host module, injected filesystem/process-fact/revision source
interfaces, exact in-memory evidence resolution, narrow Core read-only
accessors, deterministic tests, and phase documentation.

It did not add a concrete host source, process execution, live smoke, runtime
composition, admission, Cargo execution, OpenShell wiring, persistence,
events, schemas, CLI behavior, provider writes, or release changes.

## 3. Ownership And Dependency Assessment

`workflow-local-host` is the correct owner. The crate already owns the local
process boundary and depends on `workflow-core`; Core does not depend on the
local-host crate. The module is private and dead-code-tolerant until a
concrete source is separately accepted.

The Core additions are limited to read-only accessors for canonical
commitments, fact commitments and references, and completion revision. They
do not create host behavior or authority.

## 4. Request And Producer Binding Assessment

`LocalCargoObservationDefinition` must match the request's exact selection,
operation-contract, and security-requirements commitments. The observer also
requires the exact requested producer identity and version. Core validates
the request fingerprint, repository tree, ordering authority, revision order,
producer, complete nine-fact set, and root posture again when the result is
constructed.

The definition is intentionally only a commitment envelope today. It does not
yet contain the typed selected roots, executable inventory, environment
allowlist, or fixed probe definitions required by a concrete source. The next
phase must add those as closed internal types and must not interpret a hash as
configuration.

## 5. Source Closure Assessment

The observer can call only three injected interfaces: repository-root
observation, one closed fact request for each required obligation kind, and a
monotonic revision source. There is no command text, argument vector,
environment map, path, shell, network, persistence, event, approval, or
workflow mutation interface.

The current `LocalCargoProcessSource` name describes the future ownership
boundary, but its present contract returns bounded fact candidates and starts
no process. A concrete implementation must retain the fixed-probe closure in
the accepted plan.

## 6. Posture And Authority Assessment

The local boundary rejects every `Observed` fact and every proven root or fact
binding/continuity claim. Dynamic libraries and runtime base remain
`Unsupported`; environment remains `DeclaredOnly` or `Unavailable`; linker
may be declared, uncertain, or unavailable; all remaining categories remain
uncertain or unavailable.

The result is evidence, not permission. It cannot satisfy live-execution
admission without later accepted same-call custody or immutable provider
projection. `IndependentlyReviewed` describes this producer implementation
after focused review; it does not attest the machine or future process.

## 7. Revision And Root Assessment

Observation start must be strictly later than static discovery, and completion
must be strictly later than start under the request's ordering authority. The
repository-root source commitment must equal the request's immutable tree
commitment, and Core independently enforces read-only, non-invocation-scoped
root semantics.

Wall-clock time is not used as ordering authority. Stale revisions, root
substitution, and producer substitution fail the entire observation.

## 8. Evidence Bundle Assessment

Evidence references are deterministically derived from request fingerprint,
producer identity/version, completion revision, closed evidence class, and
entry commitment. They do not depend on the result fingerprint, avoiding
circular construction. The bundle separately commits to the final result.

Validation requires one unique root entry and one unique entry for every fact
reference. It rejects missing, duplicate, extra, cross-request,
cross-producer, revision-substituted, reference-substituted, and
commitment-substituted entries. The references are resolvable only inside the
in-memory bundle; they are not durable telemetry claims.

## 9. Privacy And Failure Assessment

The private types store closed enums, counts, commitments, revisions, producer
identity, and bounded telemetry references. They have no storage surface for
paths, source contents, environment names or values, Cargo metadata, stdout,
stderr, registry locations, endpoints, credentials, or provider error bodies.

Debug output redacts commitments and references. Errors use stable generic
codes and do not echo caller values. Any binding, posture, ordering, fact-kind,
or evidence ambiguity fails closed.

## 10. Test Quality Assessment

The focused tests cover complete nine-fact construction, exact definition
binding, deterministic result and reference derivation, posture ceilings,
fact-kind substitution, stale revision, root and producer substitution,
evidence missing/duplicate/extra/cross-request/cross-producer/revision
substitution, and Debug non-leakage.

All ordinary tests remain process-free. The full workspace suite, clippy,
formatting, documentation validation, and diff checks pass. Concrete
filesystem drift, symlink replacement, process timeout/output bounds,
environment clearing, parser failure, and platform behavior correctly remain
tests for the next concrete-source phase.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add typed closed probe inputs before implementing a concrete source; do not
  treat the three commitment hashes as executable configuration.
- Keep in-memory telemetry references explicitly non-durable until a separate
  persistence contract exists.
- Preserve the distinction between reviewed producer code and machine
  attestation in all operator-facing language.
- Consider renaming the process-fact interface only if the concrete fixed
  source makes its non-arbitrary nature unclear; avoid churn before then.

## 13. Recommended Next Phase

Implement the reviewed fixed local Cargo source and one opt-in
`aarch64-apple-darwin` live smoke:

- closed typed roots, executable candidates, sanitized environment, and probe
  definitions bound to the accepted commitments;
- fixed read-only filesystem operations and fixed bounded process probes only;
- no caller-selected commands, arguments, paths, or ambient environment;
- exact pre/post identity, timeout, output-bound, parser, and drift failures;
- process-free ordinary tests plus one ignored opt-in smoke; and
- the same conservative non-`Observed` posture ceilings.

Do not add runtime composition, admission, Cargo clippy execution, OpenShell
wiring, persistence, events, schemas, CLI behavior, provider writes, or
broader authority.

## 14. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; existing opt-in live tests remained
  ignored.
- `cargo test -p workflow-local-host --lib`: 13 passed.
- `npm run check:docs`: passed before review edits.
- `git diff --check`: passed before review edits.

## 15. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1792843422794053000-2`.
- Approval ID:
  `approval/run-1792843422794053000-2/review-scope-approved`.
- Approval presentation ID: `presentation/87d10024e8223476`.
- Approval presentation content hash:
  `87d10024e82234762949b3787cf4dc82cc99e03dda23d725df7a631ca29e7c03`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact focused review scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval-presentation enforcement was
  proof-enforced and its event marker was present.
- Out-of-kernel work: the delegated maintainer inspected implementation,
  tests, Core invariants, and phase documents; wrote this review; and ran
  validation. The kernel governed scope and approval but did not inspect
  files, edit documents, or execute validation commands.
