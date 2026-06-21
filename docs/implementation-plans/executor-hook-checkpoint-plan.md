# Executor Hook Checkpoint Plan

Status: Planning complete, first implementation slice complete, implementation review accepted, deterministic required-checkpoint enforcement expanded for explicit `BeforeReport` report paths, required `BeforeSkillInvocation` checkpoint planning documented in [BeforeSkillInvocation Required Checkpoint Plan](before-skill-required-checkpoint-plan.md), the first explicit selected-step required `BeforeSkillInvocation` enforcement slice implemented in [BeforeSkillInvocation Required Checkpoint Enforcement Report](../concepts/BEFORE_SKILL_REQUIRED_CHECKPOINT_ENFORCEMENT_REPORT.md), and the unknown required-step blocker fixed in [BeforeSkillInvocation Required Checkpoint Blocker Fix Report](../concepts/BEFORE_SKILL_REQUIRED_CHECKPOINT_BLOCKER_FIX_REPORT.md). This plan defines executor checkpoint placement for agent harness hooks. The explicit `BeforeReport` report-path integration is implemented as an in-memory-only executor report hook, explicit report-bearing callers can require that checkpoint before report generation as documented in [Deterministic Hook Checkpoint Enforcement Report](../concepts/DETERMINISTIC_HOOK_CHECKPOINT_ENFORCEMENT_REPORT.md), follow-on event/audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](executor-hook-event-audit-semantics-plan.md), model-only hook workflow event vocabulary is implemented, and generic hook workflow event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](hook-event-audit-projection-plan.md). Required `BeforeSkillInvocation` checkpoint behavior is implemented only for explicit selected step IDs supplied through local execution request inputs. Broader automatic executor hook invocation, executor hook event append broadening, dedicated hook audit sink emission, persistence, CLI behavior, workflow schema fields, automatic local checks, command execution, adapter invocation, side-effect modeling, writes, recursive agents, agent swarms, hosted execution, and release posture changes are not implemented.

## 1. Executive Summary

Workflow OS now has the prerequisites for a bounded hook checkpoint design:

- agent harness hook contract model;
- explicit in-memory hook invocation validation;
- model-only hook audit records;
- explicit in-memory runtime hook execution helper;
- WorkReport citation vocabulary for hook invocation IDs;
- terminal report helper and executor report input propagation for supplied hook invocation IDs.

The first implementation slice now answers the narrow question of where a low-risk report-path hook can sit in the `LocalExecutor` lifecycle.

This plan recommends a conservative checkpoint sequence. The first implementation does not place hooks before local skill execution, policy decisions, approval requests, retries, escalations, cancellations, or durable event projection. The implemented checkpoint is an explicit report-path checkpoint before terminal report construction, in-memory only, using caller-supplied hook contract/input values. Explicit report-bearing callers may now require that checkpoint; if required input is absent, report generation fails closed while the workflow run and event history remain unchanged. More authoritative checkpoints should wait until event semantics, failure behavior, idempotency, and schema/config posture are separately accepted.

## 2. Goals

- Define named executor checkpoint locations without implementing them.
- Preserve current workflow pass/fail semantics.
- Preserve current event ordering and state transitions.
- Preserve policy-before-side-effect behavior.
- Preserve approval semantics.
- Preserve retry, escalation, cancellation, and terminal-state behavior.
- Keep hook invocation explicit, local, deterministic, and testable.
- Keep hook records report-citable by stable IDs.
- Avoid raw payload copying.
- Avoid fake evidence, fake approvals, fake audit events, fake local check results, fake typed handoffs, and fake WorkReports.
- Identify the smallest next implementation phase.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic executor hook invocation;
- workflow event kinds for hooks;
- hook event append behavior;
- audit sink emission for hooks;
- hook persistence;
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
- reasoning lineage implementation;
- side-effect boundary implementation;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Executor Lifecycle

The current `LocalExecutor` lifecycle is:

1. load and validate the project;
2. prepare the execution plan;
3. evaluate pre-run policy before `RunCreated`;
4. append `RunCreated`;
5. append `RunValidated`;
6. append `RunStarted`;
7. schedule the current step with `StepScheduled`;
8. if required, evaluate approval-request policy and append `ApprovalRequested`;
9. on approval resume, append `ApprovalGranted`, record resume policy, and append `RunResumed`;
10. evaluate invoke policy before local skill invocation;
11. append `SkillInvocationRequested`;
12. append `SkillInvocationStarted`;
13. invoke the registered local handler;
14. append `SkillInvocationSucceeded` or `SkillInvocationFailed`;
15. on retryable failure, append `RetryScheduled` and `RetryStarted`;
16. on retry exhaustion, append `RetryExhausted`;
17. if escalation is configured, append `EscalationTriggered`;
18. otherwise append `RunFailed`;
19. after the last successful step, append `RunCompleted`;
20. cancellation appends `RunCanceled`;
21. `execute_with_report(...)` derives an in-memory report after execution returns a run.

Important existing constraints:

- terminal states reject further mutating workflow events;
- policy audit records can exist before `RunCreated`, but no workflow event stream exists yet;
- report generation errors are separate from workflow execution errors;
- report generation does not append workflow events or write artifacts automatically;
- hook invocation helper results are in-memory only.

## 5. Checkpoint Vocabulary Recommendation

Use the existing phase-oriented `AgentHarnessHookKind` vocabulary as the initial hook language:

- `BeforePlanning`;
- `AfterPlanning`;
- `BeforeImplementation`;
- `AfterImplementation`;
- `BeforeValidation`;
- `AfterValidation`;
- `BeforeReview`;
- `AfterReview`;
- `BeforeReport`;
- `AfterReport`.

Do not introduce executor-event-specific hook kinds yet.

Rationale:

- the hook model already uses phase-oriented terms familiar to agent harness users;
- forcing a hook onto every workflow event would overfit the current local executor implementation;
- future schemas can map authored phases to executor checkpoints later;
- phase-level hooks avoid implying that every runtime event is externally configurable.

## 6. Candidate Executor Checkpoints

Candidate future checkpoints:

| Checkpoint | Executor location | Initial assessment |
| --- | --- | --- |
| `before_run_start` | before `RunCreated` | Defer. No workflow event stream exists yet, and failure behavior could prevent run creation. |
| `after_run_started` | after `RunStarted` | Defer. Requires hook event/audit ordering and failure semantics. |
| `before_step_scheduled` | before `StepScheduled` | Defer. Could affect step progression and idempotency. |
| `after_step_scheduled` | after `StepScheduled` | Defer. Requires event ordering and step-level context design. |
| `before_approval_request` | before `ApprovalRequested` | Defer. Must not bypass policy or fabricate approval context. |
| `after_approval_decision` | after `ApprovalGranted` or `ApprovalDenied` | Defer. Requires approval/hook failure semantics. |
| `before_skill_invocation` | before `SkillInvocationRequested` or `SkillInvocationStarted` | Defer. This is high authority because it gates side-effect-adjacent behavior. |
| `after_skill_success` | after `SkillInvocationSucceeded` | Defer. Requires output reference, retry, and continuation semantics. |
| `after_skill_failure` | after `SkillInvocationFailed` | Defer. Requires retry/escalation ordering design. |
| `before_terminal_report` | after terminal run exists, before report helper construction | Recommended first. In-memory report path only, no workflow mutation. |
| `after_terminal_report` | after in-memory report helper succeeds | Defer. Terminal states reject post-terminal events, and artifact/report persistence is separate. |

## 7. First Implementation Target Recommendation

Implemented first executor-integrated checkpoint: **`before_terminal_report`, in-memory only**.

Mapping:

- use `AgentHarnessHookKind::BeforeReport`;
- run only on the explicit report-bearing path, not on `execute(...)`;
- execute after `LocalExecutor::execute(...)` returns a terminal `WorkflowRun`;
- execute before `expose_terminal_local_work_report_result(...)`;
- accept explicit hook contract/input values supplied through a new executor report input field or wrapper;
- call `execute_runtime_agent_harness_hook(...)`;
- pass successful hook invocation IDs into terminal report generation;
- preserve the run if hook execution fails;
- return hook execution failure as a report-generation-side error, not as a workflow execution error.

Why this is first:

- the workflow run already exists and is terminal;
- no post-terminal event append is required;
- no skill side effect is gated by a hook;
- current report-bearing execution already has a separate report error channel;
- WorkReport citation for hook invocation IDs already exists;
- absent hook inputs can preserve current behavior.

This is still not automatic hook execution for every workflow. It is an explicit executor report-path integration for caller-supplied hook inputs.

## 8. Explicit Inputs For First Implementation

A future implementation should add a narrow explicit input such as:

```rust
pub before_report_hook: Option<LocalExecutionBeforeReportHookInput>
```

or:

```rust
pub report_hooks: Vec<LocalExecutionReportHookInput>
```

The input should include:

- `AgentHarnessHookInvocationId`;
- `AgentHarnessHookContract`;
- explicit named input references;
- explicit named output references if required;
- supplemental stable references;
- bounded disclosures;
- redaction metadata;
- sensitivity;
- `require_outputs`;
- explicit `side_effect_requested`, which must be rejected when true.

The input must not include:

- raw prompts;
- raw specs;
- raw command output;
- provider payloads;
- parser payloads;
- local check command transcripts;
- environment values;
- credentials;
- tokens;
- unbounded summaries.

## 9. Event Ordering Policy

For the first implementation target:

- do not append workflow events;
- do not append post-terminal events;
- do not add hook workflow event kinds;
- do not project hook records into `WorkflowRunSnapshot`;
- do not emit audit sink records;
- do not write hook audit records to `StateBackend`.

Future event-producing checkpoints must define:

- event kind names;
- sequence placement;
- state transition behavior;
- snapshot projection behavior;
- idempotency keys;
- audit sink projection;
- observability projection;
- replay and duplicate behavior;
- terminal-state restrictions.

## 10. Failure Semantics Policy

For `before_terminal_report`:

- hook execution success may produce hook invocation IDs for report citation;
- hook execution failure must not change the run status;
- hook execution failure must not append events;
- hook execution failure must not emit audit/observability records;
- hook execution failure must not write report artifacts;
- hook execution failure should be returned as a report-path error beside the run.

For future pre-skill or step checkpoints, failure semantics must be separately designed before implementation. Candidate behaviors include:

- fail closed and append `RunFailed`;
- warning-only and continue;
- pause for approval;
- escalate;
- skip with disclosure;
- block before side effects.

Do not use warning-only behavior before there is a reviewed way to disclose it in reports and audit records.

## 11. Idempotency Policy

The first `before_terminal_report` implementation should use caller-supplied `AgentHarnessHookInvocationId` values.

Do not derive hook IDs implicitly in the executor yet.

Future deterministic derivation, if needed, should use immutable inputs such as:

- run ID;
- workflow ID;
- workflow version;
- spec hash;
- hook contract ID/version;
- hook kind;
- step ID or phase ID if applicable;
- attempt index if applicable.

Derivation must be documented and tested before use in replay-sensitive executor paths.

## 12. Policy And Approval Boundary

The first report-path checkpoint must not:

- create policy decisions;
- request approvals;
- decide approvals;
- bypass policy;
- infer approval state from prose;
- convert hook status into approval success;
- allow model self-review to replace deterministic policy or human approval.

Future pre-skill checkpoints may need policy evaluation before hook invocation, but that must be a separate design because it changes authorization ordering.

## 13. Local Check Boundary

Hooks may cite supplied local check result references.

Hooks must not:

- execute local checks automatically;
- register local check handlers;
- run npm, cargo, shell, or arbitrary commands;
- create local check result records;
- create command-output evidence;
- treat missing checks as success.

If a hook contract requires a local check reference and it is absent, hook validation should fail closed without mutating the workflow.

## 14. Report Citation Policy

For the first report-path checkpoint:

- successful hook invocation IDs should be forwarded into `TerminalLocalWorkReportInput`;
- generated reports should cite those IDs with `WorkReportCitationTarget::AgentHarnessHook`;
- citation summaries must be bounded and generic;
- hook audit record payloads must not be copied into the report;
- hook disclosures, named references, output summaries, workflow IDs, run IDs, actor IDs, and raw context must not be copied into report sections by default;
- failed hook invocations should not fabricate citations.

## 15. Privacy And Redaction

Executor checkpoint inputs and outputs must not store or copy:

- raw prompts;
- raw spec contents;
- raw command output;
- raw command transcripts;
- raw provider payloads;
- raw CI logs;
- raw Jira issue/comment bodies;
- raw GitHub file contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded natural-language summaries;
- hook execution transcripts.

Errors must use stable codes and must not leak raw IDs, paths, notes, disclosures, references, payloads, or secret-like values.

Debug output must redact caller-supplied IDs and text where existing model boundaries require redaction.

## 16. Future Test Plan

Tests for the first implementation should cover:

- `execute(...)` remains unchanged and does not invoke hooks;
- `execute_with_report(...)` without hook inputs remains unchanged;
- completed terminal runs can execute an explicit `BeforeReport` hook before report generation;
- failed terminal runs can execute an explicit `BeforeReport` hook before report generation;
- canceled terminal runs can execute an explicit `BeforeReport` hook before report generation;
- non-terminal report-bearing runs do not invoke report hooks;
- hook invocation IDs are forwarded into generated reports;
- reports cite hook invocation IDs through `WorkReportCitationTarget::AgentHarnessHook`;
- hook execution failure preserves the run and event history;
- hook execution failure returns a non-leaking report-path error;
- hook execution failure does not append workflow events;
- hook execution failure does not emit audit or observability records;
- hook execution failure does not write report artifacts;
- hook execution failure does not mutate snapshots;
- side-effect requests are rejected;
- missing required hook inputs fail closed;
- secret-like hook inputs fail without leakage;
- local checks are not executed automatically;
- adapters are not invoked;
- command execution does not occur;
- no raw provider/spec/command/parser payload is copied;
- existing hook, WorkReport, executor, evidence, diagnostic, validation, adapter telemetry, local check, runtime, and docs tests still pass.

## 17. Proposed Implementation Sequence

Recommended small phases:

1. Implement explicit `BeforeReport` executor report-path hook integration, in-memory only. Complete.
2. Review the `BeforeReport` integration.
3. Plan hook workflow event semantics before any state-mutating hook checkpoint.
4. Plan pre-skill hook policy and failure semantics before any side-effect-adjacent checkpoint.
5. Plan required `BeforeSkillInvocation` checkpoint behavior for explicit selected step IDs before implementation. Complete.
6. Implement one required pre-skill checkpoint slice only after event, audit, idempotency, and failure semantics are accepted. Complete for explicit selected step IDs.
7. Defer schema fields, CLI commands, default hook registration, persistence, artifacts, side effects, writes, recursive agents, agent swarms, and hosted behavior.

## 18. Open Questions

- Should the first implementation accept exactly one `BeforeReport` hook or a bounded vector?
- Should report-path hook failures become `report_generation_error`, a new hook-specific error field, or a composite report-path error?
- Should successful hook audit records ever be returned in `LocalExecutionWithReportResult`, or should only hook invocation IDs flow into reports?
- Should future hook events live in workflow event history or a separate audit ledger?
- Should future pre-skill hooks run before or after invoke-policy evaluation?
- How should hook idempotency behave during duplicate run rehydration?
- Should warning-only hooks be allowed before audit/report disclosure semantics exist?
- What compatibility posture applies before hook behavior is exposed through schemas, SDKs, or CLI?

## 19. Final Recommendation

Proceed next to **BeforeSkillInvocation required checkpoint enforcement review**.

The implemented required-checkpoint slices are explicit `BeforeReport` report-path enforcement and explicit selected-step `BeforeSkillInvocation` enforcement. The pre-skill slice remains opt-in, local-executor-only, and request-input-driven. It does not add broad automatic executor hooks, workflow-declared hook configuration, runtime hook configuration, audit sink emission, persistence, report artifact writing, CLI behavior, schemas, automatic local checks, command execution, adapter invocation, side-effect execution, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.
