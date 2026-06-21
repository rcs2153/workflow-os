# SideEffect Approval Linkage Review

Review date: 2026-06-21

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The SideEffect approval linkage validation helper stays within the approved validation-only, reference-only scope. It validates already-loaded `SideEffectRecord` values against an existing `WorkflowRun` event history, checks immutable run identity, resolves approval references, verifies granted/denied decision kind alignment, returns bounded counts, and keeps runtime side-effect execution, writes, provider mutation, schemas, CLI behavior, report artifact writing, and automatic linkage enforcement out of scope.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- `SideEffectApprovalLinkageInput`
- `SideEffectApprovalLinkageResult`
- `validate_side_effect_approval_linkage(...)`
- `workflow-core` exports for the helper and types
- immutable run identity validation between `SideEffectRecord` values and the supplied `WorkflowRun`
- approval request and decision lookup from workflow event history
- authority-to-decision matching for `ApprovedByHuman`, `DeniedByApproval`, and `RequiresApproval`
- optional step and skill identity checks when SideEffect records carry that scope
- bounded count result and redaction-safe `Debug`
- focused positive, failure, duplicate-reference, and non-leakage tests
- roadmap, concept, implementation-plan, and end-of-phase report updates

No accidental implementation was found for:

- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable adapters;
- provider mutation;
- approval evidence attachment;
- approval packet modeling;
- multi-party approval, quorum approval, or role-based approval enforcement;
- requester/approver separation enforcement;
- approval revocation or expiration behavior changes;
- external identity provider integration;
- store-backed linkage;
- automatic report or artifact linkage validation;
- automatic SideEffect discovery;
- automatic report artifact writing;
- workflow schema fields;
- CLI behavior;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately narrow and testable:

```rust
pub fn validate_side_effect_approval_linkage(
    input: SideEffectApprovalLinkageInput<'_>,
) -> Result<SideEffectApprovalLinkageResult, WorkflowOsError>
```

The input accepts a borrowed `WorkflowRun`, borrowed `SideEffectRecord` slice, and two explicit policy booleans. It does not construct a backend, read hidden global state, call executor methods, require a store, call adapters, append events, emit audit records, write artifacts, or execute side effects.

The result exposes only counts:

- inspected SideEffect records;
- approval references;
- linked approval references;
- duplicate approval references.

This is the right first boundary before store-backed linkage or report/artifact composition.

## 4. Linkage Semantics Assessment

The helper validates approval authority references only from `SideEffectRecord.authority.approval_references`.

Positive behavior is correct:

- `ApprovedByHuman` succeeds only when the referenced approval decision is granted.
- `DeniedByApproval` succeeds only when the referenced approval decision is denied.
- `RequiresApproval` can link to an approval request without treating it as granted.
- records without approval authority are accepted without fabricating approval linkage.
- duplicate approval references are counted deterministically.

The helper correctly treats approval as authority context, not lifecycle state. It does not introduce an `Approved` lifecycle state and does not claim that approval proves provider mutation or side-effect execution.

One non-blocking API clarity point remains: `require_decision_for_approved_or_denied` controls whether missing approval references fail for approved/denied authority, but once a reference is supplied the helper still requires a matching decision. That is defensible because `ApprovedByHuman` and `DeniedByApproval` should not link to a request-only approval. The boolean name and docs should be clarified or additional tests should lock the intended permissive behavior before external callers depend on it.

## 5. Identity Validation Assessment

The helper validates full immutable SideEffect/run identity:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID.

It also rehydrates the supplied run from its event history and compares the rehydrated snapshot to the supplied snapshot before validating linkage. That avoids trusting a mutated in-memory snapshot when the event history disagrees.

Approval request payload identity is validated against the run identity. Decision events are resolved through the event history and decision kind. Step ID, skill ID, and skill version are checked when SideEffect records carry those fields.

The helper does not use approval ID strings alone as proof, which is the important governance boundary.

## 6. Error Handling Assessment

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

Invalid SideEffect records and invalid runs are mapped to the linkage boundary instead of propagating lower-level details. That is the right privacy posture.

## 7. Privacy And Redaction Assessment

The helper is reference-only.

It inspects:

- typed approval references;
- authority decisions;
- approval IDs only as bounded reference keys;
- approval decision kind;
- immutable run identity;
- optional step and skill scope.

It does not copy approval reasons, SideEffect summaries, SideEffect target references, raw provider payloads, raw CI logs, Jira/GitHub bodies, command output, spec contents, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, or local paths into errors, results, reports, artifacts, or Debug output.

`SideEffectApprovalLinkageInput` `Debug` redacts the run and exposes only record count plus policy booleans. `SideEffectApprovalLinkageResult` `Debug` exposes counts only.

## 8. Runtime And State Semantics Assessment

The phase preserves existing runtime and state semantics.

No code path was added that:

- mutates `WorkflowRun`;
- mutates `WorkflowRunSnapshot`;
- appends workflow events;
- emits audit events;
- emits observability events;
- reads or writes `SideEffectRecordStore`;
- writes report artifacts;
- calls adapters;
- calls external systems;
- writes files;
- exposes CLI output.

The helper can be composed later without changing current executor behavior.

## 9. Relationship To Reports, Artifacts, And Discovery

The helper correctly stays separate from report generation, report artifact writes, artifact integrity validation, and SideEffect discovery.

Discovery answers which SideEffects should be cited. Artifact referential integrity answers whether cited SideEffect IDs resolve to matching records. Approval linkage answers whether a SideEffect authority claim is backed by approval events from the same immutable run.

Keeping these helpers separate is a healthy design. The next composition phase should be explicit and reviewed before any automatic report/artifact validation or write-capable adapter work.

## 10. Test Quality Assessment

Tests cover:

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
- input/result Debug non-leakage;
- existing SideEffect model tests;
- existing workspace tests.

Non-blocking gaps:

- Add a direct test for skill ID or skill version mismatch. The code path exists and shares the step mismatch shape, but direct coverage would make the helper easier to maintain.
- Add a test for an approval reference with a non-`ApprovalDecision` reference kind in `approval_references`.
- Add tests for approval request identity mismatch and invalid run snapshot/event mismatch. The implementation handles both, but direct regression tests would be useful before composition.
- Add tests that document `require_decision_for_approved_or_denied = false` and `require_approval_references_for_requires_approval = false` behavior.
- Consider a duplicate-reference expectation that makes clear whether duplicates are only counted or also de-duplicated for downstream work. The current helper has no store read, so validating duplicates twice is harmless.

No blocker-level test gaps were found.

## 11. Documentation Review

Documentation now states:

- approval-side-effect linkage planning is documented;
- the validation-only helper is implemented;
- the helper is reference-only and uses already-loaded records plus an existing run;
- automatic approval-side-effect validation in report/artifact paths is not implemented;
- store-backed linkage is not implemented;
- approval evidence attachment is not implemented;
- runtime side-effect execution is not implemented;
- attempted/completed/failed executor behavior is not implemented;
- write-capable adapters and provider mutations are not implemented;
- workflow schema fields, CLI behavior, examples, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

The end-of-phase report accurately summarizes scope, API, validation boundary, error handling, privacy posture, tests, commands, limitations, and the review recommendation.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Clarify or test the exact semantics of `require_decision_for_approved_or_denied = false`.
- Add direct skill ID and skill version mismatch tests.
- Add a direct non-approval-reference-kind test for `approval_references`.
- Add direct invalid-run and approval-request identity mismatch tests.
- Consider store-backed approval linkage only after a caller needs it.
- Plan any report/artifact composition explicitly rather than silently running linkage from existing artifact or executor paths.

## 14. Recommended Next Phase

Recommended next phase: **approval-side-effect linkage composition planning**.

The validation-only helper is accepted. The next useful roadmap step is to decide where, when, and how this helper should be composed with store-backed records, report artifact integrity, or report generation without making linkage automatic by accident. Runtime side-effect execution and write-capable adapters should still wait until that composition boundary is planned and reviewed.

## 15. Validation

Commands run:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed
