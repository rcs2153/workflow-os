# High-Assurance Approval Executor Enforcement Report

## 1. Executive Summary

The first executor-integrated high-assurance approval enforcement slice is implemented as an explicit opt-in local executor method.

`LocalExecutor::decide_approval_with_high_assurance(...)` validates supplied high-assurance approval controls and stable references before appending `ApprovalGranted` or `ApprovalDenied` events. The default `LocalExecutor::decide_approval(...)` path remains unchanged.

## 2. Scope Completed

- Added an explicit high-assurance approval decision request type.
- Added opt-in executor high-assurance approval validation.
- Reused the reviewed pure validation helper.
- Preserved default approval decision behavior.
- Preserved event-log source-of-truth behavior.
- Preserved approval projection rebuildability.
- Validated before approval decision events are appended.
- Appended no approval decision events on high-assurance validation failure.
- Added focused executor tests for grant, denial, missing references, and requester/approver separation.

## 3. Scope Explicitly Not Completed

- No automatic high-assurance enforcement for all approvals.
- No changes to `LocalExecutor::decide_approval(...)`.
- No workflow-declared high-assurance controls.
- No workflow schema fields.
- No runtime config.
- No CLI behavior.
- No examples.
- No WorkReport high-assurance disclosure.
- No approval evidence attachment.
- No RBAC, IdP, SSO, SCIM, team, group, or external directory integration.
- No quorum or multi-party approval enforcement.
- No role-bound approval authority.
- No approval UI.
- No hosted or distributed runtime behavior.
- No background expiration timers.
- No approval revocation events.
- No side-effect execution.
- No write-capable adapters.
- No provider mutations.
- No reasoning lineage.
- No release posture changes.

## 4. API Summary

The implementation adds:

- `LocalHighAssuranceApprovalDecisionRequest`
- `LocalExecutor::decide_approval_with_high_assurance(...)`

The request wraps the existing `LocalApprovalDecisionRequest` and adds:

- explicit high-assurance approval controls;
- supplied stable high-assurance approval references;
- a deterministic current timestamp for expiration checks.

The request implements redaction-safe `Debug` output by redacting approval details and exposing only counts plus the supplied timestamp.

## 5. Runtime Behavior

The opt-in method:

1. rehydrates the run;
2. verifies the run is waiting for approval and not terminal;
3. loads the event-backed approval request;
4. constructs the proposed approval decision;
5. validates high-assurance controls and supplied references;
6. appends no approval events if validation fails;
7. delegates to the existing grant or denial behavior if validation succeeds.

Granted approvals continue through `ApprovalGranted`, resume policy evaluation, `RunResumed`, and skill execution. Denied approvals continue through `ApprovalDenied` and fail closed.

## 6. Validation Boundary Summary

Validation remains explicit and local. The executor method does not infer controls from workflow specs, policy effect strings, runtime config, governance profiles, or hidden state.

Unsupported high-assurance control semantics still fail closed through the shared helper.

## 7. Redaction And Privacy Summary

The executor method does not store or copy raw provider payloads, command output, spec contents, parser payloads, source snippets, credentials, authorization headers, private keys, token-like values, or unbounded natural-language evidence.

Validation failures return stable helper errors. The focused same-actor regression verifies that requester/approver actor IDs are not leaked through the rendered error.

## 8. Test Coverage Summary

Added focused tests for:

- high-assurance granted approval resumes and completes the run;
- high-assurance denied approval fails closed;
- missing required stable reference rejects before decision events are appended;
- requester/approver same-actor rejection does not leak actor IDs and appends no events.

Existing approval tests continue to cover default approval behavior, event ordering, projection rebuildability, duplicate decisions, terminal-state rejection, denial, and audit metadata.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test local_executor high_assurance` - passed.

Broader validation should be run in the phase review or PR gate:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

## 10. Remaining Known Limitations

- High-assurance controls are explicit inputs only.
- Workflow-declared controls are not implemented.
- Governance profile control sources are not implemented.
- WorkReport high-assurance disclosure is not implemented.
- High-assurance validation success/failure does not create dedicated workflow events.
- Revocation and use-time expiration remain unsupported.
- Quorum and role-bound approval authority remain unsupported.
- This does not authorize write-capable adapters or provider mutations.

## 11. Recommended Next Phase

Recommended next phase: high-assurance approval executor enforcement review.

The implementation is security-sensitive because it sits at the approval decision boundary. It should be reviewed before WorkReport disclosure, workflow-declared controls, governance profile control sources, write-capable adapter readiness, or provider mutation planning.
