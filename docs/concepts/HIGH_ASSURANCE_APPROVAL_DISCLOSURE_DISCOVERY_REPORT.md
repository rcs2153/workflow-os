# High-Assurance Approval Disclosure Discovery Report

## 1. Executive Summary

The first pure high-assurance approval disclosure discovery helper is implemented.

The helper derives a report-safe `WorkReportHighAssuranceApprovalDisclosure` from explicit bounded validation/decision posture. It is in-memory only and does not inspect runtime state, scan workflow events, append events, write artifacts, call adapters, or copy approval/control/evidence payloads.

## 2. Scope Completed

- Added `HighAssuranceApprovalDisclosureDiscoveryInput`.
- Added `HighAssuranceApprovalDisclosureDiscoveryResult`.
- Added `HighAssuranceApprovalDisclosureNotAvailableReason`.
- Added `discover_high_assurance_approval_disclosure(...)`.
- Mapped successful grant and denial posture into `WorkReportHighAssuranceApprovalDisclosure`.
- Represented validation-not-used as bounded not-available output.
- Represented failed validation without claiming an approval decision was recorded.
- Rejected inconsistent validation-used/validation-passed posture.
- Rejected unbounded counts.
- Rejected passed posture for unsupported use-time expiration, revocation enforcement, and non-fail-closed denial behavior.
- Exported the helper and input/result types from `workflow-core`.
- Added focused tests for mapping, failure, not-available posture, unsupported posture, bounded debug output, and non-leaking errors.

## 3. Scope Explicitly Not Completed

- No automatic executor/report integration.
- No changes to `LocalExecutor::decide_approval(...)`.
- No changes to `LocalExecutor::decide_approval_with_high_assurance(...)` semantics.
- No workflow event scanning.
- No new workflow events.
- No audit projection.
- No report artifact gates.
- No workflow-declared high-assurance controls.
- No runtime config.
- No workflow schema fields.
- No CLI behavior.
- No examples.
- No approval evidence attachment.
- No stable high-assurance validation result IDs.
- No RBAC, IdP, SSO, SCIM, teams, groups, or directory integration.
- No quorum or multi-party approval.
- No revocation enforcement.
- No side-effect execution.
- No write-capable adapters.
- No hosted behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Helper API Summary

The helper is:

- `discover_high_assurance_approval_disclosure(input)`.

The input carries only bounded posture:

- validation-used and validation-passed booleans;
- proposed or recorded approval decision kind;
- control count;
- requester/approver rule;
- required and supplied reference counts;
- expiration policy;
- revocation policy;
- denial behavior.

The result returns:

- a discovered `WorkReportHighAssuranceApprovalDisclosure`; or
- a bounded not-available reason when validation was not used.

## 5. Source-Of-Truth Boundary

The helper does not prove that a runtime executor path used high-assurance validation. It gives future executor/report composition a reviewed, deterministic derivation boundary once such context is explicitly available.

This avoids false governance from event inference, report text, decorative YAML, or payload hydration.

## 6. Privacy And Redaction Summary

The helper does not store or copy:

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
- credentials, authorization headers, private keys, or token-like values.

Debug output exposes only booleans, counts, enums, and bounded not-available reason codes.

## 7. Test Coverage Summary

Added focused tests for:

- successful grant disclosure mapping;
- denied disclosure mapping;
- human approver posture as deferred rather than overclaimed;
- validation-not-used not-available result;
- failed validation disclosure without decision claims;
- inconsistent validation posture rejection;
- unsupported passed posture rejection;
- unbounded count rejection;
- bounded non-leaking debug output.

## 8. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test high_assurance_approval disclosure_discovery` - passed.
- `cargo clippy -p workflow-core --test high_assurance_approval -- -D warnings` - passed.

Broader validation should be run before merge:

- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

## 9. Remaining Known Limitations

- The helper is explicit input only.
- It does not integrate with executor approval decisions automatically.
- It does not create stable validation result IDs.
- It does not write report artifacts.
- It does not append events or audit records.
- Report artifact gates cannot yet rely on this automatically.
- Workflow-declared high-assurance controls remain deferred.

## 10. Recommended Next Phase

Recommended next phase: high-assurance approval disclosure discovery helper review.

This helper is small but security-sensitive because it becomes the future bridge between approval enforcement and report disclosure. It should be reviewed before executor-adjacent integration or report artifact gates.
