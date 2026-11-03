# Provider Fixture Next-Action Routing P1 Report

Status: Implemented and accepted after focused maintainer review.

## Summary

Fresh-repository evaluation found that `workflow-os status` and
`workflow-os inspect` correctly routed public GitHub repository-metadata
fixture runs through their SQLite state backend, while `workflow-os
next-action` unconditionally opened the generic local backend and filesystem
immutable-bundle store. The same fixture run therefore failed with
`runtime.events.empty` even though its durable state was present and readable.

This phase gives `next-action` the same bounded backend-selection posture. If
the fixture SQLite database contains the exact run, both run state and immutable
bundle are read from that database. Otherwise the existing local backend and
filesystem bundle store remain unchanged.

## Governed Phase

- Workflow: `dg/implement`
- Run: `run-1793723406988703000-2`
- Approval: `approval/run-1793723406988703000-2/implementation-approved`
- Approval presentation: `presentation/579d16e07b6af9bb`
- Approval outcome: granted

## Implemented Scope

- Added one backend/store-generic `next-action` preview helper.
- Routed exact fixture runs to the existing fixture SQLite backend only when
  their event stream is present there.
- Preserved the generic local backend and filesystem bundle-store fallback.
- Added bounded fixture posture fields to successful human and JSON previews.
- Added public fixture regression coverage for:
  - waiting runs returning the intentional non-running diagnostic rather than
    `runtime.events.empty`;
  - approved running runs producing a valid continuation preview from SQLite;
  - completed runs returning the intentional non-running diagnostic; and
  - byte-identical `inspect` output before and after each preview attempt,
    proving the command remains non-mutating.

## Non-Goals

This phase does not consume continuation authority, append events, mutate run
state, invoke a handler or provider, retry work, redesign the fixture, change
the continuation model, add hosted behavior, change schemas, or alter release
posture.

## Validation

- Focused public GitHub fixture lifecycle test: passed.
- Full GitHub read-only example suite: passed, 12 tests.
- Full `workflow-cli` integration suite: passed, 176 tests.
- Strict `workflow-cli` Clippy with warnings denied: passed.
- Rust formatting check: passed.
- Documentation check: passed.
- Diff hygiene check: passed.

Focused maintainer review found no remaining blocker. See [Provider Fixture
Next-Action Routing P1 Review](PROVIDER_FIXTURE_NEXT_ACTION_ROUTING_P1_REVIEW.md).

## Next Authorized Work

The final fresh-rerun P1 defect is accepted. The authoritative queue may resume
the previously paused connector lane at its already-declared Core-private
same-call authority verification increment.
