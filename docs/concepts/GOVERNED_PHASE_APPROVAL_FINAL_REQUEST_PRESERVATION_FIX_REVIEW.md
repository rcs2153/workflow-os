# Governed Phase Approval Final Request Preservation Fix Review

## 1. Executive Verdict

Blocker fixed with non-blocking follow-ups.

The fix addresses the repeated approval-boundary failure where `phase-start` emitted a complete governed handoff, the agent relayed it in commentary, and the final approval request still collapsed into vague prose. The repo-local phase runner now emits a dedicated `copy_safe_approval_request` artifact intended to be used as the final user-facing approval request. Tests, repo instructions, runbook docs, roadmap posture, and the fix-forward bug record align with that behavior.

Recommended next phase: first provider write candidate planning.

## 2. Scope Verification

The fix stayed within approved blocker-fix scope.

Implemented:

- copy-safe approval request output from `phase-start`;
- focused helper regression tests;
- `AGENTS.md` instruction update;
- self-governed build benchmark runbook update;
- roadmap update;
- final-request preservation bug record;
- blocker fix report.

No accidental implementation was introduced for:

- runtime approval semantic changes;
- automatic approvals;
- hidden approvals;
- repository automation;
- git or PR automation;
- shell execution by the kernel;
- persistence;
- report artifacts;
- schemas;
- side-effect modeling;
- writes;
- hosted behavior;
- release posture changes.

## 3. Original Blocker Restatement

The blocker was not that the kernel failed to emit approval context. It did.

The blocker was that the final agent response could still omit that context. A complete handoff in terminal output or commentary did not guarantee the final user-facing approval request preserved:

- workflow ID;
- phase;
- run ID;
- approval ID;
- status;
- approval reason;
- work summary;
- approved scope;
- strict non-goals;
- touched surfaces;
- validation expectations;
- why-now rationale;
- next action;
- approval command posture.

That made a governed checkpoint look like ordinary conversational permission.

## 4. Fix Approach Assessment

The approach is minimal and appropriate for a repo-local dogfood helper.

The fix leaves `approval_handoff` intact and adds a second output artifact:

- `copy_safe_approval_request_required: true`
- `copy_safe_approval_request`

The copy-safe request is formatted as the exact final approval request an agent can paste back to the maintainer. It repeats the handoff inside a fenced YAML block and includes a direct approval ask.

This is stronger than more documentation because the helper now produces the final-message surface directly. It is still weaker than a typed response linter or product-level approval UI, but that is acceptable for the current repo-local dogfood boundary.

## 5. Validation Boundary Assessment

The helper remains governance coordination only.

It does not:

- approve the run;
- alter runtime approval semantics;
- execute local commands through the kernel;
- write reports;
- mutate git state;
- perform PR actions;
- persist artifacts.

Live material phase starts still fail closed when required work context is missing. The copy-safe request inherits the existing bounded and redacted work-context values.

## 6. Final Approval Request Assessment

The final approval request surface is now explicit.

Verified behavior:

- `phase-start` emits `approval_handoff_required: true`;
- `phase-start` emits `copy_safe_approval_request_required: true`;
- the copy-safe request includes the full handoff;
- the copy-safe request includes a direct approval ask;
- the copy-safe request keeps the approval reason in the displayed command redacted;
- `AGENTS.md` states commentary is not enough if the final response collapses into vague prose;
- `AGENTS.md` requires the copy-safe approval request as the final approval request when emitted.

This directly addresses the repeated failure mode.

## 7. Privacy And Redaction Assessment

The existing redaction posture is preserved.

The copy-safe block uses the same `redactSecretLike(...)` and `displayCommand(...)` paths as the primary handoff. Approval command reason values remain redacted in output.

No raw secrets, tokens, provider payloads, command output, report artifacts, or side-effect payloads are introduced by the fix.

## 8. Test Quality Assessment

Focused tests cover the important regression surface:

- dry-run phase start emits the approval handoff;
- dry-run phase start emits the copy-safe request;
- copy-safe request contains workflow ID, run ID, approval ID, and final approval ask;
- copy-safe request tells the agent to preserve the complete block in the final approval request;
- displayed approval command reason stays redacted;
- repo agent instructions require handoff preservation;
- repo agent instructions require copy-safe final approval request usage.

The tests are appropriate for the repo-local helper boundary.

Non-blocking gap: there is still no automated linter that can inspect an agent's actual chat final response and reject vague approval text. The helper now gives the agent a precise artifact to preserve, but final response compliance remains a human/process boundary.

## 9. Documentation Review

Documentation is accurate.

Verified docs state:

- the final-request preservation bug is fixed;
- the helper emits `copy_safe_approval_request_required: true`;
- agents must use the copy-safe approval request in final approval asks;
- earlier commentary is not sufficient;
- runtime approval semantics are unchanged;
- automatic approvals, persistence, schemas, side effects, writes, hosted behavior, and release posture changes remain unimplemented.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Consider a typed approval handoff model if approval handoffs become product surface beyond repo-local dogfood tooling.
- Consider an agent response linter or checklist if repeated final-response degradation continues despite the copy-safe output artifact.
- Keep future governed phase reports disclosing whether the copy-safe approval request was preserved in the final approval request.

## 12. Recommended Next Phase

Recommended next phase: first provider write candidate planning.

Why: the final-request preservation blocker is fixed and reviewed. The write adapter preflight helper was accepted, and the roadmap now recommends choosing the first low-risk provider write candidate before any provider mutation implementation.

## 13. Validation

Commands required for this review:

- `npm run test:dogfood-helper` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

## 14. Dogfood Governance Summary

This review was governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1783195729264355000-2`
- Approval ID: `approval/run-1783195729264355000-2/review-scope-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Terminal: true
- Events total: 39
- Approvals: 1
- Retries: 0
- Escalations: 0
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: repository file inspection, review document creation, helper/docs validation, and phase-close inspection were performed by the agent outside kernel execution. No git or PR action was performed during this review phase.
