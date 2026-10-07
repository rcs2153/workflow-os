# Trusted-Host Explicit Reinvocation Prerequisite Proofs Blocker Fix Report

## 1. Executive Summary

The focused concurrency-proof blocker is fixed. The full-composition test no
longer assumes that the losing caller must return an error. It accepts the
existing operational-entry contract's lawful terminal zero-entry outcome in
addition to the two bounded fail-closed errors, while still requiring exactly
one underlying executor call and exactly one aggregate executor entry.

Production reinvocation behavior is unchanged. No scheduling model or runtime
surface was added.

## 2. Blocker Fixed

Focused review found that a losing caller can reach
`enter_existing_trusted_host_operation` after the winner has completed. The
existing entry helper then returns a successful terminal outcome with zero
executor entries. The prior test required one error loser and would therefore
fail under a safe scheduler interleaving.

The proof now bounds every currently lawful losing-caller class:

- `trusted_host_redispatch.directive_replayed`;
- `trusted_host_redispatch.attempt_limit_inconsistent`; or
- successful `Terminal` posture with zero executor entries.

## 3. Implementation Approach

The concurrent test now evaluates both caller results as a set:

- the shared executor call count must equal one;
- successful outcomes must contain exactly one aggregate executor entry;
- exactly one successful outcome must report one executor entry;
- every successful outcome must have a transitioned or exact-replay wake and
  terminal stop reason;
- no successful outcome may report more than one entry; and
- every error outcome must use one of the two accepted bounded race codes.

Durable continuation posture must still be `Terminal`.

## 4. Runtime And Contract Impact

None. The change is limited to a focused unit-test assertion and truthful
phase documentation. No production function, type, visibility, error,
serialization shape, state transition, authority boundary, or executor path
changed.

## 5. Security And Privacy

The fix does not weaken fail-closed behavior. A terminal zero-entry result is
not authority and cannot invoke the executor. Error codes remain bounded and
static. No payload, identifier, path, command, provider data, credential, or
authority material was added to Debug, serialization, storage, or output.

## 6. Test Coverage

The corrected test proves the security invariant across bounded result
classes rather than overfitting one thread schedule. Existing crash recovery,
exact replay, durable terminal posture, and zero-duplicate-entry tests remain
unchanged.

## 7. Scope Explicitly Not Completed

This fix does not implement a readiness assessment, scheduling ticket,
deadline waiter, timer, scheduler helper, retry loop, polling, provider
execution, OpenShell integration, nested harness execution, public API, CLI,
SDK, schema, hosted behavior, provider write, or release change.

## 8. Commands Run And Results

- focused concurrent proof repeated five times under the library test target:
  passed;
- `cargo test -p workflow-core reinvocation`: passed, including all three
  focused reinvocation tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace --quiet`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 9. Remaining Limitations

- The proof is local, SQLite-backed, crate-private, and limited to exact
  `TimeWindow` reinvocation.
- No scheduler or benign early-wake readiness surface exists yet.
- Filesystem, PostgreSQL, multi-host, public configuration, and hosted parity
  remain deferred.

## 10. Recommended Next Phase

Perform focused maintainer/security review of this blocker fix. Only after
acceptance may the private scheduling-boundary implementation begin with the
planned non-mutating readiness assessment and inert absolute-UTC ticket.

## 11. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1791351312239543000-2`
- approval: `approval/run-1791351312239543000-2/fix-approved`
- presentation: `presentation/2198bb31bcf32b47`
- presentation hash:
  `2198bb31bcf32b47613f69215c941e5ed4c8d0db8afc82659b4452135870a0ce`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: focused test-and-report blocker fix only
- phase status: `Completed`
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, zero retries, and zero escalations; approval-presentation proof
  enforced with one matching record and event marker
- validation summary: repeated focused concurrency proof, focused
  reinvocation tests, formatting, strict workspace clippy, full workspace
  tests, documentation checks, and diff checks passed
- out-of-kernel work: source edits, test execution, documentation, validation,
  and later git and pull-request actions
