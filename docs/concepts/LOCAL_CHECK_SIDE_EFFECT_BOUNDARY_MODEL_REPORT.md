# Local Check Side-Effect Boundary Model Report

Report date: 2026-06-16

## 1. Executive Summary

Implemented the model-only local check side-effect boundary for Workflow OS. The implementation adds fine-grained local check side-effect vocabulary and validation while preserving the existing serialized `LocalCheckCommandContract` shape and existing non-executing/default-safe local check posture.

No live local command execution, default registration, CLI exposure, workflow schema fields, command-output evidence, local check evidence attachment, local check result persistence, report artifact auto-writing, generic side-effect records, writes, or release posture changes were introduced.

## 2. Scope Completed

- Added `LocalCheckSideEffectKind`.
- Added `LocalCheckSideEffectBoundary`.
- Added `LocalCheckSideEffectBoundaryDefinition`.
- Added contract-level derivation of a fine-grained boundary from the existing coarse `LocalCheckSideEffectClass` and `permitted_output_directories`.
- Added validation for:
  - source-read-only local checks;
  - cache/build/temp write declarations requiring explicit output directories;
  - source-write rejection;
  - network-access rejection;
  - unclassified fail-closed behavior;
  - duplicate side-effect kind rejection;
  - secret-like or unsafe output directory rejection.
- Added redaction-safe `Debug` behavior for the boundary.
- Exported the new model types from `workflow-core`.
- Added focused tests in `crates/workflow-core/tests/local_check.rs`.
- Updated roadmap and local check planning docs.

## 3. Scope Explicitly Not Completed

- No live npm smoke test.
- No live local command execution.
- No cargo, TypeScript, contract, integration, or provider check handler.
- No default handler registration.
- No CLI exposure.
- No workflow schema fields.
- No automatic check execution.
- No command-output evidence.
- No local check evidence attachment.
- No local check result persistence.
- No report artifact auto-writing.
- No generic side-effect records.
- No source writes.
- No write-capable adapters.
- No release posture change.

## 4. Model Types Added

- `LocalCheckSideEffectKind`
  - `SourceReadOnly`
  - `CacheWriteOnly`
  - `BuildOutputWrite`
  - `TempWriteOnly`
  - `SourceWrite`
  - `NetworkAccess`
  - `Unclassified`
- `LocalCheckSideEffectBoundary`
- `LocalCheckSideEffectBoundaryDefinition`

`LocalCheckCommandContract` now stores a derived fine-grained `LocalCheckSideEffectBoundary` internally. The field is skipped during serialization to preserve the existing contract wire shape while still enforcing stricter validation at construction and deserialization boundaries.

## 5. Validation Boundary Summary

Validation is fail-closed:

- `Unclassified` is rejected.
- `SourceWrite` is rejected.
- `NetworkAccess` is rejected.
- Non-disabled network policy is rejected.
- Cache/build/temp write kinds require explicit permitted output directories.
- Source-read-only boundaries cannot declare output directories.
- Output directories must be safe relative paths.
- Secret-like output directory values are rejected without leaking.
- Duplicate side-effect kinds are rejected.

The existing `LocalCheckSideEffectClass` remains as a compatibility/coarse classification. Fine-grained behavior is exposed through `LocalCheckSideEffectBoundary`.

## 6. Redaction And Privacy Summary

The model does not store raw command output, environment values, provider payloads, source contents, parser payloads, credentials, authorization headers, private keys, or token-like values.

`Debug` for `LocalCheckSideEffectBoundary` reports the side-effect kinds and output directory count, not raw output/cache/build/temp directory names.

Direct `LocalCheckSideEffectBoundary` serialization intentionally includes only validated relative output directory names because they are model configuration, not raw command output or local absolute paths. Invalid serialized boundary payloads fail closed and do not leak rejected directory values through error messages.

Validation errors use stable codes and do not include raw directory values or secret-like values.

## 7. Test Coverage Summary

Added focused tests for:

- source-read-only boundary acceptance;
- cache/build/temp write directory requirements;
- source-write rejection;
- network-access rejection;
- unclassified rejection;
- secret-like output directory rejection;
- absolute output directory rejection;
- contract-level fine-grained source-read-only boundary exposure;
- rejection of output directories on source-read-only contracts;
- build/cache write contracts requiring safe output directories;
- redaction-safe boundary debug output;
- direct boundary serialization for validated relative output directory names;
- invalid serialized boundary directory rejection without leaking rejected values;
- contract serialization preserving existing shape and not serializing the derived boundary field.

Existing local check focused tests pass.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test local_check`
  - Passed.
- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed after adding the new boundary to the redaction-safe `LocalCheckCommandContract` `Debug` implementation.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 9. Remaining Known Limitations

- The model does not detect source writes before/after command execution.
- No live command runner uses the new boundary to enforce filesystem behavior.
- The coarse `BuildOrCacheWrites` contract class maps to both cache and build output fine-grained kinds because existing serialized contracts do not distinguish them.
- Cache directories are not yet required to be outside the repository or Git-ignored.
- Build output directories are not yet validated against `.gitignore`.
- Live `DocsCheck` smoke posture remains deferred.

## 10. Recommended Next Phase

Recommended next phase: **local check side-effect boundary model review**.

The review should verify that the model-only boundary is backward-compatible, redaction-safe, deterministic, and still non-executing. It should also confirm that live command execution, default registration, CLI exposure, schema fields, evidence attachment, local check result persistence, report artifact writing, generic side-effect records, and writes remain unimplemented.
