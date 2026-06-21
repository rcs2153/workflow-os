# Workflows

A Workflow OS workflow is a versioned, declarative definition of governed AI-driven work.

In v0, a workflow is a model only. It can declare triggers, state model intent, ordered steps, branches, approval requirements, retry and escalation references, timeout behavior, cancellation behavior, audit requirements, and observability requirements. It does not execute by itself.

## Design Intent

Workflow definitions must be:

- Human-readable.
- Versioned.
- Suitable for Git review.
- Generic across enterprise domains.
- Safe by default.
- Precise enough for validation, local execution, audit, policy, and future documentation generation.

Workflows must not encode GitHub, Jira, CI, or any other concrete integration as core behavior. Domain behavior belongs in skills, adapters, policies, examples, and project-specific specs.

## Dogfood, Examples, And User Workflows

Workflow OS uses its own kernel to govern Workflow OS development. Those self-governance workflows live under `dogfood/` and use `dg/*` identifiers. They are intentionally specific to this repository's build loop, roadmap, PR hygiene, branch cleanup, release readiness, blocker fixes, and workflow-discovery needs.

`dg/*` workflows are reference patterns, not community defaults. A user pulling down Workflow OS should read them as:

```text
This is how Workflow OS governs its own development.
```

They should not be read as:

```text
These are the workflows every company must adopt.
```

Portable learning material belongs under `examples/`. User, team, or company workflows should be authored in their own Workflow OS projects, with their own ownership, authority, evidence, approval, state, and reporting boundaries.

## Authored Definitions Versus Runtime State

Git is a good place to review and version authored workflow contracts. It is not the long-term database for governed execution.

Workflow OS should preserve a sharp boundary:

```text
Git stores authored contracts.
Workflow OS stores governed execution state.
```

Workflow definitions, examples, and dogfood specs can be source-controlled. Runtime state needs a durable Workflow OS store because it includes current run state, event history, approval decisions, evidence references, report artifacts, side-effect records, hook records, local check results, and future workflow catalog recommendations.

In v0, local file-backed state is part of the local kernel preview. Team collaboration, concurrent workflow stewardship, catalog recommendations, and organization-scale governance will require an explicit durable store and backend interface rather than treating git as the database.

## Workflow Identity

Each workflow has:

- `id`
- `version`
- `schema_version`
- canonical `spec_content_hash`

Runs reference these values exactly so a run never executes against an implicit latest definition.

## Steps

Steps reference skills by ID and optional version. A step may declare mappings, policy requirements, idempotency strategy, timeout, retry policy, escalation policy, approval policy, and terminal behavior.

Step declarations are not runtime state. They are immutable definition content once a run starts.

## Safety Boundary

Workflow specs may describe approval, retry, escalation, and timeout intent. The v0 local executor enforces these semantics only for the supported sequential local runtime path. Unsupported or unsafe behavior must fail closed and must not be presented as implemented execution behavior.
