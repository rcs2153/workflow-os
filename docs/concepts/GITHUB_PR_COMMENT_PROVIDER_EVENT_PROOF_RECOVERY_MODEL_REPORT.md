# GitHub PR Comment Provider Event-Proof Recovery Model Report

## 1. Executive Summary

The GitHub PR comment provider event-proof recovery model/helper is implemented as a local classification boundary.

The helper classifies explicit provider disclosure posture into bounded recovery posture, next-action vocabulary, retry-blocking posture, artifact-write allowance, and operator-action posture. It gives callers a safe answer when strict report artifact event-proof gates deny artifact writes because durable workflow event proof is missing, mismatched, or ambiguous.

This phase does not implement provider lookup, automatic repair, workflow event append, report artifact writes, CLI behavior, schemas, examples, hosted behavior, approval-presentation enforcement, or release posture changes.

## 2. Scope Completed

- Added `GitHubPullRequestCommentProviderEventProofRecoveryPosture`.
- Added `GitHubPullRequestCommentProviderEventProofRecoveryNextAction`.
- Added `GitHubPullRequestCommentProviderEventProofRecoveryInput`.
- Added `GitHubPullRequestCommentProviderEventProofRecoveryResult`.
- Added `classify_github_pr_comment_provider_event_proof_recovery`.
- Exported the recovery API from `workflow-core`.
- Added focused recovery tests.
- Updated the event-proof recovery plan status.
- Updated `ROADMAP.md`.

## 3. Scope Explicitly Not Completed

- No provider calls.
- No GitHub lookup or query reconciliation.
- No automatic retries.
- No automatic repair.
- No workflow event append.
- No audit or observability emission.
- No report artifact writes.
- No automatic report generation.
- No automatic report artifact writing.
- No default executor behavior changes.
- No CLI behavior.
- No workflow schema changes.
- No examples.
- No hosted or distributed runtime behavior.
- No broader write-capable adapters.
- No reasoning lineage.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy expansion.
- No approval-presentation enforcement.
- No release posture change.

## 4. Model And Helper Summary

The recovery input accepts:

- optional bounded `GitHubPullRequestCommentProviderWriteReportDisclosure`;
- explicit `event_proof_mismatch` posture supplied by the caller;
- `WorkReportSensitivity`;
- validated `RedactionMetadata`.

The recovery result exposes:

- recovery posture;
- next-action code;
- retry-blocked posture;
- artifact-write-may-proceed posture;
- operator-action-required posture;
- redaction and sensitivity metadata.

The helper is pure classification. It does not inspect GitHub, query providers, append missing events, repair state, mutate side-effect records, write artifacts, or alter workflow pass/fail semantics.

## 5. Recovery Posture Summary

Implemented recovery postures include:

- `event_proof_present`;
- `event_proof_missing`;
- `event_proof_mismatch`;
- `provider_not_called`;
- `reconciliation_required`;
- `reconciliation_unavailable`;
- `provider_response_ambiguous`;
- `local_transition_failed`;
- `local_state_ambiguous`;
- `unsupported_posture` vocabulary for future unsupported first-slice inputs.

## 6. Next-Action Summary

Implemented next-action vocabulary includes:

- `no_action_required`;
- `inspect_workflow_events`;
- `inspect_side_effect_record`;
- `inspect_reconciliation_candidate`;
- `manual_provider_lookup_required`;
- `manual_state_repair_required`;
- `retry_blocked_pending_reconciliation`;
- `artifact_write_blocked_pending_event_proof`.

These are labels only. They do not authorize command execution, provider calls, state repair, event append, artifact writes, retries, CLI behavior, schemas, hosted behavior, or release posture changes.

## 7. Validation Boundary Summary

Validation ensures:

- redaction metadata is bounded and non-secret-like through existing WorkReport redaction validation;
- artifact write may proceed only for `event_proof_present`;
- `no_action_required` cannot be paired with retry-blocking or operator-action posture;
- invalid serialized result shapes fail closed through custom deserialization;
- deserialization errors do not include raw redaction metadata values.

## 8. Privacy And Redaction Summary

The recovery model stores no provider payloads, comment bodies, PR bodies, diffs, review threads, file contents, authorization headers, tokens, credentials, environment values, CI logs, command output, parser payloads, raw specs, repository paths, URLs, or unbounded operator notes.

`Debug` output redacts redaction metadata and exposes only bounded posture fields. Serialization does not include provider references, side-effect IDs, workflow event IDs, run IDs, paths, provider payloads, or comment text.

## 9. Test Coverage Summary

Focused tests cover:

- event-proof-present classification;
- missing event proof classification;
- mismatched event proof classification;
- provider-not-called classification;
- reconciliation-required classification;
- reconciliation-unavailable classification;
- ambiguous provider response classification;
- local transition failure classification;
- local-state ambiguity classification;
- missing disclosure classification;
- provider call, workflow event append, and artifact write non-capabilities;
- secret-like redaction rejection without leakage;
- redaction-safe `Debug` and serialization behavior;
- serde round trip;
- invalid serialized result failure.

## 10. Governed Dogfood Summary

- Workflow: `dg/runtime-composition`.
- Run: `run-1783557005534714000-2`.
- Approval: `approval/run-1783557005534714000-2/composition-approved`.
- Approval outcome: granted under delegated maintainer authority after the complete approval handoff block was emitted.
- Phase close: completed with 39 events, one approval, zero retries, and zero escalations.

## 11. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase runtime-composition ...`: passed.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /var/folders/r9/y7_mqmq108z94yhyt702h2b80000gn/T/workflow-os-self-governance-state --mock-all-local-skills approve run-1783557005534714000-2 approval/run-1783557005534714000-2/composition-approved --actor user/delegated-maintainer --reason approved-event-proof-recovery-model-helper-scope`: passed.
- `cargo test -p workflow-core --test work_report github_pr_comment_provider_event_proof_recovery`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783557005534714000-2 --phase runtime-composition`: passed.

## 12. Remaining Known Limitations

- No provider lookup/query reconciliation exists.
- No automatic event repair exists.
- No manual state repair helper exists.
- No workflow event append occurs from recovery classification.
- No report artifact writes occur from recovery classification.
- No CLI recovery display or command exists.
- No schema-declared provider artifact policy exists.
- Approval-presentation enforcement remains a separate P0 hardening gap.

## 13. Recommended Next Phase

Recommended next phase: GitHub PR comment provider event-proof recovery model/helper review.

The review should verify classification semantics, retry-blocking posture, artifact-write allowance, redaction safety, serialization behavior, tests, and preservation of all non-goals before any provider lookup, event repair, artifact write composition, CLI, schema, example, hosted behavior, broader write adapter, reasoning lineage, approval-presentation enforcement, or release posture work.
