# Dogfood Approval-Presentation Denial Proof Plan

Status: Planned; not implemented.

Related work:

- [Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md)
- [Dogfood Runner Approval-Presentation Enforcement Plan](dogfood-runner-approval-presentation-enforcement-plan.md)
- [Dogfood Approval-Presentation Freshness Enforcement Report](../concepts/DOGFOOD_APPROVAL_PRESENTATION_FRESHNESS_ENFORCEMENT_REPORT.md)
- [Dogfood Approval-Presentation Freshness Enforcement Review](../concepts/DOGFOOD_APPROVAL_PRESENTATION_FRESHNESS_ENFORCEMENT_REVIEW.md)
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md)

## 1. Executive Summary

The repo-local dogfood approval path now requires persisted, fresh
approval-presentation proof for granted approvals. The remaining symmetry
question is whether denied dogfood approvals should require the same proof.

This plan recommends implementing denial-path proof enforcement for material
dogfood approvals. Denials affect governed execution just as grants do: they can
stop work, fail a run closed, and become part of the durable event trail. A
denial should therefore also prove that the reviewer saw the exact bounded
approval scope before making the decision.

This plan does not implement denial-path enforcement.

## 2. Goals

- Require persisted approval-presentation proof for material dogfood denials.
- Preserve explicit human or delegated-maintainer denial.
- Preserve default public approval behavior.
- Preserve the existing public `workflow-os approve --deny` path.
- Keep denial proof repo-local to the dogfood helper.
- Fail closed before denial events when proof is missing, mismatched, corrupt,
  ambiguous, or stale.
- Preserve stable non-leaking errors.
- Preserve current dogfood phase-start and phase-close reporting.
- Avoid changing workflow schemas, examples, side effects, writes, hosted
  behavior, or release posture.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- hidden approvals or hidden denials;
- automatic approvals or automatic denials;
- default public approval behavior changes;
- public approval-card UI;
- workflow schema fields;
- examples;
- provider writes;
- side effects;
- report artifact writes;
- hosted or distributed runtime behavior;
- high-assurance approval controls beyond existing explicit opt-in paths;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented today:

- material dogfood `phase-start` persists one bounded
  `ApprovalPresentationRecord`;
- material dogfood output prints a proof-enforced approval command;
- that command includes `--presentation-id`;
- that command includes `--max-presentation-age-ms 86400000`;
- granted dogfood approvals use the hidden
  `workflow-os dogfood approval-presentation approve` path;
- stale granted approval proof fails closed before `ApprovalGranted` events;
- default public approval behavior remains unchanged.

Open asymmetry:

- denied dogfood approvals can still be performed through the same hidden
  command using `--deny`, but the current test coverage and runner posture are
  centered on granted approval proof.

## 5. Recommended Semantics

For material dogfood denials, the hidden dogfood approval-presentation command
should require the same proof inputs as grants:

- run ID;
- approval ID;
- presentation ID;
- bounded max presentation age;
- actor;
- denial reason;
- `--deny`.

The command should call the same
`LocalExecutor::decide_approval_with_presentation(...)` boundary. The only
semantic difference should be `ApprovalDecisionKind::Denied`.

The denial should fail closed before appending denial or run-failure events if
presentation proof is missing, mismatched, corrupt, ambiguous, or stale.

## 6. Error Handling

Denial proof failures should preserve the existing approval-presentation
enforcement error family where possible.

Errors must not leak:

- approval handoff text;
- presentation record payloads;
- presentation IDs in stale or mismatch errors;
- raw command output;
- provider payloads;
- source/spec contents;
- chat transcripts;
- screenshots;
- token-like values;
- credentials or private keys.

If the hidden dogfood command needs a dogfood-specific wrapper, the wrapper must
preserve stable error codes and avoid less specific messages.

## 7. Event Semantics

On successful proof-validated denial:

- the approval denial event should be appended through the existing executor
  path;
- the run should fail closed according to existing denial semantics;
- the approval event should carry the existing proof marker posture where the
  core path already records one;
- phase-close should report proof enforcement consistently.

On failed proof validation:

- no denial event should be appended;
- no run terminal transition should occur;
- the run should remain waiting for approval;
- no post-terminal events should be fabricated.

## 8. Test Plan

Future implementation should add focused tests for:

- dogfood denial with matching fresh proof fails the run closed through the
  existing denial semantics;
- denied approval event carries proof marker posture when supported by the core
  path;
- stale denial proof fails with
  `approval_presentation_enforcement.proof_stale`;
- mismatched denial proof fails before denial events;
- missing denial proof fails before denial events;
- stale/mismatched/missing denial errors do not leak presentation IDs or
  handoff text;
- failed proof validation leaves the run `WaitingForApproval`;
- failed proof validation appends no `ApprovalDenied` or terminal run events;
- ordinary public `workflow-os approve --deny` behavior remains unchanged;
- existing granted dogfood approval proof tests still pass;
- `npm run test:dogfood-helper` passes;
- focused CLI tests pass;
- `cargo test --workspace` passes;
- `npm run check:docs` passes.

## 9. Documentation Updates

Future implementation should update:

- this plan;
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md);
- [Dogfood Runner Approval-Presentation Enforcement Plan](dogfood-runner-approval-presentation-enforcement-plan.md), if needed;
- `ROADMAP.md`;
- an implementation report under `docs/concepts/`.

Docs must state:

- dogfood denial proof enforcement is repo-local;
- default public approval and denial behavior remains unchanged;
- automatic denials are not implemented;
- public approval-card UI is not implemented;
- schemas, examples, provider writes, side effects, hosted behavior, reasoning
  lineage, and release posture changes are not implemented.

## 10. Recommended Implementation Sequence

1. Add focused CLI coverage for proof-validated denial success.
2. Add focused CLI coverage for stale/missing/mismatched denial proof failure.
3. Confirm the hidden dogfood command already routes `--deny` through
   `decide_approval_with_presentation(...)`.
4. Update helper/docs only if the command output needs clearer denial guidance.
5. Run focused tests and full validation.
6. Create an implementation report.
7. Run a maintainer review before deeper approval or write-adjacent work.

## 11. Open Questions

- Should the dogfood helper print a separate copy-safe denial command, or is the
  existing `--deny` flag sufficient?
- Should phase-close distinguish granted proof enforcement from denied proof
  enforcement in its event summary?
- Should denial proof enforcement be required for all dogfood phases or only
  material phases?
- Should denial proof be allowed after the freshness window if the denial reason
  explicitly says the presentation is stale?

## 12. Final Recommendation

Proceed to dogfood approval-presentation denial proof implementation.

Keep the implementation narrow:

- hidden dogfood path only;
- explicit `--deny` only;
- proof-enforced and freshness-bounded;
- no public approval behavior changes;
- no schemas, examples, provider writes, side effects, hosted behavior,
  reasoning lineage, or release posture changes.

## 13. Dogfood Governance

Workflow OS governed this planning phase:

- workflow: `dg/d`;
- run: `run-1783703619644415000-2`;
- approval: `approval/run-1783703619644415000-2/planning-approved`;
- approval presentation: `presentation/1be89d062dd01408`;
- approval outcome: granted;
- approval reason: `approved-denial-proof-planning-scope`;
- close status: `Completed`;
- event summary:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`;
- approval presentation enforcement: proof-enforced;
- persisted approval presentation records: 1.

Codex performed documentation edits and validation outside the kernel. The
kernel governed the planning approval boundary.
