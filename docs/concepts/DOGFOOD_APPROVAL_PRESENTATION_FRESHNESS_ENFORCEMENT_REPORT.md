# Dogfood Approval-Presentation Freshness Enforcement Report

## 1. Executive Summary

This phase hardens repo-local dogfood approvals by configuring freshness
enforcement for persisted approval-presentation proof.

Material dogfood `phase-start` already persists a bounded
`ApprovalPresentationRecord` and prints a proof-enforced approval command. That
command now also passes an explicit max-age policy into the existing opt-in
approval-presentation enforcement path, so stale presentation proof fails closed
before approval events are appended.

## 2. Scope Completed

- Added a bounded dogfood approval-presentation freshness policy to the
  repo-local benchmark helper.
- Updated the proof-enforced dogfood approval command to pass
  `--max-presentation-age-ms`.
- Updated the hidden `workflow-os dogfood approval-presentation approve`
  command to parse the max-age value and pass it to
  `LocalExecutor::decide_approval_with_presentation(...)`.
- Added focused regression coverage for stale proof rejection.
- Updated roadmap/runbook/planning docs to state that dogfood freshness
  enforcement is implemented.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- default public approval behavior changes;
- automatic approvals;
- public approval-card UI;
- workflow schema fields;
- examples;
- provider writes;
- side effects;
- report artifact writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Behavior Added

The repo-local dogfood benchmark helper now emits proof-enforced approval
commands with a bounded freshness policy:

```text
--max-presentation-age-ms 86400000
```

The hidden dogfood approval command converts that value into the existing core
`max_presentation_age` enforcement field. If the persisted presentation proof is
older than the supplied limit, the core opt-in enforcement path returns the
stable `approval_presentation_enforcement.proof_stale` error before approval or
resume events are appended.

## 5. Privacy And Redaction Summary

The change does not add new payload storage. It passes only a bounded numeric
freshness limit.

Errors continue to avoid leaking:

- approval handoff text;
- presentation record payloads;
- presentation IDs in stale-proof errors;
- provider payloads;
- command output;
- raw source/spec contents;
- tokens, credentials, private keys, or secret-like values.

## 6. Test Coverage Summary

Added or updated tests cover:

- proof-enforced dogfood approval commands include a bounded max-age flag;
- stale approval-presentation proof is rejected through the hidden dogfood CLI;
- stale rejection leaves the run waiting for approval;
- stale rejection does not append `ApprovalGranted` events;
- focused dogfood helper tests continue to pass;
- existing proof-marker inspect projection coverage continues to pass.

## 7. Commands Run And Results

- `npm run test:dogfood-helper` - passed.
- `cargo test -p workflow-cli --test cli dogfood_approval_presentation -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 8. Remaining Known Limitations

- The freshness policy is repo-local dogfood behavior, not default public
  approval behavior.
- The max-age value is static in the helper rather than configurable by a
  public runtime policy.
- Public approval-card UX remains unimplemented.
- Default public approvals remain unchanged.

## 9. Recommended Next Phase

Recommended next phase: dogfood approval-presentation freshness enforcement
review.

The review should confirm that the freshness policy is bounded, repo-local,
redaction-safe, and does not change public approval behavior.

## 10. Dogfood Governance

Workflow OS governed this implementation phase:

- workflow: `dg/implement`;
- run: `run-1783700403910320000-2`;
- approval: `approval/run-1783700403910320000-2/implementation-approved`;
- approval presentation: `presentation/f7a77d52fc829b5e`;
- approval outcome: granted;
- approval reason: `approved-dogfood-approval-presentation-freshness-scope`;
- close status: `Completed`;
- event summary:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`;
- approval presentation enforcement: proof-enforced;
- persisted approval presentation records: 1.

Codex performed repository edits, tests, and documentation updates outside the
kernel. The kernel governed the phase approval boundary.
