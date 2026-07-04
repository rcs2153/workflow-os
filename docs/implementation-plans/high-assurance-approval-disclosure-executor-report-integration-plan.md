# High-Assurance Approval Disclosure Executor/Report Integration Plan

Status: Implemented. The first explicit in-memory high-assurance approval decision-with-disclosure executor/report bridge is implemented as an additive local executor API. Explicit report artifact high-assurance disclosure gate planning is documented in [Report Artifact High-Assurance Approval Disclosure Gate Plan](report-artifact-high-assurance-disclosure-gate-plan.md); the gate is planned, not implemented.

## 1. Executive Summary

Workflow OS now has:

- a high-assurance approval control model;
- a pure high-assurance approval decision validation helper;
- an opt-in executor approval decision method;
- an explicit `WorkReport` high-assurance approval disclosure model;
- a pure in-memory helper that derives report-safe disclosure from explicit bounded validation posture.

This plan addressed the executor/report composition gap. A caller can now use an explicit local executor method that validates a high-assurance approval decision, derives report-safe disclosure before decision events are appended, applies the approval decision, and returns the resulting `WorkflowRun` plus `WorkReportHighAssuranceApprovalDisclosure`.

This plan defined the conservative integration step. The implemented slice does not change default approval behavior, automatic report generation, report artifacts, workflow events, audit events, schemas, CLI behavior, workflow-declared controls, RBAC/IdP, quorum approval, write-capable adapters, side-effect execution, hosted behavior, reasoning lineage, or release posture.

## 2. Goals

- Provide a future explicit executor-adjacent path that returns high-assurance approval decision output plus report-safe disclosure.
- Preserve existing `LocalExecutor::decide_approval(...)` behavior.
- Preserve existing `LocalExecutor::decide_approval_with_high_assurance(...)` behavior unless a later implementation intentionally adds a new additive API.
- Preserve existing `LocalExecutor::execute_with_report(...)` behavior.
- Reuse `validate_high_assurance_approval_decision(...)`.
- Reuse `discover_high_assurance_approval_disclosure(...)`.
- Reuse `WorkReportHighAssuranceApprovalDisclosure`.
- Avoid workflow event inference.
- Avoid copying approval payloads, actor IDs, control payloads, evidence payloads, policy payloads, provider payloads, or command output.
- Keep report disclosure opt-in and explicit.
- Preserve workflow pass/fail semantics.
- Keep errors stable and non-leaking.
- Prepare for future report artifact gates without implementing them now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic disclosure for all approvals;
- changing `LocalExecutor::decide_approval(...)`;
- changing existing `LocalExecutor::decide_approval_with_high_assurance(...)` return type;
- changing `LocalExecutor::execute_with_report(...)`;
- automatic report generation;
- approval-resume report-bearing methods;
- cancellation report-bearing methods;
- workflow event scanning;
- new workflow event types;
- audit projection;
- report artifact gates;
- workflow-declared high-assurance controls;
- governance-profile control sources;
- workflow schema fields;
- runtime config;
- CLI behavior;
- examples;
- approval evidence attachment;
- automatic `EvidenceReference` creation;
- stable high-assurance validation result IDs;
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
- `validate_high_assurance_approval_decision(...)`.
- `LocalExecutor::decide_approval_with_high_assurance(...)`.
- `WorkReportHighAssuranceApprovalDisclosure`.
- `TerminalLocalWorkReportInput.high_assurance_approval`.
- `LocalExecutionReportInputs.high_assurance_approval`.
- `discover_high_assurance_approval_disclosure(...)`.
- Explicit forwarding of supplied disclosure into terminal reports.

Current behavior:

- `decide_approval_with_high_assurance(...)` validates before appending approval decision events.
- On high-assurance validation failure, no approval decision events are appended.
- On high-assurance validation success, the method returns `WorkflowRun` only.
- Report generation can disclose high-assurance posture only when the caller supplies `WorkReportHighAssuranceApprovalDisclosure`.
- No runtime state stores the high-assurance validation result as a stable record.
- Approval events do not encode high-assurance validation posture.

## 5. Source-Of-Truth Decision

The first executor/report integration source of truth should be the explicit approval decision call boundary, not workflow event inference.

Rejected for this phase:

- scanning approval events after the fact;
- inferring from approval reference IDs;
- inferring from report text;
- inferring from policy strings;
- hydrating approval payloads into report generation;
- creating a stable validation-result record.

Recommended:

- create an additive executor-adjacent path that performs high-assurance validation, applies the approval decision, and returns the resulting run plus the derived report-safe disclosure.

This keeps the source-of-truth boundary at the exact point where validation is performed.

## 6. Recommended First Integration Boundary

The first implementation should add a new explicit API rather than changing existing methods.

Recommended candidate names:

- `LocalHighAssuranceApprovalDecisionWithDisclosureRequest`
- `LocalHighAssuranceApprovalDecisionWithDisclosureResult`
- `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`

The method should:

1. accept the same explicit high-assurance inputs as `LocalHighAssuranceApprovalDecisionRequest`;
2. prepare the approval decision using existing executor logic;
3. validate through `validate_high_assurance_approval_decision(...)`;
4. derive disclosure through `discover_high_assurance_approval_disclosure(...)`;
5. append the approval decision only after validation and disclosure derivation succeed;
6. return the resulting `WorkflowRun` plus the derived `WorkReportHighAssuranceApprovalDisclosure`.

This is additive, explicit, local, and deterministic.

## 7. Candidate API Shape

Candidate request:

```rust
pub struct LocalHighAssuranceApprovalDecisionWithDisclosureRequest {
    pub approval: LocalApprovalDecisionRequest,
    pub controls: Vec<HighAssuranceApprovalControl>,
    pub supplied_references: Vec<HighAssuranceApprovalSuppliedReference>,
    pub current_time: Timestamp,
}
```

This may reuse `LocalHighAssuranceApprovalDecisionRequest` directly if the implementation can avoid duplicating input types.

Candidate result:

```rust
pub struct LocalHighAssuranceApprovalDecisionWithDisclosureResult {
    run: WorkflowRun,
    high_assurance_approval: WorkReportHighAssuranceApprovalDisclosure,
}
```

Result accessors should be read-only:

- `run(&self) -> &WorkflowRun`;
- `high_assurance_approval(&self) -> &WorkReportHighAssuranceApprovalDisclosure`;
- `into_parts(self) -> (WorkflowRun, WorkReportHighAssuranceApprovalDisclosure)`.

`Debug` output must not leak run IDs, approval IDs, actor IDs, control IDs, evidence IDs, reference values, paths, or free-form text.

## 8. Derivation Rules

The executor-adjacent method should derive discovery input from bounded validation context:

- `validation_used = true`;
- `validation_passed = true`;
- `decision = approval decision kind`;
- `control_count = controls.len()`;
- `requester_approver_rule = derived from validated controls`;
- `required_reference_count = total required references from controls`;
- `supplied_reference_count = supplied_references.len()`;
- `expiration_policy = derived from validated controls`;
- `revocation_policy = derived from validated controls`;
- `denial_behavior = derived from validated controls`.

Because the current high-assurance validation helper already rejects unsupported multi-control combinations, the first implementation may derive posture only after validation has succeeded.

If posture cannot be derived deterministically from the supported controls, the method should fail before appending decision events with a stable non-leaking error.

## 9. Multi-Control Policy

The current high-assurance enforcement slice supports a conservative subset. The integration should not invent aggregation semantics.

Recommended first implementation:

- support one validated control, or support multiple controls only when all report-relevant postures are identical;
- fail closed when multiple controls contain conflicting requester/approver, expiration, revocation, or denial behavior posture;
- count all controls and all required references;
- do not choose one control silently.

Potential stable error code:

- `high_assurance_approval.disclosure_integration.control_posture_conflict`

This prevents reports from overclaiming a simple posture when the approval was validated against mixed controls.

## 10. Report Input Usage

The first implementation should not automatically generate a report.

Recommended caller flow:

1. call `decide_approval_with_high_assurance_disclosure(...)`;
2. receive `(run, high_assurance_approval)`;
3. pass `high_assurance_approval` into `LocalExecutionReportInputs.high_assurance_approval` or `TerminalLocalWorkReportInput.high_assurance_approval`;
4. call the existing explicit report-bearing execution or report generation path as appropriate.

This keeps approval decision mutation and report generation separate while giving callers a safe bridge between them.

Do not automatically attach disclosure to every later report for the run until there is a stable storage or event source for the disclosure.

## 11. Workflow Semantics

The new executor-adjacent path must preserve workflow semantics.

Rules:

- validation failure returns an error before approval decision events are appended;
- disclosure derivation failure returns an error before approval decision events are appended;
- approval decision success appends the same workflow events as the existing high-assurance approval path;
- granted approval resumes the run exactly as today;
- denied approval fails closed exactly as today;
- the returned disclosure does not mutate runtime state;
- no post-terminal events are appended for report disclosure;
- report generation remains opt-in and separate.

Existing methods must remain unchanged:

- `LocalExecutor::decide_approval(...)`;
- `LocalExecutor::decide_approval_with_high_assurance(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_and_side_effect_discovery(...)`;
- `execute_with_report_artifact_and_side_effect_gates(...)`.

## 12. Error Handling

Errors must be stable and non-leaking.

Expected error classes:

- existing high-assurance validation errors;
- disclosure integration posture-conflict errors;
- existing approval state errors from `prepare_approval_decision(...)`;
- existing state backend errors.

Errors must not include:

- run IDs;
- workflow IDs;
- approval IDs;
- actor IDs;
- control IDs;
- evidence IDs;
- supplied reference values;
- control payloads;
- approval reasons;
- report text;
- file paths;
- provider payloads;
- command output;
- spec contents;
- parser payloads;
- tokens, credentials, private keys, or secret-like values.

No partial approval decision event may be appended when disclosure derivation fails.

## 13. Privacy And Redaction

The integration must remain reference-only.

It must not store or copy into result fields, reports, errors, Debug output, or serialization:

- raw approval request payloads;
- raw approval decision payloads;
- actor IDs;
- high-assurance control payloads;
- evidence payloads;
- policy payloads;
- provider payloads;
- command output;
- CI logs;
- Jira bodies/comments;
- GitHub file contents;
- spec contents;
- parser payloads;
- source snippets;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- secret-like text.

The returned disclosure should contain only booleans, enums, and bounded counts.

## 14. Relationship To Report Artifact Gates

Report artifact gates were deferred for this phase. Explicit high-assurance approval disclosure gate planning is now documented in [Report Artifact High-Assurance Approval Disclosure Gate Plan](report-artifact-high-assurance-disclosure-gate-plan.md); implementation remains future work.

After an executor-adjacent disclosure result exists and is reviewed, a later plan can decide whether explicit artifact write inputs may require:

- high-assurance approval disclosure when high-assurance approval references are supplied;
- high-assurance approval disclosure when side-effect records indicate sensitive approval posture;
- missing disclosure to fail artifact writing while preserving workflow result.

This plan does not authorize those gates.

## 15. Relationship To Future Stable Validation Result IDs

Stable high-assurance validation result IDs remain deferred.

The proposed integration returns an in-memory disclosure, not a durable validation-result record. This is enough to avoid false governance in the local executor path while avoiding a premature persistence model.

Future stable IDs may become useful for:

- WorkReport citations;
- EvidenceReference targets;
- artifact referential integrity;
- audit projection;
- reasoning lineage.

Those require separate planning.

## 16. Test Plan

Future implementation tests should cover:

- new approval-with-disclosure path returns completed run plus disclosure on granted approval;
- new approval-with-disclosure path returns failed run plus disclosure on denied approval;
- disclosure maps decision posture correctly;
- disclosure maps requester/approver posture correctly;
- disclosure maps expiration posture correctly;
- disclosure maps revocation posture correctly;
- disclosure contains bounded control and reference counts;
- disclosure can be passed into report input and appears in the approvals section;
- existing `decide_approval(...)` remains unchanged;
- existing `decide_approval_with_high_assurance(...)` remains unchanged;
- missing required references fail before decision events are appended;
- same-actor rejection fails before decision events are appended;
- disclosure derivation failure fails before decision events are appended;
- multiple conflicting control posture fails without leaking values;
- Debug output for new result is non-leaking;
- serialization is not added unless explicitly justified;
- no raw approval/control/evidence payloads are copied;
- report generation remains opt-in;
- no report artifacts are written;
- no CLI output is emitted;
- existing WorkReport, executor, approval, side-effect, adapter, validation, and runtime tests continue to pass.

## 17. Proposed Implementation Sequence

1. Add `LocalHighAssuranceApprovalDecisionWithDisclosureResult`.
2. Add an additive executor method or free helper for approval decision with disclosure.
3. Reuse `prepare_approval_decision(...)`, `validate_high_assurance_approval_decision(...)`, and `apply_approval_decision(...)`.
4. Add a small private posture-derivation helper from validated controls to discovery input.
5. Call `discover_high_assurance_approval_disclosure(...)` before appending decision events.
6. Return the run plus disclosure.
7. Add focused executor tests and report handoff tests.
8. Review before any artifact gate or automatic report integration.

## 18. Open Questions

- Should the implementation add a new method or a free helper to avoid widening `LocalExecutor` public surface?
- Should the result include the validation result counts separately, or only the disclosure?
- Should multiple controls be rejected unless posture is identical?
- Should the result include a stable approval reference ID if one is available?
- Should future report-bearing approval/cancellation methods use this result, or should approval report generation remain a separate phase?
- When should high-assurance validation result IDs become durable records?
- What artifact gate should first require high-assurance disclosure, if any?

## 19. Final Recommendation

Implemented first phase: explicit high-assurance approval decision with disclosure result, in-memory only.

The implementation adds an additive executor-adjacent API that returns `WorkflowRun` plus `WorkReportHighAssuranceApprovalDisclosure` for successful high-assurance approval decisions. It does not change existing approval methods, generate reports automatically, write artifacts, append disclosure events, add schemas, add CLI behavior, add workflow-declared controls, add RBAC/IdP/quorum/revocation enforcement, execute side effects, add write-capable adapters, implement hosted behavior, implement reasoning lineage, or change release posture. The next planned artifact-gate step is documented separately and remains explicit, opt-in, and unimplemented.
