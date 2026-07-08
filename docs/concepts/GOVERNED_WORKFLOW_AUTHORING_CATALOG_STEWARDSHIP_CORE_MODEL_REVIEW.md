# Governed Workflow Authoring Catalog And Stewardship Core Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The workflow catalog and stewardship core model stays within the approved
model-only boundary. It adds bounded catalog, stewardship decision, and archive
metadata vocabulary with deterministic validation, validated serde, redaction-safe
`Debug`, focused tests, documentation updates, and a phase report. It does not
introduce catalog persistence, runtime workflow registration, automatic
promotion, schemas, examples, provider calls, writes, or release posture changes.

## 2. Scope Verification

The phase stayed within approved scope.

No accidental implementation was found for:

- catalog persistence;
- persisted approval or stewardship record storage;
- runtime workflow registration changes;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- draft deletion or abandon behavior;
- workflow schema changes;
- examples;
- provider calls;
- command execution;
- local check execution;
- hosted or distributed behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 3. Model Assessment

The model is appropriately domain-neutral and minimal for the first catalog
slice.

Implemented:

- `WorkflowCatalogRecordId`;
- `WorkflowStewardshipDecisionId`;
- `WorkflowArchiveRecordId`;
- `WorkflowLifecycleStatus`;
- `WorkflowStewardshipDecisionKind`;
- `WorkflowCatalogRecordDefinition`;
- `WorkflowCatalogRecord`;
- `WorkflowStewardshipRecordDefinition`;
- `WorkflowStewardshipRecord`;
- `WorkflowArchiveRecordDefinition`;
- `WorkflowArchiveRecord`.

The implementation covers workflow identity, lifecycle, content hash, schema
version, repository-relative paths, owner/escalation actors, posture summaries,
stewardship references, archive references, timestamps, sensitivity, and
redaction metadata. It intentionally does not add separate catalog version,
owner, lineage, source, conflict, promotion, or actor wrapper types yet. That is
acceptable for this model-only slice because the implemented records already
carry the needed first-order fields without creating unused abstractions.

## 4. Catalog Record Assessment

`WorkflowCatalogRecord` captures the essential future catalog shape:

- catalog record id;
- workflow id;
- workflow path;
- workflow content hash;
- schema version;
- lifecycle status;
- source recommendation id;
- source draft path;
- archived draft path;
- owner and escalation contact;
- authority, evidence/check/report, and side-effect posture summaries;
- latest stewardship, promotion, and archive references;
- created and updated timestamps;
- sensitivity and redaction metadata.

The record remains reference-oriented and does not copy workflow YAML, source
contents, package scripts, command output, provider payloads, or existing agent
instruction bodies.

## 5. Stewardship Record Assessment

`WorkflowStewardshipRecord` captures bounded decision metadata for the planned
review/promotion/archive lifecycle:

- stewardship decision id;
- decision kind;
- workflow id;
- draft, active workflow, and archive paths where relevant;
- candidate and active content hashes;
- reviewer actor;
- decision timestamp;
- bounded reason summary;
- preflight and steward-review references;
- evidence, approval, policy, validation, and work-report references;
- known limitations and strict non-goals;
- sensitivity and redaction metadata.

The decision vocabulary covers draft creation, review request, approval for
promotion, rejection, needs-changes, promotion, archive, and supersession. This
is enough for the future catalog store and promotion integration to cite
stewardship decisions without implying that those integrations already exist.

## 6. Archive Record Assessment

`WorkflowArchiveRecord` captures the first durable archive metadata shape:

- archive record id;
- original draft path;
- archive path;
- workflow id;
- draft content hash;
- active workflow path and hash when available;
- prior draft status;
- archive actor;
- bounded archive reason summary;
- archive timestamp;
- validation reference;
- stewardship decision reference;
- sensitivity and redaction metadata.

This is appropriate as metadata vocabulary only. The implementation does not
wire archive metadata into `archive-draft` or any store.

## 7. Validation Assessment

Validation is deterministic and fail-closed.

Verified:

- identifiers reject empty, too-long, invalid-character, and secret-like values;
- repository paths reject empty, too-long, absolute, traversal, prefix/root, and
  secret-like values;
- source recommendation and status/reference text is bounded;
- reason, posture, limitation, and non-goal text is bounded and rejects
  secret-like values;
- reference vectors are bounded;
- redaction metadata field names and reasons are bounded and reject secret-like
  values;
- invalid serialized records deserialize through constructors and fail closed;
- validation errors use stable `workflow_catalog.*` codes;
- error messages do not echo raw secret-like values.

Non-blocking note: the planning document listed more granular future error-code
examples than the shared validation codes implemented here. The implemented
codes are stable and non-leaking, so this is not a blocker, but storage/schema
planning should decide whether to keep the shared codes or introduce more
specific public codes before exposing the model as durable contract.

## 8. Privacy And Redaction Assessment

Privacy posture is acceptable for the phase.

The model does not store:

- raw workflow YAML;
- raw draft YAML;
- source contents;
- manifest bodies;
- package scripts;
- dependency values;
- lockfile contents;
- CI logs;
- command output;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment variable values;
- credentials, authorization headers, private keys, token-like strings;
- existing agent instruction bodies.

`Debug` redacts identifier internals for new catalog ids, redacts reason and
posture summaries, reports reference vectors as counts, and reports redaction
metadata as counts. Serialization can contain valid bounded summaries because
these are model fields, but secret-like summaries and redaction metadata fail
validation during construction and deserialization.

## 9. Serde And Compatibility Assessment

Serde support is suitable for model-only use.

Valid records serialize and deserialize. Invalid serialized catalog and
stewardship records fail closed through manually validated deserialization of
the main record types. Field names are stable and clear enough for future schema
planning.

No workflow spec schema changes were introduced, and no loader/runtime behavior
depends on the new model.

## 10. Relationship To Existing Authoring Runtime

The model aligns with current workflow authoring boundaries:

- active workflow files remain the loader-visible execution source of truth;
- inactive drafts remain file-based proposals;
- archive command behavior is unchanged;
- promotion behavior is unchanged;
- preflight and steward-review behavior is unchanged;
- catalog records are not written, read, or enforced yet.

This is the right order: model vocabulary first, then reviewed persistence and
integration planning.

## 11. Test Quality Assessment

Focused tests cover:

- valid catalog record identity and validation;
- invalid catalog id rejection without leakage;
- unsafe absolute and traversal path rejection;
- lifecycle status vocabulary;
- valid stewardship record references;
- secret-like stewardship reason rejection;
- valid archive metadata record;
- serde round trip;
- invalid serialized path failure;
- debug non-leakage;
- invalid serialized secret-like reason rejection;
- redaction metadata validation and non-leaking errors.

The tests are meaningful and cover the highest-risk boundaries for a first model
slice. They also run alongside existing WorkReport, EvidenceReference,
Diagnostic, validation, adapter telemetry, side-effect, provider-write, runtime,
and authoring tests through `cargo test --workspace`.

Non-blocking test follow-ups:

- add explicit overlong-field and too-many-reference tests before persistence;
- add archive-record serde failure tests before archive metadata storage;
- add tests for timestamp ordering if future catalog semantics require
  `updated_at >= created_at`.

## 12. Documentation Review

Docs accurately state that:

- workflow catalog and stewardship core model is implemented;
- catalog persistence is not implemented;
- persisted stewardship decisions are not implemented;
- runtime workflow registration changes are not implemented;
- automatic workflow generation, promotion, and archive cleanup are not
  implemented;
- workflow schema changes are not implemented;
- examples are not updated;
- provider calls, command execution, local check execution, hosted behavior,
  writes, and release posture changes are not implemented.

The implementation report includes the governed dogfood run id, approval id,
approval outcome, event summary, validation summary, and out-of-kernel work
disclosure.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Decide whether future persisted catalog/schema exposure needs more granular
  validation error codes than the current shared `workflow_catalog.*` codes.
- Add explicit overlong-field, too-many-reference, and archive serde failure
  tests before introducing storage.
- Decide whether future catalog validation should enforce timestamp ordering.
- Plan catalog persistence and promotion/archive integration as a separate
  reviewed phase.

## 15. Recommended Next Phase

Recommended next phase: workflow catalog persistence planning.

The core model is accepted, but the next risky step is durable catalog behavior:
where records live, when authoring commands write them, how they relate to Git
files, how promotion/archive commands consume stewardship decisions, and how
conflicts fail closed without turning loader-visible workflow files into an
opaque database.

## 16. Validation

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `npm run dogfood:benchmark -- phase-close run-1783487144235553000-2 --phase review`
  - Passed.
  - Workflow: `dg/review`.
  - Status: `Completed`.
  - Events: `39`.
  - Event summary:
    `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`.

Out-of-kernel work disclosed: review document creation, shell validation
commands, and git/PR packaging are performed by Codex as executor. The kernel
coordinated the review phase boundary and approval checkpoint only.
