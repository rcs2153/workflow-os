# SQLite Authorized Execution Continuity Backend Plan Blocker Fix Review

## 1. Executive Verdict

**Needs additional planning blocker fixes.**

The first correction substantially improves the plan, but implementation
remains unauthorized. Four concrete issues remain in operation integrity,
relational integrity, ambiguity capability consumption, and restore/support
posture.

## 2. Improvements Accepted

- Arbitrary restore is no longer described as locally detectable.
- Committed security rejection is recognized as distinct from rollback.
- The plan provides a concrete first-pass DDL specification.
- Commit ambiguity has a named error and fresh-connection reconciliation.
- Conformance state deltas, replay validation, storage whitelist, and canary
  testing are materially clearer.
- Shared semantic amendment remains sequenced before SQLite implementation.

## 3. Remaining Blocker: Operation Integrity

`continuity_operations` lacks the accepted `operation_commitment` and a clearly
distinct rejection commitment. Persisted replay therefore cannot validate the
same complete committed operation and receipt required by the reference model.

Required fix: add exact operation and rejection commitments, define their
domain-separated derivation, and include both in replay and health validation.

## 4. Remaining Blocker: Relational Integrity

Child foreign keys validate referenced IDs individually but do not guarantee
that:

- a yield's attempt belongs to the same window;
- a wait or directive generation belongs to the same window; or
- a window's active yield belongs to that window.

Required fix: define composite candidate keys and composite foreign keys for
all ownership relationships, including an implementable cyclic active-yield
constraint. Define how `scope_commitment` and `scope_generation` are derived or
remove them from DDL until the model owns them.

## 5. Remaining Blocker: Capability Consumption

The current consume request borrows its authority capability, so a commit-
ambiguous caller still retains it. Prose cannot enforce fresh authority.

Required fix: amend the private API so consume takes a non-cloneable capability
by value or burns it before storage, and add the read-only reconciliation API
to the production-private store boundary. Confirmed absence still requires a
fresh authority assessment and a fresh operation.

## 6. Remaining Blocker: Restore And Support

The plan still lists fail-closed restore as a goal and permits unconditional
SQLite support advertisement despite excluding arbitrary restore and lacking
an eligibility signal.

Required fix: remove restore from this phase's goals, make the support
declaration explicitly local-live-state-only, and prohibit future executor
integration until an external epoch-anchor eligibility contract is accepted.

## 7. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786816075544285000-2`;
- approval: `approval/run-1786816075544285000-2/review-scope-approved`;
- presentation: `presentation/ba08784171ccd6f6`;
- approval outcome: granted by delegated maintainer after complete handoff;
- independent review: read-only focused security re-review; and
- out-of-kernel work: review and documentation were performed externally.

## 8. Validation

- `npm run check:docs`: passed before re-review; and
- `git diff --check`: passed before re-review.

## 9. Recommended Next Phase

Run one additional focused plan blocker-fix phase addressing only these four
items, then repeat focused re-review. Do not implement Rust or SQLite behavior
until accepted.
