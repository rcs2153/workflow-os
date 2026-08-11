# Proportional-Governance Authority-Receipt Local Store Report

## 1. Executive Summary

Workflow OS now provides a production local create-only filesystem store for
decision-time governance authority receipt records. The store accepts only the
trusted in-memory receipt type, persists exact serialized evidence atomically,
and returns only the explicitly unverified, non-authorizing persisted-record
type. Durable provenance still does not become reusable authority.

## 2. Scope Completed

- Added `LocalGovernanceDecisionAuthorityReceiptRecordStore`.
- Added atomic create-only record publication.
- Added exact duplicate reconciliation and explicit `AlreadyExists` outcomes.
- Added safe hex-encoded receipt-address file names.
- Added exact read, validation, and storage-address verification.
- Added fail-closed corrupt and conflicting record behavior.
- Added redaction-safe store Debug output.
- Added focused restart, race, missing, corruption, conflict, path, and
  redaction tests.

## 3. Scope Explicitly Not Completed

This phase did not add automatic receipt persistence, executor or artifact
wiring, `StateBackend` integration, listing or discovery, events, audit
projection, schemas, CLI/UI behavior, provider or OpenShell integration,
SideEffect execution, hosted persistence, reusable authority, or release
posture changes.

## 4. Store API Summary

The local store implements the existing transport-neutral
`GovernanceDecisionAuthorityReceiptRecordStore` contract. Callers provide a
trusted `GovernanceDecisionAuthorityReceipt` to write one record and a stable
receipt ID to read one exact persisted record.

The store root is explicit caller input. Records live under one `records`
directory and use a hex encoding of the complete receipt ID, so receipt IDs
cannot become path structure. Debug output redacts the configured root.

## 5. Atomicity And Duplicate Behavior

The store serializes and validates the trusted receipt before touching disk,
writes a unique create-only temporary file, syncs it, and publishes through a
create-only hard link. The temporary file is removed after either success or
failure.

An exact existing byte sequence returns `AlreadyExists`. A different valid
record under the same address fails with
`governance_decision_authority_receipt_store.duplicate.conflict`. Corrupt
existing content fails closed as an invalid record and is never repaired or
overwritten automatically. Concurrent exact writers reconcile to one
`Written` result and idempotent `AlreadyExists` results.

## 6. Trust And Validation Boundary

Writes accept only the opaque trusted receipt produced by the accepted
proof-enforced decision path. Reads deserialize and validate
`PersistedGovernanceDecisionAuthorityReceiptRecord`, verify that its receipt ID
matches the storage address, and preserve these fixed postures:

- `unverified_serialized_claim`;
- `evidence_only_not_authorization`;
- `point_in_time_only`; and
- `local_unsigned`.

There is no persisted-record-to-trusted-receipt conversion.

## 7. Privacy And Redaction

The store adds no runtime facts, approval bodies, provider output, command
output, parser payloads, source contents, paths, environment values,
credentials, tokens, or arbitrary metadata. Stable errors do not echo receipt
IDs, stored bytes, file paths, or secret-like values. Missing reads return
`None` without creating the store directory.

## 8. Test Coverage

Six focused local-store tests cover process-restart durability, safe addressing,
eight concurrent exact writers, side-effect-free missing reads, corrupt record
failure without repair, conflicting valid identity failure, and redacted Debug
output. Existing transport-neutral store tests continue to cover trusted input,
non-authorizing reads, denial, serialization, and privacy behavior.

## 9. Commands Run And Results

- Focused local authority-receipt store tests: passed, 6 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1786427725121674000-2`
- Approval ID: `approval/run-1786427725121674000-2/implementation-approved`
- Presentation ID: `presentation/f0fdef70456f68b6`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: Rust implementation, tests, documentation, validation,
  phase reporting, and git/PR operations

## 11. Remaining Limitations

- Receipt persistence is explicit and never automatic.
- No receipt listing, discovery, migration, or shared-state backend exists.
- A local unsigned record does not authenticate issuer provenance.
- WorkReport artifacts do not yet resolve receipt citations against the store.
- No atomic combined receipt-and-artifact operation exists.

## 12. Recommended Next Phase

Implement the explicit validation-only WorkReport artifact authority-receipt
referential-integrity helper. Keep persistence and artifact writing as separate
caller-visible operations and defer executor composition.
