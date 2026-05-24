# TypeScript SDK

The TypeScript SDK exists for Workflow OS spec authoring ergonomics. It is not a runtime, validator, adapter framework, or policy engine.

## Purpose

The SDK helps TypeScript users build Workflow OS project artifacts:

- `workflow-os.yml`
- `workflows/*.workflow.yml`
- `skills/*.skill.yml`
- `policies/*.policy.yml`

The SDK emits JSON-formatted specs that are compatible with the YAML project layout. YAML parsers accept JSON syntax, so generated files can live in the normal `.yml` locations and remain validated by the Rust CLI.

## Boundary

Rust remains authoritative for:

- schema compatibility
- semantic validation
- runtime execution
- policy
- audit
- observability
- CLI behavior

The SDK must not execute workflows, call adapters, implement policy, or perform validation that can disagree with Rust.

## Current Helpers

The initial SDK exposes typed helpers for:

- project manifests
- workflow definitions
- skill definitions
- policy definitions
- mapping expressions
- JSON spec emission
- project file maps

Helper APIs require explicit autonomy, cancellation, audit, observability, lifecycle, policy, and approval fields where those fields matter for v0 validation. They do not hide approval or policy behavior behind implicit runtime defaults.

## Secret Handling

The SDK rejects secret-like string values in helper paths that emit specs, including literal mappings and project config values. Secrets must not be stored in specs.

Sensitive contract fields can be represented with `sensitive: true` and an explicit `redaction` behavior.
