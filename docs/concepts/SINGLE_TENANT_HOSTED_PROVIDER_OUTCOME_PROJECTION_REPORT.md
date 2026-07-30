# Single-Tenant Hosted Provider Outcome Projection Report

Report date: 2026-07-29

## 1. Executive Summary

The single-tenant hosted alpha now projects provider rejection and uncertainty
into authoritative workflow state without fabricating a receipt, success, or
ordinary failure.

A request rejected before provider start atomically fails its invocation and
run, terminalizes the work item, releases the lease, and creates no invocation
attempt or provider receipt. An invocation that may have started but has no
valid receipt atomically marks its durable attempt reconciliation-required,
moves the work item to ambiguous, escalates the run, and releases the lease.
An exactly bound ambiguous provider receipt also escalates instead of failing
the run.

This remains a no-write, single-trust-domain evaluation boundary. It does not
add a provider mutation, credential resolver, OpenShell adapter, production
authority, or production-readiness claim.

## 2. Scope Completed

- Added Core-owned unreceipted outcome vocabulary for:
  - rejection proved not to have started;
  - provider invocation that may have started and requires reconciliation.
- Added deterministic payload-free event projection for both outcomes.
- Changed exactly bound ambiguous receipts from `RunFailed` projection to
  `EscalationTriggered`.
- Added one serializable PostgreSQL transaction spanning:
  - authoritative workflow events;
  - run snapshot;
  - failed or ambiguous work-item transition;
  - reconciliation-required attempt transition when applicable;
  - fenced lease release.
- Preserved exact idempotent replay and rejected conflicting durable state.
- Updated the hosted worker to use the atomic transaction rather than
  separately mutating attempt and work-item state.
- Bound every returned receipt status to its exact durable work-item posture,
  including `ambiguous` rather than an unconditional completed transition.

## 3. Scope Explicitly Not Completed

- No provider writes or additional provider implementation.
- No access-material or credential resolution.
- No shell, process, filesystem, model, browser, or network execution.
- No OpenShell integration or fork.
- No retry, automatic reconciliation, or operator repair behavior.
- No multi-tenancy, enterprise identity, hosted UI, schema change, or CLI
  expansion.
- No production authentication, deployment, recovery, HA, or SLO claim.

## 4. Outcome Semantics

| Provider posture | Work item | Attempt | Workflow run | Receipt |
| --- | --- | --- | --- | --- |
| Rejected before start | `failed` | absent | `Failed` | absent |
| May have started, no valid receipt | `ambiguous` | `reconciliation_required` | `Escalated` | absent |
| Exactly bound ambiguous receipt | `ambiguous` | `terminal` with ambiguous status | `Escalated` | present |

The distinction is deliberate. A known pre-start rejection is authoritative
failure. A possibly started invocation is uncertainty that requires operator
reconciliation and must not be silently retried.

## 5. Atomicity And Replay

The fresh unreceipted transaction locks the current running work item,
validates the active fence and expected revisions, verifies the exact
work-item transition, locks the run snapshot, appends Core-generated events,
rehydrates and compares the resulting run, updates the snapshot and work item,
updates the attempt when reconciliation is required, and releases the lease.

Exact replay requires the same work item, exact events, exact rehydrated run,
and the expected attempt posture. A changed outcome, partial prior commit,
stale revision, stale fence, missing attempt, unexpected attempt, or changed
event fails closed.

## 6. Privacy And Error Posture

- No provider error text is copied into workflow state.
- Failure and escalation records use fixed platform-owned codes and summaries.
- Projection Debug output contains only outcome, event count, and run status.
- No raw provider payload, command output, path, credential, token, or access
  material is stored.
- A missing receipt remains explicit; the projection never invents one.

## 7. Test Coverage

Focused tests cover:

- authoritative pre-start failure event shape;
- reconciliation escalation event shape;
- exactly bound ambiguous receipt escalation;
- exact receipt-status to durable work-item status mapping;
- redaction-safe projection Debug output;
- atomic PostgreSQL commit and exact replay for pre-start rejection;
- no attempt creation on pre-start rejection;
- atomic PostgreSQL attempt/work-item/run escalation for uncertainty;
- reconciliation-required attempt posture;
- compatibility with the existing completed-receipt transaction;
- existing hosted, Core, state, approval, SideEffect, evidence, and report
  behavior.

The PostgreSQL transaction compiles locally and is exercised against the live
database in mandatory CI. A local run without
`WORKFLOW_OS_TEST_POSTGRES_URL` skips live database execution by design.

## 8. Validation

The phase validation completed successfully:

- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check`: passed under the repository Node 20 toolchain
- `npm run check:integrations`: passed
- `npm run check:docs`: passed
- `bash -n scripts/rehearse-hosted-alpha.sh`: passed
- `git diff --check`: passed

Live PostgreSQL conformance remains a mandatory CI merge gate because no local
`WORKFLOW_OS_TEST_POSTGRES_URL` was configured.

## 9. Remaining Limitations

1. The provider remains deterministic and no-write.
2. Reconciliation is explicit but not automatically resolved.
3. Static alpha authentication is not production mutation authority.
4. Access-material isolation and time-of-use resolution remain absent.
5. The deployed API-to-terminal-report restart/recovery proof remains open.
6. Exactly bound ambiguous receipts preserve a terminal attempt record while
   escalating the run; any later operator resolution model needs separate
   design and review.

## 10. Recommended Next Phase

Complete the **single-tenant hosted deployment and recovery proof**, including
API/worker restart, abandoned lease takeover, stale-fence rejection, and
terminal report inspection in the defined compose topology.

After hosted alpha acceptance, begin the already planned model-only scoped
runtime authority and capability projection lane. Do not add another provider
mutation family or OpenShell first.

## 11. Governed Phase Record

- dogfood workflow: `dg/implement`
- run ID: `run-1785378041253647000-2`
- approval ID:
  `approval/run-1785378041253647000-2/implementation-approved`
- approval presentation: `presentation/2bdaf97b15b3c846`
- approval outcome: granted with persisted presentation proof
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced with event marker present

Repository edits, shell commands, tests, documentation, and git/PR actions are
performed by Codex outside the kernel. The kernel governs the phase; it does
not execute those actions.
