# High-Assurance Approval Disclosure Executor/Report Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the intended explicit in-memory executor/report bridge for high-assurance approval disclosure without broadening into automatic report generation, report artifacts, workflow-declared controls, schemas, CLI behavior, RBAC/IdP, quorum approval, side-effect execution, writes, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved additive executor/report integration scope.

Implemented:

- `LocalHighAssuranceApprovalDecisionWithDisclosureResult`;
- `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`;
- private bounded posture derivation from validated controls;
- report-safe disclosure return for granted and denied high-assurance approval decisions;
- focused tests and documentation updates.

No accidental implementation found for:

- changing `LocalExecutor::decide_approval(...)`;
- changing `LocalExecutor::decide_approval_with_high_assurance(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- automatic disclosure for all approvals;
- automatic report generation;
- report artifacts or artifact gates;
- workflow event scanning;
- new workflow event types;
- audit projection;
- workflow-declared controls;
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
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. API Assessment

The API is appropriately narrow and additive.

`LocalHighAssuranceApprovalDecisionWithDisclosureResult` is read-only from the caller perspective and exposes:

- `run()`;
- `high_assurance_approval()`;
- `into_parts()`.

The method reuses `LocalHighAssuranceApprovalDecisionRequest` instead of introducing a parallel request type. That keeps the public model smaller and avoids drift between high-assurance approval enforcement and disclosure integration inputs.

The method name, `decide_approval_with_high_assurance_disclosure(...)`, is explicit enough that callers should not confuse it with default approval behavior or automatic report generation.

## 4. Workflow Semantics Assessment

Workflow semantics are preserved.

The new method:

1. rehydrates and checks the approval state through `prepare_approval_decision(...)`;
2. validates high-assurance controls through `validate_high_assurance_approval_decision(...)`;
3. derives disclosure through `discover_high_assurance_approval_disclosure(...)`;
4. appends approval decision events only through the existing `apply_approval_decision(...)`.

Validation failure and disclosure derivation failure occur before approval decision events are appended. Granted approvals resume runs through the existing path, and denied approvals fail closed through the existing path.

No report disclosure is persisted into runtime state, and no post-terminal disclosure event is appended.

## 5. Disclosure Derivation Assessment

Disclosure derivation is bounded and report-safe.

The private helper derives only:

- approval decision kind;
- control count;
- requester/approver rule;
- required reference count;
- supplied reference count;
- expiration policy;
- revocation policy;
- denial behavior.

It does not copy:

- approval payloads;
- actor IDs;
- control IDs;
- evidence IDs;
- supplied reference values;
- policy payloads;
- provider payloads;
- command output;
- raw text.

Multiple controls are accepted only when report-relevant posture is identical. Conflicting requester/approver, expiration, revocation, or denial posture fails closed with `high_assurance_approval.disclosure_integration.control_posture_conflict`.

This is the right conservative first aggregation policy.

## 6. Error-Handling Assessment

Error handling is stable and non-leaking.

The new integration uses existing high-assurance validation errors for enforcement failures and introduces bounded integration errors for disclosure-specific failure. The posture-conflict error does not include control IDs, actor IDs, evidence IDs, supplied reference values, approval reasons, paths, or payloads.

No partial approval decision event is appended when disclosure derivation fails.

## 7. Privacy And Redaction Assessment

The returned disclosure contains only booleans, enums, and bounded counts.

`LocalHighAssuranceApprovalDecisionWithDisclosureResult` has manual `Debug` output that reports status, event count, and a redacted disclosure field. It does not expose run IDs, approval IDs, actor IDs, control IDs, evidence IDs, reference values, paths, free-form text, provider payloads, command output, tokens, credentials, private keys, or secret-like values.

The result type does not add serialization.

## 8. Relationship To WorkReport Assessment

The implementation bridges approval enforcement and report input without generating reports automatically.

The returned `WorkReportHighAssuranceApprovalDisclosure` can be supplied to existing explicit report input. Tests confirm that the approvals section reflects the high-assurance disclosure when the caller passes it into `execute_with_report(...)`.

The implementation does not create `EvidenceReference` values, does not create stable validation result IDs, and does not infer disclosure from workflow events.

## 9. Relationship To Existing Approval Paths

Existing approval APIs remain unchanged:

- `decide_approval(...)` still returns `WorkflowRun`;
- `decide_approval_with_high_assurance(...)` still returns `WorkflowRun`;
- `execute_with_report(...)` still uses explicit report inputs and remains separate from approval decision mutation.

The new method is opt-in and does not alter default local approval behavior.

## 10. Test Quality Assessment

Tests cover the important behavior:

- granted high-assurance approval returns completed run plus disclosure;
- denied high-assurance approval returns failed run plus disclosure;
- disclosure maps decision posture;
- disclosure maps requester/approver posture;
- disclosure maps expiration posture;
- disclosure maps revocation posture;
- disclosure contains bounded required/supplied reference counts;
- disclosure can be passed into report input and appears in the approvals section;
- conflicting multi-control posture fails before decision events are appended;
- result `Debug` output does not leak run ID, approval ID, or evidence ID;
- existing high-assurance approval, WorkReport, executor, side-effect, adapter, validation, and runtime tests continue to pass.

Shallow or missing coverage:

- Missing required references and same-actor rejection are covered through existing high-assurance executor tests and pure validation tests, but not directly through `decide_approval_with_high_assurance_disclosure(...)`.
- The report handoff test checks the approvals section summary but does not assert that no report artifact was written. Existing report tests cover artifact posture separately.

These are non-blocking because the new method uses the same validation boundary before disclosure and before event append, and artifact writing is not invoked by the API.

## 11. Documentation Review

Documentation is honest about the implemented and deferred scope.

Verified docs state:

- the first explicit in-memory executor/report disclosure bridge is implemented;
- report generation remains explicit and separate;
- report artifacts are not implemented by this phase;
- workflow events and audit events for disclosure are not implemented;
- workflow-declared high-assurance controls are not implemented;
- CLI behavior is not implemented;
- schemas and examples are not implemented;
- RBAC/IdP, quorum approval, revocation enforcement, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add direct regression tests for `decide_approval_with_high_assurance_disclosure(...)` when required references are missing.
- Add direct regression tests for `decide_approval_with_high_assurance_disclosure(...)` when requester and approver are the same under a separation-required control.
- Consider a later report artifact gate plan that can require high-assurance approval disclosure when explicitly configured.
- Consider whether stable high-assurance validation result IDs are needed before artifact gates or reasoning lineage.

## 14. Recommended Next Phase

Recommended next phase: report artifact high-assurance disclosure gate planning.

The executor/report bridge now gives callers a reviewed source-of-truth handoff from approval enforcement into report disclosure. The next useful step is not broader approval automation; it is deciding whether explicit report artifact writing should be able to require this disclosure before writing an artifact, while preserving workflow semantics and avoiding automatic report generation.

## 15. Validation

Dogfood governance:

- workflow: `dg/review`;
- run ID: `run-1783122295024891000-2`;
- approval ID: `approval/run-1783122295024891000-2/review-scope-approved`;
- approval outcome: granted.
- phase-close status: completed;
- event summary: 39 events, including one approval request, one approval grant, eight policy decisions, six step schedules, six skill invocation requests, six starts, six successes, one run resume, and one run completion.

Validation commands run for this review:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`
