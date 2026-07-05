# GitHub PR Comment Provider-Call Trait/Input Model Report

## 1. Executive Summary

The GitHub pull request comment write-adapter lane now has a first live-provider-call boundary model.

This phase adds only the injected provider trait, explicit caller-supplied auth wrapper, and validated provider-call request model needed for a later provider-call helper. It does not call GitHub, load credentials, transition side-effect lifecycle state, append workflow events, write report artifacts, expose CLI behavior, add schemas/examples, or change release posture.

## 2. Scope Completed

- Added `GitHubPullRequestCommentProviderAuth`.
- Added `GitHubPullRequestCommentProviderCallInput`.
- Added `GitHubPullRequestCommentProviderCallRequest`.
- Added `GitHubPullRequestCommentProvider`.
- Added a read-only `SideEffectRecord::idempotency()` accessor so provider-call gate validation can compare local idempotency without reaching into private fields.
- Exported the new provider-call boundary types from `workflow-core`.
- Added focused provider-write tests for the provider-call boundary.
- Updated the live provider-call plan and roadmap.

## 3. Scope Explicitly Not Completed

- No live GitHub provider call implementation.
- No default executor write behavior.
- No automatic side-effect execution.
- No auth loading from environment variables, keychains, GitHub CLI, git remotes, config files, or hidden global state.
- No provider retries or duplicate-comment reconciliation.
- No lifecycle transition helper for provider success/failure.
- No workflow event append.
- No audit or observability emission.
- No report artifact write.
- No CLI behavior.
- No workflow schema fields.
- No examples.
- No hosted/distributed runtime behavior.
- No reasoning lineage.
- No recursive agents, agent swarms, or Level 3/4 autonomy.
- No release posture changes.

## 4. Model/API Summary

`GitHubPullRequestCommentProviderAuth` is a bounded, redaction-safe wrapper for caller-supplied auth material. It is intentionally not serializable. Debug output redacts the secret and bounded scope summary.

`GitHubPullRequestCommentProviderCallInput` accepts an already-attempted `SideEffectRecord`, target, bounded comment body, idempotency key, explicit live mode, explicit live-call and provider-call opt-ins, caller-supplied auth, summary, sensitivity, and redaction metadata.

`GitHubPullRequestCommentProviderCallRequest` is the validated request passed to an injected provider. It remains in-memory and non-serializable because it carries auth material.

`GitHubPullRequestCommentProvider` is an injected trait boundary. Implementations may later call a provider, but the trait definition itself does not provide a network client, load auth, append events, transition lifecycle state, write artifacts, or expose CLI behavior.

## 5. Validation Boundary Summary

The provider-call request validates:

- the supplied side-effect record is valid;
- the record is already `Attempted`;
- the record has `GitHubWrite` capability;
- the record target is a GitHub pull request comment adapter target;
- the input target matches the attempted record;
- the input idempotency key matches the attempted record;
- policy references are present;
- human-approved authority includes approval references;
- mode is explicitly `LiveSandbox`;
- live-call and provider-call opt-ins are both true;
- auth is present and bounded;
- comment body, summary, and redaction metadata are bounded and redaction-safe.

## 6. Redaction/Privacy Summary

The new auth and provider-call request types do not implement serialization. Debug output redacts auth, comment body, side-effect ID, idempotency key, summary, and redaction metadata.

Validation errors use stable codes and do not include raw auth, provider payloads, command output, paths, tokens, headers, file contents, or secret-like values.

## 7. Test Coverage Summary

Focused tests cover:

- valid provider-call request construction from an attempted record;
- injected trait use with a mock provider response and no built-in network client;
- missing auth rejection;
- auth error non-leakage;
- disabled live-call rejection;
- disabled provider-call rejection;
- non-live mode rejection;
- target mismatch rejection;
- idempotency mismatch rejection;
- Debug non-leakage for auth, comment text, side-effect ID, and idempotency key;
- existing provider-write no-provider and fixture/dry-run behavior.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test provider_write` - passed, 71 tests.
- `cargo fmt --all --check` - passed after applying rustfmt.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed after eliding one test helper lifetime.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Governed Dogfood Summary

- Workflow: `dg/implement`
- Run ID: `run-1783278628615420000-2`
- Approval ID: `approval/run-1783278628615420000-2/implementation-approved`
- Approval actor: `user/delegated-maintainer`
- Approval outcome: granted
- Approval reason: `delegated-maintainer-approved-provider-call-trait-input-model`
- Final status: `Completed`
- Event summary: 39 events; 1 approval; 0 retries; 0 escalations.
- Out-of-kernel work: code edits, tests, documentation updates, validation commands, and git/PR actions were performed by Codex outside the kernel and disclosed here.

The implementation was run under the local Workflow OS dogfood governance loop.

## 10. Remaining Known Limitations

- The provider trait has no concrete GitHub implementation.
- Provider success/failure responses are not yet orchestrated into store-backed completed/failed lifecycle transitions.
- Provider-native idempotency behavior remains undecided.
- Live sandbox smoke testing remains unplanned beyond the accepted boundary plan.
- Runtime/executor integration remains explicitly deferred.

## 11. Recommended Next Phase

Recommended next phase: provider-call trait/input model review.

The next review should decide whether the boundary is tight enough to proceed to an injected-client orchestration helper that classifies mock provider success/failure into completed/failed lifecycle transitions. That future phase must still avoid default executor writes, auth loading, CLI behavior, schemas/examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes.
