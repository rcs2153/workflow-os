# EvidenceReference Attachment Plan

Status: adapter telemetry and diagnostic evidence attachment implemented; later targets remain planning only.

Fix note: the adapter telemetry attachment review found that attached evidence vectors were publicly mutable. That validation-bypass blocker is fixed and documented in [EvidenceReference Adapter Telemetry Attachment Fix](../concepts/EVIDENCE_REFERENCE_ADAPTER_TELEMETRY_ATTACHMENT_FIX.md). Adapter invocation and runtime audit telemetry now expose read-only evidence accessors, and deserialization validates attached evidence.

This plan scopes EvidenceReference Phase 2 attachment after Phase 1 core model implementation and review. Adapter telemetry evidence attachment is implemented for adapter invocation and runtime audit telemetry records. Diagnostic evidence attachment is implemented for the core `Diagnostic` model only and is documented separately in [EvidenceReference Validation Attachment Plan](evidence-reference-validation-attachment-plan.md). Aggregate validation result evidence, automatic loader/validator evidence generation, approval attachment, persistence, CLI inspection, examples, work reports, reasoning lineage, side-effect boundaries, writes, domain packs, DLP, access control, production evidence storage, and release posture changes remain unimplemented.

## 1. Executive Summary

EvidenceReference Phase 2 should attach evidence references to selected existing Workflow OS artifacts, but only after validation boundaries are explicit.

The initial attachment order should be:

1. Adapter telemetry references. Implemented for adapter invocation and runtime audit telemetry records.
2. Validation diagnostic/result references. Diagnostic evidence attachment is implemented for `Diagnostic` only; aggregate validation result evidence and automatic validator generation remain deferred.
3. Approval decision references. Not implemented.
4. Audit and policy citations later, only if the first three boundaries remain clean.

The first implementation target was adapter telemetry evidence only. Adapter telemetry was the correct initial target because it already has read-only, redacted, runtime-visible records and a narrow Phase 2 integration posture.

## 2. Attachment Principles

- EvidenceReference remains reference-first.
- Attachments must not copy raw payloads.
- Attachments must validate references before storage or projection.
- Attachments must preserve redaction metadata.
- Attachments must not imply provider access control.
- Attachments must not turn EvidenceReference into an evidence store.
- Attachments must not create a reasoning graph.
- Attachments must not change public release posture.
- Attachments must not enable writes.
- Attachments must remain optional until a later ADR or implementation plan makes them required for a specific artifact.
- Attachments must keep fixture-first normal CI and opt-in live-provider behavior unchanged.

## 3. Validation Boundary

`EvidenceReference::validate()` should be enforced at the attachment boundary.

Recommended behavior:

- Attachment APIs should validate internally even if callers claim references are valid.
- Attachment APIs may accept caller-created references, but they must clone or otherwise take ownership only after validation succeeds.
- Invalid evidence must fail closed.
- Invalid evidence must not be partially attached.
- Invalid evidence must not be silently dropped.
- Invalid evidence must not be persisted.
- Invalid evidence should return a validation error with stable, non-secret error codes.
- Future persistence layers must reject invalid attached evidence before writing parent artifacts.

Parent artifact behavior:

| Parent artifact | Invalid evidence behavior |
| --- | --- |
| Adapter telemetry | Fail construction/mapping of the evidence-bearing telemetry record. Do not emit a partially attached evidence list. |
| Validation result/diagnostic | Fail evidence attachment. Preserve the original validation result only if it exists independently without evidence. |
| Approval request/decision | Fail the evidence-bearing approval request/decision mutation. Do not record evidence if it fails validation. |
| Future persistence | Reject write and report corruption or validation failure clearly. |

Implementation rule: no code path should rely on “already validated elsewhere” as the only defense. The receiving attachment API owns the final validation gate.

## 4. Public Field Mutability Review

EvidenceReference public fields are acceptable for Phase 2 planning and a narrow attachment implementation, but they raise risk once references become persisted or embedded in runtime artifacts.

Near-term posture:

- Fields may remain public for now.
- Attachment APIs must clone and validate defensively.
- Attachment APIs must treat post-validation caller mutation as irrelevant by storing only the validated clone.
- Tests must prove a caller cannot validate a reference, mutate it into an invalid form, and bypass attachment validation.

Future cleanup before persistence:

- Revisit private fields, builder APIs, or immutable validated wrappers.
- Consider a `ValidatedEvidenceReference` or attachment-specific newtype only if it reduces real validation risk.
- Do this before local persistence, schema exposure, or CLI rendering, not during the first adapter telemetry attachment.

Risks if left unchecked:

- Callers can mutate fields after validation.
- Optional scope fields can be cleared after construction.
- Metadata can be replaced with bounded-but-semantically-risky values.
- Future persisted artifacts could accidentally store invalid references if attachment validation is skipped.

The mitigation is simple: validate internally at every attachment boundary.

## 5. Command-Output And Metadata Safety

Command-output evidence remains high risk because a string that looks like a summary might still be raw output.

Rules:

- Store summaries only.
- Keep existing bounded sizes.
- Do not store raw logs by default.
- Sanitize secret-like values.
- Require redaction metadata that is not safe-only.
- Keep metadata bounded and redaction-aware.
- Error messages must not leak command output, metadata values, provider references, or secret-like content.
- Attachment APIs must not promote full command output into `EvidenceReferenceTarget::CommandOutput`.
- CI log evidence should prefer log references, excerpt hashes, failure summaries, and response-size metadata.

Required tests:

- command-output evidence attachment succeeds with a bounded redacted summary;
- command-output evidence with safe-only redaction fails closed;
- raw CI-log-like strings are sanitized or rejected according to the Phase 1 model;
- metadata values with token/private-key/authorization-header-like strings do not appear in Debug, Display, serialization, or error messages;
- invalid command-output evidence does not partially attach to parent artifacts.

## 6. Attachment Target 1: Adapter Telemetry

Implementation status: implemented for adapter invocation and runtime audit telemetry records.

Potential records:

- `AdapterInvocationRecord`
- `AdapterObservabilityRecord`
- `AdapterRuntimeAuditRecord`
- `AdapterRuntimeObservabilityRecord`

Recommended attachment shape:

- `AdapterInvocationRecord` carries evidence references for invocation and response summary evidence.
- `AdapterRuntimeAuditRecord` carries the same validated evidence references when runtime-visible telemetry is mapped.
- `AdapterObservabilityRecord` and `AdapterRuntimeObservabilityRecord` should carry evidence only if it remains concise and useful; observability should not become a payload mirror.

Evidence generation:

- Adapter invocation metadata can produce `EvidenceKind::AdapterInvocation`.
- Adapter response summaries can produce `EvidenceKind::AdapterResponseSummary`.
- Adapter failure summaries can cite adapter invocation evidence with error classification.
- Live smoke evidence remains a release/review artifact, not normal adapter execution evidence.

Identity to include where available:

- `workflow_id`
- `workflow_version`
- `schema_version`
- `spec_hash`
- `run_id`
- `step_id`
- `skill_id`
- `skill_version`
- `adapter_id`
- `adapter_kind`
- `correlation_id`
- `actor` or system actor
- policy precheck provenance through the parent adapter telemetry record

Provider payload safety:

- Do not store raw provider payloads.
- Do not store tokens or authorization headers.
- Do not store raw Jira descriptions/comments.
- Do not store raw CI logs.
- Do not store raw large GitHub file contents.
- Prefer external references, adapter response summaries, response-size metadata, redaction metadata, and stable provider object references.

Runtime-visible telemetry:

- Evidence can be stored on runtime-visible adapter telemetry records after validation.
- Do not change CLI inspect output in this phase.
- Operators can inspect persisted adapter telemetry through existing local artifacts only if those artifacts already expose the record; no new display contract is created.

Acceptance criteria:

- Adapter telemetry can attach validated evidence references.
- Invalid evidence fails closed at telemetry attachment boundary.
- Evidence references preserve adapter kind, action context, capability context through parent telemetry or evidence metadata.
- Evidence references include run/step/adapter identity where available.
- Evidence references do not include raw provider payloads or secrets.
- Existing read-only adapter behavior remains read-only.
- Existing fixture-backed examples and integration gates remain fixture-first.

Required tests:

- adapter telemetry evidence attachment success;
- adapter telemetry invalid evidence fails closed;
- adapter evidence does not contain tokens, raw provider payloads, raw CI logs, raw Jira bodies, or raw large GitHub file contents;
- adapter evidence preserves run/step/adapter identity where available;
- fixture/test policy provenance remains distinguishable from runtime policy approval through parent telemetry;
- public-field mutation cannot bypass attachment validation.

## 7. Attachment Target 2: Validation Diagnostics/Results

Second implementation target after adapter telemetry.

Potential records:

- `ValidationResult`
- `Diagnostic`
- project loader results
- schema/version validation diagnostics
- release/integration check summaries if represented in core later

Recommended attachment shape:

- Validation evidence should use `EvidenceKind::ValidationResult`.
- Validation-scoped evidence should include `validation_result_id` or a diagnostic reference.
- The target should reference a validation result, diagnostic code, source location, or local validation artifact, not raw command output.
- File/source locations may be represented as local/spec file evidence references or source-location metadata if bounded and non-secret.

What must not be copied:

- raw file contents;
- full command transcripts;
- secrets from specs;
- full error payloads that might include user data;
- unbounded YAML parser output.

Future work reports:

- Work reports should cite validation evidence references for “validation passed,” “validation failed,” and “known limitation accepted” statements.
- Validation evidence should not itself become the report.

Acceptance criteria:

- Validation results/diagnostics can cite validated evidence references.
- Invalid validation evidence fails closed at attachment boundary.
- Source-location behavior is bounded and non-secret.
- Existing validation behavior remains unchanged when no evidence references are supplied.

Required tests:

- validation evidence attachment success;
- validation evidence source-location behavior;
- validation evidence invalid references fail closed;
- validation evidence does not copy raw spec contents or raw command output;
- public-field mutation cannot bypass attachment validation.

## 8. Attachment Target 3: Approval Decisions

Third implementation target after adapter telemetry and validation diagnostics.

Potential records:

- `ApprovalRequest`
- approval grants
- approval denials

Recommended v1 scope:

- Approval requests may cite evidence that should be reviewed before approval.
- Approval decisions may cite evidence considered by the approver.
- Approval grants and denials should both support evidence references eventually.
- V1 should allow evidence references; it should not require evidence references for every approval.

Request versus decision:

| Attachment point | Purpose |
| --- | --- |
| Approval request | Evidence packet prepared for the approver. |
| Approval grant | Evidence the approver relied on when granting. |
| Approval denial | Evidence or missing evidence that caused denial. |

Missing evidence behavior:

- Missing optional evidence should not block normal approval behavior in v1.
- If a workflow later declares evidence as required for sensitive approval, that must be a separate validation/policy design.
- Invalid supplied evidence must fail closed and prevent recording the evidence-bearing request or decision.

Future sensitive actions:

- Approval evidence may become required for sensitive actions later, especially before writes.
- That requirement belongs with side-effect boundary and policy-gated write design, not this attachment plan.

Acceptance criteria:

- Approval request/decision records can carry validated evidence references.
- Invalid approval evidence fails closed.
- Approval behavior without evidence remains unchanged.
- No write-capable behavior is introduced.
- Approval reasons remain non-secret and evidence references do not copy sensitive payloads.

Required tests:

- approval evidence attachment success;
- approval evidence invalid references fail closed;
- grant and denial evidence behave consistently;
- missing optional evidence does not break existing approval flows;
- evidence references do not leak secret-like metadata or approval packet contents.

## 9. Deferred Targets

| Deferred target | Reason |
| --- | --- |
| Local persistence | Requires storage layout, corruption behavior, source-of-truth rules, and retention review. |
| CLI inspect evidence rendering | Requires display contract, redaction review, and experimental JSON posture decision. |
| Example integration | Should wait until attachment behavior exists and is operator-visible without overclaiming production behavior. |
| Work reports | Need EvidenceReference attachment foundations first; WorkReportContract requires separate ADR/plan. |
| Reasoning lineage | Future provenance graph; EvidenceReference must remain citation substrate, not graph nodes/edges. |
| Side-effect boundary | Needed before writes, but separate from evidence attachment. |
| Writes | Still denied and out of scope. Evidence attachment must not authorize external side effects. |
| Domain packs | Require stable core governance primitives and should not drive core evidence design. |
| Production evidence store | Requires persistence, access control, retention, and operational design. |
| DLP/access control | EvidenceReference is deterministic preview redaction, not enterprise DLP or authorization enforcement. |

## 10. Compatibility And Persistence Posture

`EvidenceReference` is exported from `workflow-core` but is not persisted yet.

Attaching evidence to persisted artifacts increases compatibility pressure because evidence references may become part of runtime records, audit projections, or local state artifacts.

Before persistence:

- define whether attached evidence is stored inline or by ID;
- define whether validation occurs at parent artifact construction, state write, or both;
- define corruption behavior for invalid attached evidence;
- define migration behavior for records without evidence references.

Before schema exposure:

- decide whether public JSON/YAML schemas need evidence fields;
- define compatibility expectations for new optional fields;
- avoid exposing evidence references through spec files until the schema contract is ready.

Before CLI exposure:

- decide whether human-readable inspect should display evidence summaries;
- preserve experimental CLI JSON posture unless separately versioned;
- test that CLI output does not include secrets, raw provider payloads, raw logs, or private issue/PR content.

## 11. Test Plan

Future implementation must include tests for:

- adapter telemetry evidence attachment success;
- adapter telemetry invalid evidence fails closed;
- adapter evidence does not contain tokens or raw payloads;
- adapter evidence preserves run/step/adapter identity;
- validation evidence attachment success;
- validation evidence source-location behavior;
- validation evidence invalid references fail closed;
- approval evidence attachment success;
- approval evidence invalid references fail closed;
- no evidence attachment leaks secret-like metadata;
- command-output summary safety;
- public-field mutation cannot bypass attachment validation;
- attachment APIs validate internally even when callers pass references that were previously validated;
- no writes, reruns, dispatches, webhooks, OAuth behavior, or generic live adapter execution are introduced.

## 12. Proposed Implementation Sequence

Small PR/prompt sequence:

1. Adapter telemetry attachment only. Implemented.
2. Validation diagnostics/results attachment only.
3. Approval request/decision attachment only.
4. Maintainer review of attachment behavior and validation boundaries.
5. Decide persistence, CLI inspection, and example updates only after the review.

The first implementation did not touch validation diagnostics or approvals. It attached validated EvidenceReferences to adapter telemetry records and proved redaction/no-payload behavior.

## 13. Non-Goals

This plan does not authorize:

- persistence;
- CLI inspection;
- example updates;
- WorkReportContract;
- WorkReport artifacts;
- Reasoning Lineage / Claim Graph;
- side-effect boundary modeling;
- writes;
- domain packs;
- production evidence store;
- DLP;
- access-control systems;
- release posture changes;
- stable CLI JSON output;
- schema changes;
- generic live adapter execution;
- hosted operation or distributed workers.

## 14. Final Recommendation

The next implementation step should be **validation diagnostics/results attachment planning or implementation**, after maintainers confirm the adapter telemetry attachment remains clean.

The completed adapter telemetry implementation:

- adds validated evidence attachment to adapter invocation and runtime audit telemetry records;
- validates internally at attachment boundaries;
- fails closed for invalid evidence;
- preserves redaction metadata and sensitivity;
- avoids raw provider payloads;
- keeps CLI, persistence, examples, validation attachments, approval attachments, work reports, reasoning lineage, writes, and domain packs out of scope.
