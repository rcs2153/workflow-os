# Trusted-Host Local Cargo Input Observer Contract Report

## 1. Executive Summary

The private, process-free local Cargo input observer contract slice is
implemented. The unpublished `workflow-local-host` crate can now compose one
exact-request-bound `TrustedHostInputObservationResult` and an exact in-memory
evidence bundle from injected sources without reading the host or starting a
process.

The implementation is non-authorizing. Every standalone fact remains below
`Observed`, and the repository root cannot claim proven execution continuity.

## 2. Scope Completed

- Added a private observation definition bound to the request's exact
  selection, operation contract, and security requirements.
- Added a private local Cargo `HostObserver` boundary restricted to an
  `IndependentlyReviewed` host-observer producer matching the request.
- Added injected filesystem, closed process-fact, and monotonic revision
  source interfaces.
- Added exact request, producer, repository-tree, ordering, revision, fact-kind,
  and posture-ceiling validation.
- Added deterministic, non-circular in-memory evidence references and a bundle
  that rejects missing, duplicate, extra, substituted, and cross-request
  entries.
- Added the narrow Core read-only accessors required for canonical comparison
  and exact evidence resolution.

## 3. Scope Explicitly Not Completed

This phase added no concrete filesystem source, standard process source, host
inspection, environment access, live smoke, runtime composition, admission,
Cargo execution, OpenShell integration, persistence, events, audit records,
schemas, CLI behavior, provider writes, capability grants, or release changes.

## 4. Private Contract Summary

`LocalCargoObservationDefinition` rejects any selection, operation-contract,
or security-requirements commitment that differs from the validated Core
request.

`LocalCargoHostObserver` accepts only injected sources. It requires the exact
producer identity and version requested by Core, obtains start and completion
revisions from the injected ordering source, requests exactly the closed nine
obligation kinds, and constructs the result through existing Core validators.

The module remains private and is explicitly dead-code-tolerant until a later
review accepts a concrete source. No public local-host API was added.

## 5. Posture And Ordering Summary

The standalone observer rejects `Observed` facts and any proven root or fact
continuity. Dynamic-library and runtime-base facts must remain `Unsupported`;
environment remains `DeclaredOnly` or unavailable; the remaining facts remain
within their accepted uncertain, declared-only, or unavailable ceilings.

Observation start must follow static discovery, and completion must strictly
follow start. Wall-clock time is not used as ordering authority.

## 6. Evidence Bundle Summary

Each entry records a closed evidence class, request fingerprint, producer
identity and version, completion revision, payload-free commitment, and stable
telemetry reference. References are derived from those fields without using
the result fingerprint, avoiding circular construction. The bundle envelope
then binds the final result fingerprint.

Validation requires exactly one repository-root entry and exactly one entry
for every present fact reference. It rejects missing, duplicate, extra,
substituted, cross-request, cross-producer, and revision-substituted entries.

## 7. Privacy And Failure Summary

The private model stores commitments and closed metadata only. It has no field
for paths, source contents, environment names or values, Cargo metadata,
stdout, stderr, registry locations, endpoints, credentials, or provider error
bodies. Debug output redacts commitments and references.

Failures use stable generic validation codes and do not echo supplied values.
Request, producer, root, ordering, fact-kind, posture, and evidence-integrity
ambiguity fails the whole observation rather than becoming workflow status or
approval denial.

## 8. Test Coverage Summary

Focused tests prove:

- complete construction of all nine facts plus the repository-root entry;
- exact selection, operation, and security binding;
- deterministic result fingerprints and evidence references;
- standalone posture ceilings;
- source fact-kind substitution rejection;
- stale revision, root, and producer substitution rejection;
- missing, duplicate, extra, cross-request, cross-producer, and completion
  revision evidence rejection; and
- non-leaking Debug behavior.

All ordinary tests remain process-free through injected fakes.

## 9. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; existing opt-in live tests remained ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Focused `workflow-local-host` library tests: 13 passed.

## 10. Remaining Known Limitations

- No live host fact has been observed.
- The injected process interface returns bounded fact candidates; the reviewed
  fixed probe implementation does not exist yet.
- `IndependentlyReviewed` describes the producer implementation review, not
  independent machine attestation.
- No fact can satisfy live-execution admission without accepted same-call
  custody or immutable provider projection.
- Evidence exists only in memory and is not persisted or consumed by runtime.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of this private contract slice.
Do not implement the concrete source or opt-in live smoke until the review
accepts source closure, posture ceilings, deterministic evidence resolution,
privacy, and the absence of runtime authority.

## 12. Governed Phase Evidence

- Workflow ID: `dg/implement`.
- Run ID: `run-1792842383545828000-2`.
- Approval ID:
  `approval/run-1792842383545828000-2/implementation-approved`.
- Approval presentation ID: `presentation/8d23d2969fd8dd24`.
- Approval presentation content hash:
  `8d23d2969fd8dd244afa43ba90f39a82b898181cbd292194c1a3e4ac4cc0671b`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact private contract implementation scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Out-of-kernel work: Codex implemented the private contracts and tests,
  authored this report, and ran validation. The kernel governed scope and
  approval but did not inspect files, edit code or documentation, or execute
  validation commands.
