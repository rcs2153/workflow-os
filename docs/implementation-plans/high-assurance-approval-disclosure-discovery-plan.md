# High-Assurance Approval Disclosure Discovery Plan

Status: First pure in-memory high-assurance approval disclosure discovery helper implemented and reviewed. The first explicit in-memory executor/report integration bridge is implemented in [High-Assurance Approval Disclosure Executor/Report Integration Plan](high-assurance-approval-disclosure-executor-report-integration-plan.md), and explicit artifact disclosure gating is implemented in [Report Artifact High-Assurance Approval Disclosure Gate Plan](report-artifact-high-assurance-disclosure-gate-plan.md). Automatic workflow-declared high-assurance controls, schemas, CLI behavior, write-capable adapters, and hosted behavior are not implemented.

## 1. Executive Summary

Workflow OS now has:

- a high-assurance approval control model;
- a pure high-assurance approval decision validation helper;
- an opt-in executor approval decision method;
- explicit `WorkReport` high-assurance approval disclosure input.

The current gap is source-of-truth composition. A terminal report can disclose high-assurance approval posture when a caller supplies a safe disclosure, but the runtime does not yet derive that disclosure from a high-assurance approval decision path.

This plan defines the next conservative step: a bounded discovery/derivation helper that converts explicit, already-validated high-assurance approval decision context into `WorkReportHighAssuranceApprovalDisclosure`. The first helper is implemented as pure in-memory code.

This plan does not implement event mutation, automatic executor/report integration, report artifact gates, workflow-declared controls, schemas, CLI behavior, RBAC/IdP, quorum approval, revocation enforcement, write-capable adapters, side-effect execution, hosted behavior, examples, reasoning lineage, or release posture changes.

## 2. Goals

- Decide the stable source of truth for high-assurance approval report disclosure.
- Avoid copying approval payloads, actor IDs, control payloads, or evidence payloads into reports.
- Preserve current `WorkReportHighAssuranceApprovalDisclosure` as the report-safe output.
- Keep discovery explicit, local, deterministic, and opt-in.
- Support future report artifact gates that can require disclosure when explicitly configured.
- Preserve default approval and report behavior.
- Preserve workflow pass/fail semantics.
- Keep errors stable and non-leaking.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic report disclosure for all approvals;
- changes to `LocalExecutor::decide_approval(...)`;
- changes to `LocalExecutor::decide_approval_with_high_assurance(...)` semantics;
- broad workflow event scanning;
- new workflow event types;
- approval evidence attachment;
- report artifact gate implementation;
- workflow-declared high-assurance controls;
- governance-profile control sources;
- workflow schema fields;
- runtime config;
- CLI behavior;
- examples;
- RBAC, IdP, SSO, SCIM, teams, groups, or directory integration;
- quorum or multi-party approval enforcement;
- role-bound approval authority;
- approval revocation enforcement;
- background expiration timers;
- provider mutations;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented:

- `HighAssuranceApprovalControl` model and validation.
- `validate_high_assurance_approval_decision(...)` pure helper.
- `LocalExecutor::decide_approval_with_high_assurance(...)` explicit opt-in decision path.
- `WorkReportHighAssuranceApprovalDisclosure` report-safe model.
- `TerminalLocalWorkReportInput.high_assurance_approval`.
- `LocalExecutionReportInputs.high_assurance_approval`.
- Executor report input forwarding for explicitly supplied disclosure.

Not implemented:

- a stable validation result ID for high-assurance approval validation;
- a high-assurance approval disclosure discovery helper;
- automatic disclosure from `decide_approval_with_high_assurance(...)`;
- workflow event/audit event vocabulary for high-assurance validation posture;
- report artifact gates requiring disclosure;
- workflow-declared high-assurance controls.

## 5. Source-Of-Truth Options

### Option A: Discover From Workflow Events

Read the run event history and infer whether high-assurance approval enforcement happened.

Assessment: reject for first implementation.

Why:

- existing approval events do not encode high-assurance validation posture;
- inference risks false governance;
- adding event vocabulary is a separate state-machine/audit planning problem;
- scanning events could tempt payload hydration.

### Option B: Discover From Existing Report Inputs

Treat `LocalExecutionReportInputs.high_assurance_approval` as discovery.

Assessment: insufficient.

Why:

- this is explicit propagation, not discovery;
- callers could supply disclosure unrelated to the actual approval path;
- report artifact gates need a stronger provenance boundary.

### Option C: Derive From Explicit Validated Decision Context

Create a pure helper that accepts already-available high-assurance approval decision inputs and validation outcome context, then produces a `WorkReportHighAssuranceApprovalDisclosure`.

Assessment: recommended first implementation.

Why:

- deterministic and local;
- no hidden state;
- no event mutation;
- no payload copying;
- reuses reviewed validation inputs and report-safe disclosure model;
- gives later executor/report composition a stable internal boundary.

### Option D: Create Stable High-Assurance Validation Result IDs

Add a new validation-result record and cite it from WorkReports.

Assessment: defer.

Why:

- useful later, but bigger than needed for first discovery;
- would require storage/identity/audit semantics;
- could overlap with future reasoning lineage and validation-result aggregation.

## 6. Recommended First Discovery Boundary

Implement a pure in-memory helper in a future phase.

Candidate shape:

- `HighAssuranceApprovalDisclosureDiscoveryInput`
- `HighAssuranceApprovalDisclosureDiscoveryResult`
- `discover_high_assurance_approval_disclosure(...)`

Inputs should be explicit and bounded:

- validation was attempted;
- validation succeeded or failed;
- proposed approval decision kind;
- control count;
- required reference count;
- supplied reference count;
- requester/approver rule posture;
- expiration policy posture;
- revocation policy posture;
- denial behavior posture;
- optional stable approval reference ID if already available.

The helper should output:

- `Option<WorkReportHighAssuranceApprovalDisclosure>`, or
- `WorkReportHighAssuranceApprovalDisclosureDiscoveryResult` containing the disclosure plus bounded skipped/not-available reason.

The helper should not:

- read runtime state;
- scan workflow events;
- append events;
- write artifacts;
- call adapters;
- create evidence references;
- inspect actor IDs;
- inspect approval payloads;
- store raw controls;
- decide approval validity by itself.

## 7. Relationship To Existing Validation Helper

The discovery helper should not replace `validate_high_assurance_approval_decision(...)`.

Recommended flow:

1. Caller runs `validate_high_assurance_approval_decision(...)` or an executor path that uses it.
2. Caller passes bounded validation outcome/context into the discovery helper.
3. Discovery helper derives `WorkReportHighAssuranceApprovalDisclosure`.
4. Report generation receives the disclosure through existing explicit report input.

This keeps validation, decision mutation, and reporting separate.

## 8. Relationship To Executor Integration

The first implementation should not change `LocalExecutor::decide_approval_with_high_assurance(...)`.

Future executor integration options:

- return a bounded disclosure result alongside the approval decision result;
- provide an executor-adjacent helper that callers can invoke immediately after successful high-assurance approval decision;
- later expose report-bearing approval/cancellation paths if separately planned.

Do not automatically attach disclosure to all report-bearing executions until the source-of-truth path is reviewed.

## 9. Report Artifact Gate Readiness

Report artifact gates should not require high-assurance disclosure until discovery has a reviewed source of truth.

After a discovery helper exists, a later artifact-gate plan can decide whether:

- explicit artifact write input may require high-assurance disclosure;
- missing disclosure fails artifact write but preserves workflow result;
- disclosure is required only when high-assurance approval references or side-effect records indicate sensitive approval posture.

That is out of scope for this plan.

## 10. Privacy And Redaction

The discovery helper must not store or copy:

- raw approval requests;
- raw approval decisions;
- actor IDs;
- high-assurance control payloads;
- evidence payloads;
- policy payloads;
- provider payloads;
- command output;
- spec contents;
- parser payloads;
- source snippets;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- secret-like text.

Debug output should expose only bounded booleans, counts, enums, and not-available reason codes.

Errors must use stable codes and must not include raw IDs, actor values, paths, snippets, payloads, tokens, or secret-like values.

## 11. Error Handling

Recommended behavior:

- invalid discovery input returns a structured non-leaking error;
- failed validation outcome can produce either no disclosure plus a bounded reason, or a disclosure with `validation_passed = false`;
- discovery failure must not mutate workflow state;
- discovery failure must not append workflow events;
- discovery failure must not change approval decision behavior;
- discovery failure must not change workflow pass/fail semantics;
- discovery failure must not become a misleading project diagnostic.

## 12. Test Plan

Future implementation tests should cover:

- successful high-assurance validation context produces disclosure;
- failed high-assurance validation context produces bounded failed or not-available posture;
- granted decision maps to `Granted`;
- denied decision maps to `Denied`;
- requester/approver rule maps to safe requester/approver posture;
- expiration policy maps to safe expiration posture;
- revocation policy maps to safe revocation posture;
- required/supplied reference counts are bounded;
- raw approval/control/evidence/policy/provider/spec/command/parser payload markers are not accepted or copied;
- actor IDs are not copied;
- invalid counts fail closed without leaking values;
- `Debug` output is redaction-safe;
- serialization of resulting disclosure remains safe;
- helper does not mutate runtime state or append events;
- existing high-assurance approval, WorkReport, executor, side-effect, EvidenceReference, validation, adapter, and runtime tests continue to pass.

## 13. Proposed Implementation Sequence

1. Add a pure in-memory discovery helper and explicit input/result types.
2. Map explicit validation outcome/context into `WorkReportHighAssuranceApprovalDisclosure`.
3. Add focused unit tests for mappings, redaction, invalid input, and non-mutation.
4. Review.
5. After review, consider executor-adjacent propagation from `decide_approval_with_high_assurance(...)`.
6. After that review, plan report artifact gates requiring high-assurance disclosure when explicitly configured.

## 14. Deferred Work

- Workflow event vocabulary for high-assurance validation posture.
- Audit projection for high-assurance validation posture.
- Stable high-assurance validation result IDs.
- Automatic disclosure from event history.
- Automatic report artifact gates.
- Workflow-declared high-assurance controls.
- Governance-profile control sources.
- Approval evidence attachment.
- RBAC, IdP, SSO, SCIM, teams, groups, or directory integration.
- Quorum or multi-party approval.
- Role-bound approval authority.
- Revocation enforcement.
- Protected-use expiration checkpoints.
- CLI rendering.
- Schemas.
- Examples.
- Hosted behavior.
- Side-effect execution.
- Write-capable adapters.
- Provider mutations.
- Reasoning lineage.
- Release posture changes.

## 15. Open Questions

- Should a failed high-assurance validation outcome produce a disclosure with `validation_passed = false`, or no disclosure plus a not-available reason?
- Should discovery output include bounded reason codes for skipped/not-available posture?
- Should the helper accept raw controls and summarize them, or only accept pre-summarized control posture? The recommended first slice is pre-summarized posture.
- Should `decide_approval_with_high_assurance(...)` eventually return disclosure context?
- Should report artifact gates require disclosure based on explicit caller policy, high-assurance approval reference presence, or side-effect sensitivity?
- Should future high-assurance validation result IDs become EvidenceReference targets, WorkReport citations, or both?

## 16. Final Recommendation

Recommended next implementation phase: pure high-assurance approval disclosure discovery helper, in-memory only.

Do not build event scanning, event mutation, report artifact gates, workflow-declared controls, schemas, CLI behavior, RBAC/IdP, quorum, revocation enforcement, write-capable adapters, side-effect execution, hosted behavior, reasoning lineage, or release posture changes.
