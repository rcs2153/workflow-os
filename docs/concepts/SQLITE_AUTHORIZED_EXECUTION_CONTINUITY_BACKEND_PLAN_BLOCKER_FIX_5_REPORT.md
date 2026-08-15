# SQLite Authorized Execution Continuity Backend Plan Blocker Fix 5 Report

## 1. Executive Summary

The fifth focused correction closes the cross-link defect demonstrated by
adversarial review. The operation row now stores canonical relational request
identity and requires a committed success to target that exact identity through
ordinary domain foreign keys.

No implementation is authorized until one narrow acceptance re-review passes.

## 2. Exact Request/Target Identity

Each operation kind has exactly one relational request target:

- register yield: yield generation;
- transition wait: wait condition and version; or
- consume directive, record outcome, and ambiguity recovery: attempt.

A success must copy the corresponding request identity into its success target.
DDL checks enforce equality and reject every other target category. Foreign
keys require that exact success target to exist. A rejection has no success
target.

For consume, the attempt row independently points back to the same successful
consume operation, closing the cycle and preventing cross-linked attempts.

`request_json` now contains only the remaining bounded request material. The
explicit relational columns plus that envelope are the request-commitment
source of truth.

## 3. Scope Preserved

No Rust, SQLite, executor, scheduler, event, approval, provider, Postgres, CLI,
workflow schema, nested harness, external write, or release behavior changed.

## 4. Governed Record

- workflow: `dg/blocker`;
- run: `run-1786818328327736000-2`;
- approval: `approval/run-1786818328327736000-2/fix-approved`;
- presentation: `presentation/88790b6b5b206cf3`;
- approval outcome: granted under standing delegated-maintainer authority after
  the complete handoff was presented; and
- out-of-kernel work: documentation analysis and edits were performed by the
  external executor; the kernel recorded governance only.

## 5. Validation

Results:

- `npm run check:docs`: passed;
- canonical SQLite DDL parse and `foreign_key_check`: passed;
- rejected-consume attempt probe: failed closed;
- missing-target probe: failed closed;
- wrong/cross-linked target probe: failed closed through request/target equality;
  and
- `git diff --check`: passed.

## 6. Recommended Next Phase

Perform one narrow acceptance re-review. If accepted, begin only implementation
sequence step 1: the shared semantic V2 amendment.
