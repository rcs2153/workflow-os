# Explicit Local Trusted-Host Production Caller Plan Blocker Fix Review

## 1. Executive Verdict

**Planning blockers fixed; proceed to private caller implementation.**

The corrected plan is phase-ready. It defines reachable synchronous
cancellation, an exact production entropy boundary, all-or-nothing identity
construction, a fixed two-wake budget, a bounded existing outcome, and correct
Core-owned authority language without broadening into a scheduler.

## 2. Scope Verification

The fix remains planning-only. No Rust code, dependency, daemon, thread, run
discovery, public surface, automatic approval, provider mutation, OpenShell,
nested harness, hosted scheduling, or release posture was added.

## 3. Cancellation Ownership Reassessment

The owner-created private pair is implementable and preserves synchronous
ownership. The owner retains the existing handle while the caller consumes the
receiver. The caller creates no thread, callback framework, arbitrary
cancellation trait, or durable cancellation record.

Cancellation remains non-authorizing and cannot mutate, fail, approve, or
complete the workflow.

## 4. Identity Source Reassessment

A direct `getrandom` 0.4 dependency is an appropriately narrow production
entropy boundary. One 96-byte fill per redispatch set and one 32-byte fill per
wake set make partial construction impossible at the entropy boundary.
Independent 128-bit partitions, operation-family prefixes, lowercase fixed
hexadecimal encoding, typed validation, and no automatic retry provide a clear
and testable contract.

The dependency must be direct in `workflow-core`, locked by the workspace, and
used only for non-authorizing identity material. Test-only deterministic byte
filling must not be reachable from production exports.

## 5. Wake-Budget Reassessment

The fixed two-wake budget is conservative and justified. It is the minimum
that proves repeated waiting, cannot be caller-configured, and keeps ownership
bounded. Exhaustion remains a non-terminal stop and cannot be interpreted as
workflow failure or completion.

## 6. Outcome And Authority Reassessment

Returning `TrustedHostRepeatedSchedulingOutcome` unchanged avoids another
model and preserves the accepted disposition, counts, and closed stop reason.
Structured failures remain errors. Removing reconstruction advice prevents a
host hint from becoming implied resume authority.

The corrected “already-selected” language accurately reflects the source of
truth. Only the accepted same-call Core boundaries may establish current
authority.

## 7. Restart, Failure, And Privacy Assessment

Restart reconstructs only process-local mechanics and reenters authoritative
Core state. No durable receipt recreates authority. Replay conflict, entropy
failure, stale binding, corruption, ambiguity, trusted-time rejection, and
executor mismatch remain fail-closed with no automatic retry.

The planned inputs, outcomes, Debug, and errors remain payload-free and exclude
paths, prompts, commands, source content, credentials, approval reasons,
evidence bodies, raw skill output, provider data, and reusable authority.

## 8. Test Plan Assessment

The corrected matrix covers:

- cancellation reachability during a blocked synchronous call;
- independent identity material and deterministic test injection;
- all-or-nothing construction on entropy and validation failure;
- duplicate conflict without retry;
- fixed two-wake posture;
- restart and competing callers;
- absence of resume advice, background threads, and public exports; and
- workspace regressions.

This is sufficient for the bounded implementation. Dependency and security
checks remain mandatory.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- A private internal adoption site requires a later separate plan.
- Operator metrics and notification remain deferred.
- Detached owner-loss semantics remain deferred.
- Other wait-source families remain deferred.

## 11. Recommended Next Phase

Implement one crate-private synchronous local production caller and private
production identity source exactly as corrected. Add the direct `getrandom`
0.4 dependency and focused tests, then run full workspace validation.

Do not add run discovery, a daemon, detached scheduling, public configuration,
CLI, SDK, schemas, automatic approval, provider mutation, OpenShell, nested
harnesses, hosted scheduling, or release changes.

## 12. Governed Re-Review Record

- workflow: `dg/review`
- run: `run-1791363184108870000-2`
- approval: `approval/run-1791363184108870000-2/review-scope-approved`
- presentation: `presentation/00f3eda88b779f16`
- presentation hash:
  `00f3eda88b779f16fa73927913725d5f7278b8003036e2b78a55dd988401be1c`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: focused blocker-fix re-review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations; approval-presentation proof was enforced with one matching record and event marker
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: corrected-plan inspection, re-review authoring,
  documentation validation, and later git and pull-request actions
