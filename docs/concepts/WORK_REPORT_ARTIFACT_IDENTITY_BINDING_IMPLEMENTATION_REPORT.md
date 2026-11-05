# WorkReport Artifact Identity Binding Implementation Report

## Decision

Workflow OS now has one Core-owned expectation for reading persisted
`WorkReportArtifactRecord` values. A valid artifact body is no longer sufficient
when the artifact is physically returned from the wrong run, report slot,
workflow identity, or immutable execution identity.

This is the bounded implementation candidate for continuity-of-authority Gate
2, **Verified Means Correctly Bound**. It does not close the gate by itself.

## Problem Closed By The Candidate

Before this work, an artifact from run B could remain internally valid and
body-committed while being stored under run A's external filesystem or database
key. Consumers could then inspect, reconcile, recover, or replay the artifact
without one shared validation contract binding the external locator to the
artifact's internal and immutable workflow identity.

Body integrity and location identity are independent properties. A complete
body commitment proves that the body matches its retained commitment. It does
not prove that the artifact belongs in the slot from which it was read.

## Core Model

`WorkReportArtifactLocationExpectation` binds:

- the requested run identity;
- an optional exact report identity;
- the workflow ID when known;
- workflow version, schema version, and specification content hash when a full
  immutable run identity is available; and
- a payload-free locator class for filesystem, SQLite, PostgreSQL,
  PostgreSQL-transaction, or injected-store reads.

The artifact type itself supplies the artifact-kind boundary. Debug output
redacts run, report, workflow, workflow-version, and specification identities.
The locator class carries no path, database key, provider payload, or secret.

## Required Validation Order

All composed reads use this order:

1. decode and structurally validate the report and artifact envelope;
2. bind the external run/report key to internal artifact identity;
3. bind available workflow and immutable execution identity;
4. verify the retained complete-body commitment when present; and
5. only then compare, project, reconcile, recover, or replay.

Wrong-location artifacts return the stable payload-free code
`work_report_artifact.read.location_mismatch`. Body-only corruption returns
`work_report_artifact.read.corrupt`. If both identity and body are substituted,
the location mismatch wins because identity is checked before body commitment.

## Enforcement Surfaces

The common expectation is enforced at:

- local filesystem exact read and list, including physical filename checks;
- SQLite exact read and list using relational run/report keys;
- PostgreSQL exact read and list using record-family keys;
- PostgreSQL transaction-local hosted replay reads;
- default expectation-aware methods on injected `WorkReportArtifactStore`
  implementations;
- hosted and collaborative report-metadata projection endpoints;
- human and JSON inspect;
- first-run terminal artifact replay;
- current-authority WorkReport metadata projection;
- store-backed continuation-context collection;
- authoritative CLI artifact recovery; and
- executor duplicate reconciliation.

Duplicate reconciliation preserves its established semantics: an exact valid
artifact is `AlreadyPersisted`, a valid identity conflict is a duplicate
conflict, and an unreadable outcome remains ambiguous and retry-blocked.

## Adversarial Coverage

Focused tests prove that:

- an injected store cannot return another run's valid artifact;
- a valid artifact physically placed under another filesystem run/report slot
  is rejected by exact read and list;
- a wrong immutable workflow identity is rejected without identifier leakage;
- combined location or immutable-workflow-identity and body substitution fails
  at location binding first;
- a legacy-unverified artifact is accepted in its correct slot but rejected in
  the wrong slot;
- SQLite rejects a valid artifact moved under different relational keys and
  checks immutable identity before body commitment;
- PostgreSQL CI exercises relational-key misplacement, immutable-identity plus
  body substitution, and restoration; and
- errors and debug output do not disclose run IDs, report IDs, paths, report
  text, or body commitments.

The initial WorkReport test target passed 241 tests. Focused review added
filesystem and SQLite immutable-identity/body-order regressions, bringing the
target to 243 tests, and added the corresponding credential-backed PostgreSQL
scenario. The broader Core all-features
run exposed one duplicate-reconciliation classification regression; the
implementation was corrected so location mismatch during duplicate lookup maps
to the existing duplicate-conflict posture rather than ambiguous persistence.
The corrected complete Core all-features run then passed, including 784 Core
unit tests, 348 local-executor tests with one opt-in live test ignored, all 243
WorkReport tests, integration targets, and doc tests. The CLI package passed
its unit, 179 CLI integration, and example targets. Clippy and rustdoc passed
with warnings denied, and formatting, documentation, release-state, and diff
checks passed. PostgreSQL test code compiled locally; credential-backed
PostgreSQL execution remains exact-head CI evidence rather than a local claim.

## Honest Boundary

This work strengthens storage-location and immutable-identity integrity. It
does not provide:

- storage-owner tamper resistance;
- signing, external witnesses, or cryptographic provenance;
- migration or backfill of legacy artifacts;
- provider mutation broadening;
- new schemas, commands, or SDK exposure;
- release publication; or
- Gate 3 project-resource authorization continuity.

Legacy artifacts remain explicitly `legacy_unverified` for complete-body
commitment. That posture does not relax run, report, workflow, or storage-slot
identity.

## Acceptance Still Required

Gate 2 remains open until all of the following pass:

1. exact-head CI, including PostgreSQL execution;
2. an independent filesystem and backend misplacement rerun;
3. confirmation that inspect and replay fail before projection or
   `already_persisted`; and
4. post-merge exact-main regression evidence.

The focused maintainer/security review is accepted after correcting validation
ordering and hosted-projection blockers. See [WorkReport Artifact Identity
Binding Review](WORK_REPORT_ARTIFACT_IDENTITY_BINDING_REVIEW.md).

Gate 3 and the cross-gate review remain blocked until this acceptance evidence
is complete.

## Governed Phase Record

The implementation phase began under:

- workflow: `dg/implement`;
- run: `run-1793880571944758000-2`;
- approval: `approval/run-1793880571944758000-2/implementation-approved`;
- approval presentation: `presentation/56cf1d7822d98b19`; and
- approval outcome: granted under the delegated-maintainer authority after the
  complete emitted handoff was presented.

The phase-start execution reported completion before implementation work began.
At closeout, the repository helper's default temporary state root no longer
contained the run's event stream. `phase-close` therefore failed with
`runtime.events.empty`, and a bounded search of the helper's known local state
roots found no durable record for the run ID. No state was edited or recreated.

This is disclosed as out-of-kernel governance continuity failure, not a
successful governed close. It does not invalidate the code and test evidence,
but it prevents claiming a re-inspected closeout event summary for this phase.
The focused review must account for this missing durable closeout record.
