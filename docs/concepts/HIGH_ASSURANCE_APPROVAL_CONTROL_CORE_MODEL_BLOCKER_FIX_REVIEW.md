# High-Assurance Approval Control Core Model Blocker Fix Review

## 1. Executive Verdict

Blocker fixed with non-blocking follow-ups.

The fix closes the public nested deserialization bypass identified in [High-Assurance Approval Control Core Model Review](HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_REVIEW.md). `HighAssuranceApprovalRequiredReference` now deserializes through its validated constructor, so standalone serialized reference requirements cannot silently carry invalid or secret-like names.

The phase stayed within blocker-fix scope. It did not add runtime high-assurance approval enforcement, write-capable adapters, provider mutations, RBAC, IdP integration, quorum approval, workflow schemas, CLI behavior, examples, hosted behavior, reasoning lineage, side-effect execution, or release posture changes.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

Implemented scope:

- constructor-backed deserialization for `HighAssuranceApprovalRequiredReference`;
- focused regression tests for standalone required-reference serde;
- invalid serialized required-reference name rejection;
- secret-like serialized required-reference name rejection;
- blocker-fix report;
- fix-forward note in the original phase review;
- roadmap link to the blocker-fix report.

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
- release posture changes;
- broad high-assurance approval model redesign.

## 3. Original Blocker Restatement

The original blocker was that `HighAssuranceApprovalRequiredReference` derived `Deserialize` directly while also exposing a validated constructor.

That meant the top-level `HighAssuranceApprovalControl` path was safe, but callers could deserialize the exported nested type directly and bypass `HighAssuranceApprovalRequiredReference::new(...)`. Invalid or secret-like `name` values could exist in a public model value until later embedded in a top-level validated control.

Because the nested type is part of the public model surface, it needed its own fail-closed deserialization boundary.

## 4. Fix Approach Assessment

The fix removes direct derived `Deserialize` from `HighAssuranceApprovalRequiredReference`.

The implementation adds a small internal wire struct inside a manual `Deserialize` implementation and then calls `HighAssuranceApprovalRequiredReference::new(...)`. This preserves the serialized shape while reusing the existing validation path.

Assessment:

- minimal: no model redesign or unrelated refactor;
- idiomatic: serde wire struct plus constructor validation matches existing repository patterns;
- compatible: valid serialized references still round trip;
- future-safe: schema exposure can later rely on this type boundary without inheriting the bypass.

## 5. Validation Boundary Assessment

Verified behavior:

- required-reference names remain bounded by the existing identifier rules;
- invalid characters are rejected during standalone deserialization;
- secret-like names are rejected during standalone deserialization;
- target references still deserialize through existing target primitives;
- invalid serialized references fail closed;
- valid serialized references still round trip;
- top-level control deserialization continues to validate through `HighAssuranceApprovalControl::new(...)`.

Validation errors remain stable and non-leaking. The new tests assert that rejected raw names such as `bad name` and `authorization_token` do not appear in the deserialization error text.

## 6. Debug And Serialization Assessment

Verified:

- `HighAssuranceApprovalRequiredReference` `Debug` continues to redact the name and target details;
- target `Debug` continues to report only a stable kind label and redacted reference;
- valid references serialize with the same shape as before;
- invalid serialized references do not silently enter the model;
- secret-like serialized reference names do not silently enter the model;
- top-level high-assurance control `Debug` remains redaction-safe.

The fix intentionally does not redact valid stable reference IDs from serialization, because these are model values. It prevents invalid and secret-like names from being accepted through deserialization.

## 7. Privacy And Redaction Assessment

No evidence was found that the fix stores or copies:

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

The direct leakage risk identified by the blocker is closed: secret-like required-reference names cannot be accepted through standalone deserialization, and error text does not include the rejected secret-like value.

## 8. Regression Assessment

Unchanged behavior was verified for:

- valid high-assurance approval control construction;
- valid required-reference construction;
- valid required-reference serialization and deserialization;
- valid high-assurance approval control serialization and deserialization;
- target reference vocabulary;
- future write and side-effect vocabulary as model-only terms;
- requester/approver separation vocabulary;
- expiration and revocation vocabulary;
- report disclosure vocabulary;
- redaction metadata rejection and non-leakage;
- existing WorkReport, SideEffect, EvidenceReference, Diagnostic, validation, adapter, and runtime tests through the workspace suite.

## 9. Test Quality Assessment

The added tests cover the exact blocker:

- standalone required-reference serde round trip uses the validated shape;
- invalid serialized required-reference names fail closed without leaking;
- secret-like serialized required-reference names fail closed without leaking.

The existing test suite continues to cover:

- valid minimal high-assurance approval control;
- invalid control ID;
- invalid version;
- secret-like schema version rejection;
- empty protected actions;
- duplicate protected actions;
- zero approval count;
- empty required references;
- duplicate required reference names;
- all required reference target vocabulary;
- future write vocabulary as model-only;
- requester/approver separation vocabulary;
- expiration and revocation vocabulary;
- report disclosure validation;
- top-level serde round trip;
- invalid top-level serde failure;
- secret-like redaction metadata rejection;
- `Debug` non-leakage;
- serialization avoiding forbidden raw payload markers.

No blocker-level test gaps remain.

## 10. Documentation Review

Documentation now says:

- the high-assurance approval control core model is implemented;
- the original model review found a nested required-reference deserialization blocker;
- the blocker is fixed in the blocker-fix report;
- the fix remains model-only;
- runtime high-assurance approval enforcement is not implemented;
- write-capable adapters are not implemented;
- RBAC/IdP/quorum approval are not implemented;
- hosted behavior is not implemented;
- side-effect execution is not implemented;
- schemas are not implemented;
- CLI behavior is not implemented;
- examples are not updated;
- Level 3/4 autonomy is not enabled.

The original review was not rewritten to hide the blocker; it includes a fix-forward note for auditability.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add an explicit top-level deserialization non-leakage test for secret-like redaction metadata values.
- Decide whether high-assurance controls should first be referenced from governance profiles, policy effects, workflow-local config, or executor-only inputs.
- Plan opt-in runtime enforcement for requester/approver separation, required references, expiration, revocation, and report disclosure before write-capable adapters.

## 13. Recommended Next Phase

Recommended next phase: **high-assurance approval runtime enforcement planning, opt-in only**.

The blocker is fixed, so the next useful work is not another model family. The next phase should plan how existing approval events, policy decisions, evidence references, side-effect records, and WorkReports compose into a narrow runtime enforcement path. It should remain local, opt-in, and explicit, and it must still avoid write-capable adapters until enforcement semantics are reviewed.

## 14. Validation

Commands run during blocker-fix review:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test -p workflow-core --test high_assurance_approval` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
