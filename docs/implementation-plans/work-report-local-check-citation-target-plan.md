# WorkReport Local Check Result Citation Target Plan

Status: WorkReport citation vocabulary for local check results is implemented and reviewed. Terminal report helper integration for supplied local check result references is implemented and documented in [Terminal Report Local Check Citation Integration Plan](terminal-report-local-check-citation-integration-plan.md). Automatic local check citation wiring, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default handler registration, side-effect modeling, source writes, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has a validated `LocalCheckResultReference` model. That model identifies local check outcomes without copying raw stdout, stderr, command transcripts, environment values, docs contents, parser payloads, provider payloads, or secrets.

The next question is how `WorkReport` should cite those references.

This plan recommends adding explicit WorkReport citation vocabulary for local check results as a narrow model-only phase. The first implementation added a `WorkReportCitationKind::LocalCheckResult` and a `WorkReportCitationTarget::LocalCheckResult` target that cites an existing `LocalCheckResultReference` by stable reference. It does not integrate terminal report generation helpers, create local check references automatically, attach evidence, persist results, write artifacts, expose CLI behavior, add schema fields, register handlers by default, or change executor behavior.

## 2. Goals

- Let future WorkReports cite local check outcomes in the `validation and quality checks` section.
- Keep local check result citations distinct from validation diagnostics.
- Cite stable references, not raw command output.
- Preserve existing `WorkReportCitation` validation, redaction, serde, and debug boundaries.
- Preserve existing terminal report generation behavior.
- Avoid creating `EvidenceReference` values implicitly.
- Keep command-output evidence policy deferred.
- Keep local check handler authority explicit and non-default.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- terminal report helper integration;
- automatic report citation wiring;
- local check result reference generation from handlers or executor paths;
- EvidenceReference attachment;
- `EvidenceKind::CommandOutput` usage;
- local check result persistence;
- report artifact writing;
- default `DocsCheck` registration;
- CLI exposure;
- workflow schema fields;
- automatic local check execution;
- `AllowlistedHandlerOnly` enablement;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- hosted or distributed runtime claims;
- recursive agents;
- agent swarms;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Governance Check

This planning phase was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-local-check-work-report-citation-plan`
- Run ID: `run-1781539740495848000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781539740495848000-2/d`
- Final status: `Completed`

The governed run completed before documentation edits were made.

## 5. Current Baseline

Implemented:

- `LocalCheckResult`;
- `LocalCheckResultId`;
- `LocalCheckResultReference`;
- `LocalCheckResultReferenceDefinition`;
- validated local check result reference construction;
- redaction-safe `Debug` for local check references;
- serde validation for local check references;
- `WorkReportCitation`;
- `WorkReportCitationKind`;
- `WorkReportCitationTarget`;
- terminal local WorkReport generation helper;
- runtime result exposure helper;
- executor-integrated report-bearing execution;
- explicit report artifact store.

Implemented after this plan:

- `WorkReportCitationKind::LocalCheckResult`;
- `WorkReportCitationTarget::LocalCheckResult`;
- citation kind mapping for local check result citations;
- serde support through existing `WorkReportCitation` validation;
- redaction-safe debug behavior through existing citation target debug;
- focused tests for valid citation construction, serde round trip, secret-like reference rejection, and no raw output copying.

Not implemented:

- terminal report helper consumption of local check result references;
- automatic local check reference creation;
- local check result persistence;
- local check result evidence attachment;
- command-output evidence.

## 6. Citation Target Decision

The next implementation should add a dedicated local check citation target.

Recommended target:

```rust
WorkReportCitationTarget::LocalCheckResult {
    reference: WorkReportStableReference,
}
```

Recommended kind:

```rust
WorkReportCitationKind::LocalCheckResult
```

Rationale:

- It keeps process/check outcomes separate from schema and semantic validation diagnostics.
- It avoids overloading `ValidationReferenceId`.
- It can cite `LocalCheckResultReference::result_id()` or a derived stable reference without importing the entire local check model into `WorkReport` targets.
- It preserves current `WorkReportCitationTarget` patterns for adapter telemetry and future reasoning lineage references.

## 7. Why Not Use ValidationDiagnostic

`ValidationDiagnostic` should remain for loader, schema, semantic, or diagnostic validation references.

Local check results are process/check outcomes. They may represent docs checks, dogfood validation commands, future cargo checks, or other allowlisted command results. Treating them as `ValidationDiagnostic` would blur the distinction between deterministic project validation diagnostics and local process check outcomes.

## 8. Why Not Use EvidenceReference Yet

Local check result citation should not create `EvidenceReference` values in this phase.

Reasons:

- command-output evidence policy is not reviewed;
- local check results can be cited by stable references without becoming evidence;
- `EvidenceKind::CommandOutput` can become raw log storage if introduced prematurely;
- WorkReport can cite local check references as model facts first.

Future evidence work may decide whether structured local check results should use `EvidenceKind::ValidationResult`, a new evidence kind, or carefully bounded `CommandOutput` evidence. That is separate planning.

## 9. Model Shape

Implemented code changes:

- added `LocalCheckResult` to `WorkReportCitationKind`;
- added `LocalCheckResult { reference: WorkReportStableReference }` to `WorkReportCitationTarget`;
- updated `WorkReportCitationTarget::citation_kind()`;
- preserved redaction-safe `Debug`;
- preserved serde tag names with `snake_case`;
- added tests for valid citation construction, serde round trip, invalid secret-like references, and no raw output copying.

Do not add a direct `LocalCheckResultReference` field to `WorkReportCitationTarget` yet. A stable reference is enough and avoids deep coupling between report citation vocabulary and the full local check reference model.

## 10. Citation Construction Rules

Future call sites should construct local check result citations by:

1. creating or receiving a validated `LocalCheckResultReference`;
2. converting its stable result ID into a `WorkReportStableReference`;
3. constructing a `WorkReportCitation` through `WorkReportCitation::new(...)`;
4. supplying bounded, non-secret summary text only when useful.

Rules:

- Do not create local check result references implicitly inside `WorkReportCitation`.
- Do not fabricate IDs.
- Do not copy stdout/stderr summaries by default.
- Do not copy command transcripts.
- Do not copy environment values.
- Do not copy provider payloads or parser payloads.
- Missing references should remain explicit not-available section text until missing-citation policy is separately designed.

## 11. Section Placement Policy

Local check result citations should eventually belong in the `validation and quality checks` section.

Potential later behavior:

- docs check result reference appears in `ValidationAndQualityChecks`;
- dogfood validation check result reference appears in `ValidationAndQualityChecks`;
- failed/timed-out/skipped local check references appear in the same section with bounded status text;
- unavailable local check references remain explicit not-available section text.

This plan does not implement section population.

## 12. Privacy And Redaction

The citation target must inherit WorkReport citation privacy rules.

Rules:

- Use `WorkReportCitation::new(...)`.
- Use `WorkReportStableReference::new(...)`.
- Reject secret-like references.
- Keep citation summaries bounded and redacted.
- Keep redaction metadata validated.
- Keep `Debug` redaction-safe.
- Keep serde fail-closed.
- Do not store raw stdout, stderr, command transcripts, CI logs, docs contents, parser payloads, provider payloads, environment values, tokens, credentials, authorization headers, private keys, or token-like strings.

## 13. Failure Semantics

Citation construction failures should fail citation construction, not local check execution.

Rules:

- invalid citation references fail closed;
- citation failures must not fabricate local check references;
- citation failures must not change workflow run status;
- citation failures must not append runtime events;
- citation failures must not become misleading project diagnostics;
- errors must use stable, non-leaking messages.

## 14. Test Plan For Future Implementation

Future implementation should test:

- `WorkReportCitationKind::LocalCheckResult` is representable;
- `WorkReportCitationTarget::LocalCheckResult` validates with a safe stable reference;
- citation kind mapping returns `LocalCheckResult`;
- serde round trip for local check result citation target;
- debug output does not leak local check result reference text;
- secret-like local check result reference is rejected without leaking values;
- citation summary does not copy raw stdout/stderr by default;
- serialization does not contain raw command-output markers;
- existing WorkReport tests still pass;
- existing LocalCheckResultReference tests still pass;
- existing EvidenceReference, Diagnostic, adapter telemetry, runtime, and executor tests still pass.

## 15. Documentation Requirements For Future Implementation

Docs must say:

- WorkReport citation vocabulary for local check results is implemented, if implemented;
- terminal report helper integration is not implemented;
- automatic local check citation wiring is not implemented;
- local check result EvidenceReference attachment is not implemented;
- command-output evidence is not implemented;
- local check result persistence is not implemented;
- report artifact writing from this path is not implemented;
- default registration is not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported.

## 16. Later Implementation Sequence

Recommended remaining sequence:

1. Maintainer review of terminal report helper integration.
2. Command-output evidence policy planning only if evidence attachment is needed.
3. Persistence/artifact integration only after separate planning.
4. CLI/schema/default registration only after authority and side-effect posture are reviewed.

## 17. Open Questions

- Should `WorkReportCitationTarget::LocalCheckResult` use `WorkReportStableReference` or a public `LocalCheckResultId` directly?
- Should `WorkReportCitationKind::LocalCheckResult` become a default `WorkReportContract::v1_default()` citation requirement, or remain optional?
- Should terminal report helpers accept `LocalCheckResultReference` values or prebuilt `WorkReportCitation` values?
- Should skipped and not-available local checks become citations, section text, or both?
- Should failed and timed-out checks be cited differently from passed checks?
- Should local check result references become persistable before report helper integration?
- Should command-output evidence ever be allowed, or should local check result citations remain the preferred path?

## 18. Final Recommendation

Proceed next with **terminal report local check citation integration review**.

The review should verify supplied stable local check result references are consumed by the terminal report helper and cited in the validation and quality checks section. Do not build automatic local check execution, local check reference creation, EvidenceReference attachment, command-output evidence, persistence, artifacts, CLI behavior, workflow schema fields, default registration, `AllowlistedHandlerOnly`, side-effect modeling, writes, or release posture changes.
