# Composable Harness Contract Core Model Report

## 1. Executive Summary

The Composable Harness Contract core model is implemented as a model-only Workflow OS core addition.

The implementation defines validated, domain-neutral harness contract types for bounded governed execution envelopes. It does not implement nested harness execution, runtime scheduling, workflow schema fields, CLI behavior, examples, reasoning lineage, side-effect boundary modeling, writes, domain packs, hosted/distributed runtime behavior, or release posture changes.

## 2. Scope Completed

- Added `HarnessContract`.
- Added `HarnessContractId`.
- Added `HarnessContractVersion`.
- Added input, context, tool, output, evidence, approval, and handoff requirement types.
- Added authority scope vocabulary.
- Added side-effect allowance vocabulary without enabling writes.
- Added failure semantics vocabulary.
- Added conservative execution policy model.
- Added validation for required fields and duplicate declarations.
- Added bounded text and secret-like value checks.
- Added redaction metadata validation.
- Added serde support with fail-closed validation.
- Added redaction-safe `Debug` behavior.
- Added focused Rust tests.
- Updated roadmap and concept documentation.

## 3. Scope Explicitly Not Completed

Not implemented:

- nested harness execution;
- runtime scheduling of harnesses;
- workflow schema fields;
- CLI behavior;
- examples;
- reasoning lineage;
- side-effect boundary modeling;
- live write integrations;
- write-capable adapters;
- domain packs;
- hosted or distributed runtime behavior;
- production compliance systems;
- Level 3 or Level 4 autonomy claims;
- release posture changes.

## 4. Model Types Added

Core model types:

- `HarnessContract`
- `HarnessContractDefinition`
- `HarnessContractId`
- `HarnessContractVersion`
- `HarnessInputRequirement`
- `HarnessContextRequirement`
- `HarnessToolAllowance`
- `HarnessToolKind`
- `HarnessAuthorityScope`
- `HarnessSideEffectAllowance`
- `HarnessOutputRequirement`
- `HarnessEvidenceRequirement`
- `HarnessApprovalRequirement`
- `HarnessExecutionPolicy`
- `HarnessFailureSemantics`
- `HarnessHandoffRequirement`

The model reuses existing `SchemaVersion`, `RedactionMetadata`, `WorkReportSensitivity`, and `WorkReportRedactionPolicy` primitives.

## 5. Validation Boundary Summary

`HarnessContract::new(...)` is the validation boundary.

Validation requires:

- valid contract ID;
- valid contract version;
- non-secret schema version;
- bounded non-secret purpose;
- at least one input declaration;
- at least one context declaration;
- at least one authority scope;
- at least one output declaration;
- at least one evidence declaration;
- at least one approval declaration;
- at least one failure semantic;
- at least one handoff declaration;
- no duplicate requirement names within a requirement family;
- no duplicate authority scopes;
- no duplicate failure semantics;
- bounded and non-secret redaction metadata.

Deserialization routes through the same constructor and fails closed on invalid contracts.

## 6. Redaction And Privacy Summary

The model does not store raw provider payloads, raw command output, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, or unbounded natural-language handoffs.

`Debug` output redacts contract IDs, versions, purpose text, requirement names, and redaction metadata. Serialization may contain valid bounded contract fields, but tests assert that forbidden raw payload markers are not present.

Validation errors use stable codes and do not echo raw invalid values.

## 7. Test Coverage Summary

Added focused tests for:

- valid minimal harness contract;
- invalid harness ID;
- invalid version;
- missing purpose;
- empty input requirements;
- required context validation;
- authority duplicate rejection;
- side-effect allowance without write support;
- evidence requirement validation;
- approval requirement validation;
- failure duplicate rejection;
- handoff requirement validation;
- sensitivity and redaction policy serde;
- serde round trip;
- invalid serialized contract fail-closed behavior;
- debug non-leakage;
- serialization non-leakage;
- no runtime execution vocabulary;
- no domain-specific core requirements.

Focused command run:

- `cargo test -p workflow-core --test harness_contract` passed.

## 8. Commands Run And Results

- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-composable-harness-contract-core-model --mock-all-local-skills run dg/d` produced `WaitingForApproval`.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-composable-harness-contract-core-model --mock-all-local-skills approve run-1781543947807649000-2 approval/run-1781543947807649000-2/d --actor codex --reason composable-harness-contract-core-model-implementation` completed the governed run.
- `cargo fmt --all` passed with the repository toolchain.
- `cargo test -p workflow-core --test harness_contract` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.

## 9. Remaining Known Limitations

- No nested harness execution exists.
- No typed handoff model exists beyond contract requirement declarations.
- No workflow schema fields reference harness contracts.
- No runtime enforces harness contracts.
- No CLI renders or validates harness contracts directly.
- No side-effect boundary model exists.
- No reasoning lineage integration exists.
- No write behavior exists.

## 10. Recommended Next Phase

Recommended next phase: Composable Harness Contract core model review.

The review should verify the model remains domain-neutral, minimal, redaction-safe, and model-only, and should confirm whether typed handoff planning is the correct next step.
