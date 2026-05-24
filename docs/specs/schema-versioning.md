# Schema Versioning

Every Workflow OS project/spec file must declare `schema_version`.

The only supported v0 schema version is:

```text
workflowos.dev/v0
```

## Rules

- Missing `schema_version` is an error.
- Unsupported `schema_version` is an error.
- Schema versions are checked before typed parsing.
- Rust owns the canonical schema model.
- JSON Schemas under `schemas/v0/` are checked-in compatibility artifacts.
- TypeScript SDK output must remain compatible with Rust validation.

## JSON Schema Strategy

Workflow OS currently uses checked-in JSON Schemas.

Checked-in schemas are used because:

- The Rust model is still early.
- The repository does not yet include schema generation tooling.
- We want editor/tooling support before adding generation complexity.

Synchronization rule:

- Any change to Rust spec structs must update the matching schema file in the same change.
- Any schema change must include Rust parser tests or fixture tests proving compatibility.
- Future schema generation may replace checked-in schemas only through an ADR or documented migration.

## Content Hashing

Spec content hashes must be deterministic.

For v0 parser tests, YAML content is parsed and converted into canonical JSON with sorted object keys before hashing. Equivalent mapping order must produce the same hash.

Future workflow run creation must document its exact canonicalization rules before using content hashes as runtime identity.

## Compatibility

Before `1.0.0`, schema changes may still occur. Even during v0, breaking changes must be explicit, documented, and tested.
