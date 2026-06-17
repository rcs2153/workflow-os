# WorkReport Hook Disclosure Citation Target Report

## 1. Executive Summary

The WorkReport hook disclosure citation target phase is implemented as a model-only vocabulary slice.

WorkReport citations can now target a bounded `AgentHarnessHookDisclosureId` through a dedicated citation kind and target. This lets future report-generation phases cite a specific hook disclosure by stable ID without embedding hook disclosure payloads, hook context, audit records, workflow events, evidence records, command output, provider payloads, or raw summaries.

This phase does not wire hook disclosure IDs into terminal report helpers, executor report inputs, automatic runtime behavior, warning/skipped continuation, blocked behavior, hook optionality, persistence, schemas, CLI behavior, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture.

## 2. Scope Completed

- Added `WorkReportCitationKind::AgentHarnessHookDisclosure`.
- Added `WorkReportCitationTarget::AgentHarnessHookDisclosure`.
- Reused the existing validated `AgentHarnessHookDisclosureId` model as the only target payload.
- Preserved `WorkReportCitation::new(...)` as the validation boundary.
- Preserved redaction-safe `Debug` behavior for citation targets and citations.
- Added focused serde, validation, fail-closed, and non-leakage tests.
- Updated planning and concept docs to state that hook disclosure citation target vocabulary is implemented as model-only.

## 3. Scope Explicitly Not Completed

- Terminal report helper support for supplied hook disclosure IDs is not implemented.
- Executor report input propagation for hook disclosure IDs is not implemented.
- Direct WorkReport section population from hook disclosure kind or severity is not implemented.
- Warning continuation is not implemented.
- Skipped-with-disclosure continuation is not implemented.
- Blocked runtime behavior is not implemented.
- Hook optionality is not implemented.
- Policy-controlled continuation is not implemented.
- Hook disclosure creation from reports is not implemented.
- Automatic hook disclosure discovery is not implemented.
- Hook audit record persistence or dedicated audit sink emission is not implemented.
- Workflow event append behavior beyond previously accepted explicit paths is not changed.
- Report artifacts, persistence, schemas, CLI rendering, examples, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, and release posture changes are not implemented.

## 4. Model Changes

`WorkReportCitationKind` now includes:

```rust
AgentHarnessHookDisclosure
```

`WorkReportCitationTarget` now includes:

```rust
AgentHarnessHookDisclosure {
    disclosure_id: AgentHarnessHookDisclosureId,
}
```

The target maps to the new citation kind through `WorkReportCitationTarget::citation_kind()`.

## 5. Validation, Serde, And Debug Summary

Hook disclosure citation targets use typed `AgentHarnessHookDisclosureId` values. Invalid or secret-like disclosure IDs fail before a valid citation can be constructed or deserialized.

Serde round trips preserve the stable disclosure ID and citation kind. Invalid serialized disclosure IDs fail closed through the existing typed-ID deserialization path.

`Debug` output remains redaction-safe because `WorkReportCitationTarget` redacts target references and `WorkReportCitation` redacts summaries and redaction metadata.

## 6. Redaction And Privacy Summary

The citation target stores only a stable disclosure ID. It does not store:

- full `AgentHarnessHookDisclosure` values;
- disclosure title or summary;
- disclosure references;
- hook input or output references;
- supplemental hook references;
- hook audit records;
- workflow event payloads;
- raw provider payloads;
- raw command output;
- raw spec contents;
- raw parser payloads;
- environment values;
- credentials, authorization headers, private keys, or token-like values.

Report serialization includes the stable disclosure ID by design, but it does not serialize the disclosure payload.

## 7. Test Coverage Summary

Added focused WorkReport tests for:

- hook disclosure citation target validation;
- citation kind mapping to `AgentHarnessHookDisclosure`;
- serde round trip;
- secret-like disclosure ID rejection without leaking the rejected value;
- invalid serialized hook disclosure citation fail-closed behavior;
- debug output not leaking disclosure IDs or disclosure payload-like text;
- serialization not copying hook disclosure title, summary, references, hook context, provider payloads, command output, spec contents, or secret-like markers.

Existing WorkReport, hook invocation citation, EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests remain in the workspace validation set.

## 8. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test work_report` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- WorkReport helper inputs cannot yet accept hook disclosure IDs.
- Executor report-bearing execution cannot yet propagate hook disclosure IDs.
- Reports do not route hook disclosures by disclosure kind or severity.
- Hook disclosures are not persisted or discovered automatically.
- Warning, skipped, and blocked hook statuses remain deferred.
- Dedicated hook audit sink behavior remains deferred.

## 10. Recommended Next Phase

Recommended next phase: **WorkReport hook disclosure citation target review**.

The implementation is intentionally small and model-only. It should be reviewed before any terminal report helper or executor propagation phase accepts hook disclosure IDs.
