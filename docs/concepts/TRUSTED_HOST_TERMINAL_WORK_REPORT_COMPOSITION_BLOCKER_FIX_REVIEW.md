# Trusted-Host Terminal WorkReport Composition Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; phase accepted.**

The narrow fix closes the durable-history integrity gap identified by the
original phase review. Trusted-host terminal report composition now accepts
only status-compatible check, invocation, and run outcomes in strict event
order. The in-memory composition phase may close.

## 2. Scope Verification

The fix stayed within the approved Core validation boundary. It added no
report artifact, persistence, released CLI behavior, runtime profile,
arbitrary command, scheduler, provider, write, hosted runtime, nested harness,
workflow schema, example, or release-posture change.

## 3. Original Blocker Restatement

The composer previously verified durable identities but allowed a concrete
local-check result to contradict the referenced invocation terminal event or
the run terminal event. It also did not require the invocation terminal event
to occur strictly after the result event and before the run terminal event.
That could allow a report to cite mutually inconsistent durable workflow
truth.

## 4. Fix Assessment

The private validation function now accepts exactly these combinations:

- `Passed` with invocation success and `RunCompleted`;
- `Failed` with invocation failure and `RunFailed`; and
- `TimedOut` with invocation failure and `RunFailed`.

All other combinations, including any concrete result paired with
`RunCanceled`, fail closed with
`trusted_host_terminal_report.terminal_status_mismatch`.

The composer also requires:

```text
result event < invocation-terminal event < run-terminal event
```

Non-monotonic or equal sequence positions fail with
`trusted_host_terminal_report.terminal_event_order_mismatch`.

The composer derives the referenced invocation outcome from the uniquely
matched durable event before applying these checks. The normal writer,
transactional lifecycle commit, report identity, report model, and fixed host
output remain unchanged.

## 5. Error And Privacy Assessment

The new validation consumes only closed status vocabulary and sequence
numbers. Both errors use stable codes and fixed messages. They contain no run,
event, result, invocation, path, hash, command, payload, environment, approval,
or credential value.

No raw output, source/spec content, parser data, provider data, or secret-like
value is added to durable state, Debug output, serialization, or reports.

## 6. Test Assessment

Focused table-driven tests cover every accepted status matrix, representative
status conflicts, cancellation, invocation-before-result ordering, equal
sequence positions, and run-before-invocation ordering. The 13-test foreground
host suite proves normal generation and exact replay remain green, and the
complete workspace suite passes.

One non-blocking follow-up remains: the mismatch matrix is tested directly at
the private invariant helper rather than by constructing fully rehydrated
tampered histories through the composition entry point. The production call is
direct and visible, while separate host tests cover the valid end-to-end path,
so this does not leave the original blocker open. A future persistence-focused
phase should add end-to-end corrupt-history fixtures before widening report
storage or recovery surfaces.

## 7. Regression Assessment

Existing workflow semantics remain unchanged:

- passing checks complete;
- failed and timed-out checks fail;
- report failure cannot rewrite the durable run result;
- exact replay does not rerun the process or append events; and
- no report is produced from contradictory terminal history.

Formatting, clippy with warnings denied, focused tests, the complete workspace
suite, documentation checks, and diff checks all pass.

## 8. Documentation Assessment

The plan, original review, blocker-fix report, and roadmap accurately preserve
the private in-memory no-write boundary. The original `Needs blocker fixes`
verdict remains historical evidence; this review supplies the independent
fix-forward acceptance.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add full composition-entry corrupt-history fixtures before report
  persistence or recovery broadens the trust boundary.
- Keep the two-profile, fixed-contract limitation explicit until a separately
  planned surface is accepted.

## 11. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792665573071876000-2`
- Approval ID:
  `approval/run-1792665573071876000-2/review-scope-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/d5d7c3f926cc910f`
- Review status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Validation summary: formatting, clippy with warnings denied, focused tests,
  the complete workspace suite, documentation checks, and diff checks passed.

The agent inspected code and tests, ran validation, and wrote this review
outside the kernel under the approved scope. The kernel coordinated the
proof-backed review gate and durable phase record; it did not inspect code,
execute checks, write files, commit changes, or perform provider activity.

## 12. Recommended Next Phase

Plan the smallest private trusted-host terminal WorkReport artifact boundary.
The plan must preserve terminal workflow truth, exact report identity,
payload-free references, replay safety, and the corrupt-history follow-up
before authorizing any artifact write. It must not add released CLI rendering,
another runtime profile, arbitrary commands, scheduling, providers, external
writes, hosted expansion, or nested harness execution.
