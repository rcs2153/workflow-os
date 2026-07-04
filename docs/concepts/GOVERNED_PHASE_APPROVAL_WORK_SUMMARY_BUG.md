# Governed Phase Approval Work Summary Bug

Status: Fixed. The original bug is preserved here as the problem statement and fix-forward record. The repo-local governed phase runner now supports bounded work-context fields and refuses live material phase approval handoffs when that context is missing.

## Summary

The governed phase runner emits a structured `approval_handoff` block and repo-level instructions require agents to preserve that block. However, a dogfood approval pause exposed a remaining gap: the handoff can still fail to explain the concrete work being approved.

Observed behavior:

```text
approval_allows: proceed with the accepted implementation phase only
```

That is structurally correct but not sufficient. The maintainer still had to ask:

```text
what work am i approving?
```

Approval checkpoints must be specific enough for a human to understand the actual proposed work without reconstructing intent from prior chat context, roadmap inference, or agent memory.

## Why This Matters

This is a governance bug, not merely a wording issue.

If the approval block does not include the work being approved, then:

- the approval can look like a generic phase permission instead of a bounded work authorization;
- the maintainer cannot confirm scope, non-goals, touched surfaces, or validation expectations at the approval moment;
- an agent can accidentally continue under an approval that was not meaningfully informed;
- the kernel dogfood loop fails to model the approval UX expected for real users;
- future enterprise approval gates would inherit a weak human-review boundary.

Workflow OS must make the approval boundary legible. A governed approval should answer:

```text
What work am I approving?
What is explicitly not approved?
What files, systems, or surfaces may be touched?
What validation will prove the work stayed inside scope?
What happens after approval?
```

## Existing Fixes That Are Not Enough

Two related P0 fixes already exist:

- [Governed Phase Approval Handoff Context Bug](GOVERNED_PHASE_APPROVAL_HANDOFF_CONTEXT_BUG.md): fixed runner-side handoff emission.
- [Governed Phase Approval Handoff Preservation Bug](GOVERNED_PHASE_APPROVAL_HANDOFF_PRESERVATION_BUG.md): fixed repo-instruction preservation so agents do not collapse the block into vague prose.

Those fixes ensure a handoff block exists and must be relayed. They do not require the block to contain a concrete work summary, approved scope, expected touched surfaces, or validation expectations.

## Required Fix

The governed phase runner should require explicit, bounded work-context fields before presenting an approval handoff for material roadmap work.

The handoff should include at least:

- `work_summary`: one or two bounded sentences describing the concrete work proposed;
- `approved_scope`: bullet-like bounded scope statement;
- `strict_non_goals`: explicit forbidden scope;
- `expected_touched_surfaces`: likely files, docs, modules, commands, or repo areas;
- `validation_required`: required checks before phase close;
- `why_now`: short link to roadmap/review/bug/plan that justifies the phase;
- `approval_allows`: what approval unlocks;
- `approval_does_not_allow`: existing explicit non-scope posture;
- `next_action_after_approval`: bounded next action.

The runner should fail closed or print `work_context_missing: true` when those fields are not supplied for material phases. Agents must not ask for approval from an underspecified handoff.

## Privacy And Redaction

Work-context fields must be bounded and redaction-safe.

They must not include:

- raw provider payloads;
- raw command output;
- raw logs;
- raw spec contents;
- raw file contents;
- local secret values;
- tokens, credentials, authorization headers, private keys, or secret-like values;
- unbounded natural-language dumps.

Missing context must be explicit as `not supplied` or `not available`; the kernel must not fabricate work descriptions, touched surfaces, or validation claims.

## Acceptance Criteria

- The roadmap tracks this as a P0 fix before further long-running governed implementation phases.
- A plan exists for requiring work-summary fields in governed phase approval handoffs.
- `phase-start` supports bounded work context for material phases.
- If material phase work context is missing, the runner does not produce an approval request that looks ready for approval.
- Tests verify approval handoff output includes work summary, approved scope, non-goals, touched surfaces, validation, and why-now fields.
- Tests verify secret-like work-context values are rejected or redacted.
- `AGENTS.md` instructs agents not to request approval when a handoff lacks concrete work context.
- The fix does not change runtime approval semantics.
- The fix does not add automatic approval, hidden approval, repo/git/PR automation, shell execution, persistence, report artifacts, schemas, side effects, writes, hosted behavior, or release posture changes.

## Fix Implemented

Implemented behavior:

- `phase-start` supports bounded `--work-summary`, `--approved-scope`, `--strict-non-goals`, `--expected-touched-surfaces`, `--validation-required`, and `--why-now` fields.
- Dry-run handoffs show placeholder field names so agents and maintainers can see what must be supplied.
- Live material phase starts fail closed with `dogfood.helper.work_context_missing` when required work context is missing.
- Work-context values are bounded to one line and rejected when secret-like.
- The emitted `approval_handoff` now includes concrete work context before `approval_allows`.
- `AGENTS.md` instructs agents not to ask for approval from underspecified handoffs.

## Recommended Next Phase

Recommended next phase: governed phase approval work-summary handoff review.

Review should confirm that material phase approvals now answer "what work am I approving?" without changing runtime approval semantics, automatic approvals, repository automation, shell execution, persistence, artifacts, schemas, side effects, writes, hosted behavior, or release posture.
