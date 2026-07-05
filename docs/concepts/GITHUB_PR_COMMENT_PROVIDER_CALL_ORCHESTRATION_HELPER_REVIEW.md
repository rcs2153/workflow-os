# GitHub PR Comment Provider-Call Orchestration Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The injected-provider orchestration helper is appropriately scoped. It connects the already-reviewed provider-call request boundary to store-backed `SideEffectRecord` lifecycle transitions without introducing a concrete GitHub client, auth loading, executor write behavior, workflow event append, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved helper scope.

Implemented:

- explicit provider-call orchestration input;
- explicit provider-call orchestration result;
- helper that invokes only a caller-supplied provider trait;
- completed transition for classified provider success;
- failed transition for classified provider failure;
- stable failures for unclassified provider errors and unsupported response outcomes;
- focused provider-write tests;
- roadmap and plan updates;
- phase report.

No accidental implementation found for:

- concrete GitHub network client;
- auth loading from environment, keychain, GitHub CLI, git remote, config, or hidden global state;
- automatic provider writes;
- executor-integrated write path;
- workflow event append;
- audit event emission;
- report artifact write;
- CLI mutation command;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 3. Helper API Assessment

The API is narrow and explicit.

`GitHubPullRequestCommentProviderCallOrchestrationInput` wraps the existing validated provider-call input and adds only lifecycle transition timestamp, transition references, and evidence reference count.

`orchestrate_github_pr_comment_provider_call` requires:

- a `SideEffectRecordStore`;
- an injected `GitHubPullRequestCommentProvider`;
- explicit orchestration input.

It does not read hidden global state, discover credentials, construct a network client, mutate workflow runs, append events, write artifacts, or expose CLI output.

## 4. Pre-Call Gate Assessment

The helper reuses `GitHubPullRequestCommentProviderCallRequest::new`, so the existing gate remains active:

- attempted `SideEffectRecord` required;
- target must match;
- idempotency must match;
- mode must be `LiveSandbox`;
- live call must be explicitly enabled;
- provider call must be explicitly enabled;
- explicit auth wrapper must validate;
- comment body, summary, sensitivity, and redaction metadata must validate.

The helper also loads the stored side-effect record by stable `SideEffectId` and verifies the stored target/idempotency still match before provider invocation.

## 5. Provider Invocation Assessment

The helper invokes only the supplied `GitHubPullRequestCommentProvider` trait. There is no built-in provider client, retry loop, auth loading, environment access, or transport behavior.

Classified provider responses are handled deterministically:

- `ProviderSucceeded` requires a provider comment reference and transitions to completed;
- `ProviderFailed` requires a bounded provider error code and transitions to failed;
- `FixtureValidated` and `DryRunValidated` are rejected for this path;
- provider trait errors are mapped to `github_pr_comment_provider.call_unclassified`.

## 6. Lifecycle Transition Assessment

The helper delegates lifecycle changes to existing store-backed side-effect transition helpers. That preserves the existing validation boundary for attempted-to-completed and attempted-to-failed transitions.

The helper correctly leaves workflow event append and report artifact write as separate boundaries. The result exposes `workflow_event_appended()` and `report_artifact_written()` as false.

## 7. Error-Handling Assessment

Errors are stable and non-leaking.

Reviewed error paths include:

- store read failure;
- missing side-effect record;
- attempted-record mismatch;
- provider-call gate failure;
- unclassified provider failure;
- unsupported non-provider response outcome;
- missing/invalid provider success reference;
- missing provider failure code;
- lifecycle transition failure mapping.

The implementation does not include raw provider payloads, auth material, comment body, token-like values, target details, or path-like values in error messages.

## 8. Privacy and Redaction Assessment

The helper uses existing validated constructors and redaction-safe Debug implementations.

No evidence found that the helper stores or outputs:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw GitHub file contents;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- secret-like metadata.

Debug tests cover non-leakage for auth, comment body, provider references, and stable side-effect identifiers.

## 9. Test Quality Assessment

The tests cover the important helper behavior:

- provider success transitions attempted record to completed;
- classified provider failure transitions attempted record to failed;
- unclassified provider errors do not transition;
- fixture/dry-run responses do not transition;
- pre-call gate failure does not invoke the provider;
- idempotency mismatch is rejected before provider invocation;
- Debug output does not leak sensitive values;
- provider-call request tests continue to pass.

Existing broader provider-write tests, side-effect tests, executor tests, work-report tests, and workspace tests passed.

Non-blocking test improvement: rename `provider_call_orchestration_store_record_mismatch_fails_without_provider_invocation` or add a separate true store-record mismatch test. The current test exercises request idempotency mismatch before provider invocation; the behavior is valuable, but the name is slightly imprecise.

## 10. Documentation Review

Documentation now states that:

- the injected provider-call orchestration helper is implemented;
- the provider trait and explicit auth wrapper exist;
- concrete GitHub clients are not implemented;
- auth loading is not implemented;
- executor writes are not implemented;
- event append and report artifact writes remain separate reviewed boundaries;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes remain unsupported.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add a true stored-record mismatch test, or rename the existing idempotency mismatch orchestration test for precision.
- Before any concrete GitHub client, plan provider-call reconciliation for the case where the remote provider succeeds but local lifecycle transition fails.
- Before any concrete GitHub client, decide whether provider-native idempotency is available or whether local-only duplicate prevention must remain a documented limitation.
- Review whether provider references should use a stricter prefix convention before they become persisted externally visible write evidence.

## 13. Recommended Next Phase

Recommended next phase: concrete provider client/auth loading planning.

Do not implement the concrete client yet. The next plan should define how a GitHub client and explicit auth source can be introduced without hidden credential discovery, default executor writes, automatic event append, report artifact writes, CLI mutation behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.

## 14. Validation

Commands run:

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test provider_write` - passed, 78 tests.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

Governed review phase:

- workflow: `dg/review`;
- run: `run-1783281274228939000-2`;
- approval: `approval/run-1783281274228939000-2/review-scope-approved`;
- approval reason: `delegated-maintainer-approved-provider-call-orchestration-review`;
- status: `Completed`;
- terminal: true;
- events total: 39;
- approvals: 1;
- retries: 0;
- escalations: 0;
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.

Out-of-kernel work disclosed:

- review documentation edits;
- local validation command execution;
- no skipped required checks;
- no implementation fixes;
- no report artifact written by the kernel for this review phase.
