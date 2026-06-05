# EvidenceReference Validation Attachment Plan Review

Review date: 2026-06-04

Reviewed materials:

- `docs/ENGINEERING_STANDARD.md`
- `docs/adr/0009-evidence-reference-core-model.md`
- `docs/concepts/evidence-reference.md`
- `docs/implementation-plans/evidence-reference-mvp.md`
- `docs/implementation-plans/evidence-reference-attachment-plan.md`
- `docs/implementation-plans/evidence-reference-validation-attachment-plan.md`
- `docs/concepts/EVIDENCE_REFERENCE_PHASE_1_REVIEW.md`
- `docs/concepts/EVIDENCE_REFERENCE_ADAPTER_TELEMETRY_ATTACHMENT_REVIEW.md`
- `docs/concepts/EVIDENCE_REFERENCE_ADAPTER_TELEMETRY_ATTACHMENT_FIX_REVIEW.md`
- security, spec, runtime, core, and test surfaces related to diagnostics, validation, redaction, loader behavior, and evidence references.

This review evaluates the validation diagnostics/results attachment plan only. It does not implement validation attachment, persistence, CLI inspection, example updates, approval attachment, work reports, reasoning lineage, side-effect modeling, writes, schemas, production evidence storage, DLP/access control, or release posture changes.

## 1. Executive Verdict

**Ready for implementation with revisions.**

The plan is directionally sound and fits the current Workflow OS architecture. The first implementation should be narrower than the full plan language: attach evidence to `Diagnostic` only, keep `ValidationResult` unchanged as a lightweight diagnostics wrapper, and defer command-output evidence at the core diagnostic layer.

These revisions are small enough to encode directly in the implementation prompt. They do not require another planning cycle before implementation prompt generation.

## 2. Scope Verification

The plan remains limited to validation diagnostics/results attachment planning.

It does not authorize:

- local persistence;
- CLI rendering or inspect/status changes;
- example updates;
- approval attachment;
- work reports;
- reasoning lineage;
- side-effect modeling;
- writes;
- schema changes;
- production evidence storage;
- DLP or access-control systems;
- release posture changes.

No accidental authorization of broader runtime behavior was found. The plan repeatedly states that attachment must not change validation execution semantics, call adapters, read secrets, fetch providers, or create report behavior.

## 3. Architecture Fit

Validation evidence attachment fits Workflow OS if it remains a citation mechanism on deterministic validation artifacts.

The current architecture has:

- a deterministic project loader that accumulates `Diagnostic` values;
- a deterministic validator that returns `ValidationResult { diagnostics: Vec<Diagnostic> }`;
- `Diagnostic` values with severity, stable code, message, and optional `SourceLocation`;
- explicit documentation that validation does not execute workflows, call adapters, evaluate live policy, or read secrets;
- EvidenceReference as a reference-first, redacted core model.

Attaching evidence references to diagnostics is consistent with the local-first kernel because it preserves validation as a local, deterministic process while allowing future reports or operator reviews to cite validation artifacts. The plan correctly avoids copying raw spec contents, raw command output, parser payloads, provider data, or secret-like values into evidence.

## 4. Target Model Review

| Candidate target | Review decision | Rationale |
| --- | --- | --- |
| `Diagnostic` | First implementation target. | It is the smallest central artifact used by loader, parser, semantic validation, and error surfaces. It already carries diagnostic code, severity, message, and source location. |
| `ValidationResult` | Later target. | It is currently only a wrapper around diagnostics. Adding aggregate evidence now would imply report or validation-summary semantics too early. |
| `ProjectLoadResult` | Defer. | Loader results already carry diagnostics. Evidence should flow through diagnostics first. |
| Semantic validation output | Later target through diagnostics. | Semantic validation emits diagnostics, so call sites should not generate evidence until the diagnostic model change is reviewed. |
| Schema/version validation diagnostics | Later target through diagnostics. | These are normal diagnostics and should benefit from diagnostic evidence once the core diagnostic attachment path exists. |
| Docs/integration check outputs | Reject for core v1. | These are script/release artifacts, not current core validation model artifacts. |

Evidence should attach directly to each `Diagnostic` in the first implementation. A `ValidationReport` or `ValidationSummary` wrapper should not be created first.

Validation success should not receive evidence in the first implementation. V1 should support evidence on diagnostics, especially errors and warnings. Aggregate success evidence belongs later in a validation-report, work-report, release-gate, or integration-gate design.

## 5. Evidence Kind And Scope Review

The plan mostly uses the right evidence kinds and scopes, with one important narrowing for implementation.

Recommended first implementation:

- Use `EvidenceKind::ValidationResult` for evidence that cites a validation diagnostic or diagnostic result reference.
- Allow `EvidenceKind::SpecFile` only when the evidence target is a source/spec file reference associated with the diagnostic source location.
- Defer `EvidenceKind::CommandOutput` for core `Diagnostic` attachment. Command output belongs in release/review tooling or a later bounded command-summary path, not in the first validation diagnostic attachment.

Recommended scopes:

- `EvidenceScope::Validation` should be the primary scope for diagnostic evidence and should include `validation_result_id`.
- `EvidenceScope::Project` may be acceptable for project-level spec/source evidence when no workflow context exists.
- `EvidenceScope::Workflow` should be allowed only when workflow context is already available without parsing or copying raw spec payloads into evidence.
- `EvidenceScope::Run` should be rejected for the first diagnostic implementation unless real immutable run identity is already available. Current validation is not run-scoped.

Unsafe or confusing combinations to avoid in v1:

- `EvidenceKind::CommandOutput` attached directly to validation diagnostics.
- Run-scoped validation evidence without real `workflow_id`, `workflow_version`, `schema_version`, `spec_hash`, and `run_id`.
- Provider/adaptor-scoped evidence on diagnostics.
- Release-review or live-smoke evidence on core validation diagnostics.

## 6. Validation Boundary Review

The plan correctly requires:

- internal validation at the attachment boundary;
- invalid evidence to fail closed;
- no partial attachment;
- private evidence collections or a validated collection wrapper;
- read-only accessors;
- validated deserialization if diagnostics serialize attached evidence;
- no reliance on references being "already validated elsewhere."

Invalid evidence should fail evidence-bearing diagnostic construction or evidence attachment. It should not create another validation diagnostic in v1 because that would mix model-integrity failures with project validation results.

The original diagnostic/result may still exist without evidence if evidence attachment fails before constructing the evidence-bearing variant. Once a diagnostic is evidence-bearing, invalid attached evidence must prevent construction or deserialization.

## 7. Privacy And Redaction Review

The plan sufficiently protects against the major privacy risks:

- no raw spec file contents;
- no raw command transcripts;
- no secret-like YAML parser output;
- no environment variable values;
- no provider payloads;
- conservative treatment of file paths;
- bounded redacted summaries and metadata.

Required implementation refinements:

- Do not copy `Diagnostic.message()` into `EvidenceReference.summary` by default. Diagnostic messages may include user-controlled context in future paths.
- Treat file paths as potentially sensitive. Source locations may remain on `Diagnostic`, but evidence summaries/debug output must not expand them into raw content.
- Keep parser errors and serde errors from becoming evidence payloads.
- Ensure evidence attachment/deserialization errors contain stable codes and do not include target, summary, metadata, file content, or source excerpts.

These refinements are compatible with the existing redaction rules and should be included in the implementation prompt.

## 8. Source-Location Review

The current `SourceLocation` already represents:

- file path;
- line;
- column;
- document path.

For the first implementation, source location should remain on `Diagnostic` as the source of truth. Evidence references may cite related source/spec files, but they should not become a second source-location model.

Recommended representation:

| Source detail | First implementation posture |
| --- | --- |
| File path | Keep on `SourceLocation`; optional `SpecFile` evidence target may reference it when useful. |
| Line/column | Keep on `SourceLocation`. Do not require duplication in evidence metadata. |
| JSON/YAML path | Keep on `SourceLocation.document_path`. |
| Diagnostic code | Keep on `Diagnostic.code`; optional bounded metadata only if needed. |
| Schema version | Existing validation context or bounded metadata later. |
| Spec hash | Use existing spec hash only when already available; do not read or copy file content to create evidence. |

The implementation should not invent a new source-location evidence model.

## 9. API Shape Review

The proposed API shape is appropriate:

- `attach_evidence_reference(...) -> Result<_, WorkflowOsError>`
- `attach_evidence_references(...) -> Result<_, WorkflowOsError>`
- `with_evidence_references(...) -> Result<Self, WorkflowOsError>`
- `evidence_references() -> &[EvidenceReference]`

Implementation requirements:

- The collection must be private or a validated collection wrapper.
- Multiple attachment must be atomic.
- APIs must validate internally and store only sanitized, validated clones.
- Read-only accessors must not expose mutable vectors.
- Public-field mutation must not bypass attachment validation.
- If `Diagnostic` deserializes with evidence, deserialization must validate evidence before constructing the diagnostic.

The adapter telemetry blocker fix provides the right precedent.

## 10. Relationship To Future Work Reports

The plan gives future `WorkReportContract` enough to cite validation evidence for:

- validation failed;
- validation warning accepted;
- known limitation accepted;
- release-readiness evidence;
- integration-gate evidence.

It is weaker for "validation passed" because there is no aggregate validation report yet. That is acceptable. Passed-validation evidence should wait for a future validation summary, work report, release gate, or integration gate artifact.

This plan does not implement reports and should not imply that work reports exist.

## 11. Test Plan Review

The future test plan is strong. It should include all listed tests plus these additions:

- validation behavior remains unchanged when no evidence is attached;
- `Diagnostic` source location remains preserved after evidence attachment;
- `Diagnostic` code, severity, and message remain preserved after evidence attachment;
- evidence attachment does not copy `Diagnostic.message()` into evidence summary by default;
- `ValidationResult` remains only a diagnostics wrapper in the first implementation;
- `EvidenceKind::CommandOutput` is rejected or deferred for diagnostic attachment in v1;
- Debug/Display/serde errors for evidence-bearing diagnostics do not leak secret-like titles, targets, metadata, source excerpts, or parser payloads.

Existing validation, loader, and project-spec tests must continue to pass.

## 12. Product-Boundary Risks

Key risks and mitigations:

| Risk | Mitigation |
| --- | --- |
| Validation evidence becomes a spec-content store. | References only; never copy raw spec contents. |
| Validation evidence duplicates diagnostics. | Keep `Diagnostic` as diagnostic source of truth; evidence only cites supporting references. |
| Evidence creates CLI/report expectations too early. | No CLI rendering or work reports in this phase. |
| Command outputs become log storage. | Defer command-output evidence for core diagnostic v1. |
| Source paths leak repository structure. | Treat paths as potentially sensitive and preserve current source-location behavior. |
| Attachment becomes schema exposure prematurely. | Do not add schema changes in this phase. |

The plan preserves Workflow OS as a governed local-first kernel, not a BPM engine, report engine, log store, or evidence warehouse.

## 13. Required Revisions

Required revisions for the implementation prompt:

1. Start with `Diagnostic` only.
2. Do not create a `ValidationReport`, `ValidationSummary`, or new aggregate result wrapper.
3. Keep `ValidationResult` behavior unchanged except that it may carry diagnostics that themselves have evidence.
4. Do not attach evidence automatically from loader or semantic validator call sites in the first implementation.
5. Do not attach validation success evidence in v1.
6. Defer `EvidenceKind::CommandOutput` for core diagnostic attachment unless a separate scoped prompt explicitly handles bounded command summaries.
7. Treat `SourceLocation` as the source of truth for file path, line, column, and document path.
8. Do not copy `Diagnostic.message()` into evidence summaries by default.
9. Require private evidence collections or a validated collection wrapper.
10. Require validated deserialization for evidence-bearing diagnostics.

No broader planning blocker remains once these constraints are included.

## 14. Final Recommendation

**Approve implementation prompt generation** with the revisions above encoded as hard scope constraints.

The next implementation prompt should implement `Diagnostic` evidence attachment only.

Still do not build:

- local persistence;
- CLI rendering or inspection;
- example updates;
- approval attachment;
- work reports;
- reasoning lineage;
- side-effect modeling;
- writes;
- schemas;
- production evidence storage;
- DLP/access control;
- release posture changes.

## Validation

Required validation for this review:

- `npm run check:docs`

