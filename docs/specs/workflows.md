# Workflow Specs

Workflow specs define what users can declare. They do not execute workflows and do not perform policy evaluation by themselves.

Workflow files live in `workflows/*.workflow.yml` and use `schema_version: workflowos.dev/v0`.

## Required Fields

- `schema_version`: must be `workflowos.dev/v0`.
- `id`: stable workflow identifier.
- `version`: workflow definition version.
- `display_name`: human-readable name. `name` is accepted as a compatibility alias.

## Identity And Metadata

- `description`: optional human-readable description.
- `owner`: ownership metadata with `owning_team`, `maintainer`, `escalation_contact`, and `lifecycle_status`.
- `tags`: non-secret labels for search and documentation.
- `spec_content_hash`: canonical hash populated by Rust parsing. Human-authored specs should not rely on hand-writing this value.
- `disabled_by_default`: required for v0 Level 3/4 declarations to pass validation.

Allowed lifecycle statuses are `experimental`, `stable`, and `deprecated`.

## Autonomy

`autonomy_level` declares the maximum intended autonomy for the workflow:

- `level_1` or `level_1_assistive`
- `level_2` or `level_2_guided_with_approval`
- `level_3` or `level_3_conditional_autonomy`
- `level_4` or `level_4_scaled_automation`

If omitted, the Rust model defaults to Level 1. Workflows that require approval should explicitly declare Level 2. Level 3 and Level 4 are declarations only in v0 and must be marked `experimental` and `disabled_by_default: true` to pass validation. They must not become default behavior without future policy enablement.

## Triggers

`triggers` declare how a workflow may be started. v0 requires trigger declarations for validation, but the CLI starts a workflow directly by workflow ID; there is no background trigger processor.

Supported trigger kinds are:

- `manual`
- `file`
- `schedule`
- `external_event`

External event triggers are declarations only. They do not imply a real external adapter.

## State Model

`state_model` may be:

```yaml
state_model:
  type: reference
  id: review-state-model
```

or:

```yaml
state_model:
  type: inline
  states:
    - received
    - reviewed
```

This is declarative structure for future richer validation and execution. It is not the runtime run-state machine.

## Steps

`steps` are ordered `StepDefinition` values.

Required fields:

- `id`: step identifier local to the workflow.
- `skill_ref`: referenced skill ID and optional version. `skill` is accepted as a compatibility alias.

Optional fields:

- `input_mapping`
- `output_mapping`
- `policy_requirements`
- `idempotency_key_strategy`
- `timeout`
- `retry_policy`
- `escalation_policy`
- `approval_policy`
- `terminal_behavior`

Example:

```yaml
steps:
  - id: draft-summary
    skill_ref:
      id: local/draft-summary
      version: v0
    input_mapping:
      - from:
          type: field
          path: request.description
        to: request_description
    idempotency_key_strategy:
      type: derived
    terminal_behavior: escalate
```

## Mappings

Mappings are typed and avoid unstructured blobs.

Allowed `from` expressions:

- `field`: reads a named field path.
- `literal`: uses a non-secret literal value.
- `config_ref`: reads a non-secret config reference by name.

Secrets must not appear in literals or config values.

## Approval, Retry, Escalation, And Timeout

Workflow-level approval requirements use `approval_requirements`.

Step-level policy references use:

- `retry_policy`
- `escalation_policy`
- `approval_policy`
- `policy_requirements`

Timeouts use structured duration wrappers:

```yaml
timeout:
  duration: 10m
```

The parser preserves these declarations. It does not evaluate policy, schedule retries, request approvals, or enforce timeouts yet.

Semantic validation requires terminal behavior to be explicit for each step.

## Audit And Observability

`audit_requirements` and `observability_requirements` declare future runtime expectations. They support documentation and later validation, but they do not emit events in this layer.

Sensitive payloads must be represented by references or summaries by default.
