# Workflow-Declared High-Assurance Artifact Requirement Executor Integration Plan

Status: Implemented. The explicit local report artifact executor path now derives workflow-declared high-assurance artifact requirements into effective artifact gate policy and composes them with caller-supplied artifact policy by strictness. Default validation and default executor paths remain conservative. This phase did not implement automatic report generation, automatic artifact writing, CLI behavior, examples, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has:

- workflow schema vocabulary for `report_artifact_requirements.high_assurance_approval`;
- default semantic validation that accepts only `not_required`;
- a pure derivation helper from `WorkflowDefinition` to `WorkReportArtifactHighAssuranceDisclosurePolicy`;
- an explicit local executor path that can generate a report, derive workflow-declared artifact policy, and write a governed report artifact when artifact gates pass.

The runtime composition gap is now closed for the explicit artifact-capable executor path only. Normal project validation still rejects enforcement postures, which is correct for default execution and `execute_with_report(...)`. The artifact-capable path validates with an explicit report-artifact capability, derives the workflow declaration, and enforces the effective policy before artifact write.

The integration remains narrow and opt-in. It preserves default validation and execution behavior everywhere else.

## 2. Goals

- Connect workflow-declared report artifact requirements to the explicit artifact-capable executor path.
- Preserve the invariant that governance declarations are either enforced or rejected.
- Keep `LocalExecutor::execute(...)` and `LocalExecutor::execute_with_report(...)` unchanged.
- Avoid automatic report generation and automatic artifact writing.
- Use the existing derivation helper and artifact gate policy.
- Compose caller-supplied artifact policy and workflow-declared policy without allowing callers to weaken authored requirements.
- Preserve workflow pass/fail semantics when report/artifact generation fails after a run exists.
- Keep errors stable and non-leaking.
- Keep default `workflow-os validate` conservative until an explicit artifact-capable validation surface is designed.

## 3. Non-Goals

This plan and implementation do not authorize:

- automatic report generation;
- automatic artifact writing;
- changing `LocalExecutor::execute(...)`;
- changing default `LocalExecutor::execute_with_report(...)`;
- making artifact persistence automatic for all runs;
- CLI artifact behavior;
- example updates;
- new workflow schema fields beyond the already implemented `report_artifact_requirements.high_assurance_approval` field;
- TypeScript SDK changes;
- runtime config for report artifacts;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, SSO, SCIM, teams, groups, or directory integration;
- quorum or multi-party approval enforcement;
- role-bound authority enforcement;
- revocation enforcement;
- approval evidence attachment;
- workflow event or audit projection for derivation;
- side-effect execution;
- provider mutation or write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented:

- `WorkReportArtifactHighAssuranceRequirement` posture vocabulary.
- `WorkReportArtifactHighAssuranceDisclosurePolicy` artifact gate policy.
- `WorkReportArtifactRequirement` internal model and mapping.
- Workflow schema/parser/SDK field for `report_artifact_requirements.high_assurance_approval`.
- Default semantic validation that rejects enforcement postures with `validation.workflow.report_artifact_requirement.runtime_not_enforced`.
- Pure derivation helper: `derive_workflow_report_artifact_gate_policy(...)`.
- Explicit artifact-capable executor path: `execute_with_report_artifact_and_side_effect_gates(...)`.

Implemented in the executor integration slice:

- artifact-capable validation context for explicit artifact-capable execution;
- executor path derivation from workflow declaration;
- effective policy composition between workflow declaration and caller-supplied policy;
- artifact-capable validation acceptance for enforcement posture only in the explicit artifact path.

Not implemented:

- automatic artifact writing.

## 5. Problem Statement

The current artifact-capable executor path accepts caller-supplied artifact policy:

```rust
LocalExecutionReportArtifactInputs {
    high_assurance_disclosure_policy: ...
}
```

The workflow declaration remains inert:

```yaml
report_artifact_requirements:
  high_assurance_approval: validated_fail_closed_disclosure_required
```

That is safe because semantic validation rejects the stronger posture today, but it is not yet useful. The next bridge must avoid two opposite failures:

- **false governance**: accepting authored enforcement posture but never enforcing it;
- **accidental automation**: treating any declaration as permission to generate or persist artifacts automatically.

The correct path is explicit executor composition, not default automation.

## 6. Recommended First Implementation Boundary

Add integration only to:

```rust
execute_with_report_artifact_and_side_effect_gates(...)
```

Do not change:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_and_side_effect_discovery(...)`;
- CLI commands;
- examples;
- default validation behavior.

The integration should run only when a caller explicitly chooses the artifact-capable API and supplies artifact inputs/stores.

## 7. Validation Capability Posture

Default validation should remain conservative:

```text
workflow-os validate
LocalExecutor::execute(...)
LocalExecutor::execute_with_report(...)
```

These should continue rejecting enforcement postures until they can prove enforcement.

The artifact-capable executor path needs an explicit validation posture, likely:

```rust
ProjectValidationCapability::Default
ProjectValidationCapability::ReportArtifactCapable
```

or a similarly small internal enum.

Rules:

- default validation keeps rejecting stronger artifact requirements;
- report-artifact-capable validation may allow stronger requirements only for the workflow being executed through the artifact-capable path;
- validation must still reject unsupported future high-assurance requirement vocabulary;
- validation errors must stay stable and non-leaking;
- the CLI should not expose artifact-capable validation in this phase.

This prevents normal validation from overclaiming enforcement while allowing the explicit artifact path to enforce what it accepts.

## 8. Executor Integration Boundary

The first implementation should avoid broad executor rewrites.

Recommended implementation shape:

1. Add an internal preparation path for artifact-capable execution that loads the project, validates with report-artifact capability, and returns the selected workflow definition alongside the execution plan.
2. Derive workflow-declared artifact gate policy from the selected workflow definition.
3. Compose derived policy with caller-supplied artifact policy.
4. Run existing report generation and artifact write logic.
5. Return report/artifact errors inside `LocalExecutionWithReportArtifactResult` without altering workflow execution result.

The implementation should not append workflow events for derivation. Artifact write success/failure should remain represented by the artifact result, not workflow state mutation.

## 9. Effective Policy Composition

The effective artifact gate policy should be:

```text
effective_policy = stricter(workflow_declared_policy, explicit_caller_policy)
```

Required ordering:

1. `disabled`
2. `require_disclosure`
3. `require_validated`
4. `require_validated_fail_closed`

Rules:

- callers may request a stricter policy than the workflow declares;
- callers must not weaken a workflow-declared policy;
- a disabled caller policy must not erase a workflow declaration;
- a disabled workflow declaration must preserve existing explicit caller behavior;
- composition must be deterministic and tested.

The first implementation may add a small policy composition helper if needed.

## 10. Report And Artifact Failure Semantics

Workflow execution semantics must remain unchanged.

Rules:

- execution failure before a run exists still returns `Err`;
- report generation failure after a run exists returns the run plus report error;
- artifact gate or artifact write failure after report generation returns the run, report, and artifact error;
- artifact gate failure must not change workflow run status;
- artifact gate failure must not append workflow events;
- artifact gate failure must not mutate `WorkflowRun`, `WorkflowRunSnapshot`, approval decisions, side-effect records, or generated reports;
- no partial artifact may be written when gates fail;
- generated reports remain available when artifact persistence fails.

## 11. Privacy And Redaction

The integration must not inspect or copy:

- raw provider payloads;
- raw report section text beyond already-validated `WorkReport` fields;
- approval payloads;
- workflow event payloads beyond stable IDs/counts already modeled;
- raw command output;
- raw spec contents;
- file contents;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Errors must use stable codes and must not echo workflow YAML, paths, report text, approval metadata, policy payloads, provider payloads, or secret-like values.

## 12. Test Plan

Future implementation tests should cover:

- default validation still rejects enforcement posture;
- artifact-capable validation allows enforcement posture only for explicit artifact path;
- `execute(...)` still rejects workflows with enforcement posture;
- `execute_with_report(...)` still rejects workflows with enforcement posture;
- `execute_with_report_artifact_and_side_effect_gates(...)` derives workflow policy;
- absent and `not_required` declarations preserve caller-supplied policy;
- caller disabled plus workflow `disclosure_required` enforces disclosure;
- caller disabled plus workflow `validated_disclosure_required` enforces validation;
- caller disabled plus workflow `validated_fail_closed_disclosure_required` enforces fail-closed disclosure;
- caller stricter than workflow wins;
- caller weaker than workflow cannot weaken the gate;
- missing required high-assurance disclosure fails artifact write without changing run/report;
- successful disclosure writes the artifact when all other gates pass;
- report/artifact errors are stable and non-leaking;
- no workflow events are appended for derivation or artifact gate failure;
- no state backend mutation occurs beyond existing execution and explicit artifact store write on success;
- no CLI behavior changes;
- existing executor/report/artifact tests still pass;
- docs checks pass.

## 13. Documentation Updates For Future Implementation

Future implementation should update:

- `ROADMAP.md`;
- `docs/specs/workflows.md`;
- `docs/implementation-plans/report-artifact-plan.md`;
- `docs/implementation-plans/report-artifact-high-assurance-disclosure-gate-plan.md`;
- `docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-plan.md`;
- `docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-runtime-derivation-plan.md`.

Docs must say:

- explicit artifact-capable executor path derives workflow-declared artifact gate policy;
- default validation and default execution paths remain conservative;
- automatic report generation is not implemented;
- automatic artifact writing is not implemented;
- CLI artifact behavior is not implemented;
- examples are not updated;
- side-effect execution and writes remain unsupported.

## 14. Proposed Implementation Sequence

Recommended small implementation sequence:

1. Add strictness/composition helper for `WorkReportArtifactHighAssuranceDisclosurePolicy`.
2. Add explicit artifact-capable validation context, internal only.
3. Refactor executor preparation minimally so the artifact-capable path can validate and retain the selected workflow definition for derivation.
4. Derive workflow policy and compose it with caller policy in `execute_with_report_artifact_and_side_effect_gates(...)`.
5. Add focused executor tests for policy composition, failure semantics, and unchanged default behavior.
6. Update docs and create an end-of-phase report.
7. Review before any CLI, examples, or automatic artifact behavior.

## 15. Open Questions

- Should artifact-capable validation be public API or internal-only in the first implementation?
- Should the effective policy be returned in `LocalExecutionWithReportArtifactResult` even when artifact write fails?
- Should the effective policy be persisted in artifact metadata later?
- Should explicit caller policy have a separate `deny_workflow_declared_policy` escape hatch for tests, or is that too dangerous?
- Should default `workflow-os validate` eventually accept enforcement postures when it can prove the project has an artifact-capable execution profile?
- Should CLI validation expose a future `--artifact-capable` mode, or would that invite false governance?

## 16. Final Recommendation

Next implementation phase: **executor artifact-path workflow-declared gate integration, explicit path only**.

The implementation should connect the already-reviewed derivation helper to the already-explicit artifact-capable executor path and keep all default execution, validation, CLI, artifact, side-effect, write, hosted, and release postures unchanged.
