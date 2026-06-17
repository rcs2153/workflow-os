# Agent Harness Hook Contract Model Report

Report date: 2026-06-16

## 1. Executive Summary

Implemented the first model-only Agent Harness Hook Contract slice. Workflow OS now has validated Rust vocabulary for deterministic named agent harness checkpoints, including hook identity, version, kind, required inputs, required outputs, failure semantics, sensitivity, redaction policy, and side-effect posture.

This phase does not implement runtime hook invocation. The current agent scaffold remains the `dbt_project.yml` equivalent for orientation, while hook contracts are the future contract layer for deterministic checkpoints.

## 2. Scope Completed

- Added `AgentHarnessHookContract`.
- Added `AgentHarnessHookContractId`.
- Added `AgentHarnessHookContractVersion`.
- Added `AgentHarnessHookKind`.
- Added `AgentHarnessHookInputRequirement`.
- Added `AgentHarnessHookOutputRequirement`.
- Added `AgentHarnessHookFailureSemantics`.
- Added `AgentHarnessHookSideEffectAllowance`.
- Exported the model through `workflow-core`.
- Added focused model tests for validation, serde, redaction, side-effect rejection, and non-runtime boundaries.
- Updated roadmap, concept, quickstart, and planning documentation to reflect model-only implementation status.

## 3. Scope Explicitly Not Completed

- No runtime hook execution.
- No automatic workflow execution.
- No automatic local check execution.
- No default handler registration.
- No command-output evidence.
- No CLI hook command.
- No workflow schema fields.
- No runtime harness generation.
- No nested harness execution.
- No recursive agents.
- No agent swarms.
- No hosted or distributed agent execution.
- No side-effect modeling.
- No writes.
- No approval evidence attachment.
- No reasoning lineage.
- No persistence changes.
- No report artifact auto-writing.
- No examples.
- No release posture change.

## 4. Model Types Added

The model defines a bounded contract for future deterministic agent harness checkpoints:

- `AgentHarnessHookContract` stores validated hook contract fields.
- `AgentHarnessHookContractDefinition` is the construction input boundary.
- `AgentHarnessHookContractId` and `AgentHarnessHookContractVersion` are bounded, secret-aware identifiers.
- `AgentHarnessHookKind` represents checkpoint vocabulary such as planning, implementation, validation, review, and report boundaries.
- `AgentHarnessHookInputRequirement` and `AgentHarnessHookOutputRequirement` model typed input and output obligations by stable names.
- `AgentHarnessHookFailureSemantics` models future failure vocabulary without invoking hooks.
- `AgentHarnessHookSideEffectAllowance` represents side-effect posture while rejecting side-effect authorization in this phase.

## 5. Validation Boundary Summary

Validation ensures:

- hook IDs and versions are non-empty, bounded, and use supported characters;
- schema version text does not look secret-like;
- purpose text is non-empty, bounded, and redaction-aware;
- input and output requirement lists are non-empty;
- duplicate input and output requirement names are rejected;
- failure semantics are non-empty and duplicate-free;
- side-effect authorization is rejected;
- redaction metadata field names and reasons are bounded and secret-aware;
- serialized hook contracts deserialize through the same validation boundary.

Validation errors use stable `agent_harness_hook.*` codes and do not include raw caller-supplied values.

## 6. Redaction And Privacy Summary

The model follows existing Workflow OS contract-model privacy patterns:

- caller-supplied hook IDs are redacted in `Debug`;
- purpose text, input names, output names, and redaction metadata are not emitted through `Debug`;
- raw provider payload markers, raw command output markers, raw spec contents, parser payload markers, environment values, authorization text, private-key text, and token-like strings are rejected at the hook contract boundary;
- serialization only carries validated bounded fields;
- invalid serialized hook contracts fail closed through the validated constructor.

## 7. Test Coverage Summary

Added focused tests covering:

- valid minimal hook contract construction;
- invalid hook ID and version rejection;
- hook kind vocabulary;
- required input and output validation;
- duplicate input and output rejection;
- failure semantic duplicate rejection;
- side-effect authorization rejection;
- no-side-effect posture representation;
- sensitivity and redaction policy serde;
- serde round trip;
- invalid serialized contract fail-closed behavior;
- secret-like deserialization error non-leakage;
- redaction metadata validation;
- raw payload marker rejection;
- redaction-safe `Debug`;
- serialization non-leakage;
- no runtime hook behavior encoded in the model.

## 8. Commands Run And Results

- `cargo fmt --all`
  - Passed.
- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test -p workflow-core --test agent_harness_hook_contract`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 9. Remaining Known Limitations

- Hook contracts are not reviewed yet.
- Hook runtime invocation is not implemented.
- Hook CLI commands are not implemented.
- Workflow schema support for hook declarations is not implemented.
- Hook audit events are not implemented.
- Hook linkage to local checks, EvidenceReference, WorkReport, approvals, and typed handoffs remains future runtime planning.
- Side effects and writes remain unsupported.

## 10. Recommended Next Phase

Recommended next phase: **agent harness hook contract model review**.

The review should verify scope, validation behavior, serde behavior, redaction/privacy posture, test quality, and documentation honesty before any runtime hook invocation planning begins.
