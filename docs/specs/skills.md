# Skill Specs

Skill specs define callable capabilities that workflows may reference. They do not execute code, call adapters, or perform external writes in this layer.

Skill files live in `skills/*.skill.yml` and use `schema_version: workflowos.dev/v0`.

## Required Fields

- `schema_version`: must be `workflowos.dev/v0`.
- `id`: stable skill identifier.
- `version`: skill definition version.
- `display_name`: human-readable name. `name` is accepted as a compatibility alias.

## Metadata

- `description`: optional human-readable description.
- `owner`: ownership metadata with `owning_team`, `maintainer`, `escalation_contact`, and `lifecycle_status`.
- `tags`: non-secret labels for search and documentation.

Allowed lifecycle statuses are `experimental`, `stable`, and `deprecated`.

## Contracts

Skills declare `input_contract` and `output_contract`.

Each contract contains:

- `fields`: schema-like field definitions.
- `required`: required field names.
- `examples`: optional examples for documentation and future tests.

Field types are:

- `string`
- `boolean`
- `number`
- `object`
- `array`

Fields can be marked sensitive:

```yaml
input_contract:
  fields:
    - name: request_description
      field_type: string
      sensitive: true
      redaction: summary_only
```

Allowed redaction behaviors are `full`, `summary_only`, and `reference_only`.

Sensitive example values must use `kind: sensitive`. Rust wraps those values in the redaction helper so display, debug, and serialization redact the value.

Semantic validation requires every sensitive field to declare redaction. Sensitive output fields must use `full` or `reference_only`.

## Capabilities And Adapters

`allowed_capabilities` declares capability names the skill may request through future policy.

`adapter_requirements` declares adapter boundaries without making adapters part of the skill model. A skill may declare `adapter_id`, optional `integration_id`, and required capability names. This does not imply write support, live credentials, or CLI execution for any concrete integration.

## Failure And Evaluation

`failure_modes` declare expected failure categories and whether they are retryable.

`evaluation_criteria` declares future test and documentation criteria. It is descriptive in v0.

## Retry And Approval Sensitivity

`retry_compatibility` may be:

- `compatible`
- `not_compatible`
- `requires_policy`

`approval_sensitivity` may be:

- `low`
- `medium`
- `high`

These values are model inputs for validation, policy, and the supported local executor path. They do not imply external writes or production skill execution.

## Audit And Observability

`audit_requirements` and `observability_requirements` declare runtime expectations used by validation and documentation. The local executor emits audit and observability from runtime events, but these fields do not create a production audit or metrics backend by themselves. Skills should prefer reference and summary storage for sensitive payloads.
