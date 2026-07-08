# Workflow Catalog Indexing Conflict Helper Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to workflow catalog command integration planning.

The blocker fix is narrow, deterministic, and reviewable. The exported catalog
index helper summary and conflict types still support serde, but deserialization
now delegates through the same validated constructors used by normal runtime
construction.

No remaining blocker was found.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

No accidental implementation was found for:

- catalog command integration;
- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- persisted approval consumption;
- promotion command catalog writes;
- archive command catalog writes;
- draft deletion;
- workflow schema changes;
- examples;
- provider calls;
- command execution or local check execution;
- hosted or distributed behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes;
- broad workflow catalog model redesign.

## 3. Original Blocker Restatement

The original review found that
`WorkflowCatalogActiveWorkflowSummary`, `WorkflowCatalogDraftSummary`,
`WorkflowCatalogArchivedDraftSummary`, and `WorkflowCatalogConflict` derived
`Deserialize` while storing private string fields whose safety depended on
constructors.

That meant serialized data could bypass:

- repository-relative path validation;
- path traversal and absolute path rejection;
- draft status bounds and secret-like rejection;
- conflict source reference bounds and secret-like rejection.

The risk was future command integration or fixtures accepting unsafe
deserialized helper values that looked equivalent to validated runtime values.

## 4. Fix Approach Assessment

The implemented approach is appropriate.

The four affected exported types no longer derive `Deserialize`. Each has a
custom `Deserialize` implementation that:

- deserializes into a private wire struct;
- calls the existing public or internal validated constructor;
- maps validation failure through serde errors without exposing the raw unsafe
  value.

This is smaller and more compatible than removing serde support outright. It
keeps the future serialized shape stable while preserving the constructor as the
single validation boundary.

## 5. Validation Boundary Assessment

Validation now applies consistently for constructor-created and deserialized
values.

Verified behavior:

- unsafe active workflow paths fail closed during deserialization;
- absolute archived draft paths fail closed during deserialization;
- secret-like draft statuses fail closed during deserialization;
- secret-like conflict source references fail closed during deserialization;
- errors retain stable `workflow_catalog_index.*` codes;
- errors do not echo the supplied unsafe or secret-like values.

The fix does not weaken existing constructor validation for normal helper input.

## 6. Serialization And Debug Assessment

Serialization support remains intentionally available for the helper boundary.
The important change is that the reverse path no longer silently accepts unsafe
payloads.

`WorkflowCatalogConflict` Debug still redacts `source_reference`. The review did
not find new Debug leakage introduced by the custom serde implementations.

`WorkflowCatalogIndex` remains serialize-only. That conservative posture is
acceptable for this phase.

## 7. Privacy And Redaction Assessment

The fix does not add raw payload storage or output.

No evidence was found that the helper reads, stores, serializes, or copies:

- raw workflow YAML;
- raw draft YAML;
- source contents;
- manifest bodies;
- package script bodies;
- dependency values;
- lockfile contents;
- CI logs;
- command output;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded reviewer reasons;
- existing agent instruction bodies.

Deserialization errors are bounded and do not leak the invalid field values
covered by the blocker tests.

## 8. Regression Assessment

The fix preserves:

- valid active workflow summary construction;
- valid draft summary construction;
- valid archived draft summary construction;
- valid conflict construction through the helper;
- deterministic index construction;
- existing conflict taxonomy and ordering;
- existing catalog record, stewardship record, and archive record behavior;
- existing workflow catalog store behavior.

No runtime behavior, command behavior, persistence behavior, or schema behavior
was changed.

## 9. Test Quality Assessment

The focused regression tests cover the original blocker directly:

- deserializing an active workflow summary with an unsafe path fails closed;
- deserializing a draft summary with a secret-like status fails closed;
- deserializing an archived draft summary with an absolute archive path fails
  closed;
- deserializing a conflict with a secret-like source reference fails closed;
- deserialization errors do not leak the supplied raw values.

Existing tests continue to cover:

- empty index construction;
- deterministic ordering;
- duplicate active workflow id and path blockers;
- missing catalog record warning/blocker posture;
- catalog active path/hash mismatch blockers;
- missing active file blocker;
- stale draft stewardship hash blocker;
- archive hash and missing archived draft blockers;
- missing owner/escalation/stewardship/side-effect warning disclosures;
- constructor-side unsafe and secret-like path rejection;
- conflict Debug non-leakage.

Non-blocking test gaps remain:

- conflict ordering tests across multiple severities and source categories;
- a no-raw-YAML serialization fixture once a future command consumes the helper.

These do not block the serde validation fix.

## 10. Documentation Review

The roadmap and catalog indexing plan now state that:

- the in-memory indexing/conflict helper is implemented;
- the original review found a serde validation-bypass blocker;
- the blocker fix is documented;
- command integration remains unimplemented;
- runtime workflow registration remains unimplemented;
- catalog command writes remain unimplemented;
- schemas, examples, providers, hosted behavior, writes, and release posture
  changes remain unimplemented.

The blocker-fix report accurately records the implementation approach,
validation boundary, tests, commands, remaining limitations, and recommended
review phase.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add conflict ordering tests that combine multiple severities and source
  categories.
- In the next command-integration plan, define warning enforcement posture per
  command: report-only, review-required, or strict blocker.
- Keep `WorkflowCatalogIndex` deserialize-free unless a concrete command or API
  needs to ingest full serialized indexes.

## 13. Recommended Next Phase

Recommended next phase: workflow catalog command integration planning.

The model/helper boundary is now reviewed and no blocker remains. The next
phase should decide the smallest command surface that can consume the index
helper without registering workflows, writing catalog records automatically,
changing schemas, enabling runtime state, adding examples, or changing release
posture.

## 14. Dogfood Governance

```text
workflow_id: dg/review
run_id: run-1783523990623138000-2
approval_id: approval/run-1783523990623138000-2/review-scope-approved
approval_outcome: granted
events_total: 39
event_summary: ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6
```

The review phase was approved before this artifact was written. The kernel
governed the phase boundary; code inspection, document authoring, validation
commands, git operations, and PR operations were performed outside the kernel
and are disclosed here.

## 15. Validation Commands Run

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Results: all commands passed.
