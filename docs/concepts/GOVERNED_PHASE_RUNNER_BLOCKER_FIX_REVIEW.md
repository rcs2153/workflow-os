# Governed Phase Runner Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to governed phase runner PR update.

The blocker fix adds first-class `approval_reason` output to `phase-start` while preserving command-display redaction for `--reason` values. The fix is narrow, test-covered, and does not broaden the runner's authority.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

No accidental implementation was found for:

- automatic approvals;
- hidden approvals;
- new phase mappings;
- local check execution;
- git operations;
- PR automation;
- report artifact writing;
- WorkReport rendering;
- workflow schema changes;
- repository writes from inside the kernel;
- hosted behavior;
- write-capable adapters;
- recursive agents;
- agent swarms;
- Level 3/4 autonomy.

## 3. Original Blocker Restatement

The original review found that `phase-start` displayed:

- `run_id`;
- `approval_id`;
- status;
- next action.

But it did not display the approval reason as a first-class field. The approval command displayed `--reason <redacted-reason>`, which was safe but did not satisfy the P0 acceptance requirement to display approval reason.

## 4. Fix Assessment

The implementation adds:

- `approval_reason: <bounded phase approval reason>` to `phase-start --dry-run`;
- `approval_reason: <bounded phase approval reason>` to live `phase-start`.

The approval reason comes from the static phase mapping table, not caller-supplied free text. Examples include:

- `approved-review-phase`;
- `approved-implementation-phase`.

This is the smallest appropriate fix.

## 5. Approval Boundary Assessment

The approval boundary remains correct:

- `phase-start` still does not approve automatically;
- `phase-start --dry-run` still reports `approval_outcome: not_requested`;
- live `phase-start` still reports `approval_outcome: pending`;
- the runner still prints an approval command for a human or agent to run separately;
- command display still redacts `--reason` values.

The blocker is fixed because the bounded approval reason is now visible independently of the redacted command.

## 6. Privacy And Redaction Assessment

The fix preserves redaction posture.

Verified behavior:

- `approval_reason: approved-implementation-phase` is visible;
- displayed approval commands do not expose `--reason approved-implementation-phase`;
- secret-like phase, actor, and explicit approval metadata validation remains unchanged.

This is acceptable because static phase approval reasons are bounded literals and not sensitive caller-supplied payloads.

## 7. Test Quality Assessment

Tests now cover:

- `phase-start --dry-run` displays `approval_reason: approved-review-phase`;
- `phase-start --dry-run --phase implementation` displays `approval_reason: approved-implementation-phase`;
- displayed commands do not expose the approval reason as a raw `--reason` value;
- existing phase mapping and explicit approval boundary behavior;
- secret-like metadata rejection and non-leakage.

The targeted blocker regression is covered.

Remaining non-blocking gap:

- There is still no table-driven test for every phase mapping.

## 8. Documentation Review

The blocker fix report documents:

- the original blocker;
- the fixed behavior;
- privacy/redaction posture;
- dogfood context;
- commands run;
- remaining limitations.

The original review remains intact and was not rewritten to erase the blocker.

## 9. Dogfood Review Context

This blocker-fix review phase was governed by:

- Workflow ID: `dg/review`
- Run ID: `run-1783052910062244000-2`
- Approval ID: `approval/run-1783052910062244000-2/review-scope-approved`
- Approval outcome: granted

Repository review and documentation edits were performed outside the kernel. The kernel governed the review boundary and approval trail.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add table-driven tests for all phase-to-workflow mappings.
- Consider documenting or refactoring the exported `buildWorkflowCommand(...)` behavior for `phase-close`, since runtime `phase-close` uses both status and inspect while the exported command builder returns the legacy inspect command shape.
- Consider a future `--exempt-reason` mode for explicitly exempted phases.

## 12. Recommended Next Phase

Governed phase runner PR update.

The branch should include:

- implementation;
- implementation report;
- initial review;
- blocker fix;
- blocker-fix report;
- blocker-fix review.

After PR update and merge, the next roadmap phase can return to WorkReport high-assurance approval disclosure plan review or the next accepted roadmap item.

## 13. Validation

Commands reviewed from the blocker-fix phase:

- `npm run test:dogfood-helper` - passed with 17 tests.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-start --phase implementation --dry-run --no-build` - passed and displayed `approval_reason: approved-implementation-phase`.
- `npm run dogfood:benchmark -- phase-close run-1783052509912061000-2 --phase blocker --state-dir /private/tmp/workflow-os-governed-phase-runner-blocker-fix --no-build` - passed.

Review-phase commands:

- `npm run dogfood:benchmark -- phase-start --phase review --state-dir /private/tmp/workflow-os-governed-phase-runner-blocker-fix-review --no-build` - passed and displayed `approval_reason: approved-review-phase`.
- `workflow-os approve ... --reason approved-governed-phase-runner-blocker-fix-review` - completed the governed review run.

The blocker-fix review document itself was added after those checks and should be included in the next docs validation run.
