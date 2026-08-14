# Collaborative Team Beta Project Boundary Review

## 1. Executive Verdict

**Phase accepted pending required live PostgreSQL CI proof.** The model, store,
hosted API/worker, catalog, audit, security hardening, and local test boundary are
coherent and remain within the accepted collaborative-beta scope. Merge remains
blocked until the repository's live PostgreSQL jobs pass.

## 2. Scope Verification

The implementation stayed within one organization, fixed project registration,
pre-provisioned principals, project-scoped hosted access, PostgreSQL bindings,
and project catalog state. It did not add production multi-tenancy, enterprise
identity, dynamic administration, notifications, provider mutations, hosted
SaaS claims, schemas, or release changes.

## 3. Model And Configuration Assessment

The Core vocabulary is closed and domain-neutral. Duplicate grants and
capabilities fail closed; deserialization reconstructs through validated
constructors; project roots and caller identities are redacted from Debug. The
deployment registries reject duplicate/nested roots, unknown grants, duplicate
principals, and duplicate credential digests.

## 4. Authorization And Isolation Assessment

Every collaborative route authenticates and authorizes before state access.
Run-owned reads require exact active project bindings. Cross-project scope is
not disclosed. Authenticated wrong-scope decisions are recorded with bounded
references. Project roots are server-owned, and hosted creation validates the
loaded manifest project against route scope. The legacy alpha API now rejects
any run-owned resource with a collaborative project binding, preventing it from
becoming an alternate unscoped read or mutation path.

## 5. Filesystem Assessment

The shared loader now rejects absolute and parent-traversing layout paths plus
symlinked manifests, directories, intermediate layout components, and spec
files. This closes the material path-escape issue found during review.

## 6. PostgreSQL And Worker Assessment

Run reservations cannot authorize reads. Dispatch atomically activates a run
and publishes its work item. Terminal receipt/report bindings are atomic with
the authoritative terminal projection. Collaborative workers require active
run/work-item bindings; legacy compatibility is explicit through the original
worker constructor.

## 7. Catalog Assessment

Catalog versions are immutable and scoped. Publication requires owner,
escalation, authenticated publisher identity, and a complete matching approved
stewardship record. Project listing queries only the scoped prefix and rejects
any decoded scope mismatch. The required live PostgreSQL milestone covers exact
publication replay plus project A read/list and project B non-discovery.

## 8. Idempotency Assessment

Run idempotency binds organization, project, actor, and run identity. Resource
binding replay is exact by scope and identity. Approval, cancellation, and
catalog mutation duplicates fail closed after the first intent; this prevents
concurrent double execution but does not yet replay a completed response.

## 9. Privacy And Error Assessment

Errors use stable bounded codes and omit bearer material, roots, hidden resource
detail, provider payloads, and caller text. Access records are payload-free.
Debug output redacts scopes, principals, resource IDs, and catalog identities.

## 10. Test Quality Assessment

Focused tests cover the major model, loader, route, binding, and restart
contracts. The live two-project/two-principal test is wired into the required
PostgreSQL CI job and includes two-actor approval, cross-project denial,
authorization-audit readback, and legacy-route rejection. The shared live
PostgreSQL test covers catalog isolation and binding restart. Acceptance must be
revisited if those jobs fail; skipped local database tests are not accepted as
isolation evidence.

## 11. Blockers

- Live PostgreSQL collaborative boundary and shared-state CI must pass before
  merge.

## 12. Non-Blocking Follow-Ups

- Store and replay completed mutation responses rather than returning a
  duplicate-intent conflict.
- Add dedicated collaborative deployment packaging after the API contract
  stabilizes.
- Add bounded administrative diagnostics for orphaned reservations.

## 13. Recommended Next Phase

After CI acceptance, proceed to **ownership, escalation, approval routing, and
bounded notifications**. The project boundary is the prerequisite for that
collaboration behavior; broader providers and dynamic identity remain later.
