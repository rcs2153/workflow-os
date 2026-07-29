# Filesystem-To-SQLite State Migration Plan Model Report

## 1. Executive Summary

Workflow OS now has an immutable, payload-free filesystem-to-SQLite migration
plan model. The model binds one validated migration identity to an accepted
read-only inventory fingerprint, a logical unreachable SQLite staging
destination, deterministic record-family operations, exact-plan resume
posture, and typed verification obligations.

This phase remains model-only. It does not create or open SQLite, import state,
rebuild projections, establish writer quiescence, write a verification receipt,
activate a backend, expose CLI behavior, or modify the filesystem source.

## 2. Scope Completed

- Added validated migration and destination identifiers.
- Added migration-plan contract version one.
- Added an immutable source binding derived only from a compatible inventory.
- Added a path-free SQLite staging-destination identity with fixed
  empty-required and runtime-unreachable posture.
- Added deterministic dependency ordering and required disposition for every
  known state family.
- Added exact-plan interruption/resume posture.
- Added typed pre-activation verification obligations.
- Added a plan fingerprint binding migration ID, source fingerprint,
  destination ID, adapter schema version, and canonical plan shape.
- Added serde reconstruction through validated types and canonical derived-plan
  comparison. Focused review subsequently found one remaining fail-closed gap
  in the serialized source quiescence posture.

## 3. Scope Explicitly Not Completed

- No importer, export stream, database creation, database write, SQL statement,
  or migration transaction exists.
- No filesystem state is mutated, repaired, archived, renamed, or deleted.
- No projection is rebuilt and no lock or writer-quiescence boundary exists.
- No verification receipt, activation decision, rollback behavior, backend
  selector, runtime default, or migration CLI exists.
- No PostgreSQL, provider, schema, SDK, example, hosted, collaborative, or
  release-posture change is introduced.

## 4. Model And API Summary

The phase adds:

- `StateMigrationId`
- `StateMigrationDestinationId`
- `StateMigrationPlanVersion`
- `StateMigrationSource`
- `StateMigrationDestinationPosture`
- `StateMigrationDestination`
- `StateMigrationPlanStep`
- `StateMigrationResumePolicy`
- `StateMigrationVerificationRequirement`
- `StateMigrationPlan`

`StateMigrationPlan::new` accepts validated logical identities, a compatible
`StateMigrationInventory`, and an explicit positive adapter schema version. It
derives all safety posture rather than accepting caller-controlled backend
kinds, destination visibility, family order, dispositions, resume semantics,
verification requirements, or activation coupling.

## 5. Source And Destination Binding

The source binding preserves:

- local-filesystem-preview backend identity;
- inventory contract version;
- source semantic fingerprint;
- writer-quiescence requirement.

The destination preserves only a bounded logical identity and adapter schema
version. Its backend is fixed to embedded SQLite, its posture is fixed to
staging, it must be empty, and ordinary runtime selection cannot reach it. No
source or destination filesystem path exists in the model.

The derived plan fingerprint changes when migration identity, source
fingerprint, destination identity, adapter schema version, or canonical plan
shape changes. A future importer can therefore bind interruption recovery to
the exact accepted plan without treating a caller label alone as sufficient
identity.

## 6. Deterministic Family Plan

The plan orders events first, then rebuildable event/run/approval projections,
approval-presentation records, idempotency and project state, policy and
adapter records, SideEffects and their projection, and WorkReports after their
cited identities. Local locks are explicitly excluded and immutable run bundles
are explicitly retained as companion state.

Every known family appears exactly once. Each step derives its disposition from
the inventory family contract, so canonical import, projection rebuild,
ephemeral exclusion, and companion preservation cannot drift independently.

## 7. Resume And Verification Boundary

Resume posture is `ExactPlanOnly`. The source must be re-inventoried before a
future import, and activation remains a separate future decision.

Typed verification obligations cover source stability, destination emptiness,
canonical counts and digests, event ordering and identity, run rehydration,
projection reconstruction, approval, SideEffect and WorkReport referential
integrity, telemetry and audit identity, lock exclusion, companion retention,
unknown destination records, and SQLite schema plus `quick_check` health.

These are obligations only. This phase does not execute or attest any check.

## 8. Privacy And Redaction

Migration and destination identifiers are bounded, character-restricted, and
reject secret-like values. Their `Debug` output is redacted. Source
fingerprints are redacted inside source and plan `Debug` output.

The model contains no paths, record payloads, command output, provider output,
environment values, credentials, authorization headers, private keys, or
operator prose. Stable validation errors do not echo rejected caller input.
Serde rejects unknown fields and most reconstructed posture that differs from
the canonical model. Focused review found that serialized
`quiescence_required` can still be weakened for the fixed local-filesystem
source. The focused blocker fix now requires writer quiescence through the
shared source validator for both construction and deserialization.

## 9. Test Coverage

Fourteen focused tests cover:

- valid source and unreachable staging-destination posture;
- malformed and secret-like identity rejection without leakage;
- incompatible source and zero schema rejection;
- complete deterministic family order and dispositions;
- exact resume, source recheck, and separate activation posture;
- complete stable verification obligations;
- plan-fingerprint sensitivity to source, destination, and schema changes;
- valid serde round trip;
- destination, step, verification, and derived-flag tamper rejection;
- invalid serialized secret-like identity non-leakage;
- redacted `Debug`;
- path-free, payload-free serialization.

Existing inventory, durable-state, SQLite, runtime, evidence, report, approval,
and provider tests remain part of workspace validation.

## 10. Validation Commands And Results

The following commands passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test -p workflow-core --test state_migration_plan` with 14 focused
  tests passing
- `npm run check:docs`
- `cargo audit`
- `git diff --check`

Opt-in live provider tests remained ignored by the workspace suite as designed.
The first sandboxed `cargo audit` attempt could not fetch the RustSec advisory
database because network access was unavailable; the required approved network
retry loaded 1,172 advisories and passed against 118 locked dependencies.

The implementation phase was governed by `dg/implement`:

- run ID: `run-1785285406037384000-2`
- approval ID:
  `approval/run-1785285406037384000-2/implementation-approved`
- approval outcome: granted after persisted presentation proof
- event summary: 39 events, one approval, zero retries, zero escalations
- terminal status: `Completed`

Repository edits, shell commands, validation commands, and this report were
performed by the agent outside the kernel execution layer. The kernel governed
scope and approval sequencing; it did not execute edits, tests, database work,
git operations, or provider actions. No required validation was skipped.

## 11. Remaining Known Limitations

- The model does not prove source writer quiescence.
- No importer or restart journal consumes the plan.
- No SQLite staging file or internal migration marker is created.
- Verification obligations have no executor in this phase.
- Immutable run bundles remain filesystem companion state.
- No completed verification receipt can authorize activation.
- Filesystem remains the preview default; SQLite remains explicit opt-in.

## 12. Recommended Next Phase

Perform a focused re-review of the fixed local-filesystem source quiescence
boundary.

The re-review should verify that public construction and serialized source
posture cannot remove required writer quiescence. It must preserve immutable
source/destination binding, canonical family order, exact-plan resume posture,
privacy, and the strict absence of importer or database behavior.
