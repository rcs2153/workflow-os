# Trusted-Host Explicit Reinvocation Prerequisite Proofs Report

## 1. Executive Summary

The two prerequisite proofs required before private trusted-host scheduling
work are implemented. They exercise the complete accepted explicit
`TimeWindow` reinvocation composition rather than isolated transition or
dispatch primitives.

Two concurrent callers now prove one-winner executor admission with an
explainable loser and terminal durable state. A test-only fault seam at the
exact committed-transition/pre-entry boundary proves restart recovery after
SQLite reopen through exact replay, with no reconstructed authority and no
duplicate executor entry.

No scheduler, readiness assessment, ticket, deadline waiter, host loop, public
API, provider execution, or write behavior was added.

## 2. Scope Completed

- Added a full-composition concurrent-caller proof through
  `reinvoke_after_time_window_wait`.
- Added a private test-only hook after a successful or exactly replayed wait
  transition and before operational entry.
- Added restart recovery coverage using a reopened SQLite backend.
- Proved terminal durable posture and one-winner executor admission.
- Updated the roadmap with the implemented proof boundary and next review.

## 3. Scope Explicitly Not Completed

This phase does not add a scheduling model, readiness assessment, scheduling
ticket, deadline waiter, timer, queue, worker, polling loop, automatic model
turn, automatic approval, provider execution, OpenShell integration, nested
harness execution, public API, CLI, SDK, schema, hosted runtime, provider
write, or release change.

## 4. Concurrent-Caller Proof

The test creates one elapsed exact `TimeWindow` wait and races two callers
through the full explicit reinvocation helper with the same durable operation
and receipt identities.

The observed contract is:

- one caller completes the accepted reinvocation path;
- exactly one executor entry occurs;
- the losing caller receives the stable
  `trusted_host_redispatch.directive_replayed` rejection;
- successful outcomes account for exactly one executor entry; and
- authoritative continuation posture is durably `Terminal`.

The loser is therefore explainable rather than silently dropped or admitted a
second time.

## 5. Transition-To-Entry Crash-Recovery Proof

The production helper delegates through an internal composition function. A
`cfg(test)`-only wrapper injects one callback after the wait transition has
committed and before any operational-entry identity is requested.

The proof:

1. injects a failure at that boundary;
2. verifies zero executor entries and zero identity-provider calls;
3. reopens the SQLite backend;
4. retries the same handoff-bound operation and observes `ExactReplay`;
5. enters the executor exactly once using fresh current state; and
6. replays again after terminal completion with zero additional entry.

No capability, directive, or authority is serialized through the fault seam.

## 6. Runtime And Contract Impact

Production behavior and visibility are unchanged. The public and crate-private
production entrypoint retains its existing signature and delegates through the
same sequence. The only new callable seam is compiled for tests and remains
crate-private.

## 7. Security And Privacy

- The hook receives no workflow payload, authority, handoff, identifier, path,
  command, provider data, credential, or secret.
- Concurrent loser reporting uses a stable bounded error code.
- The crash error is test-only and contains no runtime values.
- Recovery reloads authoritative SQLite state and consumes fresh one-use
  authority through the existing operational-entry path.
- Debug and serialization surfaces are unchanged.

## 8. Test Coverage

Focused tests prove:

- two concurrent full-composition callers produce exactly one executor entry;
- the loser is a stable directive-replay rejection;
- the resulting durable continuation posture is terminal;
- a committed transition can be interrupted before operational entry;
- backend reopen plus exact replay performs one executor entry;
- a later replay performs zero additional entries; and
- the existing explicit reinvocation and reopen replay proof still passes.

## 9. Commands Run And Results

- `cargo test -p workflow-core reinvocation`: passed, including all three
  focused reinvocation tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- Invocation remains explicit, local, SQLite-backed, and crate-private.
- Only the exact `TimeWindow` wait family is covered.
- There is no readiness assessment or benign early-wake behavior yet.
- There is no scheduler, automatic retry, re-arming, timer integration, or
  host execution loop.
- Filesystem, PostgreSQL, multi-host, public configuration, and hosted parity
  remain deferred.

## 11. Recommended Next Phase

Perform focused maintainer/security review of these two proofs. The review
should verify that the concurrent loser is sufficiently explainable, the
test-only seam is placed exactly after durable transition and before entry,
restart recovery does not reconstruct authority, and production behavior is
unchanged. Do not implement scheduling types or behavior before acceptance.

## 12. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791349101900493000-2`
- approval:
  `approval/run-1791349101900493000-2/implementation-approved`
- presentation: `presentation/217f35d09524cd3d`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: two prerequisite explicit-reinvocation proofs and the
  smallest test-only transition-to-entry seam
- phase status: `Completed`
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof record and event marker present
- validation summary: focused reinvocation tests, formatting, strict clippy,
  full workspace tests, docs checks, and diff checks passed
- out-of-kernel work: source inspection, Rust test implementation,
  documentation, validation, and later git and pull-request actions
