# GitHub PR Comment Provider-Call Trait/Input Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The provider-call trait/input model is appropriately narrow. It establishes the next live-call boundary without implementing provider network calls, auth loading, lifecycle orchestration, executor writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved model-only implementation scope.

No accidental implementation found for:

- live GitHub writes;
- concrete provider network client;
- auth loading from environment variables, keychains, GitHub CLI, git remotes, config files, or hidden global state;
- default executor write behavior;
- runtime side-effect execution;
- automatic lifecycle transition after provider response;
- workflow event append;
- audit or observability emission;
- report artifact writing;
- CLI behavior;
- workflow schema fields;
- examples;
- hosted/distributed runtime;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 3. Model Assessment

The added model set is minimal and aligned with the accepted plan:

- `GitHubPullRequestCommentProviderAuth` represents caller-supplied auth material without serialization.
- `GitHubPullRequestCommentProviderCallInput` represents the unvalidated provider-call boundary.
- `GitHubPullRequestCommentProviderCallRequest` represents the validated request passed to an injected provider.
- `GitHubPullRequestCommentProvider` defines an injected trait that returns the existing validated response model.
- `SideEffectRecord::idempotency()` provides read-only access needed for pre-call gate validation.

This is a good boundary: the model can prove pre-call posture without becoming an HTTP client.

## 4. Pre-Call Gate Assessment

The request constructor validates the important gates:

- supplied side-effect record validates;
- lifecycle state is already `Attempted`;
- capability is `GitHubWrite`;
- target is a GitHub pull request comment adapter target;
- input target matches the attempted record target;
- input idempotency key matches the attempted record;
- policy references are present;
- human-approved authority includes approval references;
- mode is explicitly `LiveSandbox`;
- live-call and provider-call opt-ins are both true;
- auth is present and bounded;
- comment body, summary, and redaction metadata are validated.

This correctly preserves the architecture rule that a token is not authority.

## 5. Auth Boundary Assessment

The auth wrapper is non-serializable and has redaction-safe `Debug` behavior. It does not load from hidden state and does not imply production credential management.

The `secret_for_provider()` accessor is acceptable for an injected provider boundary, but future provider-call helper implementation should keep its use tightly scoped and covered by non-leakage tests.

## 6. Provider Trait Assessment

The injected trait is appropriately small:

- it receives a validated provider-call request;
- it returns the existing validated `GitHubPullRequestCommentWriteResponse`;
- it does not encode retry, auth loading, lifecycle transition, event append, artifact write, CLI, schema, or hosted behavior.

This keeps the next implementation testable with mock providers before any live network behavior is added.

## 7. Privacy And Redaction Assessment

The new request and auth types are intentionally not serializable. Debug output redacts:

- auth secret;
- auth scope summary;
- comment body;
- side-effect ID;
- idempotency key;
- summary;
- redaction metadata.

Validation errors use stable codes and do not include raw auth, tokens, provider payloads, command output, raw file contents, spec contents, environment values, headers, or paths.

## 8. Test Quality Assessment

Focused tests cover:

- valid provider-call request from an attempted record;
- injected trait use with a mock provider response and no built-in network client;
- missing auth rejection;
- auth error non-leakage;
- disabled live-call rejection;
- disabled provider-call rejection;
- non-live mode rejection;
- target mismatch rejection;
- idempotency mismatch rejection;
- Debug non-leakage for auth, comment text, side-effect ID, and idempotency key;
- existing no-provider and fixture/dry-run regressions through the provider-write suite.

Non-blocking gap: there is no explicit compile-time assertion that the request/auth types do not implement serde traits. Rust’s trait system enforces this indirectly, but a future test can document the boundary with a non-serializing API pattern or trybuild-style check if the repo adopts one.

## 9. Documentation Review

Docs are honest that:

- provider-call trait/input model is implemented;
- live GitHub provider calls are not implemented;
- auth loading is not implemented;
- provider success/failure lifecycle orchestration is not implemented;
- executor writes are not implemented;
- CLI behavior is not implemented;
- schemas/examples are not updated;
- hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes remain unsupported.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add an explicit regression proving provider clients are not invoked when a future orchestration helper fails any pre-call gate.
- Decide whether provider references should require a stricter live prefix before lifecycle orchestration.
- Consider a compile-time non-serialization assertion pattern for auth-bearing request types.
- Keep `secret_for_provider()` usage contained to injected provider implementations and covered by non-leakage tests.

## 12. Governed Dogfood Summary

- Workflow: `dg/review`
- Run ID: `run-1783279619811344000-2`
- Approval ID: `approval/run-1783279619811344000-2/review-scope-approved`
- Approval actor: `user/delegated-maintainer`
- Approval outcome: granted
- Approval reason: `delegated-maintainer-approved-provider-call-model-review`
- Final status: `Completed`
- Event summary: 39 events; 1 approval; 0 retries; 0 escalations.
- Out-of-kernel work: review document edits, documentation validation, and git/PR actions were performed by Codex outside the kernel and disclosed here.

The review was run under the local Workflow OS dogfood governance loop.

## 13. Validation

- `npm run check:docs` - passed.

## 14. Recommended Next Phase

Recommended next phase: injected-client provider-call orchestration helper planning or implementation.

The next slice should likely add a helper that accepts an injected provider client, validates all pre-call gates before invoking the trait, and classifies mocked provider success/failure into completed/failed lifecycle transition inputs or results. It must still avoid default executor writes, live network tests by default, auth loading, CLI behavior, schemas/examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes.
