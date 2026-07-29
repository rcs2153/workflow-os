# Filesystem-To-SQLite Local Writer Guard Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to accelerated Operational Embedded Durable State
build.**

The narrow fix supplies direct contention and no-mutation proof for every
public canonical writer omitted by the original review. No unguarded public
writer was found, the guard design was not widened, and broad regression and
dependency validation are green.

## 2. Scope Verification

The fix stayed within the approved test-coverage and documentation boundary:

- expanded direct contention coverage for local state mutation entry points;
- added direct adapter telemetry, WorkReport artifact, immutable run-bundle,
  and governance-assessment binding proof;
- preserved the original review verdict with a fix-forward note;
- added a dedicated blocker-fix report and roadmap truth update.

It did not add or redesign guard acquisition, import filesystem state, create
or write SQLite, verify or activate a destination, expose migration CLI
behavior, change schemas, update examples, add provider mutations, introduce
hosted behavior, or change release posture.

## 3. Original Blocker Restatement

Source inspection found shared guard acquisition around every public canonical
mutation path, but direct tests covered only representative families. That was
insufficient for a migration quiescence invariant because a future refactor
could remove one public wrapper without failing the representative matrix.

Acceptance required each omitted public writer to prove:

- exclusive migration yields `state.local.writer_guard.contended`;
- no source mutation occurs;
- errors do not leak paths or fixture values;
- valid work can resume after guard release where meaningful.

## 4. State Writer Coverage Assessment

The expanded state tests directly cover:

- `LocalStateBackend::new` layout creation, proving no state directory appears
  while exclusivity is held;
- pending approval save;
- logical-lock release;
- SideEffect lifecycle update.

The existing matrix continues to cover event append, snapshot save,
idempotency record, lock acquisition, approval deletion, project state, policy
audit, SideEffect creation, approval-presentation proof, and health-check
mutation.

Splitting the matrix into run/lock and governance/projection groups satisfies
the repository's reviewability lint without weakening the assertions.

## 5. Companion Writer Coverage Assessment

Dedicated integration tests now directly cover:

- adapter audit append;
- adapter observability append;
- WorkReport artifact publication;
- immutable run-bundle definition publication;
- canonical local-check declaration-set publication;
- immutable run-bundle manifest publication;
- complete immutable run-bundle publication;
- governance-assessment binding publication.

Each test holds the same root-wide exclusive guard used by migration
inspection. Records and store roots remain absent under contention, and valid
publication succeeds after release.

Standalone immutable run-bundle stores outside
`state_root/immutable-run-bundles` remain intentionally outside the migration
quiescence claim.

## 6. Error And Privacy Assessment

All blocked operations return the stable
`state.local.writer_guard.contended` code. Tests assert that errors do not
expose source paths or bounded secret-like actor fixtures.

No raw state, provider payload, command output, parser payload, environment
value, credential, token, authorization header, or private key is introduced.
The fix does not change guard serialization, protocol markers, Debug output, or
coordination identity.

## 7. Regression Assessment

Unchanged behavior is demonstrated for:

- local state contracts and event rehydration;
- adapter audit and observability persistence;
- WorkReport artifact persistence;
- SideEffect persistence and lifecycle updates;
- immutable run-bundle and proportional-governance binding persistence;
- executor, validation, report, approval, and integration-contract suites.

Valid writes continue after exclusive guard release. Process-death and
separate-process exclusion tests remain green.

## 8. Test Quality Assessment

The blocker requested entrypoint-complete direct proof, and the test set now
provides it. Assertions cover both rejection and absence of durable mutation;
they do not merely inspect wrapper source or accept any error.

One non-blocking maintainability concern remains: future companion writer types
could be added without joining a centralized conformance list. A later
conformance helper may reduce that risk, but it is not required to accept the
current closed set of public writers.

## 9. Validation Evidence

The following passed:

- `cargo fmt --all --check`;
- workspace clippy with warnings denied;
- focused state and companion-writer suites;
- `cargo test --workspace --offline --quiet`;
- `npm run check` under pinned Node 20;
- `npm run check:integrations` under pinned Node 20;
- `npm run check:docs`;
- `cargo audit`;
- `npm audit --audit-level=moderate`;
- `git diff --check`.

Opt-in live-provider tests remained ignored as designed.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Consider a centralized conformance helper when another canonical companion
  writer family is introduced.
- Keep older-writer shutdown as an explicit operator assertion.
- Do not extend the local advisory guarantee to network filesystems without
  platform-specific proof.
- Preserve the distinction between canonical state-root storage and standalone
  stores.

## 12. Recommended Next Phase

Proceed to **Build A: Operational Embedded Durable State** under the
[Roadmap Vertical-Slice Acceleration Plan](../implementation-plans/roadmap-vertical-slice-acceleration-plan.md).

That governed vertical slice should compose the accepted guard with atomic
filesystem-to-SQLite staging import, projection rebuild, destination
verification, explicit activation and rollback posture, a bounded operator
entry point, recovery behavior, and failure-injection tests. It must not weaken
the accepted writer-quiescence boundary.

## 13. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1785325292771806000-2`
- approval ID:
  `approval/run-1785325292771806000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer with persisted presentation
  proof
- approval presentation ID: `presentation/bbfa1605d2c08e0e`
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced
- out-of-kernel work: source and test inspection, review authoring, validation
  evidence assessment, and later git/PR operations are performed by the
  delegated maintainer outside the kernel and disclosed at phase close.
