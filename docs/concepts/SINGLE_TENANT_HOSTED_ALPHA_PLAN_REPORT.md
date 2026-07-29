# Single-Tenant Hosted Alpha Plan Report

Report date: 2026-07-29

## 1. Executive Summary

The single-tenant hosted alpha is now phase-ready as one integrated future
implementation milestone. The plan composes the accepted local kernel and
shared `PostgreSQL` state foundations into an authenticated remote API,
stateless fenced workers, an explicit execution-provider boundary, one reviewed
access-material boundary, governance enforcement, observability, and recovery
proof.

This phase changed planning and roadmap documentation only. Hosted runtime
behavior is not implemented.

## 2. Scope Completed

- Defined the single-tenant alpha product and trust-domain boundary.
- Defined API, worker, state, execution-provider, and access-material
  responsibilities.
- Defined source-of-truth, idempotency, fencing, cancellation, retry, and
  ambiguous-outcome semantics.
- Defined approval, policy, proportional-governance, evidence, SideEffect,
  audit, and WorkReport composition.
- Defined deployment, observability, recovery, security, tests, and acceptance
  gates.
- Grouped implementation into one accelerated vertical milestone.
- Updated roadmap sequencing after acceptance of shared `PostgreSQL` state.

## 3. Scope Explicitly Not Completed

- No hosted API or service.
- No worker process or automatic consumer.
- No execution provider.
- No credential resolver or sensitive-material storage.
- No provider mutation expansion.
- No OpenShell integration.
- No multi-tenancy or enterprise identity.
- No workflow schema, SDK, CLI, or example changes.
- No release or production-readiness change.

## 4. Architecture Decision Summary

Core remains the governance authority. `PostgreSQL` remains the durable shared
source of truth. API and workers are stateless. Workers use database-time
leases and fencing. Hosted execution uses a new explicit execution-provider
boundary rather than treating local `SkillHandler` implementations as a shell,
network, or credential runtime.

An optional sandbox may implement the provider boundary later, but the first
hosted alpha does not depend on one.

## 5. User Feedback Reconciliation

Fresh-pull testing confirms that current scaffolding, first-run posture,
recommendation authoring, approval, event history, and honest missing-handler
behavior are credible.

The plan preserves the highest-priority product recommendation: reduce
low-risk ceremony through deterministic proportional governance and quiet
success while retaining evidence and audit. Node-version determinism and
duplicate missing-manifest diagnostics were already fixed on current `main`.
No unrelated CLI work was added to this planning phase.

## 6. Governance Record

- workflow ID: `dg/d`
- run ID: `run-1785343290147557000-2`
- approval ID:
  `approval/run-1785343290147557000-2/planning-approved`
- presentation ID: `presentation/4fcf0b15af604eb7`
- presentation content hash:
  `4fcf0b15af604eb78c41d4922b4f0f06c917b6c7464fd600efef55b1710d521e`
- approval outcome: granted
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations
- approval-presentation enforcement: proof-enforced with one persisted record

Three earlier phase-start attempts failed validation before creating a run.
No state was edited by hand and no failed attempt was represented as governed
approval.

## 7. Validation

Completed:

- `npm run check:docs`: passed with the repository-pinned Node 20 toolchain;
- `git diff --check`: passed;
- current-product and known-limitations truth review: passed;
- governed phase close and event inspection: passed with 39 ordered events.

Repository edits, shell validation, git operations, and the future PR lifecycle
were performed outside the kernel by the delegated maintainer. The kernel
governed scope, approval, and phase closure; it did not execute edits, tests,
commits, pushes, or GitHub operations.

## 8. Remaining Limitations

- Authentication mechanism remains an implementation decision within the
  bounded single-trust-domain contract.
- Runnable-work discovery has not been implemented.
- Production TLS, pooling, HA, PITR, capacity, and SLOs remain future.
- The first execution provider remains to be selected and implemented.
- Credential delivery should remain deferred until the no-write provider proof
  is accepted unless implementation review establishes a necessary narrower
  path.

## 9. Recommended Next Phase

Implement the single-tenant hosted alpha as one governed vertical milestone.
Do not restart separate planning phases for each API route, worker helper,
metric, or deployment file unless a security or correctness blocker demands it.
