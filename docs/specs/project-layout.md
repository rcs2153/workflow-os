# Workflow OS v0 Project Layout

Workflow OS projects are local-first, declarative, versioned, and suitable for Git review.

The primary human-authored format is YAML. JSON Schema files under `schemas/v0/` provide editor and tooling validation. Rust remains the canonical owner of the schema model and parser behavior.

## Canonical Layout

```text
workflow-os.yml
workflows/
  *.workflow.yml
skills/
  *.skill.yml
policies/
  *.policy.yml
tests/
  *.test.yml
```

The default directory names may be overridden in `workflow-os.yml`, but the file roles do not change.

## `workflow-os.yml`

`workflow-os.yml` declares project metadata and optional non-secret configuration overlays.

Required fields:

- `schema_version`: must be `workflowos.dev/v0`.
- `project.id`: stable project identifier.
- `project.name`: human-readable project name.

Optional fields:

- `project.description`
- `layout.workflows`
- `layout.skills`
- `layout.policies`
- `layout.tests`
- `config`

Example:

```yaml
schema_version: workflowos.dev/v0
project:
  id: acme/approval
  name: Acme Approval
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
config:
  - environment: dev
    vars:
      - name: approval_timeout
        value: 1h
```

## Workflow Specs

Workflow specs live in `workflows/*.workflow.yml`.

Required fields:

- `schema_version`
- `id`
- `version`
- `display_name` (`name` is accepted as a compatibility alias)

Optional fields:

- `description`
- `owner`
- `autonomy_level`
- `triggers`
- `state_model`
- `steps`
- `branches`
- `approval_requirements`
- `retry_policy_refs`
- `escalation_policy_refs`
- `timeout_policy`
- `cancellation_behavior`
- `audit_requirements`
- `observability_requirements`
- `tags`

Workflow steps include:

- `id`
- `skill_ref` (`skill` is accepted as a compatibility alias)
- `input_mapping`
- `output_mapping`
- `policy_requirements`
- `idempotency_key_strategy`
- `timeout`
- `retry_policy`
- `escalation_policy`
- `approval_policy`
- `terminal_behavior`

Workflow specs are parsed into the canonical Rust workflow definition model in v0. The project layout layer itself only defines file shape and parse behavior; deterministic semantic validation and local execution are implemented in later Rust layers.

## Skill Specs

Skill specs live in `skills/*.skill.yml`.

Required fields:

- `schema_version`
- `id`
- `version`
- `display_name` (`name` is accepted as a compatibility alias)

Optional fields:

- `description`
- `owner`
- `input_contract`
- `output_contract`
- `allowed_capabilities`
- `adapter_requirements`
- `failure_modes`
- `evaluation_criteria`
- `retry_compatibility`
- `approval_sensitivity`
- `audit_requirements`
- `observability_requirements`
- `tags`

Real adapter behavior is deferred. `adapter_requirements` only declare the boundary a future runtime and policy layer must enforce.

## Policy Specs

Policy specs live in `policies/*.policy.yml`.

Required fields:

- `schema_version`
- `id`
- `name`

Optional fields:

- `description`
- `rules`

Policy spec rules are parsed as declarative project files in v0. Runtime policy enforcement is currently provided by the conservative Rust policy engine rather than by a full policy language interpreter.

## Test Specs

Test specs live in `tests/*.test.yml`.

Required fields:

- `schema_version`
- `id`
- `name`
- `target`

Optional fields:

- `assertions`

Tests are declarative shells in v0. They establish the intended shape for future test execution but do not execute workflows.

## Reference Resolution

v0 reference resolution is local-project-only.

Rules:

- Workflow references resolve against local `workflows/*.workflow.yml`.
- Skill references resolve against local `skills/*.skill.yml`.
- Policy references resolve against local `policies/*.policy.yml`.
- Test targets resolve against local project specs.
- Explicit versions must match the referenced spec version.
- Missing skill versions resolve only when exactly one local version exists.
- Remote packages, registries, and marketplace lookup are not supported in v0.

## Generic Domain Boundary

The layout is intentionally generic. It must not assume GitHub, Jira, CI, ticketing, pull requests, or software-engineering-only workflows. Domain-specific behavior belongs in future examples, skills, adapters, and policies.
