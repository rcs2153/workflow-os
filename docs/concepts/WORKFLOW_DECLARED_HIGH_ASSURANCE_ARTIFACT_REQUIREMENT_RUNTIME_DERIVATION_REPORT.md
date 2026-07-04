# Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Report

## 1. Executive Summary

This phase implements the first runtime bridge for workflow-declared high-assurance report artifact requirements.

Workflow definitions can now be mapped, in memory and without side effects, from `report_artifact_requirements.high_assurance_approval` to the existing `WorkReportArtifactHighAssuranceDisclosurePolicy`. This keeps the authored declaration connected to the reviewed artifact gate vocabulary while preserving the current fail-closed validation posture for enforcement values.

## 2. Scope Completed

- Added a pure workflow artifact gate derivation helper in `workflow-core`.
- Added explicit derivation input and result types.
- Mapped absent and `not_required` workflow declarations to disabled artifact gate policy.
- Mapped `disclosure_required` to disclosure-required artifact gate policy.
- Mapped `validated_disclosure_required` to validated-disclosure artifact gate policy.
- Mapped `validated_fail_closed_disclosure_required` to validated fail-closed artifact gate policy.
- Exported the helper and types from `workflow-core`.
- Added focused tests using parsed workflow definitions.
- Updated adjacent roadmap, workflow spec, report artifact, and high-assurance artifact requirement documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- semantic validation relaxation for enforcement postures;
- executor artifact-path integration;
- automatic report generation;
- automatic report artifact writing;
- default executor behavior changes;
- runtime config;
- CLI artifact behavior;
- example updates;
- new workflow schema fields;
- TypeScript SDK changes;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, quorum approval, or revocation enforcement;
- approval evidence attachment;
- workflow event or audit projection for derivation;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The implementation adds:

```rust
WorkflowReportArtifactGateDerivationInput<'a>
WorkflowReportArtifactGateDerivation
derive_workflow_report_artifact_gate_policy(...)
```

The helper accepts a borrowed `WorkflowDefinition` and returns the derived `WorkReportArtifactHighAssuranceDisclosurePolicy`.

The helper does not read files, inspect runtime state, inspect approvals, generate a `WorkReport`, write a `WorkReportArtifactRecord`, append events, or mutate the workflow definition.

## 5. Derivation Behavior

Derivation is deterministic:

- absent `report_artifact_requirements` -> `WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()`
- `not_required` -> `WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()`
- `disclosure_required` -> `WorkReportArtifactHighAssuranceDisclosurePolicy::require_disclosure()`
- `validated_disclosure_required` -> `WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated()`
- `validated_fail_closed_disclosure_required` -> `WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated_fail_closed()`

## 6. Validation Boundary

Semantic validation remains intentionally conservative.

Only `not_required` is accepted by normal project validation. Stronger enforcement postures remain rejected with:

```text
validation.workflow.report_artifact_requirement.runtime_not_enforced
```

That rejection remains correct until an artifact-capable executor path explicitly derives and enforces workflow-declared artifact gate policy.

## 7. Redaction And Privacy Summary

The helper handles enum posture values only. It does not store, copy, log, serialize, or inspect raw provider payloads, report text, approval payloads, workflow event payloads, file contents, command output, credentials, tokens, or secret-like values.

Errors remain stable and non-leaking. Unsupported future vocabulary is still rejected during parsing or validation before derivation in the current model.

## 8. Test Coverage Summary

Added focused tests covering:

- absent workflow artifact requirement deriving disabled gate policy;
- explicit `not_required` deriving disabled gate policy;
- each enforcement posture deriving the expected explicit gate policy;
- derivation preserving the input workflow definition.

Existing validation tests continue to cover the semantic boundary that enforcement postures are rejected until runtime artifact paths can enforce them.

## 9. Commands Run And Results

Validation commands run for this implementation phase:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test -p workflow-core --test work_report workflow_artifact_gate_derivation` passed.
- `cargo test -p workflow-core --test project_validation report_artifact_requirement_enforcement_posture_is_rejected_until_runtime_wiring_exists` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

## 10. Dogfood Governance

This implementation phase was governed by the local Workflow OS dogfood runner.

- workflow phase: implementation
- workflow ID: `dg/implement`
- run ID: `run-1783137263056961000-2`
- approval ID: `approval/run-1783137263056961000-2/implementation-approved`
- approval outcome: approved by the maintainer before implementation work continued
- close status: completed
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`

The dogfood runner coordinated governance only. Code edits, documentation edits, and validation commands were performed by the executor.

## 11. Remaining Known Limitations

- Enforcement postures remain semantically rejected in normal project validation.
- No executor artifact path derives workflow declarations into artifact gate policy.
- No precedence rule is implemented for combining workflow-declared policy with caller-supplied policy.
- No report artifact is generated or written automatically.
- No artifact derivation events or audit records are emitted.

## 12. Recommended Next Phase

Recommended next phase: **workflow-declared artifact gate derivation helper review**.

The review should verify that the helper is pure, deterministic, correctly mapped, non-mutating, and does not imply runtime enforcement before executor artifact integration exists.
