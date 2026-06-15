# Work Reports

Work reports are the governed handoff artifact for Workflow OS work. They explain what work was performed, what evidence was considered, what decisions were made, what validation or quality checks ran, what approvals or policy gates mattered, what side effects were skipped or unsupported, what remains incomplete, and what risks or operator handoff notes should follow the run.

Work reports are not audit logs. Audit records preserve operational history. Evidence references provide citation pointers. Work reports are structured summaries that cite stable references instead of copying raw payloads.

## Current Implementation

The current core implementation includes:

- `WorkReportContract` and related contract model types;
- `WorkReport` and related report, section, citation, disclosure, limitation, risk, and handoff-note model types;
- redaction-safe validation, serialization, deserialization, and `Debug` behavior;
- `generate_terminal_local_work_report(...)` for explicit in-memory report construction from terminal local run inputs;
- `TerminalLocalWorkReportResult` and `expose_terminal_local_work_report_result(...)` for explicit in-memory pairing of a terminal run and report;
- `LocalExecutor::execute_with_report(...)` as an additive local executor path that can return a run plus an optional report and report-generation error;
- `WorkReportArtifactRecord` and `WorkReportArtifactStore` for explicit local artifact storage of validated reports.

The implementation is local, explicit, and model/helper focused. Existing CLI commands do not render work reports, and existing executor paths do not write report artifacts automatically.

## Required V1 Sections

The core v1 report shape supports these domain-neutral sections:

- work performed;
- evidence considered;
- decisions made;
- policy gates evaluated;
- approvals;
- validation and quality checks;
- side effects;
- incomplete or deferred work;
- known limitations;
- risks;
- operator handoff notes.

Domain-specific report templates remain future work and should live outside the core model unless separately scoped.

## Citation Model

Work report citations use stable references rather than raw payloads. Supported citation vocabulary includes:

- evidence reference IDs;
- workflow event IDs;
- audit event IDs;
- adapter telemetry references;
- validation diagnostic references;
- policy decision references;
- approval decision references;
- future reasoning-lineage references as vocabulary only.

Work reports must not recreate `EvidenceReference` values implicitly, fabricate missing IDs, or copy provider payloads into report text.

## Privacy Boundary

Work reports and report artifacts are sensitive by default.

They must not store or copy:

- raw provider payloads;
- raw CI logs;
- raw Jira issue or comment bodies;
- raw GitHub file contents;
- raw command output;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Summaries and notes must be bounded and redaction-aware. Debug output and validation errors must not leak secret-like values.

## What Is Not Implemented

The current implementation does not include:

- automatic runtime report generation for every run;
- automatic report artifact writing from executor paths;
- CLI report rendering or export;
- workflow spec schema fields for report requirements;
- example integration;
- approval-resume or cancellation report-bearing executor APIs;
- production report storage;
- report signing or notarization;
- Reasoning Lineage / Claim Graph implementation;
- side-effect boundary modeling;
- write-capable adapter behavior.

## Relationship To Other Concepts

- [Governed Work Pattern](governed-work-pattern.md) provides the operating model that work reports support.
- [Evidence Reference](evidence-reference.md) provides the citation substrate work reports should use.
- [Auditability](auditability.md) covers operational history, which work reports may cite but should not duplicate.
- [Reasoning Lineage / Claim Graph](reasoning-lineage.md) remains future provenance work that may later connect claims, evidence, decisions, and reports.

Implementation plans and phase reports:

- [WorkReportContract Planning](../implementation-plans/work-report-contract-plan.md)
- [Terminal Local Report Generation Plan](../implementation-plans/terminal-local-report-generation-plan.md)
- [Runtime Result Report Exposure Plan](../implementation-plans/runtime-result-report-exposure-plan.md)
- [Executor-Integrated Report Result Plan](../implementation-plans/executor-integrated-report-result-plan.md)
- [Report Artifact Plan](../implementation-plans/report-artifact-plan.md)
