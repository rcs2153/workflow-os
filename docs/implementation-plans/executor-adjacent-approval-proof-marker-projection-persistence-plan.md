# Executor-Adjacent Approval Proof-Marker Projection Persistence Plan

Status: implemented.

Implementation report: [Executor-Adjacent Approval Proof-Marker Projection Persistence Helper Report](../concepts/EXECUTOR_ADJACENT_APPROVAL_PROOF_MARKER_PROJECTION_PERSISTENCE_HELPER_REPORT.md).

The implemented helper remains explicit, local, and executor-adjacent. It persists bounded projection records from supplied `WorkflowRun` approval decision events into a caller-supplied local projection store. Default executor behavior, automatic projection persistence, report artifact writing, CLI behavior, schema fields, examples, provider writes, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

This plan follows the accepted [Workflow-Declared Proof-Marker Artifact Executor Integration Review](../concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_EXECUTOR_INTEGRATION_REVIEW.md). That review accepted explicit executor artifact-path enforcement for workflow-declared approval proof-marker artifact requirements and identified caller-supplied projection records as the next runtime-composition gap.

## 1. Executive Summary

Workflow OS can now enforce workflow-declared approval proof-marker artifact requirements in the explicit proof-marker-capable artifact executor path. That path still depends on a caller-supplied local approval proof-marker projection store.

The next implementation question is how a future executor-adjacent path should persist bounded approval proof-marker projection records from already proof-enforced approval decision events so report artifact gates can validate durable projection posture without callers hand-populating the store.

This plan does not implement persistence. It does not change default executor behavior, make artifact writing automatic, add CLI behavior, add schemas, add examples, call providers, execute writes, broaden approval behavior, or change release posture.

## 2. Goals

- Define an explicit executor-adjacent persistence boundary for approval proof-marker projection records.
- Persist bounded projection records only from already proof-enforced approval decision events.
- Preserve approval decision workflow events as source of truth.
- Avoid hand-populated projection stores for explicit artifact paths when reviewed executor context already contains proof markers.
- Keep projection persistence opt-in, local, and caller-owned.
- Preserve workflow pass/fail semantics.
- Preserve report artifact failure semantics.
- Keep errors stable and non-leaking.
- Prepare a small implementation prompt for a helper/API that can be reviewed independently.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- default executor projection persistence;
- automatic projection persistence for all approvals;
- automatic report generation;
- automatic report artifact writing;
- automatic artifact retry or repair;
- CLI rendering or commands;
- schema changes;
- examples;
- workflow-declared projection-store configuration;
- runtime config;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy changes;
- release posture changes.

## 4. Current Baseline

Implemented foundation:

- approval-presentation proof records for governed approvals;
- approval decision proof markers in the opt-in proof-enforced approval path;
- bounded inspect/projection output for approval proof markers;
- pure WorkReport proof-marker citation derivation;
- terminal report opt-in proof-marker citation integration;
- executor report-input propagation for proof-marker citation policy;
- in-memory approval proof-marker audit projection helper;
- explicit durable local approval proof-marker projection store helper;
- in-memory and store-backed report artifact proof-marker gates;
- helper-level artifact write composition with the store-backed gate;
- explicit executor artifact path proof-marker gate integration;
- workflow-declared proof-marker artifact requirement vocabulary, derivation, and explicit artifact-path integration.

Still not implemented:

- executor-adjacent projection persistence from proof-enforced approval events;
- default executor projection persistence;
- automatic artifact writing;
- CLI artifact behavior;
- workflow-declared projection-store configuration.

## 5. Problem Statement

The explicit proof-marker artifact executor path can enforce this authored requirement:

```yaml
report_artifact_requirements:
  approval_proof_markers: marker_required
```

However, it currently validates against a caller-supplied projection store. That is safe, but operationally awkward: the approval decision event already carries proof-marker posture when the approval used the proof-enforced path, yet the artifact path still needs a separately populated projection record.

The next slice should close that gap without changing defaults. A caller should be able to opt into an executor-adjacent helper that:

1. inspects the completed run's approval decision events;
2. derives bounded projection records for approval decisions that carry proof-marker posture;
3. persists those records to an explicit local projection store;
4. then allows existing proof-marker artifact gates to validate against that store.

## 6. Recommended First Boundary

The first implementation should be a standalone executor-adjacent helper, not a change to `LocalExecutor::execute(...)`.

Recommended shape:

```text
persist_approval_proof_marker_projections_for_run(...)
```

or an equivalent narrow helper that accepts explicit inputs:

- `WorkflowRun`;
- caller-supplied `LocalApprovalProofMarkerAuditProjectionStore`;
- projection policy;
- optional selected approval references;
- sensitivity and redaction posture.

The helper should return an in-memory result describing persisted, skipped, duplicate, and failed records.

It must not execute workflows, approve requests, generate reports, write report artifacts, append workflow events, call providers, or create hidden stores.

## 7. Source Of Truth Rules

Projection persistence must derive from approval decision workflow events only.

Allowed source events:

- `ApprovalGranted` events that include accepted approval decision proof markers;
- `ApprovalDenied` events that include accepted approval decision proof markers, if denied approvals are in scope for the caller's projection policy.

Not allowed as source of truth:

- approval-presentation records alone;
- approval reasons;
- approval handoff text;
- WorkReport prose;
- CLI output;
- chat transcripts;
- local state files edited by hand;
- provider payloads;
- inferred approval IDs;
- fabricated proof markers.

The rule is:

```text
Approval presentation proves context existed; approval decision proof marker proves the decision used that context.
```

## 8. Persistence Policy

The first implementation should support a small explicit policy:

- include granted decisions only;
- include granted and denied decisions;
- require every selected approval decision to have a proof marker;
- allow marker-free decisions to be skipped with explicit posture;
- duplicate record behavior: fail, skip existing matching record, or report already-present posture.

Recommended v1 default for the helper:

- include granted and denied decisions that carry proof markers;
- skip marker-free decisions unless the caller requires all selected approvals to be projected;
- treat duplicate identical records as already present;
- treat duplicate conflicting records as fail-closed;
- return a bounded summary rather than mutating workflow status.

## 9. Store Boundary

The helper must receive an explicit `LocalApprovalProofMarkerAuditProjectionStore`.

Rules:

- do not discover projection stores from runtime state;
- do not create hidden store roots;
- do not write outside the caller-supplied store;
- do not infer projection-store paths from workflow specs;
- do not add workflow schema fields;
- do not persist projection records to `StateBackend` unless separately planned;
- do not use report artifact stores as projection stores.

The accepted store helper should remain the only persistence writer in v1.

## 10. Artifact Path Relationship

Projection persistence and artifact writing must remain separate steps.

The future helper may be composed by an explicit artifact-capable caller like:

1. execute or rehydrate a terminal run;
2. persist bounded approval proof-marker projections from the run;
3. generate a WorkReport with approval citations;
4. validate report artifact proof-marker gates against the projection store;
5. write the report artifact only if all requested gates pass.

This plan does not make that composition automatic. It plans the projection persistence piece so later executor artifact paths can compose it explicitly.

## 11. Workflow Semantics And Failure Behavior

Projection persistence failure must not change workflow execution status.

Rules:

- execution failure before a run exists still returns execution `Err`;
- projection failure after a run exists returns a structured projection error/result;
- projection failure must not append workflow events;
- projection failure must not mutate `WorkflowRun` or `WorkflowRunSnapshot`;
- projection failure must not write report artifacts;
- projection failure must not repair missing approval proof;
- partial persistence must be avoided where feasible;
- if partial persistence is unavoidable due filesystem failure, the result must disclose bounded partial-write posture.

Recommended v1 behavior: per-record `create_new` writes with explicit result entries, no overwrite, no deletion, no repair.

## 12. Privacy And Redaction

The helper must persist bounded posture only.

Do not store or copy:

- approval-presentation payloads;
- approval handoff text;
- work summaries, scopes, non-goals, validation expectations, or why-now text;
- approval reasons;
- raw presentation IDs or content hashes if the accepted store model represents only presence booleans;
- report text;
- command output;
- provider payloads;
- CI logs;
- GitHub or Jira bodies;
- raw source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Errors and debug output must not leak approval IDs, presentation IDs, content hashes, local paths, report text, command output, provider payloads, or secret-like metadata.

## 13. Validation And Error Codes

Future implementation should use stable error codes under a bounded namespace such as:

- `approval_proof_marker_projection_persistence.no_approval_events`;
- `approval_proof_marker_projection_persistence.marker_missing`;
- `approval_proof_marker_projection_persistence.duplicate_conflict`;
- `approval_proof_marker_projection_persistence.store_write_failed`;
- `approval_proof_marker_projection_persistence.invalid_projection_record`;
- `approval_proof_marker_projection_persistence.unsupported_decision_event`;

Errors should describe posture, not raw values.

## 14. Test Plan

Future implementation tests should cover:

- proof-enforced granted approval persists one bounded projection record;
- proof-enforced denied approval persists when policy includes denied decisions;
- marker-free approval is skipped when policy allows skipping;
- marker-free approval fails when policy requires selected approvals to project;
- duplicate matching record reports already-present posture;
- duplicate conflicting record fails closed;
- projection preserves workflow ID, workflow version, schema version, spec hash, run ID, event ID, approval reference, decision kind, sensitivity, and redaction posture;
- projection does not persist presentation IDs, content hashes, handoff text, approval reasons, command output, provider payloads, source/spec contents, or secrets;
- projection failure does not mutate run status, snapshot, or event history;
- helper writes no report artifacts;
- helper appends no workflow events;
- helper does not require a live provider, CLI, runtime config, or workflow schema field;
- existing proof-marker, approval, WorkReport, artifact, executor, and docs tests still pass.

## 15. Proposed Implementation Sequence

1. Add a narrow input/result model for executor-adjacent approval proof-marker projection persistence.
2. Implement a pure-ish helper that scans a supplied `WorkflowRun` event history and writes bounded projection records through the accepted local projection store.
3. Add focused tests for proof-enforced granted/denied decisions, marker-free behavior, duplicates, non-leakage, and no workflow mutation.
4. Review the helper.
5. Only after review, plan composition into explicit report artifact executor paths.

The first implementation should not change existing executor methods.

## 16. Deferred Work

Deferred:

- default executor projection persistence;
- automatic projection persistence for all approval decisions;
- automatic report artifact writing;
- artifact path composition that both persists projections and writes artifacts in one API;
- approval-resume artifact paths;
- CLI rendering or commands;
- workflow schema fields for projection stores;
- examples;
- provider writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 17. Final Recommendation

Proceed next to implementation of a narrow executor-adjacent projection persistence helper.

The helper should be explicit, local, store-backed, and run-derived. It should persist only bounded approval proof-marker posture from proof-enforced approval decision events and should not change default executor behavior, write report artifacts, append workflow events, add CLI behavior, add schemas, call providers, execute writes, or alter release posture.

## 18. Governed Planning Record

- Dogfood workflow: `dg/d`.
- Run ID: `run-1783683227010483000-2`.
- Approval ID: `approval/run-1783683227010483000-2/planning-approved`.
- Approval presentation ID: `presentation/e145383c33de72c7`.
- Approval presentation hash: `e145383c33de72c7cf93ba1138850c3e42aec1377efe320606e3a5e352f221c3`.
- Approval outcome: granted by delegated maintainer for bounded planning scope.
- Validation: `npm run check:docs` passed.
- Phase-close status: `Completed`.
- Phase-close event summary: 39 total events, including one approval request, one approval grant, eight policy decisions, six step schedules, six skill invocation requests, six skill invocation starts, six skill invocation successes, one run resume, and one run completion.
- Approval-presentation enforcement: `proof_enforced`.
