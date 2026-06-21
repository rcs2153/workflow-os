# Report Artifact SideEffect Referential Integrity Plan

Status: Validation-only helper implemented in [Report Artifact SideEffect Referential Integrity Report](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md) and accepted in [Report Artifact SideEffect Referential Integrity Review](../concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md). This plan follows the accepted [Executor SideEffect Discovery Opt-In Review](../concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md). It does not implement artifact writes from executor paths, automatic SideEffect discovery, runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS can now:

- create validated `WorkReport` values;
- explicitly store validated `WorkReportArtifactRecord` values;
- persist validated `SideEffectRecord` values;
- discover SideEffect IDs through explicit helper paths;
- cite SideEffect IDs in WorkReports.

The next question is how explicit report artifact persistence should treat WorkReports that cite SideEffect IDs. A stored report artifact that cites a SideEffect ID should not silently imply the cited SideEffect record exists, matches the report run identity, or was validated by the artifact store unless an explicit integrity path checked that relationship.

This plan defines a conservative, opt-in referential integrity boundary for report artifacts that cite SideEffect IDs. The first validation-only helper is implemented; combined artifact writing remains deferred.

## 2. Goals

- Define how future artifact writes can validate cited SideEffect IDs.
- Preserve explicit artifact writing only.
- Preserve explicit SideEffect discovery only.
- Preserve workflow execution semantics.
- Preserve report-generation failure semantics.
- Keep report artifacts separate from workflow events and snapshots.
- Keep SideEffect records separate from WorkReport artifacts.
- Validate SideEffect citation references by stable IDs and immutable run identity.
- Avoid copying SideEffect record payloads into artifacts.
- Avoid changing existing `WorkReportArtifactStore` behavior unless a caller opts into integrity validation.
- Prepare a small implementation prompt that can be reviewed before artifact integrity becomes part of broader executor behavior.

## 3. Non-Goals

This plan does not authorize:

- automatic report artifact writing;
- automatic SideEffect discovery;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- changing `execute_with_report_and_side_effect_discovery(...)`;
- making `SideEffectRecordStore` part of `WorkReportArtifactStore`;
- making `WorkReportArtifactStore` part of `StateBackend`;
- appending workflow events for artifact integrity checks;
- mutating `WorkflowRun` or `WorkflowRunSnapshot`;
- report artifact signing or notarization;
- CLI rendering or export;
- workflow spec schema fields;
- examples;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters;
- provider mutation;
- hosted or distributed runtime behavior;
- reasoning lineage;
- DLP or access-control systems;
- release posture changes.

## 4. Current Baseline

Current report artifact baseline:

- `WorkReportArtifactRecord::new(...)` validates one contained `WorkReport`.
- `WorkReportArtifactMetadata` is derived from the contained report.
- metadata/report identity mismatch fails closed.
- `WorkReportArtifactStore` supports explicit write/read/list operations.
- `LocalStateBackend` stores artifacts under `work_reports/<run-id>/<report-id>.json`.
- existing executor paths do not write report artifacts automatically.

Current SideEffect baseline:

- `SideEffectRecord` stores validated SideEffect intent/lifecycle records.
- `SideEffectRecordStore` persists and lists records separately from report artifacts.
- local SideEffect records preserve full immutable run identity.
- SideEffect discovery helpers return stable SideEffect IDs and bounded counts.
- WorkReports cite SideEffect IDs, not raw SideEffect payloads.
- executor SideEffect discovery opt-in can generate in-memory reports with discovered SideEffect citations.

Current gap:

- an explicit artifact write can store a valid WorkReport whose SideEffect citations are syntactically valid but not checked against `SideEffectRecordStore`.

## 5. Integrity Problem Statement

WorkReport artifacts are governed handoff records. If a stored report artifact cites a SideEffect ID, later readers may reasonably ask:

- Does the cited SideEffect record exist?
- Does it belong to the same workflow run?
- Does it match the report's immutable workflow identity?
- Was it validated before the report was persisted?
- Was a missing citation intentionally allowed?

The first artifact store intentionally did not answer those questions. That was correct for the first local artifact boundary, but SideEffect citations now have enough supporting model and store infrastructure to plan an explicit integrity check.

The integrity check must avoid overclaiming. A successful referential check should mean only:

```text
This artifact's SideEffect citations resolve to validated SideEffect records that match the report's immutable run identity.
```

It must not mean:

- the SideEffect was executed;
- the SideEffect was approved;
- provider state changed;
- the cited record is complete evidence;
- all possible SideEffects for the run were discovered;
- runtime writes are supported.

## 6. Recommended First Boundary

Recommended first implementation: an explicit artifact integrity helper, not a change to the existing artifact store trait.

Recommended shape:

```rust
pub struct WorkReportArtifactSideEffectIntegrityInput<'a> {
    pub artifact: &'a WorkReportArtifactRecord,
    pub require_all_side_effect_citations: bool,
}

pub struct WorkReportArtifactSideEffectIntegrityResult {
    // bounded counts only
}

pub fn validate_work_report_artifact_side_effect_integrity(
    store: &impl SideEffectRecordStore,
    input: WorkReportArtifactSideEffectIntegrityInput<'_>,
) -> Result<WorkReportArtifactSideEffectIntegrityResult, WorkflowOsError>
```

Optional second helper:

```rust
pub fn write_work_report_artifact_with_side_effect_integrity(
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    artifact: &WorkReportArtifactRecord,
    policy: WorkReportArtifactSideEffectIntegrityPolicy,
) -> Result<(), WorkflowOsError>
```

The first implementation should prefer the validation helper only. A combined write helper can follow after review if needed.

## 7. Why Not Change `write_work_report_artifact(...)`

Do not make `WorkReportArtifactStore::write_work_report_artifact(...)` automatically validate SideEffect citations yet.

Reasons:

- existing artifact writes were reviewed as validating the report artifact itself;
- not every artifact store implementation should be forced to know about SideEffect records;
- SideEffect citations may be allowed as stable references even when the SideEffect store is unavailable;
- automatic integrity checks would add a new failure mode to existing explicit artifact writes;
- artifact integrity policy needs to be caller-visible and testable;
- future artifact types may cite other stores with different integrity policies.

The existing write method should remain a report-artifact validation boundary. SideEffect referential integrity should be an explicit extra step.

## 8. Integrity Policy

Recommended first policy:

- Validate only citations whose target is `WorkReportCitationTarget::SideEffect`.
- Extract cited `SideEffectId` values from all report sections.
- De-duplicate cited IDs deterministically.
- Read matching records through `SideEffectRecordStore`.
- Require records only when the caller chooses strict mode.
- Validate matching records against the report's immutable run identity:
  - workflow ID;
  - workflow version;
  - schema version;
  - spec hash;
  - run ID.
- Return bounded counts:
  - cited SideEffect IDs;
  - resolved records;
  - missing records;
  - mismatched records;
  - duplicate citations.

Recommended first strict mode:

- If any cited SideEffect ID has no matching record, fail closed.
- If any matching record has mismatched immutable identity, fail closed.
- If the store returns corrupt data, fail closed.

Recommended first permissive mode:

- Missing records are reported as bounded counts.
- Identity mismatches and corrupt records still fail closed.

## 9. Artifact Write Semantics

Artifact integrity checks must be explicit and separate from workflow execution.

If a future combined write helper is implemented, the recommended order is:

1. Validate the artifact itself through `WorkReportArtifactRecord::validate()`.
2. Validate SideEffect referential integrity through the explicit helper.
3. Write the artifact through `WorkReportArtifactStore::write_work_report_artifact(...)`.

Rules:

- do not generate a report during artifact write;
- do not discover SideEffects automatically unless explicitly supplied to the helper;
- do not append workflow events;
- do not emit audit or observability events;
- do not mutate workflow state;
- do not create, repair, or transition SideEffect records;
- do not execute side effects;
- do not call adapters or providers;
- do not write artifact files except through the supplied `WorkReportArtifactStore`;
- if integrity validation fails, do not write a partial artifact.

## 10. Source And Store Boundary

The integrity helper should accept an explicit `SideEffectRecordStore`.

It should not:

- construct a `LocalStateBackend`;
- assume artifact store and SideEffect store are the same object;
- read hidden global state;
- derive local filesystem paths;
- make `SideEffectRecordStore` a supertrait of `WorkReportArtifactStore`;
- make either store a `StateBackend` requirement.

This keeps future production stores flexible. One deployment may store report artifacts and SideEffect records in different systems.

## 11. Relationship To Discovery

Referential integrity is not discovery.

Discovery asks:

```text
Which SideEffect IDs should this report cite?
```

Artifact referential integrity asks:

```text
Do the SideEffect IDs already cited by this report resolve to records with matching immutable run identity?
```

The integrity helper should not add new SideEffect citations to the report. It should not repair reports. It should not create missing records. It should only validate the relationship between the artifact and the SideEffect store.

## 12. Relationship To EvidenceReference

EvidenceReference side-effect attachment remains deferred.

This phase should not create EvidenceReference values from SideEffect records or report artifacts. Later EvidenceReference work may decide how SideEffect records can be cited as evidence, but artifact referential integrity should stay at the ID/record validation boundary.

## 13. Relationship To Approvals

Approval-side-effect linkage remains deferred.

The integrity helper may validate that cited SideEffect records exist and match immutable run identity. It should not require approval references, validate approval authority, enforce quorum rules, or decide whether a SideEffect was properly approved.

Approval linkage should be planned separately before write-capable adapters.

## 14. Error Handling

Errors must be stable and non-leaking.

Candidate error codes:

- `work_report_artifact.side_effect_integrity.record_missing`;
- `work_report_artifact.side_effect_integrity.identity_mismatch`;
- `work_report_artifact.side_effect_integrity.record_corrupt`;
- `work_report_artifact.side_effect_integrity.store_read_failed`;
- `work_report_artifact.side_effect_integrity.invalid_artifact`.

Errors must not include:

- SideEffect IDs;
- WorkReport IDs;
- Workflow IDs;
- run IDs;
- workflow versions;
- schema versions;
- spec hashes;
- store paths;
- SideEffect targets;
- SideEffect summaries;
- authority context;
- lifecycle payloads;
- idempotency details;
- raw record JSON;
- report section text;
- provider payloads;
- command output;
- parser payloads;
- tokens, credentials, private keys, or secret-like values.

## 15. Privacy And Redaction

The helper must remain reference-only.

It may inspect:

- report citation targets;
- typed SideEffect IDs;
- report immutable run identity;
- SideEffect record immutable run identity;
- bounded lifecycle metadata needed only for validation if necessary.

It must not copy into results, errors, Debug output, or artifacts:

- SideEffect target references;
- SideEffect summaries;
- reason codes;
- authority context;
- idempotency details;
- raw record JSON;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira/GitHub bodies or file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- token-like values;
- local filesystem paths.

Debug output for any new input/result type should redact IDs and expose bounded counts only.

## 16. Test Plan

Future implementation tests should cover:

- valid artifact with no SideEffect citations succeeds with zero counts;
- valid artifact with SideEffect citation and matching record succeeds;
- multiple SideEffect citations de-duplicate deterministically;
- missing record fails in strict mode without leaking ID;
- missing record returns bounded count in permissive mode;
- record workflow ID mismatch fails without leaking values;
- record workflow version mismatch fails without leaking values;
- record schema version mismatch fails without leaking values;
- record spec hash mismatch fails without leaking values;
- record run ID mismatch fails without leaking values;
- corrupt store record fails closed without leaking payload;
- generic store read failure maps to stable non-leaking error;
- helper does not mutate artifact, report, SideEffect records, workflow events, snapshots, or stores;
- helper does not write report artifacts;
- helper does not create SideEffect records;
- combined write helper, if implemented later, does not write artifact after integrity failure;
- Debug output does not leak SideEffect IDs, report IDs, run IDs, paths, targets, summaries, or secret-like values;
- serialization, if any result type serializes, contains bounded counts only;
- existing WorkReport, artifact store, SideEffect store/discovery, executor, runtime, EvidenceReference, Diagnostic, validation, adapter, hook, local-check, and docs tests continue to pass.

## 17. Proposed Implementation Sequence

Recommended future phases:

1. Review the implemented validation-only integrity helper.
2. Plan an explicit combined artifact-write helper only if callers need it.
3. Review before executor artifact-writing, CLI artifact inspection, automatic discovery, approval-side-effect linkage, EvidenceReference side-effect attachment, runtime side-effect execution, or write-capable adapters.

## 18. Open Questions

- Should permissive mode exist in the first implementation, or should v1 be strict-only?
- Should integrity result types be serializable, or remain in-memory only?
- Should the helper validate only SideEffect IDs already cited in the report, or also compare against all records for the run?
- Should duplicate SideEffect citations be reported as a count or ignored after deterministic de-duplication?
- Should missing records ever be represented as explicit report warnings, or stay outside artifact validation?
- Should artifact health checks eventually scan SideEffect citation integrity?
- Should artifact writes later become idempotent when the artifact and integrity result are identical?
- Should approval-side-effect linkage be planned before or after artifact integrity implementation?

## 19. Final Recommendation

The first implementation phase is complete: **report artifact SideEffect referential integrity helper, validation-only**.

The implemented helper validates SideEffect citations already present in a `WorkReportArtifactRecord` against a caller-supplied `SideEffectRecordStore`. It is in-memory, reference-only, non-mutating, and separate from normal artifact writes.

Recommended next phase: **approval-side-effect linkage planning**.

Do not build automatic artifact writing, automatic SideEffect discovery, executor integration, CLI rendering, schemas, examples, EvidenceReference side-effect attachment, approval-side-effect enforcement, runtime side-effect execution, attempted/completed/failed executor behavior, writes, provider mutation, hosted behavior, reasoning lineage, or release posture changes.
