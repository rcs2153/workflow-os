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

Workflow specs may describe approval, retry, escalation, and timeout intent. The v0 local executor enforces these semantics only for the supported single-step local runtime path. Unsupported or unsafe behavior must fail closed and must not be presented as implemented execution behavior.
