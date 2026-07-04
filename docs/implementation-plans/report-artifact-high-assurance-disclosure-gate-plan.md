# Report Artifact High-Assurance Approval Disclosure Gate Plan

Status: Planning only. This plan follows the accepted high-assurance approval disclosure executor/report integration review. It does not implement report artifact gates, automatic report generation, artifact writing changes, schemas, CLI behavior, examples, RBAC/IdP, quorum approval, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has an explicit local executor path that can validate a high-assurance approval decision, apply that decision, and return a bounded `WorkReportHighAssuranceApprovalDisclosure` alongside the resulting `WorkflowRun`.

Workflow OS also has an explicit report artifact path that can write validated `WorkReport` artifacts only after SideEffect referential integrity and approval-linkage gates pass.

The next question is whether an explicit report artifact write may require high-assurance approval disclosure before persisting a report artifact. This plan defines that future gate. The gate should remain opt-in, local, explicit, and artifact-scoped. It should fail artifact writing when required disclosure is absent or insufficient, while preserving workflow execution results, approval decisions, generated reports, event history, and runtime semantics.

This plan does not implement the gate.

## 2. Goals

- Define an explicit artifact-write gate for high-assurance approval disclosure.
- Preserve existing `LocalExecutor::execute(...)`, report generation, approval, and artifact-store behavior.
- Preserve workflow pass/fail semantics.
- Require disclosure only when a caller explicitly opts into the gate.
- Validate report-safe high-assurance approval disclosure before artifact write.
- Avoid inferring high-assurance posture from workflow events, report text, policy strings, or approval payloads.
- Avoid raw approval, actor, control, evidence, provider, command, spec, parser, or source payloads.
- Keep gate failures stable and non-leaking.
- Compose with existing SideEffect integrity and approval-linkage gates without changing their semantics.
- Prepare a narrow implementation prompt that can be reviewed before broader artifact or write readiness work.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic report generation;
- automatic report artifact writing from existing executor paths;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- changing `LocalExecutor::decide_approval(...)`;
- changing `LocalExecutor::decide_approval_with_high_assurance(...)`;
- changing `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`;
- changing base `WorkReportArtifactStore` semantics;
- workflow event scanning for high-assurance disclosure;
- new workflow event types;
- audit projection for high-assurance disclosure;
- stable high-assurance validation result records;
- EvidenceReference creation;
- approval evidence attachment;
- workflow-declared high-assurance controls;
- workflow schema fields;
- runtime config;
- CLI behavior;
- examples;
- RBAC, IdP, SSO, SCIM, teams, groups, or directory integration;
- quorum or multi-party approval enforcement;
- role-bound approval authority;
- revocation enforcement;
- protected-use expiration checkpoints;
- side-effect execution;
- provider mutation;
- write-capable adapters;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented high-assurance approval foundation:

- `HighAssuranceApprovalControl` model and validation.
- `validate_high_assurance_approval_decision(...)`.
- `LocalExecutor::decide_approval_with_high_assurance(...)`.
- `WorkReportHighAssuranceApprovalDisclosure`.
- terminal report and executor report input propagation for explicit disclosure.
- `discover_high_assurance_approval_disclosure(...)`.
- `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`, returning a run plus report-safe disclosure.

Implemented artifact foundation:

- `WorkReportArtifactRecord`.
- `WorkReportArtifactStore`.
- local artifact read/write/list support.
- explicit executor report artifact write path with SideEffect referential integrity and approval-linkage gates.

Current gap:

- an explicit artifact write can validate the report, SideEffect references, and approval-side-effect linkage, but it does not yet let callers require high-assurance approval disclosure before persisting a report artifact.

## 5. Gate Source Of Truth

The first gate should use the `WorkReport`'s explicit `WorkReportHighAssuranceApprovalDisclosure` value as the source of truth.

Accepted source:

```text
WorkReport.high_assurance_approval disclosure supplied through existing explicit report inputs
```

Rejected sources for this phase:

- approval event inference;
- workflow event scanning;
- policy string inference;
- approval reference ID inference;
- report section text parsing;
- high-assurance control payload hydration;
- actor ID inspection;
- provider payload inspection;
- stable validation-result records that do not yet exist.

This keeps the gate aligned with the already-reviewed executor/report disclosure bridge.

## 6. Recommended First Gate Boundary

Recommended first implementation: extend the explicit governed artifact write path, not the base artifact store trait.

The existing explicit artifact path already composes:

1. local execution;
2. in-memory report generation;
3. report artifact construction;
4. SideEffect referential integrity validation;
5. approval-side-effect linkage validation;
6. artifact write.

The high-assurance disclosure gate should fit between artifact construction and artifact write:

1. validate the `WorkReportArtifactRecord`;
2. verify artifact/report/run identity;
3. validate SideEffect referential integrity when requested;
4. validate approval-side-effect linkage when requested;
5. validate high-assurance approval disclosure when requested;
6. write the artifact only after all requested gates pass.

The gate should not be added to `WorkReportArtifactStore::write_work_report_artifact(...)`. The base store should remain responsible for storing validated artifacts, while governed write helpers compose optional governance gates.

## 7. Candidate API Shape

The smallest implementation should add explicit gate inputs to the existing artifact report input shape.

Candidate policy:

```rust
pub struct WorkReportArtifactHighAssuranceDisclosurePolicy {
    pub require_disclosure: bool,
    pub require_validation_used: bool,
    pub require_validation_passed: bool,
    pub require_fail_closed_denial_behavior: bool,
}
```

Candidate result:

```rust
pub struct WorkReportArtifactHighAssuranceDisclosureGateResult {
    pub disclosure_present: bool,
    pub validation_used: bool,
    pub validation_passed: bool,
    pub decision_disclosed: bool,
}
```

The final implementation may fold this into existing artifact input/result types if that is more idiomatic for the current codebase. The important boundary is that the gate is explicit, visible to the caller, and absent by default.

## 8. Gate Policy Semantics

Recommended first strict policy:

- if `require_disclosure` is false, do not enforce high-assurance disclosure;
- if `require_disclosure` is true and the report lacks disclosure, fail before artifact write;
- if `require_validation_used` is true, require the disclosure to indicate high-assurance validation was used;
- if `require_validation_passed` is true, require the disclosure to indicate validation passed;
- if `require_fail_closed_denial_behavior` is true, require denial behavior posture to be fail-closed when denial behavior is represented;
- require disclosure fields to remain bounded report-safe values;
- return bounded counts/booleans only.

Do not require every report artifact to carry high-assurance disclosure. Many governed runs do not use high-assurance approval controls.

## 9. Disclosure Matching Rules

The gate should validate only the disclosure carried by the `WorkReport` being stored.

Rules:

- do not infer that any approval reference implies high-assurance validation;
- do not infer that any SideEffect reference implies high-assurance validation;
- do not require high-assurance disclosure merely because the report has an approvals section;
- do not parse approval section text for posture;
- do not inspect raw approval request or approval decision payloads;
- do not create missing disclosure;
- do not repair or rewrite the report.

If a caller wants the gate to pass, the caller must provide report-safe disclosure through the existing explicit report input path.

## 10. Workflow Semantics And Failure Behavior

The gate must preserve workflow semantics.

Rules:

- report artifact gate failure must not change workflow pass/fail status;
- report artifact gate failure must not append workflow events;
- report artifact gate failure must not mutate `WorkflowRun` or `WorkflowRunSnapshot`;
- report artifact gate failure must not change approval decisions;
- report artifact gate failure must not remove a generated in-memory report;
- report artifact gate failure must return a structured artifact error;
- no partial artifact may be written when the gate fails;
- existing non-artifact executor paths remain unchanged.

This follows the existing artifact posture: report and artifact failures after a run exists are separate from workflow execution success/failure.

## 11. Artifact Semantics

A stored artifact with the gate enabled should mean only:

```text
This WorkReport artifact carried explicit high-assurance approval disclosure satisfying the requested artifact gate policy when it was written.
```

It must not mean:

- every approval in the run was high-assurance;
- high-assurance validation is persisted as a stable independent record;
- actors were verified by RBAC or IdP;
- quorum approval occurred;
- revocation enforcement exists;
- provider writes were approved or executed;
- side effects completed;
- safety-critical certification exists.

## 12. Privacy And Redaction

The gate must remain reference-only and posture-only.

It must not store, copy, or leak:

- raw approval request payloads;
- raw approval decision payloads;
- actor IDs;
- high-assurance control payloads;
- required reference values;
- evidence payloads;
- policy payloads;
- provider payloads;
- command output;
- CI logs;
- Jira issue or comment bodies;
- GitHub file contents;
- raw spec contents;
- raw parser payloads;
- source snippets;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- local paths;
- secret-like metadata.

Errors, Debug output, and serialized artifact results must expose only stable codes, booleans, enum posture, and bounded counts.

## 13. Relationship To Executor Approval Disclosure Bridge

The executor approval disclosure bridge is the recommended way to produce report-safe disclosure for this gate.

Expected caller flow:

1. call `decide_approval_with_high_assurance_disclosure(...)`;
2. receive `WorkReportHighAssuranceApprovalDisclosure`;
3. pass the disclosure into explicit report inputs;
4. generate a `WorkReport`;
5. call the explicit artifact write path with high-assurance disclosure gate enabled;
6. artifact write succeeds only if the report carries sufficient disclosure.

The artifact gate should not call approval decision APIs. It validates the report/artifact boundary only.

## 14. Relationship To Future Stable Validation Result IDs

Stable high-assurance validation result IDs remain deferred.

The first gate can validate disclosure posture without a durable validation-result record. Future stable IDs may later support:

- report citations;
- EvidenceReference targets;
- artifact integrity checks;
- audit projection;
- reasoning lineage.

Those should not be introduced just to implement the first artifact gate.

## 15. Relationship To SideEffect Approval Linkage

The high-assurance disclosure gate complements, but does not replace, existing SideEffect approval-linkage gates.

SideEffect approval linkage answers:

```text
Do cited SideEffect records have required approval linkage?
```

High-assurance disclosure gating answers:

```text
Does the WorkReport artifact disclose high-assurance approval posture when the caller requires that disclosure?
```

The first implementation should keep these gates separate and compose them only in the explicit governed artifact write path.

## 16. Test Plan

Future implementation tests should cover:

- artifact write succeeds when the gate is disabled and no high-assurance disclosure is present;
- artifact write succeeds when the gate is enabled and valid disclosure is present;
- artifact write fails before persistence when required disclosure is absent;
- artifact write fails before persistence when disclosure says validation was not used and the policy requires it;
- artifact write fails before persistence when disclosure says validation did not pass and the policy requires it;
- artifact write preserves the run and generated report when the gate fails;
- artifact write failure does not append workflow events;
- artifact write failure does not mutate snapshots;
- artifact write failure does not change approval decisions;
- existing artifact SideEffect integrity and approval-linkage gates still run as expected;
- existing non-artifact executor methods still do not write artifacts;
- errors use stable codes and do not leak IDs, actor values, references, paths, payloads, tokens, or secret-like values;
- Debug output is redaction-safe;
- serialization does not leak raw payload markers;
- existing high-assurance approval, WorkReport, artifact, SideEffect, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests continue to pass.

## 17. Proposed Implementation Sequence

1. Add a small explicit high-assurance disclosure artifact gate policy.
2. Add a pure validation helper for `WorkReportArtifactRecord` plus policy.
3. Thread the policy into the existing explicit governed artifact write path.
4. Return bounded gate result posture from the artifact write result.
5. Add focused tests for pass, fail, non-mutation, and non-leakage behavior.
6. Review before broadening into workflow-declared controls, CLI artifact behavior, or write-capable adapter readiness.

## 18. Open Questions

- Should the first gate require `validation_passed`, or should requiring disclosure presence be enough?
- Should fail-closed denial behavior be mandatory whenever the gate is enabled?
- Should the gate require at least one approval citation, or is disclosure posture sufficient?
- Should missing high-assurance disclosure ever be represented as a warning instead of an artifact-write failure?
- Should a future stable validation-result record be required before high-assurance disclosure gates become workflow-declared?
- Should the gate compose with SideEffect approval linkage only when SideEffect citations exist, or independently for all high-assurance approval reports?

## 19. Final Recommendation

Recommended next implementation phase: report artifact high-assurance approval disclosure gate, explicit and opt-in.

Implement a pure gate helper first, then compose it into the existing explicit executor-adjacent governed artifact write path. Do not change existing executor methods, do not make artifact writing automatic, do not add schemas or CLI behavior, do not infer high-assurance posture from events, do not add RBAC/IdP/quorum/revocation enforcement, do not execute side effects, do not add writes, and do not change release posture.
