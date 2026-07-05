# GitHub PR Comment Report Artifact Write Composition Hardening Report

## 1. Executive Summary

Implemented the accepted non-blocking hardening follow-ups for the GitHub PR comment report artifact write composition helper.

This phase added regression coverage for composition-level identity mismatch mapping, required accepted-event failure mapping, and artifact store write failure mapping. It did not change runtime behavior or broaden the helper.

## 2. Scope Completed

- Added composition-level identity mismatch regression coverage.
- Added composition-level missing accepted-event regression coverage.
- Added composition-level artifact store failure mapping coverage.
- Verified failed composition paths do not write artifacts.
- Verified mapped errors remain stable and non-leaking.
- Updated roadmap/documentation posture for the completed hardening pass.

## 3. Scope Explicitly Not Completed

- No provider mutation.
- No GitHub PR comment creation.
- No live GitHub writes.
- No runtime side-effect execution.
- No attempted/completed/failed lifecycle behavior.
- No automatic event append.
- No automatic report artifact writing from default executor paths.
- No CLI behavior.
- No schemas.
- No examples.
- No hosted behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Tests Added

Added focused tests in `crates/workflow-core/tests/work_report.rs` covering:

- `github_pr_comment_report_artifact_write_composition_maps_identity_mismatch`;
- `github_pr_comment_report_artifact_write_composition_requires_accepted_event_before_write`;
- `github_pr_comment_report_artifact_write_composition_maps_artifact_store_failure`.

The tests pin the composition helper's public error mapping and no-write-on-failure behavior without creating provider payloads, comment bodies, live GitHub state, or synthetic evidence.

## 5. Error Mapping Summary

The hardened coverage verifies that:

- run/artifact identity mismatch maps to `github_pr_comment_report_artifact_write.identity_mismatch`;
- missing accepted proposed-event evidence maps to `github_pr_comment_report_artifact_write.citation_invalid`;
- artifact-store write failure maps to `github_pr_comment_report_artifact_write.artifact_write_failed`.

The lower-level error payloads are intentionally collapsed at the composition boundary to avoid leaking SideEffect IDs, run IDs, target references, provider payloads, local paths, tokens, or secret-like values.

## 6. Redaction And Privacy Summary

The phase remains reference-only and local.

The tests include secret-like failure text in a fake artifact store and verify the mapped composition error does not leak it. No raw GitHub provider payloads, pull request bodies, diffs, CI logs, generated comment bodies, command output, credentials, tokens, or authorization headers are copied or emitted.

## 7. Dogfood Governance

- Workflow: `dg/implement`.
- Run ID: `run-1783221298223084000-2`.
- Approval ID: `approval/run-1783221298223084000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer.
- Event summary: 39 events; `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Retries: 0.
- Escalations: 0.
- Kernel role: governance boundary and approval/event trail.
- Executor role: Codex performed test edits, docs updates, validation, git, and PR actions outside the kernel.
- Out-of-kernel work disclosed: repo edits, shell commands, validation commands, git/PR actions, and this report update.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test work_report github_pr_comment_report_artifact_write_composition` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- The helper remains explicit and local only.
- It does not make default executor paths write artifacts.
- It does not call GitHub or create comments.
- It does not validate accepted-event ordering against a targeted skill invocation.
- Attempted/completed/failed lifecycle behavior remains deferred.

## 10. Recommended Next Phase

GitHub PR comment report artifact write composition hardening review.

After review, the next runtime-oriented phase should remain narrow and explicit. Do not proceed to live provider mutation, automatic artifact writes, CLI mutation commands, schemas, examples, hosted behavior, reasoning lineage, or release posture changes without a separate reviewed plan.
