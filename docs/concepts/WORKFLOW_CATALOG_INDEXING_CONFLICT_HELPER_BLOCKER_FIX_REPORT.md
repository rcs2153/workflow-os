# Workflow Catalog Indexing Conflict Helper Blocker Fix Report

## 1. Executive Summary

The workflow catalog indexing/conflict helper serde validation blocker is fixed.

The helper still supports serde for the exported active workflow summary, draft
summary, archived draft summary, and conflict disclosure types, but
deserialization now delegates to the same validated constructors used by normal
runtime construction.

The phase remains a narrow blocker fix. It does not add command integration,
runtime workflow registration, catalog writes, promotion/archive wiring,
schemas, examples, provider calls, hosted behavior, writes, or release posture
changes.

## 2. Blocker Fixed

The review found that `WorkflowCatalogActiveWorkflowSummary`,
`WorkflowCatalogDraftSummary`, `WorkflowCatalogArchivedDraftSummary`, and
`WorkflowCatalogConflict` derived `Deserialize` while carrying private string
fields whose safety depended on constructors.

That meant serde could populate unsafe repository paths, secret-like draft
statuses, or unsafe conflict source references without calling validation.

The blocker is fixed by replacing derived deserialization for those exported
types with custom deserializers that:

- deserialize into private wire structs;
- call the existing validated constructors;
- return stable non-leaking validation errors on unsafe input.

## 3. Implementation Approach

The implementation keeps the public model shape stable and does not remove
serde support.

Custom deserialization was added for:

- `WorkflowCatalogActiveWorkflowSummary`;
- `WorkflowCatalogDraftSummary`;
- `WorkflowCatalogArchivedDraftSummary`;
- `WorkflowCatalogConflict`.

The constructors remain the validation boundary for:

- repository-relative paths;
- path traversal and absolute path rejection;
- bounded optional draft status;
- bounded conflict source references;
- secret-like path/status/reference rejection.

## 4. Validation Boundary Summary

Deserialized helper inputs now fail closed for:

- active workflow paths containing unsafe path components;
- archived draft paths that are absolute;
- draft statuses containing secret-like text;
- conflict source references containing secret-like text.

Validation errors use existing `workflow_catalog_index.*` codes and do not echo
raw invalid values.

## 5. Redaction And Privacy Summary

The fix does not introduce raw payload storage or output.

The helper still does not read or copy:

- raw workflow YAML;
- raw draft YAML;
- source contents;
- command output;
- provider payloads;
- parser payloads;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

Deserialization errors are non-leaking for the newly covered unsafe and
secret-like values.

## 6. Test Coverage Summary

Focused regression tests were added for:

- serialized active workflow summary with unsafe path fails closed;
- serialized draft summary with secret-like status fails closed;
- serialized archived draft summary with absolute archive path fails closed;
- serialized conflict disclosure with secret-like source reference fails
  closed;
- deserialization errors do not leak the supplied raw values.

Existing workflow catalog index tests continue to pass.

## 7. Commands Run And Results

Dogfood governance:

```text
workflow_id: dg/blocker
run_id: run-1783522745164461000-2
approval_id: approval/run-1783522745164461000-2/fix-approved
approval_outcome: granted
events_total: 39
event_summary: ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6
```

Commands run:

```text
cargo fmt --all
cargo test -p workflow-core --test workflow_catalog_index
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Results: all commands passed.

## 8. Remaining Known Limitations

- No command consumes the helper yet.
- No catalog health command exists.
- No authoring command writes catalog records.
- No strict catalog enforcement is wired to promotion or steward-review.
- Semantic workflow overlap detection remains deferred.

## 9. Recommended Next Phase

Recommended next phase: workflow catalog indexing/conflict helper blocker fix
review.

The blocker touched the public serde boundary of a governance helper. It should
be reviewed before command integration planning resumes.
