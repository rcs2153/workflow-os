# Agent Harness Hook Runtime Execution Plan

Status: Explicit in-memory runtime hook execution helper implemented. Agent harness hook contract, in-memory invocation result, hook audit record, WorkReport hook citation target, terminal report helper hook citation integration, executor hook report input propagation, and `execute_runtime_agent_harness_hook(...)` are implemented. Executor hook checkpoint planning is documented in [Executor Hook Checkpoint Plan](executor-hook-checkpoint-plan.md), the explicit `BeforeReport` executor checkpoint is implemented for `execute_with_report(...)` only, follow-on event/audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](executor-hook-event-audit-semantics-plan.md), the model-only hook workflow event vocabulary is implemented for future bounded, state-preserving hook events, and generic hook workflow event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](hook-event-audit-projection-plan.md). Broader automatic executor hook invocation, executor hook event append behavior, dedicated hook audit sink emission, persistence, CLI behavior, workflow schema fields, side-effect modeling, writes, recursive agents, agent swarms, and release posture changes are not implemented.

## 1. Executive Summary

Workflow OS now has the model foundations required to cite governed hook checkpoints:

- `AgentHarnessHookContract`;
- `AgentHarnessHookInvocationId`;
- `invoke_agent_harness_hook(...)` as an explicit in-memory validation helper;
- `AgentHarnessHookInvocationResult`;
- `AgentHarnessHookAuditRecord`;
- WorkReport hook citation vocabulary;
- terminal report helper hook citation integration;
- executor report input propagation for already-supplied hook invocation IDs.

The next question is how Workflow OS should execute deterministic hook checkpoints in a runtime-shaped way without turning hooks into arbitrary command execution, ambient agent orchestration, recursive agents, agent swarms, side effects, writes, or prose-only self-review.

This plan recommends a conservative first runtime execution slice: an explicit in-memory runtime hook execution service/helper that consumes a hook contract and explicit runtime context, calls the existing invocation validator, creates a model-only hook audit record, and returns both values to the caller. It should not append workflow events, emit audit sink records, persist hook records, mutate workflow state, write report artifacts, expose CLI behavior, or run commands. Automatic executor checkpoints should wait until this explicit service is implemented and reviewed.

## 2. Goals

- Define a governed runtime-shaped boundary for hook execution.
- Preserve the product model: `Agent executes. Workflow OS governs.`
- Reduce reliance on prose-only agent instruction following.
- Keep hook execution local, explicit, deterministic, and testable.
- Use existing hook contract, invocation result, and audit record constructors.
- Produce stable hook invocation IDs suitable for WorkReport citation.
- Preserve workflow pass/fail semantics until executor checkpoint semantics are separately accepted.
- Avoid raw payload copying.
- Avoid fake evidence, fake approvals, fake local check results, fake typed handoffs, fake audit events, or fake WorkReports.
- Prepare for later executor integration without implementing it now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic executor hook invocation;
- workflow schema fields;
- workflow-declared hook configuration;
- default hook registration;
- CLI hook commands;
- report artifact behavior changes;
- persistence;
- workflow event kinds;
- workflow event append behavior;
- audit sink emission;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- local shell command execution;
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

## 4. Current Baseline

Implemented:

- hook contract model and validation;
- explicit in-memory hook invocation validation helper;
- hook invocation result model;
- hook audit record model;
- WorkReport citation target for `AgentHarnessHookInvocationId`;
- terminal report helper support for supplied hook invocation IDs;
- executor report input propagation for supplied hook invocation IDs;
- explicit in-memory runtime hook execution helper.

Not implemented:

- automatic executor hook invocation;
- hook workflow events;
- hook audit sink emission;
- hook persistence;
- hook schema configuration;
- CLI hook commands.

## 5. First Execution Boundary Recommendation

Recommended first implementation: **explicit in-memory runtime hook execution service/helper**.

Candidate API names:

- `execute_agent_harness_hook_runtime(...)`;
- `execute_runtime_agent_harness_hook(...)`;
- `AgentHarnessHookRuntimeExecutor`;
- `RuntimeAgentHarnessHookInput`;
- `RuntimeAgentHarnessHookResult`.

The helper should:

1. Accept explicit inputs.
2. Validate the hook contract and context through `invoke_agent_harness_hook(...)`.
3. Accept or generate a stable `AgentHarnessHookInvocationId` according to an explicit caller-supplied policy.
4. Build `AgentHarnessHookAuditRecord::from_invocation_result(...)`.
5. Return an in-memory result containing the invocation ID, invocation result, and audit record.
6. Return structured non-leaking errors.

The helper should not:

- append workflow events;
- emit audit sink records;
- persist hook records;
- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- touch `StateBackend`;
- run local checks;
- run shell commands;
- invoke adapters;
- call external systems;
- write report artifacts;
- expose CLI output.

## 6. Runtime Input Model

A future input type should include:

- hook invocation ID or deterministic ID seed;
- hook contract;
- workflow ID;
- workflow version;
- run ID;
- schema version;
- spec hash;
- hook kind;
- actor or system actor;
- invoked timestamp;
- optional correlation ID;
- optional step ID;
- optional phase ID;
- supplied input references by stable name;
- supplied output references by stable name;
- supplemental stable references;
- bounded disclosures;
- redaction metadata;
- sensitivity;
- `require_outputs`;
- explicit side-effect requested flag, which must be rejected if true.

Inputs must be explicit. The helper must not read hidden global state or infer context from the current process, current directory, environment variables, workflow events, report notes, or agent messages.

## 7. Stable Identity Policy

The first runtime execution helper should prefer caller-supplied `AgentHarnessHookInvocationId` values.

Rationale:

- avoids ambient ID generation inside runtime paths;
- keeps replay and duplicate behavior explicit;
- lets callers cite the same ID in WorkReports;
- avoids pretending a durable event identity exists before hook events are implemented.

If deterministic ID derivation is later desired, it should be separately planned with a documented derivation rule using immutable run identity, hook contract identity, hook kind, phase/step identity, and attempt/index context.

## 8. Output Model

A future runtime result type should include:

- `hook_invocation_id`;
- `invocation_result`;
- `audit_record`;
- optional `report_citation_target` or a helper method to derive `WorkReportCitationTarget::AgentHarnessHook`;
- no persisted artifact handle;
- no workflow event ID;
- no audit event ID unless audit sink emission is separately implemented.

The output should be in-memory only. It should not imply persistence, event emission, audit sink emission, or report artifact generation.

## 9. Executor Integration Posture

Do not wire hooks automatically into `LocalExecutor` in the first runtime execution implementation.

Reasons:

- hook placement relative to `RunCreated`, `RunValidated`, `StepScheduled`, policy decisions, approval requests, skill invocation, retries, escalation, cancellation, and terminal events needs a separate accepted design;
- hook failure semantics can change workflow outcomes;
- post-terminal metadata-only events are currently disallowed;
- automatic checkpoints without schema/config support would be ambient behavior;
- hook execution must not become arbitrary command execution.

Executor integration should be a later phase after the explicit runtime helper is implemented and reviewed.

## 10. Candidate Future Executor Checkpoints

Future executor integration may consider:

- `before_run_start`;
- `after_run_validated`;
- `before_step_scheduled`;
- `before_skill_invocation`;
- `after_skill_success`;
- `after_skill_failure`;
- `before_approval_request`;
- `after_approval_decision`;
- `before_terminal_report`;
- `after_terminal_report`.

Do not implement these checkpoints in the first runtime execution slice.

Each checkpoint must later define:

- exact event ordering;
- idempotency key behavior;
- whether failure blocks, warns, pauses, fails, or records only;
- whether a hook record is appended before or after policy evaluation;
- whether approval is required for failure overrides;
- snapshot projection behavior;
- retry behavior;
- terminal-state restrictions;
- WorkReport citation behavior.

## 11. Failure Semantics

The first runtime execution helper should fail closed at the hook execution boundary when:

- the hook contract is invalid;
- the hook kind does not match the contract;
- required input references are missing;
- required output references are missing when `require_outputs` is true;
- references are invalid, duplicate, unbounded, or secret-like;
- disclosures are invalid, duplicate, unbounded, or secret-like;
- redaction metadata is invalid;
- side effects are requested;
- any constructor returns a validation error.

These failures should return structured non-leaking errors to the caller. They should not automatically fail, pause, cancel, or mutate a workflow run.

Future executor integration must separately decide how hook failures map to workflow behavior.

## 12. Audit And Event Posture

First runtime execution implementation:

- creates an in-memory `AgentHarnessHookAuditRecord`;
- does not append workflow events;
- does not emit audit sink records;
- does not persist the audit record;
- does not create audit event IDs;
- does not project hook records into `AuditEvent`.

Future audit/event phases may decide whether hook execution produces:

- dedicated hook workflow events;
- audit sink records;
- persisted hook records;
- report artifact references;
- WorkReport citations.

No event source means no event projection. The model-only audit record remains an in-memory governed record until persistence/event semantics are separately implemented.

## 13. Policy And Approval Boundary

Hooks may cite supplied policy and approval references, but the first runtime execution helper must not create them.

The helper must not:

- request approvals;
- decide approvals;
- bypass policy gates;
- create policy decisions;
- treat model self-review as policy;
- convert missing policy/approval references into success.

Future executor integration should decide whether hook execution itself requires policy evaluation.

## 14. Local Check Boundary

Hooks may cite local check result references already produced by explicit local check APIs.

The runtime hook execution helper must not:

- register local check handlers;
- run `DocsCheck`;
- run npm, cargo, shell, or arbitrary commands;
- create local check results;
- create command-output evidence;
- treat missing local check results as successful checks.

Missing required local check context should fail closed if the hook contract requires it.

## 15. Evidence And WorkReport Boundary

Hooks may cite supplied `EvidenceReference` IDs and may produce hook invocation IDs that reports can cite.

The runtime hook execution helper must not:

- create `EvidenceReference` values implicitly;
- attach evidence to diagnostics, adapters, approvals, reports, or hooks;
- copy evidence payloads;
- generate WorkReports;
- write report artifacts;
- render reports in CLI output.

The existing executor report input propagation can cite hook invocation IDs only when callers supply them.

## 16. Privacy And Redaction

The runtime hook execution helper must not store or copy:

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

Debug output must redact IDs and caller-supplied text where existing models require it. Serialization, if exposed for the result type, must be bounded and redaction-safe. Errors must use stable codes and must not leak raw values.

## 17. Test Plan

Future implementation tests should cover:

- valid runtime hook execution returns in-memory result and audit record;
- caller-supplied hook invocation ID is preserved;
- hook kind mismatch fails closed;
- missing required input fails closed;
- missing required output fails closed when outputs are required;
- invalid hook contract fails closed;
- side-effect request is rejected;
- secret-like references fail without leakage;
- duplicate named references fail closed;
- local check result references are accepted by stable reference only;
- EvidenceReference IDs are accepted by stable reference only;
- typed handoff IDs are accepted by stable reference only;
- policy and approval references are accepted only when supplied;
- absent optional references do not fabricate citations;
- no workflow state mutation;
- no workflow events appended;
- no audit sink records emitted;
- no `StateBackend` writes;
- no local checks executed;
- no adapters invoked;
- no files created;
- no CLI output emitted;
- no raw provider/spec/command/parser payload copied;
- Debug output does not leak hook context;
- serialization does not leak forbidden payload markers if serialization exists;
- existing hook, WorkReport, executor, evidence, diagnostic, validation, adapter telemetry, runtime, and docs tests still pass.

## 18. Proposed Implementation Sequence

Recommended small phases:

1. Runtime hook execution service/helper, explicit in-memory only.
2. Maintainer review.
3. Executor hook checkpoint planning.
4. Optional explicit `BeforeReport` executor hook implementation for one low-risk report-path checkpoint.
5. Hook workflow event planning.
6. Optional hook workflow event model and projection.
7. Hook audit sink emission planning.
8. Hook persistence planning.
9. Workflow schema planning only after runtime/API behavior is reviewed.
10. CLI hook commands only after schema/API behavior is stable.

## 19. Open Questions

- Should the first runtime helper require caller-supplied hook invocation IDs, or allow deterministic ID derivation?
- What is the smallest useful dogfood checkpoint: before validation, after validation, before report, or after report?
- Should hook failures ever pause for approval rather than fail closed?
- Should warning hooks be allowed before fail-closed hooks?
- Should hooks be policy-evaluated as actions before invocation?
- Should hook audit records eventually be persisted directly, projected from workflow events, or both?
- Should WorkReports cite hook invocation IDs or future hook audit record IDs?
- Should hook contracts eventually include first-class evidence, local check, approval, policy, or handoff requirements?
- How should duplicate hook execution be deduplicated during executor replay?
- What public compatibility guarantees apply before hook execution reaches schemas, SDKs, or CLI?

## 20. Final Recommendation

Proceed next with **executor hook checkpoint plan review**.

After review, the next implementation should be an explicit `BeforeReport` executor hook integration on the report-bearing path only. It must not add executor hooks to `execute(...)`, append workflow events, emit audit sink records, persist hook records, run local checks, invoke adapters, execute commands, write files, expose CLI behavior, add schema fields, authorize side effects, add writes, implement reasoning lineage, enable recursive agents, enable agent swarms, or change release posture.
