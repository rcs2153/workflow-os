# Single-Tenant Hosted Alpha Plan Review

Review date: 2026-07-29

## 1. Executive Verdict

**Plan accepted; proceed to the single-tenant hosted alpha implementation
milestone.**

## 2. Scope Verification

The phase stayed planning-only. It added no hosted code, worker, automatic
consumer, backend selection, credential storage, provider mutation, OpenShell
integration, multi-tenancy, identity system, schema/SDK surface, examples, or
release claim.

## 3. Product Boundary Assessment

The plan defines one deployment, one organization, and one administrative
trust domain. It does not imply tenant isolation or enterprise administration.
The service governs work and does not present itself as a general agent,
sandbox, build, or provider runtime.

## 4. Architecture Assessment

The separation is sound:

- Core owns governance and state transitions.
- `PostgreSQL` owns shared durable truth.
- API and workers remain stateless.
- database-time leases and fencing protect worker commits;
- execution providers cannot mutate Core state;
- access material remains external and reference-only;
- telemetry remains non-authoritative.

The plan correctly rejects use of the local development/test `SkillHandler` as
an implicit hosted execution boundary.

## 5. API And Identity Assessment

The planned API covers validation, run creation, inspect, events, approval,
cancellation, report retrieval, health, readiness, and version identity.
Mutations require authentication, correlation, idempotency, bounded input, and
audit context.

The plan does not overdesign enterprise roles. It leaves the exact
deployment-bound authentication mechanism open while requiring actor identity,
expiration, rotation, non-leakage, and separation from Core approval authority.

## 6. Worker And Correctness Assessment

The worker lifecycle preserves immutable run identity, current eligibility,
approval proof, policy, authority, evidence/check facts, capability,
SideEffect, idempotency, and lease fencing before execution.

Worker loss, cancellation, retries, stale commits, and ambiguous provider
outcomes are explicit. The plan avoids exactly-once claims and blind replay.

## 7. Security And Privacy Assessment

The access-material boundary is reference-only and time-of-use scoped. The
threat-review list covers authentication, replay, stale authority, bundle
substitution, leases, confused deputy, leakage, injection, unauthorized reads,
ambiguous effects, database privileges, and supply chain.

No raw payload, credential, command output, or arbitrary source-content path is
authorized.

## 8. Evidence And Reporting Assessment

The plan preserves separate authoritative events, audit, telemetry,
`EvidenceReference`, `SideEffect`, `WorkReport`, and operational diagnostics.
Execution receipts become evidence only through reviewed Core mapping.
Report failure remains separately inspectable instead of rewriting workflow
status.

## 9. Operations And Recovery Assessment

The plan requires health, readiness, bounded metrics, shutdown, restart,
takeover, stale-fence rejection, backup/restore, projection rebuild, rollback,
and ambiguous-outcome procedures. It explicitly avoids HA, PITR, capacity, and
SLO claims.

## 10. Test Plan Assessment

The planned tests cover the complete API-to-worker-to-report path plus
authentication, idempotency, immutable inputs, approvals, current authority,
proportional governance, leasing, cancellation, retries, provider receipts,
non-leakage, restart, recovery, and local compatibility.

This is sufficient for implementation to begin as one vertical milestone.

## 11. Blockers

None at planning level.

The implementation must select and review a minimal authentication mechanism
and first no-write execution provider before their code is accepted. Those are
implementation decisions, not reasons for another general planning cycle.

## 12. Non-Blocking Follow-Ups

- Decide whether the alpha accepts canonical bundle upload or catalog
  references only.
- Define bounded event/report retention.
- Add network/process fault injection.
- Exercise another maintained `PostgreSQL` major version.
- Continue proportional-governance quiet-success integration so low-risk
  hosted work does not inherit unnecessary approval ceremony.

## 13. Recommended Next Phase

Proceed to **single-tenant hosted alpha implementation** as one accelerated
milestone.

Begin with transport-neutral service and execution-provider contracts, then
complete authenticated API, stateless fenced worker, no-write provider proof,
governance composition, observability, deployment, and recovery before phase
review.
