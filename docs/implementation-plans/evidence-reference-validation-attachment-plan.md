# EvidenceReference Validation Attachment Plan

Status: diagnostic evidence attachment implemented; broader validation/result attachment remains planning only.

## 1. Executive Summary

EvidenceReference can now attach safely to adapter telemetry records. Adapter invocation and runtime audit telemetry use private evidence collections, read-only accessors, validated attachment APIs, and validated deserialization.

Diagnostic evidence attachment is implemented for the core `Diagnostic` model only. It uses private evidence collections, read-only accessors, validated attachment APIs, validated deserialization, and diagnostic-specific evidence kind/scope restrictions.

Broader validation/result attachment remains planning only. Automatic loader/validator evidence generation, aggregate `ValidationResult` evidence, validation success evidence, persistence, CLI inspection, examples, approval attachment, work reports, reasoning lineage, side-effect boundaries, writes, schemas, and release posture changes are not implemented.

Validation call-site attachment planning is documented separately in [EvidenceReference Validation Call-Site Attachment Plan](evidence-reference-validation-callsite-plan.md). The first call-site target is implemented for schema-version diagnostics with safe source/spec context. Broader automatic call-site attachment remains unimplemented.

## 2. Goals

- Allow validation diagnostics and/or validation results to cite `EvidenceReference` values.
- Keep validation behavior unchanged when no evidence is attached.
- Avoid copying raw spec files, raw YAML/JSON contents, or raw command output.
- Preserve diagnostic codes, severity, and source locations.
- Preserve redaction metadata and sensitivity classification on attached evidence.
- Prepare future `WorkReportContract` and terminal work reports to cite validation evidence.
- Keep validation deterministic and local. Evidence attachment must not cause validation to execute workflows, call adapters, read secrets, or fetch providers.

## 3. Non-Goals

This plan does not authorize:

- local persistence;
- CLI rendering or inspect/status output;
- example updates;
- approval request/decision attachment;
- work reports;
- reasoning lineage;
- side-effect boundary modeling;
- writes;
- schema changes;
- production evidence storage;
- DLP or access-control systems;
- release posture changes.

## 4. Candidate Attachment Targets

Current codebase shape:

- `Diagnostic` is the central structured diagnostic type for loader, parser, validation, CLI-facing errors, and `WorkflowOsError` diagnostic details.
- `ValidationResult` is a small wrapper around `Vec<Diagnostic>`.
- `ProjectLoadResult` contains `Vec<Diagnostic>` and an optional loaded bundle.
- Semantic validation returns `ValidationResult`.
- Schema/version validation appears as diagnostics produced by loader and validator paths.
- Docs/integration check outputs are not represented as core validation structures.

| Target | v1 decision | Rationale |
| --- | --- | --- |
| `Diagnostic` | Implemented. | It is the smallest central validation artifact and already carries severity, code, message, and source location. |
| `ValidationResult` | Defer. | It is currently only a diagnostics wrapper. Attaching evidence here may duplicate per-diagnostic evidence or imply report-level semantics too early. |
| `ProjectLoadResult` | Defer. | Loader results already preserve diagnostics; evidence can flow through diagnostics first. |
| Semantic validation output | Defer direct changes. | Semantic validation emits diagnostics. Start with the diagnostic model before touching validator call sites. |
| Schema/version validation diagnostics | Attach indirectly through `Diagnostic` later. | These are diagnostics with source locations and codes, so they should benefit from diagnostic evidence once implemented. |
| Docs/integration check outputs | Reject for core v1. | They are script/release-check artifacts, not current core validation structures. They may use `EvidenceKind::CommandOutput` or `EvidenceKind::TestResult` later outside the core diagnostic attachment path. |

## 5. Evidence Kind And Scope

Recommended evidence kinds:

- `EvidenceKind::ValidationResult` for evidence that cites a validation diagnostic, validator output, or validation summary.
- `EvidenceKind::SpecFile` for references to source spec files where useful.
- `EvidenceKind::CommandOutput` only for bounded, redacted summaries of command output. It must not store raw command transcripts or logs.

Implemented diagnostic attachment accepts only `EvidenceKind::ValidationResult` and `EvidenceKind::SpecFile`. `EvidenceKind::CommandOutput` remains deferred for validation diagnostics until a separate scoped plan covers bounded command summaries.

Recommended scopes:

- `EvidenceScope::Validation` for evidence attached directly to a diagnostic or validation-related result. This should include a `validation_result_id` or diagnostic reference.
- `EvidenceScope::Project` for project-level validation context that is not specific to one workflow, run, or step.
- `EvidenceScope::Workflow` only when the diagnostic already has clear workflow context or the future implementation can derive it without parsing raw payloads into evidence.
- `EvidenceScope::Run` only if validation is explicitly run-scoped in a future runtime path and immutable run identity is already available.

Implemented diagnostic attachment accepts only validation, project, and workflow scopes. Run-scoped diagnostic evidence remains deferred.

Do not invent provider-specific evidence kinds or scopes for validation diagnostics.

## 6. Validation Boundary

Validation attachment must follow the adapter telemetry precedent:

- Attachment APIs must validate internally.
- Attachment APIs must sanitize before storing.
- Invalid evidence must fail closed.
- Invalid evidence must not be silently dropped.
- Invalid evidence must not be partially attached.
- Multiple evidence attachment must be atomic.
- Public direct mutation must be impossible.
- If serialized, deserialization must validate attached evidence before constructing the parent object.

Parent artifact behavior:

- If evidence attachment fails before constructing an evidence-bearing diagnostic/result, the original validation diagnostic/result may still exist independently without evidence.
- Once code constructs an evidence-bearing diagnostic/result, invalid evidence must prevent that evidence-bearing variant from being created.
- No unsafe evidence reference should reach future persistence, CLI rendering, work reports, or report citations.

## 7. Privacy And Redaction Rules

Validation evidence must not copy:

- raw spec file contents;
- raw YAML/JSON parser output that may include user data;
- raw command transcripts;
- environment variable values;
- raw provider payloads;
- secret-like spec values;
- unredacted personal data;
- full private file contents.

Allowed with care:

- stable diagnostic codes;
- diagnostic severity;
- source locations when non-secret;
- document paths such as JSON Pointer/YAML path;
- schema version;
- spec content hash;
- redacted, bounded summaries;
- references to local files or validation artifacts.

File paths may be sensitive. Evidence that includes local paths should use sensitivity conservatively and follow redaction policy. Diagnostic messages must not include raw secret payloads. Summaries must remain bounded and redacted.

## 8. Source-Location Handling

Current `SourceLocation` can represent:

- file path;
- one-based line;
- one-based column;
- document path.

Validation evidence should preserve those fields through the diagnostic itself or through bounded metadata on an evidence reference. It should not require all fields when a diagnostic does not expose them.

Recommended reference patterns:

| Source detail | Recommended representation |
| --- | --- |
| File path | `EvidenceKind::SpecFile` with a file target, or diagnostic `SourceLocation` plus sensitivity metadata. |
| Line/column | Source-location metadata or parent diagnostic fields; do not embed source excerpts. |
| JSON/YAML path | Source-location document path or bounded metadata. |
| Diagnostic code | Parent `Diagnostic.code`; optionally repeated in bounded metadata if needed for future report citations. |
| Schema version | Parent validation context or bounded metadata when available. |
| Spec hash | `SpecContentHash` where available; do not copy file content. |

The first implementation should not invent a large source-location evidence model. It should reuse `SourceLocation` and typed evidence references conservatively.

## 9. Attachment API Shape

Implemented APIs for the first implementation target:

- `Diagnostic::attach_evidence_reference(...) -> Result<(), WorkflowOsError>`
- `Diagnostic::attach_evidence_references(...) -> Result<(), WorkflowOsError>`
- `Diagnostic::with_evidence_references(...) -> Result<Self, WorkflowOsError>`
- `Diagnostic::evidence_references() -> &[EvidenceReference]`

Rules:

- The evidence collection must be private or wrapped in a validated collection type.
- Public direct mutation must be impossible.
- Attachment must validate internally even if callers claim evidence is pre-validated.
- Deserialization must validate attached evidence if `Diagnostic` becomes serialized with evidence references.
- Invalid evidence must prevent creation of the evidence-bearing diagnostic/result.

Potential future APIs for `ValidationResult` should wait until maintainers decide whether evidence belongs on each diagnostic, the aggregate result, or both.

## 10. Relationship To Future Work Reports

Future work reports could cite validation evidence for:

- validation passed;
- validation failed;
- warning accepted;
- known limitation accepted;
- release readiness evidence;
- integration gate evidence;
- operator handoff notes about validation risk.

Work reports should cite validation evidence references rather than copying raw diagnostics, raw spec contents, or raw command output. This plan does not implement work reports or report contracts.

## 11. Test Plan

Future implementation must include tests for:

- attach one valid evidence reference to a diagnostic/result;
- attach multiple valid references atomically;
- invalid evidence fails closed;
- unsupported evidence kind/scope fails where applicable;
- direct mutation bypass is impossible through public API;
- valid serialized evidence-bearing diagnostic/result deserializes;
- invalid serialized evidence-bearing diagnostic/result fails;
- no raw spec contents are copied;
- no raw command output is copied;
- source location is preserved;
- diagnostic code is preserved;
- secret-like diagnostic values are redacted or rejected;
- deserialization errors do not leak secret-like values;
- existing loader and validation tests still pass;
- validation behavior is unchanged when no evidence is attached.

If compile-time privacy cannot be asserted directly, tests should document that Rust privacy enforces it and prove only read-only accessors plus validated APIs are available.

## 12. Proposed Implementation Sequence

1. Inspect the current diagnostic/result model. This plan confirms `Diagnostic` is the central first target and `ValidationResult` is a lightweight wrapper.
2. Add a private evidence collection and validated APIs to `Diagnostic`.
3. Add tests for attachment, atomicity, serde validation, redaction, source-location preservation, and existing validation behavior.
4. Update docs to say diagnostic evidence attachment is implemented and aggregate result attachment remains deferred.
5. Run a focused maintainer review.
6. Only after that review, decide whether loader and semantic validator call sites should start generating evidence references automatically.

Do not attach evidence in actual validation code paths until the diagnostic model change is reviewed.

A follow-on call-site plan is available in [EvidenceReference Validation Call-Site Attachment Plan](evidence-reference-validation-callsite-plan.md). Its first implementation target, source-location-backed spec-file evidence for schema-version diagnostics, is implemented. Additional call-site families remain deferred pending review.

## 13. Open Questions

- Is `Diagnostic` sufficient as the central validation evidence attachment point, or does Workflow OS need a future `ValidationReport` wrapper?
- Should evidence attach to each diagnostic, to a `ValidationResult`, or to both?
- Should successful validation have evidence, or only failures and warnings?
- How should source file evidence relate to `SpecContentHash` when a full bundle hash is available but individual file hashes are not?
- Should command outputs be represented by `EvidenceReference` at this layer, or only in release/review tooling?
- How should validation evidence references interact with future report contracts?
- Should a diagnostic-level evidence reference require `EvidenceScope::Validation`, or should `SpecFile` and `Project` scoped evidence be accepted when attached to diagnostics?

## 14. Final Recommendation

Diagnostic evidence attachment has been implemented. The next step should be a focused maintainer review before any broader validation/result attachment work.

Do not start with a `ValidationReport` wrapper. The current model does not have one, and adding it would risk introducing report semantics before `WorkReportContract` is scoped. Keep `ValidationResult` unchanged except for carrying diagnostics that may themselves contain evidence, then review before touching loader or semantic validator call sites.
