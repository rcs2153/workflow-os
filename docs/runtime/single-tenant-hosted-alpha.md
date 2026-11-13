# Single-Tenant Hosted Alpha

Status: runtime-composition evaluation; not production ready

The `workflow-hosted` crate is the first bounded remote-process proof for
Workflow OS. It preserves the project boundary:

```text
Agent or provider executes. Workflow OS governs.
```

It is one deployment, one administrative trust domain, one `PostgreSQL`
database, one authenticated API identity, and one or more stateless workers.
It is not multi-tenant, enterprise identity, a general agent runtime, or a
sandbox.

## Implemented Surface

The `workflow-os-hosted` binary supports:

- API mode by default;
- long-running worker mode with `--worker`;
- one-item worker proof with `--worker-once`;
- public liveness and dependency-aware readiness;
- authenticated build identity;
- authenticated idempotent governed-run creation from a server-owned project
  root and create-only immutable bundle;
- authenticated run, bounded ordered event-page, exact approval-request,
  terminal report-artifact metadata, and hosted-record retrieval;
- idempotency-bound proof-enforced approval decisions and eligible
  cancellation through existing Core executor paths;
- bounded queue, attempt, and receipt operational posture;
- authenticated retrieval of internally created hosted work items;
- authenticated payload-free execution-receipt retrieval;
- database-time leases, fencing, expired-worker takeover, and stale-commit
  rejection through the accepted `PostgreSQL` state contract;
- exact immutable run-bundle, governed-run, provider-request, provider-policy,
  and receipt binding;
- atomic terminal `WorkReport` artifact creation for the successfully receipted
  no-write path, with stable workflow-event and hosted receipt/telemetry
  citations;
- one no-write provider that rejects `SideEffect`, access-material, and
  non-read capability requests before invocation.

The API preview namespace is `/api/v0alpha1`. Request bodies are capped at
64 KiB. Callers cannot submit hosted work items or provider requests. Hosted
work remains an internal Core/`PostgreSQL` concern and must stay bound to a
server-owned immutable run. Run creation, approval decisions, and cancellation
require caller-supplied idempotency keys. Approval and cancellation keys are
bound to deterministic payload-free intent hashes and fail closed if reused
for different mutations.

## Required Configuration

API mode requires:

```text
WORKFLOW_OS_HOSTED_DATABASE_URL
WORKFLOW_OS_HOSTED_TOKEN
WORKFLOW_OS_HOSTED_ACTOR
WORKFLOW_OS_HOSTED_PROJECT_ROOT
```

Worker modes require:

```text
WORKFLOW_OS_HOSTED_DATABASE_URL
WORKFLOW_OS_HOSTED_ACTOR
```

`WORKFLOW_OS_HOSTED_BIND` defaults to `127.0.0.1:8080`.

`WORKFLOW_OS_HOSTED_PROJECT_ROOT` is the server-controlled validated project
root used for immutable-bundle creation. It must name an existing directory.
The API never accepts a project path from the request body.

The API token is hashed in process memory for comparison and is never written
to Workflow OS durable state. The first alpha uses one rotatable
deployment-bound bearer token mapped to one deployment actor. It does not
provide issuer, audience, expiry, operation scope, role, or enterprise identity
semantics. This mechanism is acceptable only for the single-trust-domain
evaluation and is not a production mutation authority.

## Evaluation Topology

The defined evaluation topology is:

```sh
export WORKFLOW_OS_HOSTED_DATABASE_ADMIN_PASSWORD='replace-for-local-evaluation'
export WORKFLOW_OS_HOSTED_DATABASE_PASSWORD='replace-for-local-evaluation'
export WORKFLOW_OS_HOSTED_TOKEN='replace-for-local-evaluation'
docker compose -f deploy/hosted-alpha/compose.yml up --build
```

The bounded restart rehearsal is:

```sh
scripts/rehearse-hosted-alpha.sh
```

It creates a real server-owned no-write governed run while the worker is
stopped, restarts the API, verifies that the queued run survives, starts the
worker, waits for authoritative completion, inspects the terminal event trail
and report metadata, restarts the worker, interrupts `PostgreSQL`, verifies
that readiness fails, restores the database process, and verifies that the
same terminal run and report remain readable. By default it leaves the
topology running for operator inspection. Set
`WORKFLOW_OS_HOSTED_CLEANUP=1` for an isolated CI run that removes its compose
volume on exit.

In disposable cleanup mode the rehearsal replaces the manual worker restart
with a one-shot PostgreSQL fault scoped to a new no-write run's first queue
claim. The fault rolls back the claim and exits the worker. The rehearsal
requires Docker to restart the same container exactly once without an operator
restart, then verifies one completed invocation, one report, and unchanged
events and report on exact HTTP replay. This does not prove recovery from an
interruption after invocation or an external effect.

The compose file binds the API only to local host port `8080`, uses
`PostgreSQL` 17, creates one non-superuser runtime database role, and starts
API and worker as separate processes. API and worker still share that runtime
role. Environment values in this example are local evaluation inputs, not a
production secret management recommendation. Existing volumes created before
the role-init script require a fresh evaluation volume.

## Correctness Boundary

Core can dispatch a work item only when:

- its exact immutable run bundle exists;
- its durable run snapshot exists and is already `Running`;
- run, workflow, bundle, correlation, and idempotency identities match;
- the authoritative run has exactly one supported scheduled terminal skill
  invocation;
- the request is newly queued;
- idempotency has not been bound to a conflicting intent.

The Core dispatch transaction appends `SkillInvocationRequested` and
`SkillInvocationStarted`, updates the run snapshot, and queues the exact work
item atomically. The worker claims under an expiring fenced lease, rehydrates
the authoritative run and exact immutable bundle binding immediately before
invocation, persists the durable attempt posture, invokes only the built-in
inert provider, validates the exact receipt, and commits terminal workflow
events, snapshot, work item, attempt, receipt, terminal report artifact, and
lease release atomically under the active fence. The hosted report cites the
two terminal workflow events and stable payload-free receipt/environment/
telemetry references. An exact transaction replay requires the same artifact;
a missing or conflicting artifact fails closed.

A request that the provider can prove was rejected before start is committed
through a separate Core-owned atomic projection. It appends
`SkillInvocationFailed` and `RunFailed`, updates the run snapshot and failed
work item, releases the lease, and creates no invocation attempt or receipt.

If provider invocation may have started but no valid receipt is available, the
same atomic boundary marks the invocation attempt
`reconciliation_required`, moves the work item to `ambiguous`, appends
`EscalationTriggered`, projects the run to `Escalated`, and releases the lease.
An exactly bound receipt whose status is itself `ambiguous` also escalates the
run. Neither path fabricates success, converts uncertainty into ordinary
failure, or permits blind retry.

A no-op containment check by itself is not evidence that a workflow skill
executed. Only an exactly bound receipt committed through the Core-owned atomic
result projection may produce `SkillInvocationSucceeded`; direct receipt
storage cannot.

The no-write provider emits only a bounded telemetry reference. It does not
run a shell, read files, call a network service, resolve credentials, invoke a
model, or mutate a provider.

## Recovery Posture

API and worker processes contain no authoritative run state. A process restart
reconnects to `PostgreSQL`. An abandoned running work item becomes claimable
only after its database-time lease expires, and the new claim receives a
higher fence. Lease release expires the retained row rather than deleting its
token history, so a stale worker cannot regain an earlier fence value.

The existing [PostgreSQL State Recovery](postgresql-state-recovery.md)
rehearsal remains the database backup/restore and projection-rebuild proof.
Together, the live `PostgreSQL` conformance/recovery job and hosted deployment
rehearsal prove expired-lease takeover, stale-fence rejection, schema-checksum
closure, backup/restore, projection rebuild, immutable-bundle readability,
API/worker restart, dependency-aware readiness, and terminal report recovery.
The desktop development environment used for this phase does not provide
Docker, so the compose rehearsal is enforced in Linux CI rather than claimed
as a local container result.
The hosted alpha does not claim high availability, point-in-time recovery,
recovery objectives, connection pooling, or production disaster recovery.

### Conflicting Creation History

HTTP 409 with `hosted.run_create.reservation_retry_exhausted` means the
no-write create endpoint exhausted its bounded retries while reserving the
request. Retry the unchanged HTTP request with its original idempotency key
and run identity. This response does not acknowledge queued work or prove
that another submission of the same request did not succeed.

HTTP 409 with `postgres_state.hosted_creation.prefix_conflict` means the
stored events do not match the expected creation prefix. This is not a
transient contention error. The conflicting retry does not rewrite history,
dispatch work, or create a report. It does not prove that earlier requests
had no effects.

1. Stop retries for the original request. Keep its run ID, correlation ID,
   and the error code in the operator incident record. Do not create a new
   run or change the idempotency key to bypass the conflict.
2. Using the existing authorized client, read
   `GET /api/v0alpha1/runs/{run_id}` and
   `GET /api/v0alpha1/runs/{run_id}/events`. If the run projection cannot
   load, inspect the recorded events directly through the events endpoint.
   Follow `has_more` using the last event's sequence as `after_sequence`.
   Each page for a nonempty single-tenant history includes a
   `history_revision` for the full observed event stream. If it changes
   between pages, restart inspection from the first page. A matching
   revision does not validate the history or grant authority.
   The inspection route can show typed events that fail run grammar; normal
   run reads still reject them. Inspection also rejects rows that cannot be
   decoded or whose stored identity does not match the event.
3. Escalate the original request and observed history to the operator for
   reconciliation. A missing report or incomplete history is not a
   clean failure. Do not edit stored events, assume the operation never
   ran, or use cancellation as a repair. These reads do not resolve the
   conflict or authorize redispatch.

Record the disposition in the access-controlled operator incident system,
not in the run event stream:

- Bind the record to the original run and correlation IDs, the protected
  reference to the original request, and the inspected history (last
  sequence and event ID plus a protected evidence reference). Record the
  operator, decision time, reason, and follow-up owner. Do not copy request
  bodies, credentials, or raw event payloads into incident summaries.
- Record **investigation required** if the effect is unknown, the history
  cannot be read completely, or evidence disagrees. Keep the incident open
  and record what evidence would resolve it.
- Record **effect confirmed; do not repeat** only with independently
  attributable evidence bound to the exact original operation and target.
  Do not turn that incident decision into a kernel completion or WorkReport.
- Record **abandoned without redispatch** if the operator elects to stop
  pursuing the original request. This administrative closure is not evidence
  that nothing happened; preserve any uncertainty in the incident record.

If the history changes during review, revisit the disposition against the
new evidence. None of these decisions authorize another invocation, change
the API's conflict response, or repair the run.

The single-tenant server optionally exposes
`GET /api/v0alpha1/operator/runs/{run_id}/creation-conflicts` and `POST` to
the same path. Both operator settings must be present:
`WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_TOKEN` and
`WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_ACTOR`. The deployment must supply a
separate credential and actor; ordinary API access does not grant operator
access. Keep the endpoint on the existing bounded evaluation trust domain.

Compare the GET observation to the revision of all inspected event pages.
For POST, send the same run ID, a stable decision ID, observed and expected
revisions from the observation, the current predecessor if superseding,
disposition, bounded reason, owner and same-run event IDs. Only
`investigation_required` and `abandoned_without_redispatch` are accepted.
An exact retry returns the recorded decision, not a guarantee it is still
current. A stale new decision or changed replay fails closed. The API does
not confirm effects or replace external incident follow-up.

## Explicitly Deferred

The current hosted surface still does not expose:

- caller-authored immutable-bundle upload;
- remote hosted work-item submission;
- full `WorkReport` bodies;
- a live access-material resolver;
- provider writes;
- automatic local checks;
- OpenShell or another sandbox;
- external metrics export, distributed tracing, or production logging;
- multi-tenancy, enterprise roles, SSO, SCIM, or hosted administration.

The API token posture, access-material isolation, separate API/worker database
identities, production TLS/network controls, capacity, HA, and disaster
recovery remain blockers to production claims. They are not features hidden
behind configuration.
