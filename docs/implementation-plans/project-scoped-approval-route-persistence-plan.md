# Project-Scoped Approval Route Persistence Plan

Status: model/store contract implemented; focused maintainer review required

## 1. Executive Summary

The project-scoped approval route Core model is implemented and accepted. It
can resolve one payload-free route from explicit trusted inputs, but it does not
yet prove where those inputs came from or preserve the result as durable hosted
project state.

The next implementation should add one authenticated composition-and-
persistence path. That path must reconstruct approval, ownership, escalation,
project-binding, and authority context from their accepted sources of truth,
derive the existing `ProjectApprovalRoute`, and create one immutable route
record. Exact retries may reconcile to the first record. A conflicting record
for the same logical route subject must fail closed.

This plan does not implement persistence. It does not authorize a hosted inbox,
approval decisions based on routes, route reassignment, external notifications,
provider writes, public schemas, or runtime execution changes.

## 2. Goals

- authenticate route inputs from durable or immutable sources;
- preserve the accepted Core resolver as the routing decision boundary;
- bind the complete approval subject, immutable run bundle, approval event,
  optional escalation event, project binding, and authority view to a durable
  record;
- define create-only, restart-safe, concurrency-safe persistence;
- reconcile decision-equivalent retries without changing the first record;
- reject conflicting records for the same logical route subject;
- persist routed and unresolved outcomes so absence never ambiguously means
  `not evaluated`;
- preserve routes as routing history rather than approval authority;
- support exact project and recipient indexes without scanning stored payloads;
- keep records, errors, `Debug`, and audit posture payload-free and non-leaking;
- define focused PostgreSQL, restart, tamper, and reconciliation tests.

## 3. Non-Goals

This planning phase does not authorize:

- persistence code, database migrations, or schema version changes;
- a hosted approval inbox or HTTP endpoint;
- approval decision enforcement based on a route;
- route reassignment, mutation, deletion, supersession, or repair;
- recipient fallbacks, delegation, quorum, or separation-of-duties changes;
- approval expiry scheduling or escalation timers;
- dynamic identity, groups, roles, directories, OIDC, SSO, or SCIM;
- email, Slack, Teams, SMS, paging, tickets, webhooks, or delivery receipts;
- provider writes or notification `SideEffect` execution;
- workflow YAML or public schema changes;
- CLI behavior, examples, hosted production claims, or release changes.

## 4. Existing Contracts To Reuse

The implementation should reuse, not duplicate:

- `ProjectApprovalRoute`, `ProjectApprovalRouteInput`, and
  `resolve_project_approval_route` from `project_approval_routing.rs`;
- `EventLogStore::read_events` and `WorkflowRun::rehydrate` for authoritative
  approval and escalation history;
- `WorkflowRunIdentity::immutable_run_bundle` for the run's frozen definition
  reference;
- `ImmutableRunBundleStore::read_exact_bundle` and
  `ImmutableRunBundleCanonicalDefinition::as_workflow` for immutable ownership;
- `PostgresStateBackend::read_hosted_project_resource_binding` for the active
  exact-project run binding;
- the deployment-owned `HostedPrincipalRegistry` and
  `HostedPrincipalBinding` view for current project authority;
- existing stable event, bundle, scope, run, approval, escalation, workflow,
  and actor identifiers.

An approval-store projection or current workflow file is insufficient. The
existing regression that an approval projection without an approval event does
not authorize a decision is the required precedent.

## 5. Authenticated Composition Boundary

The public composer input should contain stable lookup subjects, not caller-
authored domain records:

- exact `HostedProjectScope`;
- `WorkflowRunId`;
- approval ID;
- optional escalation ID when escalation routing is requested;
- an explicit resolution timestamp;
- references to the event log, immutable bundle store, project-binding store,
  deployment authority view, and route store.

It must not accept caller-authored `OwnershipMetadata`, `ApprovalRequest`,
`EscalationRecord`, `ProjectApprovalRoutingReason`, `HostedPrincipalBinding`
collections, or a preconstructed `ProjectApprovalRoute` as durable truth.

The composer should perform these steps in order:

1. Read the exact run event history and rehydrate `WorkflowRun`.
2. Require the selected approval to exist, remain pending, and match the run's
   workflow, schema, version, spec hash, and resolved execution context.
3. Identify the exact `ApprovalRequested` event that created the subject.
4. Require an immutable run bundle binding; legacy or unbound runs fail closed.
5. Read the exact bundle and require its manifest and binding to match the
   rehydrated run identity.
6. Extract ownership only from the canonical workflow definition in that
   bundle.
7. For escalation routing, require exactly one matching durable
   `EscalationTriggered` event and derive the escalation subject from it.
8. Require the active exact-project resource binding for the run.
9. Read the bounded deployment-owned authority view and derive its canonical
   registry commitment without copying grant inventories into the route
   record.
10. Invoke `resolve_project_approval_route` with those authenticated inputs.
11. Construct and create the durable route record in one persistence boundary.

Ordinary routing must not carry an escalation subject. Escalation routing must
not be selected merely because the caller supplied a routing enum.

## 6. Complete Approval Subject Commitment

The existing route identity commits to stable route fields, but durable
persistence needs a stronger source commitment. A route record must bind:

- schema version;
- workflow ID and workflow version;
- spec content hash;
- resolved execution context hash;
- run ID and approval ID;
- validated approval subject kind and its stable step, skill, or aggregate
  identity;
- for an aggregate subject, the complete canonical
  `GovernanceApprovalBinding`, including its exact
  `GovernanceAssessmentBinding`, rather than only its binding ID;
- approval-request event ID;
- immutable run bundle ID and manifest/content commitment;
- exact project scope and active run-binding revision or commitment;
- optional escalation ID and escalation event ID;
- routing reason;
- authority-view commitment used during resolution.

This should be a validated content-derived
`ProjectApprovalRouteSourceCommitment` or equivalent bounded model. Do not copy
approval reasons, workflow contents, escalation messages, policy payloads,
grant inventories, or source content.

The authority-view commitment is not an open-ended caller string. The hosted
deployment boundary must derive a versioned canonical content hash when it
constructs the immutable `HostedPrincipalRegistry`. The hash input includes the
organization scope and the complete, deterministically ordered principal
bindings and project capability grants, but excludes authentication-token
digests. The route record stores only the commitment version and hash. A future
dynamic authority registry must add an explicit revision and revalidation
contract before it can replace this immutable snapshot posture.

## 7. Candidate Durable Record

The smallest durable envelope should be conceptually equivalent to:

```text
ProjectApprovalRouteRecord
  record_version
  route
  logical_subject_id
  source_commitment
  created_at
```

`logical_subject_id` must identify the route slot independently of the route
outcome. It should commit to exact project scope, run, approval, routing reason,
and optional escalation ID. It must not include recipient, route status, current
authority, or `resolved_at`, because a changed outcome for the same slot is a
conflict rather than a second coexisting route.

The existing `ProjectApprovalRouteId` remains the decision content identity.
The persisted record must cross-check both identities on every read.

`created_at` is the timestamp from the first successful creation. A later retry
with the same decision and source commitments must return the stored record
rather than changing its timestamp.

## 8. Store Interface

Add a specialized `ProjectApprovalRouteStore` interface rather than route
methods on `ApprovalStore`. Approval requests are event-backed runtime
projections; routes are immutable hosted-project records with different
identity and conflict semantics.

The minimum interface should support:

- create one route record;
- read one record by exact logical subject;
- list routed records by exact project and exact recipient with strict bounds;
- list records for one exact run and approval for reconciliation or operator
  diagnostics.

The create result should distinguish:

- `Created`;
- `ReconciledExisting`;
- conflict as a stable error.

The interface must not expose update, delete, global list, cross-project list,
or arbitrary payload-query methods.

This interface is a storage primitive, not an authentication boundary. It must
not be exposed directly through hosted HTTP or accepted as proof that source
records were authenticated. The future hosted composer owns the only
user-facing create path and must produce the validated write request from
trusted source stores in the same call.

## 9. Backend Scope

Project approval routing is part of the collaborative hosted project boundary.
The first production adapter should therefore be PostgreSQL, behind the store
interface. Do not force unrelated local-filesystem or SQLite schema changes
merely to claim parity for a capability those backends do not expose.

Use an in-memory contract fixture where useful to prove store semantics. Any
future backend that claims project-scoped collaborative routing must pass the
same contract suite before support is documented.

PostgreSQL should use a dedicated table or equivalently explicit relational
columns and indexes. Do not implement recipient/project enumeration by scanning
JSON payloads or by globally listing and filtering records in memory.

## 10. Create-Only And Reconciliation Semantics

Source authentication and durable insertion use one explicit two-boundary
protocol rather than claiming that independent stores and an in-memory
deployment registry share one literal database transaction.

Before the PostgreSQL transaction, the hosted composer must:

1. reconstruct the immutable approval-request event and exact immutable run
   bundle through their trusted stores;
2. validate the complete approval subject and bundle commitments;
3. resolve against the immutable deployment registry and capture its canonical
   authority commitment; and
4. construct a short-lived authenticated write request whose fields are not
   accepted directly from an HTTP caller.

The immutable bundle and deployment registry cannot change during this call.
The mutable facts are then rechecked by the PostgreSQL transaction that owns
them. Within one serializable PostgreSQL transaction:

1. Lock the logical route subject.
2. Recheck from PostgreSQL-backed durable run history that the approval remains
   pending and still matches the authenticated approval-request event.
3. Recheck the active exact-project run binding and its revision or commitment.
4. Require the authenticated write request to carry the same immutable bundle
   and authority commitments validated by the composer.
5. Create the record when no subject exists.
6. Return the stored record when the existing record has the same route ID and
   the same complete source commitment.
7. Fail closed when the same logical subject has different route content,
   source commitment, indexed columns, or canonical payload.

If the backend cannot recheck both mutable PostgreSQL facts in one transaction,
authenticated composition must not be wired to that backend. A source change
between pre-transaction reconstruction and insertion yields a stable conflict
or stale-source error; it must never reconcile as an exact retry.

The implementation must never overwrite, revise, or delete the first record.
Concurrent identical writers must produce one creation and one reconciliation.
Concurrent conflicting writers must not both commit.

## 11. Resolved Timestamp Rule

`ProjectApprovalRouteId` intentionally excludes `resolved_at`, while route
serialization includes it. Persistence must therefore define decision-
equivalent replay explicitly:

- the first persisted `resolved_at` is canonical;
- a retry may supply a later timestamp;
- if route identity and authenticated source commitment match, return the
  stored first record;
- raw serialized payload equality is not the replay rule;
- changing any decision-relevant or source field is a conflict.

This rule must be encoded in one named comparison helper and tested directly.

## 12. Routed And Unresolved Outcomes

Persist both routed and unresolved outcomes. This lets the system distinguish:

- route resolution was never attempted;
- resolution was attempted but immutable metadata was missing;
- resolution was attempted but exact-project authority was unavailable;
- resolution produced an authorized recipient at that time.

Unresolved records must never appear in recipient inbox enumeration. Their
operator visibility remains deferred and must not leak actors, projects, or
hidden resource existence.

If authority or metadata later changes, the first slice returns a conflict for
the same logical subject rather than silently rewriting history. Re-evaluation,
supersession, and reassignment require a later append-only model.

## 13. Route Truth Is Not Authority

A durable route proves only what Workflow OS resolved from authenticated inputs
at one point in time. It does not grant or preserve approval authority.

Every future inbox read and approval decision must independently require:

- exact organization and project binding;
- authenticated principal and recipient match where applicable;
- current `ApprovalRead` or `ApprovalDecide` capability;
- current pending approval state;
- existing approval-presentation proof;
- resolved-context and immutable-run integrity.

Authority revocation must take effect even when an older route remains durable.
No decision path may treat route existence, ownership metadata, or prior
authority commitment as current authority.

## 14. Read, Index, And Integrity Rules

PostgreSQL storage should index bounded columns for:

- logical subject ID;
- route ID;
- organization and project;
- run and approval;
- recipient when routed;
- status;
- routing reason;
- optional escalation ID.

Every read and list operation must decode the canonical record and cross-check
all indexed values. A mismatch is corruption and fails closed.

Enumeration must have stable ordering, a strict maximum count, exact-scope
predicates at the query boundary, and no global-list-then-filter path.

## 15. Error, Privacy, And Redaction Posture

- Use stable route-store and composition error codes.
- Do not include project, organization, run, approval, escalation, actor,
  workflow, path, reason, payload, database, or credential values in errors.
- Keep `Debug` redacted for routes, records, commitments, and store outcomes.
- Generic deserialization and corruption errors must not echo stored content.
- Persist identifiers and cryptographic/source commitments only.
- Do not persist approval reasons, failure text, suggested actions, evidence,
  source snippets, command output, provider payloads, contact details, access
  tokens, or principal grant inventories.
- Project-not-found and unauthorized posture must preserve existing hosted
  non-disclosure semantics.

## 16. Test Plan

Future implementation tests must prove:

1. authenticated ordinary route creation from event history and immutable run
   bundle;
2. authenticated escalation route creation from the exact escalation event;
3. approval-store projection without an approval event cannot create a route;
4. live workflow metadata drift cannot change the frozen run route;
5. legacy or mismatched immutable bundle state fails closed;
6. wrong or missing project binding fails closed;
7. complete approval-subject commitment changes when a subject field changes;
8. aggregate-subject commitment changes when any nested assessment field
   changes;
9. authority commitment is deterministic, excludes credential digests, and
   changes when a relevant principal binding or capability grant changes;
10. approval, project binding, or authority changes between source
    reconstruction and insertion fail closed;
11. exact duplicate creation returns `ReconciledExisting`;
12. later `resolved_at` reconciles to the first stored record;
13. conflicting content or provenance for one logical subject fails closed;
14. concurrent identical writers create one record;
15. concurrent conflicting writers cannot both commit;
16. PostgreSQL restart recovers the same canonical record;
17. backup/restore preserves records, indexes, and integrity checks;
18. indexed-column or payload tampering fails closed;
19. routed and unresolved records remain distinguishable;
20. unresolved routes do not enter recipient enumeration;
21. exact project/recipient enumeration is bounded and deterministically
    ordered;
22. cross-project and cross-recipient records are not visible;
23. authority revocation prevents later inbox/decision use even though the
    historical route remains stored;
24. `Debug`, errors, and serialization do not leak forbidden values;
25. existing routing, approval-presentation, immutable-run, escalation,
    PostgreSQL recovery, collaborative project, and workspace tests remain
    green.

## 17. Proposed Implementation Sequence

1. Review and accept this persistence plan. **Completed.**
2. Add the bounded source-commitment, logical-subject, durable-record, and
   create-result Core models only if they are required for a clean store
   contract. **Completed.**
3. Add `ProjectApprovalRouteStore` plus an in-memory contract fixture.
   **Completed.**
4. Review model and store semantics before migrations. **Completed.**
5. Add the canonical immutable deployment-authority commitment API and one
   PostgreSQL create/read implementation with its internal migration and
   dedicated indexes.
6. Add authenticated hosted composition from event history, immutable bundles,
   active project binding, and deployment authority, using the two-boundary
   protocol in Section 10.
7. Add concurrency, restart, corruption, and backup/restore tests.
8. Perform a phase-level security and maintainer review.
9. Only after acceptance, plan the principal-filtered hosted inbox.

Implementation should begin with the smallest model/store contract slice. It
must not jump directly to an HTTP endpoint.

## 18. Open Questions

- Does unresolved-route operator visibility require a new explicit capability,
  or should it remain audit-only?
- When authority or ownership changes, should a later append-only supersession
  record reference the first route or create a new evaluation subject?
- Should route creation be triggered when an approval is requested, or remain an
  explicit hosted service call until automatic runtime composition is reviewed?
- What strict list limit and cursor identity should the future inbox use?

These questions do not authorize broader implementation. The first review
should resolve only what is required for the model/store contract slice.

## 19. Final Recommendation

The plan review is accepted, and the project approval route persistence model,
store contract, in-memory fixture, and fix-forward maintainer/security review
are complete with no database migration. Proceed next with canonical
deployment-authority commitment integration and PostgreSQL route-store
planning before any migration or authenticated composer work.

Do not implement the hosted inbox, route-based decision enforcement, external
notifications, dynamic identity, provider writes, public schemas, CLI behavior,
or release changes.
