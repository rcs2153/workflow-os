# GitHub PR Comment Report Artifact Write Composition Hardening Review

## 1. Executive Verdict

Hardening accepted; proceed to broader artifact-write integration planning.

The hardening pass closes the non-blocking coverage gaps identified in the composition helper review without broadening runtime behavior. The new tests pin stable error mapping, no-write-on-failure behavior, and non-leaking artifact-store failure handling at the composition boundary.

## 2. Scope Verification

The hardening stayed within the approved follow-up scope.

Confirmed absent:

- provider mutation;
- live GitHub PR comment creation;
- runtime side-effect execution;
- attempted/completed/failed lifecycle behavior;
- automatic report artifact writes from default executor paths;
- CLI behavior;
- schema changes;
- example updates;
- hosted behavior;
- reasoning lineage;
- release posture changes.

The phase added tests, one phase report, and a roadmap status update only.

## 3. Hardening Assessment

The hardening targeted the exact gaps identified in the prior maintainer review:

- composition-level identity mismatch error mapping;
- composition-level artifact store failure mapping;
- composition-level missing accepted-event failure.

This is the right level of hardening. It tests the public composition helper behavior rather than duplicating every lower-level citation helper test.

## 4. Error Mapping Assessment

The added tests verify stable, bounded error codes:

- run/artifact mismatch maps to `github_pr_comment_report_artifact_write.identity_mismatch`;
- missing accepted proposed-event evidence maps to `github_pr_comment_report_artifact_write.citation_invalid`;
- artifact store failure maps to `github_pr_comment_report_artifact_write.artifact_write_failed`.

The composition helper continues to collapse lower-level details into bounded public error classes. That is appropriate for this provider-write-adjacent boundary because callers should not receive raw SideEffect IDs, run IDs, target references, provider payloads, filesystem paths, or secret-like store failure text.

## 5. No-Write-On-Failure Assessment

The hardening preserves no-write-on-failure behavior.

Verified:

- identity mismatch does not persist an artifact;
- missing accepted event does not persist an artifact;
- artifact-store failure is attempted only after citation and governed artifact gates pass.

The artifact-store failure test uses a fake store and confirms the mapped error is non-leaking. It does not create a filesystem artifact or provider write.

## 6. Privacy And Redaction Assessment

The hardening remains redaction-safe.

Verified:

- error messages do not leak the raw `run-123` fixture ID in the identity mismatch path;
- error messages do not leak `github-pr-comment` from the missing accepted-event path;
- artifact-store failure mapping does not leak the secret-like `sk-secret` marker;
- no generated comment body, GitHub payload, pull request body, diff, CI log, command output, raw record JSON, token, credential, or authorization header is copied.

The fake artifact-store failure intentionally includes secret-like text, which is a useful regression guard for the mapper.

## 7. Test Quality Assessment

The test coverage is now stronger at the composition boundary.

Added tests cover:

- identity mismatch mapping;
- accepted-event requirement failure;
- artifact-store failure mapping;
- no artifact write on pre-store failures;
- non-leaking mapped errors.

Existing tests still cover:

- successful citation validation followed by artifact write;
- persisted artifact readback;
- missing GitHub citation failure before artifact write;
- approval-linkage failure before artifact write;
- bounded input Debug output.

No test appears shallow or misleading. Remaining deferred coverage is appropriately later-stage: target-step or target-skill ordering validation once the helper accepts explicit target context, and attempted/completed/failed lifecycle behavior once provider write lifecycle semantics are designed.

## 8. Documentation Review

Documentation now states:

- the explicit local composition helper is implemented;
- the helper review accepted the phase with non-blocking hardening follow-ups;
- the hardening pass is implemented and documented;
- the next work should review hardening before broader integration;
- provider mutation, runtime side-effect execution, automatic artifact writes, CLI mutation behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes remain unsupported.

The hardening report includes scope completed, explicit non-scope, tests added, error mapping, privacy posture, dogfood governance, validation commands, limitations, and next recommendation.

## 9. Dogfood Governance

- Workflow: `dg/review`.
- Run ID: `run-1783222045544139000-2`.
- Approval ID: `approval/run-1783222045544139000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer.
- Event summary: 39 events; `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Retries: 0.
- Escalations: 0.
- Kernel role: governance boundary and approval/event trail.
- Executor role: Codex performed the review document update and validation outside the kernel.
- Out-of-kernel work disclosed: docs edits, validation command, git/PR actions, and this review update.

## 10. Validation

For the hardening implementation:

- `cargo test -p workflow-core --test work_report github_pr_comment_report_artifact_write_composition` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

For this review phase:

- `npm run check:docs` - passed.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Consider target-step or target-skill ordering validation once the helper accepts explicit target context.
- Keep attempted/completed/failed GitHub PR comment lifecycle behavior separate from this proposed-write artifact handoff lane.
- Keep live provider mutation behind a separate reviewed plan and fixture/live opt-in boundary.

## 13. Recommended Next Phase

Broader artifact-write integration planning.

Reason: the explicit local helper and its hardening coverage are now accepted. The next useful phase should decide how, if at all, this validated artifact-write composition plugs into higher-level opt-in runtime or report-artifact paths without making provider writes live, automatic, or CLI-exposed.
