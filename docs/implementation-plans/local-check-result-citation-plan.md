# Local Check Result Citation Plan

Status: Local check result reference model implemented and reviewed. WorkReport local check citation target planning is documented in [WorkReport Local Check Result Citation Target Plan](work-report-local-check-citation-target-plan.md), and WorkReport citation vocabulary for local check results is implemented. Terminal report helper integration for supplied local check result references is implemented and documented in [Terminal Report Local Check Citation Integration Plan](terminal-report-local-check-citation-integration-plan.md). Evidence attachment, default handler registration, CLI behavior, schema changes, artifacts, persistence, side-effect modeling, source writes, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has a structured local check command contract model, bounded `LocalCheckResult` values, an explicit production-shaped `DocsCheckLocalHandler`, and an explicit non-default registry helper for callers that supply a `DocsCheckLocalHandler`.

The next question is how local check outcomes should become citeable by future work reports and evidence flows without turning command output into durable logs.

This plan recommends a narrow model-first path: define stable local check result references before wiring those references into `WorkReport` generation or `EvidenceReference`. The first implementation added a validated local check result reference/identity model only. It does not attach evidence, create report citations automatically, persist check results, expose CLI behavior, register handlers by default, or change local executor semantics.

## 2. Goals

- Make local check outcomes citeable by stable references.
- Preserve bounded, redaction-safe `LocalCheckResult` behavior.
- Avoid copying raw stdout, stderr, command transcripts, docs contents, parser payloads, provider payloads, or environment values.
- Prepare future `WorkReport` `validation and quality checks` sections to cite local check outcomes.
- Prepare future evidence policy decisions without using `EvidenceKind::CommandOutput` prematurely.
- Preserve deterministic local execution semantics and event ordering.
- Keep command authority explicit and non-default.
- Keep report artifact writing, persistence, CLI rendering, workflow schema fields, and default registration deferred.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- local check result persistence;
- local check result artifact writing;
- automatic work-report citation;
- automatic evidence attachment;
- `EvidenceKind::CommandOutput` usage;
- raw command-output storage;
- default `DocsCheck` registration;
- CLI handler exposure;
- CLI report/check rendering;
- workflow schema fields;
- automatic local check execution;
- `AllowlistedHandlerOnly` enablement;
- broader cargo/npm/check handlers;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- live provider access;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Governance Check

This planning phase was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-local-check-result-citation-plan`
- Run ID: `run-1781506690174487000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781506690174487000-2/d`
- Final status: `Completed`

The governed run completed before documentation edits were made.

## 5. Current Baseline

Implemented:

- `LocalCheckCommandId`;
- `LocalCheckCommandKind`;
- `LocalCheckCommandContract`;
- canonical command-template binding;
- `LocalCheckResult`;
- bounded stdout/stderr summary validation;
- redaction-safe `LocalCheckResult` debug behavior;
- `SkillOutput::output_ref` value derived from local check result data;
- `DocsCheckLocalHandler`;
- explicit non-default `LocalSkillRegistry::register_docs_check_handler(...)`;
- `WorkReportCitationTarget` vocabulary for `EvidenceReference`, workflow events, audit events, adapter telemetry, validation diagnostics, approval decisions, policy decisions, and future reasoning lineage nodes.

Implemented after this plan:

- first-class `LocalCheckResultId`;
- first-class `LocalCheckResultReference`;
- validated local check result reference construction;
- helper construction from an existing `LocalCheckResult` plus explicit run/event context;
- serde validation and redaction-safe `Debug` for local check result references.

Not implemented:

- `WorkReportCitationTarget` variant for local check results;
- durable local check result store;
- local check result work-report citation wiring;
- local check result evidence attachment;
- command-output evidence policy;
- default handler registration;
- CLI/schema exposure.

## 6. Citation Boundary

Local check citation should cite stable local check result references, not raw process output.

Allowed future citation inputs:

- local check result reference ID;
- local check command ID;
- local check command kind;
- result status;
- workflow ID;
- run ID;
- workflow event ID for the local skill invocation, if available;
- audit event ID for the skill invocation or policy decision, if available;
- bounded, redacted result summary generated through `LocalCheckResult`;
- truncation flags;
- stable error code, if present.

Forbidden citation inputs:

- raw stdout;
- raw stderr;
- complete command transcripts;
- environment variables or values;
- npm tokens or registry credentials;
- provider credentials;
- raw docs/spec contents;
- parser payloads;
- CI logs;
- paths containing secrets;
- private keys;
- authorization headers;
- token-like values.

## 7. Candidate Model Additions

The first implementation should add the smallest reference model needed for later citation.

Candidate types:

- `LocalCheckResultId`
- `LocalCheckResultReference`
- `LocalCheckResultReferenceDefinition`

Candidate `LocalCheckResultReference` fields:

- result ID;
- command ID;
- command kind;
- result status;
- workflow ID;
- run ID;
- optional workflow event ID;
- optional audit event ID;
- optional stable output reference;
- redaction metadata;
- sensitivity.

The reference should not include stdout/stderr summaries in the first implementation unless there is a strong reason. The existing `LocalCheckResult` already owns bounded summaries; the reference should identify the result rather than duplicate result contents.

## 8. Stable Reference Format

Recommended stable ID format:

- `local-check-result/<run-id>/<command-id>/<sequence-or-result-token>`

The exact token should be generated or validated by model constructors, not assembled ad hoc at call sites.

Rules:

- IDs must be bounded.
- IDs must use safe identifier characters.
- IDs must reject secret-like values.
- IDs must not include raw paths.
- IDs must not include command output.
- IDs must not encode environment values.
- Debug output must redact the underlying ID value if that matches repository identifier patterns.

## 9. Relationship To WorkReport

Work reports should eventually cite local check results in the `validation and quality checks` section.

Future options:

1. Add `WorkReportCitationTarget::LocalCheckResult { reference: WorkReportStableReference }`.
2. Reuse `WorkReportCitationTarget::ValidationDiagnostic` only for diagnostics produced by validation, not process checks.
3. Cite workflow/audit events for check invocation and include not-available text until a local check result citation target exists.

Recommendation: add an explicit `LocalCheckResult` citation target in a separate implementation after the reference model is reviewed. This keeps local process-check outcomes distinct from schema/semantic validation diagnostics and avoids overloading `ValidationReferenceId`.

This plan does not implement that citation target.

## 10. Relationship To EvidenceReference

Evidence integration should remain deferred.

Local check results may eventually support evidence references, but command-output evidence is high risk because it can become log storage. Future evidence planning must decide whether to use:

- `EvidenceKind::ValidationResult` for structured check results;
- a future check-result evidence kind, if added through an ADR or scoped plan;
- `EvidenceKind::CommandOutput` only for bounded, summarized command-output references after a dedicated command-output evidence policy review.

This plan recommends no `EvidenceReference` creation for local checks yet.

## 11. Relationship To Runtime Events And Audit

Local check result references should connect to existing runtime and audit records where available.

Future references may include:

- workflow event ID for skill invocation;
- workflow event ID for skill success/failure;
- audit event ID for policy decisions;
- audit event ID for adapter or skill invocation, if stable and available.

Rules:

- Do not append post-terminal events solely to make a citation available.
- Do not mutate historical events.
- Do not fabricate event IDs.
- If event IDs are unavailable, represent that as absent reference data or not-available report text.
- Preserve existing runtime event ordering.

## 12. Privacy And Redaction Policy

Local check result references must be sensitive by default.

Rules:

- Use validated constructors.
- Keep summaries bounded and redacted when summaries are included at all.
- Treat file paths as potentially sensitive.
- Require redaction metadata if the reference includes any redaction-relevant field.
- Reject secret-like reference IDs, output refs, metadata field names, and metadata reasons.
- Keep `Debug` redaction-safe.
- Keep serde validation fail-closed.
- Ensure deserialization errors do not include raw caller values.

## 13. Failure Semantics

Reference construction failures should be internal construction errors, not misleading user project diagnostics.

Rules:

- Invalid reference data fails closed.
- No partial reference should be created.
- No fake citation should be emitted.
- Original local check result semantics should remain unchanged.
- Report generation should continue to preserve workflow semantics when later integration is implemented.
- Errors must use stable codes and avoid raw values.

Recommended stable error code prefix:

- `local_check_result_reference.*`

## 14. First Implementation Recommendation

Implemented first phase: **local check result reference model only**.

That phase:

1. Added `LocalCheckResultId`.
2. Added `LocalCheckResultReference`.
3. Added constructors, validation, serde, redaction-safe `Debug`, and read-only accessors.
4. Added a helper to derive a reference from a `LocalCheckResult` plus explicit run/event context.
5. Added focused tests for valid references, invalid IDs, secret-like values, serde failure, redaction-safe debug, and no raw output copying.
6. Updated docs and created an implementation report.

That phase did not:

- change `WorkReportCitationTarget`;
- attach citations automatically;
- create `EvidenceReference`;
- persist local check results;
- write report artifacts;
- expose CLI behavior;
- register handlers by default;
- enable `AllowlistedHandlerOnly`;
- add schema fields.

## 15. Later Implementation Sequence

Recommended future sequence:

1. Maintainer review of terminal report helper integration.
2. Command-output evidence policy planning, if needed.
3. Evidence attachment only after command-output policy is reviewed.
4. Persistence/artifact integration only after separate planning.
5. CLI/schema/default registration only after side-effect and authority posture are reviewed.

## 16. Test Plan For Future Implementation

Future implementation should test:

- valid local check result ID;
- invalid empty/oversized/secret-like result ID rejected;
- valid local check result reference;
- reference preserves command ID, command kind, status, workflow ID, and run ID;
- optional workflow event ID is represented when supplied;
- optional audit event ID is represented when supplied;
- stable output reference is bounded and redaction-safe if included;
- raw stdout is not copied;
- raw stderr is not copied;
- command transcript is not copied;
- environment values are not copied;
- debug output does not leak IDs, paths, summaries, tokens, or metadata;
- serialization does not leak forbidden raw payloads;
- invalid serialized reference fails closed;
- deserialization errors do not leak raw values;
- existing local check tests still pass;
- existing WorkReport, EvidenceReference, Diagnostic, adapter telemetry, and runtime tests still pass.

## 17. Documentation Requirements For Future Implementation

Future citation-integration docs must say:

- local check result references are implemented;
- local check result WorkReport citation is not implemented until separately scoped;
- local check result EvidenceReference attachment is not implemented;
- command-output evidence is not implemented;
- local check result persistence is not implemented;
- report artifact writing is not implemented;
- default registration is not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported.

## 18. Open Questions

- Should local check result references be distinct from `SkillOutput::output_ref` or should `output_ref` eventually become derived from the first-class result reference?
- Should local check result citation use a dedicated `WorkReportCitationTarget` variant?
- Should local check result references include bounded summaries, or only identity and status?
- Should check result references cite workflow events, audit events, or both?
- Should local check result references become persistable before report integration?
- Should failed and timed-out checks be cited the same way as passed checks?
- Should command-output evidence ever be allowed for local checks, or should structured check-result evidence be preferred?
- How should local check result references interact with future side-effect boundary modeling?

## 19. Final Recommendation

Proceed next with **terminal report local check citation integration review**.

Do not build automatic local check execution, local check reference creation, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default registration, `AllowlistedHandlerOnly`, side-effect modeling, writes, or release posture changes in that review.
