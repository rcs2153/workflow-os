# Capability Source Continuity Plan

## 1. Executive Summary

The accepted capability-gated GitHub repository-metadata read validates source
generation and watermark continuity when trusted prior state is supplied. This
phase adds the first Core-owned durable continuity boundary so an adopting
caller cannot choose `FirstObservation` after history exists.

The implementation remains private, local, SQLite-backed, read-only, and
limited to the existing repository-metadata operation. It adds no provider
mutation, public API, executor integration, workflow schema, SDK, CLI, hosted
identity, reusable authority, or release change.

## 2. Security Invariant

For one exact registered current-authority source and exact GitHub repository
query, Core is the sole authority for whether the observation is first, a
replay, or a monotonic advancement. A caller supplies the current registered
source, not its continuity posture.

Before provider invocation, Core must atomically:

1. reconstruct any prior generation and watermark from managed durable state;
2. create the first record only when none exists;
3. accept an exact replay without weakening state;
4. accept a higher generation and advance the watermark once;
5. reject generation rollback;
6. reject same-generation watermark substitution;
7. reject a missing current generation after versioned history exists; and
8. reject stored source or query commitment substitution.

## 3. Storage Contract

The existing managed `SqliteStateBackend` receives an additive V8 table. The
table stores only:

- a domain-separated continuity-key commitment;
- source-registration commitment;
- exact-query commitment;
- optional comparable generation; and
- opaque watermark commitment.

It stores no grants, availability records, repository names, GitHub payloads,
credentials, headers, environment values, command output, or raw authority
facts. Schema creation and V7-to-V8 upgrade remain explicit and manifest-bound.

## 4. Atomicity And Recovery

Compare-and-advance runs in one SQLite `IMMEDIATE` transaction. Concurrent
writers serialize at the database boundary. One writer may create or advance;
later writers reconcile against the committed row. A storage or commit
ambiguity fails closed and the provider client is not invoked.

Continuity admission occurs after the existing policy, identity, freshness,
coverage, authority, projection, and request validation pipeline has prepared
the operation, but before the adapter client is called. A provider failure does
not roll back a valid source observation; a retry reconciles it as an exact
replay.

## 5. Integration Boundary

A new crate-private durable method composes:

1. the existing validated operation preparation;
2. SQLite continuity compare-and-advance; and
3. the existing adapter invocation and receipt construction.

The method accepts a `SqliteStateBackend` reference and does not accept
`CapabilityUseSourceContinuity`. The existing caller-assisted method remains
private and is not an executor-facing or public adoption path.

## 6. Error Posture

Errors use stable `current_authority.github_repository_read.source_continuity.*`
codes and bounded messages. Storage, conflict, rollback, substitution, and
recovery errors must not echo source identifiers, query values, repository
names, watermarks, paths, credentials, or provider payloads.

## 7. Tests

Focused tests must cover first observation, restart reconstruction, exact
replay, higher-generation advancement, rollback, same-generation substitution,
missing current generation, source/query substitution, concurrent replay,
concurrent conflict, zero adapter calls on every continuity failure, provider
failure followed by replay, and non-leaking Debug/errors/storage.

Schema tests must cover fresh V8 creation, explicit V7-to-V8 upgrade,
idempotent upgrade, older-schema rejection, and manifest validation.

## 8. Strict Non-Goals

This phase does not add another provider operation, provider mutation, public
or executor capability APIs, automatic gating of direct adapters, workflow
configuration, schema or SDK exposure, CLI behavior, automatic grants,
inferred authority, reusable authority from continuity state, hosted identity,
enterprise RBAC, OpenShell, nested harness execution, or release changes.

## 9. Validation

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 10. Recommended Close

After implementation and validation, perform a focused maintainer/security
review of the durable boundary before any executor-facing adoption or second
provider operation.
