# Independent Local Check Attestation Verifier Report

## 1. Executive Summary

Workflow OS now has a pure crate-private verifier that can convert exact,
Core-owned local-check observation context into a read-only accepted attestation.
It verifies immutable run context, pre-execution command/handler/policy binding,
the complete requirement, structured result facts, policy limits, time ordering,
and freshness.

The verifier does not execute a process and is not wired into the executor. No
persistence, event, evidence, report, artifact, schema, CLI, provider,
SideEffect, or write behavior was added.

## 2. Scope Completed

- Added crate-private `KernelObservedLocalCheck` authority and constructor.
- Added crate-private borrowed verification input and pure verifier.
- Added public read-only `AcceptedLocalCheckAttestation` with no public
  constructor and no deserialization path.
- Matched requirement, candidate, immutable bundle, pre-execution binding,
  command contract, handler selection, effective policy, observation, and
  structured result.
- Enforced accepted status, truncation, timeout, time ordering, and freshness.
- Added stable bounded error codes and canonical accepted-proof identity.
- Added focused positive, substitution, policy, freshness, privacy, and
  non-leakage tests.

## 3. Scope Explicitly Not Completed

- process execution or handler invocation;
- executor or runtime call-site integration;
- automatic check execution or default registration;
- persistence, cache reuse, events, audit, evidence, reports, or artifacts;
- proportional-governance, approval, capability, or authority consumption;
- schemas, SDKs, CLI, UI, examples, providers, SideEffects, or writes;
- remote, cryptographic, hardware-backed, or third-party attestation;
- hosted/distributed runners, enterprise identity, or release changes.

## 4. Authority Boundary

The kernel observation type, its constructor, verification input, and verifier
are crate-private. Public callers cannot construct the observation authority or
invoke verification. `AcceptedLocalCheckAttestation` exposes read-only proof
context but has no public constructor and does not implement `Deserialize`.

This prevents a public caller from upgrading a self-consistent serialized
binding into accepted proof. Publicly recomputable fingerprints remain
tamper-evident commitments, not authenticity.

## 5. Verification Boundary

Verification rejects mismatched requirement, candidate fingerprint, immutable
bundle, workflow/run/step identity, command contract, handler selection,
effective policy, invocation, idempotency reference, result identity, status,
exit posture, duration, truncation, observation time, assurance, source, or
freshness.

`NoReuse` is enforced through exact current invocation and idempotency linkage.
Maximum-age freshness uses full duration precision rather than truncating to
whole seconds. The focused suite caught and fixed a one-millisecond boundary
error during implementation.

## 6. Handler Assurance

Accepted v0 assurance remains `KernelObservedLocalProcess`. The verifier accepts
only the exact pre-bound `RegisteredUnattested` handler selection. It proves
Core observation under the selected registration and policy; it does not claim
handler implementation, binary, host, or third-party provenance.

## 7. Privacy And Redaction

Observation and accepted records contain bounded identities, fingerprints,
typed status, timing, and truncation facts only. They contain no raw stdout,
stderr, paths, arguments, environment values, source contents, credentials,
tokens, provider payloads, or free-form claims.

Debug redacts identities, fingerprints, and timestamps. Verification errors use
stable codes and do not echo supplied values. Accepted records are not
serializable in this phase.

## 8. Test Coverage

Focused unit tests cover:

- exact accepted context and a stable proof vector;
- command and observation substitution;
- bundle mismatch and assurance downgrade;
- future/invalid evaluation time;
- result duration mismatch;
- unaccepted status and forbidden truncation;
- exact maximum-age boundary and one-millisecond expiry; and
- observation and accepted-record Debug non-leakage.

Existing execution-binding tests continue to cover canonical command/policy
commitments, handler selection, serde tampering, and stable binding vectors.

## 9. Governed Phase

- workflow: `dg/implement`
- run: `run-1784516576846447000-2`
- approval: `approval/run-1784516576846447000-2/implementation-approved`
- presentation: `presentation/e1c5608a5a1c4e70`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits and validation ran
  outside the kernel

## 10. Validation

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 11. Remaining Limitations

- The verifier has no runtime call site.
- The crate-private observation is not yet produced by the Core runner.
- No accepted or rejected record is persisted or projected into events.
- Freshness is evaluated only at verification time; future consumers must
  reevaluate at time of use.
- Handler implementation provenance remains unattested.
- The dogfood approval-presentation list-cap defect remains open.

## 12. Recommended Next Phase

Perform a phase-level verifier review. If accepted, plan one explicit opt-in
`DocsCheck` runtime composition path that creates the immutable binding before
execution, produces the crate-private observation from Core-owned execution,
and invokes the verifier afterward. Do not enable checks automatically or
broaden provider writes.
