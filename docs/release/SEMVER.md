# Semantic Versioning Policy

Workflow OS uses semantic versioning for public releases.

Version format:

- `MAJOR.MINOR.PATCH`
- Pre-release versions may use suffixes such as `0.1.0-alpha.1`.
- The first public local kernel preview uses `0.1.0-preview.1`.
- The first public read-only integration preview uses `0.2.0-preview.1`.

## Major Versions

Major versions may include documented breaking changes to public schemas, CLI behavior, SDK contracts, runtime invariants, or persisted state formats.

Breaking changes require:

- ADR or release note justification.
- Migration notes where practical.
- Explicit changelog entry.

## Minor Versions

Minor versions may add backward-compatible capabilities, documented public concepts, schema fields, CLI commands, or SDK APIs.

Experimental features may appear in minor versions only when clearly marked.

## Patch Versions

Patch versions may include backward-compatible bug fixes, documentation corrections, security fixes, and non-breaking quality improvements.

## v0 Compatibility

Before `1.0.0`, public contracts may still evolve. Even during v0, breaking changes must be documented and must not be hidden inside unrelated work.

Rust crates and TypeScript packages should share the same preview version for coherent repository releases. The first preview release is `0.1.0-preview.1`; the first read-only integration preview release is `0.2.0-preview.1`.

The v0 local kernel preview treats the following as public preview surfaces:

- YAML spec shapes and schema versions under `workflowos.dev/v0`
- checked-in JSON Schemas under `schemas/v0/`
- Rust core public types exported by `workflow-core`
- CLI commands documented under `docs/cli/`
- TypeScript SDK spec-generation helpers
- event/state concepts documented under `docs/runtime/`

These surfaces are usable for preview evaluation, but they are not yet guaranteed stable at a `1.0.0` level. Breaking changes before `1.0.0` are allowed only when they are intentional, documented, tested where practical, and reflected in the changelog.

## Experimental Surfaces

The following surfaces are experimental in v0:

- CLI JSON output shape
- local filesystem state layout
- manually checked-in JSON Schema synchronization process
- TypeScript SDK helper ergonomics
- adapter contracts for future integrations
- audit and observability sink interfaces

Experimental surfaces may change in minor v0 releases with explicit release notes. They must not be presented as production-stable contracts.

## CLI JSON Output

`--json` output exists for preview automation and tests. It remains experimental through `0.2.0-preview.1` and is not yet a versioned stable machine-output contract. Changes to JSON fields, enum formatting, or response shape must still be documented during v0, but consumers should treat CLI JSON as preview-level until a future release explicitly promotes it.

Before CLI JSON can be promoted to a stable contract, Workflow OS must define a versioned response envelope, document compatibility rules, and add contract tests for every command that supports JSON output.
