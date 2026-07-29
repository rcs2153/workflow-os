# Filesystem-To-SQLite Writer Guard Capability Model Report

## 1. Executive Summary

The first writer-quiescence implementation slice is complete as model-only
vocabulary and validation. Workflow Core can now describe the required
cooperating local writer protocol, cross-process guard contract, compatibility
posture, bounded future acquisition outcomes, and one immutable
`StateMigrationAttempt`.

This phase does not acquire a filesystem lock, inspect or stop a process,
create or open a SQLite destination, import records, mutate source state,
verify a destination, activate a backend, or expose migration CLI behavior.

## 2. Scope Completed

- Added typed writer, guard, and importer-transaction protocol versions.
- Added shared-writer and exclusive-migration guard modes.
- Added bounded future guard-acquisition outcome vocabulary.
- Added a typed local/cooperating/cross-process guard boundary and
  process-death release policy.
- Added a canonical writer-guard capability contract.
- Added a pure compatibility assessment requiring an exact source marker and
  explicit older-writer-stop assertion.
- Added an immutable migration-attempt binding and fingerprint.
- Added fail-closed serde reconstruction and redaction-safe Debug behavior.
- Added focused model, compatibility, binding, tamper, and privacy tests.

## 3. Scope Explicitly Not Completed

- filesystem or operating-system lock acquisition;
- root-wide shared guard integration in mutation paths;
- exclusive migration guard integration;
- source process discovery or termination;
- writer-protocol marker persistence;
- migration authority, importer, or destination creation;
- SQLite writes or transaction execution;
- source mutation, repair, rename, archival, or deletion;
- destination verification, receipt, or activation;
- CLI behavior, schemas, SDK changes, or examples;
- PostgreSQL, distributed locks, network filesystems, or shared workers;
- provider calls, provider writes, hosted behavior, or release changes.

## 4. Model Types Added

- `StateMigrationWriterProtocolVersion`
- `StateMigrationGuardProtocolVersion`
- `StateMigrationImporterTransactionVersion`
- `StateMigrationWriterGuardMode`
- `StateMigrationWriterGuardAcquisitionOutcome`
- `StateMigrationWriterGuardBoundary`
- `StateMigrationWriterGuardReleasePolicy`
- `StateMigrationWriterGuardCapability`
- `StateMigrationWriterCompatibilityPosture`
- `StateMigrationWriterCompatibility`
- `StateMigrationAttempt`

The capability is a required contract, not proof that a lock implementation is
available or acquired.

## 5. Compatibility Boundary

`StateMigrationWriterCompatibility::assess(...)` is pure. Compatible posture
requires:

- the local-filesystem source backend;
- a declared v1 writer-protocol marker;
- exact agreement with the v1 guard capability;
- explicit confirmation that incompatible older writers are stopped.

Missing markers or missing confirmation remain unverified. Backend mismatch is
incompatible. `StateMigrationAttempt::new(...)` rejects every posture except
compatible with stable error code
`state.migration.writer.compatibility.invalid`.

The model does not inspect live writers and does not turn the operator
assertion into machine proof.

## 6. Migration Attempt Binding

The attempt fingerprint binds:

- migration ID;
- migration-plan version and fingerprint;
- source backend and source fingerprint;
- logical destination ID;
- adapter schema version;
- writer-protocol version;
- guard-protocol version;
- importer-transaction version;
- exclusive-migration guard mode.

Changing source or adapter schema changes the attempt fingerprint. Future
authority, staging metadata, exact resume, verification, and receipts must bind
this same fingerprint.

## 7. Validation And Serde

Constructors keep derived posture private. Deserialization reconstructs or
validates canonical fields and rejects:

- weakened local/cooperating guard boundaries;
- changed release policy or guard modes;
- forged compatibility posture;
- unknown protocol versions;
- shared-writer mode on a migration attempt;
- changed attempt fingerprints;
- unknown fields.

Errors remain bounded and do not echo caller identities or fingerprints.

## 8. Privacy And Redaction

The models contain only validated identities, protocol vocabulary, booleans
derived from typed boundaries, schema numbers, and SHA-256 fingerprints.
Custom `Debug` for `StateMigrationAttempt` redacts migration ID, destination
ID, source fingerprint, plan fingerprint, and attempt fingerprint.

No path, raw record, provider payload, command output, environment value,
credential, authorization header, private key, or state payload is stored.

## 9. Test Coverage

Focused tests cover:

- canonical capability posture;
- all guard modes and bounded acquisition outcomes;
- exact compatible, unverified, and incompatible assessments;
- required older-writer assertion;
- attempt identity and protocol binding;
- deterministic fingerprint derivation;
- fingerprint changes from source or schema changes;
- rejection of unverified attempts;
- serde round trips;
- derived-posture, protocol, mode, and fingerprint tamper rejection;
- redaction-safe Debug output;
- path- and payload-free serialization;
- existing migration-plan regression coverage.

## 10. Commands Run And Results

- `cargo fmt --all`: passed.
- focused `workflow-core` migration tests: 25 passed.
- focused `workflow-core` clippy with warnings denied: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live integration tests remained
  ignored as designed.
- `npm run check`: passed under the repository-pinned Node 20 toolchain.
- `npm run check:integrations`: passed under the repository-pinned Node 20
  toolchain.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `cargo audit`: passed with no vulnerable dependency reported.
- `npm audit --audit-level=moderate`: passed with zero vulnerabilities.

## 11. Workflow Semantics

No workflow execution or durable-state behavior changed. The default
filesystem and SQLite backends are untouched. Constructing any new model does
not acquire a lock, confer migration authority, or make a destination
selectable.

## 12. Remaining Limitations

- No process can yet acquire the modeled guard.
- Existing local writers do not yet participate in root-wide shared exclusion.
- No path-independent writer-protocol marker exists.
- The older-writer-stop assertion is explicit but not independently verified.
- The model covers cooperating local Workflow OS writers only.
- Non-cooperating binaries, hostile processes, network filesystems, and
  distributed workers remain outside the guarantee.

## 13. Recommended Next Phase

The focused maintainer review accepts this model-only phase.

Implement the local filesystem cooperating writer guard across every mutation
path next, with separate-process contention and process-death tests. Do not
begin importer or destination-write work until that guard is accepted
independently.

## 14. Governed Phase Record

- workflow ID: `dg/implement`
- run ID: `run-1785305904017547000-2`
- approval ID:
  `approval/run-1785305904017547000-2/implementation-approved`
- approval outcome: granted with persisted presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced
- out-of-kernel work: Rust model/test edits, documentation edits, validation,
  and later git/PR operations are performed by the delegated maintainer outside
  the kernel and disclosed at phase close.
