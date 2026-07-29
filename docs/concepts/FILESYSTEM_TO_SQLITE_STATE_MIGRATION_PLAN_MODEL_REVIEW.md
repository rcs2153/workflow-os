# Filesystem-To-SQLite State Migration Plan Model Review

Review date: 2026-07-28

## 1. Executive Verdict

**Needs blocker fixes.**

The model-only phase is otherwise appropriately small and conservative. It
binds a migration identity to an accepted inventory fingerprint, an unreachable
logical SQLite staging destination, canonical family operations, exact-plan
resume posture, and typed verification obligations without adding importer or
database behavior.

One fail-closed deserialization gap must be corrected before importer work:
serialized source posture can remove the required local-filesystem writer
quiescence flag while retaining an otherwise accepted plan.

## 2. Scope Verification

The implementation stayed within the approved model-only scope.

It added no importer, SQLite creation or write, state mutation, projection
rebuild, writer lock, verification executor, activation, backend selection,
CLI behavior, schema, SDK, example, provider, hosted behavior, or release
posture change.

## 3. Model Assessment

The public model is domain-neutral and appropriately bounded:

- `StateMigrationId` and `StateMigrationDestinationId`;
- `StateMigrationPlanVersion`;
- `StateMigrationSource`;
- `StateMigrationDestinationPosture` and `StateMigrationDestination`;
- `StateMigrationPlanStep`;
- `StateMigrationResumePolicy`;
- `StateMigrationVerificationRequirement`;
- `StateMigrationPlan`.

Identifiers are validated and redaction-safe. Constructor-created plans derive
their source from a compatible inventory and do not accept caller-controlled
family order, dispositions, destination reachability, resume semantics,
verification requirements, or activation coupling.

## 4. Source Binding Assessment

`StateMigrationSource::from_inventory` correctly preserves:

- local-filesystem backend identity;
- inventory contract version;
- source semantic fingerprint;
- writer-quiescence requirement.

The source fingerprint itself binds the inventory's quiescence posture.
Constructor-created plans therefore retain the accepted inventory boundary.

The deserialization validator, however, checks only the backend kind.
`StateMigrationSourceWire` accepts a caller-controlled
`quiescence_required`, and `StateMigrationPlan` reconstructs its canonical
derived fields from that weakened source. Because the plan fingerprint uses the
already-supplied source fingerprint but not the separately serialized
quiescence flag, changing only the flag from `true` to `false` does not
invalidate the plan.

For the fixed local-filesystem source, writer quiescence is mandatory. This is
a blocker rather than a documentation follow-up.

## 5. Destination Assessment

The destination is a bounded logical identity, not a filesystem path. Its
backend is fixed to embedded SQLite, posture is fixed to staging, emptiness is
required, and runtime selection is disabled.

Custom deserialization reconstructs the destination through
`staging_sqlite` and rejects tampered backend, posture, emptiness, selection,
or zero-schema values.

## 6. Family Plan And Resume Assessment

All 16 known state families appear exactly once in stable dependency order.
Each disposition is derived from the record-family contract:

- canonical import;
- projection rebuild;
- ephemeral exclusion;
- companion preservation.

Resume posture is `ExactPlanOnly`, source re-inventory is required, and
activation remains separate. Serialized step, order, disposition, requirement,
resume, source-recheck, and activation tampering is rejected through canonical
plan comparison.

## 7. Verification Assessment

The 15 typed obligations cover:

- source stability and destination emptiness;
- canonical count and digest comparison;
- event ordering and identity;
- run rehydration and projection consistency;
- approval, SideEffect, WorkReport, telemetry, project, and audit references;
- lock exclusion and companion retention;
- unknown destination records;
- SQLite schema metadata and `quick_check`.

These remain obligations only. No check is executed or attested in this phase.

## 8. Privacy And Error Assessment

The model stores no filesystem paths, records, provider payloads, command
output, environment values, credentials, authorization headers, private keys,
or operator prose.

Identifiers reject malformed and secret-like input without echoing it.
Identifier and source `Debug` output redacts logical IDs and the source
fingerprint. Serde errors are stable and do not include rejected metadata.

## 9. Test Quality Assessment

The 14 focused tests cover constructor validity, identifier errors, source and
schema rejection, family ordering and dispositions, exact resume posture,
verification obligations, fingerprint sensitivity, serde round trip, derived
posture tampering, secret-like input, redacted `Debug`, and path-free
serialization.

Missing blocker regression:

- changing serialized `source.quiescence_required` from `true` to `false`
  should fail closed.

The blocker fix should add that exact test without broadening into importer or
runtime behavior.

## 10. Documentation Assessment

The roadmap, implementation plan, and phase report correctly keep migration
model-only and preserve the filesystem preview default. This review adds a
fix-forward note where the phase report previously described serde posture too
broadly.

## 11. Blockers

1. **Serialized source posture can weaken required writer quiescence.**
   `StateMigrationSource::validate` accepts
   `LocalFilesystemPreview` with `quiescence_required: false`.
   `StateMigrationPlan` then reconstructs around that source, and the unchanged
   source fingerprint does not expose the mismatch. Require writer quiescence
   for this fixed source backend and add a serialized-plan tamper regression.

## 12. Non-Blocking Follow-Ups

- If a future source backend can legitimately omit quiescence, model that
  backend-specific invariant explicitly rather than weakening the
  local-filesystem contract.
- Consider including all independently serialized source safety fields in a
  later fingerprint contract version for defense in depth.
- Keep the source re-inventory and cross-process exclusion protocol mandatory
  before any importer reads canonical records.

## 13. Recommended Next Phase

Implement a focused **migration plan quiescence-deserialization blocker fix**.

The fix should:

- reject local-filesystem source posture with writer quiescence disabled;
- reject the same tamper inside a serialized plan;
- preserve stable non-leaking errors and current valid serde behavior;
- leave importer, SQLite write, activation, CLI, and backend selection absent.

After the fix, perform a focused re-review before starting the verified
importer helper.

## 14. Validation

Before this review, the implementation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `cargo audit`;
- `git diff --check`.

The review independently inspected the model, tests, implementation plan,
report, exports, and accepted inventory boundary.

## 15. Governed Review Record

- workflow: `dg/review`;
- run ID: `run-1785299979262145000-2`;
- approval ID:
  `approval/run-1785299979262145000-2/review-scope-approved`;
- presentation ID: `presentation/c9c75437e73b7c3d`;
- approval outcome: granted by delegated maintainer with persisted proof;
- phase status: completed;
- event summary: 39 events, one approval, zero retries, zero escalations;
- approval-presentation enforcement: proof enforced with an event marker.

Review inspection, documentation edits, validation, git, and PR work occurred
outside the kernel. The kernel governed phase scope and approval sequencing; it
did not execute those operations.
