# SideEffect Approval Linkage Report

## 1. Executive Summary

The approval-side-effect linkage validation helper is implemented as a model-only, reference-only boundary.

The helper validates already-loaded `SideEffectRecord` values against an already-existing `WorkflowRun` event history. It proves only that SideEffect approval authority references resolve to approval request or decision events from the same immutable workflow run and that the decision kind matches the SideEffect authority posture.

It does not create approvals, create SideEffect records, append events, mutate workflow state, write artifacts, call adapters, execute side effects, enable writes, expose CLI behavior, change schemas, update examples, implement approval evidence attachment, implement reasoning lineage, or change release posture.

## 2. Scope Completed

Implemented:

- `SideEffectApprovalLinkageInput`;
- `SideEffectApprovalLinkageResult`;
- `validate_side_effect_approval_linkage(...)`;
- `workflow-core` exports for the helper and types;
- full immutable run identity validation between SideEffect records and the supplied run;
- approval request/decision lookup from workflow event history;
- authority-to-decision matching for `ApprovedByHuman`, `DeniedByApproval`, and `RequiresApproval`;
- step and skill identity checks where SideEffect records carry step/skill scope;
- bounded count result;
- redaction-safe `Debug` for input and result;
- focused tests for positive and failure cases;
- documentation status updates.

## 3. Scope Explicitly Not Completed

Not implemented:

- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable adapters;
- provider mutation;
- approval evidence attachment;
- approval packet model;
- multi-party approval;
- quorum approval;
- role-based approval authority;
- requester/approver separation enforcement;
- approval revocation;
- approval expiration enforcement changes;
- external identity provider integration;
- store-backed linkage;
- automatic report/artifact linkage validation;
- automatic SideEffect discovery;
- automatic report artifact writing;
- workflow schema fields;
- CLI behavior;
- examples;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The new helper accepts:

- a borrowed `WorkflowRun`;
- a borrowed slice of already-loaded `SideEffectRecord` values;
- explicit policy booleans for `RequiresApproval` references and approved/denied decision requirements.

The helper returns a bounded result with:

- SideEffect record count;
- approval reference count;
- linked approval reference count;
- duplicate approval reference count.

The helper does not read hidden global state, construct a backend, read stores, write stores, call executor methods, append workflow events, emit audit events, or write report artifacts.

## 5. Validation Boundary Summary

Validation checks:

- the supplied run can be rehydrated from its event history;
- the supplied run snapshot matches the rehydrated snapshot;
- each SideEffect record validates;
- each SideEffect record matches the run's workflow ID, workflow version, schema version, spec hash, and run ID;
- approval references resolve to approval events in the same run;
- `ApprovedByHuman` references a granted approval decision;
- `DeniedByApproval` references a denied approval decision;
- `RequiresApproval` references an approval request without treating it as granted;
- step ID matches the approval request where SideEffect step scope is present;
- skill ID and skill version match the approval request where SideEffect skill scope is present;
- duplicate approval references across records are counted deterministically.

The helper intentionally does not validate role authority, approver identity separation, evidence sufficiency, approval expiration, external identity, provider state, or side-effect execution outcome.

## 6. Error Handling Summary

The helper uses stable non-leaking error codes:

- `side_effect_approval_linkage.identity_mismatch`
- `side_effect_approval_linkage.approval_missing`
- `side_effect_approval_linkage.decision_missing`
- `side_effect_approval_linkage.decision_kind_mismatch`
- `side_effect_approval_linkage.step_mismatch`
- `side_effect_approval_linkage.skill_mismatch`
- `side_effect_approval_linkage.record_invalid`
- `side_effect_approval_linkage.run_invalid`

Errors do not include SideEffect IDs, approval IDs, workflow IDs, run IDs, step IDs, skill IDs, spec hashes, target references, summaries, reasons, redaction metadata, provider payloads, command output, parser payloads, paths, snippets, credentials, tokens, authorization headers, or private keys.

## 7. Redaction And Privacy Summary

The helper is reference-only.

It inspects:

- typed SideEffect authority references;
- authority decisions;
- approval event IDs and decision kinds;
- immutable run identity;
- optional step/skill scope.

It does not copy approval reasons, SideEffect summaries, target references, raw provider payloads, raw CI logs, Jira/GitHub bodies, command output, spec contents, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, or local paths.

`Debug` output for input and result exposes only counts, booleans, and redacted placeholders.

## 8. Test Coverage Summary

Added focused tests for:

- `ApprovedByHuman` with matching granted approval;
- `DeniedByApproval` with matching denied approval;
- `RequiresApproval` with matching request and no decision;
- missing required approval reference;
- missing approval decision;
- granted/denied decision mismatch;
- SideEffect/run identity mismatch;
- step mismatch;
- duplicate approval references across records;
- records without approval authority;
- redaction-safe input/result `Debug` output.

Existing SideEffect model tests still pass.

## 9. Commands Run And Results

Commands run:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed
- `cargo test -p workflow-core --test side_effect` - passed

## 10. Remaining Known Limitations

- Linkage is over already-loaded records only. Store-backed linkage remains deferred.
- Linkage is not automatically run during report generation, report artifact writing, executor execution, or SideEffect discovery.
- The current approval reference shape uses the local runtime approval ID as the stable reference text.
- Role-based approval authority, separation of requester and approver, quorum approval, and external identity provider checks remain future high-assurance approval work.
- Approval expiration and revocation semantics are not changed.
- Successful linkage does not imply side-effect execution, provider mutation, evidence completeness, artifact persistence, or write safety.

## 11. Recommended Next Phase

Recommended next phase: **SideEffect approval linkage validation helper review**.

The helper should be reviewed before store-backed linkage, report/artifact composition, approval evidence attachment, runtime side-effect execution, or write-capable adapter planning.
