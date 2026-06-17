# Terminal Report Agent Harness Hook Citation Integration Plan Report

Report date: 2026-06-16

## 1. Executive Summary

Created the planning document for future terminal report helper integration of supplied agent harness hook invocation ID citations. The plan recommends adding explicit `AgentHarnessHookInvocationId` inputs to `TerminalLocalWorkReportInput`, constructing `WorkReportCitationTarget::AgentHarnessHook` citations through existing report citation constructors, and placing those citations in `ValidationAndQualityChecks`.

This phase is planning only. It does not implement terminal report helper hook citation integration, runtime hook execution, executor hook integration, workflow events, audit sink emission, hook audit record persistence, report artifact behavior changes, CLI behavior, workflow schema fields, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.

## 2. Scope Completed

- Added [Terminal Report Agent Harness Hook Citation Integration Plan](../implementation-plans/terminal-report-hook-citation-integration-plan.md).
- Recommended explicit `agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>` helper input for a future implementation.
- Recommended constructing hook citations with `WorkReportCitationTarget::AgentHarnessHook`.
- Recommended placing first hook citations in `ValidationAndQualityChecks`.
- Documented missing-reference, error-handling, workflow semantics, privacy, audit/event, and test boundaries.
- Updated `ROADMAP.md`.
- Updated [Governed Work Pattern](governed-work-pattern.md).
- Updated [Evidence Reference](evidence-reference.md).
- Updated [WorkReportContract Planning Document](../implementation-plans/work-report-contract-plan.md).

## 3. Scope Explicitly Not Completed

- No terminal report helper hook citation implementation.
- No automatic hook citation wiring.
- No runtime hook execution.
- No executor-integrated hook invocation.
- No executor report input propagation.
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

## 4. Planning Summary

The plan recommends a future helper input:

```rust
pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>
```

The future helper should create citations with:

```rust
WorkReportCitationTarget::AgentHarnessHook {
    hook_invocation_id,
}
```

The plan recommends `ValidationAndQualityChecks` as the first section placement because the helper would only receive stable hook invocation IDs, not hook kind, hook status, or full hook audit context.

## 5. Validation Boundary Summary

The future implementation should accept already validated `AgentHarnessHookInvocationId` values and rely on existing `WorkReportCitation::new(...)` validation for summary text, redaction metadata, sensitivity, and missing flags.

Invalid hook references must fail closed without leaking raw values. Citation failures must not mutate workflow state, append events, emit audit or observability records, persist reports, or change workflow pass/fail status.

## 6. Privacy And Redaction Summary

The plan prohibits copying hook audit records, hook disclosures, hook named references, hook input references, hook output references, supplemental hook references, workflow/run/actor context, raw prompts, provider payloads, command output, parser payloads, raw spec contents, environment values, credentials, authorization headers, private keys, or token-like values into report sections or citation summaries.

Serialization may include valid hook invocation IDs as stable references after implementation, but not hook audit payload fields. Debug output must remain redaction-safe through existing WorkReport citation debug boundaries.

## 7. Test Coverage Plan Summary

Future implementation should test:

- supplied hook invocation IDs are accepted by terminal report input;
- generated reports cite hook invocation IDs in `ValidationAndQualityChecks`;
- citations use `WorkReportCitationTarget::AgentHarnessHook`;
- citations map to `WorkReportCitationKind::AgentHarnessHook`;
- existing validation and local check citations are preserved;
- absent hook IDs preserve existing not-available section behavior;
- no hook invocation, hook result creation, hook audit record creation, event append, audit emission, persistence, CLI, schema, side-effect, or write behavior is introduced;
- Debug and serialization remain non-leaking.

## 8. Commands Run And Results

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 9. Remaining Known Limitations

- Terminal report helper support for supplied hook invocation IDs is not implemented.
- Executor-integrated report input propagation for hook invocation IDs is not implemented.
- Runtime hook execution is not implemented.
- Hook workflow events are not implemented.
- Audit sink emission for hook records is not implemented.
- Hook audit records are not persisted.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.

## 10. Recommended Next Phase

Recommended next phase: **terminal report helper hook citation integration implementation**.

That implementation should add explicit supplied hook invocation ID inputs, construct citations through existing WorkReport citation constructors, place those citations in `ValidationAndQualityChecks`, and add focused tests. It must not implement runtime hook execution, executor hook integration, workflow events, audit sink emission, persistence, report artifact behavior changes, CLI behavior, workflow schema fields, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.
