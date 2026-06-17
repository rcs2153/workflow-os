# Local Check Side-Effect Boundary Model Blocker Fix Review

Review date: 2026-06-17

## 1. Executive Verdict

Blocker fixed; proceed to opt-in live DocsCheck smoke planning.

The blocker fix resolves the prior review finding by making the standalone `LocalCheckSideEffectBoundary` serialization policy explicit and covering the accepted policy with focused regression tests. The implementation keeps direct serialization of validated relative output directory names as intentional model configuration, preserves redacted `Debug` output, and validates deserialized boundary payloads before construction.

No additional blocker was found. The fix remains model-only and does not add live command execution, default registration, CLI exposure, workflow schema fields, evidence attachment, local check result persistence, report artifact writing, generic side-effect records, source writes, write-capable adapters, or release posture changes.

## 2. Scope Verification

Reviewed references:

- `docs/ENGINEERING_STANDARD.md`
- `docs/implementation-plans/local-check-side-effect-boundary-plan.md`
- `docs/concepts/LOCAL_CHECK_SIDE_EFFECT_BOUNDARY_MODEL_REVIEW.md`
- `docs/concepts/LOCAL_CHECK_SIDE_EFFECT_BOUNDARY_MODEL_BLOCKER_FIX_REPORT.md`
- `crates/workflow-core/src/local_check.rs`
- `crates/workflow-core/tests/local_check.rs`

The reviewed fix stayed within the approved blocker scope. It addressed standalone side-effect boundary serialization policy and tests only.

No scope expansion was found for:

- live local command execution;
- default local check registration;
- CLI behavior;
- workflow schema fields;
- persistence changes;
- report artifact writing;
- evidence attachment;
- generic side-effect records;
- source writes;
- write-capable adapters;
- release posture changes.

The existing explicit docs-check handler and registration profile remain non-default and caller-supplied. The reviewed blocker fix did not broaden that posture.

## 3. Original Blocker Restatement

The prior model review found one blocker: `LocalCheckSideEffectBoundary` was public and derived `Serialize`, while its `Debug` implementation intentionally redacted `permitted_output_directories`.

That left the standalone boundary serialization policy under-specified. Reviewers could not tell whether serializing validated relative output directories was intentional public model data or an accidental leak path.

The accepted fix options were:

- make standalone boundary serialization redaction-safe;
- remove standalone serialization;
- explicitly document that validated relative output directories are serialized model data and add tests proving invalid serialized payloads fail closed without leaking raw values.

## 4. Fix Approach Assessment

The fix chose the third accepted path.

`LocalCheckSideEffectBoundary` now documents that direct serialization includes validated relative output directory names because they are model configuration, not raw command output or local absolute paths. The custom `Debug` implementation continues to disclose only the permitted output directory count.

The implementation also uses custom `Deserialize` for `LocalCheckSideEffectBoundary`, routing wire payloads through `LocalCheckSideEffectBoundary::new`. That preserves validation on deserialization rather than allowing serialized fields to bypass the constructor.

This is a minimal, production-shaped fix. It avoids a model redesign, preserves the existing public contract shape, and makes the accepted privacy policy reviewable.

## 5. Validation Boundary Assessment

The boundary remains fail-closed.

Confirmed behavior:

- empty effect lists are rejected;
- duplicate effect kinds are rejected;
- `Unclassified` is rejected;
- `SourceWrite` is rejected;
- `NetworkAccess` is rejected;
- non-disabled network policy is rejected;
- cache/build/temp write kinds require explicit output directories;
- source-read-only boundaries reject output directories;
- output directories are validated as relative safe paths;
- absolute paths and parent traversal are rejected;
- secret-like output directory values are rejected.

The validation helper still rejects shell metacharacters and whitespace in output directory values through command-token validation, then rejects absolute paths or values containing `..`. These checks are sufficient for the current model-only boundary.

## 6. Privacy/Redaction Assessment

The blocker-level privacy gap is fixed.

The new policy is explicit:

- serialized standalone boundaries may include validated relative output directory names;
- `Debug` for `LocalCheckSideEffectBoundary` redacts directory names and reports only a count;
- `Debug` for `LocalCheckCommandContract` redacts command and directory details by count;
- invalid deserialization errors use stable validation codes and do not echo rejected directory values.

The tests cover secret-like, absolute, and parent-traversal serialized directory payloads and assert that the raw rejected values do not appear in error text.

This posture is acceptable because the serialized values are bounded configuration strings, not command output, local absolute paths, source contents, provider payloads, environment values, credentials, or tokens.

## 7. Regression Assessment

No regression was found in the model-only boundary.

The fix preserves:

- `LocalCheckCommandContract` serialization compatibility;
- `#[serde(skip)]` for the derived `side_effect_boundary` on command contracts;
- reconstruction of the derived boundary during `LocalCheckCommandContract` deserialization;
- model-only execution posture;
- disabled network policy;
- explicit non-default docs-check profile behavior;
- existing local check result and report citation boundaries.

The known compatibility limitation remains acceptable: coarse `BuildOrCacheWrites` still maps to both `CacheWriteOnly` and `BuildOutputWrite` until a future compatible schema path can represent these distinctions directly.

## 8. Test Quality Assessment

The blocker fix adds the right focused test coverage.

New or relevant coverage verifies:

- standalone boundary serialization includes valid relative output directory names by policy;
- standalone boundary serde round-trip works through the validating deserializer;
- secret-like serialized output directories fail closed without leaking;
- absolute serialized output directories fail closed without leaking;
- parent-traversal serialized output directories fail closed without leaking;
- boundary `Debug` redacts output directory names;
- command contract serialization still omits the derived `side_effect_boundary` field.

The tests prove the exact blocker behavior rather than merely constructing the model. No additional blocker-level test gap was found.

## 9. Documentation Review

The blocker fix report accurately describes the chosen serialization policy, validation boundary, redaction posture, test coverage, and remaining known limitations.

The code documentation on `LocalCheckSideEffectBoundary` now states the direct serialization policy in the same place as the public type. That closes the prior documentation gap.

This review document intentionally does not change runtime docs, CLI docs, schema docs, or release posture because the fix did not introduce runtime behavior or public CLI/schema activation.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- During opt-in live DocsCheck smoke planning, decide whether the npm cache directory must be outside the repository or may be a declared repo-local ignored path.
- Before any live smoke, document how the explicit npm executable path and explicit npm cache directory are supplied without ambient `PATH` discovery or credential inheritance.
- Keep source-write detection out of the blocker fix acceptance, but decide whether a future live phase needs before/after source tree checks in addition to declared directory policy.
- Keep cargo, TypeScript, contract, integration, and provider handlers deferred until their build/cache/network boundaries are planned separately.

## 12. Recommended Next Phase

Recommended next phase: opt-in live DocsCheck smoke planning.

That phase should remain narrow. It should plan explicit npm path handling, explicit npm cache policy, sanitized environment construction, disabled network posture, source-tree protection expectations, validation commands, and review checkpoints before any live npm execution is added.
