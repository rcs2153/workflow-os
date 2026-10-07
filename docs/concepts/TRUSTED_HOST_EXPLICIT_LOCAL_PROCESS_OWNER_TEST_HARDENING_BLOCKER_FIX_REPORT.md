# Trusted-Host Explicit Local Process Owner Test Hardening Blocker Fix Report

## 1. Executive Summary

The nondeterministic competing-owner test blocker is fixed. The previously
unclassified `trusted_host_redispatch.directive_missing` result is a lawful,
bounded losing-owner observation when the winner consumes the only available
directive before the loser loads its redispatch snapshot. The test now accepts
that exact stable code and runs the complete contention scenario 32 times in
one ordinary test invocation.

No production behavior changed.

## 2. Blocker Fixed

The hardening review reproduced an intermittent failure after 15 successful
focused runs. On run 16, the losing owner returned
`trusted_host_redispatch.directive_missing`, which was absent from the claimed
complete loser set.

The result is lawful because two exact owners may both observe `ResumeNow`,
after which:

1. the winner consumes the sole available directive and enters the executor;
2. the loser loads the same active-yield generation after consumption;
3. no directive for that generation remains in `Available` state; and
4. the loser fails closed before receiving an attempt capability or entering
   the executor.

This differs from `directive_replayed`, where the loser reaches atomic consume
with an already committed exact operation, and `active_yield_missing`, where
terminal reconciliation is already visible.

## 3. Implementation Approach

- Added only `trusted_host_redispatch.directive_missing` to the closed reviewed
  loser error set.
- Kept the exact aggregate assertions:
  - one executor admission;
  - one terminal winner with one executor entry; and
  - one bounded loser.
- Kept direct Debug non-leakage checks for every outcome.
- Extracted one contention assertion helper and repeated it 32 times inside
  the ordinary test so workspace CI exercises multiple interleavings.

The fix does not treat arbitrary redispatch failures as acceptable and does
not weaken one-winner or zero-leakage requirements.

## 4. Validation Boundary

The accepted loser set remains closed:

- blocked zero-entry result;
- `trusted_host_redispatch.directive_replayed`;
- `trusted_host_redispatch.directive_missing`; or
- `trusted_host_redispatch.active_yield_missing`.

Every accepted error is produced before the losing owner receives usable
attempt authority. The shared executor counter and terminal-winner assertion
prove that only the winner entered execution.

## 5. Privacy And Redaction

No production error or output changed. The test continues to reject Debug
output containing the run identifier or skill-output marker. The newly
accepted error already uses a stable code and bounded non-leaking message.

## 6. Scope Explicitly Not Completed

This fix does not add application adoption, host topology, discovery, signal
handling, public APIs, provider behavior, hosted scheduling, schemas, examples,
writes, or release changes.

## 7. Test Coverage

- Complete competing-owner contention now runs 32 times per test invocation.
- Every iteration requires one executor entry, one terminal winner, one
  reviewed bounded loser, and redaction-safe Debug output.
- The independent simultaneous cancellation-versus-entry race remains intact.
- Repeated focused stress and the full workspace suite are required before
  closure.

## 8. Remaining Limitations

- Thread scheduling remains nondeterministic; this is stress coverage, not
  model checking.
- The private owner remains unadopted by an application.
- Active executor attempts remain non-interruptible.
- Local host application topology and internal visibility remain unplanned.

## 9. Validation

- 20 focused invocations of the 32-iteration competing-owner test passed,
  exercising 640 complete contention scenarios;
- the simultaneous cancellation-versus-entry race passed;
- `cargo fmt --all --check` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `npm run check:docs` passed; and
- `git diff --check` passed.

Rust validation used `CARGO_INCREMENTAL=0` and the clean target directory
`/private/tmp/workflow-os-owner-fix-target`.

## 10. Recommended Next Phase

Perform a focused maintainer/security re-review of this blocker fix. If
accepted, proceed to planning the explicit local host application topology and
Core-to-application visibility boundary.

## 11. Governed Fix Record

- workflow: `dg/blocker`
- run: `run-1791375483861180000-2`
- approval: `approval/run-1791375483861180000-2/fix-approved`
- presentation: `presentation/caa488ef1a36474a`
- presentation hash:
  `caa488ef1a36474a066f7c965cf407fd63cf941f972ef77c029b6304e5d8035e`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused loser-classification blocker fix only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source inspection, test edit, documentation, validation,
  and later git or pull-request work
