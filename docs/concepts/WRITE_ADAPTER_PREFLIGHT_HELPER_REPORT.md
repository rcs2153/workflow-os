# Write Adapter Preflight Helper Report

## 1. Executive Summary

The write-adapter preflight helper phase is implemented.

The phase adds an adapter-neutral, deterministic preflight model/helper in `workflow-core`. It validates whether a future adapter write request has enough governed posture to be considered ready for a later write path: supported capability, bounded target, proposed `SideEffect` ID, idempotency key, allowed policy posture, required approval references, optional high-assurance references, bounded summary, sensitivity, and redaction metadata.

The helper does not call providers, execute side effects, transition `SideEffect` lifecycle state, append workflow events, write report artifacts, expose CLI behavior, add schemas, update examples, or authorize write-capable adapters.

## 2. Scope Completed

Completed:

- added `AdapterWriteCapability`, `AdapterWriteTargetKind`, `AdapterWriteTarget`, `AdapterWritePolicyDecision`, `AdapterWriteReadinessPolicy`, `AdapterWritePreflightRequest`, and `AdapterWritePreflightDecision`;
- added `preflight_adapter_write(...)`;
- exported the helper and model types from `workflow-core`;
- added focused Rust tests for ready preflight, unsupported capability rejection, missing governance references, policy denial, approval/high-assurance requirements, redaction safety, serde validation, and no-execution flags;
- updated the roadmap and readiness plan to state that preflight-only helper support is implemented.

## 3. Scope Explicitly Not Completed

Not implemented:

- write-capable adapters;
- provider mutation;
- provider write request/response models;
- runtime side-effect execution;
- `SideEffect` attempted/completed/failed lifecycle transitions;
- workflow event appends from the preflight helper;
- report artifact writes from the preflight helper;
- executor integration;
- CLI behavior;
- workflow schemas;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Helper API Summary

The implementation adds:

- `AdapterWriteReadinessPolicy::local_preview_comments_only()`
- `AdapterWriteTarget::new(...)`
- `AdapterWritePreflightRequest::new(...)`
- `preflight_adapter_write(&AdapterWritePreflightRequest)`

The default local preview policy supports preflight classification for low-risk comment-shaped capabilities only:

- `GitHubPullRequestComment`
- `JiraIssueComment`

Unsupported or unknown capabilities fail closed. The helper returns a ready decision only after it verifies required policy, `SideEffect`, idempotency, approval, and high-assurance posture.

## 5. Validation Boundary Summary

The helper validates:

- requested capability is known and supported by the supplied readiness policy;
- target kind is known;
- target reference is bounded and not secret-like;
- proposed `SideEffect` ID is present;
- idempotency key is present;
- policy decision is allowed;
- policy decision references are present and cite policy decisions;
- approval decision references are present when required by request, sensitivity, or readiness policy;
- high-assurance references are present when high-assurance posture is required;
- summary is bounded and not secret-like;
- redaction metadata is bounded and not secret-like.

Validation errors use stable codes and avoid echoing raw target references, summaries, redaction metadata, or secret-like values.

## 6. No-Execution Boundary Summary

The helper is pure preflight. The returned `AdapterWritePreflightDecision` explicitly reports:

- `provider_call_allowed = false`
- `side_effect_lifecycle_transition_allowed = false`
- `workflow_event_append_allowed = false`
- `report_artifact_write_allowed = false`

This makes the current phase useful for runtime composition without creating accidental write authority.

## 7. Redaction And Privacy Summary

The helper rejects secret-like targets, summaries, and redaction metadata. It uses redaction-safe `Debug` implementations for request and decision types so target references, summaries, `SideEffect` IDs, idempotency keys, and redaction metadata are not printed by debug formatting.

The model does not store raw provider payloads, command output, CI logs, Jira/GitHub bodies, raw spec contents, parser payloads, environment variable values, credentials, authorization headers, private keys, or token-like values.

## 8. Test Coverage Summary

Focused tests cover:

- valid low-risk preflight decision;
- unsupported capability rejection;
- unknown capability rejection;
- missing `SideEffect` ID;
- missing idempotency key;
- missing policy reference;
- denied policy decision;
- missing approval reference when required;
- sensitive capability requiring approval;
- missing high-assurance reference;
- high-assurance evidence reference acceptance;
- duplicate reference rejection;
- secret-like target rejection without leakage;
- secret-like summary rejection without leakage;
- secret-like redaction metadata rejection without leakage;
- redaction-safe `Debug`;
- serde round trip for valid request;
- invalid serialized target failure without leakage;
- serialization non-leakage of forbidden payload markers;
- default readiness policy staying separate from broad write capabilities.

## 9. Commands Run And Results

Commands run:

- `cargo test -p workflow-core --test write_adapter_preflight` - passed.
- `cargo fmt --all --check` - passed after applying rustfmt.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Dogfood Governance Summary

This implementation phase was governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/implement`
- Run ID: `run-1783192846947482000-2`
- Approval ID: `approval/run-1783192846947482000-2/implementation-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: Codex read repository documents, implemented the Rust helper and tests, updated documentation, ran validation commands, and closed the governed implementation phase. No git or PR action was performed in this phase.

## 11. Remaining Limitations

- The helper is not integrated into executor write paths.
- No provider write request/response model exists.
- No provider write candidate is implemented.
- No live write tests exist.
- The default readiness policy only supports comment-shaped preflight classification.
- Broad sensitive-capability taxonomy remains future work.

## 12. Recommended Next Phase

Recommended next phase: write adapter preflight helper review.

If accepted, the next phase should be first provider write candidate planning, likely GitHub PR comment or Jira sandbox comment, still without jumping directly to provider mutation.
