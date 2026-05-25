# Rust And TypeScript Boundary

Workflow OS is a Rust-core plus TypeScript-SDK monorepo. This document defines the boundary between the canonical Rust implementation and the TypeScript SDK.

## Rust Responsibilities

Rust owns:

- Canonical schemas.
- Canonical validation.
- Runtime kernel.
- Event model.
- State model.
- Policy model.
- Audit model.
- Observability model.
- CLI.
- Compatibility rules for persisted and public contracts.

If Rust and TypeScript disagree, Rust is authoritative.

## TypeScript Responsibilities

The TypeScript SDK exists for developer ergonomics.

The SDK may:

- Help users construct spec files.
- Provide typed builders or helpers for public spec fields.
- Emit spec files compatible with Rust validation.
- Provide editor-friendly types generated from or checked against Rust-owned contracts.
- Include contract fixtures that prove SDK output validates through Rust.

The SDK must not:

- Maintain an incompatible parallel workflow model.
- Define independent runtime semantics.
- Bypass Rust validation.
- Claim support for behavior the Rust core does not validate.
- Implement adapter side effects as if they were core behavior.

## No Native Bindings Or WASM In v0

v0 must avoid native bindings or WASM for the TypeScript SDK unless a future ADR justifies an exception.

Reasons:

- Keep local development simple.
- Avoid packaging and platform complexity before core contracts stabilize.
- Preserve a clear boundary between spec generation and canonical validation.
- Prevent SDK ergonomics from becoming a shadow runtime.

## Contract Fixtures

Compatibility between TypeScript output and Rust validation must be proven with contract fixtures.

Future fixture tests must verify:

- SDK-emitted specs include schema version.
- SDK-emitted specs validate in Rust.
- Unsupported fields fail Rust validation.
- Experimental fields are clearly marked and validated according to policy.
- SDK examples do not imply unsupported runtime or adapter behavior.

Fixtures must be versioned with public schema changes.

## Generation Direction

The preferred direction is Rust-owned contracts flowing outward to TypeScript:

1. Rust defines or owns canonical schema contracts.
2. Spec schema artifacts are generated or checked from Rust-owned definitions.
3. TypeScript types and helpers are generated from or verified against those artifacts.
4. Contract fixtures prove compatibility.

Manual duplication is allowed only when it is temporary, clearly documented, and tested against Rust validation.

The initial TypeScript SDK manually mirrors the v0 spec shape and emits JSON-formatted spec files. This is accepted only because synchronization is guarded by Rust validation contract checks:

- package tests write SDK-generated projects and validate them with `workflow-os validate`
- the repo-level `npm run check:contracts` gate validates SDK minimal and approval-gated projects with Rust
- the same gate verifies an intentionally invalid SDK project and a schema-version mismatch fail Rust validation
- the same gate validates checked-in examples and checks that `schemas/v0/*.schema.json` are present and pinned to `workflowos.dev/v0`

Schema-derived TypeScript generation remains the preferred future direction. Until it exists, manual SDK type changes must be paired with Rust validation fixtures and schema updates in the same change.

## Review Requirements

Any change to the TypeScript SDK that adds public fields, builders, examples, validation behavior, or emitted spec shapes must answer:

- What Rust-owned schema or validation contract does this correspond to?
- How is compatibility tested?
- Does this imply runtime behavior that Rust does not implement?
- Does this introduce native binding, WASM, packaging, or supply-chain risk?

If the answer changes the architecture, write an ADR before implementation.
