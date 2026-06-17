# WorkReport Agent Harness Hook Citation Target Report

Report date: 2026-06-16

## 1. Executive Summary

Implemented model-only WorkReport citation vocabulary for agent harness hook invocation checkpoints. `WorkReportCitationKind` can now classify agent harness hook citations, and `WorkReportCitationTarget` can cite a validated `AgentHarnessHookInvocationId` without embedding hook audit records or copying hook context.

This phase does not implement terminal report helper hook citation integration, automatic hook citation wiring, runtime hook execution, executor hook integration, workflow events, audit sink emission, hook audit record persistence, CLI behavior, workflow schema fields, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.

## 2. Scope Completed

- Added `WorkReportCitationKind::AgentHarnessHook`.
- Added `WorkReportCitationTarget::AgentHarnessHook { hook_invocation_id: AgentHarnessHookInvocationId }`.
- Mapped agent harness hook citation targets to the new citation kind.
- Preserved existing `WorkReportCitation::new(...)` validation, redaction metadata validation, serde validation, and redaction-safe `Debug`.
- Added focused tests for valid construction, citation-kind mapping, serde round trip, invalid ID rejection, deserialization fail-closed behavior, and non-leakage.
- Updated roadmap and planning docs to state that hook citation vocabulary is implemented as model-only.

## 3. Scope Explicitly Not Completed

- No terminal report helper hook citation integration.
- No automatic hook citation wiring.
- No runtime hook execution.
- No executor-integrated hook invocation.
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
- No side-effect modeling.
- No writes.
- No approval evidence attachment.
- No reasoning lineage implementation.
- No recursive agents.
- No agent swarms.
- No release posture change.

## 4. Model Changes

The new citation target is:

```rust
WorkReportCitationTarget::AgentHarnessHook {
    hook_invocation_id: AgentHarnessHookInvocationId,
}
```

The target maps to:

```rust
WorkReportCitationKind::AgentHarnessHook
```

The target stores only the stable hook invocation ID. It does not store `AgentHarnessHookAuditRecord`, hook disclosures, hook named references, workflow/run context, actor context, command output, provider payloads, parser payloads, or raw spec contents.

## 5. Validation Boundary Summary

Hook citation IDs are validated by `AgentHarnessHookInvocationId::new(...)` and by serde deserialization of the ID type. Citation summaries, missing-citation flags, redaction metadata, and sensitivity continue to validate through `WorkReportCitation::new(...)`.

Invalid serialized hook citation targets fail closed through the same deserialization path. Validation errors use stable codes from the existing hook ID validation boundary and do not include raw invalid values.

## 6. Redaction And Privacy Summary

`WorkReportCitationTarget` debug output remains redaction-safe and prints the citation kind plus a redacted reference marker. `WorkReportCitation` debug output continues to redact summaries and redaction metadata values.

Serialization contains the stable hook invocation ID because it is the citation reference, but it does not serialize hook audit record fields, disclosures, input/output reference payloads, raw provider payloads, command output, parser payloads, raw spec contents, environment values, credentials, authorization headers, private keys, or token-like values.

## 7. Test Coverage Summary

Added focused tests covering:

- `WorkReportCitationKind::AgentHarnessHook` representation;
- `WorkReportCitationTarget::AgentHarnessHook` validation with a safe `AgentHarnessHookInvocationId`;
- citation-kind mapping;
- serde round trip;
- secret-like hook invocation ID rejection without value leakage;
- invalid serialized hook citation fail-closed behavior without value leakage;
- debug output redaction for hook invocation IDs and summaries;
- serialization does not copy hook audit payload markers;
- optional contract citation requirement representation.

Existing WorkReport, WorkReportContract, Agent Harness Hook, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, and docs tests remain covered by the workspace validation commands below.

## 8. Commands Run And Results

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

Recommended next phase: **WorkReport hook citation target vocabulary review**.

The review should verify the model-only citation vocabulary, validation, serde, debug redaction, non-leakage tests, and documentation honesty before any planning for terminal report helper hook citation integration.
