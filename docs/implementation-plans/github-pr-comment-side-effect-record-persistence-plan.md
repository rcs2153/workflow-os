# GitHub PR Comment Proposed SideEffectRecord Persistence Plan

Status: Implemented as an explicit store-backed helper. This plan follows the accepted GitHub PR comment proposed `SideEffectRecord` composition helper review. It defines the no-provider-call boundary for explicitly persisting a validated proposed `SideEffectRecord` through an existing `SideEffectRecordStore` before any live sandbox GitHub write planning. The implementation is documented in [GitHub PR Comment Proposed SideEffectRecord Persistence Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_PERSISTENCE_HELPER_REPORT.md). Workflow event/audit projection planning for persisted proposed records is documented in [GitHub PR Comment Proposed SideEffect Event/Audit Projection Plan](github-pr-comment-side-effect-event-audit-projection-plan.md). It does not implement provider calls, GitHub mutation, runtime side-effect execution, lifecycle transitions beyond `Proposed`, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.

## 1. Executive Summary

Workflow OS now has an in-memory helper that composes a validated proposed `SideEffectRecord` for a GitHub pull request comment write candidate from an already preflighted request. The next question is how that proposed record becomes durable without turning persistence into execution.

The correct next boundary is an explicit store-backed helper that writes only a validated `Proposed` record through `SideEffectRecordStore`. Persistence should make the governed write intent durable, but it must not call GitHub, attempt a write, append workflow events, emit audit events, create report artifacts, or expose CLI behavior.

This plan has been implemented as a small helper phase. It remains a local explicit store boundary and does not authorize runtime execution or provider mutation.

## 2. Goals

- Persist validated proposed GitHub PR comment `SideEffectRecord` values explicitly.
- Reuse the existing `SideEffectRecordStore` contract.
- Preserve composition-before-persistence.
- Preserve preflight-before-composition.
- Preserve duplicate SideEffect ID protection.
- Preserve immutable workflow/run/spec identity checks.
- Preserve redaction-safe, non-leaking errors.
- Keep persistence separate from provider mutation.
- Prepare future event/audit/report phases to cite durable proposed records.
- Keep the first implementation local, explicit, and testable.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- GitHub provider calls;
- pull request comment creation;
- live sandbox writes;
- provider auth handling;
- provider mutation;
- runtime side-effect execution;
- attempted, completed, denied, skipped, or failed SideEffect lifecycle transitions for this path;
- workflow event appends;
- audit event emission;
- report artifact writes;
- automatic executor integration;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented baseline:

- adapter-neutral write preflight helper;
- model-only GitHub pull request comment request/response boundary;
- preflighted GitHub PR comment write helper;
- fixture-only GitHub PR comment validation helper;
- in-memory proposed `SideEffectRecord` composition helper;
- `SideEffectRecordStore` trait;
- local and in-memory store implementations;
- store contract tests for write/read/list, duplicate IDs, and immutable run identity mismatch;
- SideEffect discovery, WorkReport citation, artifact integrity, and approval-linkage helpers.

Not implemented for this path:

- automatic proposed record persistence from GitHub write candidates;
- workflow event/audit projection implementation for proposed records;
- report artifact citation from the proposed record;
- live sandbox write planning;
- GitHub provider mutation.

## 5. Proposed Persistence Boundary

The implemented helper is explicit and store-backed:

```text
compose_and_persist_github_pr_comment_proposed_side_effect_record(
    store: &impl SideEffectRecordStore,
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture_response: Option<&GitHubPullRequestCommentWriteResponse>,
    input: GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<SideEffectRecord, WorkflowOsError>
```

The helper:

- call `compose_github_pr_comment_proposed_side_effect_record(...)`;
- validate the returned record is lifecycle `Proposed`;
- write the record through `store.write_side_effect_record(&record)`;
- return the same validated record on success;
- maps store errors to stable non-leaking GitHub PR comment persistence error codes;
- performs no provider call, workflow event append, audit emission, report artifact write, lifecycle transition, file write outside the supplied store, or CLI output.

## 6. Store Contract Requirements

The implementation must rely on existing `SideEffectRecordStore` behavior:

- duplicate `SideEffectId` rejection;
- read/list validation;
- workflow ID consistency for records in the same run;
- workflow version consistency;
- schema version consistency;
- spec hash consistency;
- stable non-leaking store errors.

The helper should not bypass the store with direct filesystem writes. It should not create a second persistence mechanism or assume `LocalStateBackend` specifically.

## 7. Persistence Ordering

The required ordering is:

1. Build and validate `GitHubPullRequestCommentWriteRequest`.
2. Execute write preflight.
3. Construct `GitHubPullRequestCommentPreflightedWrite`.
4. Optionally validate fixture or dry-run response posture.
5. Compose in-memory proposed `SideEffectRecord`.
6. Persist that proposed record through `SideEffectRecordStore`.
7. Return the persisted record to the caller.

Do not persist:

- raw write requests;
- raw fixture responses;
- provider payloads;
- provider comment references;
- attempted/completed/failed provider outcomes;
- denied/skipped records unless separately planned.

## 8. Lifecycle Policy

The first persistence helper must write only `SideEffectLifecycleState::Proposed` records.

It must not write:

- `Attempted`;
- `Completed`;
- `Failed`;
- `Denied`;
- `Skipped`.

Denied and skipped record composition may become useful later, but those states have different authority and audit semantics and should be separately planned.

## 9. Error Handling

Errors must use stable, non-leaking codes.

Recommended codes:

- `github_pr_comment_side_effect_record.persistence.store_failed`;
- `github_pr_comment_side_effect_record.persistence.duplicate`;
- `github_pr_comment_side_effect_record.persistence.identity_mismatch`;
- `github_pr_comment_side_effect_record.persistence.invalid_record`;
- `github_pr_comment_side_effect_record.persistence.unsupported_lifecycle`.

The helper must not leak:

- repository names;
- pull request numbers beyond what is already intentionally stored in the validated target reference;
- SideEffect IDs;
- idempotency keys;
- run IDs;
- workflow IDs;
- spec hashes;
- summaries;
- redaction metadata values;
- provider references;
- raw payloads;
- secret-like values.

If the store returns a known duplicate or identity mismatch code, the helper may preserve the semantic category but should not forward raw message text if it could carry values.

## 10. Idempotency And Duplicate Behavior

The first persistence helper should be first-write-wins.

Recommended behavior:

- duplicate `SideEffectId` from the store returns a stable duplicate persistence error;
- the helper does not silently treat duplicate writes as success;
- the helper does not overwrite records;
- the helper does not read-compare-and-return existing records in the first slice;
- retry/idempotent rehydration semantics remain deferred until runtime integration planning.

This is conservative. A later runtime path may choose an idempotent read-existing behavior only after replay semantics are designed.

## 11. Relationship To Workflow Events And Audit

Persistence is not runtime history.

The helper must not append a `SideEffectProposed` workflow event and must not emit an audit event. Event/audit planning is documented in [GitHub PR Comment Proposed SideEffect Event/Audit Projection Plan](github-pr-comment-side-effect-event-audit-projection-plan.md) and decides:

- whether a persisted proposed record is required before a `SideEffectProposed` event;
- whether event projection should cite the record ID;
- whether audit projection should cite the record, the event, or both;
- how replay handles missing persisted records;
- how corrupt persisted records affect audit/report generation.

Until then, the persisted record is durable write intent, not proof of runtime execution.

## 12. Relationship To Report Artifacts

The helper must not write report artifacts or mutate WorkReport values.

Later report integration should decide:

- whether terminal reports cite proposed records automatically;
- whether report artifact integrity requires every cited SideEffect ID to be present in the store;
- whether proposed records should appear in side-effect sections before attempted/completed states exist;
- how missing proposed records should be disclosed.

## 13. Privacy And Redaction

The helper must preserve the existing composition helper privacy posture.

It must not persist:

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

Only the validated `SideEffectRecord` should be written. Debug and error output must remain redaction-safe.

## 14. Test Plan

Implemented and expected tests cover:

- valid fixture-backed proposed record is persisted and returned;
- valid dry-run proposed record is persisted and returned;
- persisted record can be read back from `SideEffectRecordStore`;
- persisted record appears in run listing;
- lifecycle is exactly `Proposed`;
- duplicate `SideEffectId` fails closed without leakage;
- workflow identity mismatch fails closed through store contract without leakage;
- schema/spec identity mismatch fails closed through store contract without leakage;
- invalid composed record is not persisted;
- provider response is rejected before persistence;
- live sandbox mode is rejected before persistence;
- no workflow event is appended;
- no audit event is emitted;
- no report artifact is written;
- no provider call occurs;
- no CLI output is emitted;
- raw provider/spec/command/parser payload markers are not persisted;
- secret-like summaries/references fail without leakage;
- existing provider-write tests still pass;
- existing SideEffect store contract tests still pass;
- existing WorkReport/report artifact tests still pass.

## 15. Documentation Updates

Implementation updated:

- `docs/implementation-plans/github-pr-comment-side-effect-record-composition-plan.md`;
- `docs/implementation-plans/github-pr-comment-side-effect-record-persistence-plan.md`;
- `docs/implementation-plans/write-adapter-readiness-plan.md`;
- `docs/integrations/github-future.md`;
- `ROADMAP.md`;
- an end-of-phase report under `docs/concepts/`.

Docs must state:

- proposed record persistence is implemented only through an explicit store-backed helper;
- automatic persistence is not implemented;
- provider mutation is not implemented;
- runtime side-effect execution is not implemented;
- workflow events and audit events are not emitted by this helper;
- report artifacts are not written by this helper;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, and release posture changes remain unimplemented.

## 16. Implementation Sequence

Completed small implementation phase:

1. Add the explicit store-backed helper.
2. Map composition and store failures to stable non-leaking error codes.
3. Add focused provider-write persistence tests using the existing in-memory or local store implementation.
4. Verify no runtime/event/audit/report/CLI/provider behavior is introduced.
5. Update docs and create an end-of-phase report.
6. Run maintainer review before event/audit projection implementation or live sandbox planning.

Do not combine this with live sandbox write planning.

## 17. Open Questions

- Should duplicate persistence eventually be idempotent read-existing, or should first-write-wins rejection remain the public behavior?
- Should the helper be GitHub-specific, generic over composed records, or both?
- Should fixture validation eventually have a dedicated `SideEffectReferenceKind`?
- Should event/audit projection require persisted records before appending `SideEffectProposed` events?
- Should terminal reports cite proposed records automatically once persistence exists?

## 18. Final Recommendation

The proposed `SideEffectRecord` persistence helper is implemented and reviewed, and the pure proposed-event construction helper for persisted GitHub PR comment proposed records is implemented in [GitHub PR Comment SideEffect Event Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_HELPER_REPORT.md). Proceed next to focused review of that event helper.

Future phases must still not build provider calls, GitHub mutation, runtime side-effect execution, attempted/completed/failed lifecycle transitions, automatic executor behavior, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes without separate accepted planning and review.
