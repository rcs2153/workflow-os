# Dogfood Approval-Presentation Freshness Enforcement Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation is narrow, repo-local, and correctly routes dogfood approval
freshness through the existing opt-in approval-presentation enforcement boundary.
It does not change default public approval behavior.

## 2. Scope Verification

The phase stayed within the approved dogfood freshness-enforcement scope.

Implemented:

- repo-local dogfood approval commands now include a bounded freshness policy;
- the hidden dogfood approval command parses `--max-presentation-age-ms`;
- the parsed freshness value is passed to
  `LocalExecutor::decide_approval_with_presentation(...)`;
- stale proof fails closed before approval events are appended;
- focused tests and documentation were added.

Not introduced:

- default public approval behavior changes;
- automatic approvals;
- public approval-card UI;
- workflow schema changes;
- examples;
- provider writes;
- side effects;
- report artifact writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 3. Behavior Assessment

The selected behavior is appropriate for the dogfood boundary. The repo-local
benchmark helper now emits approval commands with:

```text
--max-presentation-age-ms 86400000
```

That value is static, bounded, and explicit. The hidden dogfood CLI converts it
to `Duration` and passes it to the reviewed core approval-presentation
enforcement path.

This preserves the important layering:

- the runner coordinates governance;
- the hidden dogfood CLI carries explicit proof inputs;
- the core executor validates persisted proof freshness;
- the public approval path remains unchanged.

## 4. Fail-Closed Assessment

The stale-proof regression test demonstrates the important failure invariant:

- stale proof returns `approval_presentation_enforcement.proof_stale`;
- the stale presentation ID is not leaked in stderr;
- the run remains `WaitingForApproval`;
- no `ApprovalGranted` event is appended.

This is the correct safety posture. A stale presentation cannot silently approve
material self-governed work.

## 5. Privacy And Redaction Assessment

The freshness implementation adds only a numeric max-age value. It does not
store or copy approval handoff payloads, raw command output, provider payloads,
source/spec contents, tokens, credentials, private keys, or secret-like values.

The focused stale-proof test asserts that the presentation ID is not leaked in
the stale-proof error path. Existing approval-presentation and proof-marker
tests continue to cover bounded serialization, redaction-safe Debug behavior,
and non-leaking error posture.

## 6. Documentation Review

Documentation now states:

- dogfood approval-presentation enforcement is implemented for the repo-local
  dogfood runner;
- material phase-start output passes persisted `presentation_id` and bounded
  freshness into the opt-in enforcement path;
- default public approval behavior remains unchanged;
- public approval cards, schemas, examples, provider writes, side effects,
  hosted behavior, reasoning lineage, and release posture changes remain
  unimplemented.

During review, two stale planning statements that described freshness as future
work were corrected in
`docs/implementation-plans/dogfood-runner-approval-presentation-enforcement-plan.md`.
That correction prevents a false roadmap claim and does not broaden scope.

## 7. Test Quality Assessment

Coverage is sufficient for this narrow phase:

- dogfood helper tests prove generated approval commands include the max-age
  argument;
- CLI tests prove stale persisted proof fails closed;
- the stale-proof test verifies no approval event is appended;
- the existing proof-marker inspect test continues to verify bounded proof
  marker projection;
- full workspace validation passed during implementation.

No blocker-level test gaps were found.

## 8. Validation Reviewed

Implementation report records:

- `npm run test:dogfood-helper` - passed;
- `cargo test -p workflow-cli --test cli dogfood_approval_presentation -- --nocapture` - passed;
- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed;
- `git diff --check` - passed.

Review validation:

- `npm run check:docs` - required for this review phase.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Consider whether denied dogfood approvals should also require
  approval-presentation proof.
- Consider an unambiguous lookup mode by run ID and approval ID only, while
  preserving explicit proof binding.
- Consider whether the static one-day dogfood freshness policy should become a
  repo-local helper constant surfaced in command output metadata.

These are follow-ups, not blockers.

## 11. Recommended Next Phase

Recommended next phase: approval-presentation denial-path proof enforcement
planning.

Why: granted material dogfood approvals now require persisted, fresh proof. The
remaining approval-direction asymmetry is denial handling. Planning should
decide whether dogfood denial should require the same presentation proof, how to
preserve fail-closed semantics, and whether any public behavior remains
unchanged.

## 12. Dogfood Governance

Workflow OS governed this review phase:

- workflow: `dg/review`;
- run: `run-1783702620677819000-2`;
- approval: `approval/run-1783702620677819000-2/review-scope-approved`;
- approval presentation: `presentation/a02c00119900ca67`;
- approval outcome: granted;
- approval reason: `approved-freshness-enforcement-review-scope`;
- close status: `Completed`;
- event summary:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`;
- approval presentation enforcement: proof-enforced;
- persisted approval presentation records: 1.

Codex performed documentation review and the tiny stale-roadmap correction
outside the kernel. The kernel governed the review approval boundary.
