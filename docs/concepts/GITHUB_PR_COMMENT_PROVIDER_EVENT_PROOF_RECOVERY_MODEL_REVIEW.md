# GitHub PR Comment Provider Event-Proof Recovery Model Review

## 1. Executive Verdict

Phase accepted; proceed to provider lookup/query reconciliation planning.

The GitHub PR comment provider event-proof recovery model/helper stays within the approved local-classification scope. It gives Workflow OS a bounded answer when strict report artifact gates reject provider disclosures because durable workflow event proof is missing, mismatched, or ambiguous.

No blocker fixes are required before planning the next recovery layer.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- local recovery posture vocabulary;
- local next-action vocabulary;
- explicit recovery input/result types;
- pure recovery classification helper;
- redaction-safe validation, `Debug`, serialization, and deserialization behavior;
- focused tests;
- documentation and implementation report.

Not implemented:

- provider calls;
- GitHub lookup/query reconciliation;
- automatic retry;
- workflow event append;
- audit event emission;
- observability emission;
- side-effect record mutation;
- report artifact writes;
- automatic report generation;
- default executor behavior changes;
- CLI behavior;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- broader write-capable adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- approval-presentation enforcement;
- release posture changes.

No accidental runtime capability expansion was found.

## 3. Model Assessment

The model is appropriately narrow and domain-specific to the currently implemented GitHub PR comment provider recovery lane.

The implementation adds:

- `GitHubPullRequestCommentProviderEventProofRecoveryPosture`;
- `GitHubPullRequestCommentProviderEventProofRecoveryNextAction`;
- `GitHubPullRequestCommentProviderEventProofRecoveryInput`;
- `GitHubPullRequestCommentProviderEventProofRecoveryResult`;
- `classify_github_pr_comment_provider_event_proof_recovery`.

The result model exposes the right bounded fields:

- recovery posture;
- next action;
- retry-blocking posture;
- artifact-write allowance;
- operator-action posture;
- sensitivity;
- redaction metadata.

The helper is exported from `workflow-core` consistently with adjacent WorkReport and provider disclosure APIs.

## 4. Classification Assessment

The classification matrix matches the accepted recovery plan.

Verified behavior:

- provider/local completed or failed with event proof maps to `event_proof_present`;
- provider/local completed or failed without event proof maps to `event_proof_missing`;
- caller-supplied event-proof mismatch maps to `event_proof_mismatch`;
- provider-not-called disclosure maps to `provider_not_called`;
- reconciliation-required disclosure maps to `reconciliation_required`;
- reconciliation-unavailable disclosure or missing disclosure maps to `reconciliation_unavailable`;
- ambiguous provider response maps to `provider_response_ambiguous`;
- provider outcome plus local transition failure maps to `local_transition_failed`;
- ambiguous local lifecycle state maps to `local_state_ambiguous`.

The only posture that permits artifact write is `event_proof_present`. That is the correct fail-closed boundary.

Retry remains blocked for missing event proof, mismatched event proof, reconciliation gaps, ambiguous provider response, local transition failure, and local state ambiguity. `provider_not_called` requires operator action and blocks artifact write, but does not mark retry blocked because no provider mutation is known to have occurred. That is consistent with the current plan and avoids overstating split-brain risk.

## 5. Source-Of-Truth Assessment

The implementation preserves workflow events as the durable event-proof source.

The helper accepts bounded disclosure posture and an explicit mismatch flag. It does not treat provider references, report text, operator notes, or serialized recovery output as proof that a provider event occurred.

The result can guide an operator or future report surface, but it does not become a workflow event, audit event, side-effect lifecycle transition, or artifact write authorization.

## 6. Error-Handling Assessment

Validation errors use stable non-leaking codes:

- `github_pr_comment_provider_event_proof_recovery.invalid_input`;
- `github_pr_comment_provider_event_proof_recovery.event_mismatch`;
- `github_pr_comment_provider_event_proof_recovery.reconciliation_invalid`;
- `github_pr_comment_provider_event_proof_recovery.unsupported_posture`;
- `github_pr_comment_provider_event_proof_recovery.redaction_invalid`.

The current helper path primarily exercises `invalid_input` and `redaction_invalid`; the extra codes are acceptable future vocabulary and do not create behavior claims.

Deserialization fails closed through `GitHubPullRequestCommentProviderEventProofRecoveryResult::new` and emits a generic serde error rather than raw caller-supplied metadata.

No misleading user-project diagnostic behavior was introduced.

## 7. Privacy And Redaction Assessment

The implementation is redaction-safe for this phase.

Verified:

- redaction metadata is validated through existing WorkReport redaction validation;
- `Debug` for input and result redacts redaction metadata;
- result serialization stores bounded posture fields and validated redaction metadata only;
- deserialization errors do not echo secret-like metadata values;
- the helper stores no provider payloads, comment bodies, PR bodies, diffs, review threads, file contents, authorization headers, tokens, credentials, environment values, CI logs, command output, parser payloads, raw specs, repository paths, private URLs, or unbounded operator notes.

The tests include secret-like redaction rejection and non-leakage assertions.

## 8. Test Quality Assessment

The test coverage is appropriate for the local-classification boundary.

Covered:

- event-proof-present classification;
- missing event proof;
- mismatched event proof;
- provider not called;
- reconciliation required;
- reconciliation unavailable;
- ambiguous provider response;
- local transition failure;
- local state ambiguity;
- missing disclosure;
- artifact-write allowance only for event-proof-present;
- retry-blocking posture for ambiguous and split-brain states;
- provider-call, workflow-event-append, and report-artifact-write non-capabilities;
- secret-like redaction rejection;
- redaction-safe `Debug`;
- redaction-safe serialization;
- serde round trip;
- invalid serialized result failure.

No blocker test gaps were found.

Non-blocking future test improvement: when provider lookup/query reconciliation is planned, add integration tests that demonstrate lookup output remains separate from durable workflow event proof until a separately approved event append path exists.

## 9. Documentation Review

Documentation is honest about the implemented and deferred scope.

Verified:

- `docs/implementation-plans/github-pr-comment-provider-event-proof-recovery-plan.md` marks the first model/helper as implemented;
- `docs/concepts/GITHUB_PR_COMMENT_PROVIDER_EVENT_PROOF_RECOVERY_MODEL_REPORT.md` documents completed scope, non-scope, recovery posture, next-action vocabulary, validation, privacy, tests, commands, limitations, and recommended review;
- `ROADMAP.md` links the implemented recovery planning and model report;
- docs do not claim provider lookup, event repair, artifact writing, CLI recovery, schemas, examples, hosted runtime, reasoning lineage, or release posture changes.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Consider linking this review from `ROADMAP.md` in a future documentation sweep.
- Plan provider lookup/query reconciliation as an explicit separate phase with caller-supplied auth, injected transport, no automatic retry, and no event append.
- Plan manual state repair only after provider lookup/reconciliation semantics are reviewed.
- Keep artifact write composition blocked until durable event proof or an explicitly reviewed disclosure-only artifact policy exists.

## 12. Recommended Next Phase

Recommended next phase: provider lookup/query reconciliation planning.

Why: the current helper safely classifies missing, mismatched, and ambiguous proof, but it cannot determine whether a provider-side comment exists after an ambiguous or split-brain outcome. The next safe step is planning an explicit, injected-transport lookup/query reconciliation boundary that can gather bounded provider-side evidence without writing, retrying, appending workflow events, repairing state, exposing CLI behavior, or changing workflow semantics.

Do not proceed directly to automatic repair, artifact write composition, CLI recovery commands, schemas, examples, hosted behavior, broader write-capable adapters, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 13. Validation

Validation run for this review:

- `cargo test -p workflow-core --test work_report github_pr_comment_provider_event_proof_recovery`: passed.
- `cargo fmt --all --check`: passed.
- `npm run check:docs`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.

The governed review phase used `dg/review` and was approved under delegated maintainer authority after the complete approval handoff block was emitted and preserved.
