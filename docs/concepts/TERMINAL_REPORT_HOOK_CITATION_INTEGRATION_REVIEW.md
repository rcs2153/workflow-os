# Terminal Report Agent Harness Hook Citation Integration Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted; proceed to executor hook report input propagation planning.

The phase implemented the intended terminal local WorkReport helper integration for explicitly supplied agent harness hook invocation IDs. The implementation is narrow, uses existing `WorkReportCitation` construction, places hook citations in `ValidationAndQualityChecks`, preserves behavior when no hook IDs are supplied, and does not introduce runtime hook execution, executor hook propagation, workflow events, audit sink emission, persistence, CLI behavior, schema changes, side effects, writes, recursive agents, agent swarms, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- `TerminalLocalWorkReportInput::agent_harness_hook_invocation_ids`;
- terminal report helper citation construction for `AgentHarnessHookInvocationId`;
- `WorkReportCitationTarget::AgentHarnessHook` citations through existing `WorkReportCitation::new(...)`;
- section placement in `WorkReportSectionKind::ValidationAndQualityChecks`;
- focused tests for citation construction, section placement, coexistence with validation and local check citations, absence behavior, and non-copying of hook payload markers;
- documentation updates and a phase report.

No accidental scope expansion was found:

- no automatic hook citation wiring;
- no runtime hook execution;
- no executor-integrated hook invocation;
- no executor report input propagation for hook IDs;
- no hook invocation ID creation;
- no hook invocation result creation;
- no hook audit record creation;
- no workflow event kinds;
- no workflow event append behavior;
- no audit sink emission;
- no hook audit record persistence;
- no report artifact behavior changes;
- no CLI behavior;
- no workflow schema fields;
- no workflow-declared hook configuration;
- no automatic local check execution;
- no default local check handler registration;
- no command-output evidence;
- no `EvidenceReference` creation or attachment;
- no approval evidence attachment;
- no reasoning lineage implementation;
- no side-effect boundary implementation;
- no writes;
- no recursive agents;
- no agent swarms;
- no release posture change.

## 3. Helper Input Assessment

The helper input is appropriately explicit and bounded.

`TerminalLocalWorkReportInput` now accepts `Vec<AgentHarnessHookInvocationId>`, not raw strings, hook invocation results, or hook audit records. This keeps validation at the existing typed identifier boundary and avoids accepting hook payload-bearing structures.

Existing executor-integrated report paths pass `Vec::new()` for hook invocation IDs. That preserves the intended separation between helper support and future executor input propagation.

## 4. Citation Construction Assessment

Citation construction is appropriate for this phase.

Verified:

- hook citations are created by `agent_harness_hook_citations(...)`;
- each citation target is `WorkReportCitationTarget::AgentHarnessHook { hook_invocation_id }`;
- citations are constructed through `WorkReportCitation::new(...)`;
- summary text is bounded and generic: `Agent harness hook checkpoint reference considered.`;
- report sensitivity and redaction metadata flow through the existing citation constructor;
- hook invocation IDs are cited as stable references;
- the helper does not create `EvidenceReference` values;
- the helper does not create hook invocation IDs, invocation results, audit records, workflow events, or audit sink records;
- the helper does not fabricate missing hook IDs.

## 5. Section Placement Assessment

The first section placement is acceptable.

Hook citations are placed in `ValidationAndQualityChecks` alongside validation diagnostic and local check result citations. This is the lowest-risk placement because the helper receives only hook invocation IDs, not hook kind, hook status, decision semantics, evidence semantics, or handoff semantics.

The implementation preserves existing validation diagnostic and local check citations when hook citations are supplied. When no validation, local check, or hook references are supplied, the section emits explicit not-available text instead of silently omitting the section.

Future phases may route hook citations to other sections once hook kind/status/context is represented, but doing that now would overclaim semantics the helper does not have.

## 6. Workflow Semantics Assessment

Workflow semantics remain unchanged.

Verified:

- `generate_terminal_local_work_report(...)` remains explicit, local, deterministic, and in-memory;
- `expose_terminal_local_work_report_result(...)` remains in-memory only;
- executor report input propagation for hook IDs is intentionally deferred;
- existing executor paths pass no hook IDs;
- no `WorkflowRun` mutation was introduced;
- no `WorkflowRunSnapshot` mutation was introduced;
- no workflow events are appended;
- no audit or observability events are emitted;
- no hook execution is invoked;
- no `StateBackend` write is introduced;
- no report artifact behavior changes were introduced;
- no CLI output is introduced.

## 7. Privacy And Redaction Assessment

The privacy posture is acceptable.

Verified:

- hook citations store only validated hook invocation IDs as stable references;
- `WorkReport` debug output does not expose hook invocation IDs;
- serialization includes the stable hook invocation ID, as expected for a citation reference;
- serialization does not include hook audit record fields;
- serialization does not copy hook disclosures, input references, output references, workflow/run context, actor context, output summaries, raw context, raw prompts, raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values;
- deserialization behavior for invalid hook citation targets remains covered by the existing `WorkReportCitationTarget::AgentHarnessHook` tests.

## 8. Test Quality Assessment

Test coverage is focused and adequate.

Covered:

- generated reports cite supplied hook invocation IDs in `ValidationAndQualityChecks`;
- hook citations use `WorkReportCitationTarget::AgentHarnessHook`;
- hook citations map to `WorkReportCitationKind::AgentHarnessHook`;
- hook citations preserve validation diagnostic citations;
- hook citations preserve local check result citations;
- absence of hook IDs preserves deterministic section text;
- generated report debug output does not leak hook invocation IDs;
- serialization does not copy hook audit record fields or raw payload markers;
- local executor report tests were updated to reflect the new explicit not-available wording;
- existing WorkReport, WorkReportContract, Agent Harness Hook, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, CLI, and docs tests pass through full workspace validation.

Non-blocking test follow-up:

- When executor hook report input propagation is implemented, add tests proving executor-supplied hook IDs reach the report while missing hook IDs remain explicit not-available text.

## 9. Documentation Review

Documentation is mostly aligned and honest.

Verified docs state:

- terminal report helper hook citation integration is implemented for explicitly supplied IDs only;
- executor report input propagation for hook IDs is not implemented;
- runtime hook execution is not implemented;
- hook workflow events are not implemented;
- audit sink emission is not implemented;
- hook audit record persistence is not implemented;
- CLI behavior is not implemented;
- workflow schema fields are not implemented;
- side-effect modeling and writes are not implemented;
- recursive agents and agent swarms are not introduced.

One tiny documentation correction was made during this review: `docs/implementation-plans/work-report-hook-citation-target-plan.md` still described terminal helper hook consumption as not implemented. The review updated that status to reflect the later bounded helper integration while preserving the runtime/executor non-goals.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Plan executor report input propagation for hook invocation IDs.
- Keep runtime hook execution separate from report citation propagation.
- Consider context-aware report section routing only after hook kind/status semantics are represented.
- Consider explicit missing-citation records only after contract-driven missing citation semantics are accepted.
- Review public compatibility before exposing this citation shape through generated schemas or stable CLI output.

## 12. Recommended Next Phase

Recommended next phase: **executor hook report input propagation planning**.

The helper can now cite explicitly supplied hook invocation IDs. The next bounded question is whether and how executor-adjacent report input types should accept those IDs and pass them into the helper without invoking hooks, mutating runtime state, appending events, persisting records, changing schemas, or changing workflow semantics.

## 13. Validation

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
