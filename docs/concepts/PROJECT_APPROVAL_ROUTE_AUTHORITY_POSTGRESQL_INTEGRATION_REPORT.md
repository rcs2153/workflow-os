# Project Approval Route Authority And PostgreSQL Integration Report

## 1. Executive Summary

The bounded project approval route authority and PostgreSQL integration is
implemented. Collaborative hosted configuration now derives authentication and
the sanitized routing authority view from one credential registry, binds it to
an explicit positive revision, and activates a durable authority high watermark
before collaborative state can be served. Core composes approval routes from
durable events, immutable run bundles, exact project bindings, and the canonical
authority snapshot. PostgreSQL schema v2 persists immutable route history with
transactional mutable-fact rechecks and corruption detection.

This remains an internal, local/CI-oriented governance slice. It does not add an
approval inbox, route-authorized decisions, notifications, automatic route
creation, provider mutations, schemas, CLI behavior, examples, dynamic identity,
enterprise administration, or production storage claims.

## 2. Scope Completed

- Added positive bounded `HostedAuthorityRegistryRevision` and a
  revision-bound `ProjectApprovalAuthoritySnapshotCommitment`.
- Bound the complete authority snapshot into route source provenance.
- Made the hosted credential registry the canonical source for authentication,
  the sanitized Core principal registry, and the authority commitment.
- Added explicit pre-serve authority high-watermark activation.
- Added managed PostgreSQL schema v2 and exact transactional v1-to-v2 migration.
- Implemented dedicated PostgreSQL create, read, recipient-list, and
  approval-list route storage.
- Added Core authenticated composition from stable lookup subjects and durable
  facts, plus hosted wiring that supplies canonical deployment authority.
- Rechecked pending approval history, active exact run binding, and exact
  authority revision/commitment in the create transaction.
- Extended PostgreSQL restart, concurrency, corruption, and recovery coverage.

## 3. Scope Explicitly Not Completed

No hosted approval inbox, route-based decision authority, external notification,
route mutation, automatic route creation, dynamic registry reload, enterprise
identity, provider write, OpenShell execution, workflow schema, SDK, CLI,
example, production TLS/pooling/HA/PITR, or release change was added.

## 4. Authority Boundary

`HostedCredentialRegistry` validates credential-to-principal bindings and
derives one complete sanitized `HostedPrincipalRegistry`. It derives one
content commitment and binds it to a deployment-owned monotonic revision.
`CollaborativeHostedApiConfiguration::activate` must create, replay, or advance
the PostgreSQL high watermark before it can return serving state. Lower
revisions and same-revision content changes fail closed.

Credential material is not present in the sanitized registry, authority
commitment, route record, Debug output, or PostgreSQL route columns.

## 5. Composition And Persistence Boundary

The Core composer reconstructs the pending approval and exact
`ApprovalRequested` event, verifies the run's immutable bundle binding, derives
ownership from the frozen workflow definition, reconstructs exact escalation
history when requested, and requires an active exact-project run binding. It
then resolves and commits the route through the specialized store contract.

The PostgreSQL store uses a bounded serializable transaction to recheck mutable
approval, binding, and authority facts immediately before create-only storage.
Exact retry reconciles to the first record. Conflicting content or provenance
never overwrites it.

## 6. Migration And Integrity Boundary

Schema initialization now separates bootstrap metadata from versioned DDL. A
fresh database installs v1 plus v2 and records v2 only after success. An exact
v1 checksum migrates transactionally. Unknown, newer, mismatched, or
recovery-required state fails closed.

Every route read deserializes and canonically reserializes the record, verifies
the versioned payload hash, and cross-checks every duplicated index column.
Corruption fails the whole operation with a stable non-leaking error.

## 7. Test Coverage

Focused tests cover authority revision validation and tamper rejection,
canonical hosted authority derivation, ordinary and escalation composition,
missing durable facts, cross-project binding, authority divergence, schema
migration, high-watermark replay/advance/rollback/conflict, route
create/read/list/reconcile, concurrent writers, unresolved routes, SQL-boundary
filtering, row tampering, restart, and restored-state integrity.

## 8. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed. The PostgreSQL test target compiled and its
  non-live boundary executed; live bodies skipped because the local environment
  does not provide `WORKFLOW_OS_TEST_POSTGRES_URL`.
- Live PostgreSQL conformance: deferred to the required `Shared PostgreSQL
  State` CI job because this laptop has no PostgreSQL server/client or container
  runtime.
- PostgreSQL backup/restore rehearsal: deferred to the same required CI job for
  the same local-environment reason.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The first live `Shared PostgreSQL State` PR run exposed a test-harness blocker:
the collaborative boundary fixture activated the synchronous PostgreSQL
authority store after entering a Tokio test runtime. The PostgreSQL client
correctly rejected the nested runtime. The fixture now completes schema setup,
project preparation, and authority activation on its dedicated setup thread
before exercising the asynchronous hosted router. No production handler,
database invariant, or merge gate was weakened. The same required CI job must
rerun successfully before merge. Fix-forward verification passed
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, all 37 `workflow-hosted` tests in the non-live local environment,
`npm run check:docs`, and `git diff --check`.

## 9. Remaining Limitations

Routes are durable historical routing evidence only. They do not grant current
approval authority, trigger delivery, or appear through an inbox or public API.
The authority registry is immutable for one hosted state lifetime; dynamic
refresh and revocation require a separately reviewed protocol. PostgreSQL
remains an explicit shared-state preview without production operational claims.

## 10. Recommended Next Phase

Perform the phase-level maintainer and security review. After acceptance, return
to runtime composition priorities without broadening provider mutation families
or treating stored routes as approval authority.
