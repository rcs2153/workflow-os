# Terminal Report Agent Harness Hook Citation Integration Report

Report date: 2026-06-16

## 1. Executive Summary

Implemented terminal local WorkReport helper support for explicitly supplied agent harness hook invocation IDs. `TerminalLocalWorkReportInput` now accepts `agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>`, and the helper converts supplied IDs into `WorkReportCitationTarget::AgentHarnessHook` citations through existing `WorkReportCitation` constructors.

The integration places hook citations in `ValidationAndQualityChecks` alongside validation diagnostic and local check result citations. It does not invoke hooks, discover hook IDs automatically, create hook invocation IDs, create hook invocation results, create hook audit records, append workflow events, emit audit sink records, persist hook records, change executor report input APIs, change report artifact behavior, expose CLI behavior, add workflow schema fields, model side effects, add writes, introduce recursive agents, introduce agent swarms, or change release posture.

## 2. Scope Completed

- Added `agent_harness_hook_invocation_ids` to `TerminalLocalWorkReportInput`.
- Added terminal report helper citation construction for `AgentHarnessHookInvocationId`.
- Added `WorkReportCitationTarget::AgentHarnessHook` citations through existing `WorkReportCitation::new(...)`.
- Attached hook citations to `ValidationAndQualityChecks`.
- Preserved existing validation diagnostic and local check citations in `ValidationAndQualityChecks`.
- Preserved existing behavior when no hook invocation IDs are supplied.
- Kept executor report input propagation for hook IDs deferred by passing no hook IDs from existing executor paths.
- Added focused tests for hook citation section placement, kind/target mapping, coexistence with validation/local check citations, absence behavior, and non-copying of hook payload markers.

## 3. Scope Explicitly Not Completed

- No automatic hook citation wiring.
- No runtime hook execution.
- No executor-integrated hook invocation.
- No executor report input propagation for hook IDs.
- No hook invocation ID creation.
- No hook invocation result creation.
- No hook audit record creation.
- No workflow event kinds.
- No workflow event append behavior.
- No audit sink emission.
- No hook audit record persistence.
- No report artifact behavior changes.
- No CLI behavior.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No automatic local check execution.
- No default local check handler registration.
- No command-output evidence.
- No EvidenceReference creation or attachment.
- No approval evidence attachment.
- No reasoning lineage implementation.
- No side-effect boundary implementation.
- No writes.
- No recursive agents.
- No agent swarms.
- No release posture change.

## 4. Helper API Summary

`TerminalLocalWorkReportInput` now includes:

```rust
pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>
```

Callers must provide already-validated hook invocation IDs. The helper does not accept hook audit records, hook invocation results, raw string IDs, or hook payloads.

Existing executor-integrated report paths do not expose this field yet. They pass no hook invocation IDs into the terminal helper, preserving the planned separation between helper support and executor report input propagation.

## 5. Citation Construction Summary

The terminal report helper constructs hook citations as:

```rust
WorkReportCitationTarget::AgentHarnessHook {
    hook_invocation_id,
}
```

Citations are created through `WorkReportCitation::new(...)` with bounded generic summary text. The implementation uses the report helper's existing sensitivity and redaction metadata.

The helper does not fabricate hook IDs, create hook audit records, create EvidenceReference values, or copy hook disclosures, hook named references, workflow IDs, run IDs, actor IDs, output summaries, raw context, provider payloads, parser payloads, command output, raw spec contents, environment values, credentials, authorization headers, private keys, or token-like values.

## 6. Section Population Summary

Hook citations are attached to `WorkReportSectionKind::ValidationAndQualityChecks`.

Summary behavior is deterministic:

- no validation, local check, or hook references: explicit not-available text;
- validation references only: validation reference text;
- local check references only: local check reference text;
- hook references only: hook reference text;
- mixed inputs: summary text names the supplied categories.

Existing validation diagnostic and local check result citations remain in the same section when hook citations are supplied.

## 7. Workflow Semantics Summary

The helper remains explicit, local, deterministic, and in-memory.

The implementation does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- invoke hooks;
- call `invoke_agent_harness_hook(...)`;
- create hook invocation results;
- create hook audit records;
- touch a `StateBackend`;
- persist reports;
- create filesystem artifacts;
- expose CLI output;
- change executor behavior.

## 8. Redaction And Privacy Summary

Hook citation debug output remains protected by existing `WorkReportCitation` and `WorkReportCitationTarget` debug redaction. Report debug output does not expose hook invocation IDs.

Serialization includes valid hook invocation IDs as stable citation references, but does not include hook audit record fields, hook disclosures, hook input/output payloads, provider payloads, command output, parser payloads, raw spec contents, environment values, credentials, authorization headers, private keys, or token-like values.

## 9. Test Coverage Summary

Added focused tests for:

- generated reports citing supplied hook invocation IDs in `ValidationAndQualityChecks`;
- hook citations using `WorkReportCitationTarget::AgentHarnessHook`;
- hook citations mapping to `WorkReportCitationKind::AgentHarnessHook`;
- hook citations preserving existing validation diagnostic and local check citations;
- absence of hook IDs preserving existing validation section behavior;
- generated report debug output not leaking hook invocation IDs;
- serialization not copying hook audit record fields or raw payload markers.

Existing terminal report helper, WorkReport, WorkReportContract, Agent Harness Hook, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, and docs tests remain covered by the validation commands below.

## 10. Commands Run And Results

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

## 11. Remaining Known Limitations

- Executor-integrated report input propagation for hook invocation IDs is not implemented.
- Runtime hook execution is not implemented.
- Hook workflow events are not implemented.
- Audit sink emission for hook records is not implemented.
- Hook audit records are not persisted.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.

## 12. Recommended Next Phase

Recommended next phase: **terminal report helper hook citation integration review**.

That review should verify the helper input, section placement, citation construction, non-leakage behavior, executor-boundary deferral, and documentation honesty before executor report input propagation for hook IDs is planned.
