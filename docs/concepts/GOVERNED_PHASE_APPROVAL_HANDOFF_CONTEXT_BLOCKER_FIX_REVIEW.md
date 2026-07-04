# Governed Phase Approval Handoff Context Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to workflow-declared artifact requirement runtime derivation planning.

The focused blocker fix resolves the dogfood approval-context loss that was observed during the prior governed review phase. `phase-start` now emits a deterministic, copyable `approval_handoff` block that includes the concrete run identity, approval identity, status, reason, allowed scope, disallowed scope, next action, redaction note, and agent relay instruction.

The fix stays within repo-local helper/runbook/test scope. It does not change core runtime approval semantics, approve automatically, perform repository actions, write artifacts, add schema behavior, model side effects, add writes, or change release posture.

## 2. Scope Verification

The fix stayed within approved blocker-fix scope.

Implemented:

- structured approval handoff output in `phase-start`;
- dry-run handoff shape with explicit placeholders;
- live handoff shape with real run and approval IDs;
- redaction-safe approval command display;
- secret-like `--run-id` rejection;
- focused helper tests;
- runbook, dogfood README, roadmap, bug record, and blocker-fix report updates.

No accidental implementation was found for:

- hidden approvals;
- automatic approval;
- runtime approval semantic changes;
- repository edits performed by the runner;
- git or PR operations performed by the runner;
- local command execution from inside the kernel;
- report artifacts;
- persistence;
- workflow schema changes;
- side-effect modeling;
- writes;
- release posture changes.

## 3. Original Blocker Restatement

The original blocker was not a runtime approval-state bug. The runner printed useful approval fields, but it did not provide a required agent-facing handoff block.

That allowed an executor to compress a concrete governed checkpoint into vague prose such as "Waiting on the governed review approval before proceeding." The resulting handoff omitted:

- workflow ID;
- run ID;
- approval ID;
- status;
- approval reason;
- allowed scope;
- disallowed scope;
- next action.

For Workflow OS dogfooding, that is a real product issue because approval checkpoints must be visible, specific, reviewable, and hard to accidentally flatten.

## 4. Fix Approach Assessment

The selected approach is minimal and appropriate.

The fix updates the repo-local development helper rather than changing the core runtime approval model. That is the correct boundary because the failure was in the agent/human handoff surface, not in approval projection, event rehydration, or policy execution.

The new helper output includes:

- `approval_handoff_required: true`;
- `approval_handoff`;
- `workflow_id`;
- `phase`;
- `run_id`;
- `approval_id`;
- `status`;
- `approval_reason`;
- `approval_allows`;
- `approval_does_not_allow`;
- `next_action_after_approval`;
- `redaction_note`;
- `approval_command` on live runs;
- `agent_instruction`.

The block is deterministic, bounded, human-readable, and copyable. It is not yet a typed core runtime object, which is acceptable for this repo-local dogfood helper phase.

## 5. Validation Boundary Assessment

The validation boundary is sound:

- dry-run output uses placeholders and `NotRequestedDryRun`, so it does not fabricate a real approval;
- live output uses run and approval IDs produced by the local kernel;
- helper arguments reject secret-like approval reason, actor, phase, and now run ID values;
- the approval command display still redacts the reason;
- the helper does not auto-approve after printing the block;
- the helper does not mutate workflow state by hand.

The current review run itself exercised the live path and produced the expected `approval_handoff` block before approval was granted.

## 6. Privacy And Redaction Assessment

The handoff block does not copy raw payloads, command output, provider output, local check output, report bodies, tokens, credentials, private keys, or secret-like values.

Secret-like user-supplied metadata is rejected before handoff output for:

- approval reason;
- actor;
- phase;
- run ID.

The approval command keeps `--reason` redacted in displayed output. Live run and approval IDs are stable local kernel identifiers, not payloads.

## 7. Test Quality Assessment

Focused test coverage is good for the helper boundary:

- dry-run `phase-start` emits the structured handoff;
- the handoff includes workflow ID, run ID placeholder, approval ID placeholder, status, approval reason, allowed scope, disallowed scope, next action, redaction note, and agent instruction;
- dry-run does not print an executable approval command;
- approval reason remains visible as bounded posture but redacted in command display;
- secret-like run ID is rejected without leaking the value;
- existing helper tests for command shape, redaction, unsupported commands, prompt output, and missing binary still pass.

Non-blocking test follow-up:

- add a narrow live-path fixture test if the helper later gains an injectable CLI runner. Today, the live path is covered by actual dogfood execution rather than a pure unit test.

## 8. Documentation Review

Docs now correctly say:

- the approval handoff bug is fixed;
- `phase-start` emits a structured `approval_handoff` block;
- agents must relay the complete block before asking for approval;
- the helper remains repo-local development tooling;
- the helper does not approve automatically;
- the helper does not perform repository actions, git/PR operations, local command execution from inside the kernel, persistence, artifact writes, schemas, side effects, writes, or release posture changes.

The original bug document preserves the incident and now records fixed behavior, which is useful for future product learning.

## 9. Regression Assessment

The fix does not alter existing core behavior:

- approval state and projection remain unchanged;
- policy evaluation remains unchanged;
- workflow execution remains unchanged;
- report artifact behavior remains unchanged;
- dogfood helper command shape remains compatible except for the added handoff output and secret-like run ID rejection;
- existing dogfood helper tests continue to pass;
- workspace tests continue to pass.

## 10. Blockers

No blockers remain for the governed phase approval handoff context blocker fix.

## 11. Non-Blocking Follow-Ups

- Consider a typed core approval handoff model later if approval handoffs become product surface area beyond the repo-local helper.
- Add a live-path unit fixture only if the helper gains an injectable runner boundary.
- Keep enforcing in future agent prompts that the full `approval_handoff` block must be relayed before approval.

## 12. Recommended Next Phase

Recommended next phase: workflow-declared artifact requirement runtime derivation planning.

Why: the approval-handoff blocker is fixed, so the roadmap can return to the next feature lane identified by the high-assurance artifact requirement schema review. The next work should plan how workflow-declared `report_artifact_requirements.high_assurance_approval` can be derived into explicit artifact gate inputs without making report generation, artifact writing, or runtime behavior automatic.

## 13. Validation

Validation commands run:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `npm run check:dogfood` passed.
- `git diff --check` passed.

## 14. Dogfood Governance

This blocker-fix review phase was governed by the local Workflow OS dogfood runner.

- workflow phase: review
- workflow ID: `dg/review`
- run ID: `run-1783133967161332000-2`
- approval ID: `approval/run-1783133967161332000-2/review-scope-approved`
- approval outcome: approved by the maintainer before review work continued
- close status: completed
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`

The dogfood runner coordinated governance only. Review work, documentation edits, and validation commands were performed by the executor.
