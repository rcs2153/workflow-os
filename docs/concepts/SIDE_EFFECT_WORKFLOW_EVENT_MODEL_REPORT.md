# SideEffect Workflow Event Model Report

## 1. Executive summary

The SideEffect workflow event model phase is implemented as a model-only event vocabulary plus bounded generic audit projection.

Workflow OS can now represent SideEffect lifecycle moments in the workflow event vocabulary without enabling runtime side-effect execution, write-capable adapters, executor append behavior, persistence, automatic discovery, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope completed

- Added model-only SideEffect workflow event kinds:
  - `SideEffectProposed`
  - `SideEffectDenied`
  - `SideEffectSkipped`
  - `SideEffectAttempted`
  - `SideEffectCompleted`
  - `SideEffectFailed`
- Added `SideEffectWorkflowEvent` and `SideEffectWorkflowEventDefinition`.
- Added bounded, reference-first SideEffect workflow event payload validation.
- Added lifecycle alignment validation between event kind and payload lifecycle state.
- Added idempotency-key requirements for all SideEffect workflow event kinds.
- Added state-preserving transition behavior from `Running`.
- Rejected SideEffect workflow events before `RunStarted` and after terminal states.
- Added bounded generic `AuditEvent::from_workflow_event(...)` projection for SideEffect workflow events.
- Exported the new model types from `workflow-core`.
- Added focused runtime and audit projection tests.

## 3. Scope explicitly not completed

This phase did not implement:

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

## 4. Model types and event vocabulary

`SideEffectWorkflowEvent` carries:

- `side_effect_id`;
- `lifecycle_state`;
- optional `step_id`;
- optional `skill_id`;
- optional `skill_version`;
- optional `correlation_id`;
- bounded stable `SideEffectReference` values;
- `evidence_reference_count`;
- `outcome_reference_count`;
- `redaction`;
- `sensitivity`.

The event intentionally does not embed a full `SideEffectRecord`, target reference, authority packet, reason codes, outcome body, provider payload, command output, parser payload, or raw spec content.

## 5. State transition behavior

SideEffect workflow events are state-preserving from `WorkflowRunStatus::Running`.

They are rejected:

- before `RunStarted`;
- after terminal statuses;
- when appended without an idempotency key;
- when the event kind does not match the payload lifecycle state.

The events do not introduce new workflow statuses and do not make SideEffect failures terminal by themselves. Future runtime behavior should pair any terminal outcome with existing reviewed failure or cancellation transitions.

## 6. Audit projection summary

Generic audit projection is implemented through `AuditEvent::from_workflow_event(...)`.

Projection preserves:

- workflow event ID;
- timestamp;
- event type;
- workflow identity;
- run ID;
- spec hash;
- optional step/skill context;
- actor;
- correlation ID;
- idempotency key.

Projection uses bounded lifecycle summaries such as `side effect proposed: lifecycle=proposed`, keeps `AuditEvent.action` as `None`, marks side-effect context as reference-only redaction metadata, and does not emit dedicated SideEffect audit records or observability events.

## 7. Privacy and redaction summary

The implementation is reference-first and redaction-aware.

It does not store or copy:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- side-effect target references;
- side-effect summaries;
- side-effect reason codes;
- authority packets;
- outcome bodies.

`Debug` output redacts SideEffect IDs, step/skill identifiers, references, and redaction metadata. Deserialization fails closed through validated constructors.

## 8. Test coverage summary

Tests cover:

- stable SideEffect workflow event kind names;
- valid bounded payload construction;
- secret-like SideEffect ID deserialization failure without leakage;
- secret-like redaction metadata rejection without leakage;
- serialization non-leakage for raw payload markers;
- invalid serialized redaction metadata failure without leakage;
- state-preserving replay from `Running`;
- lifecycle mismatch rejection;
- idempotency-key requirement for all SideEffect event kinds;
- rejection before `RunStarted`;
- rejection after terminal status;
- generic audit projection identity preservation;
- bounded decision context;
- count-only outcome reference projection;
- reference-only redaction metadata;
- no dedicated SideEffect audit records;
- no SideEffect observability events.

## 9. Commands run and results

Run during implementation:

- `cargo fmt --all` - passed after applying formatting.
- `cargo test -p workflow-core --test runtime_events` - passed.
- `cargo test -p workflow-core --test audit_projection` - passed.

Final validation:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Remaining known limitations

- Executor paths do not append SideEffect workflow events.
- SideEffect records are not persisted or resolved.
- WorkReports do not automatically discover SideEffect IDs from workflow events or audit projections.
- EvidenceReference side-effect attachment is not implemented.
- Runtime side-effect execution and write-capable adapters remain future scoped work.
- Event/audit projection behavior still needs maintainer review before any executor integration.

## 11. Recommended next phase

Recommended next phase: **SideEffect workflow event/audit projection review**.

Reason: the model-only vocabulary and bounded generic audit projection are now implemented. Before planning executor append behavior, persistence, automatic discovery, or EvidenceReference side-effect attachment, the event vocabulary, transition behavior, idempotency requirement, audit projection, and privacy posture should receive a phase-level maintainer review.
