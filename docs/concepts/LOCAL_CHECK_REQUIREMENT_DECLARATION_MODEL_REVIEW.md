# Local Check Requirement Declaration Model Review

## 1. Executive Verdict

**Phase accepted; proceed to the canonical declaration-set record and pure
resolver.**

The implementation adds a bounded, validated, schema-facing local-check
requirement declaration to workflow steps without granting runtime authority.
Two review blockers were corrected before acceptance: declarations for the
same command reference can no longer evade duplicate-obligation validation by
varying other fields, and the checked-in JSON Schema and TypeScript SDK now
match the canonical Rust model.

## 2. Scope Verification

The phase stayed within the approved declaration-model scope.

It added typed requirement vocabulary, a default-empty step field, project
validation, serde support, JSON Schema and TypeScript contract parity, focused
tests, documentation, and a narrow dogfood runner correction required to
govern schema-facing phases.

It did not add command resolution, declaration-set records, fingerprints,
immutable-bundle publication, local-check execution, automatic handlers,
structural-coverage authority, aggregate posture, proportional-governance
reassessment, executor gates, evidence, report or artifact behavior, provider
calls, SideEffect execution, writes, hosted behavior, examples, or release
changes.

## 3. Model Assessment

`LocalCheckRequirementDeclaration` is private-field, constructor-validated
vocabulary. It captures:

- stable requirement and command references;
- required or optional posture;
- minimum attestation assurance;
- accepted result statuses;
- freshness;
- exact immutable-run binding;
- truncation posture;
- network maximum; and
- SideEffect maximum.

The model is intentionally not a resolved command contract or an executable
gate. `command_id` remains a bounded reference whose existence and exact
contract compatibility are deferred to the pure resolver.

`StepDefinition.local_check_requirements` defaults to an empty list, preserving
existing workflow specifications. In this phase, empty means no authored
declarations; it is not an authoritative claim that runtime check coverage is
complete.

## 4. Validation Assessment

Construction fails closed unless:

- assurance is `kernel_observed_local_process`;
- accepted statuses are exactly one `passed` value;
- exact immutable-run binding is required;
- network access is disabled; and
- SideEffect posture is classified.

Existing bounded freshness validation remains in force. Requirement and command
identifiers use the established local-check identifier validator.

Project validation rejects duplicate requirement IDs and more than one
declaration for the same command reference within a step. The latter rule was
tightened during review. Before that correction, two declarations could name
the same command while varying requirement level or another field, leaving a
future resolver with conflicting obligations.

Validation codes are stable and errors do not include caller-supplied IDs or
declaration values.

## 5. Serde And Contract-Surface Assessment

Valid declarations round trip through serde. Deserialization reconstructs the
model through its validated constructor and emits a fixed error for invalid
wire values.

The checked-in v0 JSON Schema and TypeScript SDK now expose the same field and
the same constrained vocabulary as Rust:

- required or optional level;
- kernel-observed assurance;
- passed-only accepted status;
- no-reuse or bounded maximum-age freshness;
- exact binding fixed to true;
- disabled network; and
- no-source-writes or bounded build/cache SideEffect posture.

The initial omission of these surfaces was corrected during review. No
workflow-declared runtime enforcement is implied by schema representation.

## 6. Privacy And Redaction Assessment

- Declaration `Debug` output redacts requirement and command IDs.
- Requirement ID `Debug` output is redacted.
- Invalid constructors and deserialization do not echo supplied IDs or values.
- Secret-like identifiers fail closed through the existing local-check
  validation boundary.
- The model stores no command arguments, environment values, process output,
  provider payloads, credentials, source contents, paths, or parser payloads.

Serialized declarations intentionally contain bounded authored references and
posture vocabulary. They must still be handled as workflow specification data;
serialization is not a secrecy boundary.

## 7. Determinism And Compatibility Assessment

The new field is default-empty, so existing specs deserialize and retain their
previous behavior. Duplicate validation is deterministic and step-scoped.

The declaration list is not yet canonicalized, fingerprinted, or resolved.
Ordering therefore remains authored input rather than an authoritative
declaration-set identity. The next phase must define deterministic canonical
ordering and identity before immutable-bundle publication.

## 8. Test Quality Assessment

Focused Rust tests cover:

- valid construction and read-only accessors;
- weak assurance rejection;
- empty, failed, and duplicate accepted-status rejection;
- exact binding and classified SideEffect requirements;
- serde round trip and fail-closed invalid deserialization;
- redaction-safe `Debug` and errors;
- secret-like requirement ID rejection;
- valid workflow-project parsing;
- duplicate requirement IDs;
- duplicate command obligations; and
- conflicting declarations for one command.

Repository contract checks cover JSON Schema and TypeScript compatibility. The
full workspace suite passed, retaining only explicitly ignored opt-in live
provider tests.

## 9. Dogfood Runner Correction Assessment

The governed phase initially could not start because the generic runner
non-goal prohibited all schema changes, including the exact schema-facing work
authorized by `dg/spec-field-operationalization`.

The correction is appropriately narrow: ordinary phases still prohibit schema
changes, while the dedicated spec-field phase permits only its explicitly
approved field scope. Focused runner tests cover both postures. The correction
does not authorize arbitrary schema work or change runtime execution.

## 10. Documentation Assessment

The roadmap, implementation plan, workflow specification guide, phase report,
and blocker-fix report accurately state:

- declaration vocabulary and schema-facing validation are implemented;
- command references are unresolved;
- the empty declaration list is not authoritative runtime coverage;
- checks are not executed;
- immutable bundles are not published from these declarations; and
- runtime gates, providers, SideEffects, and writes remain unimplemented.

The documentation does not overclaim runtime enforcement.

## 11. Blockers

None after the duplicate-obligation and cross-language contract corrections.

## 12. Non-Blocking Follow-Ups

- Define an explicit allowlisted command-contract inventory.
- Reject missing and ambiguous command references in a pure resolver.
- Compare each declaration's maxima with the resolved command contract.
- Define a canonical declaration-set record, deterministic ordering, and
  content-derived identity.
- Keep bundle publication, structural-coverage authority, aggregate posture,
  proportional-governance reassessment, and executor enforcement in later,
  separately reviewed phases.

## 13. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `git diff --check`: passed.

## 14. Governed Review

- workflow: `dg/review`
- run: `run-1784970755605182000-2`
- approval: `approval/run-1784970755605182000-2/review-scope-approved`
- presentation: `presentation/fbea25898b546f9f`
- approval outcome: granted by delegated maintainer through proof enforcement
- event summary: 39 events, one approval, no retries, no escalations
- phase-close proof disclosure: `proof_record_read_error`; the persisted
  presentation identity is recorded above, but the helper could not read the
  proof records without expanding disclosure
- kernel boundary: governance coordination only; review, edits, and validation
  ran outside the kernel

## 15. Recommended Next Phase

Proceed to **canonical local-check declaration-set record and pure resolver
implementation**.

The phase should consume explicit workflow declarations and an explicit
allowlisted command-contract inventory, reject unknown or ambiguous references,
compare declared constraints against exact contracts, and return deterministic
model-only records. It must not publish immutable bundles, execute checks,
register handlers, convert structural coverage into authority, reassess
proportional governance, add executor gates, call providers, or enable writes.
