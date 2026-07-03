# WorkReport High-Assurance Approval Disclosure Report

## 1. Executive Summary

The first WorkReport high-assurance approval disclosure slice is implemented.

Terminal report generation can now accept an explicit, report-safe high-assurance approval disclosure and surface it through existing WorkReport approval and known-limitation sections. Executor report inputs forward the disclosure when supplied explicitly.

This is report-only. It does not add new approval enforcement, automatic discovery, workflow-declared controls, RBAC/IdP, quorum approval, revocation enforcement, writes, schemas, CLI behavior, examples, hosted behavior, or release posture changes.

## 2. Scope Completed

- Added a bounded `WorkReportHighAssuranceApprovalDisclosure` model.
- Added safe posture enums for approval decision, requester/approver separation, expiration, and revocation posture.
- Added validation for inconsistent validation-used/validation-passed posture.
- Added bounded reference-count validation.
- Added deserialization through the validated constructor.
- Added redaction-safe `Debug` behavior.
- Added explicit terminal report input support.
- Added explicit executor report input propagation.
- Populated the existing approvals section when disclosure is supplied.
- Added a known-limitation disclosure for unsupported enterprise approval capabilities.
- Preserved default report behavior when high-assurance disclosure is absent.

## 3. Scope Explicitly Not Completed

- No automatic high-assurance approval disclosure discovery.
- No automatic high-assurance approval enforcement for every approval.
- No changes to `LocalExecutor::decide_approval(...)`.
- No workflow-declared high-assurance controls.
- No workflow schema fields.
- No runtime config.
- No RBAC, IdP, SSO, SCIM, team, group, or external directory integration.
- No quorum or multi-party approval enforcement.
- No approval revocation enforcement.
- No background expiration timers.
- No approval evidence attachment.
- No provider mutations.
- No side-effect execution.
- No write-capable adapters.
- No CLI behavior.
- No examples.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Model And Input Summary

The implementation adds report-safe disclosure vocabulary:

- `WorkReportHighAssuranceApprovalDisclosure`
- `WorkReportHighAssuranceApprovalDisclosureDefinition`
- `WorkReportHighAssuranceApprovalDecision`
- `WorkReportHighAssuranceRequesterApproverPosture`
- `WorkReportHighAssuranceExpirationPosture`
- `WorkReportHighAssuranceRevocationPosture`

The disclosure stores bounded booleans, enums, and counts only. It intentionally does not store approval payloads, actor IDs, control payloads, provider data, command output, evidence payloads, policy payloads, or free-form approval text.

`TerminalLocalWorkReportInput` and `LocalExecutionReportInputs` now accept an optional high-assurance approval disclosure. The executor report path forwards the disclosure without discovering or inventing approval context.

## 5. Section And Citation Behavior

When disclosure is supplied, the approvals section summary states whether high-assurance validation was used and whether it passed before approval disclosure.

Existing approval citation behavior is preserved: supplied stable approval references continue to be cited through `WorkReportCitation`. The implementation does not add a dedicated high-assurance validation citation target and does not create evidence references implicitly.

Known limitations now include an explicit disclosure that RBAC, IdP, quorum approval, revocation enforcement, workflow-declared controls, and write access are not implemented.

Default reports without high-assurance disclosure remain unchanged.

## 6. Privacy And Redaction Summary

The disclosure boundary is intentionally narrow:

- no raw approval payloads;
- no actor IDs;
- no high-assurance control payloads;
- no evidence payloads;
- no policy payloads;
- no provider payloads;
- no command output;
- no spec contents;
- no parser payloads;
- no source snippets;
- no credentials, authorization headers, private keys, token-like values, or secret-like strings.

Invalid serialized disclosures fail closed through validated deserialization. Validation errors use stable codes and do not include raw approval values.

## 7. Test Coverage Summary

Added focused tests for:

- high-assurance disclosure populating the approvals section;
- approval citations remaining stable-reference based;
- known-limitation disclosure for unsupported enterprise approval features;
- default reports remaining unchanged when disclosure is absent;
- inconsistent validation posture rejection;
- invalid serialized disclosure failing closed;
- executor report input propagation without event mutation.

Existing WorkReport and local executor tests continue to cover report construction, citation construction, serialization, redaction-safe `Debug`, and event preservation.

## 8. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test work_report high_assurance_approval` - passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_forwards_high_assurance_approval_disclosure_without_mutating_events` - passed.

Broader validation should be run before merge:

- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

## 9. Remaining Known Limitations

- Disclosure is explicit input only.
- The report does not discover whether `decide_approval_with_high_assurance(...)` was used.
- Workflow-declared high-assurance controls are not implemented.
- Report artifact gates do not yet require high-assurance disclosure.
- Dedicated high-assurance validation result IDs are not implemented.
- Revocation and use-time expiration remain unsupported.
- Quorum and role-bound approval authority remain unsupported.
- This does not authorize write-capable adapters or provider mutations.

## 10. Recommended Next Phase

Recommended next phase: WorkReport high-assurance approval disclosure review.

This phase touches report-visible approval posture and should be reviewed before adding automatic disclosure discovery, report artifact gates, workflow-declared controls, governance profile control sources, write-capable adapter readiness, or provider mutation planning.
