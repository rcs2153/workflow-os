# Local Check Side-Effect Boundary Model Review

Review date: 2026-06-16

## 1. Executive Verdict

Needs blocker fixes.

The model-only implementation stayed within the approved scope and added the right local-check-specific side-effect boundary vocabulary. It preserved the existing `LocalCheckCommandContract` serialized shape, kept execution/default registration/CLI/schema/evidence/persistence/report artifacts/writes out of scope, and added useful validation around source-read-only, cache/build/temp output directories, disabled network posture, and fail-closed unsafe side-effect kinds.

One blocker remains before acceptance: the newly exported standalone `LocalCheckSideEffectBoundary` derives `Serialize` and serializes `permitted_output_directories` verbatim, while its `Debug` implementation intentionally hides those directory names. The serialization policy for that public boundary model is therefore not explicit or tested. Either the standalone boundary serialization must be made redaction-safe or non-serializable, or the implementation must explicitly document and test that bounded relative output directories are intentional serialized model data.

## 2. Scope Verification

The phase stayed within the approved model-only scope.

No accidental implementation was found for:

- live local command execution;
- live npm smoke checks;
- cargo, TypeScript, contract, integration, or provider check handlers;
- default handler registration;
- CLI exposure;
- workflow schema fields;
- automatic local check execution;
- command-output evidence;
- local check evidence attachment;
- local check result persistence;
- report artifact auto-writing;
- generic side-effect records;
- source writes;
- write-capable adapters;
- release posture changes.

## 3. Model Assessment

The implementation added the expected local-check-specific model concepts:

- `LocalCheckSideEffectKind`;
- `LocalCheckSideEffectBoundary`;
- `LocalCheckSideEffectBoundaryDefinition`.

The model is appropriately bounded to local checks rather than attempting to implement the broader generic side-effect boundary model. `SourceReadOnly`, `CacheWriteOnly`, `BuildOutputWrite`, and `TempWriteOnly` give the roadmap enough vocabulary to separate read-only checks from toolchain cache/build writes. `SourceWrite`, `NetworkAccess`, and `Unclassified` are represented but rejected for the current posture, which keeps the design fail-closed.

The implementation also preserves the existing coarse `LocalCheckSideEffectClass` as the compatibility layer and derives the finer-grained boundary from current contract fields.

## 4. Contract Compatibility Assessment

`LocalCheckCommandContract` stores the derived `side_effect_boundary` internally with `#[serde(skip)]`. That preserves the existing serialized contract shape while exposing the stricter boundary through a read-only accessor.

Deserialization of `LocalCheckCommandContract` reconstructs the derived boundary through `LocalCheckCommandContract::new`, so invalid serialized contract payloads still fail closed at construction time.

This is the right compatibility posture for this phase.

## 5. Validation Assessment

Validation is strong and deterministic:

- empty effect lists are rejected;
- duplicate effect kinds are rejected;
- `Unclassified` is rejected;
- `SourceWrite` is rejected;
- `NetworkAccess` is rejected;
- non-disabled network policy is rejected;
- cache/build/temp write kinds require explicit permitted output directories;
- source-read-only boundaries reject permitted output directories;
- permitted output directories reuse existing safe relative path validation;
- secret-like output directory values are rejected without leaking raw values;
- absolute output directory values are rejected without leaking raw values.

The coarse `BuildOrCacheWrites` mapping to both `CacheWriteOnly` and `BuildOutputWrite` is acceptable for compatibility and is documented as a known limitation.

## 6. Directory, Environment, And Network Assessment

The local check boundary keeps the correct v1 posture:

- source-read-only checks cannot declare output directories;
- cache/build/temp write-like checks must declare directories explicitly;
- source writes remain unsupported;
- network access remains unsupported;
- network policy remains disabled;
- environment allowlist behavior remains unchanged.

No ambient environment access, network enablement, or inferred output directories were introduced.

## 7. Privacy, Redaction, And Serde Assessment

`Debug` for `LocalCheckSideEffectBoundary` is redaction-safe for output directories: it reports a count rather than raw directory names. `Debug` for `LocalCheckCommandContract` also avoids raw executable, argument, environment variable, and output directory details.

`LocalCheckCommandContract` serialization remains compatible and intentionally still includes the existing `permitted_output_directories` field while skipping the derived `side_effect_boundary`.

The blocker is the standalone boundary model: `LocalCheckSideEffectBoundary` is public, exported, and derives `Serialize`, so serializing the boundary directly includes raw `permitted_output_directories`. There is no test proving this is intentional safe serialized data, and no test proving standalone boundary deserialization errors avoid leaking rejected path values. Because the model redacts those directories in `Debug`, the serialization policy needs to be explicit and covered before the phase is accepted.

## 8. Runtime And Non-Execution Boundary Assessment

The implementation did not cross the runtime boundary. It does not run local commands, register default handlers, mutate workflow state, append workflow events, emit audit events, persist local check results, create report artifacts, expose CLI output, or introduce write behavior.

That preserves the planned sequence: model first, review, blocker fix if needed, then future execution planning.

## 9. Test Quality Assessment

The tests cover the important model behavior:

- source-read-only boundary acceptance;
- cache/build/temp write directory requirements;
- unsafe effect kind rejection;
- secret-like and absolute output directory rejection without leaking through errors;
- contract-level fine-grained boundary exposure;
- output directory rejection on source-read-only contracts;
- build/cache write contract acceptance with explicit output directories;
- redaction-safe boundary `Debug`;
- serialized `LocalCheckCommandContract` shape preserving compatibility and skipping the derived boundary field.

Missing blocker-level coverage:

- standalone `LocalCheckSideEffectBoundary` serialization policy;
- standalone boundary invalid deserialization fail-closed behavior;
- non-leaking standalone boundary deserialization errors for rejected output directory values.

## 10. Documentation Review

The implementation report and planning docs accurately state that this phase is model-only and that live execution, default registration, CLI exposure, schema fields, evidence attachment, persistence, report artifacts, generic side-effect records, writes, and release posture changes remain unimplemented.

The report also states that `Debug` redacts boundary output directories, which is true. The remaining documentation gap is that standalone boundary serialization behavior is not stated as either intentionally allowed or intentionally redacted.

## 11. Blockers

1. `LocalCheckSideEffectBoundary` serialization policy is unsafe or under-specified.

   The type is public/exported and derives `Serialize`, which serializes `permitted_output_directories` verbatim. Since the same field is redacted from `Debug`, reviewers cannot tell whether direct serialization is intended public data or an accidental leak path.

   Acceptable fixes include one of:

   - make standalone boundary serialization redaction-safe;
   - remove or avoid `Serialize` for `LocalCheckSideEffectBoundary` if standalone serialization is not required;
   - explicitly document that validated relative output directories are serialized model data and add focused tests proving secret-like, absolute, parent-traversal, and otherwise invalid serialized boundary payloads fail closed without leaking raw values.

## 12. Non-Blocking Follow-Ups

- Consider splitting the coarse `BuildOrCacheWrites` compatibility class into clearer cache/build declarations only when a compatible schema path exists.
- Decide whether repo-local cache directories are allowed or whether caches must be outside the repository.
- Decide whether build output directories should require `.gitignore` validation before live execution.
- Keep live docs-check smoke execution deferred until after the side-effect boundary model is accepted.
- Keep generic side-effect records deferred to the broader side-effect boundary ADR path.

## 13. Recommended Next Phase

Recommended next phase: **local check side-effect boundary model blocker fix**.

The fix should address standalone boundary serialization policy and tests without adding live command execution, default registration, CLI exposure, workflow schema fields, evidence attachment, local check result persistence, report artifact writing, generic side-effect records, source writes, or release posture changes.

## 14. Validation

Validation commands for this review:

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
