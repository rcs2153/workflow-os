# Governed Phase Runner Blocker Fix Report

## 1. Executive Summary

This blocker fix adds first-class `approval_reason` output to the governed phase runner.

The prior review found that `phase-start` displayed `run_id`, `approval_id`, status, and next action, but did not display the bounded approval reason required by the P0 acceptance criteria. The runner only showed an approval command with the reason redacted.

The fix keeps command redaction intact while making the bounded phase approval reason visible as structured output.

## 2. Blocker Fixed

Original blocker:

- `phase-start` did not display approval reason as a first-class field.
- The approval command redacted the reason as `<redacted-reason>`.
- This preserved command-output safety but did not meet the requirement to display approval reason.

Fixed behavior:

- `phase-start` live output includes `approval_reason: <bounded phase approval reason>`.
- `phase-start --dry-run` output includes `approval_reason: <bounded phase approval reason>`.
- Approval command display continues to redact `--reason` values.

## 3. Scope Completed

- Added first-class `approval_reason` output to live `phase-start`.
- Added first-class `approval_reason` output to `phase-start --dry-run`.
- Added focused regression coverage.
- Preserved command-display redaction.

## 4. Scope Explicitly Not Completed

This phase did not implement:

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

## 5. Validation Boundary Summary

The approval reason is sourced from the static phase mapping table inside the repo-local helper.

The runner still rejects secret-like `--phase`, `--actor`, and explicit approval metadata values. The printed `approval_reason` values are bounded literals such as `approved-implementation-phase`, not caller-supplied free text.

## 6. Privacy And Redaction Summary

Command display still redacts values following `--reason`.

This means:

- the human sees `approval_reason: approved-implementation-phase`;
- the displayed approval command still shows `--reason <redacted-reason>`;
- arbitrary caller-supplied secret-like reasons remain rejected by existing helper validation.

## 7. Test Coverage Summary

Added or updated tests proving:

- `phase-start --dry-run` displays `approval_reason: approved-review-phase`;
- `phase-start --dry-run --phase implementation` displays `approval_reason: approved-implementation-phase`;
- displayed approval commands do not expose the approval reason as a raw `--reason` value.

Existing helper tests continue to cover:

- phase mapping;
- explicit approval boundary;
- unsupported phase non-leakage;
- command redaction;
- secret-like metadata rejection;
- missing binary fail-closed behavior.

## 8. Dogfood Context

This blocker-fix phase was governed by:

- Workflow ID: `dg/blocker`
- Run ID: `run-1783052509912061000-2`
- Approval ID: `approval/run-1783052509912061000-2/fix-approved`
- Approval outcome: granted

Repository edits and validation commands were performed outside the kernel. The kernel governed the blocker-fix boundary and approval trail.

## 9. Commands Run And Results

Commands run:

- `npm run dogfood:benchmark -- phase-start --phase blocker --state-dir /private/tmp/workflow-os-governed-phase-runner-blocker-fix --no-build` - passed and paused for approval.
- `workflow-os approve ... --reason approved-governed-phase-runner-blocker-fix` - completed the governed blocker-fix run.
- `npm run test:dogfood-helper` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-start --phase implementation --dry-run --no-build` - passed and displayed `approval_reason: approved-implementation-phase`.
- `npm run dogfood:benchmark -- phase-close run-1783052509912061000-2 --phase blocker --state-dir /private/tmp/workflow-os-governed-phase-runner-blocker-fix --no-build` - passed and produced a bounded event summary.

The focused helper test suite now has 17 passing tests.

## 10. Remaining Known Limitations

- The helper remains repo-local development tooling.
- `phase-close` still parses experimental CLI JSON internally.
- The helper does not execute real local checks.
- The helper does not produce WorkReport artifacts.

## 11. Recommended Next Phase

Recommended next phase: governed phase runner blocker-fix review.

That review should verify the acceptance blocker is fixed and no runner scope was broadened.
