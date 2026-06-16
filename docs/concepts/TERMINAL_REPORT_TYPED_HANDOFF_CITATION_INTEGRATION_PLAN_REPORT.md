# Terminal Report Typed Handoff Citation Integration Plan Report

Post-plan status: terminal report helper typed handoff citation integration is now implemented and documented in [Terminal Report Typed Handoff Citation Integration Report](TERMINAL_REPORT_TYPED_HANDOFF_CITATION_INTEGRATION_REPORT.md). This planning report is retained as the planning record.

## 1. Executive Summary

This planning phase defines how terminal local report generation should later accept explicitly supplied typed handoff IDs and cite them in generated in-memory WorkReports.

The plan recommends a narrow future implementation: add `typed_handoff_ids: Vec<TypedHandoffId>` to the terminal report helper input and construct `WorkReportCitationTarget::TypedHandoff` citations in the `OperatorHandoffNotes` section. The implementation should not generate handoffs, discover handoffs automatically, persist handoffs, change executor semantics, change report artifact behavior, add schemas, expose CLI behavior, implement nested harness execution, model side effects, add writes, implement reasoning lineage, or change release posture.

## 2. Scope Completed

Completed:

- created terminal report typed handoff citation integration plan;
- defined helper input boundary;
- defined citation construction policy;
- recommended initial section placement;
- defined missing/unavailable reference policy;
- defined workflow semantics boundary;
- defined privacy and redaction requirements;
- defined future test plan;
- updated roadmap and related planning/concept docs.

## 3. Scope Explicitly Not Completed

Not implemented:

- terminal report helper typed handoff input;
- typed handoff citation construction inside report helpers;
- executor-integrated typed handoff report input propagation;
- automatic typed handoff citation discovery;
- runtime handoff generation;
- typed handoff persistence;
- nested harness execution;
- workflow schema fields;
- CLI rendering or export;
- report artifact behavior changes;
- EvidenceReference creation;
- reasoning lineage;
- side-effect boundary;
- writes;
- domain packs;
- release posture changes.

## 4. Planning Decision Summary

The plan recommends adding explicit helper input:

```rust
pub typed_handoff_ids: Vec<TypedHandoffId>
```

The plan recommends placing typed handoff citations in `OperatorHandoffNotes` for the first implementation. This keeps typed handoffs framed as governed transfer/handoff references rather than evidence payloads, risk disclosures, or incomplete-work claims.

## 5. Validation Boundary Summary

Future implementation should accept already validated `TypedHandoffId` values and construct citations through `WorkReportCitation::new(...)`.

It should not accept raw handoff payloads, `TypedHandoff` values, generic strings, or storage lookups. Citation construction failures should return structured, non-leaking `WorkflowOsError` values and should not change workflow pass/fail status.

## 6. Redaction And Privacy Summary

Future helper integration must cite typed handoff IDs only.

It must not copy typed handoff obligations, disclosures, risks, notes, endpoint names, reference names, raw provider payloads, raw command output, raw CI logs, raw Jira or GitHub bodies, raw spec contents, parser payloads, environment variable values, credentials, authorization headers, private keys, or token-like values.

## 7. Test Coverage Plan Summary

The future implementation should test:

- helper accepts supplied typed handoff IDs;
- generated report includes typed handoff citations;
- citations use `WorkReportCitationTarget::TypedHandoff`;
- citations map to `WorkReportCitationKind::TypedHandoff`;
- citations land in `OperatorHandoffNotes`;
- absence of IDs preserves existing section behavior;
- helper does not create or mutate typed handoffs;
- helper does not recreate EvidenceReference values;
- helper does not copy typed handoff payload fields;
- debug and serialization remain safe;
- existing report, executor, typed handoff, evidence, diagnostic, validation, adapter telemetry, and runtime tests still pass.

## 8. Commands Run And Results

Commands are listed in the final implementation response for this phase.

## 9. Remaining Known Limitations

- At planning time, terminal report helper typed handoff citation integration was not implemented. It is now implemented in the follow-up implementation phase.
- Executor-integrated report paths do not accept typed handoff IDs.
- Typed handoffs are not persisted.
- Report artifacts do not validate typed handoff referential integrity.
- Runtime handoff generation and nested harness execution remain unimplemented.

## 10. Recommended Next Phase

Recommended next phase at planning time: terminal report helper typed handoff citation integration implementation. That implementation phase is now complete.

The implementation should add explicit helper input and citation construction only. It should not add automatic citation discovery, runtime handoff generation, nested harness execution, typed handoff persistence, schemas, CLI behavior, report artifact behavior changes, side-effect modeling, writes, domain packs, reasoning lineage, or release posture changes.
