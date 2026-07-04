# Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Review

## 1. Executive Verdict

Phase accepted; proceed to executor artifact-path integration planning.

The implementation adds the intended pure derivation helper and keeps the safety boundary intact. Workflow-declared enforcement postures are still rejected by normal semantic validation, so the repository does not yet imply that authored YAML is enforced by runtime artifact paths.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- explicit derivation input and result types;
- pure derivation from loaded `WorkflowDefinition` to `WorkReportArtifactHighAssuranceDisclosurePolicy`;
- focused tests for absent, no-op, and stronger high-assurance postures;
- documentation and implementation report updates.

No accidental implementation found for:

- semantic validation relaxation;
- executor artifact-path integration;
- automatic report generation;
- automatic artifact writing;
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

## 3. Helper API Assessment

The helper API is narrow and appropriate for the phase.

Reviewed API:

```rust
WorkflowReportArtifactGateDerivationInput<'a>
WorkflowReportArtifactGateDerivation
derive_workflow_report_artifact_gate_policy(...)
```

The helper accepts a borrowed `WorkflowDefinition`, returns an owned derivation result, and exposes the derived policy through a read-only accessor. It does not require a `LoadedSpec`, state backend, executor, report, artifact store, approval store, or event log. That keeps it suitable for later composition without prematurely binding it to a runtime integration point.

The `Result` return shape is acceptable even though the current enum is exhaustively mapped. It preserves a fail-closed API shape for future unsupported requirement vocabulary.

## 4. Derivation Mapping Assessment

The mapping is correct:

- absent `report_artifact_requirements` derives disabled policy;
- `not_required` derives disabled policy;
- `disclosure_required` derives `require_disclosure()`;
- `validated_disclosure_required` derives `require_validated()`;
- `validated_fail_closed_disclosure_required` derives `require_validated_fail_closed()`.

The helper reuses the existing canonical `WorkReportArtifactHighAssuranceRequirement::to_high_assurance_disclosure_policy()` mapping rather than duplicating policy logic.

## 5. Validation Boundary Assessment

The validation boundary remains safe.

Normal project validation still rejects stronger enforcement postures with:

```text
validation.workflow.report_artifact_requirement.runtime_not_enforced
```

This is the correct posture until an artifact-capable executor path explicitly derives and enforces the workflow-declared policy. The implementation does not make declarative YAML look enforceable in normal validation before enforcement exists.

## 6. Runtime And Mutation Assessment

The helper is pure and non-mutating.

Verified:

- no report generation;
- no artifact creation;
- no artifact persistence;
- no event append;
- no audit projection;
- no workflow/run/snapshot mutation;
- no approval inspection;
- no side-effect inspection;
- no filesystem, state backend, adapter, or external system dependency.

The test `workflow_artifact_gate_derivation_defaults_absent_requirement_to_disabled_policy` clones the workflow before derivation and asserts it remains unchanged.

## 7. Privacy And Redaction Assessment

The helper handles enum posture values only. It does not copy raw report text, workflow file contents, approval payloads, workflow event payloads, provider payloads, command output, paths, credentials, tokens, or secret-like values.

Debug output for the derivation result exposes only the policy structure. That is acceptable because the policy is bounded vocabulary and not caller-supplied sensitive text.

## 8. Test Quality Assessment

Test coverage is focused and appropriate for this phase.

Covered:

- absent requirement -> disabled policy;
- explicit `not_required` -> disabled policy;
- `disclosure_required` -> disclosure-required policy;
- `validated_disclosure_required` -> validated policy;
- `validated_fail_closed_disclosure_required` -> validated fail-closed policy;
- derivation uses parsed workflow definitions rather than hand-built enum-only fixtures;
- derivation does not mutate the input workflow;
- existing semantic validation still rejects enforcement posture until runtime wiring exists;
- full workspace regression suite passes.

Non-blocking gap:

- The review did not require an external public-API consumer test. Current integration tests import the public export from `workflow_core`, which is sufficient for this phase.

## 9. Documentation Review

Docs now say:

- pure runtime derivation helper is implemented;
- semantic validation still rejects enforcement postures;
- executor artifact-path derivation from workflow specs is not implemented;
- automatic report generation is not implemented;
- automatic artifact writing is not implemented;
- CLI behavior is not implemented;
- examples are not updated;
- side-effect execution and writes remain unsupported;
- hosted behavior, reasoning lineage, and release posture changes remain unsupported.

During review, one stale phrase in the runtime derivation plan was corrected so it no longer says runtime derivation does not exist. It now correctly says no artifact-capable executor path accepts and enforces the derived gate policy.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Plan the executor artifact-path integration that calls this helper only on explicit artifact-capable paths.
- Define the precedence rule for combining workflow-declared policy and caller-supplied policy, likely `stricter(workflow_declared_policy, explicit_caller_policy)`.
- Decide whether semantic validation should gain an artifact-capable validation context or stay globally rejecting until executor integration is reviewed.
- Consider whether derived policy should be included in artifact write results or metadata during the executor integration phase.

## 12. Recommended Next Phase

Recommended next phase: executor artifact-path integration planning.

The helper is safe, reviewed, and intentionally inert. The next useful step is planning how an explicit artifact-capable executor path should derive workflow-declared gate policy, compose it with caller-supplied policy, preserve workflow pass/fail semantics, and avoid automatic artifact writes.

## 13. Validation

Validation commands reviewed from the implementation phase:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test -p workflow-core --test work_report workflow_artifact_gate_derivation` passed.
- `cargo test -p workflow-core --test project_validation report_artifact_requirement_enforcement_posture_is_rejected_until_runtime_wiring_exists` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

Additional review validation:

- `npm run check:docs` passed after review documentation edits.
- `git diff --check` passed after review documentation edits.

## 14. Dogfood Governance

This review phase was governed by the local Workflow OS dogfood runner.

- workflow phase: review
- workflow ID: `dg/review`
- run ID: `run-1783138360743796000-2`
- approval ID: `approval/run-1783138360743796000-2/review-scope-approved`
- approval outcome: approved by the maintainer before review work continued
- close status: completed
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`

The dogfood runner coordinated governance only. Review writing, documentation correction, and validation commands were performed by the executor.
