# Trusted-Host Input Observer Core Model Report

## 1. Executive Summary

Workflow OS now has a provider-neutral, non-authorizing Core model for exact,
payload-free trusted-host input observations. The model binds one request to an
immutable operation context, validates one fact for each of the nine accepted
host-input obligations, composes the existing repository-root observation, and
creates deterministic canonical commitments.

This phase does not inspect a host or implement an observer.

## 2. Scope Completed

- Added a closed Cargo host-input observation contract version.
- Added exact request identity for the run bundle, operation contract, static
  request/result, repository tree, security requirements, selection, and
  attempt.
- Added bounded producer kind and assurance claims with implementation and
  configuration commitments.
- Added observed, declared-only, unavailable, unsupported, and uncertain fact
  posture.
- Added exactly one fact for every closed host-input obligation.
- Reused `TrustedHostRootObservation` for the repository input and required a
  read-only, non-invocation-scoped root bound to the exact repository-tree
  commitment.
- Added deterministic request/result commitments and pinned canonical vectors.
- Added strict deserialization, privacy-safe Debug output, stable validation
  codes, and focused regression tests.

## 3. Scope Explicitly Not Completed

- No host, filesystem, environment, executable, package source, or dynamic
  library inspection.
- No callable observer or fixture-backed observer helper.
- No process, Cargo, compiler, linker, shell, or sandbox execution.
- No provider selection, OpenShell integration, or provider client.
- No runtime assessment, admission, dispatch, approval, or authority grant.
- No persistence, workflow events, audit records, reports, artifacts, schemas,
  SDKs, CLI behavior, examples, or onboarding behavior.
- No provider mutations, external writes, or release posture changes.

## 4. Core Model Summary

`TrustedHostInputObservationRequest` commits to the exact immutable inputs an
observer would be asked to assess. `TrustedHostInputObservationResult` binds
the request commitment, repository tree, producer claim, ordered revisions,
repository-root observation, and complete fact set.

`TrustedHostInputObservationFact` stores only closed vocabulary, bounded
counts, commitments, and optional stable telemetry references. It stores no
paths, environment variable names or values, command output, provider
payloads, executable bytes, source content, credentials, or tokens.

## 5. Binding And Coherence

Construction fails closed unless:

- revisions are nonzero and monotonic;
- the repository root is the read-only `RepositoryInput` role;
- the repository root is not invocation scoped;
- the root commitment exactly equals the request repository-tree commitment;
- exactly nine unique host-obligation facts are present;
- `Observed` facts use an observation method with proven object binding and
  continuity;
- non-observed facts do not claim proven binding and continuity;
- declared, unavailable, and unsupported postures use their matching methods;
  and
- all optional fact references are telemetry references.

Result-to-request validation rejects request-fingerprint or repository-tree
substitution.

## 6. Canonical Commitments

Canonical hashing uses explicit versioned domains, fixed-width length framing,
closed stable tags, sorted fact order, and every semantic field. The result
commitment includes producer identity and assurance, producer implementation
and configuration commitments, revisions, all repository-root semantics and
reference, and all fact semantics and references. It does not depend on Rust
Debug formatting or collection insertion order.

Pinned request and result vectors detect accidental wire or hashing changes.

## 7. Privacy And Redaction

Manual Debug implementations redact commitments and stable references.
Validation errors use stable codes and fixed messages without caller values.
Closed enum deserializers reject unsupported values without echoing them.
Unknown fields fail closed, including nested repository-root and hosted
reference fields.

Serialization remains payload-free but is not itself evidence that a claimed
observation is true. Future admission must separately trust and validate the
producer channel.

## 8. Test Coverage

Focused tests cover deterministic construction, pinned canonical vectors, fact
order independence, complete obligation representation, missing and duplicate
facts, repository substitution, declared-only false proof, explicit incomplete
postures, invalid producer assurance, invalid reference kind, wrong root role,
semantic commitment sensitivity, serde revalidation, strict unknown-field and
unknown-enum rejection, and Debug/error non-leakage.

Existing workspace tests retain coverage for static discovery, live-security
contracts, hosted execution, runtime behavior, persistence, adapters, reports,
and governance models.

## 9. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live tests remained ignored by their
  existing environment gates.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Known Limitations

- No implementation currently produces these observations.
- Producer assurance remains a validated claim, not trusted admission evidence.
- The contract is provider-neutral but its first version intentionally covers
  only the accepted Cargo/Rust host-input obligations.
- No composition yet combines static discovery and host observation into a
  complete build-time input assessment.
- No runtime path consumes the model.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of the Core model. Review exact
request/result binding, repository-root reuse, complete obligation semantics,
canonical hashing, strict serde, privacy, and non-authorizing scope before any
fixture-backed proof or live provider work.

## 12. Governed Phase Evidence

- Workflow ID: `dg/implement`.
- Run ID: `run-1792835136119672000-2`.
- Approval ID:
  `approval/run-1792835136119672000-2/implementation-approved`.
- Approval presentation ID: `presentation/f746433021033c86`.
- Approval presentation content hash:
  `f746433021033c869e2bd8629b440bbabf753433e4fe7652fb75a9a5caf2f40c`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact model-only implementation scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker was present.
- Out-of-kernel work: Codex edited model, tests, and documentation and ran the
  validation commands. The kernel governed scope and approval but did not edit
  files, run checks, commit, push, or open a pull request.
