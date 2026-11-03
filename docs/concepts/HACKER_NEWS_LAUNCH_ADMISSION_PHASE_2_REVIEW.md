# Hacker News Launch Admission Phase 2 Review

Status: accepted pending terminal-green exact-head CI

## Scope Reviewed

This focused maintainer review covered:

- the exact selected first-run report-artifact operation;
- authoritative project-validation evidence preservation;
- additive Core API shape and route-truth preservation;
- initial execution, terminal replay, and missing-artifact recovery parity;
- fail-before-mutation ordering for reference metadata;
- bounded human and JSON `inspect` projections;
- default onboarding and mock-demo separation;
- nonterminal-only `next-action` behavior;
- focused and full validation evidence; and
- launch-admission plan, roadmap, and implementation claims.

## Finding Corrected

### P1: reference metadata validation could occur after route mutation

The first additive Core result derived its typed check reference after the
authoritative route returned. Although the CLI supplies bounded empty metadata,
the public function accepted general reference metadata. Invalid output or
redaction metadata could therefore have produced a top-level error after a run
had already been created or advanced.

The reviewed implementation now validates every caller-controlled
reference-only metadata field before entering the authoritative route. The
existing `LocalCheckResultReference` validator and the new route consumer share
the same internal validation helper. Exact result identity, command identity,
status, workflow, and run bindings are still derived from the same-call check
and immutable declaration after routing.

This preserves both requirements:

- caller metadata fails before mutation; and
- authoritative provenance is never caller-authored or reconstructed by the
  CLI.

## Accepted Posture

No P0 or unresolved P1 finding remains in the reviewed local diff.

The selected operation is a real local governed operation, not a mock handler
or arbitrary command path. It executes the closed project-validation profile,
binds bounded first-run analysis into immutable context, and persists a
terminal report with one typed local-check citation. `inspect` exposes only
bounded counts; it does not copy command output, report payloads, repository
source, or evidence bodies.

Recovery reruns validation, requires unchanged resolved context, and remains
idempotent. Existing complete artifacts replay without duplicate execution.
Terminal `next-action` remains rejected without event mutation.

The default UX now leads to the real report-bearing operation. The mock
first-run workflow remains visible only as an optional approval/audit demo.

## Validation Reviewed

- `cargo test --workspace --all-features`: passed;
- `cargo test -p workflow-cli --test cli first_run_`: 22 passed;
- focused selected-project-validation local-executor tests: 10 passed;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed;
- warnings-denied workspace rustdoc: passed;
- `npm run check`: passed;
- `cargo fmt --all -- --check`: passed; and
- `git diff --check`: passed.

Exact-head CI remains the merge gate. Final clean native archives must be
rebuilt from the merged post-composition commit before Phase 3 clean-room use.

## Governed Review

- workflow: `dg/review`
- run: `run-1793748129140948000-2`
- approval: `approval/run-1793748129140948000-2/review-scope-approved`
- presentation proof: `presentation/479e69c2cce35712`
- outcome: granted under delegated-maintainer authority
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record

The kernel governed review scope and approval. Diff inspection, commands,
tests, correction, and review writing were performed outside the kernel and
are disclosed here.
