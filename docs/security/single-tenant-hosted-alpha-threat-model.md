# Single-Tenant Hosted Alpha Threat Model

Status: focused implementation review for an evaluation alpha

This document narrows the general [Threat Model](THREAT_MODEL.md) to the
`workflow-hosted` proof. It is not a certification, penetration test, or
production security claim.

## Assets And Trust Boundary

Protected assets are authoritative run events, snapshots, immutable bundles,
approval records, `SideEffect` records, execution receipts, and API
authentication material.

The first boundary assumes one operator-controlled deployment and one
administrative trust domain. `PostgreSQL`, the API process, and worker
processes are trusted deployment components. The execution provider is less
trusted and receives only an exact validated request.

## Addressed Threats

| Threat | Current control |
| --- | --- |
| Anonymous API read | Bearer authentication on every non-liveness route |
| Token recovery from Debug or durable state | Hash-only in-memory comparison and redacted Debug |
| Oversized request amplification | 64 KiB body limit |
| Work against a substituted bundle | Exact bundle ID, version, root, workflow, and run validation |
| Work detached from governed state | Core derives dispatch from an authoritative scheduled invocation and atomically commits invocation events with the queued work item |
| Caller substitutes workflow source or hosted work | Run creation uses one server-owned project root; no work-item or provider-request submission route exists |
| No-op receipt fabricates workflow execution | Only the Core-owned exact-binding projection may append terminal invocation/run events; receipt-only storage is not execution proof |
| Duplicate or conflicting internal submission | Durable caller-key idempotency intent binding |
| Competing workers | Database-time lease, monotonic fence, and locked discovery |
| Stale worker commit or fence ABA | Fence validation inside the atomic terminal transaction and retained monotonic token history |
| Worker invokes after terminal cancellation | Worker rehydrates authoritative run state immediately before provider invocation and commits pre-start cancellation under the active fence |
| Worker loss after a possible invocation | Durable invocation/attempt posture is committed before provider invocation; ambiguous attempts are not blindly retried |
| Request rejected before provider start leaves the run falsely active | Core-owned atomic rejection projection fails the invocation and run, terminalizes the work item, releases the lease, and creates no attempt or receipt |
| Provider uncertainty becomes false failure or success | Core-owned atomic reconciliation projection marks the attempt reconciliation-required, makes the work item ambiguous, and escalates the run; exactly bound ambiguous receipts also escalate |
| Provider receipt substitution | Request fingerprint, provider identity/version/configuration, and policy hash validation |
| Credential or access-material persistence | Current provider rejects all access-material references |
| External mutation | Current provider rejects `SideEffect` and non-read capabilities |
| Raw output leakage | Receipts contain stable references and bounded categories, not payloads |
| Path or URL injection through references | Hosted references reject paths, traversal, URLs, controls, and secret-like values |

## Known Open Risks

- The alpha bearer token has no issuer, audience, expiry, or per-operation role
  semantics. Hosted mutation routes are evaluation-only inside one deployment
  trust domain and are not production authority.
- API and worker use a non-superuser database role, but their identities and
  privileges are not yet separated from each other.
- TLS termination and network policy are deployment responsibilities.
- Approval and cancellation mutations rely on the current one-actor API trust
  domain. They preserve Core proof and policy checks, but do not supply
  enterprise issuer, audience, expiry, role, or separation semantics. Their
  idempotency keys are durably bound to payload-free intent hashes, so
  conflicting key reuse fails closed.
- Hosted work-item creation remains an internal Core API. Remote caller-authored
  work remains absent. The implemented dispatch/result path is limited to one
  payload-free terminal skill using the deterministic no-write provider.
- The worker persists pre-invocation attempt posture and refuses blind retry
  after a possibly started invocation. Provider mutation still requires a
  dedicated reconciliation and cancellation review.
- No access-material resolver exists. Adding one requires time-of-use
  authorization, expiry/revocation handling, process-memory containment, and
  non-leakage proof.
- Operational metrics and traces are not yet exported.
- Bounded in-service metrics are inspection posture, not a production metrics
  export or SLO.
- The no-write provider remains deterministic and payload-free. A future
  execution provider must use the durable invocation identity and attempt
  record; it may not manufacture a replacement identity.
- Dependency/image supply-chain controls are limited to existing lockfiles,
  CI dependency checks, and the pinned major container images in the
  evaluation topology.

## Required Before Provider Mutation

Before any hosted provider can write:

1. persist pre-invocation intent and attempt posture;
2. prove current policy, capability, evidence/check, approval, and authority
   at time of use;
3. provide a scoped access-material resolver;
4. preserve provider idempotency and reconciliation references;
5. prevent blind retry after any possibly-started invocation;
6. add least-privilege deployment identities and reviewed network policy;
7. complete a dedicated provider threat review and failure-injection proof.

NVIDIA OpenShell or another sandbox may later implement the execution-provider
interface as an optional containment adapter. Workflow OS should not fork or
activate it in this phase. A future adapter must return bounded sandbox,
configuration-hash, status, denied-action, log-reference, artifact-reference,
and reconciliation posture without raw output or credentials. Containment does
not replace Workflow OS authority, approval, evidence, SideEffect, idempotency,
or reporting requirements.
