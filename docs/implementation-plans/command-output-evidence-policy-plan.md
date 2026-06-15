# Command Output Evidence Policy Plan

Status: Planning only. Command-output `EvidenceReference` attachment is not implemented. Current report paths should cite local check result references instead of creating command-output evidence.

## 1. Executive Summary

`EvidenceKind::CommandOutput` exists in the core evidence vocabulary, but command output is one of the highest-risk evidence categories in Workflow OS.

Local checks already have bounded `LocalCheckResult` values, validated `LocalCheckResultReference` values, and `WorkReportCitationTarget::LocalCheckResult` citations. Terminal local reports can cite supplied local check result references without recreating `EvidenceReference` values and without copying raw stdout, stderr, command transcripts, parser payloads, provider payloads, environment values, or secrets.

This plan defines the policy boundary for any future command-output evidence work. It does not implement command-output evidence attachment, local check persistence, automatic local check citation wiring, CLI rendering, schemas, examples, side-effect modeling, writes, or release posture changes.

## 2. Goals

- Preserve Workflow OS's reference-first evidence posture.
- Prevent command-output evidence from becoming raw log storage.
- Keep local check result references as the preferred current citation path.
- Define when `EvidenceKind::CommandOutput` may be considered in the future.
- Define allowed and forbidden data for command-output evidence.
- Require bounded summaries, redaction metadata, sensitivity, and stable references for any future command-output evidence.
- Preserve deterministic local validation and report behavior.
- Prepare future implementation prompts only after the policy is accepted.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- command-output `EvidenceReference` attachment;
- automatic evidence creation from local check handlers;
- automatic evidence creation from terminal report generation;
- raw stdout storage;
- raw stderr storage;
- raw command transcript storage;
- local check result persistence;
- report artifact changes;
- CLI rendering or inspection;
- workflow schema fields;
- examples;
- default handler registration;
- automatic local check execution;
- side-effect boundary modeling;
- writes;
- DLP or access-control systems;
- release posture changes.

## 4. Governance Check

This planning phase was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-command-output-evidence-policy-plan`
- Run ID: `run-1781543362724223000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781543362724223000-2/d`
- Final status: `Completed`

The governed run completed before documentation edits were made.

## 5. Current Baseline

Implemented:

- `EvidenceKind::CommandOutput` vocabulary;
- `EvidenceReferenceTarget::command_output(...)`;
- evidence validation requiring command-output evidence to include meaningful redaction metadata;
- serialization behavior that redacts command-output target summaries;
- bounded `LocalCheckResult` values;
- redaction-safe `LocalCheckResult` `Debug`;
- `LocalCheckResultReference` values that do not copy stdout or stderr summaries;
- `WorkReportCitationTarget::LocalCheckResult`;
- terminal report helper integration for supplied stable local check result references.

Not implemented:

- command-output evidence attachment;
- local check evidence attachment;
- automatic local check result reference creation from handlers or executor paths;
- local check result persistence;
- command-output CLI inspection;
- command-output report artifacts;
- workflow schema fields for command-output evidence.

## 6. Risk Statement

Command output often contains sensitive information:

- tokens or credentials accidentally printed by tools;
- environment variable values;
- authorization headers;
- private keys;
- absolute paths revealing repository or operator structure;
- raw CI logs;
- raw provider payloads;
- parser payloads;
- raw spec contents;
- stack traces with secrets;
- shell commands and arguments containing sensitive values.

Because of that, command-output evidence must never become a default transcript or log-storage mechanism.

## 7. Recommended Policy

The default policy is:

- do not use `EvidenceKind::CommandOutput` for local check result reporting today;
- cite `LocalCheckResultReference` through `WorkReportCitationTarget::LocalCheckResult`;
- treat local check citations as report citations, not evidence records;
- do not create `EvidenceReference` values implicitly during report generation;
- defer command-output evidence attachment until a separate accepted implementation prompt defines a specific safe use case.

Future command-output evidence should be allowed only when it is reference-first and bounded. It should point to a stable local check result reference, safe artifact reference, or redacted command-output summary reference. It must not embed raw stdout, stderr, transcripts, or logs.

## 8. Allowed Data For Future Command-Output Evidence

Future command-output evidence may consider only bounded, validated fields such as:

- stable local check result reference ID;
- command contract ID;
- command kind;
- result status;
- workflow ID;
- run ID;
- workflow event ID, if available;
- audit event ID, if available;
- stable output reference, if separately approved;
- non-secret result status summary;
- truncation flags;
- stable error code;
- content hash only when the hashed material and retention posture are separately approved;
- redaction metadata;
- sensitivity;
- retention hint.

Allowed data must still pass EvidenceReference validation and any command-output-specific attachment validator.

## 9. Forbidden Data

Command-output evidence must not store or serialize:

- raw stdout;
- raw stderr;
- full command transcripts;
- raw shell command lines containing sensitive arguments;
- raw CI logs;
- raw provider payloads;
- parser payloads;
- raw workflow spec contents;
- raw docs or source file contents;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- Jira issue bodies or comments;
- GitHub file contents;
- unbounded summaries;
- secret-like metadata values.

## 10. Evidence Kind And Scope Rules

If a future phase introduces command-output evidence, it should use:

- `EvidenceKind::CommandOutput` only for bounded command-output references;
- `EvidenceScope::Run` only when real run identity is present;
- `EvidenceScope::Validation` only for validation/check command outcomes;
- `EvidenceScope::Workflow` only when the reference is meaningful outside a single run and does not invent workflow context.

Unsafe or confusing combinations:

- `CommandOutput` with no run or validation context when the output came from a specific run;
- `CommandOutput` attached to diagnostics by default;
- `CommandOutput` with `EvidenceScope::External` unless the target is an external safe reference;
- `CommandOutput` created from report generation without an existing command/check result source.

## 11. Attachment Boundary Requirements

Any future command-output evidence attachment must:

- use a dedicated validated API;
- call `EvidenceReference::validate()`;
- add command-output-specific validation on top of generic evidence validation;
- require redaction metadata with at least one redacted or reference-only field;
- require conservative sensitivity;
- fail closed on invalid evidence;
- attach atomically when multiple references are supplied;
- expose read-only accessors only;
- validate deserialization;
- return stable non-leaking errors.

No implementation should rely on a caller's claim that command output was already reviewed elsewhere.

## 12. Relationship To LocalCheckResult

`LocalCheckResult` is the bounded result model for local command/check outcomes. It can store bounded stdout and stderr summaries after validation, but it remains separate from evidence.

`LocalCheckResultReference` is the current preferred citation surface. It identifies the check result without copying stdout summaries, stderr summaries, command transcripts, or raw process output.

Future command-output evidence should not duplicate `LocalCheckResult`. If it is needed, it should cite an existing local check result reference or stable artifact reference rather than storing output fields again.

## 13. Relationship To Work Reports

WorkReports can already cite supplied local check result references in the `validation and quality checks` section.

Report generation must not:

- create command-output evidence implicitly;
- recreate `EvidenceReference` values;
- copy local check stdout or stderr summaries into report text by default;
- copy raw command output;
- turn missing command output into fabricated evidence.

If command-output evidence is introduced later, WorkReports should cite its stable ID only when the evidence was created by a separately governed source.

## 14. Relationship To Persistence And Artifacts

Command-output evidence policy does not authorize persistence or artifacts.

A later artifact phase would need to define:

- where the artifact lives;
- how it is redacted;
- how long it is retained;
- who may read it;
- whether hashes are safe and useful;
- how report artifacts cite it without copying it;
- how deletion or retention failure is represented.

Until then, command-output evidence should remain unimplemented.

## 15. Future Test Plan

A future implementation must add tests proving:

- raw stdout is rejected or redacted before storage;
- raw stderr is rejected or redacted before storage;
- command transcripts are rejected;
- secret-like summaries are rejected;
- secret-like metadata is rejected;
- safe reference-only command-output evidence validates;
- safe command-output evidence requires redaction metadata;
- safe command-output evidence defaults to conservative sensitivity;
- invalid serialized command-output evidence fails closed;
- deserialization errors do not leak raw values;
- `Debug` output does not leak output or metadata;
- serialization does not leak raw output;
- local check result references continue to work without evidence;
- WorkReports continue to cite local check results without creating evidence.

## 16. Open Questions

- Is command-output evidence needed now that WorkReports can cite local check result references?
- Should Workflow OS add a distinct `EvidenceKind::LocalCheckResult` instead of using `CommandOutput` for local check outcomes?
- Should command-output evidence ever include bounded summaries, or only stable references?
- Should command-output evidence require a persisted artifact before it can exist?
- How should command-output evidence interact with future report artifacts?
- Should command-output evidence be permitted for validation diagnostics, or only for report/release review contexts?
- What retention policy is acceptable for command-output evidence?

## 17. Final Recommendation

Defer command-output evidence attachment.

The current safe path is to keep citing local check results through `WorkReportCitationTarget::LocalCheckResult`. If command-output evidence becomes necessary, the next implementation should start with a narrow command-output evidence validator and tests for reference-only command-output evidence. It should not attach evidence to handlers, reports, diagnostics, persistence, artifacts, CLI output, schemas, examples, side effects, writes, or release posture in the same phase.
