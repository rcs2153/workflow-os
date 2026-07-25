# Immutable Local Check Execution Binding Core Model Report

## 1. Executive Summary

Workflow OS now has a payload-free core model that freezes the exact local-check
command, registered-handler selection, effective execution policy, and immutable
run context before a future check observation occurs.

The model does not execute a check or prove that one ran. No verifier, accepted
attestation, runtime integration, persistence, event, schema, CLI, provider,
SideEffect, or write behavior was added.

## 2. Scope Completed

- Added `ImmutableLocalCheckExecutionBinding` and its versioned algorithm.
- Added typed, honest registered-handler selection metadata.
- Added canonical command-contract and effective-policy fingerprints.
- Bound immutable bundle, workflow, run, step, skill, command, handler selection,
  policy, and creation time into one content-addressed fingerprint.
- Added safe accessors, redaction-safe Debug, fail-closed deserialization, and
  stable non-leaking errors.
- Added focused construction, canonicalization, mismatch, tamper, serde,
  fingerprint-vector, privacy, and Debug tests.

## 3. Scope Explicitly Not Completed

- process or check execution;
- kernel observation or independent-proof verifier;
- accepted attestation records;
- executor, approval, report, artifact, or proportional-governance integration;
- persistence, cache reuse, events, evidence, audit projection, or recovery;
- schemas, SDKs, CLI, UI, examples, providers, SideEffects, or writes;
- handler implementation attestation, cryptographic provenance, hosted runners,
  enterprise identity, or release changes.

## 4. Model Types Added

- `ImmutableLocalCheckExecutionBindingAlgorithm`
- `ImmutableLocalCheckHandlerRegistrationMode`
- `ImmutableLocalCheckHandlerPosture`
- `ImmutableLocalCheckHandlerSelection`
- `ImmutableLocalCheckExecutionBindingDefinition`
- `ImmutableLocalCheckExecutionBinding`
- `compute_local_check_command_contract_fingerprint`

## 5. Binding Boundary

The binding references the existing immutable run-bundle root rather than
pretending local-check commands are current bundle definition records. It
commits the resolved workflow/run/step/skill identity, complete command contract,
handler selection, effective policy, and creation time.

The command fingerprint includes executable and fixed arguments, working
directory, environment-name policy, network, timeout, SideEffect boundary,
bounded output capture, redaction, and citation posture. Semantically unordered
environment, SideEffect, output-directory, and citation sets are canonicalized.

## 6. Handler Assurance

Handler selection commits command kind, skill ID/version, explicit registration
mode, and `RegisteredUnattested` posture. It does not inspect an opaque Rust
trait object or claim handler implementation integrity.

This is suitable for the future `KernelObservedLocalProcess` assurance only:
Core may later prove that it observed a process under this exact pre-bound
selection and policy. Stronger provenance remains unsupported.

## 7. Validation And Privacy

Construction rejects skill/handler and command/handler mismatch. Deserialization
recomputes both handler-selection and complete binding fingerprints and fails
closed without echoing caller values.

Debug redacts bundle, workflow, run, step, skill, command, timestamp, and
fingerprint values. The model stores no raw output, environment values, source
contents, provider payloads, credentials, or free-form claims.

## 8. Test Coverage

Focused tests cover:

- valid construction and round trip;
- stable command and binding vectors;
- canonical ordering for unordered contract fields;
- distinct contract fingerprints;
- skill and command mismatch;
- binding and handler-selection tampering;
- run and creation-time binding sensitivity;
- payload-free serialization and redaction-safe Debug.

## 9. Governed Phase

- workflow: `dg/implement`
- run: `run-1784513422518947000-2`
- approval: `approval/run-1784513422518947000-2/implementation-approved`
- presentation: `presentation/d882b4fd055545a8`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits and validation ran
  outside the kernel

## 10. Validation

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 11. Remaining Limitations

- The binding is not yet created by an executor path.
- No kernel observation or accepted verifier exists.
- No durable binding store or create-only persistence exists.
- Registered handlers remain implementation-unattested.
- The dogfood phase-close presentation-record list-cap defect remains open.

## 12. Recommended Next Phase

Perform a phase-level review of this model. If accepted, implement the pure
crate-private verifier over this binding in a separate phase. Do not integrate
runtime execution or broaden provider writes first.
