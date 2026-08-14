# Project-Scoped Approval Route Persistence Model And Store Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The model/store slice stays inside the approved local Core boundary and now
provides a defensible immutable route-history contract. The first review found
four blockers in authority-view construction, immutable-bundle coherence,
aggregate approval-subject coverage, and approval-list input validation. Those
findings are preserved below and were fixed before acceptance.

## 2. Scope Verification

The phase added bounded Core models, a specialized store trait, an in-memory
contract fixture, focused tests, and documentation. It did not add PostgreSQL
storage, HTTP behavior, an approval inbox, decision authority, notification
delivery, provider writes, workflow schema fields, CLI behavior, examples, or
release-posture changes.

The store remains a persistence primitive rather than an authentication
boundary. No route record can grant current approval authority.

## 3. Model Assessment

The logical subject deterministically identifies one project, run, approval,
routing reason, and optional escalation slot. The route decision remains the
existing `ProjectApprovalRoute`; the durable envelope adds a versioned complete
source commitment and first-creation timestamp.

The source commitment binds the exact route identity, pending approval subject,
approval-request event, coherent immutable run-bundle manifest, active project
run binding, escalation proof when required, and deployment authority view.
The record rejects a source commitment built for a different route decision.

## 4. Authority Commitment Assessment

The first review correctly found that accepting an arbitrary principal slice
could not truthfully represent a complete deployment authority view. The fix
introduces `HostedPrincipalRegistry`, a closed organization-scoped,
deterministically ordered, duplicate-free registry. The authority commitment is
constructible only from that registry and cannot be deserialized from an
arbitrary commitment fingerprint.

The model boundary does not itself load the deployment registry. A future
authenticated composer must obtain the registry from the canonical deployment
authority source. That integration is a non-blocking follow-up because this
phase intentionally implements no hosted composer.

## 5. Source And Approval Subject Assessment

The first review correctly found that independently supplied bundle identity,
version, and root hash could be combined without proving one coherent bundle.
The source constructor now accepts one validated `ImmutableRunBundleManifest`
and cross-checks its run, workflow, workflow version, schema version, workflow
content hash, and resolved execution-context hash against the approval.

Step/skill approvals commit their complete typed subject fields. Aggregate
approvals commit the canonical complete `GovernanceApprovalBinding`, including
the nested aggregate assessment and authoritative source binding. Focused tests
prove that nested assessment provenance changes alter the source commitment.

## 6. Store And Replay Assessment

The store contract exposes create, exact read, bounded exact-project recipient
enumeration, and bounded exact-project/run/typed-approval enumeration. It has
no update, delete, or global-list operation.

Exact retries reconcile to the first canonical record and preserve its
`resolved_at` and `created_at`. Any decision or source change for the same
logical subject conflicts without overwrite. The in-memory fixture performs
comparison and insertion under one lock; concurrent identical and conflicting
writer tests cover the expected outcomes.

## 7. Privacy And Error Assessment

The persisted envelope contains identifiers and cryptographic commitments, not
approval reasons, workflow contents, evidence, command output, provider
payloads, escalation messages, contact details, credentials, or authority
inventories. Debug implementations redact identities and commitments. Store,
validation, conflict, and deserialization errors use stable non-leaking codes
or messages.

Typed `ApprovalReferenceId` input now bounds approval enumeration before the
store query boundary. The original raw-string query finding is fixed.

## 8. Test Quality Assessment

Focused tests cover valid construction and serde, logical-subject integrity,
route/source pairing, coherent bundle matching, complete authority-registry
validation, authority ordering and grant sensitivity, aggregate nested
assessment commitment, first-write replay, conflicting provenance, concurrent
writers, exact-scope enumeration, typed approval references, unresolved-route
posture, cross-project isolation, tampering, and Debug/error/serialization
non-leakage.

The in-memory fixture cannot prove PostgreSQL isolation, restart, migration,
backup/restore, or indexed-column corruption behavior. Those tests belong to
the future durable adapter phase.

## 9. Documentation Review

The implementation plan, roadmap, and phase report accurately distinguish the
implemented Core/store contract from deferred database, composer, inbox,
decision, notification, provider-write, schema, CLI, example, and release work.
They do not claim that historical routes are current authority.

## 10. Original Blockers And Resolution

1. **Arbitrary authority slice:** fixed by the validated complete
   `HostedPrincipalRegistry` construction boundary.
2. **Independently combinable bundle fields:** fixed by requiring one coherent
   validated `ImmutableRunBundleManifest` and matching it to the approval.
3. **Aggregate subject/provenance coverage:** fixed by explicit aggregate
   approval tests that alter nested assessment provenance.
4. **Raw approval-list identity:** fixed by accepting validated
   `ApprovalReferenceId` values at the store boundary.

No blockers remain for this model/store phase.

## 11. Non-Blocking Follow-Ups

- integrate the authority commitment with the canonical deployment-owned
  principal registry rather than accepting registry construction at a hosted
  service boundary;
- implement one PostgreSQL create/read adapter with internal migration,
  serializable conflict behavior, restart, backup/restore, and corruption
  tests;
- add an authenticated composer that reconstructs inputs from accepted event,
  bundle, project-binding, and authority stores;
- review cursor design before adding any principal-filtered inbox.

## 12. Recommended Next Phase

Proceed to **canonical deployment-authority commitment integration and
PostgreSQL project approval route store planning**. The next phase should bind
the accepted model to trusted deployment sources and define the durable
transaction boundary before implementing any inbox or route-based decision
path.

Do not add an approval inbox, route-based current authority, external
notifications, provider writes, public schemas, CLI behavior, examples, or
release changes in that phase.

## 13. Governed Review Record

- dogfood workflow: `dg/review`;
- run ID: `run-1786683708734656000-2`;
- approval ID:
  `approval/run-1786683708734656000-2/review-scope-approved`;
- approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof `presentation/d49d7899f81feeae`;
- review status: `Completed`;
- first review outcome: blocker fixes required;
- fix-forward outcome: blockers resolved and focused tests passed.
- validation summary: formatting, workspace clippy, full workspace tests,
  documentation checks, and diff checks passed;
- event summary: 39 events, one approval, zero retries, zero escalations;
- proof enforcement: one persisted presentation record matched the granted
  approval and the approval event trail contains its proof marker.

Repository inspection, review, blocker fixes, tests, documentation, validation,
and git operations are executor work performed outside the kernel. The kernel
governed the review scope and approval; it did not execute those actions.
