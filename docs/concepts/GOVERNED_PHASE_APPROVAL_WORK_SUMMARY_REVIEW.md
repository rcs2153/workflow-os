# Governed Phase Approval Work Summary Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The fix closes the P0 approval-boundary bug that allowed a governed phase approval handoff to preserve workflow/run/approval identity while still failing to explain the concrete work being approved. The repo-local phase runner now requires bounded work context for live material phase starts, emits that context in `approval_handoff`, and fails closed with a stable non-leaking error when required context is missing.

## 2. Scope Verification

The phase stayed within the approved blocker-fix scope.

Implemented scope:

- bounded work-context inputs for the repo-local governed phase runner;
- explicit `approval_handoff` fields for work summary, approved scope, strict non-goals, touched surfaces, validation requirements, and why-now rationale;
- live material phase fail-closed behavior when required work context is missing;
- focused tests for output shape, missing-context behavior, and secret-like value rejection;
- docs, roadmap, bug record, implementation plan, and implementation report updates.

No accidental scope expansion found:

- no runtime approval semantic changes;
- no automatic approvals;
- no hidden approvals;
- no repository edits, git operations, PR operations, shell execution, or file writes by the kernel;
- no persistence;
- no report artifacts;
- no workflow schema changes;
- no side-effect modeling;
- no writes;
- no hosted behavior;
- no enterprise approval UI, RBAC, IdP, quorum approval, or revocation;
- no release posture change.

## 3. Original Bug Restatement

The original bug was that a governed phase approval handoff could be structurally preserved but still underspecified.

The runner already emitted fields such as workflow ID, run ID, approval ID, status, approval reason, and allowed/disallowed posture. The repo instructions also required agents to relay that block. However, the handoff could still omit the concrete work being approved, the expected scope, strict non-goals, likely touched surfaces, validation expectations, and why the phase was next.

That meant a maintainer could be asked to approve a real governed phase without enough context to make the approval meaningful.

## 4. Fix Approach Assessment

The selected fix is minimal and appropriate.

The implementation adds repo-local helper flags rather than changing runtime approval semantics or workflow schemas:

- `--work-summary`
- `--approved-scope`
- `--strict-non-goals`
- `--expected-touched-surfaces`
- `--validation-required`
- `--why-now`

The approach is intentionally narrow: it strengthens the dogfood phase-runner approval boundary without pretending to solve enterprise approval UX, workflow-declared phase manifests, or runtime approval contract changes.

This is compatible with future evolution toward structured phase manifests because the current output fields are explicit and stable enough to become contract fields later.

## 5. Behavior Assessment

The helper now prints concrete work context before `approval_allows` in the emitted `approval_handoff`.

Dry-run behavior remains useful: it shows placeholder fields so agents and maintainers can see the required handoff shape without starting a governed run.

Live material phase behavior is safer: the helper exits before printing an approval handoff when required work-context fields are missing. The stable error code is `dogfood.helper.work_context_missing`.

The fix also prevents the specific failure mode that triggered this phase: an agent should no longer ask "Approve?" using only generic phase language when the runner has not produced a meaningful work summary.

## 6. Error Handling Assessment

Error handling is stable and non-leaking.

Missing work context fails closed with `dogfood.helper.work_context_missing` and does not print an approval handoff that could be mistaken for approval-ready output.

Secret-like work-context values fail with `dogfood.helper.usage`, and tests verify that the rejected value is not echoed through stdout or stderr.

The implementation preserves existing secret-like validation for run IDs, phases, approval reasons, and actors.

## 7. Privacy And Redaction Assessment

The fix preserves the intended redaction posture.

Work-context values are:

- bounded to one line;
- length-limited;
- rejected when secret-like;
- redacted before display as a defense-in-depth measure.

The implementation does not copy raw provider payloads, command output, logs, file contents, local secret values, tokens, credentials, authorization headers, private keys, report artifacts, or workflow state.

## 8. Test Quality Assessment

Focused test coverage is strong for the bug.

Tests cover:

- dry-run handoff includes work-context placeholders;
- AGENTS instructions require preserving complete handoffs and rejecting underspecified handoffs;
- live phase-start fails closed when work context is missing;
- supplied bounded work context appears in handoff output;
- secret-like work-context values are rejected without leakage;
- existing phase mapping, phase-close, approval-reason redaction, helper command display, and unsupported command behavior.

Existing limitation:

- Tests exercise the repo-local helper, not a full runtime approval contract. That is correct for this phase because runtime approval semantic changes were explicitly out of scope.

## 9. Documentation Review

Documentation now states:

- the work-summary approval-handoff bug is fixed;
- the repo-local phase runner requires bounded work context for live material phase starts;
- agents must not ask for approval from underspecified governed phase handoffs;
- runtime approval semantics are unchanged;
- automatic approvals, hidden approvals, shell execution, repo automation, persistence, artifacts, schemas, side effects, writes, hosted behavior, and release posture changes remain unsupported.

The runbook includes a copyable `phase-start` example with the new fields.

## 10. Blockers

No blockers found.

## 11. Non-Blocking Follow-Ups

- Consider a structured phase-context sidecar or manifest once the CLI flag list becomes too long for repeated daily use.
- Consider documenting the one-line and length constraints directly in the runbook so users understand why verbose context is rejected.
- Consider including missing field names in the fail-closed error output if that can be done without leaking user-supplied values.

## 12. Recommended Next Phase

Recommended next phase: resume broader governed roadmap work using the fixed approval handoff.

The immediate bug is resolved. The next governed phase should use the new work-context fields and should not proceed from any older underspecified run created before this fix.

## 13. Validation

Passed:

- `npm run test:dogfood-helper`
- `npm run check:docs`
- `git diff --check`
