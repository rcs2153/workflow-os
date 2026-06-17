# Hook Event Audit Projection Plan

Status: Projection-only implementation complete and reviewed. Executor hook event append planning is documented in [Executor Hook Event Append Plan](executor-hook-event-append-plan.md), and the first explicit `BeforeSkillInvocation` executor hook event append path is now implemented. This plan follows the accepted model-only hook workflow event vocabulary and defines how hook workflow events relate to audit projection. The implementation updates generic `AuditEvent::from_workflow_event(...)` projection for modeled hook workflow event vocabulary only. It does not implement dedicated hook audit sink emission, executor hook broadening, automatic workflow event append behavior, persistence, CLI behavior, workflow schema fields, automatic local checks, command execution, adapter invocation, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS now has model-only hook workflow event vocabulary:

- `HookInvocationRequested`;
- `HookInvocationEvaluated`;
- `AgentHarnessHookWorkflowEvent`.

The vocabulary is bounded, redaction-safe, state-preserving from `Running`, and not emitted by executor paths.

The next question is how hook workflow events should become audit-relevant when a future executor checkpoint appends them. The current `AuditEvent::from_workflow_event(...)` can project generic workflow event identity, but it does not yet define hook-specific audit action, reference, decision context, or relationship to the existing model-only `AgentHarnessHookAuditRecord`.

This plan's first projection-only implementation is complete. It implements generic hook workflow event audit projection only. A later bounded phase implemented the first explicit `BeforeSkillInvocation` executor hook event append path. This plan does not implement dedicated hook audit sink records, hook persistence, broader hook checkpoints, local check execution, adapter invocation, command execution, side effects, writes, CLI behavior, schemas, examples, or release posture changes.

## 2. Goals

- Decide how future hook workflow events should project into audit.
- Preserve `WorkflowRunEvent` as the source of truth for runtime state.
- Preserve `AuditEvent` as a projection of accepted workflow events.
- Preserve `AgentHarnessHookAuditRecord` as model-only hook audit vocabulary until sink/storage semantics are accepted.
- Avoid hidden workflow state.
- Avoid fake hook evidence, fake approvals, fake policy decisions, fake local check results, fake WorkReports, and fake side effects.
- Keep future hook audit records reference-first, bounded, deterministic, and redaction-safe.
- Prepare a small implementation prompt that can add hook-specific audit projection without executor integration.

## 3. Non-Goals

Do not implement in this planning phase:

- audit projection code changes;
- audit sink emission for hook records;
- dedicated hook audit sink methods;
- hook audit persistence;
- hook audit stores;
- executor hook broadening;
- automatic executor hook invocation;
- workflow event append behavior in `LocalExecutor`;
- additional executor checkpoints;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime hook configuration;
- CLI hook commands or rendering;
- automatic local check execution;
- default local check handler registration;
- command execution;
- adapter invocation;
- external provider calls;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- WorkReport artifact writes;
- reasoning lineage;
- side-effect boundary implementation;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Current hook model baseline:

- `AgentHarnessHookContract` models deterministic named checkpoint contracts.
- `AgentHarnessHookInvocationResult` validates explicit hook invocation context in memory.
- `AgentHarnessHookAuditRecord` is model-only audit vocabulary derived from invocation results.
- `execute_runtime_agent_harness_hook(...)` returns an in-memory invocation result plus model-only hook audit record.
- `LocalExecutor::execute_with_report(...)` supports one explicit `BeforeReport` hook on the report-bearing path.
- Successful `BeforeReport` hook invocation IDs can be cited by WorkReports.
- `HookInvocationRequested` and `HookInvocationEvaluated` workflow event vocabulary is implemented as model-only runtime event vocabulary.

Current audit baseline:

- `AuditEvent::from_workflow_event(...)` projects accepted workflow events into generic audit events.
- `AuditSink::record_audit_event(...)` records generic audit events.
- `AuditSink::record_policy_audit_record(...)` records policy audit records.
- `AuditSink::record_adapter_audit_record(...)` records adapter runtime audit telemetry.
- `LocalExecutor::append(...)` appends runtime events, emits audit projection, emits observability projection, and writes structured logs.
- A later bounded phase implemented the first explicit `BeforeSkillInvocation` executor hook event append path.
- No executor path appends hook workflow events automatically.
- No audit sink method exists for `AgentHarnessHookAuditRecord`.
- No hook audit store exists.

## 5. Projection Options

### Option A: Generic AuditEvent Projection Only

Project future hook workflow events through `AuditEvent::from_workflow_event(...)`.

Benefits:

- Keeps workflow event log as source of truth.
- Reuses existing audit sink interface.
- Avoids a new hook audit store.
- Keeps implementation small.

Risks:

- `AuditEvent` currently has no hook-specific fields.
- `Action` has no hook-specific action variant.
- Hook payload details could be lost unless projected into bounded references or decision context.

### Option B: Dedicated Hook Audit Sink Method

Add an audit sink method such as `record_agent_harness_hook_audit_record(...)`.

Benefits:

- Preserves full `AgentHarnessHookAuditRecord` shape.
- Keeps hook audit vocabulary distinct from generic workflow events.
- Avoids overloading `AuditEvent` fields.

Risks:

- Requires audit sink trait expansion.
- Requires local sink storage semantics.
- Could create duplicate audit surfaces if workflow events are also projected.
- Needs careful failure behavior if both generic and hook-specific sink writes exist.

### Option C: Hook Audit Store

Persist hook audit records in a dedicated store and cite them from reports later.

Benefits:

- Clear durable source for hook audit records.
- Supports report citation without overloading runtime event audit.

Risks:

- Adds persistence and storage API surface.
- Premature before executor event append semantics are proven.
- Requires retention, listing, and read APIs.

### Option D: Report-Only Citation

Continue citing hook invocation IDs only from WorkReports and do not add audit projection.

Benefits:

- Smallest behavior.
- Avoids audit complexity.

Risks:

- Insufficient for future state-visible hooks.
- Does not satisfy auditability expectations once hook events influence runtime flow.

## 6. Recommended Posture

Implemented first target: **generic hook workflow event audit projection, model-only/projection-only**.

That implementation:

- keeps `WorkflowRunEvent` as the source event;
- updates `AuditEvent::from_workflow_event(...)` helper behavior for hook workflow events;
- projects hook event type, run identity, actor, correlation ID, idempotency key, optional step ID, and bounded hook reference-count summaries;
- avoids adding executor event append behavior in this projection-only phase;
- avoids adding dedicated hook audit sink methods;
- avoids adding hook persistence;
- avoids adding observability hook metrics.

`AgentHarnessHookAuditRecord` sink emission is still not implemented. The dedicated hook audit record remains model-only until a later phase decides whether it should be emitted, projected, or stored.

## 7. Hook Audit Projection Rules

Future hook workflow event projection should be reference-first and bounded.

For `HookInvocationRequested` and `HookInvocationEvaluated`, `AuditEvent::from_workflow_event(...)` should preserve:

- source event ID;
- event timestamp;
- hook workflow event type;
- workflow ID;
- schema version;
- workflow version;
- workflow run ID;
- spec hash;
- optional step ID from hook payload;
- actor from workflow event;
- correlation ID from workflow event;
- idempotency key from workflow event;
- redaction metadata showing reference-only hook context.

Projection may include a bounded decision context such as:

- `hook invocation requested`;
- `hook invocation evaluated`;
- status vocabulary only.

Projection must not include:

- raw hook phase ID values if treated as sensitive;
- raw prompt or model context;
- raw local check output;
- raw command output;
- raw provider payloads;
- raw parser payloads;
- raw spec contents;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded hook disclosures;
- evidence payloads.

## 8. Action And Capability Posture

Do not add a new `Action` variant in the first projection implementation unless separately reviewed.

Recommended v1 projection behavior:

- `AuditEvent.action` for hook workflow events remains `None`;
- `event_type` carries the stable hook event kind;
- bounded decision context identifies the hook event lifecycle status;
- future policy-gated hook checkpoints can later introduce `Action::EvaluateHook` or a more specific action if policy semantics require it.

Rationale:

- hook workflow event vocabulary is model-only today;
- adding `Action` vocabulary affects policy semantics;
- future executor integration must decide whether hooks are policy-evaluated actions, audit-only checkpoints, or both.

## 9. Relationship To AgentHarnessHookAuditRecord

`AgentHarnessHookAuditRecord` remains model-only in this phase.

Future relationships:

- Generic `AuditEvent` can record that a hook workflow event occurred.
- `AgentHarnessHookAuditRecord` can preserve richer hook invocation context.
- WorkReports can continue citing `AgentHarnessHookInvocationId` until hook audit record IDs or hook event IDs are explicitly selected.

Do not duplicate full hook audit record content into `AuditEvent`.

Do not automatically emit `AgentHarnessHookAuditRecord` when a hook workflow event is projected.

## 10. Relationship To Observability

Do not add hook observability events in the first audit projection implementation.

Reasons:

- the current phase is audit projection, not metrics;
- hook warning/failure/blocking semantics are not implemented;
- metric cardinality and labels need separate design;
- adding observability before executor append behavior could overclaim runtime support.

Future observability planning may consider hook counts, hook warning counts, hook failed-closed counts, and hook latency only after executor integration and failure semantics are accepted.

## 11. Executor Integration Boundary

Audit projection planning did not authorize executor event append behavior; a separate bounded phase implemented the first explicit append path.

Future executor integration requires a separate plan that decides:

- exact checkpoint placement;
- hook event order relative to policy decisions, approval requests, skill invocation, retry, escalation, and terminal events;
- whether hook events are pre-side-effect or post-side-effect;
- whether hook failures block, warn, pause, escalate, or fail;
- idempotency key derivation;
- replay and duplicate run behavior;
- audit sink failure behavior;
- observability behavior;
- report citation behavior.

The current `BeforeReport` checkpoint remains post-terminal, report-path-only, in-memory-only, and non-mutating. It should not append hook workflow events in the audit projection implementation.

## 12. Privacy And Redaction

Hook audit projection must:

- use existing validated hook workflow event payloads;
- avoid copying raw hook context;
- treat hook IDs and contract IDs as stable references, not user-facing summaries;
- avoid copying phase IDs into unbounded audit summaries;
- mark hook context as reference-only where represented;
- produce stable, non-leaking errors;
- keep `Debug` output redaction-safe;
- keep serialization bounded and reference-first.

Forbidden payloads:

- raw prompts;
- raw command output;
- command transcripts;
- raw local check output;
- provider payloads;
- CI logs;
- Jira or GitHub raw bodies;
- parser payloads;
- raw spec contents;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded natural-language summaries.

## 13. Failure Semantics

Projection helper construction should not fail for a valid `WorkflowRunEvent`; it should deterministically project the accepted event.

Audit sink failure behavior remains unchanged:

- if a future executor path appends a hook workflow event and emits an audit event through the existing append path, audit sink failure should be handled like other runtime audit sink failures;
- this planning phase does not change workflow semantics;
- this planning phase does not decide whether hook failure blocks execution.

## 14. Test Plan For Future Implementation

Future projection-only tests should cover:

- `AuditEvent::from_workflow_event(...)` projects `HookInvocationRequested`;
- `AuditEvent::from_workflow_event(...)` projects `HookInvocationEvaluated`;
- projected audit event preserves event ID, timestamp, event type, workflow ID, run ID, schema version, workflow version, spec hash, actor, correlation ID, and idempotency key;
- projected audit event includes optional hook step ID when present;
- projected audit event action remains `None` unless a separate action-vocabulary phase is accepted;
- projected decision context is bounded and does not copy raw phase IDs or hook metadata;
- projected input/output references do not copy raw hook input or output values;
- projected redaction metadata is reference-only and non-leaking;
- hook audit projection does not emit `AgentHarnessHookAuditRecord`;
- hook audit projection does not require a hook audit store;
- hook audit projection does not add observability hook metrics;
- invalid hook payload serde still fails closed;
- local executor tests still prove no hook workflow events are appended;
- existing audit, observability, runtime, hook, WorkReport, EvidenceReference, local check, adapter, and executor tests continue to pass.

## 15. Proposed Implementation Sequence

1. Add hook-specific projection behavior in `AuditEvent::from_workflow_event(...)`.
2. Add focused audit projection tests for `HookInvocationRequested` and `HookInvocationEvaluated`.
3. Add non-regression tests proving no `AgentHarnessHookAuditRecord` sink/store behavior is introduced.
4. Add non-regression tests proving no hook observability event is emitted by projection helper behavior.
5. Update docs and create an end-of-phase report.
6. Review.
7. Only after review, plan one pre-terminal executor checkpoint if still needed.

## 16. Deferred Work

- Broader executor hook event append behavior beyond the explicit `BeforeSkillInvocation` checkpoint.
- Pre-terminal hook checkpoint integration.
- Dedicated hook audit sink method.
- Hook audit store or persistence.
- Hook event WorkReport citation target.
- Hook observability metrics.
- Hook failure/blocking runtime semantics.
- Post-terminal `BeforeReport` workflow event representation.
- Workflow schema fields.
- CLI rendering.
- Automatic local check execution.
- Command execution.
- Adapter invocation.
- Approval attachment.
- Evidence attachment.
- Side-effect boundary implementation.
- Writes.
- Reasoning lineage.

## 17. Open Questions

- Should hook workflow events eventually project to `AuditEvent.action = None`, `Action::Unknown("hook_invocation")`, or a new `Action::EvaluateHook`?
- Should hook event IDs become WorkReport citation targets, or should reports keep citing hook invocation IDs?
- Should richer `AgentHarnessHookAuditRecord` values be emitted to a dedicated audit sink later?
- Should hook audit records be persisted in state or derived from workflow events?
- Should hook observability metrics exist before executor hook integration?
- Should `BeforeReport` ever have a durable post-terminal audit record outside workflow events?
- Should hook output references remain counts until evidence completeness rules exist?

## 18. Final Recommendation

Proceed next to **executor hook event append planning**.

That planning phase should decide the first safe pre-terminal executor append boundary before any runtime path emits hook workflow events. It must not append hook workflow events from executor paths, emit dedicated hook audit sink records, persist hook audit records, add observability metrics, add schemas, expose CLI behavior, run local checks, execute commands, invoke adapters, model side effects, add writes, or change release posture.
