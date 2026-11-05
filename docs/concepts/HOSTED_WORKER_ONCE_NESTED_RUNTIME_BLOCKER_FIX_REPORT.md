# Hosted Worker-Once Nested-Runtime Blocker Fix Report

Report date: 2026-11-04

## Executive Verdict

The deterministic `workflow-os-hosted --worker-once` nested-runtime panic is
fixed at the process-composition boundary.

One-shot and continuous hosted worker modes now call `HostedWorker::run_once`
through the same `tokio::task::spawn_blocking` helper. A blocking-task join
failure maps to the existing stable `hosted.worker.task.failed` error instead
of exposing a Tokio join error or panic. No hosted authority, provider,
scheduler, API, schema, or report contract changed.

The deployed hosted-alpha rehearsal now exercises two distinct runs. A real
one-shot process proves idle and completed outcomes plus terminal events and a
persisted report. A separate run remains assigned to the continuous worker so
the existing worker restart and PostgreSQL interruption/recovery proof is not
weakened.

## Failure Reproduced

The hosted binary is a Tokio application, while the synchronous PostgreSQL
client creates and blocks on its own current-thread runtime. The one-shot branch
called `HostedWorker::run_once` directly from the outer Tokio runtime thread.
Both an empty queue and a populated healthy queue therefore terminated with
exit code 101 and Tokio's nested-runtime panic.

The continuous worker did not have this defect because every poll already ran
inside `spawn_blocking`.

## Scope Completed

- Added one private async `run_worker_once` composition helper.
- Routed both `--worker-once` and the continuous worker loop through it.
- Preserved the existing stable `hosted.worker.task.failed` join-failure code
  and bounded message.
- Extended the hosted-alpha rehearsal to require:
  - exact `worker_outcome: idle` from an empty one-shot process;
  - exact `worker_outcome: completed` for a separately queued one-shot run;
  - authoritative `RunCompleted` event projection for that run;
  - terminal report metadata for that run;
  - completion of a distinct run through the continuous worker;
  - continuous-worker restart;
  - PostgreSQL interruption, readiness failure, recovery, and durable terminal
    report readability; and
  - another exact idle one-shot result after recovery.
- Kept the existing hosted-alpha recovery CI job as the real process and
  PostgreSQL acceptance surface.

## Scope Explicitly Not Completed

- No hosted architecture rewrite or new process owner.
- No scheduler, discovery, queue, lease, or worker concurrency change.
- No API, schema, workflow, report, or public CLI contract change.
- No provider mutation, credential, OpenShell, or execution-family expansion.
- No evidence-provenance or report-integrity runtime implementation.
- No release, tag, publication, or production-readiness claim.

## Runtime Boundary

`run_worker_once` owns the only synchronous worker call in the hosted binary.
It moves the `Arc<HostedWorker>` into `spawn_blocking`, awaits the task, maps a
join failure to the existing platform error, propagates the worker's own
bounded error, and returns only whether one item completed.

The one-shot branch prints once and exits. The continuous loop prints the same
outcome and retains its existing shutdown-or-delay behavior. Sharing this
helper prevents the two process modes from drifting across incompatible Tokio
boundaries.

## Recovery Rehearsal Design

The rehearsal intentionally uses two run identities:

1. The one-shot run is created while no continuous worker exists. A foreground
   `docker compose run --rm --no-deps worker --worker-once` must complete it and
   print exactly one completed outcome.
2. The recovery run is then created and completed by the long-running worker.
   That worker is restarted, PostgreSQL is interrupted and restored, and the
   same terminal run and report remain readable.

This prevents the new regression from replacing the older continuous-worker
proof with a weaker idle-only check.

## Validation

Passed locally:

- `cargo test -p workflow-hosted`: 37 passed.
- `cargo fmt --all --check`.
- `cargo clippy -p workflow-hosted --all-targets -- -D warnings`.
- `bash -n scripts/rehearse-hosted-alpha.sh`.
- `npm run check` with the repository Rust toolchain on `PATH`.
- `npm run check:docs`.
- `git diff --check`.

Docker is not installed in the desktop development environment. The real
binary/PostgreSQL rehearsal is therefore not represented as local evidence.
The exact-head `Hosted Alpha Recovery` GitHub Actions job must pass before
merge; its failure is a release blocker.

## Governed Phase Record

- Workflow: `dg/blocker`
- Run: `run-1793847937891006000-2`
- Approval: `approval/run-1793847937891006000-2/fix-approved`
- Presentation: `presentation/8ffd7acc61ed2e0f`
- Approval outcome: granted by the delegated maintainer after the complete
  proof-backed handoff was presented
- Kernel status after approval: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Work outside the kernel: source edits, shell validation, documentation, git,
  pull-request, and CI actions are performed by Codex as executor

## Acceptance Boundary

The implementation is ready for focused maintainer/security review. It is not
accepted for merge until the reviewed head passes the real hosted-alpha
recovery job and all required repository checks.
