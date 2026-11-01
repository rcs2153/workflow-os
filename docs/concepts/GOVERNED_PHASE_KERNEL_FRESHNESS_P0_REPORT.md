# Governed Phase Kernel Freshness P0 Report

## Status

Implemented and accepted by focused maintainer/security review. Terminal-green
GitHub acceptance remains required.

## Problem

The repo-local governed phase runner previously built `target/debug/workflow-os`
only when the binary was missing. A live material phase could therefore create
a run, approval, and phase state under older kernel behavior even though the
checkout contained newer source.

Binary timestamps and version strings cannot prove source equivalence. A binary
may have been built from an equivalent pre-merge branch, and a current binary
may have an unexpected timestamp. Rebuilding from the exact checkout is the
deterministic boundary.

## Implemented Boundary

- Every live material `phase-start` rebuilds the Workflow OS CLI before project
  validation or any governed run operation.
- Live material `phase-start --no-build` fails closed with
  `dogfood.helper.kernel_freshness_required`, even when a binary exists.
- A failed build invokes no kernel command and therefore creates no run,
  approval, approval-presentation proof, or phase state.
- Successful live starts disclose
  `kernel_freshness: rebuilt_from_current_checkout`.
- Dry-run starts disclose `kernel_freshness: not_applicable_dry_run` because
  they cannot create governed state.
- Existing behavior remains unchanged for non-phase helper commands.

## Focused Proof

The helper test suite proves:

- an existing binary does not bypass the live-phase build;
- `--no-build` cannot waive the live-phase freshness invariant;
- build failure leaves the fake kernel invocation log empty; and
- the successful path builds before validation and run creation.
- one shared fixture event log proves the complete ordering is build,
  validation, run creation, then approval-presentation persistence.

## Bootstrap Disclosure

The freshness defect affected the mechanism normally used to govern its own
fix. The narrow runner and focused-test changes therefore had to be prepared
before a trustworthy live phase could start.

The first governed start in the isolated worktree failed closed because Cargo
was not available on that worktree's default `PATH`. It created no run or
approval state. The second start used the repository's pinned Cargo toolchain,
rebuilt the kernel from the exact P0 checkout, then created and completed the
governed blocker run.

## Governance Record

- workflow: `dg/blocker`
- run: `run-1793522944903098000-2`
- approval: `approval/run-1793522944903098000-2/fix-approved`
- approval-presentation proof: `presentation/fc5c3636d0cb2bcf`
- approval outcome: granted under delegated maintainer authority
- phase status: completed

## Validation

- `npm run test:dogfood-helper`
- `npm run check:docs`
- `git diff --check`

The first focused test invocation in the clean worktree exposed six preexisting
dry-run fixture assumptions because `target/debug/workflow-os` had not yet been
built there. After the exact-checkout CLI build, all 35 helper tests passed.
This was a local test-bootstrap condition, not a governed-state bypass.

## Non-Goals

This P0 does not change kernel runtime semantics, public CLI behavior, schemas,
provider access, external writes, approval defaults, release posture, or the
paused GitHub read-only Phase 2 implementation.

## Next Step

After terminal-green acceptance and merge, rebuild from updated `main` and
resume the preserved Phase 2 implementation.
