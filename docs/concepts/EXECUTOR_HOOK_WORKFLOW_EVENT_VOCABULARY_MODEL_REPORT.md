# Executor Hook Workflow Event Vocabulary Model Report

## 1. Executive Summary

The hook workflow event vocabulary model phase is implemented as a model-only runtime event foundation. Workflow OS can now represent future hook workflow events through bounded `HookInvocationRequested` and `HookInvocationEvaluated` event kinds and a validated `AgentHarnessHookWorkflowEvent` payload.

This phase does not append hook events from `LocalExecutor`, emit audit sink records, persist hook records, broaden hook checkpoints, run local checks, invoke adapters, execute commands, model side effects, add writes, or change release posture.

## 2. Scope Completed

- Added `HookInvocationRequested` and `HookInvocationEvaluated` runtime event kind names.
- Added `WorkflowRunEventKind` variants for boxed hook workflow event payloads.
- Added `AgentHarnessHookWorkflowEvent` and `AgentHarnessHookWorkflowEventDefinition`.
- Added validation for hook invocation identity, contract identity, hook kind, hook status, optional phase ID, reference counts, redaction metadata, and sensitivity.
- Added redaction-safe `Debug` output for hook workflow event payloads.
- Added serde validation so invalid serialized hook event payloads fail closed.
- Added state-preserving transition rules for hook events from `Running`.
- Required idempotency keys for hook workflow events.
- Kept hook events invalid from terminal states and before `Running`.
- Added focused runtime event tests and local executor non-regression coverage.

## 3. Scope Explicitly Not Completed

- No executor hook broadening.
- No `LocalExecutor` hook event append behavior.
- No audit sink emission.
- No hook persistence or hook audit store.
- No report artifact writing.
- No CLI hook commands or rendering.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No runtime hook config.
- No automatic local check execution.
- No default local check handler registration.
- No command execution.
- No adapter invocation.
- No external provider calls.
- No `EvidenceReference` creation or attachment.
- No approval request or approval decision creation.
- No approval evidence attachment.
- No reasoning lineage.
- No side-effect boundary implementation.
- No write-capable adapters.
- No recursive agents or agent swarms.
- No hosted or distributed runtime claims.
- No release posture changes.

## 4. Model Types Added

- `AgentHarnessHookWorkflowEvent`: validated, redaction-safe hook workflow event payload.
- `AgentHarnessHookWorkflowEventDefinition`: explicit construction input for hook workflow event payloads.
- `WorkflowRunEventKindName::HookInvocationRequested`.
- `WorkflowRunEventKindName::HookInvocationEvaluated`.
- `WorkflowRunEventKind::HookInvocationRequested`.
- `WorkflowRunEventKind::HookInvocationEvaluated`.

The payload stores only stable identifiers, hook status vocabulary, bounded optional context, reference counts, redaction metadata, and sensitivity. It does not store raw prompts, provider payloads, command output, parser output, spec contents, local check transcripts, environment values, credentials, tokens, or unbounded summaries.

## 5. Event Vocabulary Summary

`HookInvocationRequested` and `HookInvocationEvaluated` are the minimal vocabulary needed to represent future hook checkpoint lifecycle events without implying executor integration today.

The event payload can carry:

- hook invocation ID;
- hook contract ID and version;
- hook kind;
- hook invocation status;
- optional step ID;
- optional phase ID;
- optional correlation ID;
- bounded input and output reference counts;
- validated redaction metadata;
- sensitivity.

The model intentionally uses reference counts rather than raw references or payload lists in this phase.

## 6. Transition And Idempotency Summary

Hook workflow events are state-preserving from `Running` only. They do not move a run to completed, failed, canceled, waiting-for-approval, retrying, or escalated.

Terminal states reject hook workflow events. Pre-running states reject hook workflow events.

Hook workflow events require idempotency keys. This keeps the future event-producing path honest about replay and duplicate behavior before executor integration is considered.

## 7. Audit And Executor Boundary Summary

The executor boundary remains unchanged:

- `execute(...)` behavior is unchanged.
- `execute_with_report(...)` behavior is unchanged except for non-regression tests proving it does not append hook workflow events.
- `BeforeReport` remains report-path-only, in-memory-only, and non-mutating.

Audit sink emission remains unimplemented. Future audit projection from hook workflow events or dedicated hook audit sink behavior requires separate planning and review.

## 8. Redaction And Privacy Summary

Hook workflow event payload validation is fail-closed and redaction-aware:

- optional phase IDs are bounded and secret-like values are rejected;
- redaction metadata field names and reasons are bounded;
- secret-like redaction metadata is rejected;
- `Debug` output redacts IDs, optional context, redaction metadata, and sensitivity;
- deserialization routes through the same validation boundary;
- errors use stable codes and do not include raw caller-supplied values.

Forbidden payload classes remain out of the model: raw spec contents, command output, command transcripts, provider payloads, CI logs, Jira or GitHub raw bodies, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, and unbounded summaries.

## 9. Test Coverage Summary

Focused tests cover:

- hook event kind name serialization;
- valid bounded hook workflow event payload accessors;
- redaction-safe debug output;
- secret-like phase ID rejection without value leakage;
- secret-like redaction metadata rejection without value leakage;
- serialization non-leakage for forbidden raw payload markers;
- invalid serialized payload fail-closed behavior without value leakage;
- state-preserving hook events from `Running`;
- idempotency key requirements;
- terminal state rejection;
- pre-running state rejection;
- local executor non-regression proving no hook event append behavior.

## 10. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test runtime_events`: passed.
- `cargo test -p workflow-core --test local_executor`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Known Limitations

- Hook events are vocabulary only; no executor path appends them.
- There is no hook event audit projection.
- There is no hook audit store or persistence.
- `BeforeReport` remains post-terminal and non-mutating.
- Pre-terminal hook checkpoint ordering remains unimplemented.
- Failed-closed hook event semantics are not implemented.
- Hook event output references remain counts only.
- WorkReports still cite supplied hook invocation IDs, not hook workflow event IDs.

## 12. Recommended Next Phase

Recommended next phase: **hook workflow event vocabulary model review**.

The review should verify scope cleanliness, transition behavior, idempotency requirements, redaction/privacy posture, serde fail-closed behavior, local executor non-regression, and documentation honesty before any audit projection or executor event append planning begins.
