# Trusted-Host Explicit Local Process Owner Plan Report

## 1. Executive Summary

Planning is complete for the smallest explicit process-owned local owner of
the accepted trusted-host operation composition. The candidate is a
crate-private, one-shot, synchronous object. It owns the existing cancellation
receiver, returns the cancellation handle to process control before execution,
consumes one exact preselected operational input, and returns the accepted
bounded outcome unchanged.

No runtime code is implemented.

## 2. Scope Completed

- Defined the selected private owner boundary.
- Defined the candidate constructor and consuming `run` API.
- Defined constructed, running, and closed lifecycle posture.
- Defined cancellation-handle retention and process-shutdown limits.
- Defined exact input and authority boundaries.
- Defined unchanged result and stop semantics.
- Defined restart and reconstruction posture.
- Made a direct competing-owner test an implementation acceptance requirement.
- Defined failure, privacy, tests, and implementation sequencing.

## 3. Scope Explicitly Not Completed

This phase does not implement runtime code, discovery, scanning, queueing,
polling, a daemon, detached or background work, automatic continuation,
automatic approval, public APIs, CLI, SDKs, schemas, runtime configuration,
owner persistence, host jobs, provider behavior, OpenShell, nested harnesses,
hosted scheduling, PostgreSQL parity, signal handling, operator notification,
or release changes.

## 4. Selected Owner Summary

The plan proposes one crate-private owner created from an existing
`TrustedHostOperationalEntryInput`. Construction creates the existing local
cancellation pair and returns the handle to process control. A consuming
`run` method delegates once to
`run_explicit_trusted_host_local_operation`.

The owner does not find work, construct authority, interpret policy, infer
eligibility, or translate stop posture into workflow state.

## 5. Lifecycle And Cancellation Summary

The owner is process-local and non-durable. It is constructed without Core
mutation, run synchronously once, then consumed. A retained handle may wake an
active local timer and produce the existing bounded canceled posture.

No signal handler, monitor, service, shutdown event, or automatic restart is
authorized. Process death leaves durable Core state authoritative for a later
explicitly reconstructed owner.

## 6. Authority And Concurrency Summary

The exact operational input remains the authority and binding boundary. The
owner accepts no capability or caller-authored continuation claim and creates
no new admission path.

The owner does not enforce singleton process ownership. A direct two-owner
race is required in implementation tests to prove that existing Core
transactions still admit at most one executor entry.

## 7. Privacy Summary

The planned owner has custom bounded Debug, no serialization, no persistence,
and no payload logging. It returns existing structured errors without retries
or added details.

## 8. Test Plan Summary

Future tests cover zero-write construction, one-shot delegation, terminal and
blocked bypass, lawful timed continuation, scoped-thread cancellation,
non-terminal cancellation semantics, dropped-unrun behavior, restart
rehydration, input substitution, competing owners, unsupported waits, budget
exhaustion, and Debug/error non-leakage.

## 9. Planning Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- No application constructs the owner.
- No discovery or automatic invocation exists.
- No durable owner, lease, or host-job record exists.
- No operator notification or stuck-work detection exists.
- No OS shutdown or signal integration exists.
- No durable terminal-output retrieval is defined.
- SQLite local execution is the only planned boundary.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of the process-owner plan.

The review should decide whether the one-shot API, cancellation ownership,
restart posture, direct race requirement, output ownership, and manual
operator boundary are sufficiently precise before implementation.

## 12. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791369233250729000-2`
- approval: `approval/run-1791369233250729000-2/planning-approved`
- presentation: `presentation/f3e5b86010c65466`
- presentation hash:
  `f3e5b86010c65466613b612fdf60d94e980eca793dfcc0964e8cb471b66c4978`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: planning and documentation only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source and documentation inspection, plan authoring,
  shell validation, and later git and pull-request actions
