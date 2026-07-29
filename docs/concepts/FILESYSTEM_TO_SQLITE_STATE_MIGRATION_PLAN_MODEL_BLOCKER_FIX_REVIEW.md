# Filesystem-To-SQLite State Migration Plan Model Blocker Fix Review

Review date: 2026-07-28

## 1. Executive Verdict

**Blocker fixed; accept the migration plan and staging-destination core model.**

The local-filesystem source now requires writer quiescence through one shared
validation boundary used by public construction and deserialization. The exact
serialized-plan tamper fails closed, valid behavior remains intact, and no
migration execution behavior was added.

## 2. Scope Verification

The fix stayed within the approved blocker boundary.

It changed only:

- local-filesystem source validation;
- constructor reuse of that validator;
- focused migration-plan regressions;
- migration roadmap, reports, and review documentation.

It added no importer, SQLite creation or write, source mutation, writer lock,
verification executor, activation, backend selection, CLI, schema, SDK,
example, provider, hosted behavior, or release change.

## 3. Original Blocker

`StateMigrationSource` deserialization accepted
`quiescence_required: false` while retaining
`LocalFilesystemPreview` backend posture.

`StateMigrationPlan` then reconstructed canonical derived fields around that
source. The source fingerprint remained the supplied accepted digest, so
changing only the separate quiescence flag did not invalidate the plan.

## 4. Fix Assessment

`StateMigrationSource::validate` now requires:

- `LocalFilesystemPreview` backend identity;
- writer quiescence enabled.

`StateMigrationSource::from_inventory` constructs the source and then calls
that same validator. Custom deserialization already called it.

This is the smallest idiomatic correction. It avoids duplicate policy,
preserves the existing model shape, and guarantees that construction and
persistence cannot disagree about the fixed source invariant.

## 5. Validation And Error Assessment

A complete inventory with quiescence disabled now fails with:

`state.migration.source.quiescence.invalid`

The error is stable and does not include IDs, fingerprints, paths, or payloads.
Serde continues to return the bounded source-invalid message.

Changing serialized `source.quiescence_required` from `true` to `false`
rejects the containing `StateMigrationPlan`. Valid plan serialization and
deserialization remains unchanged.

## 6. Regression Assessment

Unchanged behavior includes:

- compatible inventory construction;
- validated migration and destination IDs;
- unreachable SQLite staging posture;
- canonical family ordering and dispositions;
- exact-plan resume posture;
- complete verification obligations;
- plan fingerprint behavior;
- redacted `Debug`;
- path-free and payload-free serialization.

## 7. Test Quality Assessment

The focused suite now proves both entry paths:

- a public inventory with disabled quiescence cannot create a plan;
- a serialized plan cannot disable source quiescence.

The regression is embedded beside existing incompatible-source and tamper
coverage, keeping the safety expectation close to the affected boundary.

The focused 14-test suite, formatting, workspace clippy, full workspace tests,
docs check, RustSec audit, and diff check all passed.

## 8. Privacy Assessment

The fix stores and emits no new caller data.

No source path, source fingerprint, destination identity, record payload,
provider output, command output, environment value, credential, authorization
header, private key, or token-like value is exposed.

## 9. Blockers

None for the migration plan and staging-destination model phase.

## 10. Non-Blocking Follow-Ups

- Keep local-filesystem writer quiescence mandatory through every future
  importer and activation boundary.
- If another source backend can omit quiescence, model its source-specific
  invariant explicitly rather than relaxing this validator.
- Consider a future plan-fingerprint version that directly binds every
  independently serialized source safety field as defense in depth.

## 11. Recommended Next Phase

Define and review the **cross-process writer-quiescence and importer transaction
boundary** before implementing destination writes.

That phase should specify:

- explicit migration authority;
- exclusive source-writer exclusion;
- stable source fingerprints before and after export;
- unreachable staging transaction semantics;
- deterministic interruption and restart behavior;
- source preservation;
- separation of import completion, verification receipt, and activation.

It must not add activation, automatic backend selection, CLI execution,
PostgreSQL, collaborative state, providers, or release claims.

## 12. Governed Review Record

- workflow: `dg/review`;
- run ID: `run-1785304430071829000-2`;
- approval ID:
  `approval/run-1785304430071829000-2/review-scope-approved`;
- presentation ID: `presentation/251d90ab388fbab0`;
- approval outcome: granted by delegated maintainer with persisted proof;
- phase status: `Completed`;
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
- approval presentation enforcement: `proof_enforced`.

Source inspection, review authoring, validation, git, and PR work occurred
outside the kernel. The kernel governed scope and approval sequencing; it did
not execute those operations.
