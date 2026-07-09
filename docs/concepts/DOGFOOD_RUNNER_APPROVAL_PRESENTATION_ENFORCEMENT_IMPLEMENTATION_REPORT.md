# Dogfood Runner Approval-Presentation Enforcement Implementation Report

## 1. Executive Summary

This phase implemented repo-local dogfood approval-presentation enforcement. Material dogfood `phase-start` runs already persisted bounded approval-presentation proof; they now print a proof-enforced approval command that passes the persisted `presentation_id` into a hidden dogfood CLI path backed by `LocalExecutor::decide_approval_with_presentation(...)`.

Default public approval behavior remains unchanged. Approval is still explicit. The implementation does not add automatic approval, public approval-card UI, schemas, examples, provider writes, side effects, report artifact writes, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added hidden CLI command `workflow-os dogfood approval-presentation approve`.
- Wired the hidden command to `LocalExecutor::decide_approval_with_presentation(...)`.
- Updated the dogfood benchmark runner to print proof-enforced approval commands after proof persistence.
- Added focused helper test coverage for approval command shape.
- Updated roadmap, runbook, and planning documentation.
- Added this end-of-phase report.

## 3. Scope Explicitly Not Completed

- No automatic approval.
- No hidden approval.
- No public `workflow-os approve` behavior change.
- No public approval-card UI.
- No workflow schema fields.
- No examples.
- No provider writes.
- No side effects.
- No report artifact writes.
- No hosted behavior.
- No reasoning lineage.
- No release posture changes.

## 4. API And Command Summary

The hidden dogfood command shape is:

```sh
workflow-os dogfood approval-presentation approve \
  --run-id <run-id> \
  --approval-id <approval-id> \
  --presentation-id <presentation-id> \
  --actor <actor> \
  --reason <reason>
```

The command is repo-local dogfood tooling. It is intentionally not documented as a stable public approval command.

## 5. Enforcement Behavior

The hidden command constructs a `LocalApprovalPresentationDecisionRequest` from explicit inputs and uses `LocalApprovalPresentationProof::PresentationId(...)`.

The core opt-in path validates matching durable presentation proof before appending approval events. If proof is missing, corrupt, mismatched, or ambiguous, approval fails before run resume or skill invocation.

## 6. Runner Behavior

For material live `phase-start` runs, the runner now:

1. starts the mapped `dg/*` workflow;
2. persists approval-presentation proof;
3. prints `presentation_id` and content hash;
4. prints a proof-enforced approval command;
5. includes that command in the `approval_handoff` and copy-safe approval request.

Dry-run mode remains non-mutating and does not claim proof was persisted.

## 7. Workflow Semantics

The runner remains governance coordination only. It does not approve automatically, execute repo edits, run git operations, open or merge PRs, execute local checks, perform provider writes, append unrelated events, write report artifacts, enable side effects, or change default executor behavior.

## 8. Redaction And Privacy

The approval reason remains redacted in displayed commands. The helper rejects secret-like approval metadata through the existing dogfood helper validation boundary. The hidden CLI command returns stable Workflow OS errors from the existing core enforcement path and does not print raw approval handoff payloads, provider payloads, command output, token-like values, private keys, raw source/spec contents, chat transcripts, or screenshots.

## 9. Test Coverage Summary

Focused coverage includes:

- dogfood helper builds the proof-enforced command with `presentation_id`;
- dry-run approval handoff remains non-mutating;
- proof persistence command shape remains covered;
- secret-like helper values remain rejected without leakage;
- public approval command dry-run behavior remains covered;
- focused dogfood helper test suite passes.

Live smoke coverage confirmed a phase-start persisted proof and a proof-enforced dogfood approval command successfully approved and completed a temporary governed run.

## 10. Governed Phase Summary

- dogfood workflow ID: `dg/implement`
- run ID: `run-1783599622543651000-2`
- approval ID: `approval/run-1783599622543651000-2/implementation-approved`
- approval-presentation proof: persisted before approval
- presentation ID: `presentation/cce553b5f248fa6c`
- approval outcome: granted by delegated maintainer
- phase-close event summary: 39 events total; 1 approval request; 1 approval grant; 6 scheduled steps; 6 skill invocations requested, started, and succeeded; 0 retries; 0 escalations

The dogfood runner coordinated governance only. Repo edits, validation commands, git operations, PR actions, and this report were performed by the executor outside the kernel and are disclosed here.

## 11. Commands Run

- `npm run test:dogfood-helper` - passed
- `cargo fmt --all --check` - passed
- `cargo check -p workflow-cli` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed
- proof-enforced approval smoke using temporary dogfood state - passed
- `npm run dogfood:benchmark -- phase-close run-1783599622543651000-2 --phase implementation` - passed

## 12. Remaining Known Limitations

- The hidden dogfood command is not a public approval-card UX.
- The dogfood helper does not yet configure a freshness/max-age policy.
- Phase-close does not yet disclose whether approval used presentation enforcement.
- Denied dogfood approvals with presentation proof are not separately surfaced by the benchmark helper.

## 13. Recommended Next Phase

Recommended next phase: dogfood runner approval-presentation enforcement review.

The review should verify scope, default public approval behavior preservation, fail-closed proof enforcement, test quality, and documentation honesty before any broader approval-card or high-assurance integration work.
