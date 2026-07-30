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

The defined, but not locally rehearsed, evaluation topology is:

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

It verifies authenticated readiness, build identity, and operational posture
across API and worker restarts. It leaves the topology running so the operator
can inspect it and does not delete volumes.

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
events, snapshot, work item, attempt, receipt, and lease release atomically
under the active fence.

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
The hosted alpha does not claim high availability, point-in-time recovery,
recovery objectives, connection pooling, or production disaster recovery.

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

The API token posture, access-material isolation, explicit ambiguous/pre-start
failure recovery projection, and full deployed recovery proof remain blockers
to production claims. They are not features hidden behind configuration.
