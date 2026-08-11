# Proportional-Governance Authority-Receipt Record Store Model Report

## 1. Executive Summary

Workflow OS now defines the first authority-receipt durability boundary: a
transport-neutral create-only store contract and an explicitly non-authorizing
persisted record model. Trusted decision-time receipts may enter the write
boundary, but records read back from storage remain structurally verified,
unauthenticated serialized claims. Durable evidence does not become reusable
runtime authority.

## 2. Scope Completed

- Added `PersistedGovernanceDecisionAuthorityReceiptRecord`.
- Added `GovernanceDecisionAuthorityReceiptRecordStore`.
- Added explicit `Written` and `AlreadyExists` write outcomes.
- Added read-only access to stable receipt references and fixed postures.
- Added a test-only in-memory store proving external implementability.
- Added focused deterministic, redaction, corruption, conflict, and denial
  tests.

## 3. Scope Explicitly Not Completed

This phase did not add a filesystem store, `StateBackend` integration,
automatic persistence, artifact referential-integrity validation, executor
wiring, events, audit projection, schemas, CLI/UI behavior, providers,
OpenShell behavior, SideEffect execution, hosted behavior, or reusable
authority.

## 4. API Summary

The store write method accepts only `GovernanceDecisionAuthorityReceipt`, the
opaque trusted type produced by the accepted proof-enforced decision path. The
read method returns `PersistedGovernanceDecisionAuthorityReceiptRecord`. No API
converts that record back into a trusted receipt.

The record exposes stable identity and commitment references needed by future
integrity checks while preserving the fixed postures:

- `unverified_serialized_claim`;
- `evidence_only_not_authorization`;
- `point_in_time_only`; and
- `local_unsigned`.

Rust's type privacy enforces that persisted records cannot be constructed as
trusted receipts through the public API. This compile-time distinction is
documented; behavior tests cover the accessible store and record surface.

## 5. Validation Boundary

Deserialization and explicit validation recompute the receipt commitment and
deterministic ID. Invalid or tampered serialized records fail closed with
bounded errors. Store implementations are required to treat first writes as
create-only, exact duplicates as idempotent, and conflicting content under an
existing identity as an error.

This proves structural self-consistency only. It does not authenticate an
issuer, establish freshness, or authorize a later action.

## 6. Privacy And Redaction

The model stores only the existing typed references, counts, timestamps,
postures, and commitments. It adds no raw runtime facts, approval bodies,
provider output, command output, parser data, source contents, paths,
environment values, credentials, tokens, or arbitrary metadata.

Manual Debug output redacts receipt, workflow, run, approval, event, and
commitment identities. Deserialization and store errors use stable codes and
do not echo serialized content or identifiers.

## 7. Test Coverage

Focused tests cover:

- first trusted receipt write;
- exact duplicate reconciliation;
- persisted read posture and stable-reference fidelity;
- Debug and serialization non-leakage;
- missing record behavior;
- denial producing no receipt to persist;
- corrupt stored bytes failing closed; and
- conflicting stored identity failing closed.

The in-memory store is test-only. No production persistence behavior is implied
by it.

## 8. Commands Run And Results

- Focused authority-receipt record-store tests: passed, 5 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1786426621812625000-2`
- Approval ID: `approval/run-1786426621812625000-2/implementation-approved`
- Presentation ID: `presentation/71b36281eb3ede58`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Approval presentation enforcement: proof-enforced with one durable
  presentation record
- Out-of-kernel work: Rust implementation, tests, documentation, validation,
  shell commands, validation, report documentation, and git/PR operations

## 10. Remaining Limitations

- No durable local receipt backend exists.
- No receipt discovery or listing exists.
- No report artifact resolves receipt citations yet.
- No atomic receipt/artifact composition exists.
- Local unsigned receipt records do not authenticate issuer provenance.
- Persistence failure semantics remain for the filesystem implementation
  phase.

## 11. Recommended Next Phase

Implement create-only local filesystem receipt persistence with exact duplicate
reconciliation and corrupt/conflicting-record failure behavior. Keep artifact
integrity, executor composition, schemas, CLI/UI behavior, providers,
SideEffects, hosted expansion, and reusable authority deferred.
