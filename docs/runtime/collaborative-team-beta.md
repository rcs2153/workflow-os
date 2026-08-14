# Collaborative Team Beta

Status: first project-boundary implementation; local evaluation only

Workflow OS now contains an explicit collaborative hosted router for one
deployment organization, multiple server-registered projects, and multiple
pre-provisioned principals. This is a narrow project-boundary proof over the
existing PostgreSQL and hosted no-write foundations. It is not production
multi-tenancy, enterprise identity, or a hosted SaaS claim.

## Implemented Boundary

The collaborative router uses paths under:

```text
/api/v0alpha1/organizations/:organization_id/projects/:project_id/...
```

Deployment configuration owns:

- one `OrganizationId`;
- a fixed registry of route-safe `ProjectId` values and canonical roots;
- bearer-token digests mapped to canonical actors;
- explicit, closed project capabilities.

Callers cannot supply project roots. Registries reject duplicate or nested
roots, unknown project grants, duplicate principals, and duplicate token
digests. Invalid credentials return `401`; a known project without capability
returns `403`; wrong organization, unknown project, and resource-scope mismatch
return non-disclosing `404` responses.

The first capability vocabulary covers catalog read/publication, run create and
read, approval read and decision, cancellation, and report read. It is not a
general role language.

## Durable Scope

PostgreSQL stores create-only organization/project bindings for runs and their
work items, execution receipts, and reports. Run creation reserves the global
run identity. Dispatch atomically activates that reservation before publishing
the first project-bound work item. Terminal receipt and report bindings commit
in the same serializable transaction as the terminal work-item, run projection,
receipt, and report artifact.

Collaborative reads require an active exact binding. A reservation without a
completed run is a non-authorizing tombstone. Reusing an identity under another
project fails closed. The collaborative worker rejects unbound work; the legacy
single-tenant worker remains an explicit compatibility posture.

Project catalog versions are immutable and scoped by organization, project,
workflow, and version. Publication requires owner and escalation metadata, the
authenticated publisher, and a complete `ApprovedForPromotion` stewardship
record that is persisted atomically with the catalog version. Publication does
not write workflow files or activate workflows.

## Filesystem Boundary

The shared project loader rejects absolute or parent-traversing layout paths and
symlinked manifests, layout directories, or spec files. Hosted run creation also
requires the loaded manifest project ID to equal the registered route project.
This protects the deployment-owned root from manifest-driven path substitution.

## Compatibility

The original unscoped hosted-alpha router remains separate. It is not silently
mapped to a collaborative project, and it rejects project-bound run resources
with a non-disclosing `404`. The default legacy worker may process legacy
unbound work; the collaborative worker requires project bindings. Disabling the
collaborative router leaves durable project bindings and catalog records intact.

## Current Limits

This beta does not provide:

- hostile-tenant isolation or multiple organization trust domains;
- OIDC, OAuth, SSO, SCIM, invitations, or dynamic grant management;
- an administrator UI, notification delivery, quotas, billing, or retention;
- production TLS, pooling, HA, PITR, or SLO claims;
- cross-project workflows, evidence access, or catalog sharing;
- provider mutation expansion or caller-submitted hosted work;
- general ownership/escalation routing;
- production hosted readiness.

The next collaborative milestone should compose catalog ownership and escalation
metadata into bounded approval routing and notifications without weakening the
project boundary.
