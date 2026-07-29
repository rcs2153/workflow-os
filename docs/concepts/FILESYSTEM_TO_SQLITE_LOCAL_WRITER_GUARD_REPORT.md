# Filesystem-To-SQLite Local Writer Guard Report

## 1. Executive Summary

Workflow OS now has an operating-system-backed local writer guard for the
preview filesystem state boundary. Ordinary cooperating writers acquire a
shared guard, while migration inspection can acquire an exclusive guard that
blocks those writers and exposes read-only source inspection.

The implementation covers `LocalStateBackend` mutation families and the
canonical immutable run-bundle companion store. It adds a path-independent
writer-protocol marker and separate-process contention and process-death
release tests.

This phase does not import state, create or write SQLite, verify a destination,
activate a backend, expose migration CLI behavior, or control non-cooperating
processes.

## 2. Scope Completed

- Added shared writer and exclusive migration guard acquisition using local
  operating-system advisory file locks.
- Added a path-independent v1 writer/guard protocol marker outside the source
  state root.
- Added stable, non-leaking contention, availability, identity, and protocol
  errors.
- Guarded every public `LocalStateBackend` mutation family.
- Guarded canonical immutable run-bundle companion writes.
- Added an exclusive read-only guard API for state and migration-inventory
  inspection.
- Added separate-process contention, process-death release, protocol,
  redaction, and mutation-family regression tests.
- Corrected atomic temporary-file naming so concurrent publications in the
  coordination directory cannot collide.

## 3. Scope Explicitly Not Completed

- no source import, export, repair, rename, archive, or deletion;
- no SQLite destination creation, open, transaction, or write;
- no destination verification, receipt, activation, rollback, or backend
  selection;
- no migration CLI or automatic migration;
- no process enumeration, signaling, or termination;
- no hostile or non-cooperating writer control;
- no network-filesystem or distributed-worker guarantee;
- no schema, SDK, example, provider, hosted, or release-posture change.

## 4. Guard API And Protocol

`LocalStateBackend::acquire_migration_exclusive_guard` acquires the exclusive
local guard and returns `LocalStateMigrationExclusiveGuard`. The guard exposes
only read-only state and migration-inventory inspection. Dropping the guard or
terminating its process releases the operating-system lock.

Ordinary mutation methods acquire the shared form of the same root identity.
The coordination file identity is derived from the canonical source-root
identity with SHA-256, so the coordination filename does not expose the source
path.

The persisted marker contains only:

- backend: `local_filesystem`;
- writer protocol: v1;
- guard protocol: v1.

An incompatible marker fails closed before guarded work proceeds.

## 5. Mutation Coverage

Shared acquisition covers public mutations for:

- workflow events and event indexes;
- run snapshots;
- idempotency outcomes;
- local logical locks;
- pending approvals;
- approval-presentation proofs and indexes;
- project state;
- policy audit records;
- adapter audit and observability records;
- WorkReport artifacts;
- SideEffect records and indexes;
- health-check write probes.

The canonical `state_root/immutable-run-bundles` store also participates in the
same root guard for definition, check-declaration, manifest, full-bundle, and
governance-assessment writes. Standalone immutable run-bundle stores outside
that canonical state layout retain their existing behavior and are not claimed
as part of migration-source quiescence.

Internal unguarded helpers remain private so compound public operations can
hold one shared guard without nested acquisition.

## 6. Concurrency And Release Behavior

Separate-process tests prove:

- a shared writer blocks exclusive migration acquisition;
- an exclusive migration guard blocks cooperating state mutation;
- abrupt writer process death releases a shared guard;
- abrupt migration-holder process death releases an exclusive guard;
- mutation succeeds after exclusive release.

The guard is advisory and protects only Workflow OS writers that participate
in this protocol. It does not inspect or stop older binaries, arbitrary local
programs, or hostile writers.

## 7. Privacy And Redaction

Guard filenames are hashes of canonical root identity. Protocol-marker content
contains no path, record, actor, payload, command output, provider value,
environment value, credential, token, authorization header, or private key.

Guard Debug output is redacted. Guard and protocol errors use stable codes and
do not echo source paths, marker contents, or secret-like test values.
Exclusive inspection returns existing bounded state and inventory models
rather than raw filesystem payloads.

## 8. Test Coverage

Focused coverage includes:

- shared-versus-exclusive contention across separate processes;
- release after shared-holder and exclusive-holder process death;
- direct contention coverage for every public `LocalStateBackend` mutation
  entry point, including layout creation, lock release, approval save,
  SideEffect update, and health-check mutation;
- direct contention coverage for adapter audit and observability writers;
- direct contention coverage for the WorkReport artifact writer;
- direct contention coverage for every canonical immutable run-bundle writer,
  including definition, local-check declaration, manifest, complete bundle,
  and governance-assessment binding publication;
- read-only inspection while the exclusive guard is held;
- marker location, bounded content, and incompatible-protocol failure;
- redaction-safe Debug and non-leaking errors;
- repeated concurrent state tests;
- existing immutable run-bundle and state regressions.

During repeated concurrent testing, the phase exposed a real collision in the
pre-existing atomic temporary-file naming scheme: temporary names did not
include the destination filename and could collide within the shared
coordination directory. A process-local atomic sequence is now included in
temporary filenames, and the repeated tests pass.

## 9. Commands Run And Results

- `cargo check -p workflow-core --offline`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-core --all-targets --offline -- -D warnings`:
  passed.
- `cargo clippy --workspace --all-targets --offline -- -D warnings`: passed.
- repeated `cargo test -p workflow-core --lib state::tests --offline`: 17
  passed in each run.
- `cargo test -p workflow-core --test immutable_run_bundle_store --offline`:
  16 passed.
- focused writer-entrypoint suites covering state, adapter telemetry,
  WorkReport artifacts, immutable run bundles, and proportional-governance
  bindings: passed.
- `cargo test --workspace --offline`: passed; opt-in live-provider tests
  remained ignored as designed.
- `npm run check`: passed under the repository-pinned Node 20 toolchain.
- `npm run check:integrations`: passed under the repository-pinned Node 20
  toolchain.
- `npm run check:docs`: passed.
- `cargo audit`: passed with no vulnerable dependency reported.
- `npm audit --audit-level=moderate`: passed with zero vulnerabilities.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- The guarantee covers cooperating local Workflow OS writers only.
- Older binaries that do not participate in the guard must still be stopped by
  an explicit operator boundary before migration.
- Advisory lock behavior is not claimed for network filesystems.
- The marker proves declared protocol compatibility, not writer authenticity
  or complete process discovery.
- Canonical immutable run-bundle writes are guarded; arbitrary standalone
  bundle stores are outside the migration inventory.
- No importer, destination transaction, verification, activation, or recovery
  behavior exists yet.

## 11. Recommended Next Phase

Perform a focused maintainer review of the local writer guard blocker fix. The
review must verify complete canonical mutation coverage, protocol and
contention behavior, process-death release, privacy, and strict non-scope.

If accepted, proceed to the accelerated Operational Embedded Durable State
vertical slice: atomic staging import, projection rebuild, destination
verification, explicit activation/rollback posture, bounded operator entry
point, and failure-injection tests. Do not weaken the independently reviewed
writer-quiescence boundary.

## 12. Governed Phase Record

- workflow ID: `dg/implement`
- run ID: `run-1785311673114956000-2`
- approval ID:
  `approval/run-1785311673114956000-2/implementation-approved`
- approval outcome: granted with persisted presentation proof
- approval presentation ID: `presentation/60528439457fbfaa`
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced
- out-of-kernel work: Rust source/test edits, documentation edits, validation,
  and later git/PR operations are performed by the delegated maintainer outside
  the kernel and disclosed at phase close.
