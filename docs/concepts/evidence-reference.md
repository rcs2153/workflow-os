# Evidence Reference

`EvidenceReference` is a core concept for Workflow OS. Phase 1 implements the Rust core type model, serialization/deserialization, scope-specific validation, bounded metadata behavior, and redaction-safe display/debug behavior.

It is not implemented as a runtime persistence feature, schema, CLI command, local backend store, runtime work report generator, reasoning lineage model, domain pack, validation-result attachment, approval attachment, or write-capable adapter behavior.

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

It does not answer every governance question. Work reports, side-effect boundary modeling, adapter/validation/approval attachment points, local persistence, CLI inspection, and reasoning lineage remain future scoped work.

Phase 2 attachment planning is documented in [EvidenceReference Attachment Plan](../implementation-plans/evidence-reference-attachment-plan.md). Adapter telemetry evidence attachment is implemented for the core adapter telemetry records. Diagnostic evidence attachment is implemented for the core `Diagnostic` model only. Automatic loader/validator evidence generation, aggregate `ValidationResult` evidence, validation success evidence, approval, persistence, CLI, example, work report, and reasoning lineage attachments remain unimplemented until separately scoped.

Validation call-site planning is documented in [EvidenceReference Validation Call-Site Attachment Plan](../implementation-plans/evidence-reference-validation-callsite-plan.md). The first validation call-site attachment is implemented for schema-version diagnostics with safe source/spec context. Automatic attachment to all diagnostics is not implemented.

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

## Relationship To Future Work Reports

Future work reports should cite evidence references instead of copying evidence into report text.

`WorkReportContract` planning is documented in [WorkReportContract Planning Document](../implementation-plans/work-report-contract-plan.md). Terminal local report generation planning is documented in [Terminal Local Report Generation Plan](../implementation-plans/terminal-local-report-generation-plan.md). Runtime result exposure planning is documented in [Runtime Result Report Exposure Plan](../implementation-plans/runtime-result-report-exposure-plan.md). Executor-integrated report result planning is documented in [Executor-Integrated Report Result Plan](../implementation-plans/executor-integrated-report-result-plan.md). The `WorkReportContract` and `WorkReport` core models are implemented and define future report contract direction, terminal report shape, section requirements, citation requirements, redaction posture, and sensitivity. An in-memory terminal local report generation helper is implemented and can cite `EvidenceReference` IDs by model without recreating evidence references. An in-memory runtime result exposure helper is implemented for pairing a terminal run with a generated report. `LocalExecutor::execute_with_report(...)` is implemented as an explicit additive local execution path that can return a run with an optional generated report. Automatic runtime report generation for every run, generated report artifacts, persistence, CLI rendering, examples, approval evidence attachment, approval/cancellation report-bearing methods, reasoning lineage, side-effect modeling, writes, schemas, and release posture changes remain unimplemented.

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

Composable Harness Contracts are a future Governed Work Pattern capability, not a current runtime feature. A future harness is a bounded execution envelope inside a workflow with typed inputs, typed outputs, scoped authority, evidence requirements, approval rules, failure semantics, and handoff obligations.

EvidenceReference is one prerequisite for that direction because auditable delegation needs references to validation results, adapter telemetry, audit events, policy decisions, approvals, and final work reports without copying raw payloads into handoffs. This does not make Workflow OS a generic multi-agent framework, does not add nested harness execution, and does not change persistence, CLI, schema, side-effect, write, or release posture.

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
- generated WorkReport artifacts;
- runtime WorkReport generation;
- automatic runtime report generation for every run;
- approval/cancellation report-bearing methods;
- Composable Harness Contracts;
- nested harness execution;
- Reasoning Lineage / Claim Graph;
- side-effect boundary model;
- write support;
- domain packs;
- production evidence store;
- DLP or access-control systems.
