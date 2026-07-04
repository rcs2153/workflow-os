# Governed Phase Approval Handoff Context Blocker Fix Report

## 1. Executive Summary

The governed phase approval handoff context blocker is fixed.

`npm run dogfood:benchmark -- phase-start` now emits a deterministic `approval_handoff` block whenever it reaches the approval boundary. The block is designed to be relayed directly by an agent to a maintainer without interpretation, so governed approvals no longer depend on the agent remembering to preserve scattered run and approval fields.

The fix remains helper/runbook/test scoped. It does not change runtime approval semantics, approve automatically, run commands from inside the kernel, mutate workflow state, write report artifacts, perform git or PR actions, add schema fields, model side effects, add writes, or change release posture.

## 2. Blocker Fixed

Original blocker:

- the governed phase runner printed useful approval fields;
- the agent compressed those fields into vague prose;
- the maintainer could not see workflow ID, run ID, approval ID, approval reason, scope, non-scope, or next action at the approval moment;
- the dogfood approval checkpoint therefore felt like a generic chat permission rather than a concrete governed approval.

Fixed behavior:

- live `phase-start` output includes `approval_handoff_required: true`;
- live `phase-start` output includes an `approval_handoff` block with real run and approval IDs;
- dry-run `phase-start` output includes the same shape with explicit placeholder IDs and `NotRequestedDryRun` status;
- agents are instructed in the output to relay the full block before asking for approval.

## 3. Implementation Approach

The fix updates the repo-local development helper in `scripts/self-governed-benchmark.mjs`.

The helper now emits:

- workflow ID;
- phase;
- run ID;
- approval ID;
- status;
- approval reason;
- what approval allows;
- what approval does not allow;
- next action after approval;
- redaction note;
- approval command for live runs;
- explicit agent relay instruction.

The helper also rejects secret-like `--run-id` values, matching the existing safety posture for approval reason, actor, and phase metadata.

## 4. Boundary Summary

This fix does not:

- approve automatically;
- hide approvals;
- change core runtime approval semantics;
- change policy evaluation;
- change workflow schemas;
- run arbitrary shell commands from the kernel;
- register local check handlers by default;
- write report artifacts;
- mutate workflow state by hand;
- perform repo edits, git operations, PR actions, branch cleanup, release actions, or filesystem writes on behalf of the kernel;
- add side-effect modeling or write-capable adapters.

The runner remains governance coordination only. Codex, Claude Code, or a human remains the executor.

## 5. Redaction And Privacy Summary

The handoff block is bounded and redaction-aware:

- approval command display continues to redact the approval reason;
- secret-like approval reason, actor, phase, and run ID helper inputs are rejected without leaking the raw value;
- live run and approval IDs come from the validated local kernel path;
- no raw command output, provider payloads, local check payloads, report contents, tokens, credentials, private keys, or secret-like values are copied into the handoff block.

## 6. Test Coverage Summary

Focused helper tests now cover:

- dry-run `phase-start` emits the structured approval handoff block;
- the block includes workflow ID, run ID placeholder, approval ID placeholder, status, approval reason, approval scope, non-scope, next action, redaction note, and agent instruction;
- approval commands remain absent from dry-run handoff output;
- approval reason remains bounded and is not printed as an unredacted command reason;
- secret-like `--run-id` is rejected without leaking the raw value;
- existing helper command shape, redaction, unsupported command, prompt, and missing-binary tests still pass.

## 7. Commands Run And Results

- `npm run test:dogfood-helper` - passed.
- `npm run dogfood:benchmark -- phase-start --phase review --dry-run --no-build` - passed and printed the structured `approval_handoff` block.

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run check:dogfood` - passed.
- `git diff --check` - passed.

## 8. Dogfood Governance

This blocker fix phase was governed by the local Workflow OS dogfood runner.

- workflow phase: blocker
- workflow ID: `dg/blocker`
- run ID: `run-1783133237839872000-2`
- approval ID: `approval/run-1783133237839872000-2/fix-approved`
- approval outcome: approved by the maintainer before implementation work continued
- close status: completed
- event summary: 39 events total; 1 approval; 0 retries; 0 escalations

The dogfood runner coordinated governance only. Repo edits, tests, and reporting were performed by the executor.

## 9. Remaining Known Limitations

- The helper output is still repo-local development tooling, not a stable public CLI contract.
- The handoff block is textual; it is not yet a typed core runtime object.
- `phase-start` still depends on agents actually relaying the block, although the runner now gives them a direct copyable structure.
- The fix does not add multi-party approval, RBAC, IdP integration, notification delivery, or hosted approval UX.

## 10. Recommended Next Phase

Recommended next phase: governed phase approval handoff context blocker fix review.

The review should verify that the helper emits complete approval context, that dry-run and live paths are safe, that no runtime approval semantics changed, and that the runner remains governance coordination only.
