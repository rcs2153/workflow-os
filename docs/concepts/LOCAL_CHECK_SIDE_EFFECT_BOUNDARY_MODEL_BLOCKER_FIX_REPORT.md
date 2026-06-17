# Local Check Side-Effect Boundary Model Blocker Fix Report

Report date: 2026-06-16

## 1. Executive Summary

Fixed the local check side-effect boundary model review blocker by making the standalone `LocalCheckSideEffectBoundary` serialization policy explicit and adding focused regression tests for serialization and deserialization behavior.

The fix keeps the phase model-only. It does not add live local command execution, default handler registration, CLI exposure, workflow schema fields, command-output evidence, local check evidence attachment, local check result persistence, report artifact auto-writing, generic side-effect records, source writes, write-capable adapters, or release posture changes.

## 2. Blocker Fixed

The review found that `LocalCheckSideEffectBoundary` was public/exported and serializable, while its `Debug` implementation redacted `permitted_output_directories`. The missing piece was an explicit policy and tests explaining whether standalone boundary serialization was allowed to include validated output directory names.

The fix resolves that by documenting and testing the chosen policy:

- direct boundary serialization may include validated relative output directory names;
- those directory names are model configuration, not raw command output or local absolute paths;
- `Debug` continues to redact directory names and expose only a count;
- invalid serialized boundary payloads fail closed;
- deserialization errors do not leak rejected path or secret-like values.

## 3. Implementation Approach

The implementation uses the smallest accepted fix path from the review:

- kept `Serialize` for `LocalCheckSideEffectBoundary`;
- added code documentation to state the direct serialization policy;
- added focused tests showing valid relative output directories serialize and round-trip;
- added focused tests showing secret-like, absolute, and parent-traversal serialized directory values fail closed without leaking.

No model redesign was introduced.

## 4. Validation Boundary Summary

The validation boundary remains fail-closed:

- `Unclassified` remains rejected.
- `SourceWrite` remains rejected.
- `NetworkAccess` remains rejected.
- Non-disabled network policy remains rejected.
- Cache/build/temp write kinds still require explicit output directories.
- Source-read-only boundaries still reject output directories.
- Output directories must still be validated safe relative paths.
- Secret-like, absolute, and parent-traversal serialized directory payloads fail deserialization without leaking the rejected value.

## 5. Redaction And Privacy Summary

`LocalCheckSideEffectBoundary` now has explicit privacy semantics:

- `Debug` redacts output directory names and reports only the count.
- Serialization includes validated relative output directory names as bounded model configuration.
- Deserialization validates all serialized output directory names before storage.
- Invalid deserialization errors use stable codes and do not include raw directory values.
- The model still does not store raw command output, environment values, provider payloads, source contents, parser payloads, credentials, authorization headers, private keys, or token-like values.

## 6. Test Coverage Summary

Added focused tests for:

- direct standalone boundary serialization of validated relative output directory names;
- standalone boundary serde round trip;
- secret-like serialized output directory rejection without leaking;
- absolute serialized output directory rejection without leaking;
- parent-traversal serialized output directory rejection without leaking.

Existing local check, executor, WorkReport, EvidenceReference, Diagnostic, adapter telemetry, and runtime tests still pass.

## 7. Commands Run And Results

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 8. Remaining Known Limitations

- No live command runner uses the boundary to enforce filesystem behavior.
- No source-write detection exists before or after command execution.
- The coarse `BuildOrCacheWrites` contract class still maps to both cache and build output fine-grained kinds for compatibility.
- Cache directories are not yet required to be outside the repository or Git-ignored.
- Build output directories are not yet validated against `.gitignore`.
- Live `DocsCheck` smoke posture remains deferred.

## 9. Recommended Next Phase

Recommended next phase: **local check side-effect boundary model blocker fix review**.

That review should verify the serialization policy, regression tests, non-leaking errors, and continued model-only scope before moving back to broader local check handler execution planning.
