# Executor BeforeReport Hook Integration Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted; proceed to hook workflow event and audit semantics planning.

The implementation delivers the intended first executor-integrated hook checkpoint: an explicit, in-memory `BeforeReport` hook path scoped only to `LocalExecutor::execute_with_report(...)`. It preserves existing `execute(...)`, approval, cancellation, event-log, audit sink, report artifact, persistence, CLI, schema, local check, adapter, side-effect, write, recursive-agent, agent-swarm, hosted, and release-posture behavior.

## 2. Scope Verification

The phase stayed within the approved report-path-only implementation scope.

No accidental behavior was introduced for:

- hooks on `execute(...)`;
- automatic hooks for all runs;
- pre-skill, step, approval, retry, escalation, cancellation, or post-report hooks;
- hook workflow events;
- audit sink emission for hook records;
- observability emission for hook records;
- hook persistence;
- report artifact writes;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime hook configuration;
- CLI hook commands;
- automatic local check execution;
- command execution;
- adapter invocation;
- `EvidenceReference` creation or attachment;
- approval request or decision creation;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 3. API Assessment

The new API shape is appropriately narrow.

`LocalExecutionBeforeReportHookInput` carries a caller-supplied `AgentHarnessHookInvocationId` and explicit `AgentHarnessHookInvocationInput`. `LocalExecutionReportInputs` now accepts `before_report_hook: Option<LocalExecutionBeforeReportHookInput>`, keeping hook execution explicit and tied to report-bearing execution.

The new input exports through `workflow-core` consistently with the existing local executor and hook model exports. Debug output redacts hook IDs and workflow/run identity fields while exposing only counts and hook kind.

## 4. Runtime Integration Assessment

`LocalExecutor::execute_with_report(...)` preserves the established runtime boundary:

1. call existing `execute(...)`;
2. return execution errors unchanged if no run is produced;
3. only inspect `before_report_hook` after a run exists;
4. only execute the hook when the run is terminal;
5. validate hook kind and immutable run identity before hook execution;
6. call `execute_runtime_agent_harness_hook(...)`;
7. forward the successful hook invocation ID into terminal report inputs;
8. return hook failure as `run + no report + report_generation_error`.

This keeps hook failure on the report side of the API and does not retroactively change workflow pass/fail status.

## 5. Terminal Status Assessment

Completed and failed terminal runs are covered by focused tests and produce reports with hook citations when a valid `BeforeReport` hook input is supplied.

Non-terminal runs skip hook execution and return the existing `work_report_generation.status.not_terminal` report-path error. The test intentionally supplies a side-effect-requesting hook to prove the hook is not run before terminality is established.

Canceled report-bearing execution is not broadened by this phase. Cancellation report-bearing integration remains outside this checkpoint slice unless a future phase adds a report-bearing cancellation/resume path.

## 6. Validation Boundary Assessment

The implementation validates:

- hook kind must be `AgentHarnessHookKind::BeforeReport`;
- hook workflow ID must match the terminal run;
- hook workflow version must match the terminal run;
- hook run ID must match the terminal run;
- hook schema version must match the terminal run;
- hook spec hash must match the terminal run;
- the hook invocation itself passes the existing runtime hook execution helper.

Invalid hook kind and identity mismatch return stable non-leaking executor errors. Side-effect requests, missing required references, secret-like disclosures, and other hook model violations remain enforced by `execute_runtime_agent_harness_hook(...)`.

## 7. Event, Audit, And Persistence Assessment

The implementation does not append workflow events, mutate snapshots, emit audit sink records, emit observability records, write hook audit records, write report artifacts, touch CLI output, or introduce persistence.

Tests compare backend event history to the returned run events after successful hook execution, hook failure, non-terminal skip, and duplicate run/report behavior. Tests also verify audit and observability sinks do not receive hook-specific records and that report artifacts are not written.

## 8. Report Citation Assessment

Successful `BeforeReport` hook invocation IDs are cited through the existing WorkReport hook citation path. The implementation forwards only the stable `AgentHarnessHookInvocationId` into report generation and does not copy hook input references, output references, disclosures, audit records, workflow IDs, run IDs, actor IDs, provider payloads, or hook transcripts into the report.

Failed hook invocations do not fabricate citations because report generation is skipped and the hook error is returned through `report_generation_error`.

## 9. Privacy And Redaction Assessment

The privacy posture is consistent with earlier WorkReport and hook phases.

The implementation does not store or copy raw prompts, raw spec contents, command output, command transcripts, provider payloads, CI logs, Jira/GitHub raw bodies, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded summaries, or hook execution transcripts.

Debug output for report inputs, hook inputs, and result wrappers remains redaction-safe. Error paths use stable codes and avoid raw mismatched IDs or reference payload values.

## 10. Idempotency And Replay Assessment

Duplicate workflow execution still rehydrates the completed run without reinvoking the skill handler or appending events. The report path can re-run the explicit in-memory hook validation because no hook event, hook audit sink write, hook persistence, or report artifact write occurs in this phase.

This is acceptable for the current in-memory checkpoint. Durable hook idempotency, replay semantics, and whether a hook result should be reused rather than revalidated must be designed with future hook workflow event and audit semantics.

## 11. Test Quality Assessment

Focused tests cover:

- successful `BeforeReport` hook execution and citation;
- failed terminal run plus hook citation;
- non-terminal run skips hook execution;
- hook side-effect request failure preserves run status and events;
- hook identity mismatch fails without leaking the mismatched value;
- duplicate run/report execution does not append events or repeat skill execution;
- audit and observability sinks do not receive hook records;
- report artifacts are not written;
- existing hook ID propagation still works;
- existing report, local executor, runtime, WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, local check, and CLI tests pass.

Non-blocking gap: there is no dedicated canceled-run report-bearing hook test, because this phase does not add a report-bearing cancellation API. That should be considered if cancellation report exposure is added later.

## 12. Documentation Review

The roadmap and related planning docs state that:

- the explicit `BeforeReport` executor checkpoint is implemented for `execute_with_report(...)` only;
- broader automatic executor hook invocation is not implemented;
- hook workflow events are not implemented;
- audit sink emission is not implemented;
- persistence is not implemented;
- report artifact writing is not automatic;
- CLI hook behavior is not implemented;
- workflow schema fields are not implemented;
- automatic local checks are not implemented;
- side-effect modeling is not implemented;
- writes remain unsupported;
- recursive agents and agent swarms are not introduced;
- release posture is unchanged.

The implementation report accurately describes the completed scope and known limitations.

## 13. Blockers

No blockers.

## 14. Non-Blocking Follow-Ups

- Plan hook workflow event and audit semantics before adding any state-visible or pre-side-effect hook checkpoint.
- Decide whether future durable hook execution should reuse prior hook results on duplicate report-bearing calls.
- Add canceled-run hook coverage if a report-bearing cancellation path is introduced.
- Keep `BeforeReport` as a single explicit hook input until there is a reviewed need for a bounded vector.

## 15. Recommended Next Phase

Recommended next phase: **hook workflow event and audit semantics planning**.

That phase should define whether and how hook invocations become durable workflow events or audit records, how they are ordered, how replay and duplicate execution behave, and how failures interact with policy, approval, retry, escalation, terminal states, and WorkReport citation.

It must not implement broader automatic hooks, pre-skill hooks, persistence, CLI behavior, workflow schema fields, automatic local checks, command execution, adapter invocation, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 16. Validation

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
