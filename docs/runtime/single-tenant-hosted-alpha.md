# Single-Tenant Hosted Alpha

Status: implementation proof; not production ready

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
- authenticated retrieval of internally created hosted work items;
- authenticated payload-free execution-receipt retrieval;
- database-time leases, fencing, expired-worker takeover, and stale-commit
  rejection through the accepted `PostgreSQL` state contract;
- exact immutable run-bundle, governed-run, provider-request, provider-policy,
  and receipt binding;
- one no-write provider that rejects `SideEffect`, access-material, and
  non-read capability requests before invocation.

The API preview namespace is `/api/v0alpha1`. Request bodies are capped at
64 KiB. Remote work submission and run/event projection are deliberately not
exposed in this foundation because Core does not yet derive hosted work from
an approved immutable run at the remote boundary.

## Required Configuration

API mode requires:

```text
WORKFLOW_OS_HOSTED_DATABASE_URL
WORKFLOW_OS_HOSTED_TOKEN
WORKFLOW_OS_HOSTED_ACTOR
```

Worker modes require:

```text
WORKFLOW_OS_HOSTED_DATABASE_URL
WORKFLOW_OS_HOSTED_ACTOR
```

`WORKFLOW_OS_HOSTED_BIND` defaults to `127.0.0.1:8080`.

The API token is hashed in process memory for comparison and is never written
to Workflow OS durable state. The first alpha uses one rotatable
deployment-bound bearer token. It does not provide issuer, audience, expiry,
role, or enterprise identity semantics.

## Evaluation Topology

The defined, but not locally rehearsed, evaluation topology is:

```sh
export WORKFLOW_OS_HOSTED_DATABASE_ADMIN_PASSWORD='replace-for-local-evaluation'
export WORKFLOW_OS_HOSTED_DATABASE_PASSWORD='replace-for-local-evaluation'
export WORKFLOW_OS_HOSTED_TOKEN='replace-for-local-evaluation'
docker compose -f deploy/hosted-alpha/compose.yml up --build
```

The compose file binds the API only to local host port `8080`, uses
`PostgreSQL` 17, creates one non-superuser runtime database role, and starts
API and worker as separate processes. API and worker still share that runtime
role. Environment values in this example are local evaluation inputs, not a
production secret management recommendation. Existing volumes created before
the role-init script require a fresh evaluation volume.

## Correctness Boundary

An internal trusted caller can create a work item only when:

- its exact immutable run bundle exists;
- its durable run snapshot exists and is already `Running`;
- run, workflow, bundle, correlation, and idempotency identities match;
- the request is newly queued;
- idempotency has not been bound to a conflicting intent.

The worker claims under an expiring fenced lease, invokes only the built-in
inert provider, validates the exact receipt, and commits the terminal work
item, receipt, and lease release in one serializable transaction. A provider
rejection known not to have started transitions the item to `Failed` and does
not stop the long-running worker loop.

This proof does not append `SkillInvocationSucceeded`, complete or advance the
workflow run, or mutate its snapshot. A no-op containment check is not evidence
that a workflow skill executed. Hosted orchestration must derive work from the
authoritative run and own any later workflow event separately.

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

The current hosted surface does not yet expose:

- remote validation or immutable-bundle upload;
- remote hosted work-item submission;
- remote governed run creation;
- remote run snapshots or event pages;
- remote approval presentation or decision;
- remote cancellation;
- remote `WorkReport` or report-artifact retrieval;
- a live access-material resolver;
- provider writes;
- automatic local checks;
- OpenShell or another sandbox;
- metrics export, distributed tracing, or production logging;
- multi-tenancy, enterprise roles, SSO, SCIM, or hosted administration.

Those omissions are blockers to accepting the complete hosted-alpha plan, not
features hidden behind configuration.
