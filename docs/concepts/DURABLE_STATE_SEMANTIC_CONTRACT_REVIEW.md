# Durable State Semantic Contract Review

Review date: 2026-07-28

## 1. Executive Verdict

**Phase accepted after blocker fixes. Proceed to the bounded SQLite embedded
adapter implementation phase.**

The Core-owned contract is appropriately smaller than a database abstraction.
It names the transaction families and observable guarantees that future
backends must prove, while the executable harness records the preview
filesystem backend's actual behavior without granting it database-grade
atomicity, concurrency, migration, or recovery claims.

Two review blockers were corrected before acceptance:

- negative event scenarios no longer extend caller-supplied event IDs beyond
  the identifier bound;
- immutable run-identity validation now has an executable mismatch scenario
  before the filesystem backend may claim that capability.

No blocker remains.

## 2. Scope Verification

The implementation stayed within the approved database-free contract phase.

It added no:

- database dependency;
- SQLite or PostgreSQL adapter;
- SQL or migration;
- hosted or collaborative runtime;
- filesystem-state conversion;
- provider or sandbox expansion;
- CLI feature;
- workflow schema or SDK change;
- example update;
- release posture change.

## 3. Model Assessment

The model is domain-neutral and appropriately bounded. It defines:

- one versioned semantic contract;
- backend deployment posture;
- explicit capability declarations;
- all seven accepted Core transaction families;
- supported and unsupported transaction posture;
- conflict vocabulary;
- positive durable revisions;
- lease semantics;
- adapter-schema metadata;
- one fallible contract-provider boundary.

Construction rejects duplicate capabilities, duplicate transaction
declarations, and incomplete transaction-family declarations. Validated custom
deserialization prevents invalid revision, schema, or aggregate contract state
from bypassing constructors.

`DurableStateConflictKind` remains vocabulary for the first adapters. It does
not yet create a generic error-mapping layer or retry policy.

## 4. Transaction Boundary Assessment

The seven transaction families are specific enough to prevent a generic
transaction escape hatch:

1. validated next-event append;
2. idempotency reservation plus pre-effect intent;
3. observed external-operation outcome;
4. approval decision plus current context and presentation proof;
5. SideEffect transition plus authoritative event;
6. immutable run-bundle publication after reference validation;
7. authoritative result before dependent projections.

The filesystem backend declares all seven unsupported because its separate file
writes do not provide crash-atomic cross-record commits.

External provider effects remain outside the database transaction boundary.
The model does not claim exactly-once external effects or distributed rollback.

## 5. Capability Assessment

The filesystem backend claims only:

- validated ordered event history;
- immutable run-identity validation;
- idempotency first-write replay;
- process-local exclusive lock contention and release.

The common harness now executes each claimed behavior. It rejects a contract
that claims an advanced guarantee before an executable scenario exists.

The following remain explicitly unsupported:

- cross-record atomic commit;
- compare-and-set revision;
- expiring fenced leases;
- managed schema migration;
- verified backup and restore;
- shared-worker concurrency.

`OrderedEventAppend` means validated sequence ordering and deterministic reads
in this contract version. It does not mean crash-atomic multi-record append.
The transaction-family declaration remains the authoritative atomicity posture.

## 6. Conformance Harness Assessment

The harness requires a fresh disposable backend and produces a payload-free
report in stable scenario order.

It executes:

- backend health;
- contiguous event append and deterministic read order;
- immutable identity mismatch rejection;
- duplicate event ID rejection;
- duplicate event sequence rejection;
- non-contiguous sequence rejection;
- idempotency replay;
- lock contention, release, and reacquisition.

It returns eight passed and thirteen unsupported results for the local
filesystem backend.

The original negative scenarios created derivative IDs by appending suffixes to
caller-supplied fixture IDs. A valid maximum-length ID could therefore fail the
harness before exercising backend behavior. The fix uses bounded generated
event IDs and adds a maximum-length fixture regression test.

## 7. Filesystem Backend Honesty

The declaration matches the implementation and existing state tests.

It does not promote:

- an OS file operation into a cross-record transaction;
- a process-local lock into a distributed lease;
- an unmanaged directory layout into a migrated schema;
- successful local writes into backup or recovery proof;
- one-process behavior into shared-worker concurrency.

This is the required compatibility baseline, not a claim that the preview
filesystem backend is production durable state.

## 8. Error And Privacy Assessment

Contract validation and conformance failures use stable
`durable_state.contract.*` codes.

The fixture has no `Debug` implementation. Contract, scenario, and report
serialization contain enums and bounded numeric metadata rather than backend
paths, run payloads, idempotency values, provider data, credentials, or command
output.

Conformance errors collapse backend details to stable non-leaking failures.
Invalid fixture tests confirm that run identity and secret-like markers do not
appear in the error.

## 9. Serde And Compatibility Assessment

Valid local contracts round trip through JSON. Invalid revision, schema
metadata, duplicate declarations, and incomplete transaction declarations fail
closed through validated construction.

No workflow specification schema changed. The contract is Rust API vocabulary
for backend implementation and testing, not a user-facing configuration
surface.

## 10. Test Quality Assessment

Focused tests cover:

- all applicable local conformance scenarios;
- every unsupported transaction and advanced capability;
- maximum-length fixture event IDs;
- no overclaim of atomicity, CAS, fencing, migration, recovery, or
  shared-worker behavior;
- semantic-contract serde round trip;
- invalid revision and schema metadata;
- invalid fixture non-leakage.

The full workspace suite preserves existing runtime, approval, evidence,
report, adapter, SideEffect, onboarding, and provider-sandbox behavior.

The next adapter phase must add concurrency, crash/reopen, schema, transaction,
and recovery scenarios. Passing this local baseline alone is not sufficient for
SQLite acceptance.

## 11. Documentation Assessment

The roadmap, accepted store-selection plan, implementation report, and this
review consistently state:

- SQLite is selected for future embedded local state;
- PostgreSQL is selected later for shared collaborative state;
- one Core-owned semantic contract governs both;
- the filesystem backend has only preview local guarantees;
- no database adapter, migration, hosted runtime, or automatic conversion is
  implemented.

## 12. Blockers

None after the two review fixes.

## 13. Non-Blocking Follow-Ups

- Add executable classification coverage before `DurableStateConflictKind`
  becomes a retry or operator-decision input.
- Decide whether transaction families use an aggregate adapter boundary or
  narrowly typed transaction interfaces during SQLite implementation.
- Prove concurrent next-event conflict and deterministic retry behavior with
  multiple SQLite connections.
- Add crash/reopen and corruption scenarios before SQLite acceptance.
- Keep explicit filesystem-to-SQLite migration separate and opt-in.
- Preserve the recent product feedback priorities: quiet success for low-risk
  work, supported Node 20 tooling clarity, and removal of the duplicate
  missing-manifest diagnostic.

## 14. Recommended Next Phase

Implement the **SQLite embedded durable-state adapter**, opt-in and local only.

That governed phase should begin with dependency, license, supported-version,
and compile-option review, then implement only the schema and transaction
surface required to pass an expanded SQLite conformance suite.

It must not add automatic filesystem migration, PostgreSQL, hosted behavior,
collaboration consumers, provider mutations, CLI defaults, workflow schema
changes, or release changes.

## 15. Validation

| Validation | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| Focused durable-state contract tests | Passed, 6 tests |
| Focused Clippy with warnings denied | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` with pinned Node 20 | Passed |
| `git diff --check` | Passed |
| Dependency and schema diff inspection | No dependency or schema changes |

## 16. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1785227954855906000-2`
- approval ID:
  `approval/run-1785227954855906000-2/review-scope-approved`
- presentation ID: `presentation/a07feb6f781cdbb0`
- approval outcome: granted under delegated maintainer authority
- presentation proof: persisted
- terminal status: completed
- scope: focused contract and conformance review with blocker fixes only
