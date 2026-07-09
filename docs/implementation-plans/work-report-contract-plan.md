# WorkReportContract Planning Document

Status: `WorkReportContract` and `WorkReport` core models implemented. An in-memory terminal local report generation helper is implemented as described in [Terminal Local Report Generation Plan](terminal-local-report-generation-plan.md). An in-memory runtime result exposure helper is implemented as documented in [Runtime Result Report Exposure Plan](runtime-result-report-exposure-plan.md). Explicit executor-integrated report-bearing execution is implemented as documented in [Executor-Integrated Report Result Plan](executor-integrated-report-result-plan.md). An explicit local report artifact store is implemented as documented in [Report Artifact Plan](report-artifact-plan.md). Local check result citation target planning is documented in [WorkReport Local Check Result Citation Target Plan](work-report-local-check-citation-target-plan.md), and WorkReport citation vocabulary for local check results is implemented. Terminal report helper integration for supplied local check result references is implemented and documented in [Terminal Report Local Check Citation Integration Plan](terminal-report-local-check-citation-integration-plan.md). WorkReport typed handoff citation planning is documented in [WorkReport Typed Handoff Citation Plan](work-report-typed-handoff-citation-plan.md), and WorkReport citation vocabulary for typed handoffs is implemented. WorkReport hook citation target planning is documented in [WorkReport Agent Harness Hook Citation Target Plan](work-report-hook-citation-target-plan.md), WorkReport citation vocabulary for agent harness hook invocation IDs is implemented as model-only vocabulary, terminal report helper hook citation integration is implemented in [Terminal Report Agent Harness Hook Citation Integration Plan](terminal-report-hook-citation-integration-plan.md) for explicit supplied IDs only, WorkReport hook disclosure citation vocabulary is implemented as model-only vocabulary as documented in [WorkReport Hook Disclosure Citation Plan](work-report-hook-disclosure-citation-plan.md), and terminal report helper hook disclosure citation integration is implemented in [Terminal Report Hook Disclosure Citation Integration Plan](terminal-report-hook-disclosure-citation-integration-plan.md) for explicit supplied IDs only. Terminal report helper typed handoff citation integration is implemented in [Terminal Report Typed Handoff Citation Integration Plan](terminal-report-typed-handoff-citation-integration-plan.md). Executor-integrated typed handoff report input propagation is implemented in [Executor Typed Handoff Report Input Propagation Plan](executor-typed-handoff-report-input-plan.md). Executor hook report input propagation is implemented in [Executor Hook Report Input Propagation Plan](executor-hook-report-input-plan.md), and executor hook disclosure report input propagation is implemented in [Executor Hook Disclosure Report Input Propagation Plan](executor-hook-disclosure-report-input-plan.md). Hook disclosure discovery planning and the first in-memory implementation are documented in [Hook Disclosure Discovery Plan](hook-disclosure-discovery-plan.md). Runtime hook execution planning is documented in [Agent Harness Hook Runtime Execution Plan](agent-harness-hook-runtime-execution-plan.md), the explicit in-memory runtime hook execution helper is implemented, executor hook checkpoint planning is documented in [Executor Hook Checkpoint Plan](executor-hook-checkpoint-plan.md), the explicit `BeforeReport` executor checkpoint is implemented for `execute_with_report(...)` only, executor hook event/audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](executor-hook-event-audit-semantics-plan.md), the model-only hook workflow event vocabulary is implemented for future bounded, state-preserving hook events, and generic hook workflow event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](hook-event-audit-projection-plan.md). Report/audit/missing-citation semantics hardening is implemented in [Report, Audit, And Missing-Citation Semantics Plan](report-audit-missing-citation-semantics-plan.md). Command-output evidence policy planning is documented in [Command Output Evidence Policy Plan](command-output-evidence-policy-plan.md). WorkReport high-assurance approval disclosure planning is documented in [WorkReport High-Assurance Approval Disclosure Plan](work-report-high-assurance-approval-disclosure-plan.md), the first explicit report-only disclosure slice is implemented in [WorkReport High-Assurance Approval Disclosure Report](../concepts/WORK_REPORT_HIGH_ASSURANCE_APPROVAL_DISCLOSURE_REPORT.md), and the first pure high-assurance approval disclosure discovery helper is implemented in [High-Assurance Approval Disclosure Discovery Plan](high-assurance-approval-disclosure-discovery-plan.md). The SideEffect core model is implemented in [SideEffect Core Model Report](../concepts/SIDE_EFFECT_CORE_MODEL_REPORT.md), WorkReport side-effect citation vocabulary is implemented as documented in [WorkReport SideEffect Citation Report](../concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REPORT.md), terminal report SideEffect citation propagation is implemented in [Terminal Report SideEffect Citation Integration Report](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REPORT.md), executor SideEffect ID propagation is implemented in [Executor SideEffect Report Input Propagation Report](../concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md), model-only SideEffect workflow event vocabulary plus generic audit projection are implemented in [SideEffect Workflow Event Model Report](../concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md), explicit local executor append support for proposed/denied/skipped SideEffect events is implemented in [Executor SideEffect Event Append Report](../concepts/EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md), SideEffect persistence/discovery planning is documented in [SideEffect Persistence And Discovery Plan](side-effect-persistence-discovery-plan.md), explicit local SideEffect record persistence is implemented in [SideEffect Record Store Report](../concepts/SIDE_EFFECT_RECORD_STORE_REPORT.md), WorkReport SideEffect discovery integration planning is documented in [WorkReport SideEffect Discovery Integration Plan](work-report-side-effect-discovery-integration-plan.md), and executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md). Automatic SideEffect discovery, automatic runtime report generation for every run, approval/cancellation report-bearing methods, automatic artifact writing from executor paths, command-output evidence implementation, schema, CLI rendering, example update, reasoning lineage, approval attachment, runtime side-effect execution, write behavior, domain pack, broader automatic executor hook invocation, dedicated hook audit sink emission, discovery from workflow events or audit projections without explicit opt-in, automatic high-assurance approval discovery integration, workflow-declared high-assurance controls, RBAC/IdP/quorum approval, and release posture changes are not implemented.

Update: deterministic required-checkpoint enforcement for explicit `BeforeReport` report paths is implemented in [Deterministic Hook Checkpoint Enforcement Report](../concepts/DETERMINISTIC_HOOK_CHECKPOINT_ENFORCEMENT_REPORT.md). Existing report-bearing callers remain opt-in; no workflow-declared hook configuration, runtime hook configuration, broad automatic checkpoints, dedicated hook audit sink emission, persistence, CLI behavior, schemas, side effects, writes, or release posture changes are implemented.

Update: model-only SideEffect workflow event vocabulary and bounded generic audit projection are implemented in [SideEffect Workflow Event Model Report](../concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md), explicit local executor append support for proposed/denied/skipped SideEffect events is implemented in [Executor SideEffect Event Append Report](../concepts/EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md), SideEffect persistence/discovery planning is documented in [SideEffect Persistence And Discovery Plan](side-effect-persistence-discovery-plan.md), explicit local SideEffect record persistence is implemented in [SideEffect Record Store Report](../concepts/SIDE_EFFECT_RECORD_STORE_REPORT.md), explicit WorkReport SideEffect discovery helper integration is implemented in [WorkReport SideEffect Discovery Integration Report](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md), executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md), and report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](report-artifact-side-effect-referential-integrity-plan.md). Automatic executor SideEffect discovery, automatic artifact writes, EvidenceReference side-effect attachment, runtime side-effect execution, attempted/completed/failed side-effect append behavior, writes, schemas, CLI rendering, examples, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

Executor hook disclosure report input propagation is implemented in [Executor Hook Disclosure Report Input Propagation Plan](executor-hook-disclosure-report-input-plan.md). It forwards explicitly supplied hook disclosure IDs. Hook disclosure discovery is implemented in [Hook Disclosure Discovery Plan](hook-disclosure-discovery-plan.md) only for already-validated in-memory `BeforeReport` hook results in the explicit report-bearing executor path; discovery from workflow events, audit projections, durable stores, text, diagnostics, local checks, and adapter telemetry remains unimplemented.

Update: approval proof-marker artifact gate behavior is planned in [Report Artifact Approval Proof Marker Gate Plan](report-artifact-approval-proof-marker-gate-plan.md), and the first pure in-memory helper is implemented in [Report Artifact Approval Proof Marker Gate Helper Report](../concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REPORT.md) and accepted in [Report Artifact Approval Proof Marker Gate Helper Review](../concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REVIEW.md). Store-backed integration planning is documented in [Report Artifact Approval Proof Marker Store-Backed Gate Integration Plan](report-artifact-approval-proof-marker-store-backed-gate-integration-plan.md). The boundary is explicit and artifact-scoped: the helper validates a `WorkReportArtifactRecord` against caller-supplied durable proof-marker projection records before artifact write. Store-backed integration implementation, executor defaults, automatic artifact writing, CLI behavior, schemas, examples, writes, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

## 1. Executive Summary

`EvidenceReference` now provides the citation substrate for Workflow OS. The core model is implemented, adapter telemetry can carry validated evidence references, `Diagnostic` can carry validated evidence references, and selected schema-version validation diagnostics can attach source/spec evidence without copying raw spec contents.

`WorkReportContract` should define the governed handoff artifact for future terminal work reports. A work report should explain what was done, what evidence was considered, what decisions and approvals occurred, what validation ran, what remains incomplete, and what an operator or downstream workflow should know next.

The first model phases implemented the `WorkReportContract` core model and the `WorkReport` core model. The next phase implemented an in-memory terminal local helper that can construct a validated `WorkReport` from explicit terminal run/report inputs. Automatic runtime report generation should come later and should initially remain explicitly opted in.

Work reports are not marketing summaries. They are not audit logs. They are not reasoning lineage graphs. They should cite evidence, audit, event, adapter telemetry, validation, approval, and policy references without copying raw payloads.

This plan now tracks the implemented contract, report model, in-memory generation helper, in-memory runtime result exposure helper, executor-integrated report result, explicit local report artifact store, local check result citation target, terminal report local check citation integration, WorkReport typed handoff citation target, terminal report typed handoff citation integration, and report/audit/missing-citation semantics phases. Terminal local report generation planning is documented separately in [Terminal Local Report Generation Plan](terminal-local-report-generation-plan.md), runtime result exposure planning is documented in [Runtime Result Report Exposure Plan](runtime-result-report-exposure-plan.md), report artifact planning is documented in [Report Artifact Plan](report-artifact-plan.md), local check result citation target planning is documented in [WorkReport Local Check Result Citation Target Plan](work-report-local-check-citation-target-plan.md), terminal report local check citation integration is documented in [Terminal Report Local Check Citation Integration Plan](terminal-report-local-check-citation-integration-plan.md), terminal report typed handoff citation integration is documented in [Terminal Report Typed Handoff Citation Integration Plan](terminal-report-typed-handoff-citation-integration-plan.md), runtime hook execution planning is documented in [Agent Harness Hook Runtime Execution Plan](agent-harness-hook-runtime-execution-plan.md), report/audit/missing-citation semantics are documented in [Report, Audit, And Missing-Citation Semantics Plan](report-audit-missing-citation-semantics-plan.md), approval proof marker WorkReport/audit citation behavior is planned in [Approval Proof Marker WorkReport And Audit Citation Plan](approval-proof-marker-workreport-audit-citation-plan.md), the first pure approval proof marker citation derivation helper is implemented in [Approval Proof Marker Citation Helper Report](../concepts/APPROVAL_PROOF_MARKER_CITATION_HELPER_REPORT.md), terminal report opt-in integration is implemented in [Terminal Report Approval Proof Marker Citation Integration Report](../concepts/TERMINAL_REPORT_APPROVAL_PROOF_MARKER_CITATION_INTEGRATION_REPORT.md) and accepted in [Terminal Report Approval Proof Marker Citation Integration Review](../concepts/TERMINAL_REPORT_APPROVAL_PROOF_MARKER_CITATION_INTEGRATION_REVIEW.md), executor propagation is implemented in [Executor Proof Marker Citation Report Input Propagation Report](../concepts/EXECUTOR_PROOF_MARKER_CITATION_REPORT_INPUT_PROPAGATION_REPORT.md) and accepted in [Executor Proof Marker Citation Report Input Propagation Review](../concepts/EXECUTOR_PROOF_MARKER_CITATION_REPORT_INPUT_PROPAGATION_REVIEW.md), audit projection persistence planning is documented in [Approval Proof Marker Audit Projection Persistence Plan](approval-proof-marker-audit-projection-persistence-plan.md) and accepted in [Approval Proof Marker Audit Projection Persistence Plan Review](../concepts/APPROVAL_PROOF_MARKER_AUDIT_PROJECTION_PERSISTENCE_PLAN_REVIEW.md), the first pure in-memory audit projection posture helper is implemented in [Approval Proof Marker Audit Projection Helper Report](../concepts/APPROVAL_PROOF_MARKER_AUDIT_PROJECTION_HELPER_REPORT.md) and accepted in [Approval Proof Marker Audit Projection Helper Review](../concepts/APPROVAL_PROOF_MARKER_AUDIT_PROJECTION_HELPER_REVIEW.md), durable local audit projection persistence is implemented as an explicit helper in [Approval Proof Marker Durable Audit Projection Persistence Helper Report](../concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_HELPER_REPORT.md) and accepted in [Approval Proof Marker Durable Audit Projection Persistence Helper Review](../concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_HELPER_REVIEW.md), following [Approval Proof Marker Durable Audit Projection Persistence Plan](approval-proof-marker-durable-audit-projection-persistence-plan.md) and accepted planning in [Approval Proof Marker Durable Audit Projection Persistence Plan Review](../concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_PLAN_REVIEW.md), and the first pure report artifact approval proof-marker gate helper is implemented in [Report Artifact Approval Proof Marker Gate Helper Report](../concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REPORT.md) and accepted in [Report Artifact Approval Proof Marker Gate Helper Review](../concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REVIEW.md). Store-backed proof-marker artifact gate integration planning is documented in [Report Artifact Approval Proof Marker Store-Backed Gate Integration Plan](report-artifact-approval-proof-marker-store-backed-gate-integration-plan.md). Automatic runtime behavior, executor default proof-marker citation behavior, dedicated proof-marker audit sink records, store-backed report artifact proof-marker gate implementation, automatic artifact writing from executor paths, CLI rendering, schema exposure, examples, approval/cancellation report-bearing methods, and workflow-declared report contracts are still not implemented.

## 2. Goals

- Create a domain-neutral report contract.
- Allow workflows to declare report requirements later.
- Support future terminal governed handoff artifacts.
- Cite evidence references rather than copying evidence payloads.
- Cite audit events, workflow events, approvals, adapter telemetry, validation diagnostics, and policy decisions by stable references.
- Disclose incomplete, deferred, skipped, denied, failed, or unsupported work.
- Preserve known limitations and risk notes as first-class report content.
- Prepare for future side-effect boundary and reasoning lineage integration. The SideEffect core model, WorkReport side-effect citation vocabulary, terminal helper propagation for explicitly supplied SideEffect IDs, and executor SideEffect report input propagation are implemented. Automatic discovery and runtime side-effect execution remain future scoped work.
- Keep Workflow OS centered on governed enterprise work rather than domain-specific report templates.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- persistence;
- CLI rendering;
- report export;
- report signing or notarization;
- report UI;
- domain-specific templates;
- natural-language marketing reports as a source of truth;
- reasoning lineage implementation;
- write behavior;
- production compliance system behavior;
- SIEM or OpenTelemetry integration;
- DLP or access-control systems;
- production evidence storage;
- schema changes;
- release posture changes.

## 4. Work Report Versus Audit Versus Evidence

Workflow OS should keep the source-of-truth boundaries explicit.

| Artifact | Source of truth for | Not source of truth for |
| --- | --- | --- |
| Workflow event stream | Run state, terminal status, state transitions, policy decision events, approvals, retries, escalation, cancellation, and immutable run identity. | Human-readable handoff narrative or raw evidence payloads. |
| Audit events | Operational history and accountability: who or what acted, when, under what run identity, and with what policy context. | Report conclusions, reasoning derivation, or full provider payloads. |
| Adapter telemetry | Read-only adapter invocation, response summary, redaction, latency, success/failure, and runtime-visible adapter context. | Provider source-of-truth data or generic live adapter execution. |
| Validation diagnostics | Deterministic loader and validator findings, severity, code, message, and source location. | Complete report readiness or raw spec content. |
| Approval decisions | Human grant or denial decisions against approval requests. | Full evidence packet storage or external identity provider audit. |
| Evidence references | Citation pointers to evidence and bounded/redacted summaries. | Underlying evidence payload truth, access control, or provider replay. |
| Work report | Governed terminal handoff artifact that assembles cited facts, decisions, validation, limitations, risks, and incomplete work. | Low-level operational history or reasoning graph. |
| Future reasoning lineage | Derivation and provenance relationships among claims, findings, evidence, decisions, corrections, and report sections. | Runtime state, audit log, or report text. |

Audit is operational history. Evidence reference is a citation pointer. Work report is a governed handoff artifact. Reasoning lineage is future derivation/provenance graph work.

## 5. Candidate Core Model

The initial implementation added the contract-side model first, followed by the `WorkReport` core model, an in-memory terminal local report generation helper, an in-memory runtime result exposure helper, explicit executor-integrated report-bearing execution for local runs, and an explicit local report artifact store. Automatic terminal report artifact generation from executor paths remains future work.

| Candidate type | Purpose |
| --- | --- |
| `WorkReportContract` | Implemented. Declares required report sections, citation expectations, redaction posture, sensitivity, disclosure requirements, and contract version for a class of workflows. |
| `WorkReportContractId` | Implemented. Stable ID for a report contract. |
| `WorkReportContractVersion` | Implemented. Stable version for a report contract. |
| `WorkReportSectionRequirement` | Implemented. Required domain-neutral section kind. |
| `WorkReportSectionKind` | Implemented. Domain-neutral section category such as work performed, evidence considered, approvals, validation, side effects, incomplete work, limitations, risks, or handoff notes. |
| `WorkReportCitationRequirement` | Implemented. Citation expectations for future report sections. |
| `WorkReportCitationKind` | Implemented. Candidate citation target kind such as evidence reference, workflow event, audit event, adapter telemetry, validation diagnostic, local check result, typed handoff, agent harness hook invocation, approval decision, policy decision, or future reasoning lineage node. |
| `WorkReportSensitivity` | Implemented. Conservative sensitivity classification for report contracts. |
| `WorkReportRedactionPolicy` | Implemented. Contract-level redaction posture for future reports. |
| `WorkReportDisclosureKind` | Implemented. Domain-neutral disclosure category for incomplete/deferred work, known limitations, risks, and side effects. |
| `WorkReportDisclosureRequirements` | Implemented. Typed disclosure requirement collection used by the contract model. |
| `WorkReport` | Implemented as a core model. Terminal handoff artifact shape for one workflow run under a report contract. |
| `WorkReportId` | Implemented. Stable ID for a generated work report model. |
| `WorkReportSection` | Implemented. One bounded section of a generated report model. |
| `WorkReportCitation` | Implemented. Concrete reference from a report section to evidence, events, audit records, adapter telemetry, diagnostics, local check results, typed handoffs, agent harness hook invocations, approvals, policy decisions, or future reasoning lineage nodes. |
| `WorkReportStatus` | Implemented. Terminal report status vocabulary, distinct from runtime generation behavior. |
| `WorkReportGenerationContext` | Implemented. Run identity, actor/system actor, correlation ID, and generation time used by the report model. |

The first implementation should avoid domain-specific report section names. Domain-specific sections belong later in templates or domain packs after the core model stabilizes.

## 6. Required Report Identity

A v1 `WorkReport` should require:

| Field | Purpose |
| --- | --- |
| `report_id` | Stable report identifier. |
| `report_contract_id` | Contract used to shape the report. |
| `report_contract_version` | Contract version used to validate/generate the report. |
| `workflow_id` | Workflow definition ID. |
| `workflow_version` | Workflow version. |
| `schema_version` | Workflow spec schema version. |
| `spec_hash` | Workflow spec content hash. |
| `run_id` | Workflow run ID. |
| `terminal_run_status` | Terminal outcome: completed, failed, canceled, escalated, or blocked where represented. |
| `generated_at` | Timestamp when report was generated. |
| `generated_by` | Actor or system actor that generated the report. |
| `correlation_id` | Correlation ID when available. |
| `redaction_metadata` | How report fields, summaries, and citations were redacted or treated as reference-only. |
| `sensitivity` | Report sensitivity classification, defaulted conservatively. |

Report identity must be bound to immutable run identity. A report must not be detached from the workflow version, schema version, run ID, or spec hash that produced it.

## 7. Required Sections For V1

A minimal v1 terminal report should require these sections:

| Section | Purpose |
| --- | --- |
| Work performed | What the workflow attempted or completed. |
| Evidence considered | Evidence references and other cited artifacts used during the work. |
| Decisions made | Policy, classification, review, routing, or operator decisions. |
| Policy gates evaluated | Policy allow, deny, approval-required, and reason-code context. |
| Approvals requested/granted/denied | Human approval requests and decisions, including denials and missing approvals where applicable. |
| Validation and quality checks | Validation diagnostics, quality gates, checks run, and accepted warnings or failures. |
| Side effects attempted/completed/skipped/denied | Explicit side-effect state, even when all side effects are none, skipped, denied, or unsupported. |
| Incomplete or deferred work | Work not completed, not attempted, blocked, skipped, or intentionally deferred. |
| Known limitations | Accepted limitations that affect trust, scope, or operator follow-up. |
| Risks | Residual risks, uncertainty, missing evidence, or operational concerns. |
| Operator handoff notes | What a human or downstream workflow should do next. |

Domain-specific sections should not be required in core v1. Later domain packs or templates may add legal, finance, security, engineering, procurement, support, HR, operations, or analytics sections without changing the core report model.

## 8. Citation Model

`WorkReportCitation` should allow report sections to cite stable references without copying payloads.

Candidate citation targets:

- `EvidenceReference`;
- workflow events;
- audit events;
- adapter telemetry records;
- validation diagnostics;
- approval decisions;
- policy decisions;
- future reasoning lineage nodes.

Rules:

- Cite stable references where available.
- Do not copy raw evidence payloads.
- Do not copy raw provider payloads.
- Do not copy raw CI logs, Jira bodies, GitHub file contents, command transcripts, or spec contents.
- Keep citation summaries bounded and redacted.
- Preserve redaction metadata and sensitivity at the report and citation level.
- Make missing citations explicit rather than inventing evidence.
- Do not create fake evidence references when a cited artifact is missing.
- If citation validation fails, report generation should fail or produce a clearly marked incomplete report according to a separately accepted runtime policy.

Candidate `WorkReportCitation` fields:

| Field | Purpose |
| --- | --- |
| `citation_kind` | Evidence reference, workflow event, audit event, adapter telemetry, validation diagnostic, approval decision, policy decision, or future reasoning lineage node. |
| `reference` | Stable ID or typed reference to the cited artifact. |
| `section_id` | Section that owns the citation. |
| `summary` | Optional bounded redacted summary, not payload. |
| `redaction_metadata` | Field handling for the citation. |
| `sensitivity` | Citation sensitivity, defaulted conservatively. |

## 9. Report Generation Timing

V1 report generation should be terminal-only.

Supported terminal contexts to design for:

- completed;
- failed;
- canceled;
- escalated;
- blocked, if represented by a future terminal or report status.

Do not support mid-run reports yet. Mid-run reporting would create new runtime expectations around report freshness, report mutation, and post-terminal metadata that the current event model does not support.

## 10. Should Every Workflow Require A Report?

No, not initially.

Recommended posture:

- Workflows should not require reports by default in the first implementation.
- Future workflows may opt in through report requirements only after the contract model is implemented and reviewed.
- Vertical slice and read-only examples should opt in only after terminal report generation exists and docs can explain it honestly.
- Mandatory reports for all workflows should be reconsidered after the model, validation rules, local artifact behavior, and operator value are proven.

This keeps report work from changing existing workflow semantics prematurely.

## 11. Runtime Enforcement Posture

Future implementation should separate report declaration, validation, generation, and enforcement.

Recommended conservative v1 behavior:

- Define `WorkReportContract` before runtime generation.
- Add validation rules for workflows that declare report requirements only after the contract model is reviewed.
- Generate reports only for workflows that explicitly opt in.
- Treat report generation failure as a report-generation failure, not as a silent workflow success.
- Do not retroactively change terminal run status unless a later accepted design explicitly says report generation is part of the workflow outcome.
- If a required report cannot be generated, emit or return a clear non-secret failure for the report path and preserve the original workflow run history.
- Avoid adding metadata-only events after terminal states unless the event model is separately updated and reviewed.

For the first model-only implementation, no runtime enforcement should be added.

## 12. Privacy And Redaction

Work reports will often be sensitive even when all cited integrations are read-only.

Rules:

- Store references and bounded summaries, not raw payloads.
- Do not store provider tokens, authorization headers, private keys, environment variable values, or credentials.
- Do not store raw provider payloads.
- Do not store raw CI logs.
- Do not store raw Jira descriptions or comments.
- Do not store raw GitHub file contents.
- Do not store raw command transcripts.
- Do not store raw spec contents.
- Default sensitivity conservatively.
- Require redaction metadata.
- Ensure Debug, Display, serialization intended for operators, and CLI/report output are redaction-safe.
- Treat file paths and provider metadata as potentially sensitive.
- Make incomplete/deferred work disclosure bounded and non-secret.

`WorkReportContract` is not enterprise DLP, access control, SIEM, OpenTelemetry, or a production compliance system.

## 13. Relationship To EvidenceReference

`EvidenceReference` is required before `WorkReport` because reports should cite evidence instead of copying it.

Expected relationship:

- Reports cite `EvidenceReference` values.
- Reports may include evidence reference IDs, kinds, titles, sensitivity, and bounded redacted summaries.
- Reports must not store the referenced payload.
- Reports should not create new `EvidenceReference` values implicitly unless a later scoped implementation defines safe generation rules.
- Report evidence completeness rules should be validated later through `WorkReportContract`.
- A report may explicitly disclose missing evidence rather than fabricating a citation.

EvidenceReference remains a citation substrate. Work reports use that substrate to explain a terminal handoff.

## 14. Relationship To Reasoning Lineage

Reasoning Lineage / Claim Graph remains future proposed architecture direction.

Work reports may later include:

- a reasoning lineage section;
- citations to reasoning nodes;
- citations from reasoning nodes to report sections;
- confidence, correction, or unresolved-claim metadata.

`WorkReportContract` should not block future reasoning lineage, but it should not implement claims, edges, confidence, corrections, context binding, or reasoning graph storage now.

## 15. Relationship To Side Effects And Writes

Side-effect boundary modeling must be accepted before writes. ADR 0011 is accepted and the SideEffect core model is implemented as a model-only boundary.

WorkReport v1 should include a side-effect section even when the report says:

- no side effects were supported;
- no side effects were attempted;
- side effects were skipped;
- side effects were denied;
- side effects are future work.

Future write-capable adapters must report side-effect state explicitly:

- proposed;
- approved;
- attempted;
- completed;
- denied;
- skipped;
- failed;
- rollback or compensation as future-only where honestly supported.

This plan does not implement writes, write-capable adapters, rollback, compensation, generic live adapter execution, automatic SideEffect discovery, or runtime side-effect execution. Terminal helper and executor SideEffect ID propagation are implemented separately, model-only SideEffect workflow event vocabulary is implemented, explicit local append support exists for proposed/denied/skipped SideEffect workflow events, and explicit local `SideEffectRecordStore` persistence is implemented for validated SideEffect records.

## 16. Candidate First Implementation Sequence

Implementation should proceed in small, reviewable phases:

1. Implement the `WorkReportContract` core type model only. Completed.
2. Implement the `WorkReport` core type model only. Completed.
3. Implement report section and citation types. Completed.
4. Add validation-only tests and redaction tests. Completed.
5. Add an in-memory terminal local report generation helper. Completed.
6. Plan runtime result exposure for in-memory reports. Completed in [Runtime Result Report Exposure Plan](runtime-result-report-exposure-plan.md).
7. Implement the in-memory runtime result exposure helper. Completed.
8. Plan executor-integrated report results. Completed in [Executor-Integrated Report Result Plan](executor-integrated-report-result-plan.md).
9. Implement explicit executor-integrated report-bearing execution for local runs. Completed.
10. Plan report artifacts. Completed in [Report Artifact Plan](report-artifact-plan.md).
11. Implement report artifact core/local store model only. Completed.
12. Add CLI inspect or report display later, after output posture is reviewed.
13. Update examples later, after report generation exposure is available and docs can describe it without overclaiming.
14. Run maintainer review before SideEffect record-store discovery, automatic report discovery, runtime side-effect execution, or write-capable adapter work.

The completed implementation prompts started with contract model types, added the report core model, added the in-memory terminal local helper, added the in-memory runtime result exposure helper, added explicit executor-integrated report-bearing execution for local runs, added an explicit local report artifact store, and hardened report/audit/missing-citation semantics with regression tests. The next implementation prompt should not automatically generate reports for every run, add approval/cancellation report-bearing methods, automatically write artifacts from executor paths, broaden persistence, change specs, add CLI rendering, or update examples unless separately approved.

## 17. Test Plan

Future implementation should include tests for:

- serialization and deserialization;
- required identity fields;
- required section validation;
- citation validation;
- `EvidenceReference` citation behavior;
- missing citation behavior;
- incomplete work disclosure required when work is incomplete, deferred, skipped, blocked, or failed;
- known limitations required when limitations are declared or discovered;
- redaction-safe Debug and Display behavior;
- no raw provider payload storage;
- no raw CI log, Jira body, GitHub file content, command transcript, or spec content storage;
- terminal status handling;
- report contract versioning;
- no domain-specific sections required in core;
- report generation disabled unless explicitly opted in, if generation is implemented later;
- docs honesty and no production-readiness overclaims.

## 18. Open Questions

- Should report generation failure fail a workflow?
- Should reports be mandatory for all workflows eventually?
- Should report contracts live in specs?
- Should reports be persisted in the local backend or derived on demand?
- Should CLI JSON expose reports?
- How should reports relate to audit retention?
- How should reports cite future reasoning lineage?
- How much natural language is allowed?
- Should report sections be extensible by domain packs?
- What is the smallest useful report for the vertical slice?
- Should approval evidence attachment happen before terminal report generation?
- Should report citations use typed IDs only, embedded evidence references, or both?
- How should report artifact corruption be diagnosed if local persistence is later added?

## 19. Final Recommendation

The `WorkReportContract` core model, `WorkReport` core model, in-memory terminal local report generation helper, in-memory runtime result exposure helper, explicit executor-integrated report-bearing execution for local runs, explicit local report artifact store, and report/audit/missing-citation semantics hardening are implemented. The next scoped phase should review the report/audit/missing-citation semantics before approval/cancellation report-bearing methods, automatic artifact generation, CLI rendering, schema exposure, or contract-driven missing-citation records.

Future prompts should not implement automatic runtime `WorkReport` generation for every run, automatic artifact writing from executor paths, CLI rendering, schema changes, example updates, approval attachment, reasoning lineage, automatic SideEffect discovery, runtime side-effect execution, writes, domain packs, production evidence storage, DLP, access control, SIEM/OpenTelemetry export, or release posture changes unless separately scoped and approved.
