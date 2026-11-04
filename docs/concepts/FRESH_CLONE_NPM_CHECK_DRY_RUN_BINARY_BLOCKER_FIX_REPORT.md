# Fresh-Clone npm Check Dry-Run Binary Blocker Fix Report

Status: accepted after focused review

## Problem

An external clean-room rerun found that `npm run check` was not self-contained
on a fresh clone. Six of 35 dogfood helper tests invoked dry-run commands with
`--no-build`. The helper nevertheless required `target/debug/workflow-os` to
exist and returned:

```text
error[dogfood.helper.binary_missing]: target/debug/workflow-os is missing; rerun without --no-build to build it
```

The same check passed only after an unrelated Rust test command happened to
create the debug binary. That made a named top-level validation result depend on
ambient build order.

## Root Cause

`ensureWorkflowOsBinary` checked binary existence and `--no-build` before its
dry-run branch. Dry-run cannot execute the kernel, create a run, persist an
approval, or mutate state, so binary existence was not a valid prerequisite.

This was a helper contract defect, not evidence that every npm validation must
compile Rust first.

## Fix

`ensureWorkflowOsBinary` now returns the intended binary path immediately for
dry-run commands. It does not check existence and does not invoke Cargo.

The live boundaries are unchanged:

- a live non-phase command with `--no-build` still fails when the binary is
  absent;
- a live material `phase-start` still rejects `--no-build`; and
- every live material `phase-start` still rebuilds the kernel from the current
  checkout before creating governance state.

A clean temporary-root regression test copies only the helper script, proves no
kernel binary exists, invokes `phase-start --dry-run --no-build`, and requires a
successful non-executing handoff with no build output.

## Governed Phase

- Workflow: `dg/blocker`
- Run: `run-1793758040212331000-2`
- Approval: `approval/run-1793758040212331000-2/fix-approved`
- Presentation: `presentation/95b0cbf57abd8084`
- Approval outcome: granted under delegated maintainer authority after the
  complete blocker-fix handoff was presented
- Phase status: completed
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- Approval presentation enforcement: proof enforced through
  `presentation/95b0cbf57abd8084`
- Approved scope: dry-run binary independence, preservation of live freshness
  and no-build failure, focused regression coverage, documentation, roadmap,
  and clean-clone validation
- Out-of-kernel work: disposable local clone reproduction, source inspection,
  repository edits, and deterministic validation commands

## Focused Review

The maintainer review accepted the fix without corrective changes.

- Workflow: `dg/review`
- Run: `run-1793758399060835000-2`
- Approval:
  `approval/run-1793758399060835000-2/review-scope-approved`
- Presentation: `presentation/2d1ee11f3158471b`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- Approval presentation enforcement: proof enforced through
  `presentation/2d1ee11f3158471b`
- Review findings: none

The review confirmed that all helper dry-run paths are non-executing command
previews, so binary independence is correct across the helper rather than only
for `phase-start`. It also confirmed that existing coverage still requires a
binary for live `--no-build` commands and that live material phases still
rebuild from the current checkout.

## Validation

The implementation passed:

- `npm run test:dogfood-helper`: 36 of 36 tests passed in the working
  repository;
- `npm run check`: passed in the working repository;
- clean temporary-clone `npm run check:dogfood`: 36 of 36 tests passed while
  `target/debug/workflow-os` was absent;
- clean temporary-clone `npm ci` under the repository-pinned Node 20 runtime:
  completed with zero vulnerabilities; and
- clean temporary-clone `npm run check` under the repository-pinned Node and
  Rust toolchains: passed from the top-level named gate.

A Node-only clean-clone replay first reached the later TypeScript contract
stage and failed because `cargo` was not on `PATH`. That was the expected
documented toolchain precondition, not the original ambient-binary defect. The
same replay passed after exposing the repository-pinned Rust toolchain. In both
replays, the dogfood helper stage began without a kernel binary and passed.

No runtime kernel semantics, version, package, tag, artifact, release, or
publication surface is changed by this fix.

## Non-Goals

This fix does not:

- let a live command proceed without a required binary;
- weaken current-checkout kernel freshness;
- build or execute the kernel during dry-run;
- change approval or workflow semantics;
- change release identity or packaging; or
- authorize any external mutation.
