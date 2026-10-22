# Trusted-Host Cargo Format Check Profile Blocker Fix Report

## 1. Executive Summary

The focused trusted-host Cargo format profile blockers are fixed. The profile
now commits the complete Core-owned toolchain input inventory used by the
direct invocation, commits the exact protected-tree exclusion policy, rejects
repository/toolchain overlap in either direction, and has Cargo-specific Core
and foreground regressions for failure, timeout, missing components,
toolchain mutation, root boundaries, exclusion closure, and privacy.

The profile remains unpublished, one-step, local, and trusted-host only. This
fix does not add another command, topology, provider, external write, public
surface, schema, hosted behavior, nested harness, or release change.

## 2. Blockers Fixed

1. The resolved execution identity now binds a bounded toolchain input
   inventory rather than only three anonymous executable content hashes.
2. The protected-tree commitment now includes the exact Core-owned exclusion
   policy identity.
3. The missing Cargo-specific deterministic failure, timeout, missing
   component, mutation, root-boundary, exclusion-policy, foreground failure,
   and privacy regressions are present.

## 3. Implementation Approach

The required toolchain inventory is intentionally closed to the direct
invocation inputs: the canonical toolchain root and the logical selector,
root-relative path, and content hash for each final `cargo`, `cargo-fmt`, and
`rustfmt` executable. Core hashes the inventory policy and those inputs before
execution, recomputes them before process entry and after process return, and
fails closed on any mismatch.

The handler does not claim that every file under a broad toolchain directory
is an invocation input. It proves that the exact final executables plus their
canonical root and selectors are the complete supported inventory. Ambient
lookup, proxies outside that root, install/update behavior, and additional
toolchain inputs remain unsupported.

## 4. Identity And Filesystem Boundary

The repository root and toolchain root must be disjoint in both directions.
The protected repository commitment includes a hash of the exact Core-owned
exclusions: `.git`, `.workflow-os`, `dogfood/.workflow-os`, and `target`.
Callers cannot add, remove, or replace exclusions.

The implementation still proves final-state equivalence rather than operating-
system write prevention. A transient write restored before the post-execution
commitment is not observable. Network-disabled posture remains a contract
constraint rather than host egress isolation.

## 5. Error And Privacy Boundary

Inventory and overlap failures use stable non-leaking error codes. Debug
output redacts executable paths, roots, inventory hashes, protected-state
commitments, exclusion-policy identity, and runner details. The foreground
failure regression proves raw formatter output and fixture paths do not enter
stdout, stderr, durable events, or the persisted WorkReport artifact.

## 6. Test Coverage

Focused coverage now proves:

- non-zero exit maps to a failed result;
- timeout maps to a timed-out result;
- a missing formatter component fails before process entry without install;
- persistent mutation of a bound toolchain input fails closed;
- repository/toolchain overlap is rejected in both directions;
- valid disjoint roots execute through Core and the foreground host;
- Core-owned exclusions cannot be caller-authored and excluded runtime output
  does not change the source commitment;
- failed foreground Cargo execution terminalizes the run, persists one failed
  report artifact, and reuses bounded result references; and
- paths and raw process output remain absent from durable and serialized
  surfaces.

## 7. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test local_check`: passed, 82 active tests;
  two opt-in live smokes skipped by design.
- opt-in real Cargo format smoke against the bundled stable toolchain: passed.
- `cargo test -p workflow-local-host --test foreground_admission`: passed,
  29 tests.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Limitations

- No operating-system process, filesystem, or network sandbox is added.
- Transient restored writes remain undetectable.
- The exact executable inventory is self-committed, not independently
  attested by a third party.
- The profile remains unpublished and outside the accepted two-step workflow.

## 9. Governed Fix Record

- Workflow ID: `dg/blocker`
- Run ID: `run-1792677234108104000-2`
- Approval ID: `approval/run-1792677234108104000-2/fix-approved`
- Presentation ID: `presentation/59827a22279d3b9a`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted blocker-fix scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations, one
  persisted presentation record, and a present approval-event proof marker
- Out-of-kernel work: source edits, tests, documentation, process execution,
  and git operations are performed by the delegated maintainer. The kernel
  governs scope and approval but does not perform those actions.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Do not add
clippy, tests, arbitrary Cargo commands, provider access, external writes,
public surfaces, hosted expansion, another topology, or release changes before
that review.
