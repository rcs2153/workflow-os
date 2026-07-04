# Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Plan

Status: Implemented as a pure derivation helper in [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Report](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_RUNTIME_DERIVATION_REPORT.md) and accepted in [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Review](../concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_RUNTIME_DERIVATION_REVIEW.md). Workflow-declared `report_artifact_requirements.high_assurance_approval` can now be mapped from a loaded workflow definition to explicit report artifact gate policy. Explicit executor artifact-path integration is implemented in [Workflow-Declared High-Assurance Artifact Requirement Executor Integration Plan](workflow-declared-high-assurance-artifact-requirement-executor-integration-plan.md). Default semantic validation still rejects enforcement postures outside the artifact-capable executor path. This plan does not implement automatic report generation, automatic artifact writing from default executor paths, CLI behavior, examples, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow specs can now declare `report_artifact_requirements.high_assurance_approval`.

Today, `not_required` validates. Stronger postures are schema-known but rejected by semantic validation with `validation.workflow.report_artifact_requirement.runtime_not_enforced` because no artifact-capable executor path is explicitly wired to derive and enforce authored workflow requirements yet.

The first implementation added a narrow derivation helper that maps a loaded workflow's artifact requirement posture to the existing `WorkReportArtifactHighAssuranceDisclosurePolicy`. The helper is explicit, local, deterministic, read-only, and non-mutating. It does not generate reports, write artifacts, change executor defaults, create runtime config, or make artifact persistence automatic.

## 2. Goals

- Derive workflow-declared high-assurance artifact requirement posture into explicit artifact gate policy.
- Preserve the invariant that declared governance is either enforceable or rejected.
- Reuse existing `WorkReportArtifactRequirement` and `WorkReportArtifactHighAssuranceDisclosurePolicy` vocabulary.
- Keep derivation local, deterministic, side-effect free, and testable.
- Prepare enforcement postures to become semantically valid only for runtime paths that call the derivation helper and explicit artifact gate.
- Preserve separation between workflow execution result and report artifact persistence result.
- Preserve redaction-safe diagnostics and errors.
- Avoid automatic report generation, automatic artifact writing, CLI behavior, examples, schemas, side-effect execution, writes, hosted behavior, or release posture changes.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic report generation;
- automatic report artifact writing;
- changing `LocalExecutor::execute(...)`;
- changing default `LocalExecutor::execute_with_report(...)` semantics;
- making existing executor paths persist artifacts by default;
- new workflow schema fields;
- TypeScript SDK changes;
- CLI rendering, export, or artifact write commands;
- example updates;
- runtime config for report artifact gates;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, SSO, SCIM, teams, groups, or directory integration;
- quorum or multi-party approval enforcement;
- role-bound authority enforcement;
- revocation enforcement;
- approval evidence attachment;
- new workflow event types;
- audit projection for artifact requirements;
- side-effect execution;
- provider mutation or write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented:

- `WorkReportArtifactRequirement` internal model.
- `WorkReportArtifactRequirementDefinition`.
- `WorkReportArtifactHighAssuranceRequirement` posture vocabulary.
- Deterministic mapping from requirement posture to `WorkReportArtifactHighAssuranceDisclosurePolicy`.
- Workflow schema field `report_artifact_requirements.high_assurance_approval`.
- Rust workflow parser support.
- TypeScript SDK type support.
- Pure runtime derivation helper from loaded workflow definition to artifact gate policy.
- Semantic validation that accepts only `not_required`.
- Explicit report artifact write path with SideEffect integrity, approval-linkage, and high-assurance disclosure gates.

Not implemented:

- executor artifact path integration from loaded workflow spec to artifact gate input;
- semantic acceptance of enforcement postures for any runtime path;
- executor artifact request construction from workflow declaration;
- automatic report generation;
- automatic artifact writing;
- CLI artifact behavior;
- examples.

## 5. Problem Statement

The current schema slice is honest but incomplete. Workflow authors can see the intended posture vocabulary, but any enforceable posture is rejected because no artifact-capable executor path accepts the declaration and enforces the derived gate policy.

The next bridge must avoid two failure modes:

- **false governance**: accepting YAML that looks enforceable while the runtime ignores it;
- **premature automation**: making artifact persistence automatic simply because a workflow declared a requirement.

The right next step is an explicit derivation helper that runtime artifact paths can call when they are already explicitly writing a report artifact.

## 6. Source-Of-Truth Boundaries

The source of authored intent is the loaded workflow definition:

```text
WorkflowDefinition.report_artifact_requirements.high_assurance_approval
```

The derivation output is gate policy:

```text
WorkReportArtifactHighAssuranceDisclosurePolicy
```

The artifact gate remains the enforcement point before persistence. The derivation helper does not validate approval decisions, inspect approval payloads, inspect workflow events, generate reports, or write artifacts.

Boundary summary:

- workflow spec declares terminal artifact requirement posture;
- semantic validation decides whether the declaration is allowed for the current runtime surface;
- derivation maps allowed posture to gate policy;
- explicit artifact write helper enforces the gate before persistence;
- artifact store persists only after all requested gates pass.

## 7. Recommended First Implementation Boundary

Recommended first implementation: add a pure derivation helper in `workflow-core`.

Candidate API shape:

```rust
pub struct WorkflowReportArtifactGateDerivationInput<'a> {
    pub workflow: &'a WorkflowDefinition,
}

pub struct WorkflowReportArtifactGateDerivation {
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
}

pub fn derive_workflow_report_artifact_gate_policy(
    input: WorkflowReportArtifactGateDerivationInput<'_>,
) -> Result<WorkflowReportArtifactGateDerivation, WorkflowOsError>
```

The final naming should follow repository conventions. The helper may instead accept `&LoadedSpec<WorkflowDefinition>` if source-location diagnostics or spec identity are useful, but it must not read project files or state backends.

## 8. Derivation Rules

Derivation should be deterministic:

- absent `report_artifact_requirements` derives disabled policy;
- `not_required` derives disabled policy;
- `disclosure_required` derives `require_disclosure()`;
- `validated_disclosure_required` derives `require_validated()`;
- `validated_fail_closed_disclosure_required` derives `require_validated_fail_closed()`.

The helper must not:

- create `WorkReport` values;
- create `WorkReportArtifactRecord` values;
- write artifacts;
- inspect `WorkflowRun` events;
- inspect approval payloads;
- inspect report section text;
- fabricate high-assurance disclosure;
- downgrade stronger postures silently;
- persist derived policy.

## 9. Validation Semantics Transition

Current semantic validation rejects enforcement postures globally because no artifact-capable executor path is wired to enforce the derived gate policy.

After the derivation helper is implemented and reviewed, validation should still be conservative. Two options exist:

### Option A: Keep global validation rejection until executor artifact wiring exists

Pros:

- strongest anti-false-governance posture;
- no authored workflow can declare enforceable posture until an executor path proves it can enforce it.

Cons:

- the derivation helper cannot be exercised by valid project specs outside focused unit tests.

### Option B: Allow enforcement posture only for explicit artifact-capable validation context

Pros:

- lets artifact-capable callers validate workflows that they can enforce;
- starts connecting declared posture to runtime composition.

Cons:

- requires validation context to distinguish artifact-capable and non-artifact-capable paths;
- risks confusing users if default `workflow-os validate` still rejects the same field.

Recommended for the next implementation: **Option A**.

Keep default semantic validation rejection in place while implementing the pure derivation helper. Then review the helper before planning any validation-context relaxation or executor artifact integration.

## 10. Runtime Integration Boundary

The future runtime integration point should be the explicit report artifact path:

```text
execute_with_report_artifact_and_side_effect_gates(...)
```

When integration is later approved, the executor artifact path may:

1. read the workflow definition already loaded for the run;
2. derive artifact gate policy from `report_artifact_requirements`;
3. combine derived policy with any explicit caller-supplied artifact gate posture according to a documented precedence rule;
4. call the existing governed artifact write helper;
5. return artifact/report errors without changing workflow execution result.

This plan does not implement that integration.

## 11. Precedence Rules For Future Integration

The first integration plan must decide how explicit caller policy and workflow-declared policy combine.

Recommended rule:

```text
effective_policy = stricter(workflow_declared_policy, explicit_caller_policy)
```

Why:

- callers may choose stricter posture than the workflow declares;
- callers must not silently weaken authored workflow requirements;
- default disabled caller policy should not erase workflow declarations.

The helper may expose a small composition function only after derivation is implemented and reviewed.

## 12. Error Handling

Derivation errors should be rare because parsing and semantic validation already own malformed authored input.

Rules:

- unknown values should fail during parsing, before derivation;
- unsupported future requirement vocabulary should fail closed if it reaches derivation;
- errors must use stable codes;
- errors must not echo workflow text, file paths, actor IDs, approval payloads, report text, policy payloads, provider payloads, tokens, or secret-like strings;
- derivation failure must not mutate workflow state, append events, write artifacts, or change workflow pass/fail result.

Potential stable code:

- `workflow.report_artifact_requirement.derivation.failed`
- `workflow.report_artifact_requirement.derivation.unsupported`

## 13. Workflow Semantics And Artifact Semantics

Workflow execution semantics remain unchanged.

Rules:

- declaring a report artifact requirement does not start a workflow;
- declaring a report artifact requirement does not generate a report;
- declaring a report artifact requirement does not write an artifact;
- derivation failure does not alter run status;
- artifact gate failure does not alter run status;
- generated reports remain available when artifact persistence fails;
- no partial artifact may be written when gate validation fails;
- no workflow events are appended for derivation in the first implementation.

A persisted artifact with derived high-assurance gate policy should mean only:

```text
The report artifact satisfied the effective high-assurance disclosure gate policy at write time.
```

It must not imply RBAC, IdP verification, quorum approval, revocation enforcement, provider writes, or production compliance certification.

## 14. Privacy And Redaction

The helper must remain posture-only.

It must not read, copy, store, or output:

- raw approval request payloads;
- raw approval decision payloads;
- actor IDs;
- high-assurance control payload bodies;
- evidence payloads;
- policy payloads;
- provider payloads;
- command output;
- CI logs;
- Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- local paths;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values;
- secret-like metadata.

Debug output should be bounded to posture names and booleans/counts.

## 15. Relationship To Governance Profiles

Governance profiles and enterprise stewardship remain future work.

Eventually, enterprise profiles may require, disallow, or strengthen workflow-local artifact requirements. The derivation helper should not encode steward policy yet. It should only map the workflow's declared posture to a gate policy.

Future profile composition can reuse the same stricter-policy composition rule after profile model boundaries are accepted.

## 16. Relationship To High-Assurance Approval Controls

Artifact requirements do not replace high-assurance approval controls.

The chain remains:

```text
high-assurance approval control validation
  -> report-safe high-assurance disclosure
  -> workflow-declared artifact requirement derivation
  -> artifact gate policy enforcement before persistence
```

The derivation helper should not infer that approval validation happened. It only selects the artifact gate policy that will require the report to disclose the relevant posture.

## 17. Relationship To Side Effects And Writes

This is still prerequisite governance composition before write-capable adapters.

It does not implement:

- side-effect execution;
- provider mutation;
- rollback or compensation;
- write-capable adapters;
- write authorization;
- external idempotency behavior.

Future write-capable adapter work should require this class of declared artifact/report governance to be composed and reviewed before sensitive write paths are accepted.

## 18. Test Plan

Future implementation tests should cover:

- absent requirement derives disabled policy;
- `not_required` derives disabled policy;
- `disclosure_required` derives require-disclosure policy;
- `validated_disclosure_required` derives require-validated policy;
- `validated_fail_closed_disclosure_required` derives require-validated-fail-closed policy;
- derivation does not read state backends;
- derivation does not generate reports;
- derivation does not write artifacts;
- derivation does not mutate workflow/run state;
- derivation errors are stable and non-leaking;
- unsupported future requirement vocabulary fails closed if it reaches derivation;
- existing project validation still rejects enforcement posture until runtime integration is separately approved;
- existing explicit artifact gate tests still pass;
- existing workflow schema and SDK contract tests still pass.

## 19. Proposed Implementation Sequence

Recommended small phases:

1. Add a pure workflow artifact gate derivation helper.
2. Add focused unit tests for posture-to-policy mapping and non-mutation.
3. Keep semantic validation rejection for enforcement postures unchanged.
4. Update docs and create an implementation report.
5. Review.
6. Plan explicit executor artifact-path integration and policy precedence.
7. Only after review, consider relaxing validation for artifact-capable paths.

## 20. Deferred Work

Deferred:

- executor artifact path derivation integration;
- semantic validation-context changes;
- automatic report generation;
- automatic artifact writing;
- CLI artifact behavior;
- examples;
- workflow-declared high-assurance approval controls;
- governance profile policy composition;
- RBAC/IdP;
- quorum approval;
- revocation enforcement;
- approval evidence attachment;
- workflow event or audit projection for derivation;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage;
- release posture changes.

## 21. Open Questions

- Should default `workflow-os validate` keep rejecting enforcement postures until artifact integration exists?
- Should a future artifact-capable validation mode exist, or should validation remain global and runtime artifact calls fail when unenforceable?
- Should explicit caller policy and workflow-declared policy always compose by strictest posture?
- Should governance profiles be allowed to strengthen or disable workflow-local artifact requirements?
- Should derived artifact gate policy be included in report artifact metadata, or only in the artifact write result?
- Should artifact gate derivation ever emit an audit projection, or remain artifact-result-only?
- What is the first user-facing explanation for why a declared artifact requirement does not automatically create an artifact?

## 22. Final Recommendation

Next implementation phase after this completed helper: **executor artifact-path workflow-declared gate integration**.

Implement only the explicit artifact-capable executor integration described in [Workflow-Declared High-Assurance Artifact Requirement Executor Integration Plan](workflow-declared-high-assurance-artifact-requirement-executor-integration-plan.md). Keep default validation and default executor paths conservative.

Do not build automatic report generation, automatic artifact writing, CLI behavior, examples, runtime config, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.
