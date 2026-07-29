# Shared PostgreSQL State Review

Review date: 2026-07-29

## 1. Executive Verdict

**Needs mandatory PostgreSQL CI proof before phase acceptance.**

The implementation is within scope and the reviewed code now covers the
accepted semantic boundary. The complete local workspace, documentation,
integration, Rustdoc, metadata, dependency-audit, skip-path, and diff checks
pass. The phase cannot be accepted until the mandatory live `PostgreSQL`
concurrency test and backup/restore rehearsal pass in CI.

This is an external proof requirement, not authorization to weaken or skip the
database tests.

## 2. Scope Verification

The implementation stays within the explicit shared-state milestone. It adds
no hosted API, automatic worker, automatic backend selection, migration,
multi-tenancy, enterprise identity, provider mutation expansion, schema/SDK
surface, examples, or production-readiness claim.

## 3. Architecture Assessment

The connection factory keeps credentials, transport, pooling, and timeouts out
of durable domain state. The synchronous client matches the existing store
interfaces and avoids introducing an async runtime into Core.

The backend is explicit and opt-in. Domain semantics remain in Core types and
validation rather than becoming arbitrary SQL transaction APIs.

## 4. Schema And Compatibility Assessment

The schema separates append-only events, revisioned governed records,
idempotency, leases, and immutable content. Schema metadata binds version,
checksum, and recovery-required posture under a migration advisory lock.

Newer, mismatched, and recovery-required posture fails closed. Canonical
payload deserialization and identity checks prevent relational indexes from
silently becoming a second source of truth.

## 5. Transaction-Family Assessment

All seven Core transaction families have explicit APIs. Approval decisions
require durable pending state, matching presentation proof, a matching proof
marker, event binding, and expected-revision projection update in one
transaction.

Pre-effect intent and post-effect outcome remain separate transactions around
the external provider call. This correctly avoids claiming impossible
cross-system atomicity.

## 6. Concurrency And Retry Assessment

Serializable transactions retry only database serialization and deadlock
conflicts within a bounded budget. Domain errors remain visible.

The focused integration test races event append, idempotency intent, approval
decision, SideEffect transition, and immutable-bundle publication. CAS and
fence checks reject stale writers. These tests are structurally appropriate,
but their correctness claim remains pending live CI execution.

## 7. Lease And Consumer Assessment

Lease acquisition uses database time and increasing fencing tokens. Tests
cover live contention, renewal, expiry, takeover, stale release, and stale
fenced commit. This addresses abandoned-worker takeover without depending on a
process-local lock.

The shared consumer is deliberately narrow: one supplied event under one run
lease. It does not schedule, poll, execute providers, or run automatically.
Failure before release leaves takeover to lease expiry, which is documented.

## 8. Integrity And Recovery Assessment

Projection rebuild uses authoritative events and expected snapshot revisions.
The recovery rehearsal uses an isolated restored database and verifies schema,
rebuild, and immutable bundle readability.

The script is bounded and credential-conscious. It is not a production DR
system and the docs do not present it as one.

## 9. Privacy And Error Assessment

The backend and test `NoTls` factory redact connection posture in `Debug`.
Stable errors omit SQL, connection details, payloads, and caller values.
Adversarial payload/checksum tests specifically verify secret-like values do
not leak through errors.

Server-side database logging remains an operator concern and is documented.

## 10. Test Quality Assessment

The test design covers the milestone's load-bearing behaviors and keeps live
database tests mandatory in CI while allowing ordinary local development
without a daemon.

Remaining useful later tests include process termination during a long-lived
worker operation, multiple PostgreSQL major versions, network fault injection,
pool recycling, and physical/PITR recovery. They are not substitutes for the
current mandatory CI proof.

## 11. Documentation Assessment

The plan, runtime guide, recovery runbook, roadmap, known limitations, report,
and review distinguish:

- implemented shared-state preview;
- explicit opt-in selection;
- mandatory live conformance;
- no hosted worker or API;
- no automatic migration;
- no production TLS/pooling/HA/PITR/SLO;
- no multi-tenancy or enterprise identity;
- no provider mutation expansion;
- no production-readiness claim.

## 12. Blockers

One blocker remains:

- the `Shared PostgreSQL State` CI job must pass both live concurrent
  conformance and logical backup/restore integrity rehearsal.

Any runtime SQL, transaction, concurrency, schema, or recovery failure in that
job is a phase blocker.

## 13. Non-Blocking Follow-Ups

- Add a reviewed production TLS/pooling factory outside Core.
- Exercise another maintained PostgreSQL major version.
- Add network/process fault injection for worker takeover.
- Define operational metrics for transaction retries and lease contention.
- Preserve the external evaluator priority: use proportional governance to
  reduce low-risk ceremony without weakening durable evidence or authority.

## 14. Recommended Next Phase

Do not start another roadmap phase yet. Complete the mandatory CI proof, fix
any blocker, update this review to the final verdict, and merge this milestone.

After acceptance, proceed to **single-tenant hosted alpha planning**. Do not
broaden provider mutations or infer collaborative product readiness first.

## 15. Validation

Completed locally:

- focused test compile;
- focused strict Clippy;
- focused no-database skip behavior;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`;
- `cargo metadata --locked --format-version 1`;
- `cargo audit`;
- `npm audit --audit-level=moderate`;
- diff checks.

Pending:

- live `PostgreSQL` conformance;
- backup/restore integrity rehearsal.
