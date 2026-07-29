# Filesystem-To-SQLite State Migration Plan Model Blocker Fix Report

## 1. Executive Summary

The migration plan model now fails closed when local-filesystem source posture
does not require writer quiescence.

The correction uses the existing `StateMigrationSource` validation boundary for
both public plan construction and deserialization. It does not add importer,
database, runtime, activation, or CLI behavior.

## 2. Blocker Fixed

Focused review found that serialized `StateMigrationSource` accepted
`quiescence_required: false` while its backend remained
`LocalFilesystemPreview`.

`StateMigrationPlan` reconstructed its canonical derived fields from that
weakened source. Because the supplied source fingerprint was unchanged, the
plan could deserialize without exposing the mismatch.

## 3. Implementation Approach

`StateMigrationSource::validate` now requires:

- backend kind `LocalFilesystemPreview`;
- `quiescence_required: true`.

`StateMigrationSource::from_inventory` now calls the same validator after
deriving the source binding. This prevents a caller from constructing a plan
from a public inventory that disables the fixed local-filesystem safety
posture.

Custom source deserialization already called the validator, so the same
invariant now closes the persisted-input path without adding another wrapper or
parallel policy.

## 4. Validation Boundary

Invalid quiescence posture returns the stable code:

`state.migration.source.quiescence.invalid`

The error does not include source fingerprints, paths, IDs, payloads, or caller
values. Serde maps the failure to the existing bounded
`state migration source is invalid` message.

Valid source inventory and valid plan serde behavior remain unchanged.

## 5. Privacy And Redaction

The fix stores no new data and adds no output field.

It does not expose:

- source or destination paths;
- source fingerprints;
- record payloads;
- provider or command output;
- environment values;
- credentials, authorization headers, private keys, or token-like values.

Existing redacted `Debug` and bounded serde error behavior remains intact.

## 6. Test Coverage

Focused regressions verify:

- a complete local-filesystem inventory with quiescence disabled cannot create
  a migration plan;
- changing serialized `source.quiescence_required` from `true` to `false`
  rejects the complete plan;
- valid plan serde round trip remains supported;
- existing destination, step, requirement, resume, and activation tamper tests
  remain green.

The focused migration-plan suite now contains 14 tests with the new assertions
inside the existing source and tamper cases.

## 7. Scope Explicitly Not Added

- No importer or export stream.
- No SQLite creation, connection, statement, transaction, or write.
- No filesystem mutation, repair, archival, deletion, or lock handling.
- No verification executor, receipt, activation, rollback, or backend selector.
- No CLI, schema, SDK, example, provider, hosted, collaborative, or release
  change.

## 8. Validation Commands

Passed validation:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test -p workflow-core --test state_migration_plan`
- `npm run check:docs`
- `cargo audit`
- `git diff --check`

The focused suite passed 14 tests. The full workspace suite passed with opt-in
live-provider tests ignored as designed. The first sandboxed `cargo audit`
refresh could not reach the advisory repository; the approved network retry
loaded 1,172 advisories and scanned 118 locked dependencies without reporting a
vulnerability.

## 9. Governed Phase

- workflow: `dg/blocker`;
- run ID: `run-1785300332333113000-2`;
- approval ID: `approval/run-1785300332333113000-2/fix-approved`;
- presentation ID: `presentation/5315a25c3cd9af6c`;
- approval outcome: granted by delegated maintainer with persisted proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations;
- approval-presentation enforcement: proof enforced with an event marker.

Repository edits, tests, documentation, git, and PR work occur outside the
kernel. The kernel governs scope and approval sequencing; it does not execute
those operations.

## 10. Remaining Limitations

- Writer quiescence is required but not established by this model.
- No cross-process exclusion protocol exists yet.
- No importer consumes the plan.
- No destination verification or activation receipt exists.
- Filesystem remains the preview default.

## 11. Recommended Next Phase

Perform a focused maintainer re-review of this blocker fix.

The fix is accepted in the
[Filesystem-To-SQLite State Migration Plan Model Blocker Fix Review](FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_MODEL_BLOCKER_FIX_REVIEW.md).
The next phase should define and review writer exclusion and the importer
transaction boundary before authorizing destination writes.
