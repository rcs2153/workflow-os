# Hacker News Launch Admission Phase 1 Review

Status: accepted pending terminal-green exact-head CI

## Scope Reviewed

This focused maintainer review covered:

- deterministic native archive construction and checksums;
- exact source-commit and checkout-cleanliness posture;
- release version synchronization;
- native host, architecture, binary format, and executable version checks;
- install, overwrite, uninstall, and state-preservation behavior;
- exact-head CI checkout behavior, including fork-origin pull requests;
- provisional-versus-final artifact language;
- focused tests and repository check integration; and
- launch-admission roadmap and report claims.

## Findings Corrected

### P1: release version verification covered too little of the workspace

The initial implementation compared only `workflow-cli` with the root npm
package. The release process requires aligned versions across the product.

The packager now fails closed unless all four Rust crates, the root package,
and the TypeScript SDK have one identical version.

### P1: exact-head CI checkout needed fork-repository binding

Binding `actions/checkout` only to the pull request head SHA could fail for a
head commit that exists only in a contributor fork. The rehearsal now binds
both the head repository and exact head SHA, with push-event fallbacks.

### P2: installer accepted trailing arguments

The initial generated installer considered the first argument but did not
reject extra arguments after `--force`. It now accepts zero arguments or the
single exact `--force` argument and rejects all other shapes. The focused test
covers this behavior.

## Accepted Posture

No P0 or unresolved P1 finding remains in the reviewed local diff.

The macOS arm64 rehearsal is real but provisional and non-promotable because
the manifest records a dirty checkout. The Linux x86-64 archive is deliberately
not claimed from the macOS cross-build attempt. Native Linux proof must come
from the new exact-head CI job, which uploads nothing.

The implementation does not publish artifacts, change runtime semantics,
activate providers, add a generic executor, or enter Phase 2 composition.

## Validation Reviewed

- `npm run check`: passed;
- `npm run test:release-package`: 4 passed;
- macOS release build and exact version check: passed;
- macOS archive checksum and layout: passed;
- real archive install, overwrite refusal, uninstall, and state preservation:
  passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

Merge remains blocked until the full existing CI matrix and the new `Release
Packaging Rehearsal` job pass on the unchanged PR head.

## Governed Review

- workflow: `dg/review`
- run: `run-1793745404029042000-2`
- approval: `approval/run-1793745404029042000-2/review-scope-approved`
- presentation proof: `presentation/ebf785091268a3fb`
- outcome: granted under delegated-maintainer authority

The kernel governed review scope and approval. Diff inspection, commands,
tests, and corrections were performed by the delegated maintainer outside the
kernel and are disclosed here.
