# GitHub PR Comment Provider Lookup Recovery Integration Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation stays within the approved in-memory helper scope. It composes the existing injected provider lookup reconciliation helper with the existing event-proof recovery classifier and returns bounded recovery posture without adding provider writes, automatic lookup, hidden auth loading, retries, repair, event append, side-effect record mutation, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Confirmed in scope:

- `GitHubPullRequestCommentProviderLookupRecoveryIntegrationInput` was added as an explicit composition input.
- `GitHubPullRequestCommentProviderLookupRecoveryIntegrationResult` was added as a bounded result.
- `integrate_github_pr_comment_provider_lookup_recovery` was added and exported.
- Focused tests were added for lookup/recovery composition, retry/artifact/operator posture, non-mutation flags, error mapping, and Debug/serialization safety.
- The roadmap, lookup integration plan, and phase report were updated.

No accidental implementation was found for:

- provider writes;
- automatic lookup;
- hidden or ambient auth loading;
- retries or backoff;
- state repair;
- workflow event append from recovery;
- side-effect record mutation;
- report artifact writes;
- CLI/operator lookup behavior;
- schemas or examples;
- hosted behavior;
- reasoning lineage;
- approval-presentation enforcement;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately narrow.

It accepts:

- explicit lookup reconciliation input;
- an injected `GitHubPullRequestCommentProviderLookupClient`;
- explicit event-proof recovery context.

It returns:

- the existing lookup reconciliation result;
- the existing event-proof recovery result;
- composed retry-blocked, artifact-write, and operator-action posture;
- explicit non-mutation posture accessors.

The API does not read hidden global state, does not load credentials, and does not require a state backend, runtime executor, artifact store, CLI surface, or provider write path.

## 4. Lookup And Recovery Behavior Assessment

Lookup behavior remains explicit and injected. The helper delegates to `reconcile_github_pr_comment_provider_lookup`, so all existing lookup validation, redaction, target matching, auth validation, and response classification remain the enforcement boundary.

Recovery behavior remains classification-only. The helper delegates to `classify_github_pr_comment_provider_event_proof_recovery`, so event-proof posture, retry blocking, artifact-write allowance, and operator-action requirements remain aligned with the existing recovery model.

The implementation correctly does not treat a provider lookup observation as durable workflow event proof. This is important: a remote comment can inform operator recovery, but strict report artifact gates still require accepted local workflow event proof.

## 5. Retry And Artifact Posture Assessment

The composed result is conservative:

- retry is blocked when either lookup reconciliation or recovery classification blocks retry;
- artifact writes are blocked unless lookup posture and recovery posture both allow proceeding;
- operator action is required when either composed posture requires operator review.

The tests cover remote observed and remote absent lookup cases. The remote observed case remains retry-blocked and artifact-write-blocked. The remote absent case permits lookup-level retry reevaluation but remains blocked by recovery posture when recovery context is unavailable. This is the right current behavior.

## 6. Workflow Semantics And Mutation Assessment

The helper does not mutate workflow state or provider state.

Verified by API shape and tests:

- provider lookup may be performed through the injected lookup client;
- provider write is not performed;
- workflow events are not appended;
- side-effect records are not mutated;
- report artifacts are not written;
- CLI output is not emitted.

The helper returns an in-memory result only and does not change executor semantics.

## 7. Privacy And Redaction Assessment

Privacy posture is acceptable for this phase.

The helper composes existing validated types and maps recovery classifier errors to stable, non-leaking error codes. Debug output uses existing redacted nested Debug implementations and adds explicit boolean posture rather than raw payloads.

No storage or copying was found for:

- raw GitHub provider payloads;
- comment bodies;
- PR bodies or diffs;
- review-thread payloads;
- CI logs;
- command output;
- source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, or private keys.

The focused error test confirms secret-like recovery redaction metadata is rejected without leaking the raw field value.

## 8. Test Quality Assessment

Test coverage is strong for a first integration helper.

Covered:

- remote observed lookup composed with recovery classification;
- remote absent lookup composed with recovery classification;
- retry/artifact/operator posture;
- no provider writes, workflow event append, side-effect mutation, report artifact writes, or CLI output;
- recovery classification failure mapping without leakage;
- Debug and serialization non-leakage;
- existing provider-write lookup and recovery model tests through the workspace suite.

Non-blocking test follow-up:

- Add a focused test where recovery posture contains accepted event proof and lookup posture does not block artifact writes, if/when such a combination becomes valid for this integration boundary.
- Add table-style coverage for unauthorized, unavailable, rate-limited, ambiguous, and untrusted lookup postures at the integration layer if the helper becomes a user-facing operator surface.

## 9. Documentation Review

Documentation is honest and aligned.

The roadmap and lookup integration plan now state that the first in-memory lookup recovery integration helper is implemented. They also continue to state that automatic provider lookup, hidden auth loading, provider writes, automatic retries, repair, event append, artifact writes, CLI behavior, schemas, examples, hosted behavior, approval-presentation enforcement, and release posture changes remain unimplemented.

The phase report includes completed scope, explicit non-scope, helper API summary, lookup/recovery behavior, privacy posture, validation results, dogfood run details, limitations, and recommended next phase.

## 10. Validation

Implementation-phase validation passed before this review:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p workflow-core --test provider_write`
- `cargo test --workspace`
- `npm run check:docs`

Review-phase validation also passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run dogfood:benchmark -- phase-close run-1783569293780147000-2 --phase review`

## 11. Governed Dogfood Review Run

- workflow_id: `dg/review`
- run_id: `run-1783569293780147000-2`
- approval_id: `approval/run-1783569293780147000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-provider-lookup-recovery-helper-review-scope`
- event summary: 39 events total; 1 approval; 0 retries; 0 escalations.
- event kinds: ApprovalGranted, ApprovalRequested, PolicyDecisionRecorded, RunCompleted, RunCreated, RunResumed, RunStarted, RunValidated, SkillInvocationRequested, SkillInvocationStarted, SkillInvocationSucceeded, StepScheduled.
- out-of-kernel work disclosed: review artifact writing, shell validation commands, git/PR actions, and report posture were performed by the executor outside the kernel.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add integration-level table tests for additional lookup postures before exposing an operator-facing recovery command.
- Plan CLI/operator lookup recovery exposure separately, preserving explicit auth and no hidden lookup.
- Keep manual state repair as a separately planned phase with durable event-proof and approval-presentation requirements.
- Keep live lookup smoke testing opt-in and separate from this helper.

## 14. Recommended Next Phase

Recommended next phase: **provider lookup operator recovery planning**.

The helper is accepted, but the next useful product/runtime step should not be automatic repair or automatic retry. The next step should define the operator-facing recovery path: how a maintainer explicitly supplies or triggers lookup context, sees bounded recovery posture, understands why artifact writes remain blocked or allowed, and chooses a later repair/retry path without hidden auth, implicit provider writes, or event fabrication.
