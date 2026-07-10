# Dogfood Runner Approval-Presentation Enforcement Plan

Status: Implemented for the repo-local dogfood runner; default public approval behavior remains unchanged.

Related work:

- [Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md)
- [Approval Gate Presentation Opt-In Enforcement Plan](approval-gate-presentation-opt-in-enforcement-plan.md)
- [Dogfood Runner Approval-Presentation Persistence Plan](dogfood-runner-approval-presentation-persistence-plan.md)
- [Dogfood Runner Approval-Presentation Persistence Implementation Report](../concepts/DOGFOOD_RUNNER_APPROVAL_PRESENTATION_PERSISTENCE_IMPLEMENTATION_REPORT.md)
- [Dogfood Runner Approval-Presentation Enforcement Implementation Report](../concepts/DOGFOOD_RUNNER_APPROVAL_PRESENTATION_ENFORCEMENT_IMPLEMENTATION_REPORT.md)
- [Dogfood Approval-Presentation Freshness Enforcement Report](../concepts/DOGFOOD_APPROVAL_PRESENTATION_FRESHNESS_ENFORCEMENT_REPORT.md)
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md)

## 1. Executive Summary

Workflow OS now has the primitives needed to prove approval presentation before approval:

- bounded `ApprovalPresentationRecord` model;
- local approval-presentation record persistence;
- explicit opt-in executor approval enforcement through `decide_approval_with_presentation(...)`;
- repo-local dogfood runner persistence of the emitted approval handoff during material `phase-start` runs.

The previous dogfood gap was that the runner still printed and used the ordinary approval command. That meant the persisted proof existed, but dogfood approvals did not yet require it.

This plan defines the implemented narrow boundary: a repo-local dogfood approval path passes the persisted `presentation_id` into the existing opt-in enforcement boundary and fails closed when proof is missing, mismatched, corrupt, ambiguous, or older than the runner's bounded freshness policy.

The implementation is reported in [Dogfood Runner Approval-Presentation Enforcement Implementation Report](../concepts/DOGFOOD_RUNNER_APPROVAL_PRESENTATION_ENFORCEMENT_IMPLEMENTATION_REPORT.md).

## 2. Goals

- Make dogfood approvals use the existing opt-in approval-presentation enforcement path.
- Preserve explicit human or delegated-maintainer approval.
- Preserve default public approval behavior.
- Require persisted proof for material dogfood phase approvals.
- Bind approval decisions to the exact persisted handoff emitted by `phase-start`.
- Fail closed before approval events when proof is missing or invalid.
- Keep errors stable and non-leaking.
- Preserve current dogfood phase-start and phase-close reporting.
- Keep the runner repo-local and development-only.
- Prepare future public approval-card UX without implementing it now.

## 3. Non-Goals

This plan did not authorize:

- hidden approvals;
- automatic approvals;
- default `workflow-os approve` behavior changes;
- public CLI approval-card UX;
- workflow schema fields;
- examples;
- provider writes;
- side effects;
- report artifact writes;
- hosted or distributed runtime behavior;
- high-assurance approval controls beyond the existing opt-in proof gate;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Pre-Implementation Baseline

Implemented:

- `phase-start` validates the dogfood project and starts the mapped `dg/*` workflow.
- `phase-start` emits a complete `approval_handoff` and `copy_safe_approval_request`.
- Material `phase-start` requires bounded work context.
- Material `phase-start` persists one validated `ApprovalPresentationRecord`.
- The runner prints `presentation_id` and `presentation_content_hash`.
- Core exposes `LocalExecutor::decide_approval_with_presentation(...)` for explicit callers.

Before this implementation, the following were not implemented:

- dogfood approval command that passes a `presentation_id`;
- dogfood helper path that calls opt-in approval enforcement;
- dogfood default fail-closed behavior when approval proof is missing or stale;
- public approval-card rendering.

## 5. Implemented Boundary

The implementation adds the smallest repo-local dogfood approval helper path.

Implemented shape:

1. `scripts/self-governed-benchmark.mjs` persists approval-presentation proof during material `phase-start` runs.
2. `phase-start` prints an enforcement-ready hidden dogfood CLI command that includes `--presentation-id`.
3. The hidden CLI command `workflow-os dogfood approval-presentation approve` calls the existing core opt-in enforcement path.
4. The old public `workflow-os approve` command remains unchanged.
5. Dry-run behavior remains non-mutating and still marks proof as `not_persisted`.

## 6. Input Requirements

The dogfood approval helper must accept explicit inputs:

- project directory;
- state directory;
- run ID;
- approval ID;
- presentation ID;
- actor;
- reason;
- optional freshness policy if already supported by the core path.

It must not infer proof from:

- chat history;
- browser state;
- model memory;
- terminal scrollback;
- screenshots;
- hidden global state.

## 7. Enforcement Rules

For material dogfood approvals:

- missing `presentation_id` fails closed;
- missing proof record fails closed;
- mismatched run ID fails closed;
- mismatched approval ID fails closed;
- mismatched workflow/step identity fails closed;
- corrupt proof fails closed;
- ambiguous proof fails closed;
- stale proof fails closed through the runner's explicit max-age freshness policy;
- no approval decision event may be appended before proof validation succeeds.

On success, the helper must reuse the existing approval decision path through the reviewed opt-in enforcement boundary.

## 8. Error Handling

Errors must use stable codes and must not leak:

- approval handoff text;
- raw command output;
- provider payloads;
- token-like values;
- private keys;
- local filesystem secrets;
- raw source/spec contents;
- chat transcripts;
- screenshots.

Recommended stable error families:

- `dogfood.approval_presentation.presentation_required`;
- `dogfood.approval_presentation.presentation_invalid`;
- `dogfood.approval_presentation.presentation_mismatch`;
- `dogfood.approval_presentation.presentation_stale`;
- `dogfood.approval_presentation.approval_failed`.

If the core path already returns a stable enforcement error, the dogfood helper should preserve that code rather than wrap it in a less specific message.

## 9. Runner Output Policy

`phase-start` output should remain human-reviewable.

It should include:

- `approval_presentation_proof: persisted`;
- `presentation_id`;
- `presentation_content_hash`;
- an enforcement-ready approval command;
- the complete `approval_handoff`;
- the copy-safe approval request.

It should not print:

- raw record JSON by default;
- full state paths unless already part of current dogfood command posture;
- raw command output;
- hidden proof payloads;
- secret-like context.

## 10. Workflow Semantics

The runner remains governance coordination only.

It must not:

- approve automatically;
- run repo edits;
- run git operations;
- open or merge PRs;
- execute local checks;
- perform provider writes;
- append unrelated events;
- mutate report artifacts;
- enable side effects;
- change default executor behavior.

The only intended semantic change is inside the repo-local dogfood approval path: approval should require matching persisted presentation proof before the approval decision is accepted.

## 11. Test Plan

Implementation and follow-up tests should cover:

- material `phase-start` prints an enforcement-ready approval command;
- the command includes the persisted `presentation_id`;
- dogfood approval with matching proof succeeds;
- ordinary public approval behavior remains unchanged;
- missing `presentation_id` fails closed before approval events;
- mismatched `presentation_id` fails closed without leakage;
- stale proof fails when a freshness policy is configured in a future hardening slice;
- corrupt proof fails without leakage;
- no partial approval or resume events are appended on proof failure;
- `phase-close` still summarizes event posture correctly;
- dry-run remains non-mutating;
- no repo files, git state, provider state, report artifacts, schemas, or examples are mutated by the proof gate;
- existing dogfood helper tests pass;
- existing approval-presentation core/executor tests pass;
- `npm run check:docs` passes.

## 12. Documentation Updates

The implementation updates:

- this plan;
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md);
- [Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md), if needed;
- [Dogfood Runner Approval-Presentation Persistence Plan](dogfood-runner-approval-presentation-persistence-plan.md), if needed;
- `ROADMAP.md`;
- an end-of-phase implementation report under `docs/concepts/`.

Docs state:

- dogfood approval-presentation proof persistence is implemented;
- dogfood approval-presentation enforcement is implemented for the repo-local dogfood approval command;
- default public approval behavior is unchanged;
- automatic approvals are not implemented;
- public approval-card UI is not implemented;
- schemas, examples, provider writes, side effects, hosted behavior, reasoning lineage, and release posture changes are not implemented.

## 13. Open Questions

- Should material dogfood approvals require a freshness policy immediately, or only require identity/content matching first?
- Should the compatibility ordinary approval command remain visible for one phase, or should material dogfood output switch directly to the enforcement-ready command?
- Should the dogfood helper require explicit `--presentation-id`, or allow unambiguous lookup by run ID and approval ID?
- Should denied dogfood approvals require presentation proof in the same first implementation?
- Should phase-close disclose whether the approval used presentation enforcement?

## 14. Final Recommendation

The implementation phase is complete. Proceed next to dogfood runner
approval-presentation enforcement review.

Any broader public approval-card UX, default public approval enforcement,
freshness-policy configuration, high-assurance integration, write-capable
adapter work, schemas, examples, hosted behavior, reasoning lineage, and release
posture changes remain deferred.
