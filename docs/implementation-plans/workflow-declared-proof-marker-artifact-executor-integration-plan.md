# Workflow-Declared Proof-Marker Artifact Executor Integration Plan

Status: planning only. Do not implement in this phase.

## 1. Executive Summary

Workflow OS now has:

- workflow schema vocabulary for `report_artifact_requirements.approval_proof_markers`;
- default semantic validation that accepts only `not_required`;
- a pure helper that derives effective approval proof-marker artifact gate policy from one selected workflow declaration and caller-supplied policy;
- an explicit local artifact-capable executor path that can write a governed local report artifact with caller-supplied approval proof-marker projection gates.

The next implementation question is how that explicit artifact-capable executor path should consume workflow-declared proof-marker artifact requirements. The correct next slice is narrow: derive the selected workflow's proof-marker artifact requirement inside the existing artifact-capable path and compose it with caller policy before artifact write.

This plan does not implement executor integration, artifact writes, projection persistence, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

## 2. Goals

- Connect workflow-declared approval proof-marker artifact requirements to the explicit artifact-capable executor path.
- Preserve the invariant that accepted governance declarations are enforced before artifact write.
- Keep default validation and default executor behavior conservative.
- Use the reviewed pure derivation helper.
- Compose caller-supplied proof-marker artifact policy and workflow-declared policy without allowing callers to weaken authored requirements.
- Preserve existing explicit caller-supplied proof-marker gate behavior when the workflow declares `not_required`.
- Fail before artifact write when the effective proof-marker gate is not satisfied.
- Preserve workflow pass/fail semantics when report/artifact generation fails after a run exists.
- Keep errors stable and non-leaking.
- Avoid automatic report generation, automatic artifact writing, projection persistence, CLI behavior, and schema changes.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- changing `execute_with_report_artifact_and_side_effect_gates(...)` behavior when no proof-marker gate is supplied;
- automatic report generation;
- automatic report artifact writing from default executor paths;
- automatic approval proof-marker projection persistence;
- automatic proof-marker citation generation beyond existing explicit report input policy;
- CLI rendering or artifact commands;
- examples;
- new workflow schema fields;
- TypeScript SDK changes;
- runtime config;
- public approval-card UX;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented foundation:

- approval-presentation proof records and proof markers for proof-enforced approvals;
- WorkReport approval proof-marker citation helpers;
- terminal report explicit proof-marker citation integration;
- durable local approval proof-marker audit projection store helper;
- in-memory and store-backed report artifact proof-marker gates;
- explicit executor artifact path proof-marker gate integration for caller-supplied policy;
- internal `WorkReportArtifactApprovalProofMarkerRequirement` model and mapping;
- workflow schema/parser/SDK vocabulary for `report_artifact_requirements.approval_proof_markers`;
- default semantic validation rejecting `projection_required` and `marker_required`;
- pure workflow-declared proof-marker artifact derivation helper;
- accepted maintainer review of the derivation helper.

Not implemented:

- executor artifact-path derivation from selected workflow proof-marker declaration;
- artifact-capable validation acceptance for workflow-declared proof-marker requirements in the explicit proof-marker artifact path;
- composition between workflow declaration and caller policy inside the executor artifact path;
- automatic approval proof-marker projection persistence.

## 5. Problem Statement

The existing explicit proof-marker artifact path accepts caller-supplied policy:

```rust
execute_with_report_artifact_and_proof_marker_gates(
    executor,
    artifact_store,
    side_effect_store,
    proof_marker_gate,
    request,
)
```

The workflow declaration remains rejected by default validation:

```yaml
report_artifact_requirements:
  approval_proof_markers: marker_required
```

That is safe, but incomplete. Once a caller selects the explicit artifact-capable proof-marker path, the runtime can enforce the declaration before artifact write. The integration must avoid two failures:

- false governance: accepting authored `projection_required` or `marker_required` without enforcing it;
- accidental automation: making artifact persistence or projection persistence automatic for default runs.

The correct path is explicit executor composition only.

## 6. Recommended First Integration Boundary

Add integration only to:

```rust
execute_with_report_artifact_and_proof_marker_gates(...)
```

Do not change:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_artifact_and_side_effect_gates(...)` when no proof-marker gate is supplied;
- CLI commands;
- examples;
- default validation behavior.

The integration should run only when the caller explicitly chooses the proof-marker artifact-capable API and supplies proof-marker gate inputs.

## 7. Validation Capability Posture

Default validation must remain conservative:

```text
workflow-os validate
LocalExecutor::execute(...)
LocalExecutor::execute_with_report(...)
```

These surfaces should continue rejecting `projection_required` and `marker_required`.

The explicit proof-marker artifact-capable executor path should validate with an explicit artifact-capable posture for the selected workflow only. This should mirror the existing high-assurance artifact-capable validation approach.

Rules:

- default validation keeps rejecting stronger proof-marker artifact requirements;
- explicit proof-marker artifact-capable validation may allow stronger proof-marker requirements only for the selected workflow;
- validation must still reject unsupported future proof-marker requirement vocabulary;
- validation errors must stay stable and non-leaking;
- CLI must not expose artifact-capable validation in this phase.

## 8. Executor Integration Shape

The first implementation should avoid broad executor rewrites.

Recommended implementation shape:

1. Reuse the existing artifact-capable execution preparation path that returns the selected `WorkflowDefinition`.
2. Call `derive_workflow_report_artifact_approval_proof_marker_gate_policy(...)` with:
   - selected workflow;
   - caller-supplied `proof_marker_gate.policy`;
   - `WorkflowReportArtifactProofMarkerDerivationMode::ArtifactCapable`.
3. Replace the caller policy passed to `write_work_report_artifact_with_governance_gates(...)` with the derived effective policy.
4. Preserve all existing report generation, side-effect integrity, approval linkage, high-assurance disclosure, provider-candidate validation, and artifact write behavior.
5. Return artifact gate/write failures inside `LocalExecutionWithReportArtifactResult` without altering workflow execution result.

The integration should not append workflow events for derivation. Artifact gate failure should remain an artifact error, not a workflow state transition.

## 9. Effective Policy Composition

The effective proof-marker gate policy must be:

```text
effective_policy = stricter(workflow_declared_policy, explicit_caller_policy)
```

Required ordering:

1. no proof-marker gate required;
2. projection required with marker-free approvals allowed;
3. projection required with present proof markers required.

Rules:

- workflow `not_required` plus disabled caller policy -> disabled;
- workflow `not_required` plus caller marker-free policy -> marker-free policy;
- workflow `not_required` plus caller marker-required policy -> marker-required policy;
- workflow `projection_required` plus disabled caller policy -> marker-free policy;
- workflow `projection_required` plus marker-free caller policy -> marker-free policy;
- workflow `projection_required` plus marker-required caller policy -> marker-required policy;
- workflow `marker_required` plus any weaker caller policy -> marker-required policy;
- workflow `marker_required` plus marker-required caller policy -> marker-required policy.

Callers may strengthen but must not weaken workflow-declared requirements.

## 10. Report And Artifact Failure Semantics

Workflow execution semantics must remain unchanged.

Rules:

- execution failure before a run exists still returns `Err`;
- report generation failure after a run exists returns the run plus report error;
- proof-marker gate failure after report generation returns the run, report, and artifact error;
- artifact gate failure must not change workflow run status;
- artifact gate failure must not append workflow events;
- artifact gate failure must not mutate `WorkflowRun`, `WorkflowRunSnapshot`, approval decisions, side-effect records, projection records, generated reports, or artifact records;
- no partial artifact may be written when gates fail;
- generated reports remain available when artifact persistence fails.

## 11. Projection Store Boundary

The integration must not persist proof-marker projection records.

It may read from the explicit projection store already supplied in:

```rust
LocalExecutionReportArtifactProofMarkerGateInputs
```

Rules:

- do not discover hidden stores;
- do not write projection records;
- do not synthesize projection records from report text;
- do not infer proof markers from approval reason text, presentation IDs, content hashes, or handoff prose;
- accept only durable bounded projection records through the existing store-backed gate helper.

## 12. Privacy And Redaction

The integration must not inspect or copy:

- approval presentation payloads;
- approval reason text;
- report section text beyond already-validated report model fields;
- workflow event payloads beyond stable references already modeled;
- projection record payloads beyond bounded fields used by existing gate helpers;
- provider payloads;
- raw command output;
- raw spec contents;
- file contents;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Errors must use stable codes and must not echo workflow YAML, local paths, report text, approval metadata, projection payloads, policy payloads, provider payloads, or secret-like values.

## 13. Test Plan

Future implementation tests should cover:

- default validation still rejects `projection_required` and `marker_required`;
- `execute(...)` still rejects workflows with enforceable proof-marker artifact requirements;
- `execute_with_report(...)` still rejects workflows with enforceable proof-marker artifact requirements;
- `execute_with_report_artifact_and_side_effect_gates(...)` still rejects enforceable proof-marker artifact requirements when no proof-marker gate path is selected;
- `execute_with_report_artifact_and_proof_marker_gates(...)` accepts enforceable proof-marker declarations only for the selected workflow;
- workflow `not_required` preserves existing caller-supplied proof-marker gate behavior;
- workflow `projection_required` strengthens disabled caller policy to marker-free projection coverage;
- workflow `marker_required` strengthens disabled or marker-free caller policy to present-marker coverage;
- caller marker-required policy remains stricter than workflow `projection_required`;
- missing required projection fails artifact write without changing run/report;
- marker-free projection passes only when effective policy allows marker-free approvals;
- present-marker projection passes when effective policy requires markers;
- unsupported future proof-marker requirement vocabulary still fails closed;
- no workflow events are appended for derivation or artifact gate failure;
- no projection records are persisted by the executor integration;
- no artifact is written when the effective proof-marker gate fails;
- artifact is written when proof-marker gate, side-effect integrity, approval linkage, high-assurance disclosure, and provider-candidate gates all pass;
- errors are stable and non-leaking;
- existing executor/report/artifact tests still pass;
- docs checks pass.

Required validation for future implementation:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Run `npm run check:contracts` only if the implementation unexpectedly changes schema or SDK contract surfaces. The intended implementation should not require contract changes.

## 14. Documentation Updates For Future Implementation

Future implementation should update:

- `ROADMAP.md`;
- `docs/implementation-plans/workflow-declared-proof-marker-artifact-runtime-derivation-plan.md`;
- `docs/implementation-plans/report-artifact-approval-proof-marker-gate-plan.md`, if gate status wording needs clarification;
- an implementation report in `docs/concepts/`.

## 15. Deferred Work

Deferred:

- implementation in this planning phase;
- automatic proof-marker projection persistence;
- automatic report artifact writing;
- automatic report generation;
- default executor proof-marker enforcement;
- CLI rendering or artifact commands;
- examples;
- public approval cards;
- provider writes;
- side-effect execution;
- hosted or distributed runtime;
- reasoning lineage;
- release posture changes.

## 16. Final Recommendation

Proceed next with explicit executor artifact-path proof-marker derivation integration.

The implementation should wire the reviewed pure derivation helper into `execute_with_report_artifact_and_proof_marker_gates(...)`, compose the selected workflow declaration with caller policy by strictness, and enforce the effective policy before artifact write. It must not broaden into default executor behavior, automatic artifact writing, projection persistence, CLI behavior, schemas, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

## 17. Governed Planning Record

- Dogfood workflow: `dg/d`.
- Run ID: `run-1783678933171520000-2`.
- Approval ID: `approval/run-1783678933171520000-2/planning-approved`.
- Approval presentation ID: `presentation/4ad1e3134c514406`.
- Approval presentation hash: `4ad1e3134c514406c8620c9af97824a7068384d319af05f87d0bfba7aff3998d`.
- Approval outcome: granted by delegated maintainer for planning-only scope.

## 18. Validation

- `npm run dogfood:benchmark -- phase-start --phase planning --work-summary "plan executor artifact path proof-marker derivation integration" --approved-scope "create integration planning document only" --strict-non-goals "no implementation, no artifact writes, no projection persistence, no CLI behavior" --expected-touched-surfaces "docs/implementation-plans and roadmap docs" --validation-required "npm run check:docs" --why-now "helper review accepted pure derivation and recommended explicit executor artifact path integration planning"` - passed.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /var/folders/r9/y7_mqmq108z94yhyt702h2b80000gn/T/workflow-os-self-governance-state --mock-all-local-skills dogfood approval-presentation approve --run-id run-1783678933171520000-2 --approval-id approval/run-1783678933171520000-2/planning-approved --presentation-id presentation/4ad1e3134c514406 --actor user/delegated-maintainer --reason approved-proof-marker-artifact-executor-integration-planning` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783678933171520000-2 --phase planning` - passed.
