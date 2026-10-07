# Trusted-Host Explicit Local Process Owner Test Hardening Review

Fix-forward note: the intermittent `directive_missing` loser is now proven as
a lawful post-consumption snapshot outcome and included in the closed loser
set. The full contention scenario now repeats 32 times inside the ordinary
test. See the [blocker fix
report](TRUSTED_HOST_EXPLICIT_LOCAL_PROCESS_OWNER_TEST_HARDENING_BLOCKER_FIX_REPORT.md).
The original blocker finding below remains unchanged as the historical review
record.

## 1. Executive Verdict

**Needs blocker fixes. Do not proceed to local host application topology
planning.**

The simultaneous cancellation-versus-entry test is well scoped and exercises
the production mutex without changing runtime behavior. The competing-owner
hardening is not yet deterministic: repeated independent execution observed a
losing owner return `trusted_host_redispatch.directive_missing`, while the test
accepts only a blocked zero-entry outcome, `directive_replayed`, or
`active_yield_missing`. The test therefore fails intermittently and does not
yet establish the claimed complete loser classification.

## 2. Scope Verification

The implementation stayed within test hardening and documentation. The Rust
changes are test-only blocks inside existing source modules. No production
logic, owner contract, application adoption, topology, discovery, scheduling,
public API, provider behavior, schema, write path, or release posture changed.

## 3. Cancellation Race Assessment

The timer test launches `begin_entry` and `cancel` on separate scoped threads,
releases both through one barrier, and repeats the contention 128 times. It
accepts only the two mutex-linearized outcomes:

- cancellation wins, after which entry remains canceled; or
- entry wins, after which a second entry receives
  `trusted_host_local_timer.entry_already_started`.

This is a useful direct exercise of the production mutex. It does not depend
on sleeps or elapsed-time thresholds, and it preserves the documented
non-interruptible active-attempt boundary. It is a stress test rather than a
formal concurrency proof, which is acceptable while the owner remains private
and unadopted.

## 4. Competing Owner Assessment

The competing-owner test correctly requires:

- exactly one aggregate executor admission;
- exactly one terminal winner with one executor entry;
- exactly one bounded loser with zero executor entries or a reviewed stable
  error; and
- no run identifier or skill-output marker in Debug output.

However, the reviewed error set is incomplete in executable practice. Across
20 repeated focused invocations, iteration 16 produced:

```text
trusted_host_redispatch.directive_missing
```

and the test failed at the unexpected-result branch. This is a blocker because
the phase explicitly claimed complete classification and because a flaky
concurrency test makes the workspace validation nondeterministic.

The blocker fix must first establish whether `directive_missing` is a lawful
transaction-order loser result or indicates a deeper redispatch invariant
gap. It may add that code to the closed accepted set only if the existing
state-transition semantics prove it is bounded, zero-entry, and non-misleading.
Otherwise production behavior must be corrected in a separately governed
scope. The review does not authorize either implementation choice.

## 5. Privacy And Error Assessment

The direct Debug assertions are appropriate and cover both success and error
results. The observed failure used a stable code and bounded message; it did
not expose the run identifier or skill output. No payloads, paths, locators,
credentials, or provider content were added to production output.

## 6. Regression Assessment

No production behavior changed. The cancellation race passed independently.
The competing-owner test passed 15 consecutive repeated invocations before
the newly observed loser result caused a failure on iteration 16. That result
demonstrates that a single passing workspace run is insufficient evidence for
this concurrency claim.

## 7. Validation

Focused validation:

- `simultaneous_cancellation_and_entry_remain_linearized`: passed with one
  executed test;
- `competing_process_owners_admit_at_most_one_executor_entry`: passed in 15
  repeated runs and failed on run 16 with
  `trusted_host_redispatch.directive_missing`.

The repository-wide formatting, lint, test, documentation, and diff checks are
also passed for review closure:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

A passing single workspace test run does not erase the reproduced focused
failure.

## 8. Blockers

1. Determine and document the semantic status of
   `trusted_host_redispatch.directive_missing` under two exact competing
   owners.
2. Make the competing-owner assertion deterministic against the proven closed
   outcome set without weakening the one-winner, one-admission, zero-leakage
   requirements.
3. Repeat the focused concurrency test enough times to demonstrate the
   blocker is resolved before re-review.

## 9. Non-Blocking Follow-Ups

- Consider a deterministic interleaving harness or model-based concurrency
  test if future application adoption makes the owner operationally reachable.
- Keep the current 128-iteration cancellation race as useful regression
  coverage; it is not a substitute for backend transition conformance.

## 10. Recommended Next Phase

Implement a focused blocker fix for competing-owner loser classification,
then perform a focused re-review. Local host application topology planning
remains blocked until the test hardening is deterministic and accepted.

## 11. Governed Review Record

- workflow: `dg/review`
- run: `run-1791374924864895000-2`
- approval: `approval/run-1791374924864895000-2/review-scope-approved`
- presentation: `presentation/40670aa20f1dc69a`
- presentation hash:
  `40670aa20f1dc69a1514afd20638797c6053ddf8ab4bffd39fa0b964ee2dc3a2`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused review only; no implementation fix or adoption
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source and documentation inspection, focused test
  execution, review authoring, validation, and later git or pull-request work
