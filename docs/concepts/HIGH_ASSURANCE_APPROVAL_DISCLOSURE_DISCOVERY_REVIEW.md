# High-Assurance Approval Disclosure Discovery Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a narrow pure in-memory helper for deriving report-safe high-assurance approval disclosure from explicit bounded validation posture. It preserves the source-of-truth boundary, avoids runtime inference, and does not broaden into executor automation, event scanning, report artifact gates, schemas, CLI behavior, writes, or hosted behavior.

## 2. Scope Verification

The phase stayed within the approved discovery-helper scope.

Implemented scope:

- `HighAssuranceApprovalDisclosureDiscoveryInput`;
- `HighAssuranceApprovalDisclosureDiscoveryResult`;
- `HighAssuranceApprovalDisclosureNotAvailableReason`;
- `discover_high_assurance_approval_disclosure(...)`;
- mapping for passed grant and denial posture;
- bounded not-available output when validation was not used;
- failed-validation disclosure without approval decision claims;
- fail-closed rejection for inconsistent validation posture;
- fail-closed rejection for unsupported passed posture;
- count bounds for control and reference counts;
- redaction-safe `Debug`;
- focused tests and documentation.

No accidental implementation was found for:

- automatic executor/report integration;
- changes to `LocalExecutor::decide_approval(...)`;
- changes to `LocalExecutor::decide_approval_with_high_assurance(...)` semantics;
- workflow event scanning;
- new workflow events;
- audit projection;
- report artifact gates;
- workflow-declared high-assurance controls;
- runtime config;
- workflow schema fields;
- CLI behavior;
- examples;
- approval evidence attachment;
- stable high-assurance validation result IDs;
- RBAC, IdP, SSO, SCIM, team, group, or directory integration;
- quorum or multi-party approval;
- revocation enforcement;
- side-effect execution;
- write-capable adapters;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately small and explicit.

`HighAssuranceApprovalDisclosureDiscoveryInput` carries only bounded posture:

- whether validation was used;
- whether validation passed;
- decision kind;
- control count;
- requester/approver rule posture;
- required and supplied reference counts;
- expiration policy posture;
- revocation policy posture;
- denial behavior posture.

It does not accept approval payloads, actor IDs, control payloads, provider data, command output, raw evidence, or free-form approval text.

`HighAssuranceApprovalDisclosureDiscoveryResult` returns either a report-safe `WorkReportHighAssuranceApprovalDisclosure` or a bounded not-available reason. Its `Debug` output reports only whether a disclosure exists and the not-available reason code.

## 4. Source-Of-Truth Assessment

The implementation correctly avoids false source-of-truth claims.

The helper does not prove that a runtime executor path used high-assurance validation. It only derives disclosure from explicit caller-supplied validation posture. This is the right first boundary because existing approval events do not encode high-assurance validation posture, and inferring from event history would risk decorative governance.

The docs and phase report are honest that future executor/report composition still needs a reviewed source-of-truth path before automatic disclosure or artifact gates can rely on this helper.

## 5. Disclosure Mapping Assessment

The mapping is appropriately conservative.

Validated passed grant maps to `Granted`.

Validated passed denial maps to `Denied`.

Failed validation maps decision, requester/approver posture, expiration posture, and revocation posture to `NotAvailable`, preventing a failed validation from implying an approval decision was safely recorded.

`HumanApproverMustDiffer` maps to `HumanApproverDeferred`, which avoids overclaiming actual human identity or directory-backed authority in the local v0 model.

Unsupported use-time expiration, revocation enforcement, and non-fail-closed denial behavior are rejected when the input claims validation passed. This fail-closed posture is important because these semantics are not implemented yet.

## 6. Validation Assessment

Validation ensures:

- validation cannot pass when validation was not used;
- validation-used input must include at least one control;
- control count is bounded;
- required-reference count is bounded;
- supplied-reference count is bounded;
- unsupported use-time expiration cannot be marked passed;
- unsupported revocation posture cannot be marked passed;
- unsupported denial behavior cannot be marked passed;
- errors use stable codes;
- errors do not include raw IDs, actor values, paths, snippets, payloads, tokens, or secret-like values.

The helper also delegates final disclosure construction to `WorkReportHighAssuranceApprovalDisclosure::new(...)`, preserving the reviewed WorkReport validation boundary.

## 7. Privacy And Redaction Assessment

The discovery helper is redaction-safe.

Verified behavior:

- no raw approval requests are stored;
- no raw approval decisions are stored;
- no actor IDs are stored;
- no high-assurance control payloads are stored;
- no evidence payloads are stored;
- no policy payloads are stored;
- no provider payloads are stored;
- no command output is stored;
- no spec contents are stored;
- no parser payloads are stored;
- no source snippets are stored;
- no environment variable values are stored;
- no credentials, authorization headers, private keys, token-like values, or secret-like strings are stored;
- `Debug` output is bounded and posture-only;
- validation errors are stable and non-leaking.

The helper serializes only the bounded not-available reason enum. The disclosure itself continues to use the already-reviewed WorkReport disclosure serialization boundary.

## 8. Relationship To Executor And Reports

The helper remains separate from executor mutation and report generation.

Existing explicit report input propagation remains unchanged: reports can include high-assurance approval disclosure only when the caller supplies a validated disclosure. This helper gives future executor/report composition a deterministic derivation boundary, but it does not automatically attach disclosure to executor results.

That separation is correct. Automatic integration should wait until the project decides whether the source of truth is an executor-adjacent result, a stable validation result ID, workflow event vocabulary, audit projection, or an artifact gate input.

## 9. Test Quality Assessment

Tests cover:

- successful grant disclosure mapping;
- denied disclosure mapping;
- deferred human-approver posture;
- validation-not-used not-available result;
- failed validation disclosure without decision claims;
- inconsistent validation posture rejection;
- unsupported passed posture rejection for expiration, revocation, and denial behavior;
- unbounded count rejection;
- bounded non-leaking `Debug` output;
- existing high-assurance approval, WorkReport, executor, side-effect, validation, and runtime regression coverage through the workspace suite.

Minor non-blocking gaps:

- add an integration-style test that feeds the real high-assurance validation result counts into the discovery helper;
- add explicit coverage that zero controls are rejected;
- add explicit coverage for `SameActorAllowed`, `RequiredOnRequest`, and `MustBeUnexpiredAtDecision` success posture;
- add future executor-adjacent tests only after automatic disclosure source-of-truth integration is planned.

## 10. Documentation Review

Documentation now states that:

- the first pure high-assurance approval disclosure discovery helper is implemented;
- the helper is in-memory and explicit-input only;
- automatic executor/report integration is not implemented;
- workflow event scanning is not implemented;
- report artifact gates are not implemented;
- workflow-declared high-assurance controls are not implemented;
- stable high-assurance validation result IDs are not implemented;
- RBAC/IdP/quorum/revocation enforcement is not implemented;
- write-capable adapters are not implemented;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, side-effect execution, and release posture changes remain unsupported.

The end-of-phase report is present at [High-Assurance Approval Disclosure Discovery Report](HIGH_ASSURANCE_APPROVAL_DISCLOSURE_DISCOVERY_REPORT.md).

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add a direct zero-control rejection test.
- Add an integration-style test from `validate_high_assurance_approval_decision(...)` result counts into `discover_high_assurance_approval_disclosure(...)`.
- Add success coverage for additional supported posture variants.
- Plan executor-adjacent high-assurance disclosure integration before any automatic report or artifact gate depends on this helper.
- Keep workflow-declared controls, stable validation result IDs, and event/audit projection deferred until explicitly planned.

## 13. Recommended Next Phase

Recommended next phase: high-assurance approval disclosure executor/report integration planning.

Reason: the pure derivation helper is now implemented and reviewed. The next unresolved boundary is deciding how an executor path should carry the result of high-assurance approval validation into report inputs without event inference, payload copying, or workflow semantic changes.

Report artifact gates should remain later because they need a reviewed integration source before they can safely require high-assurance disclosure.

## 14. Validation

Governed review run:

- workflow: `dg/review`
- run ID: `run-1783057232213918000-2`
- approval ID: `approval/run-1783057232213918000-2/review-scope-approved`
- approval outcome: granted
- status: completed

Validation commands run:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
