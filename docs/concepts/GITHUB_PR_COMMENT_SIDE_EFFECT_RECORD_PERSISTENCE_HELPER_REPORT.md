# GitHub PR Comment Proposed SideEffectRecord Persistence Helper Report

## 1. Executive Summary

The GitHub PR comment write-candidate lane now has an explicit store-backed helper for persisting validated proposed `SideEffectRecord` values.

The helper composes the proposed record through the existing no-provider-call composition boundary, verifies the lifecycle remains `Proposed`, writes only through a caller-supplied `SideEffectRecordStore`, and returns the validated record. It does not call GitHub, append workflow events, emit audit events, transition lifecycle beyond `Proposed`, write report artifacts, expose CLI behavior, add schemas, update examples, or change release posture.

## 2. Scope Completed

- Added `compose_and_persist_github_pr_comment_proposed_side_effect_record(...)`.
- Reused `compose_github_pr_comment_proposed_side_effect_record(...)` before persistence.
- Wrote proposed records only through `SideEffectRecordStore`.
- Enforced first-write-wins duplicate behavior through the store.
- Mapped store failures to stable non-leaking GitHub PR comment persistence error codes.
- Exported the helper from `workflow-core`.
- Added focused provider-write persistence tests using `LocalStateBackend`.
- Updated roadmap and integration documentation.

## 3. Scope Explicitly Not Completed

- No GitHub provider call.
- No GitHub PR comment mutation.
- No live sandbox write.
- No runtime side-effect execution.
- No lifecycle transition beyond `Proposed`.
- No workflow event append.
- No audit event emission.
- No report artifact write.
- No automatic executor integration.
- No CLI behavior.
- No workflow schema fields.
- No examples.
- No hosted or distributed behavior.
- No reasoning lineage.
- No autonomy expansion or release posture change.

## 4. Helper API Summary

The new helper accepts:

- a `SideEffectRecordStore`;
- a preflighted GitHub PR comment write;
- an optional fixture/dry-run response;
- the existing GitHub PR comment side-effect record input.

It returns the persisted `SideEffectRecord` on success. It does not create a store, assume `LocalStateBackend`, bypass the store contract, or write directly to filesystem paths outside the supplied store implementation.

## 5. Persistence Boundary Summary

The persistence ordering is:

1. Validate the GitHub PR comment write request.
2. Execute write preflight.
3. Construct the preflighted write model.
4. Optionally validate fixture or dry-run response posture.
5. Compose an in-memory proposed `SideEffectRecord`.
6. Verify the lifecycle is `Proposed`.
7. Persist through `SideEffectRecordStore`.
8. Return the same validated record.

This makes proposed write intent durable without turning persistence into execution.

## 6. Store Contract And Error Handling Summary

The helper relies on the existing store contract for:

- duplicate `SideEffectId` rejection;
- immutable workflow/run/spec identity checks;
- read/list validation;
- durable local persistence behavior where the supplied store is `LocalStateBackend`.

Known store errors are mapped to GitHub-specific persistence codes:

- `github_pr_comment_side_effect_record.persistence.duplicate`;
- `github_pr_comment_side_effect_record.persistence.identity_mismatch`;
- `github_pr_comment_side_effect_record.persistence.store_failed`;
- `github_pr_comment_side_effect_record.persistence.unsupported_lifecycle`.

Mapped errors do not forward raw store messages, SideEffect IDs, run IDs, target paths, summaries, provider references, payloads, or secret-like values.

## 7. Redaction And Privacy Summary

The helper persists only validated `SideEffectRecord` values. It does not persist raw GitHub tokens, authorization headers, provider payloads, PR bodies, diffs, CI logs, command output, file contents, spec contents, environment variable values, prompts, or secret-like values.

Provider-response outcomes are rejected before persistence for this proposed-record path. Secret-like summary overrides are rejected before persistence.

## 8. Test Coverage Summary

Focused tests cover:

- fixture-backed proposed record persistence and read-back;
- dry-run proposed record persistence;
- run-level list visibility;
- exact `Proposed` lifecycle;
- duplicate SideEffect ID rejection without leakage;
- provider response rejection before store write;
- secret-like summary rejection before store write;
- existing provider-write request, fixture, response, and composition behavior.

Existing SideEffect store contract tests continue to cover broader store duplicate and identity-mismatch behavior.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test provider_write` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Dogfood Governance Summary

- Dogfood workflow: `dg/implement`.
- Governed run ID: `run-1783206611856447000-2`.
- Approval ID: `approval/run-1783206611856447000-2/implementation-approved`.
- Approval outcome: granted by `user/dogfood-reviewer`.
- Terminal status: completed.
- Event summary: 39 events, including one approval request, one approval grant, eight policy decisions, six scheduled steps, six skill invocation requests, six skill starts, six skill successes, and run completion.

The phase stayed inside the approved implementation boundary. The kernel governed scope and approval; Codex performed repository edits and validation.

## 11. Remaining Known Limitations

- Proposed record persistence is explicit helper-only.
- Automatic persistence from executor paths is not implemented.
- Proposed record workflow event/audit projection is not implemented.
- Report artifact citation from the persisted proposed record is not automatic.
- Duplicate writes are rejected rather than treated as idempotent read-existing replays.
- Live sandbox GitHub PR comment write planning remains future work.

## 12. Recommended Next Phase

Recommended next phase: proposed record persistence helper review.

This is write-readiness-adjacent and store-backed, so it should receive a focused maintainer review before any workflow event/audit projection, executor integration, or live sandbox planning.
