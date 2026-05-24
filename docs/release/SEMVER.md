# Semantic Versioning Policy

Workflow OS uses semantic versioning for public releases.

Version format:

- `MAJOR.MINOR.PATCH`
- Pre-release versions may use suffixes such as `0.1.0-alpha.1`.

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
