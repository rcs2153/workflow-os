# Evidence Reference

`EvidenceReference` is a core concept for Workflow OS. Phase 1 implements the Rust core type model, serialization/deserialization, scope-specific validation, bounded metadata behavior, and redaction-safe display/debug behavior.

It is not implemented as a runtime persistence feature for evidence itself, schema, CLI command, evidence store, reasoning lineage model, domain pack, aggregate validation-result attachment, approval attachment, or write-capable adapter behavior.

Adapter telemetry evidence attachment is implemented for adapter invocation and runtime audit telemetry records only. That attachment remains reference-first, validates internally, and does not change persistence, CLI inspection, examples, writes, or release posture.

The purpose of `EvidenceReference` is simple: give Workflow OS a safe, domain-neutral way to point at evidence without copying raw sensitive payloads into workflow state.

## Why It Exists

Governed enterprise work depends on evidence. A reviewer, approver, auditor, operator, or downstream workflow needs to know what facts supported a decision, validation result, recommendation, approval, or report.

Today Workflow OS has several evidence-adjacent records:

- workflow events;
- audit events;
- adapter telemetry records;
- validation diagnostics;
- approval decisions;
- policy decisions;
- CLI output;
- release and readiness reviews.

Those records are valuable, but they do not provide a single reusable way to cite evidence. Without that shared model, future work reports could become prose summaries with weak citations, future domain packs could invent incompatible evidence shapes, and future reasoning lineage would lack a safe reference substrate.

`EvidenceReference` exists to make evidence citeable while keeping Workflow OS safe by default.

## Relationship To Governed Work Pattern

Governed Work Pattern is accepted architecture direction. It says serious AI-assisted enterprise work should bind context, evidence, policy, approvals, side-effect boundaries, validation, audit, observability, and structured reporting.

`EvidenceReference` is the first core MVP concept under that direction. Phase 1 answers:

- What evidence was actually used?
- Where can an operator find it?
- Which workflow/run/step/adapter/approval/validation context produced or cited it?
- How was it summarized or redacted?
- How sensitive is it?

It does not answer every governance question. Work report models and explicit local report helper/artifact APIs are implemented. Side-effect boundary architecture is accepted in [ADR 0011: Side-Effect Boundary Core Model](../adr/0011-side-effect-boundary.md), the model-only SideEffect core model is accepted in [SideEffect Core Model Review](SIDE_EFFECT_CORE_MODEL_REVIEW.md), WorkReport side-effect citation vocabulary is implemented as documented in [WorkReport SideEffect Citation Report](WORK_REPORT_SIDE_EFFECT_CITATION_REPORT.md), terminal report helper SideEffect citation propagation is implemented as documented in [Terminal Report SideEffect Citation Integration Report](TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REPORT.md), executor SideEffect report input propagation is accepted in [Executor SideEffect Report Input Propagation Review](EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REVIEW.md), model-only SideEffect workflow event vocabulary plus generic audit projection are implemented in [SideEffect Workflow Event Model Report](SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md), the first explicit local executor append path for proposed/denied/skipped SideEffect events is implemented in [Executor SideEffect Event Append Report](EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md), SideEffect persistence/discovery planning is documented in [SideEffect Persistence And Discovery Plan](../implementation-plans/side-effect-persistence-discovery-plan.md), explicit local SideEffect record persistence is implemented in [SideEffect Record Store Report](SIDE_EFFECT_RECORD_STORE_REPORT.md), executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md), report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](../implementation-plans/report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md), approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](../implementation-plans/approval-side-effect-linkage-plan.md), the validation-only helper is implemented in [SideEffect Approval Linkage Report](SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md), approval-side-effect linkage composition planning is documented in [Approval SideEffect Linkage Composition Plan](../implementation-plans/approval-side-effect-linkage-composition-plan.md), and the explicit store-backed approval linkage helper is implemented in [SideEffect Approval Linkage Store-Backed Report](SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REPORT.md) and accepted in [SideEffect Approval Linkage Store-Backed Review](SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REVIEW.md). Explicit executor report artifact writing with SideEffect integrity and approval-linkage gates is implemented in [Executor Report Artifact SideEffect Gates Report](EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REPORT.md), but automatic approval-side-effect validation in existing report/artifact paths, automatic artifact writes from existing executor paths, side-effect writes, schemas, CLI behavior, runtime side-effect execution, automatic SideEffect discovery, and EvidenceReference side-effect attachment are not implemented. Additional adapter/validation/approval attachment points, evidence persistence, CLI inspection, and reasoning lineage remain future scoped work.

Phase 2 attachment planning is documented in [EvidenceReference Attachment Plan](../implementation-plans/evidence-reference-attachment-plan.md). Adapter telemetry evidence attachment is implemented for the core adapter telemetry records. Diagnostic evidence attachment is implemented for the core `Diagnostic` model only. Automatic loader/validator evidence generation, aggregate `ValidationResult` evidence, validation success evidence, approval, persistence, CLI, example, work report, and reasoning lineage attachments remain unimplemented until separately scoped.

Validation call-site planning is documented in [EvidenceReference Validation Call-Site Attachment Plan](../implementation-plans/evidence-reference-validation-callsite-plan.md). The first validation call-site attachment is implemented for schema-version diagnostics with safe source/spec context. Automatic attachment to all diagnostics is not implemented.

Command-output evidence policy planning is documented in [Command Output Evidence Policy Plan](../implementation-plans/command-output-evidence-policy-plan.md). Command-output `EvidenceReference` attachment remains unimplemented. Current local check report paths cite `LocalCheckResultReference` values through WorkReport citations instead of creating command-output evidence.

## What It References

An evidence reference may point to:

- a local project file;
- a workflow spec file;
- a validation result or diagnostic;
- a workflow event;
- an audit event;
- an adapter invocation;
- an adapter response summary;
- an approval decision;
- a policy decision;
- an operator note;
- an external provider object;
- a test result;
- a command output summary;
- a release review;
- live smoke evidence.

The reference should be stable enough for review, but it does not guarantee the underlying provider object will remain readable forever.

## What It Does Not Store

`EvidenceReference` should not store raw sensitive evidence by default.

It must not store:

- provider tokens;
- authorization headers;
- private keys;
- environment variable values;
- raw CI logs;
- raw Jira descriptions or comments;
- raw large GitHub file contents;
- raw provider payloads;
- unredacted personal data;
- secrets copied from specs, shells, logs, or screenshots.

The default pattern is reference over copy, summary over payload, and redaction metadata over hidden assumptions.

## Examples Across Domains

| Domain | Example evidence reference |
| --- | --- |
| Legal | Contract clause reference, policy document section, reviewer note, approval decision. |
| Finance | Exception request, threshold calculation output, validation result, approver decision. |
| HR | Policy document reference, redacted employee-impacting context, human approval decision. |
| Security | Alert reference, triage command summary, audit event, escalation decision. |
| Procurement | Vendor intake record, compliance check result, risk review note. |
| Customer support | Case reference, policy citation, redacted customer context summary. |
| Operations | Local state health report, incident timeline event, diagnostic command output. |
| Data/analytics | Data quality check result, source table reference, validation output. |
| Software engineering | Pull request reference, changed-file summary, CI status summary, release review. |

Core names should remain domain-neutral. Provider-specific objects belong in adapters, examples, skills, or later domain packs.

## Relationship To Read-Only Adapters

The `0.2.0-preview.1` read-only adapters for GitHub, Jira, and GitHub Actions / CI already produce redacted adapter telemetry and response summaries.

Adapter telemetry records can now carry validated evidence references for adapter invocation and adapter response summary evidence. Future provider-specific attachment work should use safe references such as:

- GitHub repository metadata reference;
- GitHub file reference by path and ref;
- GitHub pull request metadata summary;
- Jira issue metadata reference;
- Jira comment summary reference;
- GitHub Actions workflow run reference;
- CI job status or failure summary reference.

This does not turn examples into generic live adapter execution. Fixture-first normal CI, opt-in live tests, and no-write boundaries remain unchanged.

## Relationship To Work Reports

Work reports should cite evidence references instead of copying evidence into report text.

`WorkReportContract` planning is documented in [WorkReportContract Planning Document](../implementation-plans/work-report-contract-plan.md). Terminal local report generation planning is documented in [Terminal Local Report Generation Plan](../implementation-plans/terminal-local-report-generation-plan.md). Runtime result exposure planning is documented in [Runtime Result Report Exposure Plan](../implementation-plans/runtime-result-report-exposure-plan.md). Executor-integrated report result planning is documented in [Executor-Integrated Report Result Plan](../implementation-plans/executor-integrated-report-result-plan.md). Report artifact planning is documented in [Report Artifact Plan](../implementation-plans/report-artifact-plan.md). Report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](../implementation-plans/report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md). Report/audit/missing-citation semantics are documented in [Report, Audit, And Missing-Citation Semantics Plan](../implementation-plans/report-audit-missing-citation-semantics-plan.md). The `WorkReportContract` and `WorkReport` core models are implemented and define future report contract direction, terminal report shape, section requirements, citation requirements, redaction posture, and sensitivity. An in-memory terminal local report generation helper is implemented and can cite `EvidenceReference` IDs by model without recreating evidence references. An in-memory runtime result exposure helper is implemented for pairing a terminal run with a generated report. `LocalExecutor::execute_with_report(...)` is implemented as an explicit additive local execution path that can return a run with an optional generated report. An explicit local report artifact store is implemented for validated `WorkReport` artifacts. Terminal report helper typed handoff citation integration and executor-integrated typed handoff report input propagation are implemented as documented in [Executor Typed Handoff Report Input Propagation Plan](../implementation-plans/executor-typed-handoff-report-input-plan.md). Report/audit/missing-citation semantics are hardened so reports remain derived handoff artifacts rather than audit events, report-generation errors remain separate from workflow execution errors, and absent optional references remain explicit section text rather than fabricated citations. The SideEffect core model is implemented and accepted, WorkReport side-effect citation vocabulary is implemented, terminal report SideEffect citation propagation is implemented as documented in [Terminal Report SideEffect Citation Integration Report](TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REPORT.md), executor SideEffect report input propagation is accepted in [Executor SideEffect Report Input Propagation Review](EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REVIEW.md), model-only SideEffect workflow event vocabulary plus generic audit projection are implemented in [SideEffect Workflow Event Model Report](SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md), the first explicit local executor append path for proposed/denied/skipped SideEffect events is implemented in [Executor SideEffect Event Append Report](EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md), SideEffect persistence/discovery planning is documented in [SideEffect Persistence And Discovery Plan](../implementation-plans/side-effect-persistence-discovery-plan.md), explicit local SideEffect record persistence is implemented in [SideEffect Record Store Report](SIDE_EFFECT_RECORD_STORE_REPORT.md), and executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md). Explicit executor report artifact writing with SideEffect integrity and approval-linkage gates is implemented in [Executor Report Artifact SideEffect Gates Report](EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REPORT.md), but automatic SideEffect discovery, runtime side-effect execution, and write support remain unimplemented. Automatic artifact writing from executor paths, automatic runtime report generation for every run, CLI rendering, examples, approval evidence attachment, approval/cancellation report-bearing methods, reasoning lineage, writes, schemas, and release posture changes remain unimplemented.

A work report might say:

- validation passed, citing a validation result evidence reference;
- an approval was granted, citing an approval decision evidence reference;
- a CI failure summary was considered, citing a CI adapter response summary reference;
- a limitation remains, citing a release review or operator note reference.

The report explains what was done and why it is ready, blocked, risky, incomplete, or escalated. The evidence reference points to the supporting evidence.

## Relationship To Future Reasoning Lineage

Reasoning Lineage / Claim Graph remains proposed future architecture direction.

If implemented later, reasoning lineage may connect claims, findings, corrections, confidence metadata, actor attribution, and context bindings to evidence references. The evidence reference should remain a citation substrate, not a reasoning graph by itself.

## Relationship To Future Composable Harness Contracts

Composable Harness Contracts are a future Governed Work Pattern capability, not a current runtime feature. A future harness is a bounded execution envelope inside a workflow with typed inputs, typed outputs, scoped authority, evidence requirements, approval rules, failure semantics, and handoff obligations. The Composable Harness Contract core model and typed handoff core model are implemented as model-only foundations. WorkReport typed handoff citation planning is documented in [WorkReport Typed Handoff Citation Plan](../implementation-plans/work-report-typed-handoff-citation-plan.md), and WorkReport citation vocabulary for typed handoffs is implemented. Terminal report helper typed handoff integration is implemented in [Terminal Report Typed Handoff Citation Integration Plan](../implementation-plans/terminal-report-typed-handoff-citation-integration-plan.md). Executor-integrated typed handoff report input propagation is implemented in [Executor Typed Handoff Report Input Propagation Plan](../implementation-plans/executor-typed-handoff-report-input-plan.md). WorkReport hook citation target planning is documented in [WorkReport Agent Harness Hook Citation Target Plan](../implementation-plans/work-report-hook-citation-target-plan.md), WorkReport citation vocabulary for agent harness hook invocation IDs is implemented as model-only vocabulary, terminal report helper hook citation integration is implemented in [Terminal Report Agent Harness Hook Citation Integration Plan](../implementation-plans/terminal-report-hook-citation-integration-plan.md) for explicit supplied IDs only, executor report input propagation for hook IDs is implemented in [Executor Hook Report Input Propagation Plan](../implementation-plans/executor-hook-report-input-plan.md), runtime hook execution planning is documented in [Agent Harness Hook Runtime Execution Plan](../implementation-plans/agent-harness-hook-runtime-execution-plan.md), the explicit in-memory runtime hook execution helper is implemented, executor hook checkpoint planning is documented in [Executor Hook Checkpoint Plan](../implementation-plans/executor-hook-checkpoint-plan.md), deterministic required-checkpoint enforcement for explicit `BeforeReport` report paths is implemented in [Deterministic Hook Checkpoint Enforcement Report](DETERMINISTIC_HOOK_CHECKPOINT_ENFORCEMENT_REPORT.md), executor hook event/audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](../implementation-plans/executor-hook-event-audit-semantics-plan.md), hook event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](../implementation-plans/hook-event-audit-projection-plan.md), the first explicit `BeforeSkillInvocation` executor hook event append path is implemented in [Executor Hook Event Append Plan](../implementation-plans/executor-hook-event-append-plan.md), BeforeSkillInvocation status/failure semantics planning plus boundary hardening are documented in [BeforeSkillInvocation Hook Status And Failure Semantics Plan](../implementation-plans/before-skill-hook-status-failure-semantics-plan.md), the first explicit failed-closed result path is implemented in [BeforeSkillInvocation Failed-Closed Result Path Plan](../implementation-plans/before-skill-hook-failed-closed-result-plan.md), warning/skipped disclosure semantics planning is documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](../implementation-plans/before-skill-hook-warning-skipped-disclosure-plan.md), unsupported-status hardening tests are implemented in [BeforeSkillInvocation Unsupported Status Hardening Report](BEFORE_SKILL_HOOK_UNSUPPORTED_STATUS_HARDENING_REPORT.md), required pre-skill checkpoint planning is documented in [BeforeSkillInvocation Required Checkpoint Plan](../implementation-plans/before-skill-required-checkpoint-plan.md), the first explicit selected-step required enforcement slice is implemented in [BeforeSkillInvocation Required Checkpoint Enforcement Report](BEFORE_SKILL_REQUIRED_CHECKPOINT_ENFORCEMENT_REPORT.md), the unknown required-step blocker is fixed in [BeforeSkillInvocation Required Checkpoint Blocker Fix Report](BEFORE_SKILL_REQUIRED_CHECKPOINT_BLOCKER_FIX_REPORT.md), the bounded hook disclosure core model is implemented as documented in [Hook Disclosure Model Plan](../implementation-plans/hook-disclosure-model-plan.md), WorkReport hook disclosure citation vocabulary is implemented as model-only vocabulary as documented in [WorkReport Hook Disclosure Citation Plan](../implementation-plans/work-report-hook-disclosure-citation-plan.md), terminal report helper hook disclosure citation integration is implemented in [Terminal Report Hook Disclosure Citation Integration Plan](../implementation-plans/terminal-report-hook-disclosure-citation-integration-plan.md) for explicit supplied IDs only, executor hook disclosure report input propagation is implemented in [Executor Hook Disclosure Report Input Propagation Plan](../implementation-plans/executor-hook-disclosure-report-input-plan.md), and hook disclosure discovery planning plus the first in-memory implementation are documented in [Hook Disclosure Discovery Plan](../implementation-plans/hook-disclosure-discovery-plan.md). The explicit `BeforeReport` executor checkpoint is implemented for `execute_with_report(...)` only and remains report-path-only, in-memory-only, and non-mutating; explicit report-bearing callers can require it before report generation, and it can discover hook disclosure IDs only from the already-validated in-memory `BeforeReport` hook result. `Passed` remains the only continuing hook status today; explicit `FailedClosed` hook input now appends requested/evaluated events and fails before `SkillInvocationRequested`. Local execution requests can now require `BeforeSkillInvocation` for explicit selected step IDs; missing or mismatched required hook input fails closed before hook or skill invocation events, and unknown required step IDs fail closed before run creation. Broader automatic executor hook invocation, workflow-declared hook configuration, runtime hook configuration, discovery from workflow events or audit projections, warning/skipped/blocked status broadening, dedicated hook audit sink emission, runtime handoff execution, nested harness execution, persistence, CLI behavior, schemas, side effects, writes, and reasoning lineage are not implemented.

Executor hook disclosure report input propagation forwards explicitly supplied hook disclosure IDs. The explicit `BeforeReport` report path can also discover hook disclosure IDs from the already-validated in-memory hook result it just executed; discovery from workflow events, audit projections, durable stores, text, diagnostics, local checks, and adapter telemetry remains unimplemented.

EvidenceReference is one prerequisite for that direction because auditable delegation needs references to validation results, adapter telemetry, audit events, policy decisions, approvals, and final work reports without copying raw payloads into handoffs. This does not make Workflow OS a generic multi-agent framework, does not add nested harness execution, and does not change persistence, CLI, schema, side-effect, write, or release posture.

The agent harness onboarding scaffold (`workflow-os init-agent-harness`) is documentation-only. It can help users point a coding agent at the local kernel, but it does not create evidence references, execute workflows, attach evidence, persist evidence, render evidence in the CLI, or change evidence validation boundaries.

## Privacy And Redaction Principles

Evidence references must be conservative:

- Use references instead of copying payloads.
- Use summaries instead of raw content.
- Include redaction metadata.
- Classify sensitivity or default to conservative sensitivity.
- Treat provider metadata as sensitive unless explicitly safe.
- Ensure debug and display output are redaction-safe.
- Do not assume the current user can still access referenced external evidence.

`EvidenceReference` is not enterprise DLP, access control, provider replay, or a production evidence store. Those require separate scoped work.

## Current Status

Phase 1 core model implemented.

ADR 0009 proposes the core model and MVP implementation plan. The current implementation includes only the core Rust model and Phase 1 safety tests.

Phase 2 adapter telemetry attachment is implemented for adapter invocation and runtime audit telemetry records. Diagnostic evidence attachment is implemented for the core `Diagnostic` model. The first validation call-site attachment is implemented for schema-version diagnostics that already have safe source/spec context. Aggregate `ValidationResult` evidence attachment, broad automatic loader/validator evidence attachment, validation success evidence, and other attachment behavior are not implemented.

Validation call-site attachment remains limited to the selected schema-version diagnostic family.

Not implemented:

- local persistence;
- CLI inspection;
- example integration;
- aggregate validation result attachment;
- automatic loader or semantic validator evidence attachment;
- validation success evidence attachment;
- approval decision attachment;
- automatic WorkReport artifact writing from executor paths;
- runtime WorkReport generation;
- automatic runtime report generation for every run;
- approval/cancellation report-bearing methods;
- command-output evidence attachment;
- Composable Harness Contracts;
- nested harness execution;
- Reasoning Lineage / Claim Graph;
- runtime side-effect execution;
- automatic SideEffect discovery;
- write support;
- domain packs;
- production evidence store;
- DLP or access-control systems.
