# Trusted-Host Terminal WorkReport Composition Blocker Fix Report

## 1. Executive Summary

The trusted-host terminal WorkReport composition integrity blocker is fixed.
Core now requires concrete check status, invocation terminal outcome, run
terminal outcome, and event ordering to agree before a report can be composed.
Malformed or tampered terminal history fails closed with stable non-leaking
errors.

## 2. Blocker Fixed

The original composer accepted any matching invocation success/failure event
and any completed/failed/canceled run terminal event for all concrete result
states. It also did not require the invocation-terminal event to occur after
the result event and before the run-terminal event.

The fix rejects contradictory or non-monotonic durable history before report
construction.

## 3. Implementation Approach

One private invariant function now validates the exact accepted matrices:

- `Passed` + invocation success + `RunCompleted`;
- `Failed` + invocation failure + `RunFailed`; and
- `TimedOut` + invocation failure + `RunFailed`.

All other combinations fail with
`trusted_host_terminal_report.terminal_status_mismatch`. A concrete result
paired with cancellation therefore fails closed.

The function also requires:

```text
result event < invocation-terminal event < run-terminal event
```

Non-monotonic or equal sequence positions fail with
`trusted_host_terminal_report.terminal_event_order_mismatch`.

The normal writer, atomic persistence, report model, report identity, and host
output remain unchanged.

## 4. Privacy And Error Boundary

The new checks consume only closed status vocabulary and event sequence
numbers. Errors contain stable codes and a fixed message. They do not include
identifiers, paths, hashes, command output, payloads, environment values, or
secret-like input.

## 5. Test Coverage

Focused table-driven tests cover:

- every accepted success/failure/timed-out combination;
- passed result with failed invocation or run;
- failed result with successful invocation or completed run;
- timed-out result with successful invocation or cancellation;
- invocation before the result event;
- equal result/invocation sequence numbers;
- run terminal before invocation terminal; and
- equal invocation/run terminal sequence numbers.

The full 13-test foreground host suite remains green, including valid report
generation and replay without event growth.

## 6. Commands Run And Results

- `cargo test -p workflow-core --features trusted-host-application-spi trusted_host_terminal_report_integrity_tests`: passed, 3 tests.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 13
  tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 7. Governed Phase Record

- Dogfood workflow ID: `dg/blocker`
- Run ID: `run-1792665169306279000-2`
- Approval ID: `approval/run-1792665169306279000-2/fix-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/c489a7bf01fc8da6`
- Terminal status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.

The agent performed code edits, tests, documentation, and validation outside
the kernel under the approved blocker-fix scope. The kernel coordinated the
approval and phase record; it did not edit files, execute checks, create
commits, or perform provider activity.

## 8. Remaining Limitations

All implementation-phase limitations remain: reports are in memory only, the
host exposes fixed posture only, only two closed no-write profiles participate,
and no provider, artifact, persistence, scheduling, schema, or released CLI
surface is added.

## 9. Recommended Next Phase

Perform focused maintainer/security review of this blocker fix. If accepted,
the trusted-host terminal WorkReport composition phase can close and the active
delivery queue can move to its next no-write runtime-composition item while the
separate sandbox credential dependency remains tracked.
