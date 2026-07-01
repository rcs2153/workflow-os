# High-Assurance Approval Control Core Model Review

## 1. Executive Verdict

Needs blocker fixes.

The phase stayed within the approved model-only scope and the implemented control model is directionally correct. The core `HighAssuranceApprovalControl` constructor and top-level deserialization path validate required fields, duplicate declarations, sensitivity posture, and redaction metadata without adding runtime enforcement.

One blocker remains: the exported nested `HighAssuranceApprovalRequiredReference` type derives `Deserialize` directly. That allows standalone deserialization of the public type to bypass the validated constructor and carry an invalid or secret-like reference name until it is embedded in a validated control. Because this is part of the public model surface, deserialization must fail closed at the type boundary.

Fix-forward note: this blocker is addressed in [High-Assurance Approval Control Core Model Blocker Fix Report](HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_BLOCKER_FIX_REPORT.md). This review preserves the original finding for auditability.

## 2. Scope Verification

The phase stayed within model-only scope.

Implemented scope:

- domain-neutral high-assurance approval control model types;
- protected action vocabulary;
- requester/approver separation vocabulary;
- minimum approval count;
- required governance reference vocabulary;
- expiration and revocation posture vocabulary;
- denial behavior vocabulary;
- report disclosure vocabulary;
- sensitivity and redaction posture;
- deterministic validation;
- serde support;
- redaction-safe `Debug`;
- focused tests;
- documentation and phase report.

No accidental implementation was found for:

- runtime high-assurance approval enforcement;
- changes to existing approval request or decision execution;
- requester/approver identity enforcement;
- role-based approval authority;
- IdP, SSO, SCIM, groups, teams, or enterprise directory integration;
- quorum or multi-party approval enforcement;
- approval revocation enforcement;
- approval expiration timers;
- evidence sufficiency enforcement;
- approval evidence attachment;
- write-capable adapters;
- provider mutations;
- runtime side-effect execution;
- workflow schema fields;
- CLI behavior;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Model Assessment

The model is domain-neutral and appropriately small for the first high-assurance approval foundation.

Verified modeled concepts:

- `HighAssuranceApprovalControl`;
- `HighAssuranceApprovalControlDefinition`;
- `HighAssuranceApprovalControlId`;
- `HighAssuranceApprovalControlVersion`;
- `HighAssuranceProtectedActionKind`;
- `HighAssuranceRequesterApproverRule`;
- `HighAssuranceApprovalExpirationPolicy`;
- `HighAssuranceApprovalRevocationPolicy`;
- `HighAssuranceApprovalDenialBehavior`;
- `HighAssuranceApprovalReportDisclosure`;
- `HighAssuranceApprovalRequiredReference`;
- `HighAssuranceApprovalRequiredReferenceTarget`.

The model uses existing governance reference primitives instead of introducing domain-specific provider concepts. It aligns with the plan by preparing the approval boundary before write-capable adapters without claiming enforcement.

## 4. Approval Posture Assessment

The model captures the expected high-assurance approval posture:

- protected action classes;
- requester/approver separation policy vocabulary;
- minimum approval count;
- expiration policy;
- revocation policy;
- denial behavior;
- report disclosure requirements;
- sensitivity;
- redaction policy.

The requester/approver vocabulary is intentionally local and model-only. It does not imply strong identity, human identity proof, external directory integration, quorum, or role authority.

## 5. Required Reference Assessment

The required reference target vocabulary is appropriate and reference-only.

Supported target vocabulary includes:

- EvidenceReference ID;
- policy decision event ID;
- SideEffect ID;
- validation reference ID;
- local check result ID;
- workflow event ID;
- audit event ID;
- WorkReport ID;
- adapter telemetry stable reference.

This is useful for future approval packets because it lets approval controls require cited governance context without copying raw payloads.

The blocker is isolated to standalone deserialization of `HighAssuranceApprovalRequiredReference`, not to the target ID primitives or top-level control validation.

## 6. Validation Assessment

Top-level `HighAssuranceApprovalControl` validation checks:

- control ID validity;
- control version validity;
- schema version not secret-like;
- protected action list is non-empty;
- duplicate protected actions are rejected;
- minimum approvals is nonzero;
- required reference list is non-empty;
- duplicate required reference names are rejected;
- report disclosure list is non-empty;
- duplicate report disclosures are rejected;
- redaction metadata fields and reasons are bounded and not secret-like.

Validation errors use stable `high_assurance_approval.*` codes and do not include raw values.

Blocker:

- `HighAssuranceApprovalRequiredReference` derives `Deserialize` directly, so a caller can deserialize that public type without running `HighAssuranceApprovalRequiredReference::new(...)` or `validate_identifier(...)`.
- Fix by adding constructor-backed deserialization for `HighAssuranceApprovalRequiredReference`, with focused tests for invalid and secret-like serialized reference names.

## 7. Privacy And Redaction Assessment

The model avoids storing raw payloads and keeps `Debug` output redaction-safe.

Verified safe posture:

- `HighAssuranceApprovalControlId` and version `Debug` redact values;
- `HighAssuranceApprovalRequiredReferenceTarget` `Debug` redacts underlying references;
- `HighAssuranceApprovalRequiredReference` `Debug` redacts name and target value details;
- `HighAssuranceApprovalControl` `Debug` reports counts and posture without raw references or redaction values;
- redaction metadata is bounded and screened for secret-like field names and reasons at the top-level control boundary.

No evidence was found that the model stores:

- raw provider payloads;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw command output;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

The remaining privacy concern is the deserialization bypass for the nested required-reference type.

## 8. Serde And Compatibility Assessment

Valid top-level controls serialize and deserialize.

Invalid top-level controls fail closed through constructor-backed `Deserialize` for `HighAssuranceApprovalControl`.

Field names are stable and sensible for a future schema shape. No workflow schema changes were introduced.

Blocker:

- nested `HighAssuranceApprovalRequiredReference` has serde support but does not independently enforce its constructor validation on deserialization.

Non-blocking compatibility note:

- Future schema exposure should decide whether the high-assurance control surface remains a core model API first or becomes workflow-declared configuration. This phase correctly avoids schema exposure.

## 9. Relationship To Existing Approvals

The model does not alter existing approval runtime behavior.

Current approval semantics remain:

- approval-gated steps pause before skill invocation;
- approval request events remain source of truth;
- grants resume execution;
- denials fail closed;
- duplicate decisions and decisions after terminal states are rejected;
- approval projections remain rebuildable.

The model prepares future enforcement for requester/approver separation, evidence-required context, expiration/revocation, and disclosure, but does not implement those runtime behaviors.

## 10. Relationship To SideEffect And WorkReport

The model fits the SideEffect and WorkReport roadmap:

- SideEffect IDs can be required references without executing side effects;
- policy decision events and approval context can be required before future protected actions;
- WorkReports can later disclose requested, granted, denied, expired, revoked, skipped, deferred, evidence-considered, and side-effect-authorized approval posture.

No report generation, report artifact behavior, SideEffect execution, provider mutation, or automatic approval-side-effect validation was added.

## 11. Test Quality Assessment

Tests cover:

- valid minimal high-assurance approval control;
- invalid control ID;
- invalid version;
- secret-like schema version rejection;
- empty protected action rejection;
- duplicate protected action rejection;
- zero approval count rejection;
- empty required reference list rejection;
- duplicate required reference names;
- required reference target vocabulary;
- future write vocabulary as model-only;
- requester/approver separation vocabulary;
- expiration and revocation vocabulary;
- non-empty and unique report disclosures;
- all report disclosures;
- serde round trip for valid control;
- invalid serialized top-level control failure;
- secret-like redaction metadata rejection without leakage;
- `Debug` non-leakage;
- serialization avoiding forbidden raw payload markers.

Missing blocker tests:

- invalid serialized `HighAssuranceApprovalRequiredReference` fails closed;
- secret-like serialized `HighAssuranceApprovalRequiredReference.name` fails closed without leakage;
- valid standalone required-reference serde round trip still works after constructor-backed deserialization.

Non-blocking test follow-ups:

- add a test that deserialization errors for top-level invalid redaction metadata do not leak raw secret-like values;
- add a test that top-level serialized control does not include raw reference IDs in `Debug` while serialization still carries allowed stable reference IDs.

## 12. Documentation Review

Documentation correctly states:

- high-assurance approval controls are planned and the core model is implemented;
- runtime high-assurance approval enforcement is not implemented;
- write-capable adapters are not implemented;
- RBAC/IdP/quorum approval are not implemented;
- hosted behavior is not implemented;
- side-effect execution is not implemented;
- schemas are not implemented;
- CLI behavior is not implemented;
- examples are not updated;
- Level 3/4 autonomy is not enabled.

During review, one roadmap wording correction changed "safety-critical prerequisite" to "safety-sensitive prerequisite" to avoid implying safety-critical certification.

## 13. Blockers

1. `HighAssuranceApprovalRequiredReference` must not derive unchecked `Deserialize`.

   Required fix:

   - implement constructor-backed deserialization for `HighAssuranceApprovalRequiredReference`;
   - ensure invalid or secret-like names fail closed;
   - ensure errors do not include raw names or secret-like values;
   - add focused regression tests for standalone nested deserialization.

## 14. Non-Blocking Follow-Ups

- Add explicit top-level deserialization non-leakage tests for secret-like redaction metadata.
- Decide in a later planning phase whether high-assurance controls should first be referenced from governance profiles, policy effects, workflow-local config, or executor-only inputs.
- Plan runtime enforcement only after the blocker fix review accepts the model.

## 15. Recommended Next Phase

Recommended next phase: **High-assurance approval control core model blocker fix**.

The fix should be narrow: close the nested deserialization validation bypass, add tests, rerun validation, and update the phase report/review with a fix-forward note. It should not add runtime enforcement, write-capable adapters, schemas, CLI behavior, examples, RBAC/IdP, quorum, hosted behavior, side-effect execution, or release posture changes.

## 16. Validation

Commands run during review:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed

Validation was run after merging the latest `origin/main` into the review branch.
