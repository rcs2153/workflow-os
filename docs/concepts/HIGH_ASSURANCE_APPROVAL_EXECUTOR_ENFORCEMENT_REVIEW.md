# High-Assurance Approval Executor Enforcement Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The executor integration is narrow, opt-in, deterministic at the validation boundary, and aligned with the approved phase scope. It validates explicit high-assurance approval controls before approval decision events are appended, while preserving the existing default approval path.

## 2. Scope Verification

The phase stayed within the approved executor-enforcement scope.

Implemented scope:

- explicit `LocalHighAssuranceApprovalDecisionRequest`;
- opt-in `LocalExecutor::decide_approval_with_high_assurance(...)`;
- reuse of `validate_high_assurance_approval_decision(...)`;
- validation before `ApprovalGranted` or `ApprovalDenied` events;
- no decision events appended on high-assurance validation failure;
- preservation of default `LocalExecutor::decide_approval(...)` behavior;
- focused tests and documentation updates.

No accidental implementation was found for:

- automatic high-assurance enforcement;
- workflow-declared high-assurance controls;
- workflow schema fields;
- runtime config;
- CLI behavior;
- examples;
- WorkReport high-assurance disclosure;
- approval evidence attachment;
- RBAC, IdP, SSO, SCIM, team, group, or external directory integration;
- quorum or multi-party approval;
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

## 3. Executor API Assessment

The added API is appropriately minimal and executor-adjacent.

`LocalHighAssuranceApprovalDecisionRequest` wraps the existing `LocalApprovalDecisionRequest` and adds only the high-assurance inputs required by the reviewed validation helper:

- explicit controls;
- supplied stable references;
- explicit current time for decision-time expiration checks.

`LocalExecutor::decide_approval_with_high_assurance(...)` is additive. Existing callers of `decide_approval(...)` are unaffected.

The request `Debug` implementation redacts the nested approval request and reports only bounded counts plus the explicit timestamp. That is consistent with the redaction posture used elsewhere in the runtime surface.

## 4. Runtime And Event Ordering Assessment

The implementation validates high-assurance controls before mutating runtime state.

The executor path:

1. rehydrates the run through the existing approval path;
2. verifies terminal and waiting-for-approval state through the existing checks;
3. loads the event-backed approval request;
4. constructs the same proposed approval decision shape as the default path;
5. runs high-assurance validation;
6. appends no approval decision events if validation fails;
7. delegates to the existing grant or denial behavior after validation succeeds.

This preserves event-log source-of-truth semantics and approval projection rebuildability. The tests verify that validation failures leave the run in `WaitingForApproval` with no added decision events.

## 5. Validation Boundary Assessment

The phase correctly keeps high-assurance enforcement explicit and local.

The executor does not infer controls from:

- policy effect strings;
- workflow specs;
- runtime config;
- governance profiles;
- hidden global state.

Required references are supplied explicitly and validated by the shared helper. Missing references fail closed with stable codes. Unsupported helper semantics such as use-time expiration, revocation enforcement, and non-fail-closed denial behavior remain rejected rather than partially implemented.

## 6. Grant And Denial Assessment

Both granted and denied decisions are validated before durable approval decision events are appended.

This is the right posture. Grants authorize continuation, and denials create durable approval history. Validating both paths avoids recording unsupported or mismatched high-assurance packets under sensitive approval IDs.

After validation succeeds:

- granted approvals preserve existing resume behavior;
- denied approvals preserve existing fail-closed behavior;
- gated skill invocation remains blocked until approval grant and resume complete.

## 7. Privacy And Error Assessment

The implementation does not copy raw provider payloads, command output, spec contents, parser payloads, source snippets, credentials, authorization headers, private keys, token-like values, or unbounded natural-language evidence.

Validation errors come from the shared high-assurance helper and use stable non-leaking codes. The same-actor regression test verifies that actor identifiers are not leaked through the rendered error.

No misleading project diagnostics are introduced. High-assurance validation failure remains a runtime approval decision failure.

## 8. Compatibility Assessment

The default approval API remains unchanged:

- `LocalExecutor::decide_approval(...)` still applies the existing approval decision behavior;
- existing approval grant, denial, duplicate decision, terminal-state rejection, and approval-resume tests continue to pass;
- public exports are consistent with existing executor model exports.

The public surface is intentionally additive. It does not force high-assurance controls on current local users or existing workflow projects.

## 9. Test Quality Assessment

The focused tests cover the critical first slice:

- high-assurance grant validates and resumes;
- high-assurance denial validates and fails closed;
- missing required references fail before decision events are appended;
- requester/approver separation failure is non-leaking and appends no decision events.

The broader workspace suite covers existing approval behavior, event ordering, report behavior, side-effect linkage, evidence references, diagnostics, adapter telemetry, project validation, and runtime events.

Remaining useful test expansion is non-blocking:

- explicit expiration-at-decision success and failure through the executor method;
- unsupported revocation policy through the executor method;
- duplicate supplied reference failure through the executor method;
- redaction-safe `Debug` regression for the new request type;
- explicit proof that default `decide_approval(...)` remains free of high-assurance validation.

## 10. Documentation Review

Documentation now states that:

- the first opt-in executor boundary is implemented;
- default approval behavior remains unchanged;
- automatic high-assurance enforcement is not implemented;
- workflow-declared controls are not implemented;
- WorkReport disclosure is not implemented;
- approval evidence attachment is not implemented;
- write-capable adapters, provider mutations, side-effect execution, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

The end-of-phase report is present.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add executor-level tests for decision-time expiration, unsupported revocation policy, duplicate references, and request `Debug` redaction.
- Decide whether high-assurance approval decisions should persist the explicit validation `current_time` as `ApprovalDecision.decided_at`, or keep using the existing default approval timestamp behavior.
- Plan WorkReport disclosure for high-assurance approval posture after this executor boundary is accepted.
- Plan workflow-declared high-assurance controls only after explicit executor semantics remain stable.

## 13. Recommended Next Phase

Recommended next phase: WorkReport high-assurance approval disclosure planning.

The executor can now enforce explicit high-assurance approval controls before recording approval decisions. The next useful connection is disclosure: terminal reports should be able to cite and summarize high-assurance approval posture without copying sensitive approval context or implying unsupported RBAC, quorum, revocation, write-capable adapter, or hosted behavior.

## 14. Validation

Review validation passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
