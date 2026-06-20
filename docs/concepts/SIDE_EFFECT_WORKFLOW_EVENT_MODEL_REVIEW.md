# SideEffect Workflow Event Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implemented phase adds model-only SideEffect workflow event vocabulary and bounded generic audit projection without enabling executor append behavior, persistence, automatic discovery, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved event/audit projection scope.

Implemented scope:

- `SideEffectProposed`;
- `SideEffectDenied`;
- `SideEffectSkipped`;
- `SideEffectAttempted`;
- `SideEffectCompleted`;
- `SideEffectFailed`;
- `SideEffectWorkflowEvent`;
- `SideEffectWorkflowEventDefinition`;
- reference-first event payload validation;
- state-preserving replay from `Running`;
- idempotency-key requirements;
- lifecycle alignment validation;
- bounded generic `AuditEvent::from_workflow_event(...)` projection;
- focused runtime and audit projection tests;
- documentation and end-of-phase report.

No accidental implementation was found for:

- executor SideEffect event append behavior;
- SideEffect persistence or a SideEffect store;
- automatic SideEffect discovery;
- EvidenceReference side-effect attachment;
- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- local filesystem writes through runtime;
- dedicated SideEffect audit sink storage;
- observability events for SideEffect workflow events;
- workflow schema fields;
- CLI rendering or commands;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- release posture changes.

## 3. Event Vocabulary Assessment

The selected vocabulary is appropriately minimal and matches the accepted SideEffect lifecycle boundary:

- proposed;
- denied;
- skipped;
- attempted;
- completed;
- failed.

The implementation correctly avoids `SideEffectApproved`, because approval is authority context rather than a lifecycle event. It also avoids rollback and compensation vocabulary, which would require separate runtime and adapter semantics.

The event variants use a shared `SideEffectWorkflowEvent` payload, which keeps the model compact and makes validation consistent across lifecycle moments.

## 4. Payload Model Assessment

The payload is bounded and reference-first.

It includes:

- `side_effect_id`;
- `lifecycle_state`;
- optional step and skill context;
- optional correlation ID;
- bounded stable `SideEffectReference` values;
- evidence and outcome reference counts;
- sensitivity;
- redaction metadata.

It intentionally does not copy:

- full `SideEffectRecord` values;
- side-effect target references;
- authority packets;
- reason-code lists;
- outcome bodies;
- provider payloads;
- command output;
- parser payloads;
- raw spec content.

Using bounded `SideEffectReference` values instead of dedicated policy, approval, adapter, and evidence fields is acceptable for this model-only phase. It keeps the event vocabulary useful for future discovery without prematurely designing a SideEffect store or write execution layer.

## 5. State Transition And Replay Assessment

The transition behavior is conservative and consistent with the hook event precedent.

Verified behavior:

- SideEffect workflow events are state-preserving from `WorkflowRunStatus::Running`.
- SideEffect workflow events are rejected before `RunStarted`.
- SideEffect workflow events are rejected after terminal states.
- SideEffect workflow events require idempotency keys.
- Event kind and embedded lifecycle state must match.
- No new workflow statuses are introduced.
- SideEffect failure does not become terminal by itself.

This preserves workflow replay determinism and avoids using SideEffect event vocabulary to smuggle execution semantics into the runtime.

## 6. Audit Projection Assessment

Generic audit projection is appropriately bounded.

The projection preserves:

- workflow event identity;
- timestamp;
- event kind;
- workflow identity;
- run ID;
- spec hash;
- actor;
- optional step and skill context;
- correlation ID;
- idempotency key.

The projection marks side-effect context as reference-only redaction metadata, uses bounded lifecycle summaries, and keeps `AuditEvent.action` as `None`. `SideEffectCompleted` exposes outcome presence through a count-only output reference rather than copying outcome bodies.

No dedicated SideEffect audit sink, audit record family, persistence behavior, or observability event emission was introduced.

## 7. Privacy And Redaction Assessment

The implementation is redaction-aware and does not store raw side-effect payloads.

Verified privacy posture:

- no raw provider payloads;
- no raw command output;
- no raw CI logs;
- no raw Jira or GitHub bodies;
- no raw spec contents;
- no parser payloads;
- no environment variable values;
- no credentials;
- no authorization headers;
- no private keys;
- no token-like values;
- no side-effect target references;
- no authority packets;
- no outcome bodies.

`SideEffectWorkflowEvent` Debug output redacts IDs, references, and redaction metadata. Deserialization runs through validated constructors and fails closed with stable, non-leaking error codes.

The audit projection intentionally includes a stable SideEffect ID reference in `input_reference`. That is acceptable because the ID is already validated, bounded, and treated as a reference, not payload content.

## 8. Relationship To Existing Models

The phase preserves existing source-of-truth boundaries.

- `SideEffectRecord` remains the source of truth for side-effect intent, authority, lifecycle, idempotency, targets, and outcomes.
- `WorkflowRunEvent` gains lifecycle vocabulary but does not create or resolve SideEffect records.
- `AuditEvent` remains a bounded projection of accepted workflow events.
- `WorkReport` can cite SideEffect IDs, but report discovery from workflow events or audit projections remains unimplemented.
- `EvidenceReference` side-effect attachment remains deferred.

This is the right boundary before any executor append, persistence, automatic discovery, or runtime execution work.

## 9. Test Quality Assessment

The tests are strong for the approved scope.

Covered:

- stable event kind names;
- valid payload construction and accessors;
- Debug redaction;
- secret-like SideEffect ID deserialization failure without leakage;
- secret-like redaction metadata rejection without leakage;
- serialization non-leakage for raw payload markers;
- invalid serialized redaction metadata failure without leakage;
- state-preserving replay from `Running`;
- lifecycle mismatch rejection;
- idempotency-key requirement for all SideEffect lifecycle events;
- rejection before `RunStarted`;
- rejection after terminal states;
- generic audit projection identity preservation;
- bounded decision context;
- count-only outcome reference projection;
- reference-only redaction metadata;
- absence of dedicated SideEffect audit records;
- absence of SideEffect observability events.

Non-blocking test gaps:

- add direct `SideEffectWorkflowEvent` tests for duplicate references;
- add direct max-reference-list and max-reference-count tests at the workflow event payload boundary;
- add direct redaction field/reason length-boundary tests.

These gaps do not block the phase because the validation paths exist and the high-risk semantics are covered.

## 10. Documentation Review

Documentation is honest about the current state.

Verified:

- SideEffect workflow event vocabulary is implemented as model-only vocabulary.
- Generic audit projection is implemented as bounded projection-only behavior.
- Executor append behavior is not implemented.
- SideEffect persistence is not implemented.
- Automatic SideEffect discovery is not implemented.
- EvidenceReference side-effect attachment is not implemented.
- Runtime side-effect execution is not implemented.
- Write-capable adapters are not implemented.
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

During review, the current-baseline section in `side-effect-workflow-event-audit-projection-plan.md` was corrected to remove a stale false statement that SideEffect event kinds and audit projection rules were still unimplemented.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add direct payload-boundary tests for duplicate `SideEffectReference` values.
- Add max-list and max-count tests for `SideEffectWorkflowEvent` references.
- Add redaction metadata length-boundary tests for SideEffect workflow event payloads.
- Before any executor append behavior, write a focused plan for event ordering, append inputs, idempotency keys, failure semantics, and replay impact.
- Before automatic WorkReport discovery, decide whether workflow events, audit projections, a future SideEffect store, or a combination is the source of discoverable SideEffect references.

## 13. Recommended Next Phase

Recommended next phase: **executor SideEffect event append planning**.

Reason: the model-only vocabulary and generic audit projection are accepted. The next risky boundary is not persistence or writes yet; it is deciding where and when an executor may append SideEffect lifecycle events without creating fake SideEffect records, changing workflow semantics, or implying runtime side-effect execution.

That planning phase must not implement runtime side-effect execution, write-capable adapters, provider mutations, persistence, automatic discovery, EvidenceReference side-effect attachment, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 14. Validation

Run during review:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
