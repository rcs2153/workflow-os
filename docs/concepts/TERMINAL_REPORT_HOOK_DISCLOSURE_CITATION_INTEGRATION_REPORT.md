# Terminal Report Hook Disclosure Citation Integration Report

## 1. Executive Summary

The terminal local WorkReport helper now accepts explicitly supplied `AgentHarnessHookDisclosureId` values and cites them in generated in-memory reports.

This is a helper-level integration only. It does not propagate hook disclosure IDs through executor report inputs, discover disclosures automatically, create disclosures, append events, emit audit records, persist reports, write files, expose CLI output, change schemas, broaden hook statuses, add side effects, add writes, implement reasoning lineage, or change release posture.

## 2. Scope Completed

- Added explicit `agent_harness_hook_disclosure_ids` input to `TerminalLocalWorkReportInput`.
- Added bounded WorkReport citation construction for `WorkReportCitationTarget::AgentHarnessHookDisclosure`.
- Added hook disclosure citations to the `ValidationAndQualityChecks` section.
- Preserved existing behavior when no hook disclosure IDs are supplied.
- Preserved existing hook invocation, local check, and validation diagnostic citation behavior.
- Kept executor propagation deferred by passing an empty disclosure-ID list from the current executor-integrated report path.
- Added focused tests for citation construction, section placement, non-copying behavior, and mixed validation/local-check/hook/disclosure citations.

## 3. Scope Explicitly Not Completed

- Executor propagation for hook disclosure IDs is not implemented.
- Automatic hook disclosure discovery is not implemented.
- Hook disclosure creation from reports is not implemented.
- Hook invocation result creation from reports is not implemented.
- Hook audit record creation or persistence is not implemented.
- Workflow event append behavior is not implemented for disclosures.
- Audit sink emission is not implemented.
- Warning, skipped, blocked, or optional hook continuation is not implemented.
- Context-aware section routing by disclosure kind or severity is not implemented.
- Runtime report auto-generation is not implemented.
- Report artifact behavior, persistence, CLI rendering, schemas, examples, side-effect modeling, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, and release posture changes are not implemented.

## 4. Helper API Summary

`TerminalLocalWorkReportInput` now carries:

```rust
pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>
```

The helper accepts typed disclosure IDs only. It does not accept full `AgentHarnessHookDisclosure` values, raw string IDs, disclosure title, disclosure summary, disclosure references, disclosure redaction metadata, hook context, hook audit records, or workflow events.

The existing executor-integrated path remains unchanged at the public API boundary. It passes an empty disclosure ID list into the helper, so executor propagation is still deferred.

## 5. Citation Construction Summary

The helper constructs citations through the existing `WorkReportCitation::new(...)` path and uses:

```rust
WorkReportCitationTarget::AgentHarnessHookDisclosure { disclosure_id }
```

Each citation uses bounded generic summary text:

```text
Agent harness hook disclosure reference considered.
```

The helper does not create `AgentHarnessHookDisclosure` values, recreate `EvidenceReference` values, fabricate IDs, copy disclosure payloads, or inspect persisted disclosure records.

## 6. Section Placement Summary

Hook disclosure citations are placed in the `ValidationAndQualityChecks` section.

This placement is intentionally conservative because the helper receives disclosure IDs only. Without disclosure kind, severity, policy linkage, skipped context, or warning semantics, routing disclosures into risk, incomplete work, policy, approval, or handoff sections would overclaim what the runtime knows.

## 7. Workflow And Runtime Semantics Summary

The helper remains local and in-memory. This phase does not mutate `WorkflowRun`, mutate snapshots, append events, emit audit events, emit observability events, write state, create files, persist reports, or expose CLI output.

Report-generation failure remains separate from workflow execution semantics. The current executor path does not propagate disclosure IDs and therefore cannot fail due to hook disclosure citation construction.

## 8. Redaction And Privacy Summary

The helper stores stable typed disclosure IDs as citations and does not copy:

- disclosure title or summary;
- disclosure references;
- hook context;
- audit records;
- workflow event payloads;
- provider payloads;
- command output;
- parser output;
- raw spec contents;
- file contents;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

`Debug` output remains redaction-safe through existing WorkReport and WorkReportCitation behavior. Serialization of generated reports includes citation targets and stable IDs only, not disclosure payloads.

## 9. Test Coverage Summary

Focused tests cover:

- supplied hook disclosure IDs are cited through `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- disclosure citations are placed in `ValidationAndQualityChecks`;
- disclosure citations coexist with validation diagnostic, local check result, and hook invocation citations;
- generated reports without disclosure IDs preserve existing section text;
- disclosure citation serialization does not copy disclosure title, summary, checkpoint notes, hook inputs, hook outputs, audit payloads, provider payloads, command output, raw spec contents, or token-like strings;
- existing WorkReport helper behavior remains covered by the broader WorkReport test suite.

## 10. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test work_report` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 11. Remaining Known Limitations

- Executor report inputs cannot yet carry hook disclosure IDs.
- Disclosure IDs are not validated against a durable disclosure store.
- Reports do not automatically discover disclosures from hook invocation results, audit records, workflow events, or persistence.
- Disclosure kind and severity do not yet influence section placement.
- Warning, skipped, blocked, and optional hook continuation semantics remain deferred.
- Dedicated hook audit sink emission and hook persistence remain deferred.

## 12. Recommended Next Phase

Recommended next phase: **terminal report hook disclosure citation integration review**.

The helper-level behavior should be reviewed before executor propagation is planned or implemented. Executor propagation is the likely next implementation slice after review acceptance.
