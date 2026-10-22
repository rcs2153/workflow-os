# Trusted-Host Terminal WorkReport Artifact Blocker Fix Report

## 1. Executive Summary

The focused implementation review found that terminal report artifact
generation and persistence failures were rendered as fixed failure text but
returned through the unpublished host's success path. The blocker is fixed.
Artifact obligation failures now produce bounded nonzero host outcomes while
the already-terminal workflow run and event history remain authoritative and
unchanged.

## 2. Blocker Fixed

`artifact_posture(...)` previously returned only a fixed string. `run()` wrapped
that string in `Ok(...)`, including for `GenerationFailed` and
`PersistenceFailed`. Callers could therefore observe process success when the
required terminal artifact was absent.

The host mapping now returns `Result<&'static str, &'static str>`:

- persisted and exactly reconciled artifacts remain successful fixed postures;
- report generation failure returns a fixed non-success posture; and
- artifact persistence failure returns a fixed non-success posture.

Fresh execution and terminal replay use the same closed mapping. No underlying
error, report content, artifact identity, run identity, path, or payload is
rendered.

## 3. Duplicate Failure Coverage

Two focused entry-point fixtures now exercise the real SQLite boundary:

- a different valid artifact at the same run/report key produces bounded
  persistence failure and leaves the conflicting row unchanged; and
- unreadable artifact JSON at the same key produces bounded persistence
  failure and leaves the unreadable row unchanged.

Both fixtures verify that terminal workflow events remain unchanged. The host
does not overwrite, repair, delete, or recreate either duplicate.

## 4. Corrupt-History Coverage

The existing contradictory result/run, cancellation, and event-order fixtures
now require process non-success. They no longer permit a zero-exit generation
failure posture. The tests continue to verify that no replacement artifact is
written.

Some corrupt histories are rejected at admission as
`trusted_host_local_application.failure.invalid_state`; histories that reach
report composition fail as
`admission.terminal_replay_report_generation_failed`. Both outcomes are fixed,
bounded, and non-successful.

## 5. Workflow Semantics

The correction changes only the unpublished host result contract. It does not
rewrite workflow pass/fail truth, append events, rerun the process, change the
run snapshot, alter artifact identity, or broaden runtime authority. A failed
artifact obligation is visible to the caller without changing the already
committed terminal run.

## 6. Privacy And Redaction

Success and failure output remain fixed posture strings. Tests require empty
stdout for artifact failures and exact bounded stderr. The implementation does
not expose report contents, identifiers, paths, command output, provider data,
source/spec contents, parser payloads, environment values, credentials,
authorization headers, keys, tokens, or approval reasons.

## 7. Test Coverage

The focused trusted-host suite now contains 21 passing tests. New or tightened
coverage proves:

- valid conflicting duplicate failure without overwrite;
- unreadable duplicate failure without repair;
- nonzero artifact-generation failure after a terminal failed run;
- nonzero corrupt-history replay failure;
- empty stdout and bounded stderr on artifact failure; and
- unchanged durable event history across duplicate reconciliation failure.

Existing passing, failed, replay, concurrency, crash-recovery, profile,
required-output, identity-substitution, and unsupported-topology behavior
continues to pass.

## 8. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 21
  tests.
- `cargo test --workspace`: passed; opt-in live-provider tests remained ignored
  by design.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Governed Phase Record

- Dogfood workflow ID: `dg/blocker`
- Run ID: `run-1792667683531433000-2`
- Approval ID: `approval/run-1792667683531433000-2/fix-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/c38a26cfc4380643`
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.

The kernel coordinated the proof-backed blocker scope and approval boundary.
The agent performed repository inspection, code edits, tests, documentation,
and git work outside the kernel under that approved scope.

## 10. Remaining Limitations

- The artifact path remains private and has no released inspect/export surface.
- The host supports only the accepted closed local profiles.
- The correction adds no generalized storage fault-injection abstraction.
- No provider, external write, scheduling, nested harness, schema, example,
  hosted, or release behavior is added.

## 11. Recommended Next Phase

Perform focused maintainer/security review of this blocker fix. The review
should verify the nonzero host contract, unchanged workflow truth, exact
duplicate handling, non-overwrite behavior, bounded output, and regression
coverage. Do not broaden the private artifact path during review.
