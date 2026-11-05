# Hosted Worker-Once Nested-Runtime Blocker Fix Review

Review date: 2026-11-04

## Executive Verdict

**Accepted for exact-head CI validation; no code-level blocker remains.**

The fix closes the deterministic nested-runtime panic through the narrowest
available composition change. Both hosted process modes now cross the same
Tokio blocking boundary, preserve the existing bounded error contract, and do
not alter hosted authority or runtime semantics.

Merge remains blocked until the real containerized hosted-alpha recovery job
passes for the unchanged reviewed head.

## Scope Review

The change is limited to:

- one private helper in the hosted binary;
- the existing hosted-alpha recovery rehearsal;
- the authoritative roadmap queue; and
- implementation and review evidence.

It does not change APIs, schemas, queues, leases, provider behavior, report
contracts, credentials, mutation authority, release state, or publication.

## Async Boundary Review

The defect was caused by calling the synchronous PostgreSQL-backed worker from
the outer Tokio runtime thread. The private helper moves the complete
`HostedWorker::run_once` call into `spawn_blocking`, awaits the task, preserves
the worker's own error, and maps only task-join failure to the existing
`hosted.worker.task.failed` code and bounded message.

The one-shot branch calls the helper once and exits. The continuous loop calls
the same helper for each poll and retains its prior shutdown and delay logic.
There is no second synchronous worker call left in the hosted binary.

## Process-Proof Review

The rehearsal does not substitute the new one-shot path for the established
continuous-worker proof.

- An empty foreground one-shot process must print exactly
  `worker_outcome: idle`.
- A separately queued one-shot run must print exactly
  `worker_outcome: completed`, end with `RunCompleted`, and expose terminal
  report metadata for the same run.
- A second run is completed by the long-running worker and remains the subject
  of worker restart, PostgreSQL interruption, recovery, and report-readability
  checks.
- A final foreground one-shot process after recovery must again return the
  exact idle outcome.

Any panic, exit 101, nonzero process result, extra stdout, wrong outcome,
missing terminal event, or missing report fails the rehearsal under
`set -euo pipefail`.

## Error And Security Review

- No Tokio join detail or PostgreSQL payload is exposed.
- No credential value is printed by the new path.
- The one-shot process reuses the service's existing environment and
  no-write provider.
- `--no-deps` prevents the foreground check from silently changing topology;
  the rehearsal already owns PostgreSQL lifecycle.
- The implementation does not create authority, approve work, or change queue
  ownership.

## Validation Review

Local evidence is sufficient for source and contract review:

- `cargo test -p workflow-hosted`: 37 passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-hosted --all-targets -- -D warnings`: passed.
- `bash -n scripts/rehearse-hosted-alpha.sh`: passed.
- `npm run check`: passed with the repository Rust toolchain on `PATH`.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Docker is unavailable on the development host. The review therefore does not
claim local PostgreSQL process evidence. The unchanged reviewed head must pass
the mandatory `Hosted Alpha Recovery` CI job before merge.

## Blockers

No implementation blocker remains.

The exact-head containerized rehearsal and all required repository checks are
merge blockers, not deferred follow-ups.

## Governed Review Record

- Workflow: `dg/review`
- Run: `run-1793848280966789000-2`
- Approval:
  `approval/run-1793848280966789000-2/review-scope-approved`
- Presentation: `presentation/15f48a12ca01aaea`
- Approval outcome: granted by the delegated maintainer after complete
  proof-backed presentation
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Review posture: focused owner maintainer/security review; not independent
  external assurance
- Work outside the kernel: source inspection, validation, documentation, git,
  pull-request, and CI actions are performed by Codex as executor

## Decision

Commit this bounded slice, open one draft pull request, verify the exact diff
and conflict-free head, and require terminal-green exact-head CI before making
the PR ready or merging. Do not resume release or publication work from this
review alone.
