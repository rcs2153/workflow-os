# GitHub PR Comment Provider Client/Auth Loading Plan Review

## 1. Executive Verdict

Plan accepted; proceed to concrete GitHub PR comment provider client implementation, explicit injected transport only.

The plan defines a narrow and appropriate next boundary. It keeps provider writes explicit, keeps credential discovery out of runtime defaults, preserves the injected-provider trait contract, and defers executor integration, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, and release posture changes.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization found for:

- implementation in the planning phase;
- automatic provider writes;
- default executor write behavior;
- executor integration;
- approval-resume or cancellation write behavior;
- automatic workflow event append;
- automatic audit or observability emission;
- automatic report artifact writing;
- CLI mutation commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- hidden auth discovery;
- OAuth app behavior;
- webhook ingestion;
- broad GitHub write support;
- Jira or other provider writes;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 3. Boundary Assessment

The plan correctly positions the first concrete client as an implementation of the existing `GitHubPullRequestCommentProvider` trait. That keeps the transport/client concern behind the already-reviewed provider-call request and injected orchestration helper.

The plan also correctly keeps the provider client from:

- mutating workflow state;
- writing side-effect records;
- appending workflow events;
- emitting audit events directly;
- writing report artifacts;
- printing CLI output;
- reading repository files;
- inferring target identity from git state.

This is the right boundary for the first concrete provider slice.

## 4. Auth Loading Assessment

The plan is appropriately conservative.

Accepted first implementation:

- caller passes `GitHubPullRequestCommentProviderAuth` directly to the provider constructor;
- auth wrapper remains non-serializable and redaction-safe;
- tests use explicit fake auth values.

Deferred auth paths:

- environment variable loading;
- GitHub CLI discovery;
- keychain discovery;
- config-file loading;
- OAuth installation tokens;
- secret manager integration;
- hosted credential brokering.

That separation avoids the common trap where adding a concrete client silently becomes ambient credential discovery.

## 5. Request Execution Assessment

The plan authorizes one provider call per validated request and disallows automatic retries unless a future reviewed retry policy exists.

The plan correctly requires:

- pre-existing attempted `SideEffectRecord`;
- matching target;
- matching idempotency key;
- `LiveSandbox` mode;
- explicit live-call and provider-call enablement;
- explicit auth;
- validated comment body, summary, sensitivity, and redaction metadata.

## 6. Idempotency and Reconciliation Assessment

The plan is honest about GitHub comment creation not providing a simple generic idempotency key for this use case.

The conservative v1 posture is acceptable:

- rely on Workflow OS local idempotency gates;
- do not assume provider-native idempotency;
- do not silently retry ambiguous provider failures;
- plan remote-success/local-transition-failure reconciliation before executor-integrated live writes.

This should remain a highlighted limitation during implementation and review.

## 7. Provider Response Classification Assessment

The proposed provider error vocabulary is suitably bounded:

- `github.auth_failed`;
- `github.forbidden`;
- `github.not_found`;
- `github.rate_limited`;
- `github.validation_failed`;
- `github.conflict`;
- `github.server_error`;
- `github.timeout`;
- `github.transport_unclassified`.

The implementation prompt should choose a precise policy for ambiguous transport failures: either classified `ProviderFailed` with a stable code, or returned `WorkflowOsError` with no lifecycle transition. The plan allows either, but implementation should not leave it implicit.

## 8. Privacy and Redaction Assessment

The plan properly forbids storing or outputting:

- tokens;
- authorization headers;
- raw request bodies;
- raw response bodies;
- raw provider payloads;
- GitHub file contents;
- environment variable values;
- credentials;
- private keys;
- command output;
- CI logs;
- parser payloads;
- secret-like values.

The Debug redaction requirements are appropriate for auth, comment body, provider reference, target identity, and redaction metadata.

## 9. Test Plan Assessment

The future tests cover the important safety boundary:

- concrete provider implements the provider trait;
- success maps to `ProviderSucceeded`;
- stable provider reference is bounded;
- provider failure classes are stable and non-leaking;
- raw request/response payloads are not stored;
- Debug output is redaction-safe;
- provider client does not append events, write side-effect records, write artifacts, or emit CLI output;
- hidden auth discovery does not occur;
- existing provider-call orchestration tests continue to pass.

Non-blocking improvement: include an explicit test that the concrete provider does not read process environment variables, even when a tempting token-shaped variable is present.

## 10. Documentation Review

The plan and roadmap state that:

- concrete provider client/auth loading is planned, not implemented;
- the first concrete client should use explicit caller-supplied auth;
- hidden credential discovery remains deferred;
- executor integration is deferred;
- CLI behavior, schemas, examples, hosted behavior, report artifact writes, workflow event append, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes remain unsupported.

## 11. Planning Blockers

No planning blockers.

## 12. Non-Blocking Follow-Ups

- In the implementation prompt, choose the ambiguous transport failure policy explicitly.
- Define the exact provider comment reference shape before persisting live references broadly.
- Add an explicit no-environment-token-discovery test in the concrete client phase.
- Keep remote-success/local-transition-failure reconciliation as a required plan before executor-integrated live writes.

## 13. Recommended Next Phase

Recommended next phase: concrete GitHub PR comment provider client implementation, explicit injected transport only.

The implementation must not add executor integration, CLI behavior, hidden auth discovery, schemas, examples, report artifact writes, workflow event append, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 14. Validation

Commands run:

- `npm run check:docs` - passed.

Governed review phase:

- workflow: `dg/review`;
- run: `run-1783281994635454000-2`;
- approval: `approval/run-1783281994635454000-2/review-scope-approved`;
- approval reason: `delegated-maintainer-approved-provider-client-auth-plan-review`;
- status: `Completed`;
- terminal: true;
- events total: 39;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.

Out-of-kernel work disclosed:

- review documentation edits;
- roadmap status update;
- docs validation command execution;
- no skipped required checks;
- no implementation;
- no report artifact was written by the kernel for this review phase.
