# Governed Phase Approval Work Summary Plan

Status: Implemented P0 fix. The repo-local governed phase runner now supports bounded work-context fields in `approval_handoff` output and fails closed for live material phase starts when required work context is missing. This plan did not implement runtime approval semantic changes, automatic approvals, repository automation, shell execution, persistence, report artifacts, schemas, side effects, writes, hosted behavior, or release posture changes.

## 1. Executive Summary

The governed phase runner already emits an `approval_handoff` block, and agents are required to preserve it. A new dogfood failure showed that this is still insufficient: the block can preserve IDs and phase posture while failing to describe the concrete work being approved.

This plan added a P0 approval-handoff work-summary requirement. Material governed phases must present a bounded, redaction-safe work summary, approved scope, strict non-goals, expected touched surfaces, required validation, and why-now rationale before asking a maintainer to approve.

## 2. Goals

- Make governed approval requests answer "what work am I approving?"
- Preserve explicit human approval before material roadmap work.
- Require bounded work context in approval handoffs.
- Keep approval context redaction-safe.
- Prevent agents from asking approval on underspecified phase handoffs.
- Keep the runner as governance coordination only.
- Preserve current runtime approval semantics.

## 3. Non-Goals

Do not implement:

- runtime approval semantic changes;
- automatic approval;
- hidden approval;
- repository edits by the kernel;
- git or PR automation;
- shell execution by the kernel;
- automatic local check execution;
- persistence;
- report artifacts;
- workflow schema changes;
- side-effect modeling;
- writes;
- hosted/distributed runtime behavior;
- enterprise approval UI;
- RBAC, IdP, quorum approval, or revocation;
- release posture changes.

## 4. Current Gap

Existing handoff fields include workflow ID, phase, run ID, approval ID, status, approval reason, allowed scope, disallowed scope, next action, redaction note, approval command, and agent instruction.

That protects identity and preservation, but it does not require:

- concrete work summary;
- approved implementation/review scope;
- expected files or modules touched;
- validation required;
- why the phase is next;
- source plan/review/bug link.

The result can be technically structured but still human-insufficient.

## 5. Required Handoff Fields

For material roadmap phases, `phase-start` requires or emits:

- `work_summary`: bounded description of concrete proposed work;
- `approved_scope`: bounded statement of what may be done;
- `strict_non_goals`: explicit list or compact statement of forbidden scope;
- `expected_touched_surfaces`: likely files, docs, modules, scripts, tests, or repo areas;
- `validation_required`: expected validation commands or checks;
- `why_now`: roadmap, review, blocker, or plan basis for doing this phase now;
- existing `approval_allows`;
- existing `approval_does_not_allow`;
- existing `next_action_after_approval`.

Fields are provided by CLI flags in the current repo-local helper. Structured sidecar input or a future phase manifest remains deferred.

## 6. Missing Context Behavior

For material phases, missing work context must be visible and fail closed at the approval boundary.

Recommended behavior:

- `phase-start --dry-run` may show placeholder context and `approval_outcome: not_requested`.
- live `phase-start` for material phases does not present approval as ready if work context is missing.
- the runner prints a stable non-leaking `dogfood.helper.work_context_missing` error when required work context is missing.
- agents must not ask maintainers to approve an underspecified handoff.

## 7. Privacy And Redaction

Work-context fields must be bounded and redaction-safe.

Reject or redact:

- token-like values;
- credentials;
- authorization headers;
- private keys;
- raw command output;
- raw provider payloads;
- raw logs;
- raw spec or file contents;
- secret-like paths or metadata;
- unbounded text.

Errors must use stable codes or stable text and must not echo rejected values.

## 8. Agent Instruction Update

`AGENTS.md` is updated to require:

- preserve the complete `approval_handoff` block;
- verify the block contains concrete work context;
- stop and report a handoff-context bug if work context is missing;
- do not ask for approval from a handoff that fails to explain the proposed work.

## 9. Test Plan

Add focused tests proving:

- dry-run handoff includes work-context field names;
- live or simulated material phase handoff includes work summary;
- approved scope is present;
- strict non-goals are present;
- expected touched surfaces are present;
- validation required is present;
- why-now is present;
- missing work context fails closed or produces non-ready posture;
- secret-like work summary is rejected or redacted;
- secret-like touched surface is rejected or redacted;
- existing approval handoff preservation tests still pass;
- docs check passes.

## 10. Proposed Implementation Sequence

1. Add bounded work-context input support to the repo-local phase runner.
2. Add missing-context fail-closed or not-ready behavior for live material phases.
3. Extend `approval_handoff` output with work-context fields.
4. Update `AGENTS.md` and self-governed benchmark docs.
5. Add focused tests.
6. Create an implementation report.
7. Run docs and helper tests.
8. Review before resuming broader runtime-composition phases.

## 11. Documentation Updates

Update:

- `ROADMAP.md`;
- `AGENTS.md`;
- `docs/user-guide/self-governed-build-benchmark.md`;
- [Governed Phase Approval Work Summary Bug](../concepts/GOVERNED_PHASE_APPROVAL_WORK_SUMMARY_BUG.md).

Docs say:

- approval handoff emission is implemented;
- approval handoff preservation is implemented;
- approval handoff work-summary context is implemented for the repo-local governed phase runner;
- agents must not ask for approval when work context is missing;
- runtime approval semantics are unchanged;
- automatic approvals, hidden approvals, shell execution, repo automation, persistence, artifacts, schemas, side effects, writes, hosted behavior, and release posture changes remain unsupported.

## 12. Final Recommendation

Next phase: governed phase approval work-summary handoff review.

This should happen before additional long-running governed implementation phases, because the approval boundary must be clear enough for maintainers to approve real work without reconstructing context from chat history.
