# Project-Scoped Approval Routing And Bounded Notification Plan

Status: Core route model and route persistence plan accepted; model/store contract next

Implementation update: the first model-only slice now provides validated,
content-addressed `ProjectApprovalRoute` records and a pure
`resolve_project_approval_route` function. The resolver accepts explicit active
run binding, immutable ownership, approval subject, exact project scope, and
deployment-owned principal bindings. It does not persist routes, expose a
hosted inbox, deliver notifications, or change approval authority or workflow
execution.

Review update: ordinary maintainer routing preserves the planned authority
boundary. The escalation-contact blocker is fixed: the resolver now requires an
exact run-matched `EscalationRecord`, validates its contact against immutable
ownership metadata, stores only its stable escalation reference, and rejects
missing, unexpected, or mismatched subjects. Focused blocker-fix review accepted
the Core model. Route persistence planning is documented in
[Project-Scoped Approval Route Persistence Plan](project-scoped-approval-route-persistence-plan.md).
That plan requires authenticated reconstruction from event history and immutable
run state, create-only reconciliation, and an independent current-authority
check before any hosted consumer may rely on a route. Hosted inbox work remains
deferred.

## 1. Executive Summary

The collaborative team beta proves one deployment organization, multiple
registered projects, pre-provisioned principals, project-scoped capabilities,
two-actor approval collaboration, and durable cross-project isolation. It does
not yet decide which authorized project actor should receive a particular
approval request or escalation.

The next implementation should compose existing workflow ownership and
escalation metadata with existing project-scoped principal grants. Routing must
select only an actor who is both named by the immutable workflow definition and
already authorized for the exact project. Metadata must never create authority.

The first implementation should be a deterministic Core resolver plus one
project-scoped hosted approval-inbox read path. It should return payload-free
routing and notification posture in memory. It should not send email, Slack,
pages, tickets, or provider messages, and it should not change approval decision
authority, workflow execution, persistence schemas, or public workflow YAML.

## 2. Product Outcome

For one project-bound pending approval, Workflow OS should be able to answer:

```text
Who is the intended accountable recipient, is that actor currently authorized
to decide this project approval, and what bounded notification posture can an
operator surface truthfully?
```

An authorized principal should be able to query a project-scoped inbox and see
only approval route summaries assigned to that principal. The summary should
identify stable project/run/approval references, routing reason, urgency, and
notification posture without copying approval reasons, workflow payloads,
source content, credentials, or evidence payloads.

## 3. Goals

- compose immutable workflow ownership/escalation metadata with project scope;
- preserve hosted capability grants as the authority source;
- route ordinary approval requests to the configured maintainer when that actor
  has exact project `ApprovalDecide` authority;
- route runtime escalations to the configured escalation contact when that actor
  has exact project authority;
- fail closed when configured routing metadata is absent, ambiguous,
  cross-project, or not backed by current authority;
- expose payload-free route and bounded notification posture;
- provide one project-scoped, principal-filtered hosted approval inbox;
- preserve approval-presentation proof and resolved-context integrity;
- keep authorization denials and routing failures auditable and non-leaking;
- integrate monotonically with proportional-governance decisions.

## 4. Non-Goals

This plan does not authorize:

- email, Slack, Teams, SMS, paging, ticket creation, or webhooks;
- proof that a human saw, understood, or acknowledged a notification;
- dynamic users, invitations, groups, roles, directories, OIDC, SSO, or SCIM;
- a general RBAC engine or administrator UI;
- workflow YAML or public schema changes;
- treating `owning_team`, `maintainer`, or `escalation_contact` as authority;
- automatic approval, delegated approval expansion, quorum, or separation-of-
  duties changes;
- escalation timers, schedules, reassignment, or on-call rotation resolution;
- provider writes or additional mutation families;
- changes to workflow pass/fail semantics;
- cross-project routing or notification;
- notification persistence, retry queues, or delivery guarantees;
- hosted production-readiness or release-posture claims.

## 5. Existing Contracts To Reuse

### 5.1 Workflow Metadata

`OwnershipMetadata` already carries:

- `owning_team` as descriptive metadata;
- `maintainer` as an optional `ActorId`;
- `escalation_contact` as an optional `ActorId`;
- lifecycle posture.

The immutable run bundle and resolved execution context must remain the source
of workflow metadata for a running approval. Live workflow files must not be
re-read as routing authority after the run starts.

### 5.2 Project Authority

`HostedPrincipalBinding`, `HostedProjectGrant`, and
`HostedProjectCapability::ApprovalDecide` already define the deployment-owned
project authority boundary. The resolver must consume an immutable view of that
registry. It must not create, widen, or persist grants.

### 5.3 Approval And Escalation

`ApprovalRequest` carries exact workflow/run/spec/resolved-context identity and
one validated approval subject. `EscalationRecord` carries the configured
escalation contact and bounded failure posture. Routing must preserve those
records as sources of truth and must not infer subjects from free-form approval
IDs or reasons.

### 5.4 Disclosure Delivery

`GovernanceDisclosureDeliveryRequest` and its receipt establish an important
claim boundary: a configured surface can acknowledge a payload-free disclosure,
but that does not prove human observation or acknowledgement. The approval
notification model should reuse that semantic posture, not misuse the existing
visible-proceed assessment contract for a different subject.

## 6. Authority And Routing Invariant

Routing and authority are separate:

```text
routing candidate = immutable workflow metadata
decision authority = deployment-owned exact project capability
effective recipient = candidate intersection authority
```

The resolver must never:

- grant `ApprovalDecide` because an actor is named as maintainer;
- route a project-A approval using a project-B grant;
- fall back to another project or an arbitrary deployment principal;
- interpret `owning_team` as an actor or dynamic group;
- use an escalation contact for an ordinary approval unless an explicit later
  routing policy authorizes that fallback;
- expose whether an unprivileged actor exists in another project.

When the intersection is empty, the route is unresolved and the request remains
pending. This is a governance configuration failure, not permission to choose a
recipient heuristically.

## 7. Candidate Core Model

Add the smallest domain-neutral model set required by the resolver:

- `ProjectApprovalRouteId`;
- `ProjectApprovalRoutingReason`:
  - `WorkflowMaintainer`;
  - `WorkflowEscalationContact`;
- `ProjectApprovalRouteStatus`:
  - `Routed`;
  - `UnresolvedMissingMetadata`;
  - `UnresolvedAuthorityUnavailable`;
- `ProjectApprovalRoute` with exact project scope, stable run and approval
  references, optional redacted recipient reference, reason, status, and
  creation time;
- `ProjectApprovalRouteInput` carrying validated project scope, immutable
  approval subject, immutable ownership metadata, routing purpose, and an
  explicit bounded authority-view interface;
- `ProjectApprovalNotificationPosture`:
  - `AvailableForProjectInbox` for a routed blocking approval;
  - `UnavailableRouteUnresolved` when no authorized recipient exists.

`NotRequired` remains proportional-governance decision posture rather than a
project approval-route state. This first resolver is called only for an actual
pending approval subject and does not create route records for quiet, visible,
or denied decisions.

Names may be adjusted to local conventions during implementation. Do not add
free-form role, channel, team, or notification-provider strings.

The route must use redaction-safe `Debug`, validated serde reconstruction, stable
non-leaking errors, bounded collections, and read-only accessors.

## 8. Deterministic Resolution Rules

1. Validate project scope and approval subject.
2. Require the approval's run to have an active binding to the exact project.
3. Load ownership metadata from the immutable run definition, not live files.
4. Select `maintainer` for an ordinary pending approval.
5. Select `escalation_contact` only for an actual runtime escalation route.
6. Verify the selected actor is a pre-provisioned principal in the deployment
   organization with exact-project `ApprovalDecide` authority.
7. Produce one deterministic route or one bounded unresolved posture.
8. Do not mutate the approval request, run, principal registry, or workflow.
9. Do not create notification delivery receipts during resolution.

Duplicate evaluation of identical inputs must produce the same route identity
and posture. A changed immutable source, project scope, approval subject,
recipient, or authority view must produce a different route commitment or fail
validation.

## 9. Proportional-Governance Composition

Routing must not turn quiet work into an interruption.

- `Proceed + Quiet`: no notification route is required; evidence/audit/report
  capture remains quiet.
- `Proceed + Visible`: the existing disclosure surface owns visibility. This
  phase does not create an approval notification.
- `RequireApproval`: resolve an accountable project approval recipient and make
  the route available to the project inbox.
- `Deny`: preserve denial/audit/report posture; do not ask a recipient to
  approve an already denied action.
- Runtime escalation: route to the configured escalation contact only after an
  escalation record exists.

This preserves the principle that visible presentation is a surface concern,
while interruption is justified by the authoritative governance decision.

## 10. Bounded Notification Semantics

The first notification surface is a pull-based project approval inbox, not a
message-delivery system. An inbox item may disclose only:

- route ID;
- exact organization/project scope reference;
- run and approval references;
- routing reason and status;
- bounded urgency/sensitivity posture;
- requested timestamp and expiry posture;
- next action to retrieve the separately authorized approval context.

It must not contain approval reason text, spec contents, evidence payloads,
command output, provider payloads, source snippets, credentials, paths, or human
contact details.

Inbox availability proves only that the authorized hosted surface returned the
route summary. It does not prove independent delivery, human observation,
acknowledgement, or decision.

## 11. Hosted Integration Boundary

After the Core resolver is reviewed, add one additive endpoint under the
collaborative project router, conceptually:

```text
GET /organizations/{organization}/projects/{project}/approvals/inbox
```

The exact path should follow current router conventions. The endpoint must:

- authenticate one pre-provisioned principal;
- require exact-project `ApprovalRead`;
- enumerate only pending project-bound approval requests;
- resolve routes from each run's immutable definition;
- return only route summaries assigned to the authenticated actor;
- return unresolved counts to an appropriately authorized project operator only
  if a later explicit capability is approved; otherwise omit them;
- use bounded pagination or a strict count limit;
- avoid changing approval decision authorization;
- emit payload-free access decisions through the existing audit boundary.

The first slice should derive results on demand. Do not add notification tables,
queues, or delivery workers before route semantics are accepted.

## 12. Failure And Privacy Posture

- Missing ownership metadata yields `UnresolvedMissingMetadata`.
- A named actor without exact project authority yields
  `UnresolvedAuthorityUnavailable`.
- Corrupt immutable definitions, scope mismatches, malformed approval subjects,
  or ambiguous identities fail closed.
- Unresolved routing must not silently fall back to the requester, deployment
  administrator, arbitrary reviewer, or another project's principal.
- Errors must use stable codes and omit actor IDs, project IDs, approval IDs,
  paths, reasons, payloads, and authorization details.
- `Debug` may expose enum posture and bounded counts only.
- Hosted `404`/`403` non-disclosure semantics remain unchanged.

## 13. Implementation Sequence

1. Implement the pure Core route model and resolver with explicit inputs.
   **Implemented.**
2. Review the model/resolver, especially metadata-versus-authority separation.
   **Accepted after one escalation-subject blocker fix.**
3. Define authenticated source composition, complete approval-subject
   commitment, logical route-subject identity, create-only reconciliation, and
   conflict-safe storage semantics. **Planned and accepted after three planning
   blocker fixes.**
4. Add the project approval route record and store contract, then one
   PostgreSQL adapter. Exact duplicate routes may reconcile; conflicting
   duplicates must fail closed. A deciding actor must not be constrained by a
   persisted route presented as authority.
5. Add the principal-filtered hosted approval inbox read path and require a
   deciding actor to match the durable route as well as hold `ApprovalDecide`.
6. Add payload-free route/access audit projection and focused restart tests.
7. Review the complete same-project and cross-project path.
8. Only then plan optional external notification adapters separately. Any real
   external send must enter the governed `SideEffect` lifecycle with policy,
   authority, idempotency, reconciliation, evidence, and report posture.

The immediate next phase is the route persistence record and store contract
only. This is a bounded prerequisite to PostgreSQL route storage and the
runnable inbox slice, not authorization to remain indefinitely in model-only
work.

## 14. Test Plan

Future tests must cover:

- workflow maintainer with exact project authority routes successfully;
- escalation contact with exact project authority routes an actual escalation;
- metadata never grants authority;
- missing maintainer and missing escalation contact remain unresolved;
- named actor without `ApprovalDecide` remains unresolved;
- project-A metadata cannot consume project-B authority;
- owning-team text is never interpreted as an actor;
- live workflow metadata drift does not change a run's route;
- malformed and mixed approval subjects fail closed;
- deterministic route identity and serde round trip;
- invalid serialized routes fail closed;
- `Debug`, errors, and serialization do not leak forbidden content;
- quiet and visible non-blocking proportional routes do not create approval
  notifications;
- blocking approval creates inbox posture without claiming human observation;
- principal-filtered inbox hides other recipients and projects;
- pagination/count limits and restart behavior;
- existing collaborative, approval-presentation, proportional-governance,
  escalation, audit, and report tests remain green.

## 15. Open Questions

- Should a future project operator capability inspect unresolved route counts,
  or should unresolved routing be visible only through audit/health posture?
- Should approval reassignment be modeled as a new append-only route decision
  rather than mutation of an existing route?
- How should expiration and revocation of future dynamic authority invalidate an
  already derived route?
- Should persisted route changes always be additive supersession records rather
  than updates, including delegated-maintainer reassignment?
- When should an escalation contact become eligible as an explicit fallback for
  an ordinary approval?
- Which external notification adapter, if any, should be proven first after the
  pull-based inbox is accepted?

## 16. Final Recommendation

Proceed next with the accepted project-scoped approval route persistence record
and store contract, then one PostgreSQL adapter before the collaborative hosted
approval inbox.

Do not implement external notifications, dynamic identity, enterprise admin,
provider writes, workflow/public schema changes, or broader mutation families
in either slice. The PostgreSQL adapter slice explicitly requires a bounded
internal storage migration and indexes for the private route-record table; that
does not authorize workflow spec, public API, or provider schema exposure.
