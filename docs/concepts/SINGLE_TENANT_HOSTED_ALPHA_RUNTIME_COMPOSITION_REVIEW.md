# Single-Tenant Hosted Alpha Runtime Composition Review

Review date: 2026-07-29

## 1. Executive Verdict

**Runtime-composition hardening accepted; complete hosted alpha remains
blocked.**

The implementation closes the immutable-store, durable-attempt,
fence-preserving renewal, bounded inspection, approval-proof, cancellation,
report-metadata, and operational-posture gaps without enabling provider writes.
It does not yet close the governance-derived dispatch and authoritative result
projection gap. That remaining boundary is specific and must be the next hosted
phase.

## 2. Scope Verification

The phase remains single-tenant, local-evaluation, and no-write. It does not
add OpenShell, general tool execution, a provider mutation family,
multi-tenancy, enterprise identity, a hosted UI, workflow schema changes,
examples, or production claims.

The server-owned project root and internal provider boundary prevent callers
from supplying paths, work items, or provider requests.

## 3. Core Composition Assessment

The new immutable-store trait is narrow and preserves the existing executor as
the owner of bundle validation and binding. The local store remains compatible,
and `PostgreSQL` can satisfy the same create-only exact-read contract.

This is the correct way to compose hosted run creation. Duplicating immutable
bundle invariants inside HTTP or worker code would have created a second source
of truth.

## 4. API Assessment

Run creation, read projection, bounded event pages, exact approval requests,
idempotency-bound proof-enforced approval decisions, eligible cancellation,
and terminal report-artifact metadata are useful evaluation surfaces.

Every route requires authentication except liveness. Request bodies are
bounded, transport errors are fixed, and the project root comes from server
configuration.

The static bearer identity remains a blocker to production mutation. It lacks
issuer, audience, expiry, operation scope, and human/service role semantics.
The routes are acceptable only inside the documented single administrative
trust domain.

## 5. Worker And Attempt Assessment

Durable invocation identity and attempt posture now precede provider
invocation. Exact request/provider/configuration binding, fenced transitions,
reconciliation-required posture, and atomic terminal commit prevent a lease
takeover from blindly repeating a possibly started invocation.

The worker rehydrates current run state and bundle identity before invocation.
Cancellation before invocation fails closed.

The remaining P0 is orchestration ownership: a work item is not yet atomically
derived from a scheduled authoritative skill invocation, and a terminal receipt
is not projected back into the exact invocation and run. The implementation
correctly avoids fabricating `SkillInvocationSucceeded`.

## 6. Privacy And Redaction Assessment

The new models store stable identities, hashes, statuses, timestamps, and
bounded counts. They do not store raw provider payloads, logs, command output,
spec contents, source contents, environment values, credentials, authorization
headers, private keys, or token-like values.

Debug output and API errors remain redaction-safe. Report transport is
metadata-only.

## 7. OpenShell Assessment

The provider interface remains the correct future OpenShell boundary.
OpenShell should be optional and upstream-tracked, not forked or activated
during this phase.

A future adapter must provide exact sandbox identity, provider/configuration
hash, timing, exit posture, denied-action references, log/artifact references,
and reconciliation posture. It must not become the authority source or append
Core events directly.

## 8. Test And Operational Assessment

Focused tests cover immutable-store replay, attempt lifecycle, receipt
substitution, fenced transitions, renewal, terminal commit, metrics, API
authentication, provider rejection, and existing executor/state behavior.

The restart rehearsal is useful evaluation evidence. It does not substitute
for fault injection, role separation, backup/restore proof with hosted attempts,
or production operational validation.

## 9. Blockers

1. Atomically derive hosted work from the exact authoritative scheduled skill
   invocation and immutable bundle.
2. Atomically project the terminal hosted result into the exact invocation and
   run state without fabricating success.
3. Replace the static alpha actor/token posture before production mutation.
4. Prove deployed recovery for durable attempts and reconciliation-required
   outcomes.

## 10. Non-Blocking Follow-Ups

- Move event pagination into a bounded database query.
- Separate API and worker database privileges.
- Add external low-cardinality metrics export only after a reviewed telemetry
  boundary.
- Correct the repository-wide Rust compatibility declaration separately.

## 11. Recommended Next Phase

Proceed directly to the **Core-owned atomic hosted dispatch and result
projection path**, still using only the deterministic no-write provider.

Do not broaden into OpenShell, provider writes, additional mutation families,
multi-tenancy, or enterprise administration first.
