# Trusted Host Opaque Wait Handoff Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to explicit reinvocation planning.**

The fix closes both blockers from the original focused review. Trusted-host
observation now opens only an existing SQLite database through a private
read-only connection, and the focused security matrix proves the accepted
identity-movement, non-handoff, missing-storage, corruption, restart, and
coherent-race boundaries.

The handoff remains private, inert, and non-authorizing. Acceptance permits a
planning phase for explicit reinvocation only. It does not authorize scheduler
behavior, executor reinvocation, or any public handoff surface.

## 2. Scope Verification

The blocker fix stayed within its approved boundary. It changed the private
SQLite observation connection, added test-only derivation access, completed
focused regression coverage, and updated phase documentation.

It did not add wait satisfaction, polling, timers, queues, scheduling,
automatic reinvocation, executor calls, public APIs, serde, schemas, CLI, SDK,
provider or sandbox execution, nested harnesses, writes, hosted behavior, or
release posture changes.

## 3. Existing Read-Only Observation Assessment

`observe_with_hook` no longer uses the create-capable general connection
helper. It calls `existing_read_only_connection`, which opens the configured
database with `SQLITE_OPEN_READ_ONLY | SQLITE_OPEN_URI` and applies only the
bounded busy timeout. It does not initialize schema or apply journal and
durability pragmas.

The observer still performs snapshot loading, trusted-time observation,
classification, and handoff derivation inside one deferred read transaction.
The correction therefore removes storage-creation authority without weakening
the coherent-read boundary.

## 4. Missing-Storage Assessment

The focused missing-database regression removes the database and related WAL
and SHM files before observation. Observation fails with the stable
`state.sqlite.open.failed` code, recreates none of those files, and does not
include the local database path in Debug or Display output.

This directly closes the original no-write evidence gap.

## 5. Identity-Movement Assessment

Separate focused tests prove that each accepted authoritative dimension
changes handoff identity:

- continuity revision;
- continuity cursor;
- active yield generation; and
- wait-condition identity and version.

Unchanged coherent state remains deterministic, and reopening the same state
preserves the same handoff identity. Splitting the dimensions into independent
tests gives each protected contract a precise failure signal.

## 6. Non-Handoff And Corruption Assessment

Focused tests prove that no handoff is produced for:

- closed, revoked, superseded, or validly expired windows;
- satisfied waits, which classify as `ResumeNow`;
- canceled, expired, or superseded waits, which classify as blocked; and
- unsupported dependency posture.

Locator mismatch and relational projection corruption fail closed with stable,
non-leaking errors. The corruption test modifies a persisted relational field
and confirms that projection validation rejects it before handoff construction.

## 7. Authority And Runtime Boundary

The fix does not increase handoff authority. No mutating method accepts the
handoff or its identifier, no conversion exists to wake or execution
capabilities, and the only next-operation vocabulary remains
`RequestFreshClassification`.

Future callers must still request a fresh authoritative classification before
any operation. This review does not authorize using the handoff as a wake,
dispatch, retry, approval, or execution grant.

## 8. Privacy And Error Assessment

The model remains non-serializable and crate-private. Custom Debug output omits
window, generation, condition, cursor, deadline, trusted-time, dependency, and
authority values. New tests verify that missing-storage paths and intentionally
corrupted workflow markers do not appear in errors.

No raw payload, provider data, command output, source content, credential, or
secret value is introduced or copied.

## 9. Test Quality Assessment

The focused `trusted_host_wait_` family contains 12 passing tests and now
protects:

- deterministic and restart-stable identity;
- revision, cursor, generation, and condition movement;
- terminal and non-actionable states;
- missing-storage non-creation;
- corrupt relational projection rejection;
- unsupported and mismatched binding rejection;
- no continuity-state mutation; and
- coherent old-or-new behavior under a concurrent transition.

The tests exercise behavior rather than construction and add no production
execution path.

## 10. Documentation Assessment

The plan, implementation report, blocker-fix report, original review
fix-forward note, and roadmap consistently describe the handoff as private,
inert, and non-authorizing. They continue to state that reinvocation and
scheduler behavior are not implemented.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Keep the first planned caller limited to the existing bounded `TimeWindow`
  shape.
- Preserve fresh authoritative classification as the sole route from handoff
  observation to any future operation.
- Consider compile-fail API-shape coverage only if another private consumer or
  a public representation is later proposed.

## 13. Recommended Next Phase

Plan one explicit reinvocation vertical slice from fresh authoritative
classification. The plan must define the trusted-host boundary, operation
idempotency, restart posture, wait revalidation, and fail-closed behavior before
any scheduler or executor integration is implemented.

Provider mutation broadening, OpenShell integration, nested harnesses, public
configuration, CLI, SDK, schemas, and hosted scheduling remain blocked.

## 14. Validation

- `cargo test -p workflow-core trusted_host_wait_`: 12 passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; explicit live opt-in tests remained ignored.
- `npm run check:docs`: passed before this review and rerun after authoring.
- `git diff --check`: passed before this review and rerun after authoring.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791066015810350000-2`
- approval:
  `approval/run-1791066015810350000-2/review-scope-approved`
- presentation: `presentation/bcfc5d56547c12eb`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: focused maintainer and security review only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation summary: focused tests, formatting, workspace clippy, workspace
  tests, documentation checks, and diff checks passed
- out-of-kernel work: source and test inspection, review authoring, validation,
  and later git and pull-request work
- missing coverage: the kernel coordinated governance only; it did not inspect
  source, write this review, execute checks, create a WorkReport artifact, or
  perform git actions
