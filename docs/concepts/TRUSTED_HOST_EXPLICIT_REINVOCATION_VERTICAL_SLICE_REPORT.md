# Trusted-Host Explicit Reinvocation Vertical Slice Report

## 1. Executive Summary

Workflow OS now has one crate-private, explicit local SQLite composition for
an already-registered exact `TimeWindow` wait. The helper accepts a previously
returned inert handoff, validates its payload-free commitment against current
authoritative state, applies the accepted source-specific wait transition,
and enters only through the existing trusted-host supervisor path.

The handoff remains orientation and stale-detection input, not authority. The
slice adds no scheduler, polling, automatic agent reinvocation, provider
execution, public API, CLI, schema, or hosted behavior.

## 2. Scope Completed

- Added `TrustedHostTimeWindowReinvocationInput`, its bounded outcome, and
  `reinvoke_after_time_window_wait` as crate-private types.
- Added an existing-window-only operational entry input that cannot carry
  initial-opening context or opening-persistence placeholders.
- Bound handoff-based `TimeWindow` transitions to a Core-derived payload-free
  handoff commitment.
- Persisted that commitment in a domain-separated private replay envelope.
- Kept direct wake requests on the existing v1 commitment and envelope path.
- Rejected stale or substituted handoff commitments before fresh mutation.
- Returned a stable security error rather than a disposition when the wake
  transition records a security rejection.
- Reused existing directive consumption, dispatch reservation, current
  authority reassessment, required-context checks, and bounded redispatch.

## 3. Durable Replay Boundary

Handoff-bound transitions use a v2 request commitment that includes the
payload-free handoff commitment. Their private request envelope uses a
separate handoff domain and carries the exact bounded transition correlation
fields. Exact replay verifies the durable domain and supplied handoff
commitment before continuing.

Legacy direct wake operations remain distinguishable. Omitting a handoff
commitment cannot promote a direct wake operation into reinvocation, and a
direct-wake replay cannot satisfy a handoff-bound caller.

## 4. Operational Entry Boundary

`enter_existing_trusted_host_operation` accepts only the existing backend,
immutable locator, executor binding, skill input, and identity provider. It
does not accept fresh-opening authorization or opening-persistence identities.

The helper reloads authoritative state, confirms the exact existing window,
recomputes the immutable invocation binding, obtains and consumes a fresh
resume directive when lawful, reserves dispatch through the accepted path,
and invokes the existing bounded redispatch loop.

## 5. Failure And Security Posture

- A stale handoff fails with a stable, non-leaking security error before a
  transition is written.
- A different handoff commitment cannot replay a committed transition even
  when operation, receipt, window, and wait identities otherwise match.
- Trusted-time and exact dependency checks remain inside the accepted
  transition verifier.
- Security rejection is converted to a stable reinvocation error and exposes
  no current disposition.
- Blocked or terminal operational posture does not enter the executor.
- Exact replay after backend reopen follows current authoritative state and
  does not duplicate a completed executor entry.

## 6. Privacy And Redaction

The new input and outcome have bounded custom Debug implementations and no
serde or public export. Errors do not echo handoff, window, condition,
operation, receipt, cursor, deadline, trusted-time, path, command, provider,
payload, credential, token, or test-secret values. The only new durable value
is a domain-separated payload-free commitment in the existing private
operation envelope.

## 7. Scope Explicitly Not Added

This phase does not add scheduling, polling, timers, queues, background work,
model-turn creation, automatic approval, automatic reinvocation, new wake
families, provider or sandbox execution, OpenShell, nested harnesses, public
Rust APIs, CLI, SDK, schemas, workflow configuration, hosted or distributed
runtime, provider writes, or release changes.

## 8. Test Coverage

Focused tests prove:

- stale handoff commitment rejection occurs before mutation;
- handoff-bound transition survives backend reopen as exact replay;
- substituted handoff commitment cannot replay and does not leak its marker;
- an elapsed exact `TimeWindow` transitions and enters one existing operation;
- replay after reopen reaches terminal posture with zero additional executor
  entries; and
- legacy direct wake tests continue using the unchanged v1 path.

## 9. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- Only one already-registered exact `TimeWindow` family is composed.
- Invocation remains explicit and crate-private.
- There is no scheduler, wake transport, host loop, or automatic agent turn.
- Ordinary multi-wait and non-TimeWindow continuation remain deferred.
- The helper is not exposed through CLI, SDK, workflow schema, or hosted
  runtime.

## 11. Recommended Next Phase

Perform focused maintainer/security review of the private reinvocation slice.
Review should concentrate on durable replay binding, stale-handoff races,
one-winner executor entry, security-oracle posture, and the absence of public
or scheduling surface. Do not broaden provider mutation or nested harness work
before that review.

## 12. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791067629080559000-2`
- approval:
  `approval/run-1791067629080559000-2/implementation-approved`
- presentation: `presentation/5007b7f53285ffe6`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: one private local SQLite explicit reinvocation slice
- phase status: `Completed`
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof record and event marker present
- validation summary: focused tests and required repository checks passed
- out-of-kernel work: source inspection, Rust implementation, tests,
  documentation, validation, and later git and pull-request work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, invoke the private helper, create a WorkReport
  artifact, or perform git actions
