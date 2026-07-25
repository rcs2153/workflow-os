# Local Check Requirement Declaration Model Report

## 1. Executive Summary

Workflow steps can now declare typed local-check requirements through a
default-empty `local_check_requirements` field. The implementation is a
schema-facing model and validation slice only. It does not resolve command
contracts, execute checks, publish immutable-bundle records, claim
authoritative structural coverage, or enforce an executor gate.

## 2. Scope Completed

- Added a validated, redaction-safe local-check requirement identifier.
- Added required and optional requirement-level vocabulary.
- Added a private validated declaration with explicit command, assurance,
  accepted-status, freshness, immutable-run binding, truncation, network, and
  SideEffect fields.
- Added a default-empty `StepDefinition.local_check_requirements` field.
- Added deterministic duplicate ID and duplicate command-obligation project
  validation.
- Added fail-closed serde and focused privacy/compatibility tests.
- Updated the checked-in v0 JSON Schema, TypeScript SDK contract, and workflow
  spec documentation for the exact new field.

## 3. Scope Explicitly Not Completed

This phase does not add:

- command-contract inventory resolution or unknown-command rejection;
- declaration-set records, fingerprints, or immutable-bundle publication;
- local-check execution, handlers, shell commands, or automatic checks;
- authoritative structural coverage or aggregate evidence/check posture;
- proportional-governance reassessment or executor gates;
- evidence, report, artifact, CLI, provider, SideEffect execution, write,
  hosted, schema-artifact, example, or release behavior.

## 4. Model And Validation Boundary

The v0 declaration accepts only `kernel_observed_local_process` assurance and
exactly one accepted status, `passed`. Exact immutable-run binding is
mandatory. Network maximum is disabled. SideEffect posture must be classified
as no source writes or bounded build/cache writes; unclassified posture fails
closed. Freshness uses the already bounded attestation freshness type.

Requirement and command identifiers are validated and redacted from `Debug`.
Invalid serialized declarations return a fixed non-leaking serde error.
Project validation rejects duplicate IDs and multiple declarations for the
same command reference within one step.

## 5. Compatibility

`local_check_requirements` uses an empty default, so existing workflow specs
remain valid and preserve their prior behavior. The empty list means no authored
requirements in this model slice; it does not yet become an authoritative
runtime coverage record.

## 6. Tests Added

Focused tests cover:

- valid model construction and accessors;
- weak assurance rejection;
- empty, failing, and duplicate accepted-status rejection;
- exact immutable-run binding and classified SideEffect requirements;
- serde round trip and fail-closed invalid deserialization;
- redaction-safe `Debug` and non-leaking errors;
- secret-like requirement ID rejection;
- valid YAML project declaration parsing; and
- duplicate declaration ID and command-obligation diagnostics.

## 7. Governed Execution

- workflow: `dg/spec-field-operationalization`
- run: `run-1784968194949625000-2`
- approval: `approval/run-1784968194949625000-2/implementation-scope-approved`
- presentation: `presentation/83160aed55c1203c`
- approval outcome: granted by delegated maintainer through proof enforcement
- event summary: 44 events, one approval, no retries, no escalations
- phase-close proof disclosure: `proof_record_read_error`; the persisted
  presentation identity is recorded above, but the helper could not read the
  proof records without broadening disclosure
- kernel boundary: governance coordination only; edits and validation ran
  outside the kernel

The phase followed the focused runner blocker fix documented in
[Governed Spec-Field Phase Schema-Scope Blocker Fix Report](GOVERNED_SPEC_FIELD_PHASE_SCHEMA_SCOPE_BLOCKER_FIX_REPORT.md).

Maintainer review found and corrected two contract blockers: conflicting
declarations for one command could evade semantic duplicate detection, and the
checked-in JSON Schema/TypeScript SDK initially omitted the canonical Rust
field. Duplicate identity now binds to the command reference, and all supported
contract surfaces represent the same declaration shape.

## 8. Validation

The following checks passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

The workspace suite retained only its explicitly ignored opt-in live tests; no
required test was skipped.

## 9. Known Limitations

The declaration's `command_id` is a validated reference, not a resolved
contract. A later pure resolver must reject missing or ambiguous command
contracts, compare declaration maxima against the exact contract, and produce
canonical deterministic records before any runtime consumer may treat the list
as authoritative.

## 10. Recommended Next Phase

The phase-level maintainer review is documented in
[Local Check Requirement Declaration Model Review](LOCAL_CHECK_REQUIREMENT_DECLARATION_MODEL_REVIEW.md).
Proceed next with the canonical declaration-set record and pure resolver only.
