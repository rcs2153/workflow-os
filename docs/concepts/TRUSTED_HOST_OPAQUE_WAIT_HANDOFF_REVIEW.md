# Trusted Host Opaque Wait Handoff Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation preserves the central architecture: the handoff is private,
deterministic, redaction-safe, and structurally non-authorizing. It is derived
from one explicit SQLite transaction and cannot satisfy a wait, consume a
directive, reserve dispatch, or invoke an executor.

Two acceptance gaps block reinvocation planning. The observation path opens the
database through a create-capable connection helper despite claiming to be
observation-only, and the focused suite does not yet prove the complete
state-movement and non-handoff matrix required by the accepted plan review.

## 2. Findings

### Blocker 1: Observation uses a create-capable SQLite open path

`observe_with_hook` calls `SqliteStateBackend::connection`. That helper uses
`rusqlite::Connection::open`, which may create a missing database, before
configuring write-affecting durability pragmas. A missing or replaced database
must fail closed without creating a new file when the operation is presented as
an inert observation.

The current no-write test compares decoded continuity snapshots before and
after a successful observation. It does not prove that a missing backing file
remains absent or that the observation path cannot initialize storage.

Required fix:

- open only an already-existing database for handoff observation;
- preserve the coherent read transaction;
- add a regression test that removes or points at a missing database and proves
  observation fails without creating the database or sidecar files; and
- keep the resulting error stable and value-free.

### Blocker 2: Required state-movement evidence is incomplete

The accepted plan review made implementation acceptance conditional on proof
that relevant revision, cursor, generation, and condition movement changes or
invalidates handoff identity, and that satisfied, canceled, expired,
superseded, terminal, unsupported, and corrupt posture cannot produce a
handoff.

The current five focused tests prove unchanged determinism, restart stability,
one blocked posture, unsupported dependency and locator mismatch failure, and
one old-snapshot race. They do not directly prove the rest of the accepted
matrix. Existing continuity-semantic tests reduce implementation risk but do
not prove this new projection boundary cannot regress independently.

Required fix:

- add focused handoff tests for relevant identity movement;
- add focused no-handoff tests for terminal and non-actionable wait states;
- add corrupt-row or projection-mismatch failure coverage; and
- keep the tests private, deterministic, and free of new runtime behavior.

## 3. Scope Verification

The phase stayed within its approved implementation boundary. It did not add
wait satisfaction, polling, scheduling, automatic reinvocation, provider or
sandbox execution, nested harnesses, public APIs, serde, schemas, CLI, SDK,
hosted behavior, writes, or release posture changes.

The module is crate-private and currently disconnected from public runtime
surfaces. The `dead_code` allowance is consistent with the model-first slice
and does not itself authorize a caller.

## 4. Model And Identity Assessment

The model is appropriately minimal:

- `TrustedHostWaitHandoffId` is a dedicated private newtype;
- the cursor is represented by a domain-separated commitment;
- condition posture is closed to `TimeWindow` and `Unsatisfied`;
- the only next operation is `RequestFreshClassification`; and
- custom Debug output exposes posture and counts, not selectors.

The handoff commitment uses length-framed, domain-separated fields with a fixed
condition-field arity. Repeated coherent reads and restart over unchanged state
derive the same identity. No public accessor or serde surface was introduced.

## 5. Coherent Read Assessment

The implementation opens one deferred SQLite transaction, loads and
projection-validates the continuity snapshot, obtains trusted time, runs the
existing authoritative classifier, derives the handoff, and ends the same
transaction. It does not classify and then reload rows independently.

The concurrent transition test demonstrates a coherent old snapshot followed
by a fresh `ResumeNow` observation. This supports the no-mixed-projection claim.
The create-capable connection blocker is separate from snapshot coherence.

## 6. Authority And Security Assessment

The handoff grants no authority:

- no mutating method accepts the handoff or its ID;
- no conversion exists to wake, attempt, directive, dispatch, or capability
  values;
- the source-specific wait transition remains independent;
- blocked posture returns no handoff; and
- unsupported dependency posture fails closed.

The internal exact selectors support correlation only. Their privacy is
protected by crate visibility and custom Debug behavior. Fresh Core
classification remains mandatory for any future operation.

## 7. Privacy And Error Assessment

Debug output omits window, generation, condition, cursor, deadline, trusted
time, dependency, authority, command, payload, and secret values. Stable error
codes and messages disclose only bounded failure classes. The focused mismatch
test confirms a caller-supplied private marker is absent from error output.

No serialization path exists. The implementation does not copy provider data,
logs, source content, prompts, credentials, or execution output.

## 8. Test Quality Assessment

Existing focused coverage is meaningful but incomplete. It proves:

- deterministic unchanged identity;
- restart stability;
- exactly one condition and one closed next operation;
- decoded continuity-state non-mutation;
- no handoff for one blocked posture;
- unsupported dependency and locator mismatch failure;
- redaction-safe Debug and errors; and
- coherent old-or-new behavior across one transition race.

Missing blocker coverage is listed in Finding 2. The full workspace suite,
formatting, clippy, documentation checks, and diff checks pass.

## 9. Documentation Assessment

The plan, report, and roadmap accurately state that the handoff is private,
inert, non-authorizing, and not a scheduler or reinvocation path. The
implementation report's no-write claim must be qualified or restored by the
connection-boundary blocker fix.

## 10. Blockers

1. Replace the create-capable observation connection path and prove a missing
   database is not created.
2. Complete the accepted state-movement, non-handoff, and corruption-focused
   handoff test matrix.

## 11. Non-Blocking Follow-Ups

- Keep the first production caller limited to the existing bounded
  `TimeWindow` shape.
- Consider a compile-fail API-shape test if the handoff later gains another
  private consumer.
- Keep public serialization deferred until a separately reviewed transport
  boundary exists.

## 12. Recommended Next Phase

Perform one narrow blocker-fix phase covering the non-creating SQLite
observation connection and the missing focused security tests. Then repeat this
focused maintainer/security review.

Do not begin explicit reinvocation planning until the blocker-fix review
accepts the handoff boundary.

## 12.1 Fix-Forward Status

The two blockers are addressed in the [blocker-fix
report](TRUSTED_HOST_OPAQUE_WAIT_HANDOFF_BLOCKER_FIX_REPORT.md). Observation
uses an existing read-only connection and the required focused security matrix
is implemented. This note does not erase the original findings. Focused
[blocker-fix review](TRUSTED_HOST_OPAQUE_WAIT_HANDOFF_BLOCKER_FIX_REVIEW.md)
accepted the corrected boundary before reinvocation planning.

## 13. Validation

- `cargo test -p workflow-core trusted_host_wait_`: 5 passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; explicit live opt-in tests remained ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1791064612444649000-2`
- approval:
  `approval/run-1791064612444649000-2/review-scope-approved`
- presentation: `presentation/9d9da2a14308991c`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: focused maintainer and security review only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation summary: focused handoff tests, formatting, workspace clippy and
  tests, documentation checks, and diff checks passed
- out-of-kernel work: source inspection, review authoring, validation, and later
  git and pull-request work
- missing coverage: the kernel coordinated governance only; it did not inspect
  source, write the review, execute checks, create a WorkReport artifact, or
  perform git actions
