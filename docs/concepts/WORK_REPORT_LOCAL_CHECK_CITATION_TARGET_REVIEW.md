# WorkReport Local Check Citation Target Review

Review date: 2026-06-15

## 1. Executive Verdict

Phase accepted; proceed to terminal report helper integration planning.

The WorkReport local check citation target implementation is narrow, model-only, and consistent with the accepted plan. It adds citation vocabulary for local check result references without wiring report generation, creating evidence, persisting local check results, exposing CLI behavior, changing schemas, registering handlers by default, modeling side effects, adding writes, or changing release posture.

## 2. Scope Verification

The phase stayed within the approved WorkReport citation-vocabulary scope.

Implemented:

- `WorkReportCitationKind::LocalCheckResult`;
- `WorkReportCitationTarget::LocalCheckResult`;
- citation-kind mapping;
- serde support through existing citation target serialization and deserialization;
- redaction-safe debug behavior through the existing `WorkReportCitationTarget` debug implementation;
- focused tests;
- roadmap and planning documentation updates;
- phase report.

No accidental implementation was found for:

- terminal report helper integration;
- automatic local check citation wiring;
- local check result reference creation from handlers or executor paths;
- EvidenceReference attachment;
- `EvidenceKind::CommandOutput`;
- local check result persistence;
- report artifact writing;
- default `DocsCheck` registration;
- CLI exposure;
- workflow schema fields;
- automatic local check execution;
- `AllowlistedHandlerOnly` enablement;
- side-effect boundary implementation;
- source writes;
- release posture changes.

## 3. Model Assessment

The model change is appropriately minimal.

The implementation adds `LocalCheckResult` to `WorkReportCitationKind` and a `LocalCheckResult { reference: WorkReportStableReference }` variant to `WorkReportCitationTarget`. That shape matches the established stable-reference pattern already used for adapter telemetry and future reasoning-lineage vocabulary.

The implementation does not embed `LocalCheckResultReference` directly in `WorkReportCitationTarget`, which keeps WorkReport citation vocabulary decoupled from the full local check model while still allowing future report sections to cite local check outcomes by stable reference.

## 4. Citation Boundary Assessment

The citation boundary remains safe and explicit.

Verified:

- local check citations use stable references only;
- `WorkReportStableReference::new(...)` validates reference text and rejects secret-like values;
- `WorkReportCitation::new(...)` remains the construction boundary for citation summaries, redaction metadata, missing flags, and sensitivity;
- `WorkReportCitationTarget::citation_kind()` maps the new target to `WorkReportCitationKind::LocalCheckResult`;
- the new target does not create or recreate `EvidenceReference` values;
- the new target does not fabricate local check result IDs;
- the new target does not consume raw local check output.

## 5. Serialization And Debug Assessment

Serde support is acceptable for this phase.

Verified:

- `WorkReportCitationKind::LocalCheckResult` serializes with the existing `snake_case` enum policy;
- `WorkReportCitationTarget::LocalCheckResult` serializes with the existing tagged target representation;
- valid local check citation targets round trip through `WorkReportCitation` serialization and deserialization;
- `WorkReportStableReference` deserialization validates via `TryFrom<String>`;
- `WorkReportCitation` deserialization routes through `WorkReportCitation::new(...)`;
- debug output for `WorkReportCitationTarget` redacts target references;
- debug output for `WorkReportCitation` redacts summaries and redaction metadata.

Serialization may include stable local check references. That is acceptable because the target is a citation pointer, not a payload store. Raw command output, command transcripts, and secret-like values remain excluded.

## 6. Privacy And Redaction Assessment

No raw payload copying was introduced.

The implementation does not store or copy:

- raw stdout;
- raw stderr;
- command transcripts;
- command output summaries by default;
- environment values;
- docs contents;
- parser payloads;
- provider payloads;
- CI logs;
- credentials;
- tokens;
- authorization headers;
- private keys.

Secret-like local check reference text is rejected at `WorkReportStableReference` construction with a stable non-leaking error. Debug output redacts target references.

## 7. Relationship To Local Check Results

The implementation correctly distinguishes citation vocabulary from local check execution and local check result production.

This phase does not create `LocalCheckResult` values, does not create `LocalCheckResultReference` values, does not run local checks, and does not make local check results durable. It only adds the WorkReport citation target that future report-generation code may use once a stable local check reference already exists.

That boundary preserves the earlier local-check posture: handlers remain explicit, non-default, bounded, and non-write-capable.

## 8. Relationship To EvidenceReference

The implementation does not prematurely convert local check results into EvidenceReference values.

That is the right choice for this phase. Command-output evidence remains a separate policy question because it can become raw log storage if introduced too early. WorkReport can cite local check references as model facts without making them evidence.

## 9. Test Quality Assessment

The tests cover the important behaviors for this narrow phase:

- valid local check result citation target construction;
- citation kind mapping;
- serde round trip;
- secret-like stable reference rejection without leaking the rejected value;
- debug non-leakage for the stable reference;
- serialization non-copying of raw command-output markers;
- existing WorkReport, WorkReportContract, LocalCheckResult, EvidenceReference, Diagnostic, adapter telemetry, executor, and runtime tests through workspace validation.

One non-blocking hardening opportunity remains: add an explicit invalid serialized `local_check_result` citation payload test before schema or artifact exposure. The current implementation should already fail closed through `WorkReportStableReference` and `WorkReportCitation` deserialization, but a dedicated regression test would make that guarantee easier to review later.

## 10. Documentation Review

Documentation is honest about the current state.

Verified docs state or preserve that:

- WorkReport citation vocabulary for local check results is implemented;
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

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add an explicit invalid serialized `local_check_result` citation payload regression test before any schema, artifact, or CLI exposure.
- Plan terminal report helper integration for supplied local check result references.
- Keep command-output EvidenceReference policy deferred until separately reviewed.
- Keep default local check handler registration deferred until authority and side-effect posture are reviewed.

## 13. Recommended Next Phase

Recommended next phase: terminal report helper integration planning for supplied local check result references.

The model vocabulary is now in place. The next safe step is planning how explicit local check result references should be accepted by the in-memory terminal report helper and placed in the `validation and quality checks` section without creating evidence, persisting results, invoking checks, changing executor behavior, or adding CLI/schema exposure.

## 14. Governance Run

This review phase was governed by the self-governance dogfood workflow before the review document was written.

- State directory: `/tmp/workflow-os-local-check-citation-target-review`
- Run ID: `run-1781541142604839000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781541142604839000-2/d`
- Final status: `Completed`

## 15. Validation

Validation commands run for this review:

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
