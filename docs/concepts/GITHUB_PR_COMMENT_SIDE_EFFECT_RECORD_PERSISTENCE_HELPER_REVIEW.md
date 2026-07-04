# GitHub PR Comment Proposed SideEffectRecord Persistence Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The phase correctly adds an explicit store-backed persistence helper for GitHub PR comment proposed `SideEffectRecord` values. It stays inside the no-provider-call, no-runtime-execution, no-event/audit/report-artifact boundary and is suitable to proceed to the next scoped planning/review phase.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- explicit helper for composing and persisting a GitHub PR comment proposed `SideEffectRecord`;
- persistence only through caller-supplied `SideEffectRecordStore`;
- lifecycle limited to `Proposed`;
- stable non-leaking store error mapping;
- focused tests;
- roadmap, integration, plan, and phase report documentation.

No accidental implementation found for:

- GitHub provider calls;
- GitHub PR comment mutation;
- live sandbox write;
- runtime side-effect execution;
- lifecycle transition beyond `Proposed`;
- workflow event append;
- audit event emission;
- report artifact writes;
- automatic executor integration;
- CLI behavior;
- workflow schemas;
- examples;
- hosted or distributed behavior;
- reasoning lineage;
- autonomy expansion;
- release posture changes.

## 3. Helper API Assessment

The helper API is narrow and appropriate:

```text
compose_and_persist_github_pr_comment_proposed_side_effect_record(
    store: &impl SideEffectRecordStore,
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture_response: Option<&GitHubPullRequestCommentWriteResponse>,
    input: GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<SideEffectRecord, WorkflowOsError>
```

The helper accepts an explicit store rather than constructing or assuming a backend. It reuses the existing composition helper and returns the validated persisted record in memory. It does not introduce executor coupling, runtime config, CLI output, or provider dependencies.

## 4. Persistence Boundary Assessment

The ordering is correct:

1. Preflighted write and optional fixture/dry-run response posture are validated by the existing composition path.
2. The proposed record is composed in memory.
3. The helper defensively checks the lifecycle is `Proposed`.
4. The record is written through `SideEffectRecordStore`.
5. The same record is returned.

The helper does not bypass the store with direct filesystem writes and does not create a second persistence mechanism.

## 5. Lifecycle Assessment

The implementation only persists `SideEffectLifecycleState::Proposed`.

No support was added for:

- `Attempted`;
- `Completed`;
- `Failed`;
- `Denied`;
- `Skipped`.

This is the correct conservative boundary. Denied/skipped/attempted/completed/failed records need separate authority, audit, idempotency, and replay semantics.

## 6. Error Handling Assessment

The helper maps store failures to stable, non-leaking GitHub-specific persistence codes:

- `github_pr_comment_side_effect_record.persistence.duplicate`;
- `github_pr_comment_side_effect_record.persistence.identity_mismatch`;
- `github_pr_comment_side_effect_record.persistence.store_failed`;
- `github_pr_comment_side_effect_record.persistence.unsupported_lifecycle`.

The mapper does not forward raw store messages, SideEffect IDs, run IDs, target references, summaries, provider references, or secret-like values. Composition failures still fail before persistence.

No blocker found. One non-blocking test follow-up is to add a direct helper-level regression for identity-mismatch mapping, even though the underlying store contract already covers identity mismatch.

## 7. Privacy And Redaction Assessment

The phase preserves the existing privacy posture.

The helper does not persist:

- raw GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw PR bodies;
- raw diffs;
- raw CI logs;
- raw command output;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded prompts;
- secret-like values.

Provider-success/provider-failure response outcomes are rejected before persistence in this proposed-record path. Secret-like summaries are rejected before store write. Debug/error behavior remains bounded and redaction-safe.

## 8. Relationship To Workflow Events, Audit, And Reports

The phase correctly keeps persistence separate from runtime history.

It does not append `SideEffectProposed` workflow events, emit audit events, mutate reports, or write report artifacts. Later phases can decide whether persisted proposed records are prerequisites for event/audit projection or report artifact references.

## 9. Relationship To Write Adapter Readiness

This is a useful write-readiness increment because future provider writes need durable proposed write intent before mutation. The phase still does not authorize provider mutation. The next write-candidate work should remain fixture-first and should not skip to live sandbox behavior.

## 10. Test Quality Assessment

Tests cover:

- fixture-backed proposed record persistence;
- dry-run proposed record persistence;
- persisted record read-back;
- run-level list behavior;
- exact `Proposed` lifecycle;
- duplicate SideEffect ID rejection without leakage;
- provider response rejection before store write;
- secret-like summary rejection before store write;
- existing provider-write request, fixture, response, and composition behavior.

Existing store contract tests cover broader duplicate and immutable identity mismatch behavior.

Missing or shallow coverage:

- direct helper-level identity-mismatch mapping test is missing. This is non-blocking because the store contract already tests identity mismatch and the helper mapper has a simple explicit branch, but it would improve regression confidence.
- the review did not require a fake failing store test for generic `store_failed`; current behavior is simple and non-leaking, but direct coverage would be useful if store implementations expand.

## 11. Documentation Review

Documentation now states:

- proposed record persistence is implemented only through an explicit store-backed helper;
- automatic persistence from GitHub write candidates is not implemented;
- provider mutation is not implemented;
- runtime side-effect execution is not implemented;
- workflow events and audit events are not emitted by this helper;
- report artifacts are not written by this helper;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, and release posture changes remain unimplemented.

The docs preserve the distinction between durable proposed write intent and actual provider execution.

## 12. Validation Review

The implementation report records successful validation:

- `cargo fmt --all`;
- `cargo test -p workflow-core --test provider_write`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

This review reran the required validation commands after creating the review document.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Add a direct helper-level test for identity-mismatch error mapping if a future persistence hardening phase touches this area.
- Add a direct helper-level generic store-failure mapping test if additional `SideEffectRecordStore` implementations are introduced.
- Decide in a later plan whether duplicate proposed-record persistence should remain first-write-wins or become idempotent read-existing behavior for replay.

## 15. Recommended Next Phase

Recommended next phase: workflow event/audit projection planning for persisted proposed GitHub PR comment side-effect records.

Rationale: the proposed record can now become durable without provider mutation. The next useful boundary is deciding when and how a persisted proposed record should be referenced by workflow events, audit projection, and later report artifacts, while still avoiding live GitHub writes.
