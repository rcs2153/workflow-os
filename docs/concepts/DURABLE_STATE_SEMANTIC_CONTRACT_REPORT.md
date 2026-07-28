# Durable State Semantic Contract Report

Report date: 2026-07-28

## 1. Executive Summary

Workflow OS now has a Core-owned durable-state semantic contract and an
executable common conformance harness without a database dependency.

The contract makes backend guarantees explicit. The preview filesystem backend
passes only the local behaviors it can demonstrate and declares stronger
transaction, concurrency, lease, schema, and recovery guarantees unsupported.
It does not acquire those guarantees merely because individual file operations
succeed.

## 2. Scope Completed

- Added versioned durable-state contract vocabulary.
- Enumerated the seven accepted Core transaction families.
- Added capability, support, conflict, revision, lease, and adapter-schema
  vocabulary.
- Added a fallible backend contract-declaration boundary.
- Declared the current `LocalStateBackend` posture.
- Added a reusable executable conformance runner for a fresh disposable
  backend.
- Added focused behavior, serialization, invalid-input, and non-leakage tests.
- Updated the roadmap and accepted store-selection plan.

## 3. Scope Explicitly Not Completed

The phase did not add:

- a database crate or dependency;
- SQLite or PostgreSQL adapters;
- SQL, database schemas, or migrations;
- filesystem-state migration;
- cross-record transactions;
- compare-and-set mutation APIs;
- expiring or fenced leases;
- concurrent shared workers;
- backup or restore behavior;
- hosted or collaborative runtime behavior;
- provider or sandbox expansion;
- workflow schema or SDK changes;
- CLI features or examples;
- release posture changes.

## 4. Contract Model

The model adds:

- `DurableStateContractVersion`;
- `DurableStateBackendKind`;
- `DurableStateCapability`;
- `DurableStateTransactionKind`;
- `DurableStateTransactionSupport`;
- `DurableStateSupport`;
- `DurableStateConflictKind`;
- `DurableRevision`;
- `DurableLeaseSemantics`;
- `DurableStateSchemaMetadata`;
- `DurableStateSchemaPosture`;
- `DurableStateSemanticContract`;
- `DurableStateContractProvider`.

Contract construction rejects duplicate capabilities, duplicate transaction
declarations, and incomplete transaction-family declarations. Revision and
schema metadata deserialize through validation and fail closed.

## 5. Transaction Boundary

Core owns seven named transaction families:

1. append one validated next run event;
2. reserve idempotency and record pre-effect intent;
3. record an observed external-operation outcome;
4. record an approval decision with current context and presentation proof;
5. transition a SideEffect record with its authoritative event;
6. publish an immutable run bundle after reference validation;
7. commit an authoritative result before dependent projections.

No generic transaction escape hatch was added.

The filesystem backend declares every transaction family unsupported because
its multi-file operations do not provide crash-atomic cross-record commits.
The conformance runner fails when a backend claims a stronger transaction or
advanced capability before the harness has an executable scenario for it.

## 6. Conformance Harness

`run_durable_state_conformance` executes a prepared bounded fixture against a
fresh disposable backend.

The current common baseline verifies:

- backend health;
- contiguous ordered event append and deterministic reads;
- immutable run-identity mismatch rejection;
- duplicate event ID rejection;
- duplicate event sequence rejection;
- non-contiguous sequence rejection;
- idempotency first-write replay;
- local lock contention, release, and reacquisition.

The report also emits explicit unsupported results for all seven transaction
families and these advanced capabilities:

- cross-record atomic commit;
- compare-and-set revision;
- expiring fenced leases;
- managed schema migration;
- verified backup and restore;
- shared-worker concurrency.

The current report therefore contains eight passed scenarios and thirteen
unsupported scenarios. Unsupported is a contract result, not simulated proof.

## 7. Filesystem Backend Posture

The filesystem backend currently declares:

- ordered event append;
- immutable run-identity validation;
- idempotency replay;
- process-local exclusive locking;
- process-local unfenced lease semantics;
- unmanaged adapter schema.

It does not claim database-grade crash atomicity, shared-worker coordination,
revision fencing, migrations, or operational recovery.

## 8. Privacy And Error Posture

The contract and conformance results are payload-free enums and bounded numeric
metadata. The conformance fixture does not implement `Debug`. Failures use
stable `durable_state.contract.*` codes and do not include run IDs,
idempotency keys, stored values, paths, provider payloads, credentials, or
secret-like test markers.

No raw provider payload, command output, source content, connection string, or
credential storage was added.

## 9. Tests

Focused tests cover:

- all applicable filesystem conformance scenarios;
- explicit unsupported transaction and advanced-capability posture;
- no overclaim of atomicity, CAS, fencing, migration, backup, or shared-worker
  guarantees;
- validated semantic-contract serialization round trip;
- invalid revision and schema metadata deserialization;
- invalid fixture failure without identity or secret-like value leakage.

The existing state backend contract remains unchanged and continues to test
in-memory and filesystem behavior.

## 10. Validation

Completed successfully:

- `cargo fmt --all --check`;
- focused durable-state contract tests: 6 passed;
- focused Clippy with warnings denied;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs` under the repository's supported Node 20 toolchain;
- `git diff --check`.

## 11. Remaining Limitations

- The common harness does not yet prove cross-record atomicity, concurrent
  writers, crash/restart behavior, CAS, lease expiry/fencing, migrations,
  corruption recovery, or backup/restore.
- The aggregate `StateBackend` still does not contain every newer specialized
  store trait.
- No adapter schema exists.
- No migration path from filesystem state exists.
- No database dependency has been reviewed.

## 12. Recommended Next Phase

Focused review is complete in the
[Durable State Semantic Contract Review](DURABLE_STATE_SEMANTIC_CONTRACT_REVIEW.md).
The phase was accepted after bounded negative-scenario IDs and immutable
run-identity mismatch proof were added.

Proceed to the opt-in SQLite embedded adapter. PostgreSQL, collaboration
consumers, hosted behavior, and migration tooling remain later separately
governed phases.
