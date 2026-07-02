# High-Assurance Approval Runtime Validation Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implemented helper is a narrow, explicit, in-memory validation gate for high-assurance approval decisions. It is suitable to proceed toward executor-integrated high-assurance approval enforcement planning, provided that executor wiring remains opt-in and separately reviewed.

## 2. Scope Verification

The phase stayed within approved helper-only scope.

Confirmed:

- no changes to `LocalExecutor::decide_approval(...)`;
- no automatic high-assurance approval enforcement;
- no workflow-declared high-assurance controls;
- no workflow schema changes;
- no CLI behavior;
- no examples;
- no write-capable adapters;
- no provider mutations;
- no runtime side-effect execution;
- no approval evidence attachment;
- no RBAC, IdP, SSO, SCIM, group, team, or external directory integration;
- no quorum or multi-party approval enforcement;
- no role-bound approval authority;
- no approval UI;
- no hosted or distributed runtime behavior;
- no background expiration timers;
- no approval revocation events;
- no reasoning lineage;
- no release posture changes.

## 3. Helper API Assessment

The helper API is appropriately explicit and local:

- `HighAssuranceApprovalDecisionValidationInput` carries borrowed approval request and decision records, explicit controls, supplied stable references, and a current timestamp.
- `validate_high_assurance_approval_decision(...)` returns a structured validation result or a stable `WorkflowOsError`.
- `HighAssuranceApprovalSuppliedReference` validates supplied reference names and reuses the existing redaction-safe target model.
- `HighAssuranceApprovalDecisionValidationResult` exposes counts only, avoiding accidental payload or ID exposure.

The API does not read hidden global state, runtime config, workflow schema fields, state backends, adapters, or filesystem artifacts. That matches the intended first enforcement boundary.

## 4. Supported Control Assessment

The helper correctly supports the first local subset:

- `SameActorAllowed`;
- `MustDiffer`;
- `HumanApproverMustDiffer` as local actor-difference only;
- `minimum_approvals == 1`;
- required reference presence and exact target matching;
- `NotRequired`;
- `RequiredOnRequest`;
- `MustBeUnexpiredAtDecision`;
- `Unsupported` revocation policy;
- `FailClosed` denial behavior.

The exact target match for supplied references is stricter than kind-only matching and is acceptable for the first enforcement slice because it avoids false proof from a same-kind but different stable reference.

## 5. Fail-Closed Assessment

Unsupported or unsafe semantics fail closed:

- empty controls are rejected;
- approval request and decision ID mismatch is rejected;
- already-decided requests are rejected;
- duplicate supplied reference names are rejected;
- missing required references are rejected;
- mismatched reference targets are rejected;
- `minimum_approvals > 1` is rejected;
- use-time expiration is rejected;
- revocation policies requiring future events or report behavior are rejected;
- non-`FailClosed` denial behavior is rejected.

This matches the roadmap posture: model vocabulary may exist ahead of runtime support, but unsupported governance behavior must not silently pass.

## 6. Runtime Semantics Assessment

The implementation does not change current approval runtime behavior. Existing default approvals still use the established event-sourced request/decision path, and high-assurance validation only runs when explicitly invoked by a caller.

The helper itself:

- does not mutate `WorkflowRun`;
- does not append approval events;
- does not write approval projections;
- does not emit audit or observability records;
- does not persist records;
- does not create report artifacts;
- does not authorize side effects or writes.

That is the correct boundary for a pre-executor-integration helper.

## 7. Privacy and Redaction Assessment

Privacy posture is acceptable.

Verified:

- error codes are stable;
- error messages do not include actor IDs, approval IDs, reference names, reference IDs, paths, payloads, command output, provider payloads, or token-like values;
- `Debug` for the validation input redacts approval request and decision records;
- `Debug` for supplied references redacts names and relies on redaction-safe target debug output;
- validation uses existing high-assurance identifier and redaction helpers;
- no raw provider payloads, raw spec contents, command output, environment values, credentials, authorization headers, private keys, or token-like values are stored by the helper.

## 8. Relationship to Existing Approval Model

The helper composes with the existing approval model without replacing it:

- it consumes `ApprovalRequest` and `ApprovalDecision` records;
- it does not create approval requests or decisions;
- it does not alter projection rebuildability;
- it does not replace the event log as source of truth;
- it prepares a future opt-in executor method to validate before appending approval events.

This is the right shape for future executor integration.

## 9. Test Quality Assessment

Tests cover the main accepted and rejected paths:

- successful different requester/approver validation;
- same actor allowed when explicitly configured;
- same actor rejected without actor leakage;
- missing required reference;
- reference target mismatch;
- duplicate supplied reference names;
- unsupported minimum approvals;
- missing expiration metadata;
- expired decision-time request;
- unsupported use-time expiration;
- unsupported revocation;
- unsupported denial behavior;
- approval ID mismatch without ID leakage;
- debug output non-leakage;
- existing model serde/redaction/vocabulary behavior.

Non-blocking test gaps:

- empty controls rejection is implemented but not directly tested;
- already-decided request rejection is implemented but not directly tested;
- multiple-control validation is not directly tested;
- optional required-reference absence is not directly tested;
- unexpired `MustBeUnexpiredAtDecision` success is not directly tested;
- `RequiredOnRequest` success with only `expires_after` or only `expires_at` is not directly tested;
- denied decision handling is not directly distinguished from granted decision handling.

These gaps do not block the phase because the high-risk supported and unsupported paths are covered, and full workspace regression tests pass.

## 10. Documentation Review

Docs accurately state:

- the pure validation helper is implemented;
- executor-integrated enforcement is not implemented;
- the default approval path is unchanged;
- high-assurance controls remain explicit and opt-in;
- write-capable adapters are not implemented;
- RBAC/IdP/quorum/revocation/background timers are not implemented;
- workflow schema fields are not implemented;
- CLI behavior and examples are not implemented;
- side-effect execution, reasoning lineage, hosted behavior, and release posture changes are not implemented.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add targeted tests for empty controls and already-decided request rejection.
- Add a multiple-control validation test before executor integration.
- Add optional-reference absence and present/mismatch tests.
- Add positive expiration tests for `RequiredOnRequest` and unexpired `MustBeUnexpiredAtDecision`.
- Decide whether denied approval decisions should have a distinct high-assurance validation posture before executor integration.
- In the executor-integration plan, define whether the helper should validate a grant-only path or both grant and deny decisions.

## 13. Recommended Next Phase

Recommended next phase: executor-integrated high-assurance approval enforcement planning.

The helper is now acceptable as a reviewed local enforcement primitive. The next phase should plan a narrow opt-in executor path that calls the helper before approval events are appended, without changing existing `decide_approval(...)`, workflow schemas, CLI behavior, side-effect execution, writes, or release posture.

## 14. Validation

Validation run for the implementation under review:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test high_assurance_approval` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
