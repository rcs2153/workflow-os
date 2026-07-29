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
| Work detached from governed state | Existing `Running` bundle-backed snapshot required |
| Caller forges governed execution through the API | No remote work-submission or run-mutation route is exposed |
| No-op receipt fabricates workflow execution | Receipt commit cannot append run events or mutate snapshots |
| Duplicate or conflicting internal submission | Durable caller-key idempotency intent binding |
| Competing workers | Database-time lease, monotonic fence, and locked discovery |
| Stale worker commit or fence ABA | Fence validation inside the atomic terminal transaction and retained monotonic token history |
| Provider receipt substitution | Request fingerprint, provider identity/version/configuration, and policy hash validation |
| Credential or access-material persistence | Current provider rejects all access-material references |
| External mutation | Current provider rejects `SideEffect` and non-read capabilities |
| Raw output leakage | Receipts contain stable references and bounded categories, not payloads |
| Path or URL injection through references | Hosted references reject paths, traversal, URLs, controls, and secret-like values |

## Known Open Risks

- The alpha bearer token has no issuer, audience, expiry, or per-operation
  role semantics.
- API and worker use a non-superuser database role, but their identities and
  privileges are not yet separated from each other.
- TLS termination and network policy are deployment responsibilities.
- The API does not yet expose proof-enforced approvals, cancellation, report
  retrieval, or current-authority reassessment.
- Hosted work-item creation is currently an internal Core API. Remote creation
  must remain absent until runtime composition derives the exact request from
  an approved immutable run rather than trusting caller-authored work.
- The worker does not yet persist a pre-invocation attempt marker. Lease
  takeover is safe only because the implemented provider is no-write and
  deterministic. A provider that might cause an external effect must not be
  added until ambiguous-attempt and reconciliation semantics are durable.
- No access-material resolver exists. Adding one requires time-of-use
  authorization, expiry/revocation handling, process-memory containment, and
  non-leakage proof.
- Operational metrics and traces are not yet exported.
- Different caller idempotency keys can describe the same inert request. This
  proof permits that only because the worker is hard-bound to the deterministic
  no-write provider. Future execution needs one durable invocation identity.
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
interface. It would add containment; it would not replace these governance
requirements.
