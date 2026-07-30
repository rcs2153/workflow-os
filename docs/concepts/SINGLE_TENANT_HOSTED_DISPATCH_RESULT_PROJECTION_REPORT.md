# Single-Tenant Hosted Dispatch And Result Projection Report

Report date: 2026-07-29

## Executive Summary

The single-tenant hosted alpha now has one Core-owned, end-to-end no-write
execution path. Core derives a hosted request from an authoritative scheduled
skill invocation, atomically commits the invocation events and queued work
item, and atomically projects an exactly bound terminal provider receipt into
the workflow event stream and run snapshot.

This closes the gap where hosted work and receipts were durable but detached
from authoritative workflow execution. It does not make the hosted alpha a
production service and does not enable provider writes, OpenShell, credentials,
general tools, or caller-authored work.

## Scope Completed

- Added an explicit hosted-provider handler posture to immutable run bundles.
- Added a Core-owned single-step no-write executor entry point.
- Preserved proof-enforced approval presentation before approval-gated
  dispatch.
- Derived payload-free hosted requests from immutable run and scheduled-step
  state.
- Atomically committed `SkillInvocationRequested`,
  `SkillInvocationStarted`, the run snapshot, idempotency binding, and queued
  work item in PostgreSQL.
- Validated provider receipts against the exact durable request.
- Atomically committed invocation outcome, terminal run event, run snapshot,
  work-item transition, attempt transition, receipt, and lease release.
- Preserved exact idempotent replay and rejected substituted events, receipts,
  stale revisions, stale fences, and changed bindings.
- Updated the hosted API and worker to use the Core-owned path.

## Scope Explicitly Not Completed

- No provider write or additional provider family.
- No OpenShell, shell, filesystem, process, browser, model, or network
  execution.
- No credential or access-material resolution.
- No caller-authored work items or provider requests.
- No multi-step, branching, parallel, or nested hosted execution.
- No multi-tenancy, enterprise identity, hosted UI, CLI expansion, workflow
  schema change, or production-readiness claim.
- No automatic `WorkReport` artifact generation.

## Runtime Boundary

The hosted API accepts bounded run and immutable-bundle identities but not
provider requests or work items. Core loads the server-owned project, validates
one supported single-step terminal workflow, builds the immutable bundle,
records run and scheduled-step state, and either pauses for approval or creates
the exact dispatch.

Approval resume uses the existing presentation-proof enforcement. A grant
records approval and resume policy events before dispatch. A denial preserves
the existing fail-closed run semantics.

The worker cannot construct workflow success directly. It can only submit an
exact provider receipt to `HostedTerminalResultProjection`, and PostgreSQL
commits the resulting authoritative events only while the expected work-item
revision, attempt revision, and worker fence remain current.

## Atomicity And Replay

Dispatch is one serializable PostgreSQL transaction spanning invocation
events, snapshot projection, idempotency, and work-item creation.

Terminal result projection is one serializable transaction spanning terminal
workflow events, snapshot projection, work item, invocation attempt, provider
receipt, and lease release.

Exact replay returns the prior durable result. Changed events, changed work,
changed receipts, stale revisions, and stale fences fail closed with stable
non-leaking errors.

## Privacy And Security Posture

- Hosted requests remain payload-free and reference-only.
- The deterministic provider receives no source, spec, command, log,
  credential, environment, or authorization value.
- Workflow success stores a stable receipt reference, not provider output.
- Debug output redacts work-item, request, receipt, and projection identity.
- Projection errors do not echo caller-supplied identifiers or metadata.
- Direct receipt storage remains insufficient to claim workflow execution.

## Test Coverage

Focused coverage proves:

- authoritative dispatch requires an unconsumed scheduled step;
- dispatch preserves a running workflow projection;
- completed receipts produce invocation success and run completion;
- substituted receipt bindings fail closed without leakage;
- immutable bundle handler posture represents hosted-provider binding;
- PostgreSQL dispatch and result projection support exact replay;
- the terminal transaction updates run, work item, attempt, receipt, and lease
  together;
- existing hosted provider, executor, state, evidence, approval, SideEffect,
  and report behavior remains compatible.

## Validation

The following validation passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test -p workflow-core --test postgres_state_backend`
- `npm run check` under the repository-pinned Node 20 runtime
- `npm run check:integrations` under the repository-pinned Node 20 runtime
- `bash -n scripts/rehearse-hosted-alpha.sh`
- `git diff --check`

The focused PostgreSQL test passed its local model coverage and skipped the
live database proof because no test database was configured. Live PostgreSQL
concurrency remains mandatory in CI. Docker is not available in the desktop
environment, so the compose deployment rehearsal cannot be claimed as local
evidence.

## Remaining Limitations

1. The dispatch path accepts only one payload-free terminal skill.
2. Provider rejection known not to have started does not yet project a
   terminal workflow failure through the same atomic path.
3. Ambiguous provider outcomes remain reconciliation-required and need an
   authoritative escalation projection.
4. Static alpha authentication is not production mutation authority.
5. Access-material isolation and time-of-use authority remain absent.
6. The full deployed API-to-terminal-report and recovery proof remains open.

## Recommended Next Phase

Complete the hosted alpha's **provider-failure and reconciliation projection
hardening**, then close the deployment/recovery proof. After hosted alpha
acceptance, begin the model-only scoped runtime authority and capability
projection lane before any additional provider mutation family.

Do not add OpenShell or broader writes first.

## Governed Phase Record

- dogfood workflow: `dg/implement`
- run ID: `run-1785371531386068000-2`
- approval ID:
  `approval/run-1785371531386068000-2/implementation-approved`
- approval presentation: `presentation/3f3a4a6d72ed1dc3`
- approval outcome: granted as delegated maintainer
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof-enforced, with the presentation
  marker present in the approval event trail

Repository edits, shell commands, tests, documentation, and git/PR actions are
performed by Codex outside the kernel. The kernel governs the phase; it does
not execute those actions.
