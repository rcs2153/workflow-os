# Executor Hook Workflow Event Vocabulary Model Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted; proceed to hook event audit projection planning.

The implementation delivers the intended model-only hook workflow event vocabulary. It adds bounded `HookInvocationRequested` and `HookInvocationEvaluated` event vocabulary, a validated `AgentHarnessHookWorkflowEvent` payload, state-preserving transition behavior from `Running`, idempotency requirements, serde validation, and focused tests. It does not wire hook events into `LocalExecutor`, emit audit sink records, persist hook records, broaden hook checkpoints, run local checks, invoke adapters, execute commands, model side effects, add writes, or change release posture.

## 2. Scope Verification

The phase stayed within the approved model-only scope.

No accidental implementation was found for:

- executor hook broadening;
- automatic executor hook invocation;
- `LocalExecutor` hook event append behavior;
- audit sink emission;
- hook persistence;
- hook audit store;
- report artifact writes;
- CLI hook commands or rendering;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime hook configuration;
- automatic local check execution;
- default local check handler registration;
- command execution;
- adapter invocation;
- external provider calls;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary implementation;
- write-capable adapters;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 3. Model Assessment

The model is appropriately small and domain-aligned.

Implemented model surface:

- `WorkflowRunEventKindName::HookInvocationRequested`;
- `WorkflowRunEventKindName::HookInvocationEvaluated`;
- `WorkflowRunEventKind::HookInvocationRequested`;
- `WorkflowRunEventKind::HookInvocationEvaluated`;
- `AgentHarnessHookWorkflowEvent`;
- `AgentHarnessHookWorkflowEventDefinition`.

The payload is reference-first and bounded. It stores hook invocation identity, hook contract identity, hook kind, hook invocation status, optional step/phase/correlation context, bounded input/output reference counts, redaction metadata, and sensitivity. It does not store raw prompts, raw command output, provider payloads, parser payloads, local check transcripts, spec contents, environment values, credentials, tokens, or unbounded summaries.

## 4. Event Vocabulary Assessment

The selected vocabulary is the smallest useful slice from the accepted plan.

`HookInvocationRequested` and `HookInvocationEvaluated` are sufficient to represent a future checkpoint lifecycle without adding the higher-authority failure/blocking vocabulary yet. The implementation appropriately defers `HookInvocationFailedClosed`, `HookInvocationSkipped`, and `HookInvocationBlocked` because those imply additional failure, pause, escalation, or report-disclosure semantics.

The event names are model vocabulary only. They do not imply that hooks currently execute automatically or that workflow event history currently includes hook events from executor paths.

## 5. Transition And Idempotency Assessment

The transition behavior is conservative.

Hook workflow events are accepted only as state-preserving events from `Running`. They do not transition a run to completed, failed, canceled, waiting for approval, retrying, waiting for external event, or escalated.

Terminal states reject hook events. Pre-running states reject hook events. This matches the plan's requirement not to add post-terminal metadata events for the current `BeforeReport` checkpoint.

Hook workflow events require idempotency keys through the same runtime event validation path used for other meaningful idempotency-sensitive events. That is the right default before any event-producing executor integration is considered.

## 6. Executor Boundary Assessment

The executor boundary remains clean.

The implementation does not add hook event append behavior to `LocalExecutor`. Existing `execute(...)` and `execute_with_report(...)` behavior remains unchanged. Focused local executor coverage verifies that explicit `BeforeReport` report-path hook execution does not append `HookInvocationRequested` or `HookInvocationEvaluated` events.

The current `BeforeReport` checkpoint remains report-path-only, in-memory-only, and non-mutating.

## 7. Audit And Observability Boundary Assessment

No audit sink emission or observability event emission was introduced.

The audit and observability projection helpers remain passive. They can still be called by existing append paths, but no executor path appends hook events or emits hook audit sink records. This is acceptable for the model-only phase.

Non-blocking follow-up: the next planning phase should explicitly decide whether future hook workflow events project through `AuditEvent::from_workflow_event(...)`, use a dedicated hook audit sink, or remain linked through a separate hook audit store.

## 8. Privacy And Redaction Assessment

The privacy posture is acceptable for this phase.

Validation enforces:

- bounded optional phase IDs;
- bounded input/output reference counts;
- bounded redaction metadata field count and field-state count;
- bounded redaction field names;
- bounded redaction reasons;
- secret-like rejection for phase IDs, redaction field names, and redaction reasons;
- stable non-leaking error codes/messages.

`AgentHarnessHookWorkflowEvent` implements redaction-safe `Debug` and does not print hook IDs, contract IDs, phase IDs, correlation IDs, redaction metadata values, or raw context.

Serialization can include validated stable IDs and bounded validated redaction metadata, which is consistent with this model's current policy. Secret-like serialized values fail closed on deserialization through the constructor boundary.

## 9. Serde And Compatibility Assessment

Serde behavior is appropriate.

Valid payloads serialize and deserialize through the validated model. Invalid serialized payloads fail closed through custom deserialization that calls `AgentHarnessHookWorkflowEvent::new(...)`.

The event kind names serialize as stable PascalCase enum names consistent with existing runtime event kind naming. No workflow schema fields or public spec schema changes were introduced.

Compatibility note: adding new `WorkflowRunEventKind` variants expands the Rust runtime event vocabulary. Because no executor path emits these events and no schema exposure was added, the compatibility impact is bounded to the internal Rust model phase.

## 10. Test Quality Assessment

The focused tests are strong for the approved model-only phase.

Covered behavior includes:

- hook event kind name representation and serialization;
- valid bounded hook event payload accessors;
- redaction-safe debug output;
- secret-like phase ID rejection without leakage;
- secret-like redaction metadata rejection without leakage;
- serialization non-leakage for forbidden raw payload markers;
- invalid serialized payload fail-closed behavior without leakage;
- state-preserving rehydration from `Running`;
- idempotency key requirement;
- terminal state rejection;
- pre-running state rejection;
- executor non-regression proving no hook event append behavior.

No blocker-level test gaps were found.

Non-blocking future test additions:

- add direct tests for too-large reference counts;
- add direct tests for too-long redaction fields and reasons;
- add direct tests for optional empty phase IDs;
- add audit projection behavior tests once the audit projection policy is planned.

## 11. Documentation Review

Documentation is honest about the current state.

The roadmap, concept docs, planning docs, and end-of-phase report state that:

- the model-only hook workflow event vocabulary is implemented;
- the explicit `BeforeReport` executor checkpoint remains report-path-only and non-mutating;
- executor hook event append behavior is not implemented;
- audit sink emission is not implemented;
- persistence is not implemented;
- CLI behavior is not implemented;
- workflow schema fields are not implemented;
- automatic local checks are not implemented;
- command execution and adapter invocation are not implemented;
- side-effect modeling and writes remain unsupported;
- recursive agents, agent swarms, hosted behavior, and release posture changes are not introduced.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Plan whether hook workflow events should project into generic `AuditEvent`, a dedicated hook audit sink, or a separate hook audit store.
- Add additional boundary tests for reference-count and redaction metadata length limits.
- Decide whether future hook event IDs should become WorkReport citation targets or whether WorkReports should continue citing hook invocation IDs.
- Keep `BeforeReport` out of workflow event history unless a post-terminal metadata-event model is separately accepted.

## 14. Recommended Next Phase

Recommended next phase: **hook event audit projection planning**.

Now that the model-only hook workflow event vocabulary is accepted, the next question is not executor integration yet. The next safe step is to decide how future hook events relate to audit: generic workflow-event audit projection, dedicated hook audit sink records, separate hook audit store, or report-only citation. That planning should happen before any executor path appends hook workflow events.

## 15. Validation

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.
