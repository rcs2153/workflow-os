# Approval Gate Presentation Opt-In Enforcement Review

## 1. Executive Verdict

Phase accepted; proceed to dogfood runner approval-presentation persistence
planning.

The implementation adds a narrow, explicit opt-in enforcement path for local
approval decisions while preserving default approval behavior. It validates
durable presentation proof before appending approval events and includes focused
regression tests for fail-closed behavior.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- explicit opt-in approval decision path;
- durable proof lookup by presentation ID;
- durable proof lookup by run ID and approval ID when unambiguous;
- proof matching against the pending approval request;
- optional freshness/staleness validation;
- stable non-leaking errors;
- focused tests;
- documentation and implementation report updates.

Not introduced:

- default approval behavior changes;
- automatic approvals;
- hidden approvals;
- dogfood runner proof persistence;
- CLI approval-card rendering;
- workflow schema fields;
- examples;
- high-assurance approval integration;
- WorkReport citation changes;
- provider writes;
- side effects;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 3. API Assessment

The API is appropriately narrow:

- `LocalApprovalPresentationProof`;
- `LocalApprovalPresentationDecisionRequest`;
- `LocalExecutor::decide_approval_with_presentation(...)`.

The request wraps the existing `LocalApprovalDecisionRequest` rather than
duplicating approval fields. That keeps existing approval semantics centralized
and makes the enforcement path reviewable.

The request Debug implementation redacts the nested approval request and does
not expose approval reason, approval ID, run ID, or presentation ID.

## 4. Runtime Semantics Assessment

The implementation reuses the existing approval transition boundary:

1. `prepare_approval_decision(...)` loads and validates the pending approval
   without mutating runtime state.
2. The opt-in path resolves and validates presentation proof.
3. `apply_approval_decision(...)` appends the existing approval decision events
   only after proof validation succeeds.

This is the right sequencing. Proof failures happen before approval decision
events, `RunResumed`, downstream skill invocation, side-effect events, report
artifacts, or other runtime mutation.

Existing `LocalExecutor::decide_approval(...)` remains unchanged.

## 5. Proof Resolution Assessment

The phase implements both planned resolution modes:

- explicit presentation ID lookup;
- run/approval lookup when exactly one matching record exists.

The ambiguity behavior is correct:

- zero records fails closed as missing proof;
- one record proceeds to validation;
- multiple records fail closed as ambiguous proof.

Store read/list failures are mapped to a stable corrupt-proof enforcement code
without leaking stored payloads or identifiers.

## 6. Proof Validation Assessment

The enforcement path validates:

- run ID;
- approval ID;
- workflow ID;
- workflow version;
- schema version;
- step ID;
- presentation timestamp before approval decision timestamp;
- optional max-age freshness.

The implementation delegates identity matching to
`validate_approval_presentation_for_request(...)`, preserving the already
reviewed model/helper validation boundary.

## 7. Error Handling Assessment

Error handling is stable and non-leaking.

Implemented enforcement codes:

- `approval_presentation_enforcement.proof_missing`;
- `approval_presentation_enforcement.proof_ambiguous`;
- `approval_presentation_enforcement.proof_mismatch`;
- `approval_presentation_enforcement.proof_stale`;
- `approval_presentation_enforcement.proof_corrupt`;
- `approval_presentation_enforcement.decision_time_invalid`.

The tests verify non-leakage for missing proof. The code paths use fixed
messages and do not interpolate approval IDs, presentation IDs, run IDs,
handoff text, actor values, paths, raw payloads, command output, provider
payloads, or token-like values.

## 8. Privacy And Redaction Assessment

The implementation does not introduce raw approval-card storage in executor
state. It consumes already validated `ApprovalPresentationRecord` values from
the state backend and never copies raw provider payloads, command output, spec
contents, token-like values, or unbounded handoff text into errors.

Debug output for the new request is redaction-safe and bounded.

## 9. Test Quality Assessment

Focused tests cover:

- matching proof grant resumes the run;
- matching proof denial fails the run closed;
- missing proof fails before approval events are appended;
- missing proof keeps the run waiting for approval;
- mismatched proof fails closed;
- ambiguous run/approval proof lookup fails closed;
- future-dated proof fails closed;
- stale proof fails closed when a max age is supplied;
- request Debug output does not leak approval reason, approval ID, or
  presentation ID.

Existing approval, high-assurance approval, state, runtime, work report,
adapter, validation, and CLI tests continue to pass through the workspace test
suite.

Non-blocking test improvement: add a direct corrupt-proof executor-path test
that exercises the `proof_corrupt` mapping through a deliberately corrupted
store record. The lower-level store already tests corrupt record behavior, so
this is not a blocker.

## 10. Documentation Review

Docs now state that:

- opt-in approval-presentation enforcement is implemented;
- default approval behavior remains unchanged;
- dogfood runner persistence of presented handoff proof is not implemented;
- CLI approval-card rendering is not implemented;
- high-assurance integration is not implemented;
- WorkReport citation changes are not implemented;
- provider writes, side effects, hosted behavior, schemas, examples, reasoning
  lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release
  posture changes remain unsupported.

## 11. Validation

Implementation validation was run before merge and is recorded in
[Approval Gate Presentation Opt-In Enforcement Implementation Report](APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_IMPLEMENTATION_REPORT.md):

- `cargo fmt --all --check` - passed;
- `cargo test -p workflow-core --test local_executor presentation` - passed;
- `cargo test -p workflow-core --test approval_presentation` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed.

Review validation:

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test -p workflow-core --test local_executor presentation` - passed;
- `cargo test -p workflow-core --test approval_presentation` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Add an executor-path corrupt-proof regression test.
- Plan dogfood runner persistence of the emitted approval handoff as an
  `ApprovalPresentationRecord`.
- Plan how dogfood phase approvals should opt into
  `decide_approval_with_presentation(...)` once the runner can persist proof.
- Later, plan WorkReport citation of approval-presentation proof by stable
  reference.

## 14. Recommended Next Phase

Recommended next phase: dogfood runner approval-presentation persistence
planning.

The opt-in executor gate is now available, but the repo-local dogfood runner
still emits approval handoffs without persisting them as proof. The next phase
should plan how the runner creates durable presentation records from the exact
handoff block before approval decisions are submitted.
