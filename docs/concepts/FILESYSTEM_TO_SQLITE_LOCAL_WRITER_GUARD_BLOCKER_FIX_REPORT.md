# Filesystem-To-SQLite Local Writer Guard Blocker Fix Report

## 1. Executive Summary

The local writer guard review found no unguarded public mutation path in source
but blocked acceptance because direct contention tests did not cover every
public canonical writer claimed by the migration quiescence boundary.

This fix completes that proof. Every omitted public state, telemetry, report,
and canonical immutable run-bundle writer now has a direct test showing that an
exclusive migration guard blocks the mutation, leaves no record behind, emits a
stable non-leaking error, and permits valid work after guard release where
applicable.

The guard design, migration scope, runtime behavior, and product posture are
unchanged.

## 2. Blocker Fixed

The fix adds direct proof for:

- `LocalStateBackend::new` layout creation;
- pending approval save;
- logical-lock release;
- SideEffect lifecycle update;
- adapter audit append;
- adapter observability append;
- WorkReport artifact publication;
- immutable run-bundle definition publication;
- canonical local-check declaration-set publication;
- immutable run-bundle manifest publication;
- complete immutable run-bundle publication;
- governance-assessment binding publication.

The existing matrix continues to cover event append, snapshot save,
idempotency recording, lock acquisition, approval deletion, project state,
policy audit, SideEffect creation, approval-presentation proof, and health-check
mutation.

## 3. Implementation Approach

The fix is test-only apart from documentation truth updates. It exercises the
existing public APIs while a real exclusive migration guard is held.

Assertions verify:

- the stable error code is `state.local.writer_guard.contended`;
- no source record, artifact, binding, or state layout is created;
- errors do not expose state-root paths or bounded fixture values;
- the same valid mutation succeeds after guard release where meaningful.

No private helpers were exposed and no test-only bypass was added.

## 4. Validation Boundary

Direct tests now cover every public canonical write entry point participating
in filesystem migration quiescence:

- local state and index mutations;
- adapter audit and observability records;
- WorkReport artifacts;
- canonical `state_root/immutable-run-bundles` records and bindings.

Standalone immutable run-bundle stores outside the canonical state-root layout
remain outside the migration guarantee, as already documented.

## 5. Privacy And Error Posture

Tests use bounded secret-like fixtures and state paths only to prove absence
from errors. No raw payload, provider value, command output, environment value,
credential, token, authorization header, private key, or source path is added
to guard state or error messages.

The fix does not change serialization, Debug behavior, marker contents, or
coordination identity.

## 6. Test Coverage

Focused suites now prove:

- state layout creation is blocked without leaving a directory;
- every public local-state mutation family is blocked;
- adapter audit and observability stores remain unchanged;
- WorkReport artifacts remain absent;
- each canonical immutable run-bundle record family remains absent;
- governance-assessment binding publication remains absent;
- valid writes resume after exclusive guard release.

Existing separate-process contention, process-death release, protocol-marker,
read-only inspection, redaction, and regression tests remain in place.

## 7. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets --offline -- -D warnings`:
  passed.
- `cargo test -p workflow-core --lib state::tests --offline --quiet`: passed.
- focused adapter, immutable run-bundle, proportional-governance binding, and
  WorkReport suites: passed.
- `cargo test --workspace --offline --quiet`: passed; opt-in live-provider
  tests remained ignored as designed.
- `npm run check`: passed under the repository-pinned Node 20 toolchain.
- `npm run check:integrations`: passed under the repository-pinned Node 20
  toolchain.
- `npm run check:docs`: passed.
- `cargo audit`: passed with no vulnerability reported.
- `npm audit --audit-level=moderate`: passed with zero vulnerabilities.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- The guard controls cooperating local Workflow OS writers only.
- Older binaries, hostile programs, and network filesystems remain outside the
  guarantee.
- No importer, SQLite transaction, destination verification, activation,
  rollback, recovery CLI, schema, SDK, example, provider, or hosted behavior is
  implemented.
- Complete coverage proves current public writers; future writers still need an
  explicit conformance mechanism or equivalent direct test.

## 9. Recommended Next Phase

Perform a focused blocker-fix review. If accepted, proceed directly to the
accelerated Operational Embedded Durable State vertical slice described in the
[Roadmap Vertical-Slice Acceleration Plan](../implementation-plans/roadmap-vertical-slice-acceleration-plan.md).

## 10. Governed Fix Record

- workflow ID: `dg/blocker`
- run ID: `run-1785321391757083000-2`
- approval ID:
  `approval/run-1785321391757083000-2/fix-approved`
- approval outcome: granted with persisted presentation proof
- approval presentation ID: `presentation/9643e3cddbe67ca6`
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced
- out-of-kernel work: focused Rust test and documentation edits were performed
  by the delegated maintainer outside the kernel and are disclosed at phase
  close.
