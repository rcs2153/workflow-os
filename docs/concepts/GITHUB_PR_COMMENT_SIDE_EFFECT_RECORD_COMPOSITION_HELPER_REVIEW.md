# GitHub PR Comment Proposed SideEffectRecord Composition Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the intended in-memory helper for composing a validated proposed `SideEffectRecord` from an already preflighted GitHub pull request comment write candidate. The phase stays within the approved no-provider-call boundary and does not add persistence, runtime side-effect execution, lifecycle transitions beyond `Proposed`, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

The next phase should be proposed `SideEffectRecord` persistence planning, still before any live sandbox write planning.

## 2. Scope Verification

The phase stayed within the approved in-memory composition-helper scope.

Implemented scope:

- `GitHubPullRequestCommentSideEffectRecordInput`;
- `compose_github_pr_comment_proposed_side_effect_record(...)`;
- read-only accessors needed to derive request identity from a validated GitHub PR comment write request;
- response mode accessor needed to validate fixture/dry-run posture;
- focused tests and documentation updates;
- implementation phase report.

No accidental implementation was found for:

- GitHub provider calls;
- pull request comment creation;
- live sandbox writes;
- provider auth handling;
- provider mutation;
- proposed record persistence;
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

## 3. Helper API Assessment

The helper API is appropriately narrow:

```text
compose_github_pr_comment_proposed_side_effect_record(
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture_response: Option<&GitHubPullRequestCommentWriteResponse>,
    input: GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<SideEffectRecord, WorkflowOsError>
```

It requires `GitHubPullRequestCommentPreflightedWrite`, not a raw request. That preserves the intended sequence of request validation, preflight validation, optional fixture/dry-run posture validation, then proposed record composition. The helper returns an in-memory `SideEffectRecord` and has no store, executor, CLI, provider, or artifact dependency.

The input type is also bounded: it carries only context not safely derivable from the preflighted request, such as created timestamp, optional skill identity, optional additional references, optional summary override, and optional sensitivity override. Its `Debug` implementation redacts caller-supplied identity and summary fields.

## 4. SideEffectRecord Mapping Assessment

The helper maps the proposed record correctly:

- request `side_effect_id` is preserved;
- lifecycle is always `Proposed`;
- target kind is `AdapterResource`;
- target reference is the bounded GitHub pull request target reference;
- capability is `GitHubWrite`;
- actor is the request actor;
- workflow ID, workflow version, schema version, spec hash, run ID, and optional step ID are preserved;
- optional skill ID/version are accepted only through explicit input;
- adapter ID, `GitHub` adapter kind, and integration ID are preserved;
- idempotency uses the request idempotency key with run scope;
- policy, approval, and additional stable references are included;
- outcome reference remains absent for proposed records;
- summary is bounded and redaction-checked;
- sensitivity is the conservative maximum of request and input sensitivity;
- request redaction metadata is preserved.

This is the right first composition shape. It makes write intent governable without claiming that a provider mutation has happened.

## 5. Authority And Idempotency Assessment

Authority mapping is conservative:

- no approval references maps to `AllowedByPolicy`;
- approval references map to `ApprovedByHuman`;
- denied policy, unsupported capability, and missing required approval cannot reach this helper through valid preflighted-write construction.

The helper also rejects a supplied `system_actor` because the GitHub PR comment request already requires a requesting actor. That avoids ambiguous dual-authority records in the first slice.

Idempotency is correctly scoped to the workflow run. Broader adapter or integration idempotency can wait until provider mutation and retry semantics are designed.

## 6. Fixture, Dry-Run, And Provider Response Assessment

Fixture and dry-run responses are treated as validation posture, not provider truth.

The helper accepts only:

- `FixtureValidated`;
- `DryRunValidated`.

It rejects:

- live sandbox request mode;
- provider succeeded/failed outcomes;
- mismatched response mode;
- fixture/dry-run responses containing provider comment references or provider error codes.

It also does not create an outcome reference from fixture validation. That is correct because fixture validation is not external mutation evidence.

## 7. No-Execution Boundary Assessment

The implementation preserves the no-execution boundary.

The helper does not:

- call GitHub;
- read credentials;
- mutate provider state;
- persist the record;
- append workflow events;
- emit audit events;
- transition lifecycle beyond `Proposed`;
- write report artifacts;
- write files;
- emit CLI output.

The helper validates that the preflighted write and fixture path do not authorize provider calls, workflow event appends, side-effect lifecycle transitions, or report artifact writes.

## 8. Privacy And Redaction Assessment

The privacy posture is consistent with the provider-write and SideEffect models.

The helper does not accept or copy:

- raw GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw pull request bodies;
- raw diffs;
- raw CI logs;
- raw command output;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded prompt text;
- secret-like summaries or references.

Errors are stable and non-leaking. The implementation wraps invalid target, idempotency, reference, authority, response, mode, and record-construction failures with bounded GitHub PR comment SideEffect error codes rather than exposing caller-supplied values.

`GitHubPullRequestCommentSideEffectRecordInput` uses redaction-safe `Debug`, and existing `WorkReport`/SideEffect redaction boundaries remain unchanged.

## 9. Test Quality Assessment

Tests cover the main acceptance criteria:

- fixture response composes a proposed record;
- dry-run response composes a proposed record;
- lifecycle is `Proposed`;
- capability is `GitHubWrite`;
- target kind and bounded target reference are correct;
- policy-only authority maps to `AllowedByPolicy`;
- approval references map to `ApprovedByHuman`;
- workflow/run/SideEffect identity is preserved;
- stable references are included;
- outcome reference is absent;
- provider responses are rejected without leaking provider references;
- secret-like summary overrides are rejected without leakage;
- system actor is rejected when request actor exists;
- input `Debug` redacts sensitive fields;
- existing request, preflight, fixture, response, serde, provider-write, and side-effect tests continue to pass.

Non-blocking test follow-ups:

- add a direct mode-mismatch fixture response test;
- add a direct live-sandbox request rejection test for the composition helper;
- add a direct assertion that the helper has no store dependency and performs no persistence once proposed-record persistence planning starts.

These are follow-ups rather than blockers because the current API shape and tests already protect the phase boundary.

## 10. Documentation Review

Documentation accurately states:

- in-memory proposed `SideEffectRecord` composition is implemented for GitHub PR comment writes;
- fixture-backed adapter validation remains no-provider-call;
- proposed record persistence is not implemented;
- GitHub provider mutation is not implemented;
- runtime side-effect execution is not implemented;
- workflow event and audit projection are not implemented for this path;
- report artifacts are not written by this path;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes remain unimplemented.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Plan proposed `SideEffectRecord` persistence before live sandbox write planning.
- Decide whether proposed-record persistence should happen through a GitHub-specific helper or a generic store-backed helper.
- Keep workflow event and audit projection separate from persistence until the record source-of-truth boundary is settled.
- Consider a dedicated fixture-validation reference kind later if fixture evidence needs first-class citation semantics.
- Add the extra composition-helper negative tests listed above when touching this surface again.

## 13. Recommended Next Phase

Recommended next phase: proposed `SideEffectRecord` persistence planning.

That phase should decide how a validated proposed record becomes durable through the existing `SideEffectRecordStore`, while preserving no provider calls, no runtime side-effect execution, no lifecycle transition beyond `Proposed`, no workflow event append, no audit event emission, no report artifact write, no CLI behavior, no schema changes, no examples, no hosted behavior, no reasoning lineage, no autonomy expansion, and no release posture changes.

## 14. Validation

Validation commands run for this review:

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test provider_write` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 15. Dogfood Governance Summary

This review phase is governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1783204941454562000-2`
- Approval ID: `approval/run-1783204941454562000-2/review-scope-approved`
- Approval outcome: granted
- Final run status: completed.
- Terminal: true.
- Events total: 39.
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Retries: 0.
- Escalations: 0.

Out-of-kernel work disclosed: repository inspection, review drafting, validation commands, and phase-close inspection are performed by the agent outside kernel execution. The kernel coordinates governance only.
