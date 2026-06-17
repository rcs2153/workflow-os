# Executor Hook Checkpoint Plan Review

Review date: 2026-06-17

## 1. Executive Verdict

Plan accepted; proceed to explicit `BeforeReport` executor hook integration implementation.

The plan is appropriately conservative, grounded in the current `LocalExecutor` lifecycle, and ready to drive a narrow implementation. It correctly recommends `BeforeReport` on the explicit report-bearing path as the first executor-integrated hook checkpoint because that placement can reuse the existing report error boundary without mutating workflow state, appending post-terminal events, changing approval or policy semantics, invoking local checks, invoking adapters, executing commands, writing report artifacts, or changing release posture.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not authorize:

- automatic executor hook invocation in this phase;
- hook workflow events;
- audit sink emission for hooks;
- hook persistence;
- report artifact writes;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime hook config;
- CLI hook commands;
- automatic local check execution;
- command execution;
- adapter invocation;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 3. Checkpoint Boundary Assessment

The plan defines a clear checkpoint boundary and avoids overfitting hooks to every executor event.

The current executor lifecycle includes pre-run policy, run creation/validation/start, step scheduling, approval request/resume, invoke policy, skill invocation, retry, escalation, cancellation, completion, and report-bearing result construction. The plan accurately identifies that most of these locations are too semantically sensitive for a first hook integration.

The recommendation to keep the existing phase-oriented `AgentHarnessHookKind` vocabulary is sound. It avoids adding executor-event-specific hook kinds before workflow schema, runtime event, and audit compatibility questions are settled.

## 4. First Target Assessment

The selected first target, `before_terminal_report`, is the right low-risk implementation slice.

It is appropriate because:

- it runs after a terminal `WorkflowRun` exists;
- it can be scoped to `execute_with_report(...)`;
- it does not affect `execute(...)`;
- it can preserve run status and event history on hook failure;
- it can use the existing `LocalExecutionWithReportResult` report-error shape;
- it can cite successful hook invocation IDs in `WorkReport`;
- it does not need post-terminal workflow events;
- it does not gate local skill side effects.

The plan correctly defers `before_skill_invocation`, `after_skill_success`, `after_skill_failure`, approval-adjacent hooks, retry hooks, escalation hooks, and state-mutating checkpoints until event, audit, idempotency, and failure semantics are separately designed.

## 5. Runtime Semantics Assessment

The plan preserves current workflow semantics.

It explicitly requires:

- `execute(...)` remains unchanged;
- `execute_with_report(...)` remains explicit;
- hook execution failure does not change run status;
- hook execution failure does not append events;
- hook execution failure does not emit audit or observability records;
- hook execution failure does not write artifacts;
- hook execution failure is returned as a report-path error beside the run.

This is aligned with the existing executor boundary where report generation errors remain separate from workflow execution errors.

## 6. Event And Audit Assessment

The plan correctly rejects hook workflow events and audit sink emission for the first implementation.

That is important because the current runtime rejects mutating events after terminal states, and the first recommended checkpoint runs after the workflow run is terminal. Adding hook events here would require a new event model and transition policy. The plan avoids that and keeps hook audit records in-memory for now.

Future event-producing checkpoint phases are correctly required to define event kind names, sequence placement, state transitions, snapshot projection, idempotency, audit sink projection, observability projection, replay behavior, and terminal-state restrictions.

## 7. Idempotency Assessment

The plan's caller-supplied hook invocation ID recommendation is acceptable for the first implementation.

It avoids implicit executor ID derivation before replay and duplicate-run semantics are designed. The deferred deterministic derivation inputs are also appropriate: run ID, workflow ID, workflow version, spec hash, hook contract ID/version, hook kind, step or phase identity, and attempt index where applicable.

The implementation phase should test duplicate run rehydration explicitly to ensure hook execution is not repeated when `execute_with_report(...)` returns an existing terminal run.

## 8. Policy And Approval Assessment

The plan correctly keeps the first report-path checkpoint away from policy and approval mutation.

It does not allow hooks to:

- create policy decisions;
- request approvals;
- decide approvals;
- bypass policy;
- infer approval state from prose;
- convert hook status into approval success;
- replace deterministic policy or human approval with model self-review.

Future pre-skill hook work will need a separate policy-ordering design before implementation.

## 9. Local Check And Adapter Boundary Assessment

The plan preserves the local check and adapter boundaries.

It allows supplied local check result references to be cited, but it rejects automatic local check execution, handler registration, npm/cargo/shell execution, local check result creation, command-output evidence, and treating missing checks as success.

It also rejects adapter invocation and external provider calls. This keeps hooks from becoming an ambient command or integration execution layer.

## 10. Report Citation Assessment

The report citation policy is correct.

Successful hook invocation IDs may be forwarded into terminal report generation and cited with `WorkReportCitationTarget::AgentHarnessHook`. The plan correctly prohibits copying hook audit record payloads, disclosures, named references, output summaries, workflow IDs, run IDs, actor IDs, or raw context into report sections by default.

Failed hook invocations must not fabricate citations.

## 11. Privacy And Redaction Assessment

The privacy posture is strong and consistent with prior phases.

The plan forbids storing or copying raw prompts, raw spec contents, command output, command transcripts, provider payloads, CI logs, Jira/GitHub raw bodies, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded summaries, and hook execution transcripts.

It also requires stable non-leaking errors and redaction-safe debug output.

## 12. Test Plan Assessment

The future test plan is adequate for the first implementation.

It covers:

- unchanged `execute(...)`;
- unchanged `execute_with_report(...)` when no hook inputs are supplied;
- completed, failed, and canceled terminal run cases;
- non-terminal runs not invoking report hooks;
- hook ID forwarding into report citations;
- run/event preservation on hook failure;
- no workflow events;
- no audit or observability emission;
- no report artifact writes;
- no snapshot mutation;
- side-effect request rejection;
- missing input failure;
- secret-like input failure without leakage;
- no local check, adapter, or command execution;
- no raw payload copying;
- existing workspace behavior.

One implementation note: tests should prove whether hook execution happens before or after duplicate-run rehydration. The safer expected behavior is that a duplicate run should rehydrate and then either skip hook execution unless explicitly requested for report construction, or execute only as a report-path helper without mutating runtime state. The implementation prompt should make this behavior explicit.

## 13. Documentation Review

The plan and linked docs now clearly say:

- executor hook checkpoint planning is documented;
- explicit in-memory runtime hook execution helper exists;
- automatic executor hook invocation is not implemented;
- hook workflow events are not implemented;
- audit sink emission is not implemented;
- persistence is not implemented;
- report artifact writing is not implemented for hooks;
- CLI behavior is not implemented;
- workflow schema fields are not implemented;
- automatic local checks are not implemented;
- side-effect modeling is not implemented;
- writes remain unsupported;
- recursive agents and agent swarms are not introduced;
- release posture is unchanged.

## 14. Planning Blockers

No planning blockers.

## 15. Non-Blocking Follow-Ups

- The implementation prompt should choose one bounded input shape, preferably a single optional `before_report_hook` before supporting a vector.
- The implementation prompt should define whether hook failure uses `report_generation_error` directly or wraps it in a hook-specific report-path error.
- The implementation should test duplicate run behavior so report-path hooks do not create hidden replay semantics.
- A later planning phase must define hook workflow events before any state-mutating checkpoint is implemented.

## 16. Recommended Next Phase

Recommended next phase: **explicit `BeforeReport` executor hook integration implementation, in-memory only**.

That implementation should be report-path-only, use caller-supplied hook contract/input values, call `execute_runtime_agent_harness_hook(...)`, forward successful hook invocation IDs into report generation, preserve run status and event history on hook failure, and return hook failure through the report-path error channel.

It must not add hooks to `execute(...)`, workflow events, audit sink emission, persistence, report artifact writes, CLI behavior, schemas, automatic local checks, command execution, adapter invocation, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 17. Validation

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.
