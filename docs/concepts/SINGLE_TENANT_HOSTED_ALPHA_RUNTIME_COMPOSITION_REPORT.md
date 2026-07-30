# Single-Tenant Hosted Alpha Runtime Composition Report

Report date: 2026-07-29

## Executive Summary

The hosted alpha now composes more of the accepted local and shared-state
kernel without broadening external execution. A transport-neutral immutable
bundle store seam lets the existing executor publish and re-read bundles
through `PostgreSQL`. The hosted API exposes bounded governed-run, event,
approval, cancellation, report-metadata, and operational inspection paths.
The worker rechecks authoritative run and bundle state and uses a durable
invocation-attempt record before crossing the no-write provider boundary.

This is a single-tenant evaluation runtime, not a production hosted service.
The built-in provider remains deterministic and no-write. Callers cannot
submit hosted work items or provider requests.

## Scope Completed

- Added a transport-neutral create-only immutable run-bundle store contract.
- Implemented the contract for the local immutable store and `PostgreSQL`.
- Reused the existing immutable-bundle executor path for server-owned hosted
  run creation.
- Added authenticated bounded retrieval for runs, ordered event pages, exact
  approval requests, terminal report-artifact metadata, work items, receipts,
  and fixed operational metrics.
- Added idempotency-bound proof-enforced approval decisions and eligible
  cancellation through existing Core executor paths.
- Added a payload-free durable hosted invocation-attempt lifecycle.
- Added fence-preserving lease renewal and atomic attempt/receipt/work-item
  terminal commit.
- Added claim-time cancellation and immutable-bundle eligibility checks before
  provider invocation.
- Added an explicit API/worker restart rehearsal for the evaluation topology.

## Scope Explicitly Not Completed

- No provider writes or new mutation family.
- No OpenShell, shell, process, filesystem, network, browser, or model
  execution.
- No caller-authored hosted work item or provider request.
- No atomic scheduled-skill-to-hosted-work dispatch.
- No terminal hosted receipt projection into
  `SkillInvocationSucceeded` or workflow completion.
- No full `WorkReport` body transport.
- No multi-tenancy, enterprise identity, hosted UI, workflow schema change, or
  production-readiness claim.

## API And Runtime Boundary

The API uses one server-owned project root. Run creation accepts identities
and bounded immutable-bundle options, but not a project path, workflow payload,
provider request, or work item. Core loads and validates the project, builds
the exact immutable bundle, publishes it create-only, and creates the run.

Approval decisions require an exact validated presentation record, matching
run and approval identity, matching authenticated actor, and a bounded
freshness window before the existing proof-enforced executor path is called.
Cancellation uses the existing policy-tested Core path. Both mutation shapes
require caller-supplied idempotency keys bound to deterministic, payload-free
intent hashes; conflicting reuse fails closed without exposing request values.

The alpha bearer token still maps to one deployment actor and has no issuer,
audience, expiry, operation scope, or enterprise role semantics. Mutation
routes therefore remain single-trust-domain evaluation surfaces.

## Durable Attempt And Recovery Posture

Each hosted provider invocation has one durable identity bound to the work
item, request fingerprint, provider identity/version, and provider
configuration hash. The attempt is prepared before invocation, transitions
under the active work-item fence, and becomes terminal only with an exactly
matching receipt.

A possibly started invocation becomes reconciliation-required and is not
blindly retried. Lease renewal preserves the active fence rather than minting a
new token. Terminal receipt commit updates the attempt, work item, receipt, and
lease release atomically.

The restart rehearsal validates authenticated readiness and operational
inspection across API and worker process restarts. It does not claim high
availability, disaster-recovery objectives, or fault-injected production
recovery.

## Privacy And Security Posture

- API bodies are bounded.
- Paths, authentication material, actor identity, and request identity are
  redacted from Debug output.
- Transport errors use fixed non-leaking messages.
- Hosted attempts, receipts, and metrics are payload-free.
- Report retrieval is metadata-only.
- The provider receives no raw workflow/spec content, command output,
  credential, or access-material value.

## Testing And Validation

Focused coverage exercises:

- local and `PostgreSQL` immutable-bundle store behavior;
- durable attempt lifecycle and substituted-receipt rejection;
- create-only attempt identity and replay;
- fenced attempt transitions and stale-fence rejection;
- fence-preserving renewal;
- atomic terminal attempt/work-item/receipt commit;
- fixed low-cardinality metrics;
- authenticated route posture and no-write provider rejection;
- existing local executor and hosted-state regressions.

Validation results:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed, with opt-in live-provider tests remaining
  ignored as designed;
- `npm run check`: passed under the repository-pinned Node 20 runtime;
- `npm run check:integrations`: passed under Node 20;
- `npm run check:docs`: passed;
- `bash -n scripts/rehearse-hosted-alpha.sh`: passed;
- `git diff --check`: passed.

The container restart rehearsal was not run because Docker is unavailable in
the desktop environment. The compose topology and rehearsal script are
evaluation assets, not claimed deployment evidence.

## Known Limitations

1. Hosted work is not yet atomically derived from a scheduled skill invocation
   and projected back into the authoritative workflow event stream.
2. The bearer-token mechanism is not suitable production mutation authority.
3. API and worker still share one non-superuser database role.
4. Event pagination bounds the response but does not yet use a database-level
   page query.
5. The restart rehearsal is an evaluation check, not a chaos or recovery
   certification, and it was not executable in this Docker-less environment.
6. OpenShell remains an unimplemented optional future execution provider.

## Recommended Next Phase

Implement and review one **Core-owned atomic hosted dispatch and result
projection path** for the deterministic no-write provider:

1. derive the exact work request from the immutable bundle and authoritative
   scheduled invocation;
2. atomically pause the run and commit the work item;
3. prevent claim after cancellation, authority loss, or changed eligibility;
4. atomically project the terminal receipt into the exact invocation outcome
   and resume or complete the run.

Do not add OpenShell or provider writes first.

## Governed Phase Record

- dogfood workflow: `dg/implement`
- run ID: `run-1785362530571483000-2`
- approval ID:
  `approval/run-1785362530571483000-2/implementation-approved`
- approval presentation: `presentation/ad5126a7b2c91445`
- approval outcome: granted as delegated maintainer
- phase status: `Completed`
- phase-close event summary: 39 events, 1 approval, 0 retries, 0
  escalations
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`,
  `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`,
  `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, `StepScheduled:6`
- approval-presentation enforcement: proof enforced with the presentation
  marker present in the event trail

Repository edits, shell commands, validation, deployment probes, documentation,
and git/PR actions are performed by Codex outside the kernel. The kernel
governs the phase; it does not execute those actions. Docker deployment
rehearsal was skipped because the dependency was unavailable, and no hosted
handler, container, external system, or report artifact was simulated.
