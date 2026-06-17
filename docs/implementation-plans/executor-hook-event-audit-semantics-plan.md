# Executor Hook Event And Audit Semantics Plan

Status: Model-only hook workflow event vocabulary implemented and reviewed; generic hook workflow event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](hook-event-audit-projection-plan.md); executor hook event append planning is documented in [Executor Hook Event Append Plan](executor-hook-event-append-plan.md). This plan follows the accepted explicit `BeforeReport` executor hook integration and defines the durable semantics boundary before any state-visible or pre-side-effect hook checkpoint is implemented. The implementation adds bounded hook workflow event payload vocabulary, state-preserving transition rules for future non-terminal hook events, and generic audit projection for those modeled hook event kinds. It does not add executor event append behavior, dedicated hook audit sink emission, persistence, executor hook broadening, CLI behavior, workflow schema fields, automatic local checks, command execution, adapter invocation, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS now has:

- an agent harness hook contract model;
- an in-memory hook invocation helper;
- a model-only hook audit record;
- WorkReport hook citation vocabulary;
- terminal report helper hook citation support;
- executor report input propagation for hook IDs;
- an explicit in-memory runtime hook execution helper;
- an explicit `BeforeReport` checkpoint on `LocalExecutor::execute_with_report(...)`.

The current `BeforeReport` checkpoint is intentionally report-path-only and non-mutating. It does not append workflow events or emit audit sink records.

The next question is how future hook checkpoints should become durable runtime history without corrupting the event log, changing workflow semantics prematurely, or turning hooks into ambient agent orchestration.

This plan defines the event and audit semantics needed before any future state-visible hook checkpoint. The first implementation is complete as a conservative model-only slice: **hook workflow event vocabulary and transition rules**, with no executor append behavior yet.

## 2. Goals

- Preserve `WorkflowRunEvent` as the source of truth for workflow state.
- Preserve `WorkflowRunSnapshot` as a rebuildable projection.
- Preserve audit records as projections or explicit audit-shaped ledgers, not hidden workflow state.
- Define where hook workflow events may eventually fit in the run lifecycle.
- Define whether post-terminal `BeforeReport` should become a workflow event.
- Define audit sink emission prerequisites.
- Define idempotency and replay questions before runtime mutation.
- Preserve existing `execute(...)` and `execute_with_report(...)` semantics.
- Avoid fake evidence, fake approvals, fake local check results, fake WorkReports, fake policy decisions, and fake hook outcomes.
- Keep hook semantics deterministic, explicit, bounded, and reviewable.
- Prepare a small code-bearing model-only implementation phase.

## 3. Non-Goals

Do not implement in this plan:

- runtime hook broadening;
- automatic executor hook invocation;
- additional executor checkpoints;
- workflow event append behavior;
- audit sink emission for hook records;
- hook persistence;
- hook audit store;
- report artifact writes;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime hook config;
- CLI hook commands;
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
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Runtime Baseline

Current `LocalExecutor` behavior:

1. loads and validates the project;
2. evaluates pre-run policy;
3. appends `RunCreated`;
4. appends `RunValidated`;
5. appends `RunStarted`;
6. schedules steps;
7. requests approvals when required;
8. evaluates policy before skill invocation;
9. appends skill invocation request/start/success/failure events;
10. handles retry and escalation events;
11. appends terminal completion, failure, or cancellation events;
12. derives in-memory reports only on explicit report-bearing paths.

Current hook behavior:

- `execute_runtime_agent_harness_hook(...)` validates explicit hook context and returns an in-memory invocation result plus model-only audit record.
- `LocalExecutor::execute_with_report(...)` can execute a supplied `BeforeReport` hook after a terminal run exists and before report generation.
- Successful `BeforeReport` hook invocation IDs are forwarded into WorkReport citations.
- Hook failure returns as `report_generation_error` and does not change the run.
- No hook workflow events, audit sink records, persistence, artifact writes, or CLI output are produced.

## 5. Source-Of-Truth Boundaries

The boundaries remain:

- `WorkflowRunEvent`: source of truth for runtime state transitions.
- `WorkflowRunSnapshot`: projection derived from workflow events.
- `AuditEvent`: audit projection of accepted runtime events.
- `PolicyAuditRecord`: explicit policy audit record, including cases before run creation.
- `AgentHarnessHookInvocationResult`: in-memory hook validation result.
- `AgentHarnessHookAuditRecord`: model-only hook audit record.
- `WorkReport`: governed handoff artifact.
- `EvidenceReference`: citation pointer, not payload storage.

Hook execution must not become hidden workflow state. If a hook affects workflow execution, it needs a workflow event or a separately accepted audit/store boundary.

## 6. Post-Terminal BeforeReport Policy

The implemented `BeforeReport` checkpoint runs after a terminal `WorkflowRun` exists.

Policy: **do not convert the current `BeforeReport` checkpoint into a workflow event yet**.

Reasons:

- terminal states currently reject further mutating events;
- post-terminal metadata-only events would change the event model;
- adding post-terminal events would affect replay, projections, audit, and artifact semantics;
- report generation already has a separate error channel;
- current `BeforeReport` is intentionally in-memory and report-path-only.

If durable `BeforeReport` records are needed later, prefer a separately planned hook audit store or report artifact linkage before allowing post-terminal workflow events.

## 7. Candidate Future Hook Workflow Events

Future pre-terminal hook checkpoints may need event vocabulary such as:

- `HookInvocationRequested`;
- `HookInvocationEvaluated`;
- `HookInvocationFailedClosed`;
- `HookInvocationSkipped`;
- `HookInvocationBlocked`.

These names are candidates. The first code-bearing phase should keep vocabulary minimal and model-only.

Recommended minimal v1 event payload fields:

- hook invocation ID;
- hook contract ID;
- hook contract version;
- hook kind;
- hook invocation status;
- optional step ID;
- optional phase ID;
- optional correlation ID;
- stable input reference count or IDs if already validated;
- stable output reference count or IDs if already validated;
- redaction metadata;
- sensitivity.

Do not store raw prompts, raw spec contents, raw command output, provider payloads, parser payloads, local check transcripts, environment values, credentials, tokens, or unbounded summaries in hook events.

## 8. Event Ordering Policy

Future event-producing checkpoints must define exact placement before implementation.

Candidate ordering:

| Checkpoint | Possible event placement | Initial posture |
| --- | --- | --- |
| `BeforeReport` | after terminal event | Defer. Current terminal-state rules reject post-terminal events. |
| `BeforeSkillInvocation` | after invoke policy allow, before `SkillInvocationRequested` | High authority. Requires separate failure semantics. |
| `AfterSkillSuccess` | after `SkillInvocationSucceeded`, before next step scheduling | Useful for validation/handoff. Requires retry/continuation semantics. |
| `AfterSkillFailure` | after `SkillInvocationFailed`, before retry/escalation decision | Useful for failure classification. Requires retry ordering design. |
| `BeforeApprovalRequest` | after policy says approval required, before `ApprovalRequested` | Defer. Must not fabricate or bypass approval context. |
| `AfterApprovalDecision` | after approval event, before resume/terminal outcome | Defer. Must preserve approval authority. |

No event-producing checkpoint should be implemented until the exact ordering and state transition effect are reviewed.

## 9. State Transition Policy

Hook events should initially be non-terminal, state-preserving events only where possible.

Candidate transition posture:

- `HookInvocationRequested`: state-preserving from `Running`.
- `HookInvocationEvaluated`: state-preserving from `Running`.
- `HookInvocationSkipped`: state-preserving from `Running` when explicitly disclosed.
- `HookInvocationFailedClosed`: should not by itself mark the run failed unless paired with a reviewed `RunFailed` transition.
- `HookInvocationBlocked`: should not invent a new blocked status unless a future status model is accepted.

Do not add post-terminal transitions in the first event vocabulary implementation.

Do not add new runtime statuses unless a separate status-model phase is accepted.

## 10. Failure Semantics Policy

Future hook failures must be explicit.

Recommended defaults:

- `Passed`: may allow the scoped executor action to continue.
- `Warning`: may allow continuation only if the warning is durable and reportable.
- `SkippedWithDisclosure`: may allow continuation only if the skip is explicit and reportable.
- `FailedClosed`: should block or fail the scoped action unless a policy-approved safer pause exists.
- `Blocked`: should pause, escalate, or fail according to a separately accepted policy.

For future pre-side-effect checkpoints, warning-only behavior is unsafe unless policy, audit, and report disclosure requirements are implemented.

The current `BeforeReport` hook failure remains a report-generation-side error and must not change the workflow result.

## 11. Audit Sink Emission Policy

Do not emit hook audit sink records automatically until event/source semantics are accepted.

Future audit options:

1. Project hook workflow events through `AuditEvent::from_workflow_event(...)`.
2. Add a dedicated hook audit sink method for `AgentHarnessHookAuditRecord`.
3. Persist hook audit records in a hook-specific store and cite them from reports.

Recommended sequencing:

1. Model-only hook workflow event vocabulary.
2. Review.
3. Event projection/audit sink planning.
4. Optional audit sink model changes.
5. Executor integration for one pre-terminal checkpoint.

No event source, no audit projection.

## 12. Idempotency And Replay Policy

Future event-producing hooks must define:

- deterministic hook invocation ID policy;
- idempotency key strategy;
- duplicate run behavior;
- replay behavior;
- whether hook execution is re-run or prior event state is reused;
- whether hook audit records are reconstructed from events or stored separately.

The current explicit `BeforeReport` path may re-run in-memory hook validation on duplicate report-bearing calls because it writes no events or records. That behavior is acceptable only for the current non-mutating path.

State-visible hook checkpoints must not re-run silently on replay.

## 13. Relationship To Policy Gates And Approvals

Hooks must not replace deterministic policy or human approval.

Future hook events may cite policy or approval references when those references already exist. They must not:

- create policy decisions;
- request approvals;
- grant or deny approvals;
- bypass approval gates;
- infer approval from model self-review;
- change autonomy level.

Pre-skill hook checkpoints must run only after policy ordering is designed.

## 14. Relationship To Local Checks And Adapters

Hooks may cite stable local check result references or adapter telemetry references supplied by callers.

Hooks must not:

- run `DocsCheck`;
- register local check handlers;
- execute shell commands;
- invoke adapters;
- call providers;
- create command-output evidence;
- treat missing local checks as passing.

Any hook that wants local check output must depend on a separately executed and governed local check result reference.

## 15. Relationship To EvidenceReference And WorkReports

Future hook events and audit records may be cited by WorkReports when stable IDs exist.

They must not:

- create `EvidenceReference` values implicitly;
- copy evidence payloads;
- copy hook disclosures or raw context into report summaries;
- fabricate missing hook evidence;
- treat hook success as proof of validation unless validation references exist.

WorkReports should continue to cite hook invocation IDs or future hook event IDs by stable reference only.

## 16. Privacy And Redaction

Future hook event and audit semantics must:

- use validated ID types;
- use bounded redaction metadata;
- reject secret-like field names and reasons;
- redact `Debug` output;
- serialize only stable references and bounded metadata;
- fail closed on invalid serialized payloads;
- avoid leaking raw values in errors.

Forbidden payloads include:

- raw prompts;
- raw spec contents;
- raw command output;
- raw command transcripts;
- provider payloads;
- CI logs;
- Jira or GitHub raw bodies;
- parser payloads;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded summaries.

## 17. First Implementation Target Status

The first code-bearing phase after review was **hook workflow event vocabulary model, model-only**, and it is implemented.

That phase added only the smallest model surface necessary to represent future hook workflow events and transition names. It did not wire hooks into `LocalExecutor`, append events, emit audit records, persist hook records, expose CLI behavior, add schemas, run local checks, invoke adapters, model side effects, add writes, or change release posture.

Implemented scope:

- added `HookInvocationRequested` and `HookInvocationEvaluated` event kind names;
- added a bounded hook event payload model;
- added state transition rules only for non-terminal, state-preserving hook events from `Running`;
- added serde/redaction tests;
- added runtime event tests proving terminal states still reject post-terminal hook events;
- kept executor behavior unchanged.

The next phase should review the model-only hook workflow event vocabulary before any audit sink projection or executor event append behavior is planned.

## 18. Test Plan For Future Implementation

Future model-only tests should cover:

- hook event kind names are representable;
- valid minimal hook event payload;
- invalid hook invocation ID rejected;
- invalid contract ID/version rejected;
- invalid hook kind rejected where applicable;
- invalid or secret-like references rejected without leaking;
- duplicate named references rejected if named references are included;
- redaction metadata validates;
- debug output redacts IDs and caller text;
- serialization does not include raw payload markers;
- invalid serialized payload fails closed;
- state-preserving hook events are valid only from accepted non-terminal statuses;
- terminal states reject hook events;
- hook events do not execute hooks;
- hook events do not emit audit sink records;
- hook events do not persist hook audit records;
- hook events do not run local checks, commands, or adapters;
- existing runtime, local executor, WorkReport, hook, EvidenceReference, local check, and adapter tests continue to pass.

## 19. Open Questions

- Should hook workflow events cite `AgentHarnessHookInvocationId` only, or embed a bounded hook audit record summary?
- Should hook audit records be projected from hook workflow events or stored separately?
- Should `BeforeReport` ever support a post-terminal metadata event, or should it remain report-path/audit-store-only?
- Should failed-closed hook events be followed by `RunFailed`, `RunPaused`, or `EscalationTriggered`?
- Should warning hooks be allowed before side-effect-adjacent checkpoints?
- Should hook event IDs become WorkReport citations, or should WorkReports keep citing hook invocation IDs?
- Should hook events include output references or only counts until evidence completeness rules exist?
- Should hook events be allowed from `WaitingForApproval`, `Retrying`, or `Escalated`?
- How should duplicate hook invocation IDs behave across replay?

## 20. Final Recommendation

Proceed next to **executor hook event append planning**.

That planning phase is now documented in [Executor Hook Event Append Plan](executor-hook-event-append-plan.md). It must not add executor hook broadening, event append behavior in `LocalExecutor`, dedicated hook audit sink emission, hook persistence, CLI behavior, workflow schema fields, automatic local checks, command execution, adapter invocation, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.
