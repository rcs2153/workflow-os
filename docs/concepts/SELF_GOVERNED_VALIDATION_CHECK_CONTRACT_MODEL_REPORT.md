# Self-Governed Validation/Check Contract Model Report

Report date: 2026-06-14

## 1. Executive Summary

Workflow OS now has a local validation/check command contract model for future self-governed validation/check dogfooding.

The model captures allowlisted command vocabulary, model-only execution posture, argument-vector boundaries, working-directory policy, environment policy, network policy, side-effect classification, output capture limits, redaction policy, result status vocabulary, and report citation hooks.

This phase does not execute commands. It does not add real local build/check skill handlers, CLI exposure, workflow schema fields, automatic execution, side-effect boundary implementation, writes, recursive agents, agent swarms, or production self-hosting.

## 2. Governance Run

This phase was governed by the self-governance dogfood workflow before implementation.

- State directory: `/tmp/workflow-os-self-governance-state.Mn9MF4`
- Run ID: `run-1781495333813334000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781495333813334000-2/d`
- Final status: `Completed`

The inspected run included `RunCreated`, `RunValidated`, `RunStarted`, `StepScheduled`, `PolicyDecisionRecorded`, `ApprovalRequested`, `ApprovalGranted`, `RunResumed`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `RunCompleted`.

## 3. Scope Completed

- Added `LocalCheckCommandContract`.
- Added `LocalCheckCommandId`.
- Added allowlisted `LocalCheckCommandKind` vocabulary.
- Added `LocalCheckExecutionPosture`.
- Added working-directory, environment, and network policy enums.
- Added side-effect classification.
- Added bounded output capture policy.
- Added redaction policy.
- Added local check result status vocabulary.
- Added validation that rejects premature execution posture, shell metacharacters, secret-like values, raw output persistence, unbounded output, duplicate citation kinds, and unclassified side effects.
- Added serde round-trip and invalid serialized payload tests.
- Updated docs to state the contract model exists while real execution remains unsupported.

## 4. Scope Explicitly Not Completed

- No local command execution.
- No real build/check skill handlers.
- No arbitrary shell execution.
- No CLI exposure.
- No workflow schema changes.
- No example updates.
- No automatic report generation.
- No automatic report artifact writing.
- No side-effect boundary implementation.
- No writes.
- No provider calls or live adapter execution.
- No recursive agents or agent swarms.
- No hosted or distributed runtime behavior.
- No production self-hosting claim.
- No release posture change.

## 5. Model Types Added

- `LocalCheckCommandContract`
- `LocalCheckCommandContractDefinition`
- `LocalCheckCommandId`
- `LocalCheckCommandKind`
- `LocalCheckExecutionPosture`
- `LocalCheckWorkingDirectoryPolicy`
- `LocalCheckEnvironmentPolicy`
- `LocalCheckNetworkPolicy`
- `LocalCheckSideEffectClass`
- `LocalCheckOutputCapturePolicy`
- `LocalCheckRedactionPolicy`
- `LocalCheckResultStatus`

The built-in helper `LocalCheckCommandContract::dogfood_validate_model_only()` represents the planned dogfood validation command contract without authorizing execution.

## 6. Validation Boundary Summary

The contract model validates:

- identifier shape;
- model-only execution posture;
- fixed executable token and argument vector;
- rejection of shell metacharacters;
- rejection of secret-like command, argument, environment, and path values;
- environment variable name shape;
- safe relative output directory declarations;
- non-zero bounded timeout;
- bounded stdout/stderr capture;
- no raw output persistence;
- classified side effects;
- duplicate citation-kind rejection.

Validation errors use stable codes and do not echo caller-supplied command text, arguments, environment values, paths, or secret-like payloads.

## 7. Redaction And Privacy Summary

The model treats command authority and output as sensitive by default.

Debug output redacts executable text, arguments, environment names, output directory values, and command identifiers. Serialization can represent valid contract values, but invalid serialized secret-like values fail closed during deserialization. The model does not store raw command output, command transcripts, provider payloads, spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values.

## 8. Test Coverage Summary

Added focused tests for:

- valid model-only local check contract;
- built-in dogfood validation contract;
- planned command kind vocabulary;
- result status vocabulary;
- premature handler execution posture rejection;
- shell metacharacter rejection;
- secret-like argument and environment rejection;
- raw output persistence rejection;
- output capture bounds;
- unclassified side-effect rejection;
- duplicate citation kind rejection;
- serde round trip;
- invalid serialized payload failure without leaking secret-like values;
- Debug non-leakage.

Existing workspace tests are expected to remain unchanged.

## 9. Commands Run And Results

- Self-governance dogfood run and approval:
  - Passed; final status `Completed`.
- `cargo test -p workflow-core --test local_check`
  - Passed.

Full validation commands for the phase:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Results are recorded in the final implementation report.

## 10. Remaining Known Limitations

- The model does not execute commands.
- The model does not register local skill handlers.
- The model does not expose CLI behavior.
- The model does not write report artifacts.
- The model does not define workflow schema fields.
- The model does not enforce a real sandbox.
- Side-effect boundary implementation remains future work.
- Command-output evidence attachment remains separately scoped future work.

## 11. Recommended Next Phase

Recommended next phase: **self-governed validation/check contract model review**.

The review should verify that the model remains narrow, redaction-safe, deterministic, and non-executing before any real allowlisted check handler is planned.
