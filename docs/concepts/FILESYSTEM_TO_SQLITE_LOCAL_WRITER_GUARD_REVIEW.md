# Filesystem-To-SQLite Local Writer Guard Review

## 1. Executive Verdict

**Needs blocker fixes.**

The operating-system guard, marker protocol, stable errors, read-only
exclusive inspection, and reviewed mutation wrappers are coherent and
appropriately narrow. The phase cannot yet be accepted because the direct
contention tests do not prove every public canonical mutation entry point that
the migration guarantee claims to exclude.

## 2. Scope Verification

The implementation stayed within the approved guard-only scope:

- local shared-writer and exclusive-migration advisory guards;
- path-independent guard identity and protocol marker;
- `LocalStateBackend` mutation integration;
- canonical immutable run-bundle companion integration;
- read-only exclusive inspection;
- separate-process contention and release tests;
- stable, non-leaking errors;
- documentation and report updates.

No importer, SQLite destination write, verification, activation, migration CLI,
schema, SDK, example, provider, hosted, or release behavior was added.

## 3. Guard Boundary Assessment

`LocalStateBackend` derives a coordination identity from the canonical parent
and source-root leaf, hashes that identity, and stores the advisory lock and
protocol marker outside the source root. The marker contains only bounded
backend and protocol vocabulary.

Ordinary mutation acquires a non-blocking shared lock. Migration inspection
acquires a non-blocking exclusive lock. Contention and unavailable-lock
outcomes map to stable mode-specific errors. The marker is validated again
after acquisition, so changed or malformed protocol metadata fails closed.

The boundary is correctly documented as cooperating local writers only. It
does not claim process discovery, hostile-writer control, older-binary
exclusion, distributed locking, or network-filesystem guarantees.

## 4. Mutation Coverage Assessment

Code inspection found shared acquisition around:

- layout creation through `LocalStateBackend::new`;
- event append;
- snapshot save;
- idempotency record;
- logical-lock acquire and release;
- approval save and delete;
- project-state save;
- policy-audit append;
- adapter audit and observability append;
- WorkReport artifact write;
- SideEffect write and update;
- approval-presentation write;
- health-check write probe.

Read paths no longer create the state layout. Internal lock and compound-write
helpers are unguarded and private so one public operation can hold one shared
guard without nested acquisition.

The canonical `state_root/immutable-run-bundles` store participates in the
same root guard. Its definition, local-check declaration, manifest, complete
bundle, and governance-assessment write paths are wrapped. Standalone bundle
stores outside that canonical layout remain outside the migration guarantee,
as documented.

No unguarded public mutation path was identified during source inspection.

## 5. Concurrency And Release Assessment

Separate-process tests prove:

- a shared writer blocks exclusive migration acquisition;
- an exclusive migration guard blocks a representative state mutation;
- process death releases shared and exclusive locks;
- mutation resumes after exclusive release.

Same-process tests prove the exclusive guard rejects a representative matrix
of public state mutations. Protocol incompatibility and path/secret
non-leakage are also covered.

The advisory lock lifetime is bound to the file descriptor and explicitly
unlocked on drop. Process termination provides the required local release
posture.

## 6. Atomic Publication Assessment

Repeated concurrent tests exposed a pre-existing temporary-file collision:
create-only atomic writes used process ID and timestamp but omitted a
process-local sequence. Multiple publications in the same coordination
directory could therefore select the same temporary path.

Adding an atomic sequence to create-only temporary names is a narrow and
justified correctness fix. Repeated focused state tests and the full workspace
suite pass after the correction.

## 7. Privacy And Error Assessment

- coordination filenames contain a hash rather than the source path;
- marker content contains no path or payload;
- guard Debug output redacts the backend;
- contention, unavailable, identity, and protocol errors are stable and do not
  echo paths, marker payloads, or secret-like values;
- exclusive inspection exposes existing bounded inspection models only.

No raw state, provider payload, command output, environment value, credential,
authorization header, token, or private key is added to guard state.

## 8. Test Quality Assessment

The focused tests are strong for protocol behavior, separate-process
contention, process-death release, representative state mutations, canonical
bundle integration, and redaction.

The blocker is incomplete direct proof for the claimed public mutation
boundary. The current contention matrix does not directly exercise:

- `LocalStateBackend::new` layout creation;
- approval-request save;
- logical-lock release;
- adapter audit append;
- adapter observability append;
- WorkReport artifact write;
- SideEffect lifecycle update;
- individual canonical immutable run-bundle public writers, especially the
  governance-assessment binding path.

The implementation appears to guard these methods, but migration quiescence is
a load-bearing invariant. A future refactor could remove one wrapper while the
current representative tests remain green.

## 9. Validation Evidence

The following passed:

- `cargo check -p workflow-core --offline`;
- `cargo fmt --all --check`;
- focused and workspace clippy with warnings denied;
- repeated 17-test local-state guard suite;
- 16-test immutable run-bundle store suite;
- `cargo test --workspace --offline`;
- `npm run check` under pinned Node 20;
- `npm run check:integrations` under pinned Node 20;
- `npm run check:docs`;
- `cargo audit`;
- `npm audit --audit-level=moderate`;
- `git diff --check`.

Opt-in live-provider tests remained ignored as designed.

## 10. Blockers

Add a focused contention assertion for every omitted public canonical mutation
entry point listed in section 8. Each assertion must prove:

- exclusive migration causes
  `state.local.writer_guard.contended`;
- the operation does not mutate source state;
- error text does not expose bounded secret-like fixture values or source
  paths;
- the same operation can proceed after guard release where a valid setup makes
  that meaningful.

## 11. Non-Blocking Follow-Ups

- Keep older-writer shutdown as an explicit operator assertion until a
  separately approved process protocol exists.
- Do not extend the advisory guarantee to network filesystems without
  platform-specific validation.
- Consider a future conformance helper for guard-participating companion stores
  so new public writers cannot be added without an explicit guard test.
- Preserve the distinction between canonical state-root bundle storage and
  independent standalone bundle stores.

## 12. Recommended Next Phase

Perform a narrow local-writer-guard blocker fix that adds complete public
mutation contention coverage only. Do not redesign the guard or begin import,
SQLite, verification, activation, or CLI work.

After the blocker-fix review accepts complete coverage, proceed to the
accelerated Operational Embedded Durable State vertical slice.

## 13. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1785321161177220000-2`
- approval ID:
  `approval/run-1785321161177220000-2/review-scope-approved`
- approval outcome: granted with persisted presentation proof
- approval presentation ID: `presentation/1c171818265973e2`
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced
- out-of-kernel work: source/test/doc inspection, validation evidence review,
  and review authoring were performed by the delegated maintainer outside the
  kernel.

## 14. Fix-Forward Status

The original blocker finding above remains the historical review verdict. A
narrow fix subsequently added direct contention and no-mutation proof for every
omitted public writer named in section 8:

- state-layout creation, approval save, logical-lock release, and SideEffect
  update;
- adapter audit and observability append;
- WorkReport artifact publication;
- individual canonical immutable run-bundle definition, check-declaration,
  manifest, complete-bundle, and governance-assessment binding publication.

The fix is documented in
[Filesystem-To-SQLite Local Writer Guard Blocker Fix Report](FILESYSTEM_TO_SQLITE_LOCAL_WRITER_GUARD_BLOCKER_FIX_REPORT.md).
Acceptance remains subject to a focused blocker-fix review; this note does not
erase or pre-empt that review.
