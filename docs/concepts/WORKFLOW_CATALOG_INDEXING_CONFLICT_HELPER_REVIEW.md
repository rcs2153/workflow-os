# Workflow Catalog Indexing Conflict Helper Review

## 1. Executive Verdict

Needs blocker fixes.

The pure in-memory helper is correctly scoped, deterministic, and useful as the
next catalog governance primitive. It should not proceed to command integration
until the serde validation bypass described below is fixed.

Fix-forward note: the blocker is fixed in
[Workflow Catalog Indexing Conflict Helper Blocker Fix Report](WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_BLOCKER_FIX_REPORT.md).
This review preserves the original finding for auditability.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

No accidental implementation was found for:

- command integration;
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
- release posture changes.

## 3. Helper API Assessment

The implemented API is appropriately small:

```text
build_workflow_catalog_index(input: WorkflowCatalogIndexInput) -> Result<WorkflowCatalogIndex, WorkflowOsError>
```

The helper accepts explicit active workflow, draft, archived draft, catalog,
stewardship, and archive inputs. It does not read project files, discover store
roots, mutate records, register workflows, promote drafts, archive drafts, or
write derived index files.

The exported summary types and read-only accessors make the helper usable by
future command integration without tying it to a local store implementation.

## 4. Conflict Taxonomy Assessment

The first conflict taxonomy is appropriately bounded and deterministic.

Blocker conflicts cover exact identity, path, hash, lifecycle, and archive
record mismatches:

- duplicate active workflow id;
- duplicate active workflow path;
- strict active workflow missing catalog coverage;
- active catalog record missing workflow file;
- catalog active path mismatch;
- catalog active content hash mismatch;
- draft stewardship hash mismatch;
- archive record missing archived draft;
- archive path mismatch;
- archive hash mismatch.

Warning conflicts cover missing governance metadata:

- active workflow missing catalog record when strict coverage is not requested;
- missing owner;
- missing escalation contact;
- missing latest stewardship decision;
- missing side-effect posture.

The helper does not use model-generated semantic similarity or infer business
process overlap. That is the right boundary for this phase.

## 5. Determinism Assessment

The helper sorts active workflows, drafts, archived drafts, catalog records,
stewardship records, archive records, and conflicts before returning the index.

Ordering does not depend on caller vector order, filesystem traversal order, or
hash-map iteration. Conflict ordering is stable by severity, kind, workflow id,
source, and source reference.

## 6. Validation Assessment

Constructor validation is mostly strong:

- active workflow paths, draft paths, original draft paths, and archive paths
  must be non-empty, bounded, repository-relative, and free of traversal,
  absolute path, and secret-like substrings;
- optional draft status is bounded and checked for secret-like substrings;
- conflict source references are bounded and checked for secret-like
  substrings;
- invalid helper inputs return stable `workflow_catalog_index.*` validation
  codes without echoing raw unsafe values.

However, the public serde boundary is not safe enough yet.

`WorkflowCatalogActiveWorkflowSummary`, `WorkflowCatalogDraftSummary`,
`WorkflowCatalogArchivedDraftSummary`, and `WorkflowCatalogConflict` derive
`Deserialize` while carrying private string fields whose safety depends on
their constructors. Serde can populate those fields without calling the
constructors, which can silently bypass path and reference validation for
deserialized helper inputs or conflict disclosures.

That is a blocker because future command integration or test fixtures could
deserialize unsafe paths, secret-like values, or unbounded references into
validated-looking helper types.

## 7. Privacy And Redaction Assessment

The implementation does not read or copy raw workflow YAML, source contents,
command output, provider payloads, parser payloads, environment values, or
existing agent instruction bodies.

`WorkflowCatalogConflict` Debug redacts `source_reference`, and
`WorkflowCatalogIndex` Debug reports counts rather than embedded records or
free-text posture fields.

The privacy posture is acceptable at constructor-created runtime boundaries.
The serde bypass noted above weakens the serialized/deserialized boundary and
must be fixed before command integration.

## 8. Store And Authoring Boundary Assessment

The helper consumes already-loaded `WorkflowCatalogRecord`,
`WorkflowStewardshipRecord`, and `WorkflowArchiveRecord` values and does not own
storage.

It remains compatible with:

- local catalog store integration;
- future `workflow-os catalog status`;
- future preflight and steward-review disclosure integration;
- future strict promotion enforcement.

It does not prematurely decide command outcomes. It returns conflicts and lets
future command integration choose enforcement policy.

## 9. Test Quality Assessment

Focused tests cover:

- valid empty index construction;
- deterministic active workflow ordering;
- draft, archived draft, catalog, stewardship, and archive inputs;
- duplicate active workflow id blocker;
- duplicate active workflow path blocker;
- missing catalog warning by default;
- strict missing catalog blocker;
- catalog active path mismatch blocker;
- catalog active content hash mismatch blocker;
- catalog active missing workflow file blocker;
- stale draft stewardship hash blocker;
- archive hash mismatch blocker;
- archive record missing archived draft blocker;
- missing owner, escalation, stewardship, and side-effect posture warnings;
- unsafe and secret-like constructor input rejection;
- conflict Debug non-leakage.

Missing blocker coverage:

- deserializing `WorkflowCatalogActiveWorkflowSummary` with an unsafe path must
  fail closed or be impossible;
- deserializing `WorkflowCatalogDraftSummary` with a secret-like draft status or
  path must fail closed or be impossible;
- deserializing `WorkflowCatalogArchivedDraftSummary` with an unsafe archive
  path must fail closed or be impossible;
- deserializing `WorkflowCatalogConflict` with a secret-like or overlong source
  reference must fail closed or be impossible.

Non-blocking test follow-up:

- add explicit conflict ordering tests that include multiple severities and
  multiple sources;
- add a no-raw-YAML serialization fixture test once serde policy is fixed.

## 10. Documentation Review

The roadmap, implementation plan, persistence plan, catalog/stewardship plan,
and phase report accurately state that:

- the in-memory indexing/conflict helper is implemented;
- command integration is not implemented;
- runtime workflow registration is not implemented;
- authoring commands do not automatically write catalog files;
- schemas, examples, providers, hosted behavior, writes, and release posture
  changes remain unimplemented.

The report honestly records dogfood governance, validation commands, remaining
limitations, and the recommended review phase.

## 11. Blockers

1. Serde deserialization can bypass constructor validation for exported catalog
   index summary and conflict types.

   Required fix: remove `Deserialize` from these types unless needed now, or
   implement custom deserialization that delegates to the validated constructors
   and returns stable non-leaking validation errors.

   Required tests: add deserialization regression tests for unsafe paths,
   secret-like paths/statuses, and unsafe conflict source references.

## 12. Non-Blocking Follow-Ups

- Add broader conflict ordering tests across severities and sources.
- Decide whether `WorkflowCatalogIndex` itself should ever deserialize; the
  current serialize-only posture is conservative and acceptable.
- In the future command-integration plan, define whether warnings become
  report-only, review-required, or strict blockers per command.

## 13. Recommended Next Phase

Recommended next phase: workflow catalog indexing/conflict helper blocker fix.

The fix should stay narrow: harden or remove serde deserialization for the
helper boundary, add focused regression tests, update the report/review with a
fix-forward note, and rerun the standard validation suite. Command integration,
promotion enforcement, archive writes, schemas, examples, hosted behavior,
provider calls, writes, and release posture changes should remain out of scope.

## 14. Dogfood Governance

```text
workflow_id: dg/review
run_id: run-1783521711283809000-2
approval_id: approval/run-1783521711283809000-2/review-scope-approved
approval_outcome: granted
events_total: 39
event_summary: ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6
```

The review phase was approved before this review artifact was written. The
kernel governed the phase boundary; code inspection, document authoring, and
validation commands were performed outside the kernel and are disclosed here.

## 15. Validation Commands Run

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Results: all commands passed.
