# Trusted-Host Explicit Local Process Owner Test Hardening Report

## 1. Executive Summary

The two non-blocking test follow-ups from the accepted private process-owner
review are complete. The timer suite now repeatedly races cancellation against
entry from separate threads behind a shared barrier. The competing-owner suite
now classifies the complete observed loser set, requires exactly one terminal
winner with one executor entry, and verifies bounded Debug and error output do
not expose run identity or skill output.

No production behavior changed. The owner remains crate-private, unadopted,
one-shot, synchronous, SQLite-specific, and non-automatic.

## 2. Scope Completed

- Added a 128-iteration barrier-controlled simultaneous cancellation-versus-
  entry race.
- Proved a cancellation-first result remains canceled.
- Proved an entry-first result remains one-shot after cancellation.
- Required exactly one competing owner to return terminal with one executor
  entry.
- Classified the losing owner as exactly one of:
  - a bounded blocked entry result with zero executor entries;
  - `trusted_host_redispatch.directive_replayed`; or
  - `trusted_host_redispatch.active_yield_missing`.
- Added direct non-leakage assertions over both owner results.

## 3. Scope Explicitly Not Completed

This phase did not change cancellation behavior, owner outcomes, operational
entry, redispatch, application topology, runtime adoption, discovery,
automatic invocation, automatic approval, signal handling, public APIs, CLI,
SDKs, schemas, providers, hosted scheduling, writes, or release posture.

## 4. Simultaneous Race Proof

Each iteration creates one cancellation pair, starts `begin_entry` and
`cancel` on separate scoped threads, and releases both through the same
barrier. The test accepts only the two mutex-linearized outcomes:

- cancellation wins and every later entry observation remains canceled; or
- entry wins and every later entry attempt receives the stable
  `trusted_host_local_timer.entry_already_started` error.

This exercises actual contention on the production mutex rather than only
serial permutations.

## 5. Competing-Owner Classification

The direct durable-state test still starts two owners against the same exact
operation. It now requires:

- aggregate executor admission equals one;
- exactly one terminal winner reports one executor entry;
- exactly one loser reports a reviewed bounded zero-entry posture or stable
  replay/state-unavailable error; and
- no result Debug text contains the run ID or skill output marker.

The loser set reflects transaction ordering at existing Core boundaries. The
test does not add an owner-level lock or collapse valid concurrency outcomes
into one fabricated status.

## 6. Privacy And Error Posture

The hardening adds no new production errors or output. Tests assert that
accepted owner outcomes and stable errors remain bounded and omit durable run
identity and executor output. No payload, locator, skill input, path,
credential, or provider content is copied.

## 7. Validation

Focused validation passed:

- simultaneous cancellation-versus-entry race test;
- competing-owner classification test.

Full required validation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

Rust clippy and tests used `CARGO_INCREMENTAL=0` and the established clean
target directory `/private/tmp/workflow-os-owner-target`.

## 8. Remaining Limitations

- No application invokes the owner.
- No explicit local host application topology is selected.
- Active executor attempts remain non-interruptible.
- Signal orchestration and process shutdown ownership remain deferred.
- Hosted/PostgreSQL trusted-host scheduling parity remains deferred.

## 9. Recommended Next Phase

Perform a focused maintainer/security review of this test hardening. After
acceptance, plan the explicit local host application topology and internal
Core-to-application visibility boundary identified by the adoption plan.

Do not implement a hidden CLI path, state translation, provider broadening,
discovery, or automatic scheduling first.

## 10. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1791374261912918000-2`
- approval: `approval/run-1791374261912918000-2/implementation-approved`
- presentation: `presentation/da6ee0a779881dd7`
- presentation hash:
  `da6ee0a779881dd73c2d9178214ad787031bb436b864cb241f76e36ad63081cc`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: private owner test hardening only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source inspection, test edits, command execution,
  documentation, validation, and later git or pull-request actions
