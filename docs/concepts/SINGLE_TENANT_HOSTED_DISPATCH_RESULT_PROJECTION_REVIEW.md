# Single-Tenant Hosted Dispatch And Result Projection Review

Review date: 2026-07-29

## 1. Executive Verdict

**Atomic hosted dispatch and terminal result projection accepted; complete
hosted alpha remains blocked.**

The phase closes the principal orchestration-integrity gap identified in the
prior hosted review. A no-write provider receipt can now affect authoritative
workflow state only through a Core-derived dispatch and an exact, atomic,
fenced terminal projection.

## 2. Scope Verification

The implementation remains single-tenant, evaluation-only, and no-write. It
does not add OpenShell, credentials, caller-authored work, general tool
execution, provider mutation, multi-tenancy, enterprise identity, UI, CLI
expansion, schema changes, or production claims.

## 3. Core Ownership Assessment

Core remains the source of workflow truth. It derives the request from a
validated immutable bundle and scheduled skill, records policy and approval
state, owns invocation identities, validates the provider receipt, and creates
the terminal event projection.

The API does not manufacture work items. The worker does not append workflow
events or mutate snapshots independently. PostgreSQL persists Core-validated
transitions atomically.

## 4. Dispatch Assessment

The supported path requires one payload-free terminal skill with no adapter,
input mapping, SideEffect event, or required before-skill hook. Unsupported
workflow shapes fail closed before dispatch.

Invocation request/start events, snapshot projection, idempotency binding, and
queued work are one serializable transaction. Exact replay is accepted;
changed durable state is rejected.

## 5. Result Projection Assessment

The receipt must match the exact request fingerprint and policy binding.
PostgreSQL additionally validates the active work-item revision, invocation
attempt revision, provider binding, and worker fence.

The terminal transaction appends the exact invocation and run events,
rehydrates and compares the projected run, updates its snapshot, terminalizes
the work item and attempt, stores the receipt, and releases the lease.

Direct receipt storage is not treated as workflow success.

## 6. Approval And Cancellation Assessment

Approval-gated dispatch preserves the existing presentation-proof enforcement
and resume policy path. Denial fails the run closed.

Claim-time cancellation remains protected by authoritative run rehydration.
A cancellation or revision race before terminal commit causes the fenced
transaction to fail rather than overwriting newer state.

## 7. Privacy And Redaction Assessment

Requests, work items, receipts, and projections remain reference-only and
payload-free. Debug output redacts identities. Stable errors do not echo paths,
tokens, raw metadata, command output, provider output, or secret-like values.

## 8. Test Quality Assessment

The new focused model tests cover successful dispatch/result projection,
substituted-receipt rejection, terminal event kinds, status preservation, and
Debug non-leakage. PostgreSQL conformance covers atomic dispatch, exact replay,
fenced attempt lifecycle, atomic terminal projection, durable receipt, and
authoritative completed-run rehydration.

Live PostgreSQL proof remains CI-dependent when the local environment lacks a
configured test database.

## 9. Blockers

1. Project provider rejection known not to have started into authoritative
   terminal failure without leaving a running workflow.
2. Project ambiguous provider outcomes into an explicit reconciliation or
   escalation state without fabricating success or retrying blindly.
3. Complete deployed API-to-terminal-report restart and recovery evidence.
4. Replace static alpha authentication before any production mutation claim.
5. Add scoped access-material and time-of-use authority before provider writes.

## 10. Non-Blocking Follow-Ups

- Move bounded event pagination into the database query.
- Separate API and worker database privileges.
- Add fault-injection coverage for commit loss after provider return.
- Correct the repository-wide Rust compatibility declaration separately.

## 11. Recommended Next Phase

Proceed with **hosted provider-failure and reconciliation projection
hardening**, still with the deterministic no-write provider. Then complete the
deployment/recovery proof and review the hosted alpha as one milestone.

OpenShell remains a promising optional execution-provider adapter after these
Core boundaries are accepted. A fork is not justified unless upstream prevents
stable request binding, policy/configuration hashing, bounded references,
denied-action reporting, cancellation/reconciliation semantics, or
non-leaking receipts.
