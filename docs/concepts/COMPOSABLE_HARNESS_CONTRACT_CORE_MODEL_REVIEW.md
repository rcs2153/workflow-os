# Composable Harness Contract Core Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The Composable Harness Contract core model is domain-neutral, model-only, and appropriately bounded. It adds validated contract vocabulary for bounded harness envelopes without introducing nested execution, runtime scheduling, schemas, CLI behavior, examples, reasoning lineage, side-effect boundary behavior, writes, domain packs, hosted/distributed runtime claims, or release posture changes.

The next phase should be typed handoff planning.

## 2. Scope Verification

The phase stayed within the approved core-model scope.

Implemented scope:

- `HarnessContract`;
- harness contract identity and version;
- input, context, tool, output, evidence, approval, and handoff requirement types;
- authority scope vocabulary;
- side-effect allowance vocabulary without enabling writes;
- failure semantics vocabulary;
- conservative execution policy model;
- validation;
- serde;
- redaction-safe `Debug`;
- focused tests;
- documentation and phase report.

No accidental implementation was found for:

- nested harness execution;
- runtime scheduling;
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

## 3. Model Assessment

The model is appropriately minimal and domain-neutral.

The implementation adds:

- `HarnessContract`;
- `HarnessContractId`;
- `HarnessContractVersion`;
- `HarnessInputRequirement`;
- `HarnessContextRequirement`;
- `HarnessToolAllowance`;
- `HarnessToolKind`;
- `HarnessAuthorityScope`;
- `HarnessSideEffectAllowance`;
- `HarnessOutputRequirement`;
- `HarnessEvidenceRequirement`;
- `HarnessApprovalRequirement`;
- `HarnessExecutionPolicy`;
- `HarnessFailureSemantics`;
- `HarnessHandoffRequirement`.

The model uses existing core primitives where appropriate:

- `SchemaVersion`;
- `RedactionMetadata`;
- `WorkReportSensitivity`;
- `WorkReportRedactionPolicy`.

That reuse avoids inventing a parallel redaction or sensitivity system before harness contracts have runtime behavior.

## 4. Product Boundary Assessment

The implementation preserves the Workflow OS product boundary.

The model describes bounded governed execution envelopes. It does not position Workflow OS as:

- a generic multi-agent framework;
- an agent swarm runtime;
- recursive agent orchestration;
- a hosted/distributed execution platform;
- a write-capable automation product;
- a Level 3 or Level 4 autonomy system.

The names remain domain-neutral and do not encode software-engineering-specific concepts such as pull requests, Jira tickets, branches, or CI runs.

## 5. Contract Field Assessment

The model captures the planned contract concerns:

- name or ID through `HarnessContractId`;
- contract version through `HarnessContractVersion`;
- schema version;
- purpose;
- allowed or required inputs;
- required context;
- allowed tools;
- scoped authority;
- side-effect allowance;
- output requirements;
- evidence requirements;
- approval requirements;
- timeout/budget/retry posture through `HarnessExecutionPolicy`;
- failure semantics;
- handoff requirements;
- sensitivity;
- redaction policy and redaction metadata.

This is enough for a model-only phase. Runtime enforcement and workflow-declared contract integration remain deferred.

## 6. Validation Assessment

Validation is deterministic and fail-closed.

`HarnessContract::new(...)` validates:

- contract ID;
- contract version;
- schema version secret-like content;
- bounded non-secret purpose;
- non-empty input requirements;
- non-empty context requirements;
- non-empty authority scopes;
- non-empty output requirements;
- non-empty evidence requirements;
- non-empty approval requirements;
- non-empty failure semantics;
- non-empty handoff requirements;
- duplicate requirement names within a requirement family;
- duplicate tool allowances;
- duplicate authority scopes;
- duplicate failure semantics;
- redaction metadata bounds and secret-like values.

Serde deserialization routes through `HarnessContract::new(...)`, so invalid serialized contracts fail closed.

Validation errors use stable codes and do not echo rejected values.

## 7. Privacy And Redaction Assessment

The implementation is redaction-safe for the current model-only surface.

Positive findings:

- `HarnessContract` uses private fields and read-only accessors.
- Requirement names are bounded and secret-like values are rejected.
- Purpose text is bounded and secret-like values are rejected.
- Redaction field names and reasons are bounded and secret-like values are rejected.
- `Debug` redacts purpose, requirement names, and redaction metadata.
- Tests cover debug non-leakage and serialization forbidden-marker non-leakage.

Serialization can contain valid bounded contract strings, which is acceptable for a model contract. The model should still be treated as sensitive if serialized later.

## 8. Serde And Compatibility Assessment

Serde support is appropriate for a core model.

Valid contracts serialize and deserialize.

Invalid serialized contracts fail closed through the validated constructor.

Field names are stable and sensible for a future schema-facing shape, but no workflow schema fields were introduced.

No TypeScript SDK or schema compatibility commitment was added in this phase.

## 9. Relationship To Work Reports And EvidenceReference

The model aligns with existing governed-work foundations.

It does not create `EvidenceReference` values.

It does not create WorkReports.

It does not require report generation or report artifacts.

It gives future harness contracts a place to declare evidence and handoff requirements while leaving actual evidence creation, citation, and report generation to existing or future scoped boundaries.

## 10. Relationship To Side Effects And Writes

The side-effect posture is safe.

`HarnessSideEffectAllowance` represents:

- unsupported;
- none;
- proposed only.

No attempted/completed write state, adapter write behavior, policy-gated write execution, or side-effect boundary runtime behavior was introduced.

`HarnessAuthorityScope::ProposeSideEffects` is vocabulary only and does not authorize execution.

## 11. Relationship To Reasoning Lineage

The implementation does not implement Reasoning Lineage / Claim Graph.

This is correct. Harness contracts can later provide useful boundaries for typed handoffs and provenance, but the current model does not add claim nodes, edges, corrections, confidence, actor attribution, lineage storage, or lineage citations.

## 12. Test Quality Assessment

The focused tests are appropriate for this phase.

Covered:

- valid minimal harness contract;
- invalid harness ID;
- invalid version;
- missing purpose;
- empty input requirements;
- required context;
- duplicate authority scopes;
- side-effect allowance without write support;
- evidence requirement;
- approval requirement;
- duplicate failure semantics;
- handoff requirement;
- sensitivity and redaction policy serde;
- serde round trip;
- invalid serde failure;
- debug non-leakage;
- serialization non-leakage;
- no runtime execution vocabulary;
- no domain-specific core requirements.

Existing workspace tests also passed.

Non-blocking test hardening:

- Add explicit tests for duplicate input, context, output, evidence, approval, and handoff requirement names.
- Add explicit tests for secret-like purpose and redaction metadata failures.
- Add explicit tests documenting whether empty tool allowances are allowed.

## 13. Documentation Review

Documentation is accurate about current state.

Verified docs state:

- Composable Harness Contract core model is implemented;
- nested harness execution is not implemented;
- runtime scheduling is not implemented;
- workflow schema fields are not implemented;
- CLI behavior is not implemented;
- examples are not updated;
- reasoning lineage is not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported;
- domain packs, hosted/distributed runtime behavior, and release posture changes are not introduced.

## 14. Blockers

None.

## 15. Non-Blocking Follow-Ups

- Decide whether empty tool allowance lists should remain valid for deterministic/no-tool harnesses or become invalid before schema exposure.
- Decide whether redaction metadata should require at least one declared state before schema exposure.
- Add duplicate-name tests for every requirement family.
- Add explicit secret-like purpose/redaction metadata tests.
- Consider whether a shared internal helper should reduce repeated requirement wrapper logic if another contract model adds similar fields.

## 16. Recommended Next Phase

Recommended next phase: typed handoff planning.

The contract model is now sufficient to discuss what a harness handoff should contain, how it should cite evidence, reports, local check results, approvals, risks, and next obligations, and how to keep handoffs typed instead of natural-language context dumps.

Do not implement nested harness execution yet. Do not add schemas, CLI behavior, examples, reasoning lineage, side-effect boundaries, writes, domain packs, hosted/distributed runtime behavior, or release posture changes in the next phase.
