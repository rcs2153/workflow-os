# Project Approval Route Authority And PostgreSQL Integration Plan Review

## 1. Executive Verdict

**Plan accepted after fix-forward corrections; proceed to one governed
implementation vertical slice.**

The plan now defines a defensible canonical authority source, binds authority
revision and content into route provenance, advances PostgreSQL through an
explicit schema migration, preserves create-only route semantics, and assigns
provider-neutral composition to Core. No planning blocker remains.

## 2. Scope Verification

The plan stays within authority-source, PostgreSQL adapter, authenticated
internal composition, recovery, test, and documentation boundaries. It does not
authorize an approval inbox, route-authorized decisions, notifications,
provider writes, workflow schemas, CLI behavior, examples, dynamic identity,
enterprise administration, or release changes.

## 3. Canonical Authority Assessment

The credential-bearing hosted registry becomes the sole source for both
authentication and the sanitized Core authority registry. Hosted state can no
longer accept independently constructed authentication and authority views.
Credential digests remain excluded from the sanitized registry and authority
commitment.

The plan explicitly distinguishes the hosted credential registry from Core's
same-named sanitized registry. A rename or wrapper is required to prevent type
and review ambiguity.

## 4. Authority Revision And Rollback Assessment

The first review found that a content commitment alone could not prove which
deployment revision declared the content current. That would leave route
provenance ambiguous and make rollback detection impossible.

The fix adds a positive monotonic `u64` authority revision, a complete authority
snapshot commitment, route source binding to both revision and content, and a
PostgreSQL high watermark. Lower revisions and same-revision content changes
fail closed. Exact replay is allowed, and a higher validated revision advances
atomically.

The first review also found activation timing unresolved. The fix requires an
explicit initialization operation after complete deployment configuration
validation and before serving collaborative traffic. Constructors perform no
hidden I/O, and route requests do not lazily activate authority.

## 5. PostgreSQL Migration Assessment

The plan correctly rejects adding route DDL to the current schema-v1 batch
before compatibility metadata is checked. It requires separate metadata
bootstrap, exact v1 recognition, a transactional v1-to-v2 migration under the
existing advisory lock, direct v2 installation for empty databases, and
metadata advancement only after successful DDL.

Unknown, newer, checksum-mismatched, interrupted, and recovery-required states
fail closed. The dedicated route table and high-watermark table are preferable
to the generic upsert-capable records table.

## 6. Store And Reconciliation Assessment

The route table carries bounded relational columns, a canonical payload, and a
payload hash. Dedicated exact-scope indexes support recipient and approval
queries without global enumeration.

Serializable create behavior uses the accepted decision-equivalence helper,
preserves first timestamps, reconciles exact retries, rejects conflicting
content or provenance, and never updates or deletes the first route row.
Concurrent identical and conflicting writer requirements are explicit.

## 7. Integrity And Privacy Assessment

Every read must decode the canonical record, verify canonical serialization and
payload hash, and cross-check every duplicated column. Any mismatch fails the
entire operation with a stable non-leaking corruption error.

The plan excludes credentials, approval reasons, messages, source contents,
evidence, command output, provider payloads, contact details, and grant
inventories. Route records remain historical evidence, not current authority.

## 8. Authenticated Composition Assessment

The first review found composer ownership unresolved. The fix assigns
provider-neutral source reconstruction, validation, route resolution,
commitment, and persistence orchestration to Core. `workflow-hosted` supplies
credential validation, canonical authority snapshots, explicit activation, and
deployment wiring. This preserves dependency direction and prevents hosted code
from duplicating Core governance semantics.

The transaction rechecks the mutable approval, project-binding, and authority
high-watermark facts. Immutable bundle content remains governed by its existing
immutable contract. No HTTP endpoint is added.

## 9. Test And Recovery Assessment

The planned tests cover authority/authentication coherence, rollback,
same-revision conflicts, schema installation and migration, exact replay,
concurrency, cross-project isolation, row corruption, stale mutable facts,
restart, and backup/restore. The recovery rehearsal includes routed and
unresolved records plus the authority high watermark.

The plan correctly avoids converting logical backup/restore proof into an HA,
PITR, RPO, RTO, or production-readiness claim.

## 10. Original Planning Blockers And Resolution

1. **Authority revision absent from route provenance:** fixed by the complete
   authority snapshot commitment and explicit route source binding.
2. **Authority activation timing unresolved:** fixed by explicit validated
   pre-serve activation and no lazy first-request advancement.
3. **Composer ownership unresolved:** fixed by Core-owned provider-neutral
   composition and hosted-owned deployment authority wiring.

No blockers remain.

## 11. Non-Blocking Follow-Ups

- dynamic authority refresh and revocation need a later protocol;
- unresolved-route operator visibility remains audit-only;
- automatic route creation remains deferred;
- a principal-filtered hosted inbox requires separate cursor, authority, and
  non-disclosure planning after this implementation is accepted.

## 12. Recommended Next Phase

Implement the canonical authority snapshot, authority high watermark,
PostgreSQL schema-v2 migration, route store, Core-owned authenticated composer,
and recovery proof as one governed vertical slice.

Do not add an inbox, route-authorized decisions, notifications, provider writes,
public schemas, CLI behavior, examples, dynamic identity, or release changes.

## 13. Governed Review Record

- dogfood workflow: `dg/review`;
- run ID: `run-1786688648197351000-2`;
- approval ID:
  `approval/run-1786688648197351000-2/review-scope-approved`;
- approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof `presentation/8e0e9599d8b207d3`;
- review status: `Completed`;
- validation required: `npm run check:docs` and `git diff --check`;
- out-of-kernel work: repository inspection, review analysis, documentation
  edits, and validation are executor work; the kernel governed scope and
  approval but did not execute those actions.
