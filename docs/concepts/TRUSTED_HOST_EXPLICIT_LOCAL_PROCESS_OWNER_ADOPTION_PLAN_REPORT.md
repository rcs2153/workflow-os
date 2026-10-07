# Trusted-Host Explicit Local Process Owner Adoption Plan Report

## 1. Executive Summary

The application-adoption planning phase is complete with a topology blocker.
No current Workflow OS application owns the complete SQLite trusted-host
operational binding required by the accepted private process owner. The plan
therefore rejects a convenient but unsafe integration and preserves the owner
as private and uncalled.

## 2. Scope Completed

- Inspected the private owner and accepted lower-level composition.
- Inspected the CLI, hosted worker, SQLite backend visibility, and current
  runtime boundaries.
- Defined the exact adoption contract for state, authority, execution,
  cancellation, shutdown, and outcomes.
- Assessed and rejected the current CLI, hosted worker, and Core library as
  immediate application adoption sites.
- Carried the implementation-review test-hardening requirements into the
  adoption gate.
- Defined the prerequisite local host application topology decision.

## 3. Scope Explicitly Not Completed

No runtime code, application caller, public API, CLI command, SDK, schema,
configuration, discovery, polling, automatic invocation, automatic approval,
daemon, signal handler, hosted scheduler, provider behavior, OpenShell
integration, nested harness behavior, write family, or release change was
added or authorized.

## 4. Assessment Result

The current CLI uses the filesystem-backed local executor path and does not
hold the private SQLite trusted-host binding. The hosted worker is PostgreSQL
and has different fencing and lease semantics. Core owns the private types but
does not own a process lifecycle.

Operational adoption is blocked until Workflow OS deliberately defines an
explicit local host application boundary and an internal Core-to-application
visibility contract.

## 5. Safety Rationale

Rebuilding a locator from filesystem state, invoking scheduling from approval
or preview commands, placing SQLite scheduling in the hosted worker, or adding
a hidden CLI command would each cross an unreviewed boundary. The plan treats
the absence of a safe caller as a real finding rather than filling the gap
with another wrapper.

## 6. Test Requirements Carried Forward

Before adoption:

- add a real simultaneous cancellation-versus-entry race test; and
- classify both competing-owner results, including bounded loser and
  non-leakage assertions.

Application implementation must later prove cancellation custody, cooperative
shutdown, admitted-work waiting, bounded outcomes, exact invocation, no
discovery, and no default activation.

## 7. Validation

The phase validation passed:

- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 8. Remaining Limitations

- No application invokes the owner.
- No explicit local host application exists.
- No process signal or active-attempt interruption contract exists.
- No hosted/PostgreSQL trusted-host parity exists.
- No operator-facing stop projection exists.

## 9. Recommended Next Phase

Implement and review the two private-owner test-hardening follow-ups, then plan
the explicit local host application topology. Do not broaden provider writes,
public scheduling, discovery, or automatic continuation first.

## 10. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791373890846364000-2`
- approval: `approval/run-1791373890846364000-2/planning-approved`
- presentation: `presentation/8485f3983707db47`
- presentation hash:
  `8485f3983707db47319c3a7655fa0994ae6fc0f9633cfa80426bc88621c22a08`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: documentation and read-only application-boundary
  inspection only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source and architecture inspection, documentation edits,
  validation, and later git or pull-request actions
