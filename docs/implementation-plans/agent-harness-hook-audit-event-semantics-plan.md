# Agent Harness Hook Audit/Event Semantics Plan

Status: Planning complete; the agent harness hook audit record core model is implemented. Runtime hook execution planning is documented in [Agent Harness Hook Runtime Execution Plan](agent-harness-hook-runtime-execution-plan.md), and the explicit in-memory runtime hook execution helper is implemented. Workflow event emission, audit sink emission, and persistence are not implemented.

## 1. Executive Summary

The Agent Harness Hook contract model and in-memory invocation helper model are implemented and reviewed. Workflow OS can now represent deterministic named checkpoint contracts and validate explicit phase-level hook invocation context, but hook invocation does not mutate runtime state, append workflow events, emit audit records, run local checks, invoke adapters, or execute agent code.

The next question is how hook invocation results should relate to runtime event history and audit records before any executor integration is attempted.

This plan defines conservative audit/event semantics for future hook work. It does not implement runtime hook execution, executor integration, workflow events, audit record emission, local check execution, CLI commands, schema fields, persistence changes, report artifact auto-writing, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Goals

- Preserve the event log as the source of truth for workflow state.
- Preserve audit records as governed projections or audit-shaped ledgers, not hidden workflow state.
- Decide when hook invocation should become audit-relevant.
- Decide whether hook invocation needs a stable result ID before executor integration.
- Keep hook semantics deterministic, local, explicit, and reviewable.
- Avoid metadata-only post-terminal workflow events.
- Avoid fake evidence, fake local check results, fake approvals, fake WorkReports, or fake hook outcomes.
- Prepare a small model-only implementation phase for hook audit records and stable hook result references.

## 3. Non-Goals

Do not implement in this plan:

- runtime hook execution;
- executor-integrated hook invocation;
- automatic workflow execution;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- CLI hook commands;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime harness generation;
- nested harness execution;
- recursive agents;
- agent swarms;
- hosted or distributed execution;
- side-effect modeling;
- writes;
- approval evidence attachment;
- reasoning lineage;
- persistence changes;
- report artifact auto-writing;
- examples;
- release posture changes.

## 4. Source-Of-Truth Boundaries

Workflow OS already separates runtime state, audit projections, evidence references, and reports:

- `WorkflowRunEvent` is the source of truth for run state.
- `WorkflowRunSnapshot` is a rebuildable projection.
- `AuditEvent` is an audit-shaped projection of runtime events.
- `PolicyAuditRecord` can exist before `RunCreated` when denied starts must not create misleading runs.
- `EvidenceReference` points at evidence without copying payloads.
- `WorkReport` is a governed handoff artifact, not an audit log.
- `AgentHarnessHookInvocationResult` is currently an in-memory validation result, not runtime state.

Future hook semantics must preserve these boundaries. A hook result should not become workflow state unless a scoped runtime event is accepted. A hook audit record should not imply that the hook executed side effects or changed workflow state.

## 5. Recommended Audit/Event Posture

Recommended posture: add a model-only hook audit record before executor integration.

The next implementation should define a validated, redaction-safe model for recording that a hook invocation context was evaluated. It should include stable identity and references, but it should not append workflow events or write audit records automatically.

Do not add hook workflow events in the next implementation.

Reasons:

- workflow events affect state transition validation and replay semantics;
- v0 rejects post-terminal events and has no metadata-only event model;
- executor integration must decide exact hook placement before event kinds are meaningful;
- a hook audit record can be modeled and reviewed without mutating runtime history.

## 6. Candidate Hook Audit Record Model

A future model-only phase should consider:

- `AgentHarnessHookInvocationId`;
- `AgentHarnessHookAuditRecord`;
- `AgentHarnessHookAuditRecordDefinition`;
- `AgentHarnessHookAuditScope`;
- `AgentHarnessHookAuditOutcome`;
- `AgentHarnessHookAuditReference`.

Required fields should include:

- hook invocation ID;
- hook contract ID;
- hook contract version;
- hook kind;
- invocation status;
- workflow ID;
- workflow version;
- run ID;
- schema version;
- spec hash;
- actor or system actor;
- invoked timestamp;
- optional correlation ID;
- optional step ID;
- optional phase ID;
- input reference names and stable reference targets;
- output reference names and stable reference targets;
- supplemental stable references;
- bounded disclosures;
- redaction metadata;
- sensitivity.

The model should not store raw prompts, raw spec contents, raw command output, provider payloads, parser payloads, environment values, credentials, tokens, or unbounded natural-language text.

## 7. Hook Result Identity

Recommendation: introduce a stable hook invocation/result ID before executor integration.

Rationale:

- WorkReports need stable citations if hook results become report inputs.
- Audit records need stable references for durable review.
- Executor integration should not invent IDs after the fact.
- Stable IDs make retry and duplicate-invocation semantics easier to reason about later.

The first ID model should remain simple and explicit. It should be caller-supplied or derived only through a documented deterministic rule in a later runtime phase. The model-only phase should not create ambient ID generation inside runtime paths.

## 8. Workflow Event Semantics

Do not add hook workflow event kinds in the next phase.

Future executor integration may later add event kinds such as:

- `HookInvocationRequested`;
- `HookInvocationEvaluated`;
- `HookInvocationFailedClosed`;
- `HookInvocationSkipped`;
- `HookInvocationBlocked`.

These event kinds should not be implemented until a separate executor integration plan decides:

- exact placement relative to `StepScheduled`, policy decisions, approval requests, and skill invocation events;
- whether hook failure blocks, warns, fails the run, pauses for approval, or records only audit context;
- idempotency key requirements;
- retry behavior;
- state transition effects;
- terminal-state restrictions;
- snapshot projection behavior.

## 9. Audit Record Semantics

Hook audit records should be explicit and bounded.

A future hook audit record should mean:

- a specific hook contract and invocation context were evaluated;
- supplied references were validated at the hook boundary;
- the result had a stable status;
- the record can be cited by future reports or reviews.

It should not mean:

- a workflow event was appended;
- a local check was executed;
- an adapter was invoked;
- an approval was requested or decided;
- evidence was created;
- a side effect was authorized;
- a write occurred;
- runtime execution semantics changed.

## 10. Relationship To Existing Audit Events

`AuditEvent` currently projects workflow events. Hook audit records should not be forced into that projection until hook workflow events exist.

Recommended boundary:

- model-only hook audit record first;
- optional audit sink emission later, only after executor integration semantics are accepted;
- workflow-event projection later, only if hook events become runtime events.

This keeps the audit model honest: no event source, no event projection.

## 11. Relationship To Policy Gates And Approvals

Hooks may cite policy decisions and approval decisions by stable reference, but they must not create them.

Future hook audit records may include policy and approval references that already exist. Missing required policy or approval context should fail closed once requirements become part of hook contracts.

Do not treat model self-review as policy. Do not treat hook invocation as approval. Do not use hook audit records to bypass approval gates.

## 12. Relationship To Local Checks

Hooks may cite local check result IDs already produced by explicit local check APIs.

Hooks must not:

- execute `DocsCheck`;
- register default handlers;
- run arbitrary commands;
- create local check results;
- create command-output evidence;
- treat missing local check results as successful checks.

Hook audit records should cite local check result references only when supplied by a caller that already has a stable result ID.

## 13. Relationship To EvidenceReference

Hook audit records may cite `EvidenceReference` IDs supplied by the caller.

They must not:

- create EvidenceReference values implicitly;
- attach evidence to diagnostics, adapters, approvals, reports, or hooks;
- copy evidence payloads;
- copy raw source, command, provider, parser, CI, Jira, or GitHub payloads.

Evidence completeness requirements remain future work.

## 14. Relationship To WorkReports

Future WorkReports may cite hook audit records or hook invocation IDs once stable IDs exist.

Do not update WorkReport citation vocabulary in this planning phase. A future WorkReport hook citation phase should decide whether reports cite:

- hook invocation ID;
- hook audit record ID;
- workflow event ID, if hook events exist later;
- or another stable reference.

Reports must not copy hook disclosures, raw context, or raw payloads by default.

## 15. Failure Semantics

Future hook audit/event semantics must decide how each status affects runtime behavior.

Recommended conservative defaults for future executor planning:

- `Passed`: may allow execution to continue if all other gates pass.
- `FailedClosed`: should block or fail the scoped action unless policy explicitly permits a safer paused state.
- `Warning`: should not block by default, but must be auditable if executor-integrated.
- `SkippedWithDisclosure`: should be explicit and auditable, not silent.
- `Blocked`: should pause or stop the scoped action according to a separately accepted policy.

This plan does not implement those semantics.

## 16. Privacy And Redaction

Future hook audit records must:

- use validated ID and redaction primitives;
- bound all caller-supplied text;
- reject secret-like values;
- redact `Debug` output;
- fail closed on invalid serialized payloads;
- avoid leaking raw values in errors;
- serialize only validated bounded references and metadata.

Hook audit records may be sensitive even when they contain only stable references.

## 17. Test Plan For Future Implementation

Future model-only hook audit record tests should cover:

- valid minimal hook audit record;
- required identity fields;
- invalid hook invocation ID rejection;
- invalid contract ID/version rejection;
- invalid workflow/run identity rejection;
- invalid schema version rejection;
- invalid spec hash rejection;
- invalid actor rejection;
- stable input/output/supplemental references;
- duplicate named references rejected;
- secret-like references rejected without leakage;
- bounded disclosures;
- secret-like disclosures rejected without leakage;
- redaction metadata validation;
- redaction-safe `Debug`;
- serde round trip;
- invalid serialized record fails closed;
- no workflow event append behavior encoded;
- no audit sink emission encoded;
- no local check execution encoded;
- no adapter invocation encoded;
- no filesystem artifact behavior encoded;
- no CLI behavior encoded.

Future executor integration tests should be planned separately.

## 18. Proposed Implementation Sequence

Recommended small phases:

1. Hook audit record core model, model-only.
2. Maintainer review.
3. WorkReport hook citation target planning.
4. Optional WorkReport hook citation target model.
5. Executor hook integration planning.
6. Optional executor-integrated hook invocation implementation.
7. Workflow schema planning only after runtime/API behavior is reviewed.
8. CLI hook commands only after schema/API behavior is stable.

## 19. Open Questions

- Should hook invocation ID and hook audit record ID be the same identifier?
- Should hook audit records be stored in a future dedicated store, projected from events, or both?
- Should hook records be emitted before or after policy decisions in an executor-integrated path?
- Should hook failure ever emit a workflow event before failing the run?
- Should warning hooks be allowed in self-governance dogfood before fail-closed hooks?
- Should hook audit records be citeable by WorkReports immediately after the model exists?
- Should hook contracts eventually declare evidence, local check, approval, policy, or handoff requirements as first-class fields?
- What is the smallest useful dogfood checkpoint: before validation, after validation, before report, or after report?

## 20. Final Recommendation

The hook audit record core model is implemented. Proceed next to **agent harness hook audit record core model review**.

That review should verify the validated hook invocation ID and hook audit record model, privacy posture, serde behavior, test quality, and scope boundaries. It must not append workflow events, emit audit records, integrate with `LocalExecutor`, run local checks, invoke adapters, write files, persist records, expose CLI behavior, add schema fields, authorize side effects, add writes, implement reasoning lineage, enable recursive agents, enable agent swarms, or change release posture.
