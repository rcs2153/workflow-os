# Single-Tenant Hosted Alpha Implementation Review

Review date: 2026-07-29

Fix-forward update:

The subsequent
[runtime-composition review](SINGLE_TENANT_HOSTED_ALPHA_RUNTIME_COMPOSITION_REVIEW.md)
accepts the immutable-store, proof-enforced mutation, durable-attempt, bounded
inspection, and operational hardening while preserving the original blocker
finding for atomic scheduled-skill dispatch and authoritative terminal result
projection.

## 1. Executive Verdict

**Needs blocker fixes.**

The hosted execution foundation is coherent, bounded, and worth retaining, but
the complete milestone acceptance criteria are not met. The implementation
must not be described as an accepted hosted alpha yet.

## 2. Scope Verification

The implementation stayed within the single-tenant, no-write boundary. It did
not add multi-tenancy, enterprise identity, UI, OpenShell, general agent
execution, provider writes, workflow schema changes, examples, or release
posture.

The new compose topology is an evaluation proof only.

## 3. Architecture Assessment

The ownership boundary is correct:

- Core owns validation and authoritative state transitions.
- `PostgreSQL` owns durable shared truth.
- API and worker remain stateless.
- execution providers cannot mutate Core state;
- receipt validation binds the provider result to the exact immutable request.

The separate `workflow-hosted` crate keeps HTTP and process wiring out of Core
without moving domain invariants out of Core.

## 4. State And Concurrency Assessment

Internal work creation validates immutable bundle and running-run identity.
Discovery uses locked bounded selection. Claims use database-time leases and
increasing fences. Release retains token history, preventing fence-token ABA.
Terminal commit derives the exact request from the locked work item and
atomically stores only the receipt, work item, and lease release.

The worker does not append authoritative workflow events or mutate snapshots.
This is a sound receipt-only no-write concurrency foundation, not runtime
composition.

## 5. API And Authentication Assessment

Authentication is required for every implemented non-liveness route. Remote
work submission and run/event projection are absent. Errors are stable and do
not relay Core or database details.

The one-token trust-domain mechanism is acceptable only for an alpha proof. It
does not satisfy the plan's longer-term issuer, audience, expiry, operator/
worker role, or enterprise authority posture.

## 6. Execution-Provider Assessment

The request/receipt contracts are appropriately payload-free. The no-write
provider rejects side effects, access material, and non-read capability posture
before invocation. It does not recreate evidence or claim sandboxing.

An OpenShell adapter would fit this provider boundary later. A fork is not
justified by the present requirements.

## 7. Privacy And Redaction Assessment

Manual Debug implementations are redacted, token comparison uses a digest and
constant-time equality, hosted references reject unsafe shapes, and transport
errors use fixed messages. No raw provider payload, command output, source
content, credential, authorization header, or access-material value is stored
in the new models.

## 8. Test Quality Assessment

The focused model, API, provider, and conditional live-`PostgreSQL` tests cover
the implemented boundary, including receipt commit, replay after terminal
transition, and monotonic fence reacquisition. Workspace regression and
deployment checks are required at phase close.

The most important missing tests correspond to missing product behavior:
remote run creation, approval proof, cancellation, report retrieval,
pre-invocation attempt persistence, operational metrics, and a full
restart/recovery vertical slice.

The hosted image now uses the locally validated Rust 1.95 builder. The
repository's pre-existing Rust 1.78 compatibility declaration does not match
the current locked dependency graph and needs a separately reviewed
compatibility correction.

## 9. Blockers

1. The API cannot validate/register an immutable bundle and create a governed
   run.
2. Approval retrieval/decision with exact presentation proof is not exposed.
3. Eligible cancellation is not exposed.
4. `WorkReport` and report-artifact metadata are not retrievable.
5. Pre-invocation attempt posture is not durable, so a possibly-effecting
   provider cannot safely use lease takeover.
6. Bounded operational metrics and a complete deployed restart/recovery proof
   are absent.
7. The workspace's declared Rust 1.78 compatibility is not satisfied by the
   current locked dependencies; the hosted image avoids that false build
   posture by pinning Rust 1.95.
8. A governance-derived remote work-creation boundary and unique durable
   invocation identity do not exist; caller-authored work must not be exposed
   remotely before they do.

## 10. Non-Blocking Follow-Ups

- Replace the alpha bearer token with an expiring issuer/audience-bound
  mechanism when the deployment trust model advances.
- Separate the current shared non-superuser API/worker database role.
- Add retention and pagination posture for every future collection.
- Add network/process fault injection.
- Evaluate OpenShell as an optional provider only after the hosted governance
  blockers pass.

## 11. Recommended Next Phase

Execute the **single-tenant hosted alpha runtime-composition blocker fix** as
one integrated phase. Compose existing immutable-bundle, approval,
cancellation, report, event, and authority primitives into the hosted API and
worker rather than introducing another model family.

Provider writes, multi-tenancy, enterprise identity, and production hardening
remain later.
