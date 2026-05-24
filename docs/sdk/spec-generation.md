# TypeScript Spec Generation

The TypeScript SDK emits spec artifacts for Rust validation.

## Emission Format

The initial SDK emits formatted JSON strings. Generated files are intended to be written into the normal Workflow OS project layout:

```text
workflow-os.yml
workflows/*.workflow.yml
skills/*.skill.yml
policies/*.policy.yml
```

This is valid because JSON is accepted by the YAML parser used by the Rust project loader.

## Contract Tests

SDK-generated fixtures are tested by writing a complete temporary Workflow OS project and running:

```text
workflow-os validate
```

Tests cover:

- minimal valid generated project
- approval-gated generated project
- invalid generated project failing Rust validation
- schema version emission
- lifecycle status emission
- sensitive field representation
- helper rejection of secret-like spec values

## Synchronization Strategy

Rust-owned schemas remain the source of truth. TypeScript types are manually mirrored in the initial SDK because schema-derived TypeScript generation has not been introduced yet.

This duplication is temporary and guarded by contract tests against the Rust CLI. Any SDK field or builder change must correspond to Rust-owned spec behavior and must be covered by CLI validation fixtures.

Future work may generate TypeScript types from Rust-owned JSON Schema artifacts once that generation path is stable.
