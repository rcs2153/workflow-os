# Filesystem-To-SQLite Writer Guard Capability Model Review

## 1. Executive Verdict

**Phase accepted; proceed to local filesystem cooperating writer guard
implementation.**

The implementation is appropriately model-only, deterministic, fail-closed,
redaction-safe, and compatible with the accepted writer-quiescence plan. It
does not overclaim that exclusion exists.

## 2. Scope Verification

The phase stayed within approved scope:

- typed protocol, mode, boundary, release-policy, and outcome vocabulary;
- pure compatibility assessment;
- immutable migration-attempt binding;
- validated serde and redaction-safe Debug behavior;
- focused tests and documentation.

No filesystem guard, process inspection, mutation-path integration, SQLite
open or write, import, verification, activation, CLI behavior, schema change,
provider work, or release change was introduced.

## 3. Capability Model Assessment

`StateMigrationWriterGuardCapability::local_filesystem_v1()` is canonical and
appropriately narrow. It requires:

- the local filesystem preview backend;
- writer protocol v1;
- guard protocol v1;
- shared-writer and exclusive-migration modes;
- a local, cross-process, cooperating-writers-only boundary;
- release on process exit.

Private fields and canonical deserialization prevent a caller from weakening
those requirements. The API documentation correctly states that construction
does not prove availability or acquire a lock.

## 4. Compatibility Assessment

Compatibility is derived rather than caller-labelled. It remains:

- `Compatible` only for the exact local backend, v1 source marker, exact
  capability protocol, and explicit older-writer-stop assertion;
- `Unverified` for a missing source marker or missing assertion;
- `Incompatible` for backend mismatch.

`StateMigrationAttempt::new(...)` accepts only compatible posture and returns
stable code `state.migration.writer.compatibility.invalid` otherwise.

The operator assertion remains a statement, not machine proof. That is
documented and acceptable for this model phase.

## 5. Attempt Binding Assessment

The attempt fingerprint includes every fact required by the accepted plan:

- migration ID;
- plan version and plan fingerprint;
- source fingerprint;
- destination identity;
- adapter schema version;
- writer-protocol version;
- guard-protocol version;
- importer-transaction version;
- exclusive-migration mode.

The source backend is also retained as validated posture. Source or schema
changes produce different fingerprints. Future authority, staging metadata,
resume, verification, and receipts must use this fingerprint rather than
reconstructing a weaker identity.

## 6. Serde And Tamper Assessment

The capability and compatibility types reconstruct canonical derived posture.
The attempt validates its protocol versions, backend, schema, exclusive mode,
and derived fingerprint. Unknown fields and unknown enum values fail closed.

The model does not provide authenticity against a party able to construct an
entire new valid serialized attempt. That is not a blocker: this is an
immutable identity model, not a signature or authority receipt. Future
consumers must compare it to authoritative plan and source records.

## 7. Privacy Assessment

The model stores no paths or payloads. Attempt Debug output redacts all IDs and
fingerprints. Validation and deserialization errors do not echo supplied
identity or digest values. Serialization contains bounded typed posture and
validated references only.

No raw state, provider payload, command output, environment value, credential,
authorization header, private key, or process detail is present.

## 8. Test Quality Assessment

Focused tests cover:

- canonical capability requirements;
- mode and acquisition-outcome vocabulary;
- compatible, unverified, and incompatible posture;
- older-writer assertion requirements;
- complete attempt binding;
- deterministic and fact-sensitive fingerprints;
- incompatible-attempt rejection;
- serde round trips;
- derived posture, version, mode, and fingerprint tampering;
- Debug redaction and payload-free serialization;
- existing migration-plan regression behavior.

The complete workspace suite, integrations, docs, clippy, formatting, and
dependency audits also pass.

## 9. Runtime Compatibility Assessment

No current executor, filesystem store, SQLite store, CLI, or backend-selection
path consumes these types. Existing runtime semantics are unchanged. The
models cannot acquire a guard, mutate a source, create a destination, or confer
migration authority.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- The guard implementation must produce machine-observed acquisition posture;
  the capability contract alone must never be treated as proof.
- Define and persist a path-independent writer-protocol marker.
- Define the bounded operator assertion for stopping incompatible older
  writers at the runtime authority boundary.
- Keep guarded public mutation methods separate from unguarded internal
  helpers to prevent accidental nested acquisition.
- Use separate-process tests for contention and process-death release.

## 12. Recommended Next Phase

Implement the local filesystem cooperating writer guard.

The phase should add shared guard acquisition to every mutating
`LocalStateBackend` path and an exclusive read-only inspection proof for
migration, with separate-process contention, release-on-exit, and regression
tests. It must not create SQLite, import records, activate a backend, or add
migration CLI behavior.

## 13. Validation Evidence

- focused migration model tests: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check`: passed under pinned Node 20;
- `npm run check:integrations`: passed under pinned Node 20;
- `npm run check:docs`: passed;
- `git diff --check`: passed;
- Rust and npm dependency audits: passed.

## 14. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1785310760003391000-2`
- approval ID:
  `approval/run-1785310760003391000-2/review-scope-approved`
- approval outcome: granted with persisted presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced
- out-of-kernel work: source/test/doc inspection and review authoring were
  performed by the delegated maintainer outside the kernel.
