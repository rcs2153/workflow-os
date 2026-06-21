# Approval SideEffect Linkage Composition Plan

Status: Implemented as a validation-only store-backed helper in [SideEffect Approval Linkage Store-Backed Report](../concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REPORT.md). This plan follows the accepted validation-only helper in [SideEffect Approval Linkage Review](../concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REVIEW.md). It does not implement report/artifact composition, runtime side-effect execution, write-capable adapters, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has a validation-only helper that can prove approval-side-effect linkage for already-loaded `SideEffectRecord` values against an existing `WorkflowRun` event history.

The next question is how to compose that helper with persisted SideEffect records, WorkReport SideEffect citations, and report artifact integrity checks without making linkage automatic by accident.

This plan recommended the smallest next implementation: an explicit store-backed approval linkage helper. The helper now loads already-persisted SideEffect records through a supplied `SideEffectRecordStore`, applies caller-visible policy, and delegates approval linkage validation to `validate_side_effect_approval_linkage(...)`.

The implementation remains explicit and validation-only.

## 2. Goals

- Compose the accepted in-memory approval linkage helper with persisted SideEffect records.
- Preserve approval as authority context, not lifecycle state.
- Preserve deterministic workflow execution and replay.
- Keep linkage validation explicit and caller-requested.
- Keep report generation, report artifact writing, and executor execution unchanged unless a future phase opts in.
- Validate using existing workflow approval events and existing SideEffect records.
- Fail closed when a SideEffect claims approval authority that cannot be proven.
- Keep errors stable, bounded, and non-leaking.
- Prepare for later report/artifact composition before write-capable adapters.
- Preserve current approval, SideEffect, WorkReport, artifact, and executor behavior.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic approval-side-effect validation during report generation;
- automatic approval-side-effect validation during report artifact writes;
- automatic approval-side-effect validation during executor execution;
- changing `validate_side_effect_approval_linkage(...)`;
- changing `LocalExecutor::execute(...)`;
- changing `LocalExecutor::execute_with_report(...)`;
- changing `execute_with_report_and_side_effect_discovery(...)`;
- changing `WorkReportArtifactStore::write_work_report_artifact(...)`;
- creating, mutating, or deleting SideEffect records;
- creating, mutating, or deleting approval requests or decisions;
- appending workflow events;
- emitting audit or observability events;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable adapters;
- provider mutation;
- approval evidence attachment;
- workflow schema fields;
- CLI rendering or commands;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Baseline

Implemented and accepted:

- `SideEffectRecord` core model;
- explicit proposed/denied/skipped SideEffect workflow event append path;
- `SideEffectRecordStore` and local persistence;
- explicit SideEffect discovery helpers;
- WorkReport SideEffect citation vocabulary;
- terminal report helper propagation of explicitly supplied SideEffect IDs;
- executor report input propagation for explicitly supplied SideEffect IDs;
- WorkReport-side SideEffect discovery integration;
- executor SideEffect discovery opt-in helper;
- report artifact SideEffect referential integrity helper;
- validation-only approval linkage helper for already-loaded records.
- store-backed approval linkage helper for explicitly supplied IDs and/or all records for a run.

Not implemented:

- automatic approval linkage validation in any report, artifact, executor, or discovery path;
- approval evidence attachment;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters.

## 5. Composition Problem Statement

The accepted linkage helper answers:

```text
Do these already-loaded SideEffect records cite approval events from the same immutable run, and do those approval decisions match the SideEffect authority posture?
```

Future callers will often have only a `WorkflowRun`, a `SideEffectRecordStore`, and either:

- all records for the run;
- a set of SideEffect IDs cited by a WorkReport;
- a set of SideEffect IDs discovered from workflow events or persisted records.

Those callers need a safe composition boundary that loads records explicitly and then reuses the accepted linkage helper. They should not have to duplicate store reads, identity checks, or non-leaking error mapping in every future report/artifact/executor path.

The first composition boundary must still avoid overclaiming. Successful approval linkage should mean only:

```text
The supplied or discovered SideEffect records that claim approval authority are backed by matching approval events in this run.
```

It must not mean:

- the SideEffect was executed;
- provider state changed;
- the approver had high-assurance role authority;
- evidence was sufficient for approval;
- all relevant SideEffects were discovered;
- a WorkReport artifact was written;
- a write-capable adapter is safe to enable.

## 6. Recommended First Boundary

Recommended first implementation: **explicit store-backed approval linkage helper**.

Candidate shape:

```rust
pub struct SideEffectApprovalLinkageFromStoreInput<'a> {
    pub run: &'a WorkflowRun,
    pub side_effect_ids: &'a [SideEffectId],
    pub load_mode: SideEffectApprovalLinkageStoreLoadMode,
    pub missing_record_policy: SideEffectMissingRecordPolicy,
    pub require_approval_references_for_requires_approval: bool,
    pub require_decision_for_approved_or_denied: bool,
}

pub enum SideEffectApprovalLinkageStoreLoadMode {
    ExplicitIds,
    AllRecordsForRun,
    ExplicitIdsAndAllRecordsForRun,
}

pub enum SideEffectMissingRecordPolicy {
    RequireAll,
    CountMissing,
}

pub struct SideEffectApprovalLinkageFromStoreResult {
    // bounded counts only
}

pub fn validate_side_effect_approval_linkage_from_store(
    store: &impl SideEffectRecordStore,
    input: SideEffectApprovalLinkageFromStoreInput<'_>,
) -> Result<SideEffectApprovalLinkageFromStoreResult, WorkflowOsError>
```

Exact names may follow local conventions, but the first implementation should:

1. accept an explicit `SideEffectRecordStore`;
2. accept an existing `WorkflowRun`;
3. optionally accept explicit `SideEffectId` values;
4. optionally load all SideEffect records for the run;
5. validate loaded records through the existing `validate_side_effect_approval_linkage(...)`;
6. return bounded counts and no record payloads.

Do not integrate this helper into report generation, artifact writing, or executor behavior in the first implementation.

## 7. Store Read Policy

The helper should support two caller-visible modes:

- explicit ID mode: read only caller-supplied `SideEffectId` values;
- all-records-for-run mode: use `SideEffectRecordStore::list_side_effect_records_for_workflow_run(...)`.

Rules:

- At least one source must be selected.
- Explicit IDs must not be fabricated.
- Duplicate explicit IDs should be de-duplicated deterministically and counted.
- Records loaded through explicit IDs must match the supplied run identity.
- Records loaded by run must already be filtered and validated by the store, then revalidated by the linkage helper.
- Missing explicit records fail closed when `require_all_referenced_records` is true.
- Missing explicit records may be counted when `require_all_referenced_records` is false.
- Store read failures fail closed and must be mapped to stable non-leaking errors.

The helper should not construct a local backend, infer filesystem paths, or assume the SideEffect store and report artifact store are the same object.

## 8. Relationship To Discovery

Discovery and approval linkage should remain separate.

Discovery asks:

```text
Which SideEffect IDs are relevant to this run or report?
```

Approval linkage asks:

```text
Do the loaded SideEffect records that claim approval authority have matching approval events?
```

A future caller may compose them explicitly:

1. discover SideEffect IDs;
2. load records from the store;
3. validate approval linkage;
4. generate or validate report/artifact output.

The first store-backed approval linkage helper should not run discovery itself unless a later implementation phase explicitly scopes a combined discovery-plus-linkage helper.

## 9. Relationship To WorkReport Generation

WorkReport generation should not automatically run approval linkage in the next implementation.

Reasons:

- existing report helpers were reviewed as citation constructors, not authority validators;
- report generation failure semantics are already carefully separated from workflow execution;
- not every WorkReport caller has a `SideEffectRecordStore`;
- not every report needs approval linkage validation immediately;
- automatic linkage would introduce a new failure mode to existing explicit report generation paths.

Future report-generation composition should be additive and explicit, such as:

```rust
generate_terminal_local_work_report_with_side_effect_discovery_and_approval_linkage(...)
```

or a narrower helper that validates linkage before calling the existing report helper.

That future helper must preserve workflow result semantics and must not write artifacts.

## 10. Relationship To Report Artifact Integrity

Report artifact SideEffect referential integrity and approval linkage answer different questions.

Referential integrity asks:

```text
Do cited SideEffect IDs resolve to records matching this report artifact's immutable run identity?
```

Approval linkage asks:

```text
Do SideEffect records that claim approval authority resolve to matching approval events?
```

A future artifact composition phase may validate both before writing an artifact, but the first store-backed approval linkage helper should not change `WorkReportArtifactStore::write_work_report_artifact(...)`.

If a future combined artifact helper is implemented, recommended order:

1. validate artifact;
2. validate SideEffect citation referential integrity;
3. validate approval linkage for resolved records;
4. write artifact only if all requested checks pass.

If any validation fails, no partial artifact should be written.

## 11. Policy Model

The first store-backed helper should expose policy rather than hiding it.

Recommended policy fields:

- require every explicitly cited record to exist;
- include all records for the run;
- require approval references for `RequiresApproval`;
- require approval decisions for `ApprovedByHuman` and `DeniedByApproval`;
- optionally count records with no approval authority.

The first implementation should not add role-based approval policy, quorum policy, approver/requester separation, expiration enforcement, revocation handling, or external identity provider checks. Those belong in future high-assurance approval phases.

## 12. Error Handling

Errors must use stable, non-leaking codes.

Candidate codes:

- `side_effect_approval_linkage.store_read_failed`
- `side_effect_approval_linkage.record_missing`
- `side_effect_approval_linkage.record_corrupt`
- `side_effect_approval_linkage.identity_mismatch`
- `side_effect_approval_linkage.invalid_input`

Existing in-memory helper errors should remain unchanged:

- `side_effect_approval_linkage.approval_missing`
- `side_effect_approval_linkage.decision_missing`
- `side_effect_approval_linkage.decision_kind_mismatch`
- `side_effect_approval_linkage.step_mismatch`
- `side_effect_approval_linkage.skill_mismatch`
- `side_effect_approval_linkage.record_invalid`
- `side_effect_approval_linkage.run_invalid`

Errors must not leak:

- SideEffect IDs;
- approval IDs;
- workflow IDs;
- run IDs;
- step IDs;
- skill IDs;
- spec hashes;
- target references;
- summaries;
- reasons;
- redaction metadata;
- store paths;
- record JSON;
- provider payloads;
- command output;
- parser payloads;
- snippets;
- credentials;
- tokens;
- authorization headers;
- private keys.

## 13. Privacy And Redaction

The composition helper must remain reference-only.

It may inspect:

- SideEffect IDs;
- approval references;
- authority decisions;
- approval decision kind;
- immutable run identity;
- optional step and skill scope;
- bounded record counts.

It must not copy:

- approval reasons;
- SideEffect summaries;
- SideEffect target references;
- raw provider payloads;
- raw CI logs;
- Jira or GitHub bodies;
- command output;
- spec contents;
- parser payloads;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- local paths.

`Debug` output for input and result should expose only booleans and bounded counts.

## 14. Runtime And State Semantics

The composition helper must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- create SideEffect records;
- update SideEffect records;
- repair corrupt records;
- transition SideEffect lifecycle state;
- create approval requests or decisions;
- emit audit events;
- emit observability events;
- generate WorkReports;
- write report artifacts;
- execute side effects;
- call adapters or providers;
- create files;
- expose CLI output;
- change workflow pass/fail behavior.

It should only read from a caller-supplied `SideEffectRecordStore` and validate against a borrowed run.

## 15. Test Plan

Future implementation tests should cover:

- explicit SideEffect ID mode succeeds for matching approval linkage;
- all-records-for-run mode succeeds for matching approval linkage;
- duplicate explicit IDs are de-duplicated or counted deterministically;
- missing explicit records fail closed when required;
- optional missing explicit records are counted without leaking IDs;
- store read failures map to stable non-leaking errors;
- corrupt stored records map to stable non-leaking errors;
- immutable identity mismatch fails without leaking workflow/run/spec values;
- granted approval linkage succeeds for `ApprovedByHuman`;
- denied approval linkage succeeds for `DeniedByApproval`;
- request-only linkage succeeds for `RequiresApproval`;
- wrong decision kind fails;
- missing decision fails when required;
- non-approval authority records do not require approval references;
- policy flags for permissive behavior are explicitly tested;
- input/result `Debug` output is redaction-safe;
- helper does not mutate workflow run, event history, SideEffect records, stores, reports, artifacts, or files;
- existing SideEffect, discovery, artifact, WorkReport, executor, runtime, adapter, EvidenceReference, hook, local-check, and docs tests continue to pass.

## 16. Proposed Implementation Sequence

Recommended small phases:

1. Implement explicit store-backed approval linkage helper.
2. Add focused tests for store-backed loading, missing/corrupt records, policy flags, and non-leakage.
3. Review the store-backed helper.
4. Plan report/artifact composition if there is a concrete caller.
5. Only after review, consider a combined artifact integrity plus approval linkage helper.
6. Keep runtime side-effect execution and write-capable adapter planning deferred until approval linkage composition is accepted.

## 17. Open Questions

- Should explicit ID mode require every supplied ID by default?
- Should all-records-for-run mode be the first implementation path, or should v1 require explicit IDs only?
- Should the store-backed helper return separate counts for approval-authority records and non-approval records?
- Should duplicate approval references remain a count only, or should duplicates become validation errors in high-assurance mode?
- Should `RequiresApproval` without an approval reference be accepted in any store-backed composition path?
- Should report artifact writes eventually require both referential integrity and approval linkage when SideEffect records cite approval authority?
- Should approval linkage composition eventually emit audit events, or remain a validation-only precondition?
- When should high-assurance approval controls add requester/approver separation and role/quorum policy?

## 18. Final Recommendation

Recommended next phase: **store-backed SideEffect approval linkage helper review**.

Do not integrate approval linkage into report generation, report artifact writing, executor execution, CLI, schemas, examples, runtime side-effect execution, or write-capable adapters before the store-backed helper is reviewed. After review, the project should move out of helper micro-phases and into a larger write-capable adapter readiness slice plan that composes the accepted governance primitives without enabling provider mutation prematurely.
