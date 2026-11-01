# Capability-Gated GitHub Read-Only Contract Foundation Review

Status: Accepted after one blocking contract-closure correction.

## Review Scope

This review assessed only the Phase 1 dark contract foundation for the exact
`github.repository.metadata` operation:

- Rust, JSON Schema, and TypeScript representation parity;
- exact operation and project activation validation;
- access-reference and resource non-disclosure;
- authoritative-execution prerequisite enforcement;
- immutable run-bundle binding and compatibility; and
- proof that the change does not activate provider or CLI behavior.

Provider dispatch, transient credential delivery, authority derivation,
recovery execution, additional operations, mutations, and public selection
were strict non-goals.

## Finding And Correction

The initial implementation validated the exact adapter identifier, operation,
integration absence, and adapter capability, but did not require the enclosing
skill's complete accepted shape. A skill with unrelated inputs or extra
capabilities could therefore select the exact operation.

This was blocking because the operation identity is intended to be a closed
security contract, not a loose routing hint.

The correction requires all of the following in both Rust and the TypeScript
SDK:

- exactly one adapter requirement;
- adapter `symbolic/github-read-only`;
- operation `github.repository.metadata`;
- no integration identifier;
- exact adapter capability `external.read`;
- exact skill capabilities `external.read` and `adapter.invoke`;
- exactly two required nonsensitive string inputs, `owner` and `repo`;
- no input examples; and
- no additional input fields, required names, capabilities, or adapters.

Focused negative tests prove incomplete or widened contracts fail closed.

## Security Assessment

The corrected slice preserves the intended boundaries:

- project declaration remains constraint, not authority;
- access material is neither represented nor read;
- endpoint, access reference, and resource targets are closed and bounded;
- immutable run hashing commits the provider-read declaration;
- legacy declarations and bundles remain non-live; and
- no provider, network, CLI, or external-write path is enabled.

## Validation

- focused Rust project-spec, project-validation, and immutable-bundle tests:
  61 passed;
- TypeScript typecheck, lint, and tests: passed, including 15 tests;
- `cargo fmt --all -- --check`: passed;
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed;
- pre-correction full `cargo test -p workflow-core`: passed;
- pre-correction full `cargo test --workspace --all-targets`: passed;
- pre-correction full `npm run check`: passed; and
- `git diff --check`: passed.

The review correction changed only deterministic validation and focused tests;
the post-correction focused suites cover that changed behavior.

## Governed Review Record

- Workflow: `dg/review`
- Run: `run-1793477296317809000-2`
- Approval: `approval/run-1793477296317809000-2/review-scope-approved`
- Presentation: `presentation/a4f04ba21f6f950e`
- Presentation hash:
  `a4f04ba21f6f950eb57ea009a2f3e5b9f88e3f6b9cee8a5ed77a332ff0060e34`
- Approval outcome: granted under delegated maintainer authority with persisted
  presentation proof.

The Workflow OS kernel governed scope, approval, and phase state. Codex used
the local editor and deterministic toolchains as the execution layer.

## Decision

Accept Phase 1 after correction. Phase 2 may begin only as a Core-private,
no-write slice covering Core-owned authority derivation, separate transient
access, atomic operational-window and attempt lifecycle, terminal truth, and
ambiguity recovery. Public provider selection and live transport remain
blocked.
