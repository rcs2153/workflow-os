# GitHub PR Comment Write Boundary Report

## 1. Executive Summary

The model-only GitHub pull request comment write request/response boundary is implemented.

This phase adds bounded Rust model types for a future GitHub PR comment write candidate. The boundary validates target identity, comment body, preflight alignment, proposed SideEffect identity, idempotency posture, redaction metadata, response shape, serde, and redaction-safe Debug behavior.

It does not call GitHub, mutate providers, execute side effects, append workflow events, write report artifacts, add CLI behavior, add schemas, update examples, implement hosted behavior, implement reasoning lineage, or change release posture.

## 2. Scope Completed

Completed:

- added `GitHubPullRequestCommentTarget`;
- added `GitHubPullRequestCommentWriteMode`;
- added `GitHubPullRequestCommentWriteOutcome`;
- added `GitHubPullRequestCommentWriteRequest`;
- added `GitHubPullRequestCommentWriteRequestDefinition`;
- added `GitHubPullRequestCommentWriteResponse`;
- added `GitHubPullRequestCommentWriteResponseDefinition`;
- added `github_pr_comment_preflight_definition(...)` helper for matching preflight input construction;
- exported the new model types from `workflow-core`;
- added focused model tests;
- updated write-readiness, first-candidate, roadmap, and GitHub posture docs.

## 3. Scope Explicitly Not Completed

Not implemented:

- GitHub provider calls;
- pull request comment creation;
- write-capable GitHub adapter execution;
- fixture-backed adapter execution;
- live sandbox write smoke;
- runtime side-effect execution;
- SideEffect attempted/completed/failed lifecycle transitions;
- automatic workflow event appends for write attempts;
- automatic report generation or report artifact writing;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- OAuth app behavior or webhook ingestion;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Model Boundary Summary

The request model captures:

- adapter and integration identity;
- correlation ID;
- workflow ID and version;
- schema version;
- spec hash;
- run ID;
- optional step ID;
- actor;
- bounded GitHub owner/repository/pull request target;
- bounded comment body;
- bounded summary;
- proposed SideEffect ID;
- idempotency key;
- write mode;
- matching adapter write preflight request;
- sensitivity;
- redaction metadata.

The response model captures:

- correlation ID;
- write mode;
- outcome;
- optional provider comment reference for future live success;
- optional provider error code for future classified failure;
- bounded summary;
- sensitivity;
- redaction metadata.

Both models are validation-only. They do not authorize provider calls, event appends, report writes, or SideEffect lifecycle transitions.

## 5. Validation Boundary Summary

Validation ensures:

- GitHub owner and repository are bounded and not URL/path shaped;
- pull request number is greater than zero;
- comment body is non-empty, bounded, and not secret-like;
- summaries are non-empty, bounded, and not secret-like;
- redaction metadata field names and reasons are bounded and not secret-like;
- preflight capability is `GitHubPullRequestComment`;
- preflight target kind is `GitHubPullRequest`;
- preflight target reference matches the GitHub target;
- preflight SideEffect ID matches the request;
- preflight idempotency key matches the request;
- fixture and dry-run responses cannot include provider references or provider errors;
- provider success responses require a provider reference;
- provider failure responses require a bounded provider error code.

Validation errors use stable codes with the `github_pr_comment_write.*` prefix and avoid echoing raw values.

## 6. Redaction And Privacy Summary

The model rejects or redacts:

- secret-like comment body text;
- forbidden raw payload markers;
- secret-like target values;
- secret-like redaction metadata;
- secret-like provider references and error codes.

`Debug` redacts:

- correlation ID;
- spec hash;
- run ID;
- step ID;
- actor;
- target owner/repository;
- comment body;
- summary;
- SideEffect ID;
- idempotency key;
- preflight details;
- provider comment reference;
- redaction metadata.

Serialization preserves valid bounded request/response fields for future fixture use, but invalid serialized requests fail closed through constructors during deserialization.

## 7. Tests Added

Added `crates/workflow-core/tests/provider_write.rs` covering:

- valid model-only request;
- target URL/path rejection;
- zero pull request number rejection;
- preflight target mismatch rejection;
- preflight SideEffect mismatch rejection;
- secret-like comment body rejection;
- forbidden raw payload marker rejection;
- secret-like redaction metadata rejection;
- request Debug non-leakage;
- request serde round trip;
- invalid serialized request failure without leaking secret-like value;
- valid fixture response;
- fixture response provider reference rejection;
- provider success response provider reference requirement;
- provider failed response error code requirement;
- response Debug non-leakage.

## 8. Commands Run And Results

Validation commands:

- `cargo fmt --all` - passed
- `cargo test -p workflow-core --test provider_write` - passed
- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

## 9. Dogfood Governance Summary

This implementation phase was governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/implement`
- Run ID: `run-1783196673620878000-2`
- Approval ID: `approval/run-1783196673620878000-2/implementation-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Terminal: true
- Events total: 39
- Approvals: 1
- Retries: 0
- Escalations: 0
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: repository inspection, Rust model implementation, tests, documentation updates, validation commands, and phase-close inspection were performed by the agent outside kernel execution. The kernel coordinated governance only. No provider write, git operation, or PR action was performed by the kernel.

## 10. Remaining Known Limitations

- No provider write adapter exists.
- No fixture-backed adapter path exists.
- No preflight execution composition helper exists for this request model yet.
- No persisted proposed SideEffectRecord is required by this model.
- No live sandbox write is approved.
- No CLI or schema surface exists.

## 11. Recommended Next Phase

Recommended next phase: GitHub PR comment write boundary review.

After review, the next implementation should be fixture-backed GitHub PR comment adapter planning or preflight composition planning, still with no live provider mutation.
