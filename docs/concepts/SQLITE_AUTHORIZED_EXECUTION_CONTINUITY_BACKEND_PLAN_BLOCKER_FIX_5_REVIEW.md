# SQLite Authorized Execution Continuity Backend Plan Blocker Fix 5 Review

## 1. Executive Verdict

**Needs additional planning blocker fixes.**

The fifth correction rejects direct request/success mismatches, wrong target
categories, nonexistent target identifiers, and rejected consume ownership.
Independent adversarial review nevertheless reproduced a commit-valid
consume/attempt cross-link and a success with no target. Implementation remains
blocked.

## 2. Accepted Corrections

- Canonical DDL parses with foreign keys enabled.
- Direct yield, wait, and attempt request/success mismatches fail closed.
- A success naming a nonexistent non-null target fails closed.
- A committed security rejection carrying a success target fails closed.
- An attempt cannot reference a security-rejected consume operation.
- Historical trusted-time, receipt, replay, eligibility, reconciliation,
  ambiguity-authority, restore-scope, and implementation-sequencing corrections
  remain acceptable.

## 3. Blocker: Consume And Attempt Pair Identity

The attempt row requires its named consume operation to be a successful
`consume_directive`. The operation row independently requires its named target
attempt to exist. Those two relationships do not require the same pair.

Adversarial review committed this cycle:

```text
attempt-a -> consume-a
attempt-b -> consume-b
consume-a request/success -> attempt-b
consume-b request/success -> attempt-a
```

The transaction committed and `PRAGMA foreign_key_check` returned no rows. The
same construction worked across windows and runs. The plan's claim that the
current reverse relationship closes the cycle is therefore false.

Required correction: successful consume ownership must relationally enforce
that `(success_attempt_id, operation_id)` equals the target attempt's
`(attempt_id, consume_operation_id)` pair. A dedicated consume-result relation
or an equivalent exact composite foreign key is acceptable. Cross-window and
cross-run variants must fail closed in executable probes.

## 4. Blocker: Null Success Targets

SQLite treats a `CHECK` expression that evaluates to `NULL` as satisfied. The
current equality branches do not explicitly require applicable success-target
columns to be non-null. A `committed_success` operation can therefore omit its
yield, wait, or attempt success target and still commit.

Required correction: each successful operation-kind branch must require its
applicable success-target columns with explicit `IS NOT NULL` predicates while
requiring every inapplicable target column to be null. Add null-target probes
for all five operation kinds.

## 5. Blocker: Rejected Request Identifier Bounds

The relational request-target identifier columns do not carry the same bounded
length checks as their domain identifiers. A committed security rejection has
no success-target foreign key, so an oversized request identifier can be
persisted despite the bounded-storage contract.

Required correction: apply the canonical identifier bounds to every nullable
request and success identifier column when present. Add committed-rejection
probes with oversized identifiers.

## 6. Validation Evidence

The review ran:

- `npm run check:docs`: passed;
- `git diff --check`: passed;
- canonical DDL parse and `PRAGMA foreign_key_check`: passed;
- direct cross-target equality probe: failed closed;
- nonexistent non-null target probe: failed closed;
- rejection-with-success-target probe: failed closed;
- two-operation consume/attempt cross-link probe: **committed unexpectedly**;
- null success-target probe: **committed unexpectedly**; and
- independent full-history security review: found the identifier-bound blocker.

## 7. Scope Verification

The review changed documentation only. It did not implement Rust or SQLite
code, runtime integration, scheduling, automatic resume, provider mutation,
tool execution, schema exposure, CLI behavior, hosted behavior, or release
posture changes.

## 8. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786818560622527000-2`;
- approval: `approval/run-1786818560622527000-2/review-scope-approved`;
- presentation: `presentation/ae55dda232fb1ebd`;
- approval outcome: granted under standing delegated-maintainer authority after
  the complete proof-enforced handoff was presented; and
- out-of-kernel work: documentation review, independent adversarial review, and
  SQLite probes were performed by external executors; the kernel recorded
  governance only.

## 9. Recommended Next Phase

Perform one focused sixth correction covering exact consume/attempt pair
identity, explicit non-null success targets, and bounded relational request
identifiers. Then repeat the adversarial acceptance review. Do not begin the
shared semantic V2 amendment or SQLite implementation before acceptance.
