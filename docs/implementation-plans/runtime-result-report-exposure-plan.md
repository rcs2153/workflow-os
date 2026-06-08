# Runtime Result Report Exposure Plan

Status: In-memory runtime result exposure helper implemented. Executor-integrated report-bearing execution for local runs is implemented as documented in [Executor-Integrated Report Result Plan](executor-integrated-report-result-plan.md). Automatic report generation for every run, persistence, artifacts, CLI rendering, schemas, and examples are not implemented.

## 1. Executive Summary

`WorkReportContract`, `WorkReport`, and the in-memory terminal local report generation helper are implemented and reviewed. The helper can construct validated terminal `WorkReport` values from explicit inputs for completed, failed, and canceled local runs.

An in-memory runtime result exposure helper is implemented. It pairs an already-terminal `WorkflowRun` with the generated `WorkReport` in a `TerminalLocalWorkReportResult` without changing existing executor return types, appending events, mutating runtime state, writing files, persisting reports, or exposing CLI output.

This plan still does not implement automatic report generation, persistence, report artifacts, CLI rendering, examples, workflow schema changes, reasoning lineage, side-effect modeling, writes, approval evidence attachment, runtime config, or release posture changes.

## 2. Goals

- Expose in-memory terminal `WorkReport` values from an explicit runtime result surface.
- Preserve current workflow execution semantics.
- Keep report generation local, deterministic, and opt-in.
- Avoid appending post-terminal workflow events.
- Avoid writing reports to state backends, files, or external systems.
- Use the existing terminal report generation helper.
- Use existing `WorkReport` and `WorkReportCitation` constructors.
- Keep report generation failure behavior explicit and non-leaking.
- Prepare for future artifact, CLI, and workflow-declared contract planning without implementing them now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic report generation for every run;
- report artifact writing;
- local persistence;
- filesystem output;
- CLI rendering or commands;
- JSON export commands;
- examples;
- workflow spec schema changes;
- workflow-declared report contracts;
- runtime configuration for reports;
- reasoning lineage;
- side-effect boundary modeling;
- write behavior;
- approval evidence attachment;
- production reporting;
- report signing or notarization;
- SIEM or OpenTelemetry integration;
- DLP or access-control systems;
- release posture changes.

## 4. Current Runtime Surface

Current relevant surfaces:

- `LocalExecutor::execute(...) -> Result<WorkflowRun, WorkflowOsError>`.
- `LocalExecutor::decide_approval(...) -> Result<WorkflowRun, WorkflowOsError>`.
- `LocalExecutor::cancel(...) -> Result<WorkflowRun, WorkflowOsError>`.
- `WorkflowRun` contains `WorkflowRunSnapshot` and append-only event history.
- `WorkflowRunStatus::is_terminal()` returns true for completed, failed, and canceled.
- Runtime `Escalated` is not terminal today.
- The in-memory helper accepts a borrowed terminal `WorkflowRun` and explicit report inputs.

The current executor result type is the run itself. Exposing reports directly from existing methods would be a public API shape change, so the first implementation should add a new explicit result wrapper or explicit helper path rather than changing existing method return types.

## 5. Exposure Options

| Option | Assessment |
| --- | --- |
| Change `LocalExecutor::execute` to return a report-bearing wrapper | Too disruptive for v1 exposure because existing callers expect `WorkflowRun`. |
| Add optional report fields to `WorkflowRun` | Reject. A work report is not workflow state and should not mutate the event-sourced run model. |
| Append a post-terminal report event | Reject for this phase. Current terminal-state rules reject mutating post-terminal events. |
| Add an explicit execution method that returns a wrapper | Deferred. Keeps existing API stable, but still introduces executor surface area. |
| Add an explicit in-memory result helper/wrapper | Implemented. Keeps existing API stable and proves the report-bearing result shape. |
| Keep using only the standalone helper | Already implemented; insufficient for runtime result exposure. |

Implemented recommendation: add a new explicit helper path that returns a report-bearing result wrapper without changing existing `execute(...)`.

## 6. Candidate Runtime Result Model

The implementation adds the smallest model needed to carry a run and report:

- `TerminalLocalWorkReportResult`

Result fields:

- `run: WorkflowRun`
- `work_report: WorkReport`

The result wrapper remains in memory only. It does not imply persistence, artifacts, CLI output, schema exposure, or workflow state mutation.

## 7. Trigger And Opt-In Policy

Runtime result exposure should be explicit opt-in.

Acceptable v1 triggers:

- a new explicit executor method;
- an explicit generation request parameter on a new request type;
- a test-visible runtime helper that receives a `WorkflowRun` and report inputs.

Not acceptable for v1:

- automatic generation for all terminal runs;
- workflow spec declarations;
- runtime configuration files;
- environment variables;
- CLI flags;
- implicit generation during rehydration or state inspection.

Implemented posture: use a new explicit helper with explicit report inputs. Existing executor behavior remains unchanged.

## 8. Contract Source

V1 runtime result exposure should continue using explicit contract identity/version inputs unless a separately reviewed default contract helper is introduced.

Do not add workflow schema fields. Do not infer report requirements from workflow specs. Do not create runtime config.

Future workflow-declared contract support requires separate schema planning, validation behavior, compatibility rules, and docs.

## 9. Report Generation Inputs

Allowed inputs:

- terminal `WorkflowRun`;
- report ID;
- report contract ID;
- report contract version;
- generated timestamp;
- generated actor/system actor;
- sensitivity;
- redaction metadata;
- optional correlation ID;
- supplied `EvidenceReferenceId` values;
- supplied workflow event IDs;
- supplied audit event IDs;
- supplied adapter telemetry stable references;
- supplied validation reference IDs;
- supplied policy decision event IDs;
- supplied approval reference IDs where stable;
- bounded incomplete-work disclosures;
- bounded known limitations;
- bounded risks;
- bounded operator handoff notes.

Forbidden inputs:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue/comment bodies;
- raw GitHub file contents;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded notes, limitations, risks, or disclosures.

## 10. Failure Behavior

Runtime result exposure must preserve workflow semantics.

Recommended v1 behavior:

- workflow execution succeeds or fails exactly as it does today;
- report generation occurs only after a terminal `WorkflowRun` exists;
- report generation failure does not mutate the run;
- report generation failure does not append events;
- report generation failure does not write backend state;
- report generation failure returns a structured report-generation error from the new report-bearing path;
- existing `execute`, `decide_approval`, and `cancel` methods remain unchanged.

Implemented executor decision: the explicit executor-integrated path returns the `WorkflowRun` with a separate report error field when report generation fails after execution returns a run. The underlying run is not changed.

## 11. Workflow State And Event Boundary

Runtime result exposure must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- mutate event history;
- append post-terminal events;
- emit audit events;
- emit observability events;
- write local state;
- create files;
- persist reports;
- alter rehydration behavior;
- alter terminal status semantics.

The report is a derived in-memory result, not event-sourced state.

## 12. Citation Policy

Runtime result exposure should reuse the helper citation policy:

- cite stable IDs only;
- never recreate `EvidenceReference` values;
- never fabricate IDs;
- use explicit none/not-available section text where stable references are absent;
- decide in a later phase whether missing citations should become explicit `missing=true` citation records;
- keep summaries bounded and redaction-safe;
- do not copy raw payloads into citations or sections.

The first runtime exposure implementation should not attempt to discover citations from stores. It should accept supplied stable references explicitly.

## 13. Privacy And Redaction

Runtime result exposure must use existing constructors:

- `generate_terminal_local_work_report(...)`;
- `WorkReport::new(...)` through the helper;
- `WorkReportSection::new(...)` through the helper;
- `WorkReportCitation::new(...)` through the helper.

All existing WorkReport redaction metadata validation remains active.

Runtime exposure must not create a path where raw payloads or secret-like values bypass the helper/model validation boundary.

## 14. Compatibility Posture

Existing public methods should remain source-compatible:

- `LocalExecutor::execute(...) -> Result<WorkflowRun, WorkflowOsError>`;
- `LocalExecutor::decide_approval(...) -> Result<WorkflowRun, WorkflowOsError>`;
- `LocalExecutor::cancel(...) -> Result<WorkflowRun, WorkflowOsError>`.

Any new report-bearing API should be additive and documented as local/in-memory report exposure, not durable reporting.

Do not expose reports through schemas, CLI JSON, persisted state, or examples until separate planning accepts those surfaces.

## 15. Test Plan

Future implementation tests should cover:

- existing executor methods still return `WorkflowRun`;
- explicit report-bearing execution path returns `WorkflowRun` and `WorkReport`;
- completed run exposure works;
- failed run exposure works;
- canceled run exposure works;
- non-terminal run input is rejected by the report path;
- report generation failure does not mutate the run;
- report generation failure does not append events;
- report generation failure does not write backend state;
- no filesystem artifacts are created;
- no CLI output is emitted;
- all v1 report sections are present;
- stable supplied references are cited;
- absent optional references become explicit not-available section text;
- EvidenceReference IDs are cited without recreating evidence;
- raw provider/spec/command/parser payload markers are not copied;
- secret-like report inputs are rejected without leakage;
- Debug and serialization remain safe;
- existing runtime tests still pass;
- existing WorkReport and WorkReportContract tests still pass.

## 16. Proposed Implementation Sequence

Small phases:

1. Add an additive runtime result exposure model and explicit report-bearing helper. Completed.
2. Preserve existing executor method signatures. Completed.
3. Add tests for completed, failed, canceled, non-terminal rejection, non-mutation, no persistence, and non-leakage. Completed.
4. Plan executor method integration. Completed in [Executor-Integrated Report Result Plan](executor-integrated-report-result-plan.md).
5. Implement explicit executor-integrated report-bearing execution. Completed.
6. Review before considering approval/cancellation report-bearing methods, CLI, artifacts, schema, persistence, or examples.
7. Plan report artifacts separately if executor-integrated report-bearing execution is accepted.

## 17. Open Questions

- Should the first API be an executor method or a standalone runtime exposure helper?
- Should report generation failure return `Err`, or return a run plus report error field?
- Should the runtime exposure path accept a full `WorkReportContract` instance or continue with contract ID/version?
- Should missing citations remain section text or become explicit missing-citation records?
- Should runtime exposure gather audit/event IDs automatically or require explicit supplied references?
- Should report exposure remain crate-public but experimental until artifact/CLI/schema posture is designed?
- Should approval evidence attachment precede approval citation exposure?
- How should runtime result exposure interact with rehydrated existing runs?

## 18. Final Recommendation

Proceed next with a focused review of the executor-integrated report result implementation.

Future implementation should not add approval/cancellation report-bearing methods, automatic report generation for every run, append events, persist reports, create artifacts, render CLI output, change schemas, update examples, implement reasoning lineage, model side effects, add writes, attach approval evidence, or change release posture unless separately scoped and approved.
