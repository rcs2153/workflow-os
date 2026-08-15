# Authorized Execution Continuity Plan Report

## 1. Executive Summary

Workflow OS now has a P0 plan for preserving lawful external work across agent
turn boundaries without misclassifying executor yield as workflow completion or
approval wait. The plan keeps kernel governance distinct from host scheduling
and preserves fresh one-time source-backed authorization for every material
operation.

## 2. Scope Completed

- Defined actionable approval-gate readiness.
- Defined bounded durable execution-window posture.
- Defined executor yield as non-terminal workflow state.
- Defined typed wait conditions and wake posture.
- Defined authoritative resume dispositions.
- Defined delegated approval capability constraints.
- Defined kernel and host-supervisor responsibilities.
- Defined failure, recovery, privacy, and test posture.
- Sequenced a model phase, security review, and local end-to-end proof.

## 3. Scope Explicitly Not Completed

- No Rust model or runtime implementation.
- No event or snapshot changes.
- No host scheduler or worker integration.
- No automatic approval or self-approval.
- No provider mutation broadening.
- No nested harness runtime.
- No workflow schema, SDK, CLI automation, UI, hosted, or release changes.

## 4. Architecture Decision

The plan distinguishes a durable execution window from a reusable execution
credential. The window survives executor turns for scheduling and audit. Every
material operation still requires a fresh, private, same-call, cursor-bound
continuation directive and one-time claim.

## 5. Gate Readiness Decision

An approval gate with unmet evidence, checks, policy, authority,
presentation-proof, separation-of-duty, immutable-binding, or SideEffect
prerequisites is pending and not actionable. Approval cannot satisfy its own
prerequisites.

## 6. Runtime Continuity Decision

Only durable kernel state may declare a run runnable, waiting, blocked, or
terminal. An agent turn ending while a lawful execution window remains open is
executor yield. A final response cannot make the run terminal.

## 7. Host Boundary

Core can produce durable resume-required posture and exact resume directives.
An integrated host supervisor must observe that posture and schedule an
executor. The plan does not claim that the local Core library can create a new
agent turn by itself.

## 8. Privacy And Security Summary

The plan rejects reusable serialized authority, model-inferred delegation,
human-gate bypass, evidence-by-approval substitution, and raw transcript or
payload storage. Delegated approval remains scoped, expiring, revocable, and
subject to separation-of-duty requirements.

Independent architecture review additionally requires atomic cursor-bound
yield/directive state operations, fresh owner-record revalidation instead of a
stored readiness boolean, continuation attempt/outcome posture for
crash-after-claim ambiguity, and deferral of general delegated approval until
parent-grant attenuation and revocation chains can be proven.

## 9. Governed Planning Record

- workflow: `dg/d`
- run: `run-1786797789530355000-2`
- approval: `approval/run-1786797789530355000-2/planning-approved`
- presentation: `presentation/21ff9aa70060a687`
- approval outcome: granted by the delegated maintainer after full handoff
  review
- planning execution: performed outside the kernel; the kernel governed scope
  and approval but did not inspect code or write documentation

## 10. Validation

- `npm run check:docs`: passed after final security-review reconciliation.
- `git diff --check`: passed after final security-review reconciliation.
- independent architecture/security analysis: completed and incorporated.
- final docs and diff validation: complete.

## 11. Remaining Limitations

- Current runtime does not persist execution windows, yields, or typed waits.
- Current host integration does not automatically resume yielded executors.
- Existing `WorkflowRunStatus` compatibility posture needs review before event
  integration.
- Delegated approval capability is not operationally wired to this lane.

## 12. Recommended Next Phase

Perform focused maintainer/security review of the plan. If accepted, implement
the core continuity decision model and validation first, then proceed directly
to one local injected-supervisor continuity vertical slice.

Fix-forward: focused review is complete and accepts the plan in
[Authorized Execution Continuity Plan Review](AUTHORIZED_EXECUTION_CONTINUITY_PLAN_REVIEW.md).
Proceed next to the core continuity decision model only.
