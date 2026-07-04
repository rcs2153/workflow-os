# Governed Phase Approval Handoff Preservation Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to workflow-declared high-assurance artifact requirement executor integration review.

The fix addresses the actual remaining failure mode: the runner emitted a complete `approval_handoff` block, but the agent could still collapse that block into vague approval prose. The implemented fix makes preservation mandatory in `AGENTS.md`, updates the runbook and bug record, and adds a focused regression check that protects the repo-level instruction.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

Implemented:

- mandatory approval handoff preservation rule in `AGENTS.md`;
- self-governed build benchmark runbook update distinguishing handoff emission from handoff preservation;
- preservation bug record updated as a fixed fix-forward record;
- focused dogfood helper test proving repo-level agent instructions keep the preservation rule;
- blocker-fix report with dogfood run, approval, event summary, validation summary, and limitations.

No accidental implementation was found for:

- runtime approval semantic changes;
- automatic approval;
- hidden approvals;
- repository actions performed by the kernel;
- git or PR automation;
- local command execution by the kernel;
- persistence;
- report artifacts;
- schema changes;
- side-effect modeling;
- writes;
- release posture changes.

## 3. Original Blocker Restatement

The original preservation blocker was not that the kernel failed to emit approval context.

The runner already emitted:

- `approval_handoff_required: true`;
- `approval_handoff`;
- workflow ID;
- phase;
- run ID;
- approval ID;
- status;
- approval reason;
- allowed scope;
- disallowed scope;
- next action;
- redaction note;
- approval command posture;
- agent relay instruction.

The failure was that the agent response layer still summarized the approval pause into vague prose. That made a governed checkpoint look like an ordinary chat permission and omitted the concrete approval context.

## 4. Fix Approach Assessment

The fix is minimal and correctly targeted.

It does not change the local runtime approval model, event projection, policy evaluation, or approval decision semantics. Instead, it hardens the repo-local dogfood executor boundary:

- `AGENTS.md` now requires the full block in user-facing approval requests;
- the rule explicitly applies to final responses that wait for approval;
- the rule prohibits vague replacement prose;
- a focused regression test reads `AGENTS.md` and fails if the preservation rule disappears.

This is the right layer for the immediate blocker. A typed core approval handoff model or response linter may be useful later, but neither is required to fix the current dogfood regression.

## 5. Validation Boundary Assessment

The validation boundary is appropriate:

- helper output tests still prove `phase-start` emits the structured handoff;
- the new test proves repo-level agent instructions require preserving the emitted handoff;
- the fix does not pretend to verify arbitrary assistant final messages;
- the known limitation is documented rather than hidden.

The review run itself also exercised the intended behavior: the approval request preserved the full emitted `approval_handoff` block before approval was granted.

## 6. Privacy And Redaction Assessment

The fix does not introduce new sensitive data surfaces.

The preserved handoff block contains bounded run and approval identifiers, scope text, next action, redaction note, and the approval command with the reason redacted. It does not copy raw provider payloads, command output, local check output, report text, file contents, environment values, credentials, tokens, private keys, or secret-like values.

The approval command display remains redaction-safe.

## 7. Test Quality Assessment

Test coverage is suitable for this narrow blocker fix.

Covered:

- `phase-start` dry-run emits approval handoff fields;
- `phase-start` dry-run does not approve automatically;
- approval command display redacts reason values;
- secret-like helper values are rejected without leaking;
- repo-level agent instructions require:
  - `approval_handoff_required: true`;
  - complete `approval_handoff` block;
  - no vague prose replacement;
  - final response includes the complete handoff block.

Non-blocking gap:

- There is no general linter for actual assistant final responses. That is acceptable for this phase because current enforcement lives in repo instructions and focused tests, not in a typed product surface.

## 8. Documentation Review

Docs now clearly say:

- runner-side approval handoff emission is fixed;
- agent-side approval handoff preservation is fixed at the repo-instruction boundary;
- agents must preserve and present the complete emitted block;
- approval requests must not collapse the block into vague prose;
- runtime approval semantics are unchanged;
- no automatic approval, hidden approval, repository action, persistence, artifacts, schema changes, side effects, writes, or release posture changes were introduced.

The bug record preserves the incident and states the implemented fix, which should help future dogfood work avoid confusing emission failures with preservation failures.

## 9. Regression Assessment

No regressions were found.

Unchanged:

- runtime approval state;
- approval projection;
- policy evaluation;
- workflow execution;
- report artifact behavior;
- dogfood helper authority;
- command redaction posture.

The fix only changes repo instructions, docs, and helper tests.

## 10. Blockers

No blockers remain for the governed phase approval handoff preservation blocker fix.

## 11. Non-Blocking Follow-Ups

- Consider a typed core approval handoff model if approval handoff becomes product surface beyond repo-local dogfood tooling.
- Consider an agent-response linter or transcript checker if there is a future reliable place to validate assistant final responses.
- Keep phase reports disclosing whether approval handoff preservation occurred for governed dogfood runs.

## 12. Recommended Next Phase

Recommended next phase: workflow-declared high-assurance artifact requirement executor integration review.

Why: the approval handoff preservation blocker is fixed and reviewed. The earlier implementation of explicit artifact-capable executor integration is security-sensitive runtime composition and should receive the planned phase-level review before broader high-assurance approval, artifact automation, CLI, or write-readiness work continues.

## 13. Validation

Validation commands run:

- `npm run test:dogfood-helper` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

## 14. Dogfood Governance

This review phase was governed by the local Workflow OS dogfood runner.

- workflow phase: review
- workflow ID: `dg/review`
- run ID: `run-1783142120610696000-2`
- approval ID: `approval/run-1783142120610696000-2/review-scope-approved`
- approval outcome: approved by the maintainer before review work continued
- event summary: completed terminal run with 39 events, 1 approval, 0 retries, and 0 escalations
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

The approval request preserved the full emitted `approval_handoff` block in the user-facing final response before approval was granted.

The dogfood runner coordinated governance only. Review work, documentation edits, validation commands, and reporting were performed outside the kernel by the executor.
