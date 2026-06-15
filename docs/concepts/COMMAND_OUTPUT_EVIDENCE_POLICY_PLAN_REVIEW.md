# Command Output Evidence Policy Plan Review

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The policy plan is appropriately conservative. It keeps command-output evidence attachment deferred, preserves the current local check result reference and WorkReport citation path, and avoids turning `EvidenceReference` into raw command-output or log storage.

The plan is ready to guide future implementation prompts only if a concrete command-output evidence need appears. The immediate roadmap should defer command-output evidence implementation and return to the broader governed-work queue.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

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

No accidental implementation scope was introduced.

## 3. Architecture Fit

The plan fits the current Workflow OS architecture.

Workflow OS already has:

- bounded `LocalCheckResult` values;
- validated `LocalCheckResultReference` values;
- `WorkReportCitationTarget::LocalCheckResult`;
- terminal report helper integration for supplied local check result references;
- `EvidenceKind::CommandOutput` vocabulary with command-output-specific safety tests.

The plan correctly treats those as separate layers:

- local check result: bounded command/check outcome;
- local check result reference: stable citation pointer;
- WorkReport citation: governed report reference;
- command-output evidence: future high-risk evidence category.

That separation preserves the local-first kernel, avoids premature persistence or artifact assumptions, and keeps evidence reference usage reference-first.

## 4. Baseline Assessment

The current baseline described by the plan is accurate.

Implemented:

- `EvidenceKind::CommandOutput`;
- `EvidenceReferenceTarget::command_output(...)`;
- command-output validation requiring meaningful redaction metadata;
- command-output serialization redaction behavior;
- bounded `LocalCheckResult`;
- redaction-safe `LocalCheckResult` debug behavior;
- `LocalCheckResultReference` without stdout/stderr summaries;
- `WorkReportCitationTarget::LocalCheckResult`;
- terminal report helper integration for supplied local check result references.

Not implemented:

- command-output evidence attachment;
- local check evidence attachment;
- automatic local check result reference creation from handlers or executor paths;
- local check result persistence;
- command-output CLI inspection;
- command-output report artifacts;
- workflow schema fields for command-output evidence.

The baseline avoids false claims about current support.

## 5. Policy Assessment

The recommended policy is correct:

- do not use `EvidenceKind::CommandOutput` for local check result reporting today;
- cite `LocalCheckResultReference` through `WorkReportCitationTarget::LocalCheckResult`;
- treat local check citations as report citations, not evidence records;
- do not create `EvidenceReference` values implicitly during report generation;
- defer command-output evidence attachment until a separate implementation prompt defines a specific safe use case.

This policy prevents command-output evidence from becoming default log storage and gives future work a narrow, reviewable path.

## 6. Allowed And Forbidden Data Assessment

The allowed data list is bounded and reference-oriented:

- stable local check result reference ID;
- command contract ID;
- command kind;
- result status;
- workflow/run IDs;
- event IDs where available;
- stable output reference if separately approved;
- non-secret status summary;
- truncation flags;
- stable error code;
- approved content hash only;
- redaction metadata;
- sensitivity;
- retention hint.

The forbidden data list is appropriately strict:

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

This is enough to prevent the most obvious transcript, log, and secret leakage paths.

## 7. Evidence Kind And Scope Assessment

The plan's evidence kind and scope rules are appropriate.

Allowed future shape:

- `EvidenceKind::CommandOutput` only for bounded command-output references;
- `EvidenceScope::Run` only with real run identity;
- `EvidenceScope::Validation` for validation/check command outcomes;
- `EvidenceScope::Workflow` only when the reference is meaningful outside a single run.

The plan correctly flags confusing combinations:

- command output without run or validation context;
- command output attached to diagnostics by default;
- command output created from report generation without an existing command/check result source;
- external-scoped command output without an external safe reference.

## 8. Attachment Boundary Assessment

The future attachment boundary requirements are strong enough:

- dedicated validated API;
- `EvidenceReference::validate()`;
- command-output-specific validation in addition to generic evidence validation;
- meaningful redaction metadata;
- conservative sensitivity;
- fail-closed behavior;
- atomic multi-attachment;
- read-only accessors;
- deserialization validation;
- stable non-leaking errors.

The plan explicitly rejects relying on a caller's claim that command output was already reviewed elsewhere. That is the right posture for a high-risk evidence class.

## 9. Relationship To Local Checks And Work Reports

The plan correctly preserves the current safe path:

- `LocalCheckResult` remains the bounded check result model;
- `LocalCheckResultReference` remains the preferred citation surface;
- WorkReports cite local check result references in `validation and quality checks`;
- report generation must not create command-output evidence implicitly;
- report generation must not copy local check stdout/stderr summaries by default;
- report generation must not fabricate evidence for missing command output.

This keeps report construction useful without creating a hidden evidence store.

## 10. Persistence And Artifact Assessment

The plan does not authorize persistence or artifacts.

It correctly states that a later artifact phase would need to separately define location, redaction, retention, access, hash posture, citation behavior, and deletion/failure behavior.

That avoids accidental product commitments around durable command-output storage.

## 11. Privacy And Redaction Assessment

The plan is privacy-safe for planning purposes.

It treats command output as high-risk because it may contain:

- secrets;
- credentials;
- authorization headers;
- private keys;
- environment values;
- raw CI logs;
- raw provider payloads;
- parser payloads;
- raw spec contents;
- sensitive paths.

It also preserves existing redaction-safe behavior for local check and evidence models by requiring any future command-output evidence to pass validated constructors and non-leaking error paths.

## 12. Test Plan Assessment

The future test plan is adequate.

It covers:

- raw stdout/stderr rejection or redaction;
- transcript rejection;
- secret-like summary and metadata rejection;
- reference-only command-output validation;
- required redaction metadata;
- conservative sensitivity;
- invalid serde failure;
- deserialization error non-leakage;
- debug and serialization non-leakage;
- local check result references continuing to work without evidence;
- WorkReports continuing to cite local checks without creating evidence.

Non-blocking hardening:

- Add a future test that a valid `LocalCheckResultReference` can be cited by a WorkReport without any `EvidenceReference` objects existing.
- Add a future test that report artifact storage, if used later, does not convert local check citations into command-output evidence.

## 13. Documentation Review

Documentation is honest about current state.

Updated docs state:

- command-output evidence policy planning is documented;
- command-output evidence attachment remains unimplemented;
- local check report paths cite `LocalCheckResultReference` values through WorkReport citations;
- command-output evidence implementation remains deferred;
- persistence, artifacts, CLI, schemas, examples, side effects, writes, and release posture changes are not introduced.

No dangerous false claim was found.

## 14. Planning Blockers

None.

## 15. Non-Blocking Follow-Ups

- Decide later whether Workflow OS needs a distinct `EvidenceKind::LocalCheckResult` instead of using `CommandOutput` for local check outcomes.
- Before any artifact or schema exposure, add invalid serialized local-check-reference and command-output evidence regression tests.
- Keep command-output evidence out of report generation until a concrete evidence use case exists.
- If command-output evidence becomes necessary, start with validator-only or reference-only implementation work.

## 16. Recommended Next Phase

Recommended next phase: defer command-output evidence implementation and return to the broader roadmap queue.

The current local check result citation path is sufficient for WorkReports to cite check outcomes without copying output. Implementing command-output evidence now would add risk before there is a concrete need. If the roadmap later needs command-output evidence, the next phase should be a narrow reference-only command-output evidence validator, not handler/report attachment.
