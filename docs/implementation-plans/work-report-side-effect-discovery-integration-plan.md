# WorkReport SideEffect Discovery Integration Plan

Status: Implemented for the first explicit WorkReport-side helper. Store-backed SideEffect discovery is implemented and accepted in [SideEffect Store-Backed Discovery Review](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md). WorkReport SideEffect citation vocabulary, terminal report helper propagation for explicitly supplied `SideEffectId` values, and executor report input propagation for explicitly supplied `SideEffectId` values are implemented and reviewed. The explicit WorkReport-side discovery helper is implemented in [WorkReport SideEffect Discovery Integration Report](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md). This plan does not implement automatic discovery, executor integration, report artifact integration, runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has the pieces needed for bounded SideEffect citation in WorkReports:

- `SideEffectId` and `SideEffectRecord` model types;
- `SideEffectRecordStore` persistence for validated records;
- explicit SideEffect workflow event vocabulary for proposed/denied/skipped events;
- an accepted in-memory SideEffect discovery helper;
- an accepted store-backed SideEffect discovery helper;
- WorkReport citation vocabulary for `SideEffectId`;
- terminal report helper and executor report-input propagation for explicitly supplied `SideEffectId` values.

The next question is how a report-generation path can opt into store-backed discovery without making every executor report automatically scan state, without broadening `StateBackend`, and without turning reports into side-effect ledgers.

This plan recommends a small explicit WorkReport-side discovery integration helper. The helper should accept an existing `SideEffectRecordStore`, an already-terminal `WorkflowRun`, existing report inputs, and a bounded discovery policy. It should merge discovered SideEffect IDs with caller-supplied `side_effect_ids`, then call the existing terminal WorkReport generator. It must not mutate runtime state, append events, create records, execute side effects, write artifacts, or change workflow semantics.

## 2. Goals

- Allow explicit report-generation code to opt into SideEffect discovery.
- Reuse `discover_side_effect_references_from_store` as the only store-backed discovery boundary.
- Preserve existing `generate_terminal_local_work_report(...)` behavior when discovery is not requested.
- Preserve existing `LocalExecutor::execute_with_report(...)` behavior.
- Keep the integration local, deterministic, and in-memory.
- Cite stable `SideEffectId` values only.
- Merge discovered IDs with caller-supplied SideEffect IDs deterministically.
- Preserve source-of-truth boundaries: records/events remain authoritative; reports cite.
- Keep discovery errors structured, stable, and non-leaking.
- Keep report-generation failure separate from workflow execution failure.
- Prepare for a later executor opt-in path without forcing it now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic SideEffect discovery in every report;
- changing `LocalExecutor::execute_with_report(...)`;
- adding `SideEffectRecordStore` as a supertrait of `StateBackend`;
- executor integration;
- report artifact integration;
- automatic artifact writing;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable GitHub, Jira, CI, HTTP, local filesystem, or generic adapters;
- provider mutation;
- SideEffect record creation, repair, update, or lifecycle transition;
- workflow-declared SideEffect configuration;
- runtime SideEffect configuration;
- workflow schema fields;
- CLI commands or rendering;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented and accepted:

- `WorkReportCitationTarget::SideEffect { side_effect_id }`;
- SideEffect citations in `WorkReportSectionKind::SideEffects`;
- `TerminalLocalWorkReportInput::side_effect_ids`;
- `LocalExecutionReportInputs::side_effect_ids`;
- `terminal_report_input_for_run(...)` forwarding explicitly supplied SideEffect IDs;
- `discover_side_effect_references` for explicit IDs, workflow events, and already-loaded records;
- `discover_side_effect_references_from_store` for explicit IDs, caller-supplied workflow events, and persisted records loaded through `SideEffectRecordStore`;
- local `SideEffectRecordStore` implementations.

Implemented:

- explicit WorkReport-side discovery from store-backed records before WorkReport generation, when directly requested by the caller.

Not implemented:

- automatic discovery from store-backed records during normal WorkReport generation;
- executor-level opt-in discovery;
- report artifact referential integrity checks;
- EvidenceReference side-effect attachment;
- runtime side-effect execution;
- write-capable adapters.

## 5. Integration Target Decision

The first implementation target should be a WorkReport-side helper, not an executor method.

Recommended shape:

```rust
pub struct TerminalLocalWorkReportSideEffectDiscoveryInput {
    pub include_workflow_events: bool,
    pub include_store_records: bool,
    pub require_records: bool,
}

pub fn generate_terminal_local_work_report_with_side_effect_discovery(
    store: &impl SideEffectRecordStore,
    input: TerminalLocalWorkReportInput<'_>,
    discovery: TerminalLocalWorkReportSideEffectDiscoveryInput,
) -> Result<WorkReport, WorkflowOsError>
```

Name and exact shape may vary with local conventions, but the first implementation should:

1. borrow the already-terminal run from `TerminalLocalWorkReportInput`;
2. build `SideEffectStoreBackedDiscoveryInput` using immutable run identity;
3. seed `explicit_side_effect_ids` from `input.side_effect_ids`;
4. include `run.events.clone()` only when explicitly requested;
5. call `discover_side_effect_references_from_store`;
6. merge discovered SideEffect IDs into `input.side_effect_ids`;
7. call `generate_terminal_local_work_report(input)`.

If `include_store_records` is false, a separate in-memory helper path may be used, but the first useful slice should include store records because store-backed discovery is the accepted primitive this phase follows.

## 6. Why Not Executor Integration First

`LocalExecutor` is generic over `StateBackend`. `SideEffectRecordStore` is a separate store trait. Calling store-backed discovery directly inside the existing `execute_with_report(...)` method would require either:

- broadening `StateBackend` to include `SideEffectRecordStore`; or
- adding an extra bound to an existing public method implementation surface; or
- branching into a more complex executor API.

That is too much for the first integration step.

The safer path is:

1. add an explicit WorkReport-side discovery helper that accepts `&impl SideEffectRecordStore`;
2. review it;
3. later plan an additive executor opt-in path, such as a new method or helper requiring `B: StateBackend + SideEffectRecordStore`.

Existing `LocalExecutor::execute_with_report(...)` should remain explicit-ID-only until that separate executor phase is scoped.

## 7. Discovery Policy

The discovery policy should be explicit and small.

Recommended fields:

- `include_workflow_events`: whether supported SideEffect workflow events from the supplied run should be considered;
- `include_store_records`: whether persisted SideEffect records should be loaded from the supplied store;
- `require_records`: whether every discovered ID must have a matching record.

Rules:

- At least one discovery source should be enabled.
- Store-backed discovery should use `SideEffectRecordStore::list_side_effect_records_for_workflow_run`.
- Workflow events must be the events already present on the supplied `WorkflowRun`.
- Supported workflow event states remain `Proposed`, `Denied`, and `Skipped`.
- `Attempted`, `Completed`, and `Failed` remain unsupported and counted by discovery, not cited.
- Absence of discovered IDs should not be treated as proof that no side effects existed.
- Missing optional records should remain bounded discovery metadata, not report prose with invented IDs.

## 8. Identity And Source-Of-Truth Rules

The helper must derive immutable identity from the supplied run:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID.

It must not trust:

- report text;
- local paths;
- index files alone;
- event prose;
- audit projections;
- adapter telemetry payloads;
- natural-language summaries;
- caller-provided duplicate identity fields.

Source-of-truth boundaries:

- workflow events remain source of truth for run history;
- `SideEffectRecord` remains source of truth for side-effect intent, authority, lifecycle state, idempotency, and outcome references;
- WorkReport remains a governed handoff artifact that cites stable references;
- EvidenceReference remains a separate evidence citation substrate.

## 9. Merge And Ordering Policy

Discovered IDs should be merged with existing `input.side_effect_ids`.

Rules:

- Caller-supplied explicit IDs keep first-source priority through the discovery helper.
- Workflow event IDs keep priority over store-record-only IDs.
- Store-record-only IDs are included after explicit/event identities.
- Final citation order should remain deterministic.
- Duplicate IDs should not produce duplicate citations.
- Existing terminal report helper citation construction should remain the only WorkReport citation construction path.

Implementation should prefer reusing `SideEffectDiscoveryResult::references()` in deterministic order and replacing or rebuilding the side-effect ID vector once, rather than appending in multiple places.

## 10. Error Handling

Discovery errors must remain report-generation errors, not workflow execution errors.

Rules:

- identity mismatch should fail with stable non-leaking discovery errors;
- corrupt records should fail closed;
- generic store read failures should fail closed;
- missing required records should fail closed;
- invalid report citation construction should fail through existing WorkReport validation;
- no partial `WorkReport` should be returned if discovery or citation construction fails;
- the original workflow run must not be modified;
- errors must not become user project diagnostics;
- errors must not include SideEffect IDs, workflow IDs, run IDs, target references, paths, record JSON, snippets, provider payloads, command output, credentials, tokens, or secret-like values.

When this helper is later used by an executor result path, failure should produce a report-generation error while preserving the run.

## 11. Runtime And State Boundary

The helper must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- create SideEffect records;
- update SideEffect records;
- repair corrupt records;
- transition SideEffect lifecycle state;
- emit audit events;
- emit observability events;
- execute side effects;
- call adapters or providers;
- write report artifacts;
- create files;
- expose CLI output;
- change workflow pass/fail behavior.

The only state interaction is reading through the caller-supplied `SideEffectRecordStore` when store-backed discovery is enabled.

## 12. Privacy And Redaction

The integration must remain reference-only.

The helper must not copy into report sections, summaries, citations, errors, Debug output, or serialization:

- side-effect target references;
- side-effect summaries;
- side-effect reason codes;
- side-effect authority context;
- side-effect lifecycle payloads;
- side-effect outcome references;
- side-effect idempotency details;
- side-effect redaction metadata;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue/comment bodies;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- raw record JSON;
- local filesystem paths.

Generated WorkReport serialization may include valid stable SideEffect IDs, as existing citation behavior already does, but must not include `SideEffectRecord` payload fields.

## 13. Report Section Behavior

SideEffect citations should still appear only in:

- `WorkReportSectionKind::SideEffects`

When no IDs are discovered or supplied, the existing side-effects summary should remain:

```text
No write side effects are supported; side effects are none, skipped, or unsupported.
```

When IDs are discovered or supplied, the existing supplied-reference summary remains appropriate:

```text
Side-effect records were supplied as stable references; no side-effect payloads are copied.
```

The helper must not claim that writes were attempted, approved, completed, denied, or skipped beyond what is present in the cited source records/events.

## 14. Relationship To Report Artifacts

Report artifacts remain out of scope.

This phase should not:

- write report artifacts;
- update artifact metadata;
- add referential integrity checks to artifact stores;
- require SideEffect records to exist before an artifact can be stored;
- change artifact serialization.

A later artifact-integrity phase may decide whether persisted WorkReport artifacts should validate cited SideEffect IDs against `SideEffectRecordStore`.

## 15. Relationship To EvidenceReference

EvidenceReference side-effect attachment remains separate.

This integration must not:

- create EvidenceReference values;
- attach EvidenceReference values to SideEffect records;
- treat SideEffect records as evidence payload storage;
- infer evidence from store presence;
- cite provider payloads or command outputs as evidence.

SideEffect citations and EvidenceReference citations may coexist in the same WorkReport, but they answer different questions.

## 16. Test Plan

Future implementation tests should cover:

- explicit helper generates a valid WorkReport with discovered store-backed SideEffect IDs;
- helper preserves existing behavior when no discovery is requested;
- helper merges caller-supplied IDs with discovered IDs without duplicates;
- explicit IDs retain priority over event and record sources;
- workflow event IDs retain priority over record-only sources;
- store-record-only IDs are cited when store discovery is enabled;
- no discovered/supplied IDs leaves existing none/skipped/unsupported text;
- required missing records fail closed without leaking IDs;
- corrupt store records fail closed without leaking record payloads;
- identity mismatch fails closed without leaking workflow/run identity;
- attempted/completed/failed events remain unsupported and are not cited;
- generated citations appear only in the side-effects section;
- generated reports do not copy target references, summaries, reason codes, outcomes, idempotency details, provider payloads, command output, or raw record JSON;
- helper does not mutate the run or event history;
- helper does not append events;
- helper does not create or update SideEffect records;
- helper does not write report artifacts;
- helper does not require `LocalExecutor` or runtime config;
- Debug output does not leak IDs, paths, identity values, or secret-like values beyond existing valid serialized citation IDs;
- existing WorkReport, SideEffect, SideEffect discovery, SideEffect record store, executor, runtime, adapter, hook, local check, and evidence tests continue to pass.

## 17. Proposed Implementation Sequence

Recommended small phases:

1. Add `TerminalLocalWorkReportSideEffectDiscoveryInput` and an explicit WorkReport-side discovery helper.
2. Use `discover_side_effect_references_from_store` to produce deterministic SideEffect IDs.
3. Merge IDs into `TerminalLocalWorkReportInput::side_effect_ids`.
4. Call existing `generate_terminal_local_work_report(...)`.
5. Add focused tests and docs.
6. Review.
7. Plan executor opt-in discovery only after helper review.
8. Plan artifact referential integrity only after executor/report integration is reviewed.

## 18. Open Questions

- Should the first helper always use store-backed discovery, or allow event-only discovery without store reads?
- Should `require_records` be allowed when `include_store_records` is false?
- Should discovery metadata, such as missing-record or unsupported-event counts, be surfaced in report sections, or remain internal to errors/tests for now?
- Should a later executor method be named separately to make opt-in discovery obvious?
- Should artifact storage later validate SideEffect citations against the store?
- Should attempted/completed/failed lifecycle events become discoverable only after runtime execution semantics exist?
- Should future approval-side-effect linkage require discovered SideEffect IDs to cite approval references?

## 19. Final Recommendation

Implemented and reviewed phase: **WorkReport SideEffect discovery helper integration, in-memory only**, documented in [WorkReport SideEffect Discovery Integration Report](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md) and accepted in [WorkReport SideEffect Discovery Integration Review](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REVIEW.md).

Executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md), following [Executor SideEffect Discovery Opt-In Plan](executor-side-effect-discovery-opt-in-plan.md), and accepted with non-blocking follow-ups in [Executor SideEffect Discovery Opt-In Review](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md).

Report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](report-artifact-side-effect-referential-integrity-plan.md), and accepted with non-blocking follow-ups in [Report Artifact SideEffect Referential Integrity Review](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md).

Recommended next phase: **approval-side-effect linkage planning**.

Do not change `LocalExecutor::execute_with_report(...)` yet. Do not add automatic discovery, automatic artifact writes, EvidenceReference side-effect attachment, approval-side-effect linkage, runtime side-effect execution, attempted/completed/failed executor behavior, writes, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.
