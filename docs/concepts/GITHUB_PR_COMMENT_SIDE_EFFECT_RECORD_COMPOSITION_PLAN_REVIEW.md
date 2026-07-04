# GitHub PR Comment Proposed SideEffectRecord Composition Plan Review

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The plan defines the correct next boundary after fixture-backed GitHub pull request comment validation: compose a validated in-memory proposed `SideEffectRecord` from an already preflighted write candidate, without provider calls, persistence, lifecycle transitions, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

It is safe to proceed to the GitHub PR comment proposed `SideEffectRecord` composition helper implementation, scoped to in-memory composition only.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization was found for:

- implementation in the planning phase;
- GitHub provider calls;
- pull request comment creation;
- live sandbox writes;
- provider auth handling;
- runtime side-effect execution;
- attempted, completed, or failed SideEffect lifecycle transitions;
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

A small documentation correction was made in the phase report to remove a misleading "persisted proposed" phrase. Persistence remains deferred.

## 3. Boundary Assessment

The proposed helper boundary is appropriately narrow:

```text
compose_github_pr_comment_proposed_side_effect_record(...)
  -> Result<SideEffectRecord, WorkflowOsError>
```

The plan requires `GitHubPullRequestCommentPreflightedWrite` rather than raw write request values. That preserves the intended sequence:

1. construct and validate the request;
2. execute adapter write preflight;
3. optionally validate fixture or dry-run response posture;
4. compose a proposed `SideEffectRecord`;
5. leave persistence, event/audit projection, report artifacts, and provider mutation to later reviewed phases.

This is the right runtime-composition direction. It closes part of the gap between write intent and SideEffect governance without crossing into actual writes.

## 4. SideEffectRecord Mapping Assessment

The plan maps cleanly to the existing `SideEffectRecord` model:

- `side_effect_id` from the request;
- `lifecycle_state` as `Proposed`;
- `target` as an adapter resource using the GitHub pull request target reference;
- `capability` as `GitHubWrite`;
- `authority` from preflight policy and approval references;
- workflow/run/spec identity from the request;
- optional step, skill, adapter, and integration identity;
- idempotency binding from the request idempotency key;
- stable references from policy, approval, fixture, and caller-supplied records;
- no outcome reference for proposed records;
- explicit created timestamp;
- optional correlation ID;
- bounded summary, conservative sensitivity, and validated redaction metadata.

The model vocabulary already supports the needed pieces: `SideEffectLifecycleState::Proposed`, `SideEffectCapability::GitHubWrite`, `SideEffectTargetKind::AdapterResource`, `SideEffectAuthorityDecision`, `SideEffectIdempotencyBinding`, and `SideEffectReference`.

## 5. Authority And Idempotency Assessment

The authority mapping policy is conservative enough for the next implementation.

Accepted first-slice posture:

- allowed preflight without required approval maps to `AllowedByPolicy`;
- allowed preflight with accepted approval references may map to `ApprovedByHuman`;
- approval-required-but-not-granted, denied policy, and unsupported capability must fail before proposed-record composition.

The plan correctly avoids composing attemptable records for denied or incomplete authority paths. A future denied/skipped record composition helper may be useful, but it should be separately planned because it has different audit semantics.

The idempotency posture is also appropriate: use the request idempotency key and scope it to the run for the first slice. Later adapter/integration-wide idempotency can wait until provider mutation behavior exists.

## 6. Fixture Response Relationship

The plan correctly treats fixture and dry-run responses as validation posture, not provider truth.

It requires:

- no provider comment references from fixture validation;
- no `Attempted`, `Completed`, or `Failed` lifecycle state from fixture output;
- no use of fixture response to bypass preflight, policy, approval, or idempotency;
- no raw fixture response payload stored in the record.

This is important because a fixture response can prove local construction behavior, but it cannot prove an external GitHub mutation.

## 7. Persistence, Event, And Audit Posture

The plan keeps the first helper in-memory only.

It explicitly defers:

- automatic `SideEffectRecordStore` writes;
- workflow event appends;
- audit event emission;
- report artifact writes;
- executor integration;
- runtime side-effect execution.

That boundary is correct. A proposed `SideEffectRecord` can become the future source of truth for write intent only after a separate persistence helper or runtime path explicitly writes it through the existing `SideEffectRecordStore`.

## 8. Privacy And Redaction Assessment

The privacy posture is strong and consistent with the existing provider-write and SideEffect models.

The plan forbids:

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

The future helper should preserve redaction-safe `Debug` behavior for any new input wrapper and should map constructor failures to stable non-leaking error codes. It should not expose owner/repository names, PR targets, SideEffect IDs, idempotency keys, run IDs, spec hashes, summaries, redaction metadata, or fixture values in errors.

## 9. Error-Handling Assessment

The candidate error codes are suitable and bounded:

- `github_pr_comment_side_effect_record.preflight.not_ready`;
- `github_pr_comment_side_effect_record.mode.unsupported`;
- `github_pr_comment_side_effect_record.authority.unsupported`;
- `github_pr_comment_side_effect_record.target.invalid`;
- `github_pr_comment_side_effect_record.reference.invalid`;
- `github_pr_comment_side_effect_record.record.invalid`.

The next implementation should avoid forwarding raw constructor errors where those errors could include caller-supplied values. It should wrap failures in these stable codes.

## 10. Test Plan Assessment

The planned tests cover the right behaviors:

- valid fixture and dry-run preflighted writes compose proposed records;
- raw write request values are not accepted by the helper API;
- live sandbox remains unsupported;
- lifecycle is `Proposed`;
- capability is `GitHubWrite`;
- target is bounded and redaction-safe;
- authority references preserve policy and approval context;
- idempotency and immutable workflow/run/spec identity are preserved;
- fixture response does not create provider references;
- no attempted/completed/failed lifecycle state is produced;
- no store write, event append, audit event, report artifact, provider call, or CLI output occurs;
- secret-like values fail without leakage;
- debug output is redaction-safe;
- existing provider-write and side-effect tests continue to pass.

Non-blocking additions for implementation:

- direct test for `AllowedByPolicy` authority mapping;
- direct test for `ApprovedByHuman` authority mapping when approval references are present;
- direct test that `RequiresApproval` without accepted references fails closed;
- direct test that fixture response omission still produces a valid proposed record;
- direct test that any supplied fixture reference is treated only as a stable reference, not a provider outcome.

## 11. Documentation Review

Docs now accurately state:

- proposed `SideEffectRecord` composition is planned, not implemented;
- fixture-backed GitHub PR comment validation is implemented;
- provider mutation is not implemented;
- proposed record persistence is not implemented for this path;
- runtime side-effect execution is not implemented;
- workflow event and audit projection are not implemented for this path;
- report artifacts are not written by this path;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes remain unimplemented.

## 12. Planning Blockers

No planning blockers.

## 13. Non-Blocking Follow-Ups

- Keep denied/skipped SideEffect record composition as a separate future planning topic.
- Decide later whether fixture-validation references need a dedicated `SideEffectReferenceKind`.
- Keep persistence separate from the first composition helper.
- Keep live sandbox writes blocked until proposed-record composition, persistence posture, event/audit posture, and approval linkage are reviewed together.

## 14. Recommended Next Phase

Recommended next phase: GitHub PR comment proposed `SideEffectRecord` composition helper implementation.

The implementation should be in-memory only, accept `GitHubPullRequestCommentPreflightedWrite`, optionally accept fixture/dry-run response context, return a validated proposed `SideEffectRecord`, and preserve no provider calls, no persistence, no lifecycle transition beyond `Proposed`, no workflow event append, no audit event, no report artifact, no CLI behavior, no schema changes, no examples, no hosted behavior, no reasoning lineage, no autonomy expansion, and no release posture changes.

## 15. Validation

Review validation:

- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 16. Dogfood Governance Summary

This review phase is governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1783203361850466000-2`
- Approval ID: `approval/run-1783203361850466000-2/review-scope-approved`
- Approval outcome: granted
- Final run status: completed.
- Terminal: true.
- Events total: 39.
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Retries: 0.
- Escalations: 0.

Out-of-kernel work disclosed: repository inspection, maintainer review drafting, validation commands, and phase-close inspection are performed by the agent outside kernel execution. The kernel coordinates governance only.
