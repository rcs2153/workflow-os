# Dogfood Runner Approval-Presentation Enforcement Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The phase successfully moved repo-local dogfood approvals from "presentation proof exists but ordinary approval is still used" to a proof-enforced dogfood approval path. Material dogfood `phase-start` output now includes persisted approval-presentation proof, prints a `presentation_id`, and emits a hidden dogfood approval command that calls the reviewed opt-in executor enforcement path.

Default public `workflow-os approve` behavior remains unchanged.

## 2. Scope Verification

The phase stayed within the approved repo-local dogfood enforcement scope.

Implemented:

- hidden `workflow-os dogfood approval-presentation approve` command;
- explicit `presentation_id` input;
- use of `LocalExecutor::decide_approval_with_presentation(...)`;
- dogfood runner approval command output that includes persisted proof;
- focused dogfood helper tests;
- implementation report and roadmap/runbook updates.

No accidental scope expansion found:

- no automatic approvals;
- no hidden approvals;
- no default public approval behavior change;
- no public approval-card UI;
- no workflow schema changes;
- no examples;
- no provider writes;
- no side effects;
- no report artifact writes;
- no hosted or distributed runtime behavior;
- no high-assurance approval control expansion;
- no reasoning lineage;
- no recursive agents, agent swarms, or Level 3/4 autonomy;
- no release posture changes.

## 3. Enforcement Assessment

The implementation uses the existing opt-in approval-presentation enforcement path rather than duplicating approval transition logic. The hidden dogfood CLI command constructs `LocalApprovalPresentationDecisionRequest` with `LocalApprovalPresentationProof::PresentationId(...)`, then calls `decide_approval_with_presentation(...)`.

This is the right boundary for the first implementation because:

- it reuses reviewed core validation and event-append behavior;
- it requires explicit proof identity at the dogfood boundary;
- it fails before approval/resume events when proof validation fails;
- it leaves public approval behavior untouched.

The implementation does not configure a freshness/max-age policy for dogfood approvals yet. That is acceptable for this phase because identity/content proof was the approved first enforcement target and freshness is already represented in the core opt-in path.

## 4. Runner Output Assessment

Material `phase-start` output now includes:

- `approval_presentation_persisted: true`;
- `presentation_id`;
- `presentation_content_hash`;
- proof-enforced approval command;
- complete `approval_handoff`;
- copy-safe approval request.

The new output fixes the prior operational gap where a maintainer could see a detailed approval handoff but still approve through an ordinary command that did not prove the presented scope.

The command display continues to redact the approval reason. The runner remains governance coordination only and does not execute repo edits, git operations, checks, provider writes, report artifacts, or side effects.

## 5. Default Approval Preservation

Default public approval behavior is preserved.

The existing `workflow-os approve <run-id> <approval-id>` command still uses the ordinary approval path. The proof-enforced command is under the hidden dogfood namespace and requires explicit `--presentation-id`.

This is important because the phase was scoped to dogfood governance hardening, not a public approval UX or default runtime semantics change.

## 6. Error Handling And Privacy Assessment

The implementation delegates proof validation errors to the reviewed core path, preserving stable error behavior. Inputs are parsed through existing ID constructors, and the dogfood helper continues to reject secret-like helper values without echoing raw values.

No leakage found in reviewed behavior:

- no raw approval handoff payloads in errors;
- no provider payloads;
- no command output;
- no tokens, private keys, or local secrets;
- no raw source/spec contents;
- no screenshots or chat transcripts.

The implementation report accurately states that the hidden command is not a public approval-card UX.

## 7. Test Quality Assessment

Test coverage is adequate for the first dogfood enforcement slice.

Reviewed coverage includes:

- proof-enforced command construction includes `presentation_id`;
- proof persistence command shape remains covered;
- dry-run handoff remains non-mutating;
- secret-like helper values remain rejected;
- ordinary public approval command dry-run behavior remains covered;
- existing core approval-presentation executor tests cover matching proof, missing proof, mismatch, stale/future proof, ambiguity, denial, and debug redaction;
- live smoke verified a persisted `presentation_id` can approve a temporary dogfood run through the hidden command.

Remaining test gaps are non-blocking:

- no dedicated CLI integration test for `workflow-os dogfood approval-presentation approve`;
- no dogfood helper test for denied approvals with presentation proof;
- no phase-close assertion that reports whether proof enforcement was used;
- no dogfood freshness/max-age policy test because dogfood does not configure freshness yet.

## 8. Documentation Review

Documentation now states:

- dogfood approval-presentation proof persistence is implemented;
- dogfood approval-presentation enforcement is implemented for the repo-local dogfood command;
- default public approval behavior remains unchanged;
- automatic approvals are not implemented;
- public approval-card UI is not implemented;
- schemas, examples, provider writes, side effects, hosted behavior, reasoning lineage, and release posture changes are not implemented.

One stale planning section still described the implementation as future work. A tiny documentation correction was made during this review to clarify that the stale subsection was the pre-implementation baseline and that the next phase is review, not implementation.

## 9. Validation

Implementation phase validation:

- `npm run test:dogfood-helper` - passed;
- `cargo fmt --all --check` - passed;
- `cargo check -p workflow-cli` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed;
- `git diff --check` - passed;
- proof-enforced approval smoke using temporary dogfood state - passed;
- governed implementation phase close - passed.

Review phase validation:

- `npm run check:docs` - passed.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a dedicated CLI integration test for `workflow-os dogfood approval-presentation approve`.
- Add phase-close disclosure of whether the approval used presentation enforcement.
- Add denied dogfood approval coverage through the proof-enforced command.
- Decide whether dogfood approvals should configure a freshness/max-age policy.
- Consider when, separately, public approval-card UX should consume the same proof model.

## 12. Recommended Next Phase

Recommended next phase: dogfood phase-close proof-enforcement disclosure.

That phase should remain narrow: expose whether a governed dogfood run used proof-enforced approval in the phase-close summary. It should not change public approval semantics, add UI approval cards, add schemas, enable writes, add hosted behavior, or expand high-assurance approval controls.
