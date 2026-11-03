# Provider Fixture Next-Action Routing P1 Review

Status: Accepted with no remaining blocker.

## Review Scope

This focused maintainer review examined fixture-run state and immutable-bundle
source selection, ordinary local fallback, provider disclosure, waiting,
running, and terminal lifecycle behavior, non-mutation evidence, roadmap
status, and the implementation report.

The review was governed by:

- Workflow: `dg/review`
- Run: `run-1793723630397157000-2`
- Approval: `approval/run-1793723630397157000-2/review-scope-approved`
- Approval presentation: `presentation/d1188dc69d059da9`
- Approval outcome: granted under delegated-maintainer authority

## Findings

No blocking finding remains.

The routing boundary matches the established fixture detection used by
`status` and `inspect`: an existing fixture database is selected only when the
exact run has an event stream there. The fixture SQLite backend then supplies
both durable run state and the exact immutable bundle. This avoids mixing a
fixture run with the generic filesystem bundle store. Runs absent from the
fixture backend preserve the existing local backend and bundle-store path.

Waiting and completed fixture runs now fail with the intentional
`executor.governed_continuation.run.not_running` diagnostic rather than the
misleading `runtime.events.empty` error. An approved running fixture produces
the same bounded, non-authoritative continuation preview as an eligible local
run and discloses fixture posture in human and JSON output.

## Read-Only Assurance

Tests compare complete provider-aware `inspect` output immediately before and
after waiting, running, and terminal `next-action` calls. The output is
byte-identical. The command still performs no authority claim, event append,
state transition, handler invocation, provider call, artifact write, or retry.

## Validation

- Focused public GitHub fixture lifecycle test: passed.
- Full GitHub read-only example suite: passed, 12 tests.
- Full `workflow-cli` integration suite: passed, 176 tests.
- Strict `workflow-cli` Clippy with warnings denied: passed.
- Rust formatting check: passed.
- Documentation check: passed.
- Diff hygiene check: passed.

## Verdict

Accepted. The provider fixture now participates in the public read-only
continuation preview without creating a parallel continuation model or
broadening runtime authority.

The authoritative post-compaction preemption and its two fresh-rerun P1
acceptance fixes are complete. The roadmap may resume the paused connector lane
at the already-declared Core-private same-call connector-authority verification
increment.
