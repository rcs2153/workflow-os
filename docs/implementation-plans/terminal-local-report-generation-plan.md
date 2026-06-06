# Terminal Local Report Generation Plan

Status: In-memory terminal local report generation helper implemented. In-memory runtime result exposure helper implemented in [Runtime Result Report Exposure Plan](runtime-result-report-exposure-plan.md). Automatic runtime report generation is not implemented.

## 1. Executive Summary

`WorkReportContract` and `WorkReport` core models now exist. `EvidenceReference` provides the citation substrate, adapter telemetry and `Diagnostic` records can carry evidence references, and selected schema-version diagnostics can attach safe source/spec evidence.

An in-memory terminal local report generation helper is implemented. It accepts explicit terminal run/report inputs, supports completed, failed, and canceled runtime statuses, returns a validated `WorkReport`, and does not mutate runtime state, append events, write files, persist reports, or expose CLI output.

This plan records the conservative boundary: terminal-only, local, deterministic, in-memory generation using existing stable references and model constructors. In-memory runtime result exposure is documented separately. Automatic runtime generation, persistence, CLI rendering, examples, schemas, writes, side-effect modeling, approval evidence attachment, report artifacts, and reasoning lineage remain unimplemented.

## 2. Goals

- Generate `WorkReport` values for terminal workflow runs.
- Keep generation local and deterministic.
- Cite existing stable references instead of copying payloads.
- Preserve workflow pass/fail semantics unless a later accepted runtime design changes them.
- Capture incomplete, deferred, skipped, unsupported, failed, or blocked work.
- Capture known limitations and risks.
- Capture validation, audit, adapter telemetry, policy, and approval references where available.
- Maintain redaction-safe report construction through existing `WorkReport` constructors.
- Keep generated reports useful for future operator handoff without making them audit logs, marketing summaries, or reasoning graphs.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- a persistent report store;
- CLI rendering;
- a JSON export command;
- report signing or notarization;
- UI;
- examples;
- workflow spec schema changes;
- domain-specific templates;
- reasoning lineage;
- side-effect boundary modeling;
- writes;
- production compliance system behavior;
- SIEM or OpenTelemetry integration;
- DLP or access-control systems;
- release posture changes.

## 4. Generation Trigger

V1 generation should be terminal-only.

Candidate terminal report statuses:

- completed;
- failed;
- canceled;
- escalated;
- blocked.

Current runtime `WorkflowRunStatus::is_terminal()` only treats completed, failed, and canceled as terminal. `WorkflowRunStatus::Escalated` exists, but is not terminal in the current runtime projection. `WorkReportStatus::Blocked` exists as report vocabulary, but no runtime blocked terminal state exists yet.

Generation choices:

| Option | Assessment |
| --- | --- |
| Automatically generate for all terminal runs | Too broad for v1; changes runtime expectations and may surprise existing callers. |
| Generate only when a `WorkReportContract` is declared in workflow specs | Correct long term, but requires schema changes that are out of scope. |
| Generate only when runtime config opts in | Best v1 posture; preserves existing workflow behavior and avoids schema changes. |

Recommendation: first implementation should use explicit runtime opt-in or a test-visible/internal generation helper. It should not auto-generate for every run and should not require workflow schema support yet.

## 5. Report Contract Source

Possible contract sources:

- built-in default contract;
- explicitly declared workflow contract;
- runtime configuration;
- test-only contract.

Recommendation: use a built-in default v1 contract supplied directly to the generation helper/service for the first implementation. This avoids workflow spec schema changes while proving report construction against known contract vocabulary.

Do not add schema fields yet. Workflow-declared report contracts should wait until report generation, compatibility expectations, and operator value are reviewed.

## 6. Runtime Integration Boundary

Relevant current runtime surfaces:

- `LocalExecutor::execute` returns `WorkflowRun`.
- `WorkflowRun` contains a `WorkflowRunSnapshot` and append-only event history.
- `WorkflowRunSnapshot` carries immutable run identity, status, skill invocations, approval requests, retries, escalations, cancellation, and failure state.
- Runtime events carry stable `EventId`, sequence numbers, immutable workflow identity, timestamps, actor/correlation metadata where relevant, and state transition details.
- Validation diagnostics exist before execution through loader/validator surfaces.
- Adapter telemetry can be emitted from controlled read-only fixture handlers and stored through adapter telemetry stores.
- Policy decisions are represented in events and durable pre-run policy audit records.
- Approval requests and decisions are modeled in runtime state/events, but approval evidence attachment is not implemented.

Smallest future integration point:

1. Add an internal report-generation service/helper that accepts an already terminal `WorkflowRun`, a `WorkReportContract`, validation diagnostics, and optional reference collections.
2. Return an in-memory `WorkReport`.
3. Do not append a runtime event, write a file, persist a report, or expose a CLI command in the first implementation.

This keeps report generation outside the core executor state transition path until maintainers decide whether report generation should affect runtime results.

## 7. Report Generation Inputs

Allowed inputs:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- terminal status;
- generated timestamp;
- generated actor or system actor;
- correlation ID;
- validation diagnostics;
- `EvidenceReference` IDs;
- audit event IDs;
- adapter telemetry record IDs or stable adapter telemetry references;
- policy decision IDs or policy event IDs;
- approval request/decision IDs where available;
- bounded operator/system handoff notes where available.

Disallowed inputs:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira bodies or comments;
- raw GitHub bodies or file contents;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials, secrets, tokens, authorization headers, or private keys.

## 8. Citation Construction Policy

Citations should be built from existing stable IDs or bounded stable references.

Rules:

- Use `WorkReportCitationTarget::EvidenceReference` for existing `EvidenceReferenceId` values.
- Use workflow event IDs for workflow event citations.
- Use audit event IDs for audit citations.
- Use adapter telemetry stable references for adapter telemetry citations until a dedicated adapter telemetry ID exists.
- Use `ValidationReferenceId` for validation diagnostics that expose or can derive a stable validation reference.
- Use approval/policy citation vocabulary only where stable IDs already exist; do not implement approval evidence attachment in this phase.
- Do not recreate `EvidenceReference` values implicitly.
- Missing citations must be explicit through `WorkReportCitation::missing`.
- Citation summaries must be bounded, redacted, and never copied from raw payloads.
- If a required citation cannot be built, report generation should fail or produce an explicit missing citation according to the accepted generation policy. It must not fabricate evidence.

Mandatory versus best-effort:

- Required identity and core sections are mandatory because `WorkReport` validation requires them.
- Evidence, event, audit, validation, policy, approval, and adapter citations should be best-effort in the first generation implementation unless the built-in contract explicitly requires them and the inputs are available.
- Missing citation cases should be visible as missing citations or incomplete-work disclosures, not silent omissions.

## 9. Section Population Policy

Every v1 section should be present. A section may contain explicit none/skipped/not-available text when the current runtime lacks a source.

| Section | Likely source | Required | Empty/none policy | Must not copy |
| --- | --- | --- | --- | --- |
| Work performed | Run status, event history, skill invocation summaries, step/skill IDs. | Yes | Use bounded summary such as completed, failed, canceled, skipped, or unavailable. | Raw skill input/output payloads. |
| Evidence considered | EvidenceReference IDs, diagnostic evidence, adapter telemetry evidence. | Yes | Explicitly say no evidence references were available if none. | Evidence payloads or provider content. |
| Decisions made | Policy decisions, approval decisions, cancellation/failure/escalation records. | Yes | State not available if no stable decision reference exists. | Raw policy context or approval packet payloads. |
| Policy gates evaluated | Policy decision events, policy audit records. | Yes | State not available for paths without policy audit access. | Raw policy context beyond bounded summaries/references. |
| Approvals | Approval requests and decisions in snapshot/events. | Yes | State none requested, pending, denied, granted, or unavailable. | Raw approval evidence packets. |
| Validation and quality checks | Validation diagnostics and diagnostic evidence. | Yes | State no validation diagnostics supplied when absent. | Raw spec contents or parser payloads. |
| Side effects | Runtime event/action summary and current no-write posture. | Yes | State none/skipped/unsupported for v1 local/read-only paths. | Provider mutation payloads or claims of write support. |
| Incomplete or deferred work | Failure, cancellation, escalation, missing citations, unavailable references. | Yes | Explicit none if completed with no known incomplete work. | Raw failure payloads. |
| Known limitations | Runtime/report generator limitations, fixture/local-only boundaries. | Yes | Include at least local/in-memory/no-persistence limitation until artifact phase exists. | Marketing language or hidden limitations. |
| Risks | Missing citations, unavailable telemetry, sensitive metadata, terminal failure context. | Yes | Explicit low/no known risk only when defensible. | Raw provider/spec/command data. |
| Operator handoff notes | Supplied bounded notes or generated conservative notes. | Yes | State no operator handoff notes supplied if absent. | Free-form unbounded notes or secrets. |

## 10. Workflow Semantics And Failure Behavior

Conservative v1 behavior:

- Report generation should not change workflow pass/fail semantics.
- Report generation should not mutate terminal run status.
- Report generation should not append post-terminal workflow events.
- Report generation failure should not retroactively fail a successfully completed workflow unless a future accepted design makes report generation part of the workflow outcome.
- If report construction fails validation, return a structured report-generation error from the helper/service.
- Do not convert report construction failures into misleading user project diagnostics.
- Errors must use stable codes and avoid raw paths, payloads, metadata values, snippets, command output, provider data, or secrets.

Recommendation: first implementation should be a separate in-memory helper that can fail independently of the workflow run. Runtime result exposure should be considered only after review.

## 11. Storage And Artifact Posture

No persistence in the first generation implementation unless separately approved.

Initial posture:

- generated `WorkReport` exists only in memory;
- no filesystem artifact;
- no local backend storage;
- no CLI rendering;
- no export command;
- no schema exposure;
- no report signing/notarization.

A later artifact phase would need:

- storage location and retention policy;
- corruption behavior;
- compatibility/versioning policy;
- redaction and sensitivity review for files;
- CLI/operator display posture;
- tests proving no raw payloads are written;
- migration or cleanup guidance.

## 12. Privacy And Redaction

Generation must use `WorkReport` constructors and validated `WorkReportCitation` constructors.

Rules:

- bounded summaries only;
- no raw payloads;
- no raw command/log/spec/parser/provider content;
- no secret-like metadata;
- redaction metadata must pass WorkReport-boundary validation;
- file paths are potentially sensitive and should be omitted, bounded, or represented by safe references unless specifically required;
- Debug and serialization paths must remain safe;
- reports may be confidential even when citations are read-only.

## 13. Contract Enforcement

V1 generation should validate the generated report against the built-in/default `WorkReportContract` vocabulary at the model boundary.

Current model validation ensures all core v1 sections are present. It does not yet enforce a specific contract instance beyond report contract ID/version alignment and core section shape.

Recommendations:

- First implementation may use `WorkReportContract::v1` as the contract source.
- Missing required sections should fail report construction before returning a report.
- Missing optional citations should become explicit missing citations or incomplete-work disclosures.
- Future workflow-declared contracts must wait for schema planning and validation rules.
- Compatibility rules for contract IDs, versions, and serialized report shape should be reviewed before persistence or schema exposure.

## 14. Test Plan

Future implementation tests should cover:

- terminal completed run can produce a valid in-memory `WorkReport`;
- failed, canceled, escalated, and blocked statuses are representable where runtime context supports them;
- report generation uses existing model constructors;
- report generation preserves workflow result semantics;
- missing optional citation becomes an explicit missing citation;
- invalid report construction fails safely;
- no raw provider/spec/command/parser payload is copied;
- validation diagnostics are cited by stable reference;
- adapter telemetry is cited by stable reference;
- `EvidenceReference` IDs are cited, not recreated;
- side-effects section is present as none/skipped/unsupported;
- Debug and serialization non-leakage;
- no persistence, files, CLI output, or report artifacts are created;
- existing runtime tests still pass.

## 15. Proposed Implementation Sequence

Recommended small phases:

1. Add an internal report-generation service/helper that returns model-only `WorkReport` output. Completed.
2. Generate an in-memory `WorkReport` for one terminal local runtime path using explicit inputs. Completed.
3. Add tests for terminal statuses, citation construction, section population, validation failure, and non-leakage. Completed.
4. Review the helper/service before wiring it into runtime results. Completed.
5. Plan in-memory runtime result exposure. Completed in [Runtime Result Report Exposure Plan](runtime-result-report-exposure-plan.md).
6. Implement the in-memory runtime result exposure helper. Completed.
7. Only after separate planning, consider persistence or artifact writing.
8. CLI rendering and examples remain later.

## 16. Open Questions

- Should terminal report generation be automatic or opt-in?
- Should report generation failure ever fail a workflow?
- Should reports be returned in runtime results immediately or only be test-visible first?
- Should a built-in default contract exist before workflow schema support?
- How should report generation cite audit events if a future store changes ID/index behavior?
- How should approval/policy citations behave before approval evidence attachment exists?
- How should operator handoff notes be supplied?
- Should reports include bounded natural-language summaries or only structured section text?
- When should reports become persisted artifacts?
- What minimum vertical slice proves value without expanding scope?
- Should `WorkflowRunStatus::Escalated` become terminal before report generation supports escalated reports?
- Should blocked remain report-only vocabulary until runtime has a blocked terminal state?

## 17. Final Recommendation

Recommended next phase: runtime result exposure helper review.

Future phases should not implement automatic generation for every run, persistence, CLI rendering, examples, workflow spec schema changes, report artifacts, reasoning lineage, side-effect boundary modeling, writes, approval evidence attachment, production compliance integrations, DLP/access control, or release posture changes unless separately scoped and approved.
