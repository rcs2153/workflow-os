# Single-Tenant Hosted Alpha Implementation Report

Report date: 2026-07-29

## Executive Summary

The first single-tenant hosted foundation is implemented as a deliberately
inert, receipt-only proof. Workflow OS now has transport-neutral hosted request
and receipt contracts, durable PostgreSQL queue/fence/receipt primitives, an
authenticated read-only inspection API, and a stateless worker hard-bound to a
no-write provider.

The worker does not append workflow events, mutate run snapshots, execute
skills, accept remote work submissions, or claim that a no-op receipt proves
workflow execution. The complete hosted-alpha milestone remains blocked.

## Files Changed

- `crates/workflow-core/src/hosted.rs`
- `crates/workflow-core/src/postgres_state.rs`
- `crates/workflow-core/src/lib.rs`
- `crates/workflow-core/tests/postgres_state_backend.rs`
- `crates/workflow-hosted/**`
- `deploy/hosted-alpha/**`
- `docs/runtime/single-tenant-hosted-alpha.md`
- `docs/security/single-tenant-hosted-alpha-threat-model.md`
- the hosted plan, roadmap, report, and review documents

## Behavior Added Or Changed

- Added payload-free hosted execution requests, receipts, work items, budgets,
  policy bindings, references, and provider errors.
- Added internal idempotent work creation bound to an existing immutable bundle
  and `Running` run.
- Added PostgreSQL queue discovery, fenced claims, expired-lease takeover, and
  atomic hosted receipt commits.
- Retained released lease rows so fencing tokens remain monotonic across
  release and reacquisition.
- Added one built-in provider that performs no filesystem, process, network,
  model, credential, or provider action.
- Added authenticated liveness/readiness/version and hosted-record inspection.
- Excluded remote work creation and run/event projection from the API.
- Converted known-before-start provider rejection into a terminal failed work
  item so one poison item does not stop the long-running worker.

## Contracts Added Or Changed

`HostedExecutionRequest` binds immutable run identity, stable input references,
capability and SideEffect posture, policy hash, budget, correlation, and
idempotency without carrying credentials or raw payloads.

`HostedExecutionReceipt` binds provider identity/version/configuration, request
fingerprint, policy hash, environment reference, timing, status, and stable
references.

`PostgresCommitHostedReceiptRequest` derives the authoritative request from the
locked work item. It cannot substitute a separate request and cannot append a
workflow event or overwrite a snapshot.

## Tests Added Or Changed

Focused coverage includes:

- hosted request, work-item, receipt, and serde validation;
- exact request/receipt/provider binding;
- no-write provider success and pre-start rejection;
- authenticated API posture and absence of remote mutation/run routes;
- idempotent replay after a work item reaches a terminal state;
- atomic live-PostgreSQL receipt/work-item/lease commit when CI provides the
  required database;
- fence-token monotonicity after release and reacquisition;
- stale-fence rejection;
- redaction-safe Debug and stable non-leaking errors.

## Docs Added Or Changed

The runtime guide and threat model now state that:

- this is a receipt-only no-write foundation;
- remote work submission and run/event projection are absent;
- the worker does not record skill success;
- the compose topology is defined but was not locally rehearsed;
- provider mutation, access-material resolution, remote approval, report
  retrieval, unique invocation identity, and production operations remain
  deferred.

The latest fresh-pull user review reinforces the accepted product direction:
first-run honesty is working, and low-risk ceremony should be reduced through
Risk-Proportional Governance and Quiet Success without weakening evidence,
audit, or fail-closed boundaries.

## Validation Performed

Passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `cargo audit`
- `npm audit --audit-level=moderate`
- `git diff --check`
- `sh -n deploy/hosted-alpha/init/001-runtime-role.sh`

The conditional PostgreSQL tests require the CI database environment. Docker
image build, compose startup, service health, restart, and recovery were not
run locally because Docker is unavailable in the validation environment.

## Security And Privacy Considerations

- API token material is hashed for comparison, redacted from Debug, and not
  persisted by Workflow OS.
- Hosted references reject paths, URLs, controls, traversal, and secret-like
  values.
- Provider receipts contain stable references and bounded categories rather
  than payloads.
- Remote callers cannot author hosted work through this API foundation.
- A no-write receipt cannot mutate the authoritative run or fabricate a
  `SkillInvocationSucceeded` event.
- Static bearer auth, one shared non-superuser runtime database role,
  deployment TLS/network policy, and image supply-chain hardening remain
  evaluation limitations.

## Assumptions Made

- One operator-controlled deployment and one administrative trust domain.
- PostgreSQL, API, and worker are trusted deployment components.
- Only the built-in deterministic no-write provider is worker-eligible.
- Work items are created by a trusted internal caller until runtime
  composition can derive them from approved immutable runs.
- Lease duration is sufficient only for the inert provider.

## Risks And Follow-Ups

- Different idempotency keys can still describe the same inert request.
  Effecting providers need a unique durable invocation identity.
- Providers that may have started require durable pre-attempt posture,
  reconciliation, lease renewal, and no blind retry.
- Remote run creation, proof-enforced approval, cancellation, report retrieval,
  current-authority reassessment, metrics, and deployed recovery remain
  blockers.
- The repository's declared Rust 1.78 floor does not match the current locked
  dependency graph. The evaluation image uses Rust 1.95 pending a separate
  compatibility correction.
- NVIDIA OpenShell remains a plausible optional future provider substrate; it
  should not replace Workflow OS governance or be forked absent a concrete
  upstream blocker.

## Incomplete Or Placeholder Work

This phase does not implement useful hosted execution, a sandbox, credentials,
provider writes, workflow event integration, automatic report generation,
multi-tenancy, enterprise identity, UI, hosted administration, or production
operations. The compose files are evaluation definitions, not deployment
proof.

## Recommended Next Phase

Proceed with the **single-tenant hosted alpha runtime-composition blocker fix**:

1. derive hosted work from an immutable governed run and durable invocation
   identity;
2. expose proof-enforced approval and eligible cancellation;
3. expose bounded WorkReport and artifact metadata;
4. persist pre-invocation attempt and reconciliation posture;
5. add operational metrics and complete API/worker/restart/recovery proof.

Do not add OpenShell, provider writes, multi-tenancy, enterprise identity,
workflow schema expansion, or production claims first.

## Governed Phase Record

- dogfood workflow: `dg/implement`
- run ID: `run-1785344216466147000-2`
- approval ID:
  `approval/run-1785344216466147000-2/implementation-approved`
- approval outcome: granted as delegated maintainer
- approval presentation: `presentation/185424c0276ccfe7`
- terminal status observed at phase close: `Completed`
- event summary observed at phase close: 39 events, one approval, zero retries,
  zero escalations

Repository edits, shell commands, validation, dependency probes,
documentation, and git/PR actions were performed by Codex outside the kernel.
The kernel governed the phase; it did not execute those actions. The current
untracked dogfood state does not retain this run, so the report records the
phase-close output without claiming an independently re-inspected local state
record.
