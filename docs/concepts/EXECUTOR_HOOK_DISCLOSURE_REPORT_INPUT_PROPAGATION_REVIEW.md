# Executor Hook Disclosure Report Input Propagation Review

## 1. Executive Verdict

Phase accepted; proceed to hook disclosure discovery planning.

The implementation is narrow, additive, and consistent with the accepted plan. Executor-integrated report-bearing execution now accepts explicitly supplied `AgentHarnessHookDisclosureId` values and forwards them into terminal local WorkReports without executing hooks, discovering disclosures, creating disclosure records, appending workflow events, emitting audit sink records, writing report artifacts, changing schemas, adding side effects, enabling writes, implementing reasoning lineage, or changing release posture.

## 2. Scope Verification

The phase stayed within the approved executor report input propagation scope.

Implemented scope:

- `LocalExecutionReportInputs` accepts `agent_harness_hook_disclosure_ids`.
- `LocalExecutionReportInputs` `Debug` exposes only `agent_harness_hook_disclosure_count`.
- `terminal_report_input_for_run(...)` forwards supplied disclosure IDs into `TerminalLocalWorkReportInput`.
- Generated reports cite supplied hook disclosure IDs through the existing WorkReport citation path.
- Existing executor behavior is preserved when no hook disclosure IDs are supplied.
- Focused tests and documentation/report updates were added.

No accidental scope expansion was found:

- no runtime hook execution changes;
- no automatic hook disclosure discovery;
- no hook disclosure creation from reports;
- no hook invocation result creation from reports;
- no hook audit record creation or persistence;
- no hook disclosure workflow event append behavior;
- no audit sink emission;
- no warning, skipped, blocked, optional, or policy-controlled continuation behavior;
- no context-aware disclosure routing by kind or severity;
- no `EvidenceReference` creation or attachment;
- no approval evidence attachment;
- no automatic report artifact writing;
- no CLI hook commands or report rendering;
- no workflow schema fields;
- no automatic local check execution;
- no command execution or adapter invocation;
- no side-effect boundary implementation;
- no writes;
- no reasoning lineage;
- no recursive agents, agent swarms, hosted behavior, or release posture changes.

## 3. API Assessment

The API change is the smallest useful executor-adjacent addition:

```rust
pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>
```

This mirrors the existing hook invocation ID propagation pattern and keeps the boundary typed. The executor accepts already-validated IDs instead of raw strings or full disclosure payloads. It does not accept disclosure titles, summaries, references, redaction metadata, hook context, hook audit records, workflow event payloads, or persistence handles.

The field is additive to `LocalExecutionReportInputs`. Existing executor methods remain unchanged.

## 4. Propagation Assessment

The propagation implementation is appropriately direct:

```rust
agent_harness_hook_disclosure_ids: report.agent_harness_hook_disclosure_ids.clone()
```

That keeps the executor as an explicit input bridge between `LocalExecutionReportInputs` and `TerminalLocalWorkReportInput`. It does not introduce hidden global state, runtime config, storage reads, disclosure existence checks, or automatic discovery from hook events, audit records, invocation results, report notes, local checks, typed handoffs, validation diagnostics, or persistence.

## 5. Report Behavior Assessment

Generated reports use the existing citation model:

- citation target: `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- citation kind: `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- section placement: `ValidationAndQualityChecks`;
- summary behavior: bounded and generic through the terminal report helper.

The behavior is useful for future governed handoffs because report-bearing executor callers can now surface explicit hook disclosure references alongside validation, local check, hook invocation, typed handoff, adapter telemetry, evidence, policy, approval, audit, and workflow event references.

Absent hook disclosure IDs preserve current no-disclosure behavior. The implementation does not fabricate missing disclosure IDs or missing-citation records.

## 6. Workflow Semantics Assessment

The executor semantics remain unchanged.

The implementation still:

- calls `execute(&request.execution)` first;
- returns execution errors unchanged;
- preserves the run when report generation fails after execution;
- keeps report generation errors separate from workflow execution errors;
- does not mutate `WorkflowRun` or `WorkflowRunSnapshot`;
- does not append workflow events for hook disclosure propagation;
- does not emit audit or observability events for hook disclosure propagation;
- does not invoke hooks because disclosure IDs were supplied;
- does not call `execute_runtime_agent_harness_hook(...)`;
- does not create hook disclosures, hook invocation results, or hook audit records;
- does not touch `StateBackend` beyond existing executor execution behavior;
- does not write report artifacts automatically;
- does not expose CLI output;
- does not change `execute(...)`, `decide_approval(...)`, or `cancel_run(...)`.

Focused tests compare backend events to the returned run events and verify no report artifacts are written automatically.

## 7. Debug And Redaction Assessment

`LocalExecutionReportInputs` `Debug` exposes only a count:

```rust
agent_harness_hook_disclosure_count
```

Tests cover that request debug includes the count field but does not leak the supplied hook disclosure ID. Result debug also does not leak the disclosure ID.

Generated WorkReport serialization includes the stable disclosure ID as a citation target. That is consistent with the existing WorkReport reference model. The implementation does not serialize disclosure titles, summaries, hook audit markers, hook input/output payload markers, raw provider payloads, raw command output, raw spec contents, parser payloads, credentials, tokens, or secret-like values.

## 8. Privacy And Payload Assessment

The executor report input remains reference-only.

No evidence was found that the implementation copies:

- hook disclosure title or summary;
- hook disclosure references;
- hook disclosure redaction metadata;
- hook input or output references;
- supplemental hook references;
- hook audit records;
- hook invocation results;
- hook contracts;
- workflow/run/actor context from hook records;
- hook output summaries;
- raw prompts;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

The privacy posture is appropriate for this phase: stable IDs are carried as report citations, while disclosure payloads remain outside the executor report input boundary.

## 9. Error Handling Assessment

Invalid raw hook disclosure values fail before this executor propagation boundary because callers must construct `AgentHarnessHookDisclosureId` values.

If report generation fails after execution, existing `LocalExecutionWithReportResult` behavior preserves the run, returns `work_report: None`, and carries a structured report-generation error separately. The implementation does not convert report-generation failures into workflow diagnostics and does not append workflow events or audit records for report-generation failures.

No new leaking error surface was introduced.

## 10. Relationship To Existing Concepts

The implementation remains aligned with:

- Governed Work Pattern: explicit references, bounded report handoff, no ambient authority.
- EvidenceReference: reports cite references without copying evidence payloads or creating evidence references implicitly.
- WorkReport: generated reports remain derived handoff artifacts, not audit logs.
- Agent harness hooks: hook disclosure IDs are cited; hook execution and automatic discovery remain separate future concerns.
- Hook event/audit vocabulary: no fabricated audit or workflow event history is introduced.
- Typed handoffs and Composable Harness Contracts: this supports traceable handoff posture without adding nested harness execution.

## 11. Test Quality Assessment

Test coverage is strong for the implemented boundary.

Covered:

- `LocalExecutionReportInputs` construction with hook disclosure IDs;
- `execute_with_report(...)` forwarding supplied disclosure IDs;
- generated citation target and kind;
- placement in `ValidationAndQualityChecks`;
- coexistence with hook invocation citations;
- absence behavior for no supplied disclosure IDs;
- count-only request debug behavior;
- result debug non-leakage;
- generated report serialization includes stable ID citation but not disclosure payload markers;
- no workflow event mutation;
- no automatic report artifact writes;
- existing WorkReport, hook disclosure, hook invocation, EvidenceReference, Diagnostic, validation, adapter telemetry, local-check, and runtime tests through the full workspace suite.

No blocker-level test gaps were found.

Non-blocking test follow-ups:

- Add a future regression test if serializer support is ever added to executor report input request types.
- Add referential-integrity tests only after a durable disclosure store exists.
- Add section-routing tests only after warning/skipped disclosure semantics define routing rules.

## 12. Documentation Review

Docs accurately state:

- terminal report helper hook disclosure citation integration is implemented;
- executor-integrated report-bearing execution forwards explicitly supplied hook disclosure IDs;
- the implementation is input propagation only;
- automatic hook disclosure discovery is not implemented;
- runtime hook behavior changes are not implemented;
- warning/skipped/blocked continuation behavior is not implemented;
- event append behavior and audit sink emission are not implemented;
- persistence and report artifact behavior changes are not implemented;
- CLI rendering, workflow schemas, side effects, writes, reasoning lineage, hosted behavior, recursive agents, agent swarms, and release posture changes are not implemented.

The end-of-phase report records the commands run and passed.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Plan automatic hook disclosure discovery only after durable disclosure, event, or audit semantics are clear.
- Decide whether future disclosure citations need referential integrity against a disclosure store.
- Decide whether disclosure kind or severity should affect WorkReport section placement.
- Decide whether warning/skipped disclosures should become continuation semantics or remain report-only references.
- Consider whether approval-resume and cancellation report-bearing APIs should eventually accept hook disclosure references.

## 15. Recommended Next Phase

Recommended next phase: **hook disclosure discovery planning**.

The explicit citation path now exists from model vocabulary through terminal report helper and executor report input propagation. Before implementing automatic discovery, maintainers should decide the source of truth for disclosures, whether discovery reads workflow events, hook audit records, a future disclosure store, or report inputs, and how to avoid fabricating runtime history or weakening the current reference-only safety boundary.

## 16. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
