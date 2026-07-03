# WorkReport High-Assurance Approval Disclosure Plan

Status: First explicit WorkReport high-assurance approval disclosure slice implemented. Terminal report generation and executor report inputs can carry a bounded report-safe disclosure when supplied explicitly. High-assurance approval disclosure discovery planning and the first pure in-memory derivation helper are documented in [High-Assurance Approval Disclosure Discovery Plan](high-assurance-approval-disclosure-discovery-plan.md).

## 1. Executive Summary

High-assurance approval controls now have a domain-neutral core model, a pure validation helper, and a first opt-in local executor enforcement boundary.

The next question is how terminal `WorkReport` values should disclose that high-assurance approval posture when explicit approval enforcement has been used.

This plan defined a conservative report-disclosure path. The first implementation adds explicit report input and bounded disclosure propagation. It does not create new approval enforcement, workflow-declared controls, evidence attachment, schemas, CLI behavior, examples, write-capable adapters, side-effect execution, RBAC, IdP integration, quorum approval, hosted behavior, reasoning lineage, or release posture changes.

The intended first implementation should use existing report sections and citation vocabulary wherever possible. The report should cite stable approval references and bounded posture/disclosure summaries rather than copying approval payloads, high-assurance control payloads, actor IDs, evidence payloads, policy payloads, or provider data.

## 2. Goals

- Disclose when a terminal report was generated from a run whose approval decision used opt-in high-assurance validation.
- Preserve existing WorkReport and approval semantics.
- Preserve the default approval path behavior.
- Keep high-assurance disclosure explicit and local.
- Cite existing stable approval references where available.
- Cite supporting evidence, policy, validation, local check, audit, workflow event, adapter telemetry, side-effect, and work-report references by stable ID where supplied.
- Avoid raw approval context, actor IDs, provider payloads, command output, spec contents, parser payloads, source snippets, credentials, tokens, and secret-like values.
- Keep summaries bounded and redaction-safe.
- Preserve terminal report generation determinism.
- Prepare for later workflow-declared controls and governance-profile control sources without implementing them now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- new approval enforcement behavior;
- changes to `LocalExecutor::decide_approval(...)`;
- automatic high-assurance enforcement for every approval;
- workflow-declared high-assurance controls;
- workflow schema fields;
- runtime config;
- CLI behavior;
- examples;
- approval evidence attachment;
- automatic `EvidenceReference` creation;
- raw high-assurance control payload storage in reports;
- raw approval request or approval decision payload storage in reports;
- RBAC, IdP, SSO, SCIM, teams, groups, or external directory integration;
- quorum or multi-party approval enforcement;
- role-bound approval authority;
- approval UI;
- hosted or distributed runtime behavior;
- background expiration timers;
- approval revocation events;
- side-effect execution;
- write-capable adapters;
- provider mutations;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented approval and high-assurance baseline:

- approval-gated steps pause before skill invocation;
- approval requests and decisions are event-sourced;
- approval projections are rebuildable from event history;
- granted approvals resume execution;
- denied approvals fail closed;
- duplicate decisions and decisions after terminal states are rejected;
- `validate_high_assurance_approval_decision(...)` validates explicit controls and supplied stable references without mutation;
- `LocalExecutor::decide_approval_with_high_assurance(...)` validates before appending approval decision events;
- the default `LocalExecutor::decide_approval(...)` path remains unchanged.

Implemented WorkReport baseline:

- `WorkReport` includes a required approvals section;
- `WorkReportCitationTarget::ApprovalDecision` can cite stable `ApprovalReferenceId` values;
- terminal local report generation can cite explicitly supplied approval references;
- report generation uses bounded section text, bounded citations, redaction metadata, and conservative sensitivity;
- report artifact and side-effect integrity/linkage gates exist as explicit scoped helpers.

Not implemented:

- WorkReport-specific high-assurance disclosure fields;
- automatic high-assurance approval discovery from executor decisions;
- automatic report citation of high-assurance validation results;
- high-assurance disclosure in terminal report helper input;
- high-assurance disclosure in executor report input propagation;
- report artifact high-assurance validation gates;
- workflow-declared high-assurance controls.

## 5. Disclosure Boundary

The first disclosure boundary should be explicit report input, not automatic discovery.

Recommended first input shape:

- add bounded high-assurance approval disclosure inputs to terminal report generation or report input propagation;
- use existing approval citation targets for stable approval references;
- optionally add a small report-safe high-assurance disclosure model only if existing section text and citations are insufficient.

The first implementation should not inspect hidden global state, infer controls from policy strings, hydrate approval payloads, discover high-assurance controls from workflow specs, read provider data, or create evidence references.

## 6. What The Report Should Disclose

The approvals section should be able to disclose, in bounded text and citations:

- high-assurance approval validation was used;
- validation passed before the approval decision event was appended;
- whether the decision was granted or denied, if already represented by supplied approval references or safe section text;
- requester/approver separation posture, without copying actor IDs;
- required reference posture, using counts or stable cited references rather than payloads;
- expiration posture, such as not required, required on request, unexpired at decision, or unsupported/deferred;
- revocation posture, including unsupported/deferred behavior when applicable;
- denial behavior posture, limited to fail-closed for the implemented local subset;
- remaining limitations such as no RBAC, no IdP, no quorum, no revocation events, and no workflow-declared controls.

The report must not claim:

- enterprise identity assurance;
- role-bound authority;
- quorum approval;
- revocation enforcement;
- protected-use expiration enforcement;
- write authorization;
- provider mutation approval;
- safety-critical certification;
- Level 3/4 autonomy.

## 7. Citation Policy

The first implementation should reuse existing citation targets wherever possible:

- `ApprovalDecision` for approval references;
- `EvidenceReference` for supplied evidence reference IDs;
- `PolicyDecision` for policy event IDs;
- `ValidationDiagnostic` for validation reference IDs;
- `LocalCheckResult` for local check result references;
- `WorkflowEvent` for workflow event IDs;
- `AuditEvent` for audit event IDs;
- `AdapterTelemetry` for adapter telemetry references;
- `SideEffect` for side-effect IDs;
- `WorkReport` only if later disclosure cites prior reports.

Rules:

- cite stable references only;
- do not recreate `EvidenceReference` values;
- do not fabricate IDs;
- missing optional citations remain explicit section text;
- missing required citations should fail only when the future implementation explicitly declares them required;
- citation summaries must be bounded and redacted;
- citation construction failure must not create fake evidence or misleading approval posture.

## 8. Section Population Policy

Recommended first affected sections:

- `Approvals`: primary high-assurance approval posture disclosure.
- `Evidence considered`: cite supplied evidence references used by the high-assurance approval packet, if any.
- `Policy gates evaluated`: cite policy decision events used as required references, if any.
- `Validation and quality checks`: cite validation or local check references used as required references, if any.
- `Side effects`: cite side-effect IDs only when a high-assurance approval packet supplied side-effect references.
- `Known limitations`: disclose unsupported high-assurance capabilities such as no RBAC, no IdP, no quorum, no revocation enforcement, and no workflow-declared controls.
- `Risks`: disclose any missing optional high-assurance context or unresolved identity assumptions.
- `Operator handoff notes`: identify follow-up actions if high-assurance disclosure is incomplete.

Sections should remain present even when high-assurance disclosure is absent. Default report generation for runs without high-assurance inputs should behave exactly as before.

## 9. Input And Model Options

Option A: section-text-only disclosure.

- Add no new model type.
- Let callers supply bounded approval section text and existing approval/evidence/policy citations.
- Lowest implementation risk.
- Weakest structured posture for future validation.

Option B: report-safe disclosure input model.

- Add a small `WorkReportHighAssuranceApprovalDisclosure` or equivalent input type.
- Store bounded posture fields and counts, not raw controls or actor IDs.
- Convert the disclosure into approvals section text and existing citations.
- Better future validation boundary.

Option C: new WorkReport citation target.

- Add a dedicated high-assurance approval validation citation target.
- Higher public-surface cost.
- Should be deferred unless stable IDs for high-assurance validation results are introduced.

Recommendation: start with Option B only if the implementation needs structured posture beyond section text. Otherwise start with Option A and keep explicit citations as the source of structure.

## 10. Privacy And Redaction

High-assurance approval disclosures are sensitive by default.

The implementation must not store or copy:

- raw high-assurance control payloads;
- raw approval request payloads;
- raw approval decision payloads;
- actor IDs unless they are already safe stable references and explicitly intended for citation;
- raw evidence payloads;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira bodies/comments;
- raw GitHub file contents;
- raw spec contents;
- raw parser payloads;
- source snippets;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- secret-like redaction metadata.

Debug, serialization, deserialization errors, and report-generation errors must remain non-leaking.

## 11. Error Handling

Recommended conservative behavior:

- invalid disclosure input fails report generation or returns report-generation error according to the existing report path;
- invalid disclosure input must not mutate workflow state;
- invalid disclosure input must not append workflow events;
- invalid disclosure input must not change workflow pass/fail semantics;
- invalid disclosure input must not become a misleading project diagnostic;
- errors must use stable codes and avoid raw IDs, actor values, paths, snippets, payloads, tokens, and secret-like values.

If disclosure is optional and absent, report generation should continue with explicit not-available or no high-assurance disclosure text.

## 12. Test Plan

Future implementation tests should cover:

- reports with high-assurance approval disclosure populate the approvals section;
- default reports without high-assurance disclosure remain unchanged;
- approval decision references are cited by stable `ApprovalReferenceId`;
- supplied evidence references are cited without recreating `EvidenceReference` values;
- supplied policy, validation, local check, workflow event, audit event, adapter telemetry, and side-effect references are cited by existing targets where provided;
- missing optional disclosure remains explicit not-available section text;
- requester/approver posture does not copy actor IDs;
- required reference posture uses counts or citations, not payloads;
- expiration and revocation posture text is bounded and honest;
- unsupported RBAC/IdP/quorum/revocation/workflow-declared controls are disclosed as limitations when relevant;
- raw approval/control/provider/spec/command/parser payload markers are not copied;
- secret-like disclosure text is rejected or safely handled;
- `Debug` output is redaction-safe;
- serialization does not leak forbidden payloads;
- invalid disclosure input fails safely without mutating runtime state;
- existing WorkReport, approval, high-assurance helper, executor, side-effect, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests continue to pass.

## 13. Proposed Implementation Sequence

1. Add the smallest explicit terminal report disclosure input, preferably section-text and existing citation propagation first.
2. If needed, add a small report-safe high-assurance approval disclosure input type with bounded posture fields.
3. Populate the approvals section without changing required v1 sections.
4. Reuse existing `WorkReportCitation` constructors and citation targets.
5. Add focused report tests for disclosure, non-leakage, absent-disclosure behavior, and citation behavior.
6. Review before executor report-input propagation.
7. Only after review, consider executor propagation from `decide_approval_with_high_assurance(...)` outputs or report-bearing execution paths.

## 14. Deferred Work

- Automatic high-assurance disclosure discovery.
- Workflow-declared high-assurance controls.
- Governance-profile control sources.
- High-assurance validation result IDs.
- Dedicated high-assurance report citation targets.
- Approval evidence attachment.
- Approval revocation events.
- Protected-use expiration checkpoints.
- RBAC, IdP, SSO, SCIM, teams, groups, or external directory integration.
- Quorum or multi-party approval.
- Role-bound approval authority.
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

- Is bounded section text sufficient for the first disclosure slice, or should a small report-safe disclosure input model be introduced immediately?
- Should high-assurance validation produce a stable validation result ID before reports cite it directly?
- Should high-assurance disclosure remain attached to the approvals section only, or should it also populate known limitations and risks automatically?
- Should `current_time` used for high-assurance validation also become report disclosure metadata?
- Should report artifact gates later validate that high-assurance approval disclosure is present when high-assurance executor enforcement was used?
- How should workflow-declared high-assurance controls eventually require report disclosure without adding decorative schema fields?

## 16. Final Recommendation

Recommended next implementation phase: terminal WorkReport high-assurance approval disclosure input, explicit and report-only.

Start with existing report sections and citation targets. Do not build automatic discovery, workflow-declared controls, schemas, CLI behavior, approval evidence attachment, RBAC/IdP, quorum, revocation, write-capable adapters, side-effect execution, hosted behavior, reasoning lineage, or release posture changes.
