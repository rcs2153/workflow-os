# High-Assurance Approval Runtime Validation Helper Report

## 1. Executive Summary

The first high-assurance approval runtime enforcement slice is implemented as a pure, explicit, in-memory validation helper.

The helper validates a proposed approval decision against supplied `HighAssuranceApprovalControl` values, the event-sourced `ApprovalRequest`, the proposed `ApprovalDecision`, supplied stable references, and a decision-time timestamp. It returns a validated summary or a stable non-leaking `WorkflowOsError`.

This phase does not wire high-assurance controls into `LocalExecutor::decide_approval(...)`. Existing approval behavior remains unchanged unless callers explicitly invoke the helper.

## 2. Scope Completed

- Added `HighAssuranceApprovalSuppliedReference`.
- Added `HighAssuranceApprovalDecisionValidationInput`.
- Added `HighAssuranceApprovalDecisionValidationResult`.
- Added `validate_high_assurance_approval_decision(...)`.
- Exported the helper and new types from `workflow-core`.
- Added focused tests for supported decision validation and fail-closed unsupported control paths.
- Updated roadmap and approval runtime documentation.

## 3. Scope Explicitly Not Completed

- No executor-integrated high-assurance approval enforcement.
- No changes to `LocalExecutor::decide_approval(...)`.
- No automatic enforcement for existing approvals.
- No workflow-declared high-assurance controls.
- No workflow schema changes.
- No CLI behavior.
- No examples.
- No write-capable adapters.
- No provider mutations.
- No runtime side-effect execution.
- No approval evidence attachment.
- No RBAC, IdP, SSO, SCIM, teams, groups, or external directory integration.
- No quorum or multi-party approval enforcement.
- No role-bound approval authority.
- No approval UI.
- No hosted or distributed runtime behavior.
- No background expiration timers.
- No approval revocation events.
- No reasoning lineage.
- No release posture changes.

## 4. Helper API Summary

`validate_high_assurance_approval_decision(...)` takes:

- an `ApprovalRequest`;
- a proposed `ApprovalDecision`;
- one or more `HighAssuranceApprovalControl` values;
- supplied stable references by name and target;
- a current timestamp for decision-time expiration checks.

The helper:

- validates the decision belongs to the request;
- rejects already-decided requests;
- enforces requester/approver separation where supported;
- supports exactly one required approval;
- validates required stable reference presence and exact target match;
- supports request expiration metadata checks and decision-time expiration checks;
- rejects unsupported use-time expiration, revocation, quorum, and non-fail-closed denial semantics.

It does not mutate workflow state, append events, emit audit output, write files, call adapters, or produce CLI output.

## 5. Validation Boundary Summary

Supported first:

- `SameActorAllowed`;
- `MustDiffer`;
- `HumanApproverMustDiffer` as local actor-difference only;
- `minimum_approvals == 1`;
- required reference presence by stable target;
- `NotRequired`, `RequiredOnRequest`, and `MustBeUnexpiredAtDecision` expiration policies;
- `Unsupported` revocation policy;
- `FailClosed` denial behavior.

Rejected fail-closed:

- `minimum_approvals > 1`;
- use-time expiration;
- revocation policies requiring future events or report behavior;
- denial behavior that requires runtime step blocking, cancellation, or escalation.

## 6. Redaction and Privacy Summary

The helper uses stable non-leaking validation errors. Error messages do not include actor IDs, approval IDs, reference names, reference IDs, paths, payloads, tokens, snippets, command output, provider payloads, or secret-like values.

`Debug` for the new input type redacts the approval request and approval decision and reports counts rather than approval packet contents. Supplied references reuse redaction-safe target debug behavior.

## 7. Test Coverage Summary

Focused tests cover:

- successful validation with different requester and approver;
- same-actor approval when the control explicitly allows it;
- self-approval rejection without actor leakage;
- missing required references;
- reference target mismatch;
- duplicate supplied reference names;
- unsupported minimum approval count;
- missing expiration metadata;
- expired decision-time approval request;
- unsupported use-time expiration;
- unsupported revocation policy;
- unsupported denial behavior;
- approval ID mismatch without ID leakage;
- debug output non-leakage.

Existing high-assurance approval model tests continue to cover serde, redaction metadata validation, target vocabulary, report disclosure vocabulary, and secret-like value rejection.

## 8. Commands Run and Results

- `cargo test -p workflow-core --test high_assurance_approval` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- The helper is explicit and opt-in only.
- The default local approval executor path does not call the helper.
- Workflow specs cannot declare high-assurance controls.
- There is no role authority, identity-provider integration, quorum, revocation event, background expiration timer, or protected-use checkpoint.
- Evidence sufficiency is not evaluated; only required stable reference presence and target equality are validated.
- WorkReport disclosure of high-assurance approval validation is not automatic.

## 10. Recommended Next Phase

Recommended next phase: high-assurance approval validation helper review.

The helper is security-sensitive and should be reviewed before any executor-integrated approval path uses it.
