# Workflow-Declared High-Assurance Artifact Requirement Executor Integration Blocker Fix Review

## 1. Executive Verdict

Blockers fixed; proceed to next runtime composition phase.

The blocker fix addresses both review blockers:

- artifact-capable validation is now scoped to the selected workflow instead of relaxing the whole project;
- rehydrated artifact-path runs now fail closed when the currently loaded workflow no longer matches the immutable run identity/spec hash.

No additional blocker was found.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

Implemented:

- selected-workflow scoping for `ProjectValidationCapability::ReportArtifactCapable`;
- identity matching before deriving artifact policy for existing runs;
- stable non-leaking mismatch error;
- focused regression tests;
- blocker-fix report.

No accidental implementation found for:

- automatic report generation;
- automatic artifact writing from default executor paths;
- CLI artifact behavior;
- example updates;
- new workflow schema fields;
- TypeScript SDK changes;
- runtime config;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, quorum approval, or revocation enforcement;
- approval evidence attachment;
- workflow event or audit projection for artifact derivation;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage;
- release posture changes.

## 3. Original Blockers Restated

Original blocker 1: artifact-capable validation was project-wide.

The artifact-capable executor path executes one selected workflow, but validation previously treated the entire project as artifact-capable. This could allow unrelated workflows in the same project to declare high-assurance artifact requirements without validation errors, even though those workflows were not being executed or enforced by the artifact path.

Original blocker 2: rehydrated artifact-path policy derivation used mutable current project files.

When a run already existed, the artifact path rehydrated the run and then derived artifact gate policy from the current workflow YAML. It did not prove the loaded workflow still matched the run's immutable workflow identity, version, schema version, and spec content hash.

## 4. Fix Approach Assessment

The selected approach is minimal and idiomatic.

`ProjectValidationCapability::ReportArtifactCapable` now carries the selected `WorkflowId`. `validate_report_artifact_requirements(...)` only relaxes workflow-declared report artifact requirements when the workflow being validated matches that selected ID. All other workflows keep the default conservative validation posture.

For rehydrated artifact paths, the executor now:

- rehydrates the existing run;
- loads and validates the project with the selected workflow capability;
- finds the selected workflow;
- compares the loaded workflow ID, schema version, workflow version, and content hash against `WorkflowRunIdentity`;
- fails closed with `executor.report_artifact.workflow_identity_mismatch` if any identity field differs;
- derives artifact policy only after the identity match is proven.

This avoids introducing persistence or new event projection while still closing the false-governance gap.

## 5. Validation Boundary Assessment

Validation scoping is now correct for this phase.

Verified:

- default validation remains unchanged;
- default executor paths still reject workflow-declared enforcement posture;
- artifact-capable validation relaxes only the selected workflow;
- unselected workflows with enforcement posture still trigger `validation.workflow.report_artifact_requirement.runtime_not_enforced`;
- rejected validation writes no events for the requested run.

The capability is now an execution-context capability rather than a broad project validation mode.

## 6. Rehydration And Runtime Semantics Assessment

The rehydrated-run blocker is fixed.

Verified:

- existing runs are rehydrated before artifact policy derivation;
- loaded workflow identity is checked against immutable run identity;
- changed current workflow YAML fails closed before policy derivation or artifact write;
- the mismatch path appends no new workflow events;
- the mismatch error is stable and does not leak YAML, paths, hashes, report text, or secret-like values.

The implementation still derives policy from currently loaded YAML after identity matching. That is acceptable for this blocker fix because the content hash match proves the loaded YAML is the same workflow definition for this run. Persisting the derived policy at run creation remains a possible future hardening step.

## 7. Policy Composition Assessment

The fix does not change the previously reviewed strictness composition rule:

```text
effective_policy = stricter(caller_policy, workflow_declared_policy)
```

Callers can still request stricter artifact gates, but cannot weaken workflow-declared gates. The fix makes the workflow-declared policy being composed correctly scoped and identity-checked.

## 8. Privacy And Redaction Assessment

No privacy or redaction regression was found.

The fix compares typed identity fields and a bounded spec content hash. It does not copy raw workflow YAML, report text, provider payloads, command output, parser payloads, file contents, environment values, credentials, authorization headers, private keys, or token-like values.

The new error code is stable and the message is non-leaking:

```text
executor.report_artifact.workflow_identity_mismatch
```

## 9. Test Quality Assessment

The added tests cover the blocker cases directly:

- `artifact_capable_validation_does_not_relax_unselected_workflows`;
- `artifact_path_rehydrated_run_fails_closed_when_workflow_spec_changes`.

The tests verify:

- multi-workflow project behavior;
- selected workflow scoping;
- unselected workflow rejection;
- no event append for rejected validation;
- rehydrated run identity mismatch behavior;
- no event append on rehydrated mismatch;
- stable mismatch error code.

Existing tests continue to cover:

- default executor rejection;
- default report-bearing executor rejection;
- artifact-path derivation;
- workflow/caller strictness composition;
- artifact gate failure preservation;
- successful artifact write when disclosure satisfies the workflow-declared gate.

Non-blocking gap: there is not a separate test for schema-version or workflow-version mismatch. The content-hash mutation test proves the critical mutable-YAML path, and the helper checks all identity fields. Additional identity-field-specific tests can be added if this helper becomes reusable.

## 10. Documentation Review

Docs now state:

- artifact-capable validation is selected-workflow scoped;
- rehydrated artifact paths fail closed on workflow identity/spec mismatch;
- default validation still rejects workflow-declared artifact enforcement postures;
- the artifact-capable validation posture remains internal, not a CLI mode;
- automatic report generation is not implemented;
- automatic artifact writing from default paths is not implemented;
- CLI artifact behavior is not implemented;
- examples are not updated;
- schemas are not changed;
- side-effect execution and writes remain unsupported;
- hosted/distributed runtime, reasoning lineage, and release posture changes remain unsupported.

## 11. Blockers

No remaining blockers found.

## 12. Non-Blocking Follow-Ups

- Consider persisting or event-projecting derived artifact policy at run creation in a later phase.
- Consider extracting the workflow identity comparison helper if side-effect, hook, or artifact paths need the same check in multiple modules.
- Consider additional narrow tests for schema-version and workflow-version mismatch if the identity helper is made public or reused.
- Consider compatibility notes for `ProjectValidationCapability` because the exported enum changed from a unit variant to a workflow-scoped variant during a preview-stage API.

## 13. Recommended Next Phase

Recommended next phase: next runtime composition phase.

The enforcement-boundary blockers are fixed and reviewed. The project can proceed to the next runtime composition item while keeping write-capable adapters, side-effect execution, CLI artifact behavior, schemas, examples, hosted/distributed runtime, and reasoning lineage out of scope until separately planned and reviewed.

## 14. Validation

Validation reviewed from the blocker-fix phase:

- `cargo fmt --all` passed.
- `cargo fmt --all --check` passed.
- `cargo test -p workflow-core --test local_executor artifact_capable_validation_does_not_relax_unselected_workflows` passed.
- `cargo test -p workflow-core --test local_executor artifact_path_rehydrated_run_fails_closed_when_workflow_spec_changes` passed.
- `cargo test -p workflow-core --test local_executor` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

Additional review-phase validation:

- `npm run check:docs` passed.
- `git diff --check` passed.

## 15. Dogfood Governance

This blocker-fix review phase was governed by the local Workflow OS dogfood runner.

- workflow phase: review
- workflow ID: `dg/review`
- run ID: `run-1783144122587167000-2`
- approval ID: `approval/run-1783144122587167000-2/review-scope-approved`
- approval outcome: approved by the maintainer before review work continued
- close status: completed
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`

The dogfood runner coordinated governance only. Review writing, code inspection, and validation commands were performed by the executor.
