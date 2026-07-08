# Promotion Catalog Write Plan Report

## 1. Executive Summary

Promotion catalog write planning is complete. The plan defines the next
implementation boundary as an explicit opt-in
`author workflow promote --persist-catalog-record` path that writes one
validated local workflow catalog record after active promotion validation.

The plan keeps default promotion behavior unchanged and does not implement
catalog writes, runtime registration, archive metadata writes, schemas,
examples, providers, hosted behavior, external writes, or release posture
changes.

## 2. Scope Completed

- Created [Promotion Catalog Write Plan](../implementation-plans/promotion-catalog-write-plan.md).
- Defined the proposed CLI shape for opt-in promotion catalog persistence.
- Defined how supplied persisted stewardship decisions should be verified.
- Defined catalog record construction, write timing, failure, privacy, and test
  boundaries.
- Updated [Roadmap](../../ROADMAP.md) to identify opt-in promotion catalog write
  implementation as the next recommended phase.
- Updated [Workflow Catalog Persistence And Stewardship Integration Plan](../implementation-plans/workflow-catalog-persistence-plan.md)
  to link the new plan.

## 3. Scope Explicitly Not Completed

- No implementation.
- No promotion catalog writes.
- No archive metadata writes.
- No workflow runtime registration.
- No catalog repair.
- No workflow schema changes.
- No examples.
- No provider calls.
- No hosted behavior.
- No external writes.
- No release posture changes.

## 4. Planning Decisions

The plan recommends:

- keeping default `author workflow promote` unchanged;
- adding explicit `--persist-catalog-record`;
- accepting `--catalog-root` only when catalog persistence is requested;
- accepting `--stewardship-decision-id` only when catalog persistence is
  requested;
- allowing, but not requiring, a verified persisted stewardship decision in the
  first implementation;
- writing one validated `WorkflowCatalogRecord` only after active promotion
  validation succeeds;
- returning a stable partial-integration error if active promotion succeeds but
  the catalog sidecar write later fails.

## 5. Validation Summary

- `npm run check:docs` passed.

## 6. Governed Dogfood Summary

- workflow id: `dg/d`;
- run id: `run-1783532400584473000-2`;
- approval id: `approval/run-1783532400584473000-2/planning-approved`;
- approval outcome: granted by delegated maintainer after the complete approval
  handoff was emitted;
- final status: `Completed`;
- event summary:
  `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`,
  `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`,
  `RunValidated:1`, `SkillInvocationRequested:6`,
  `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`,
  `StepScheduled:6`;
- total events: 39.

Out-of-kernel work disclosed: repository documentation edits, docs validation,
git/PR actions, and this report were performed by the maintainer/agent outside
the kernel. The kernel governed the phase boundary and approval checkpoint.

## 7. Remaining Limitations

- The promotion command does not yet write workflow catalog records.
- Promotion does not yet cite persisted stewardship decisions.
- Catalog record update semantics are not designed.
- Archive metadata writes remain deferred.
- Runtime registration remains deferred.

## 8. Recommended Next Phase

Recommended next phase: opt-in promotion catalog write implementation.

The implementation should add `--persist-catalog-record` to
`author workflow promote`, use existing catalog model/store constructors, keep
default promotion unchanged, and avoid runtime registration, schemas, examples,
providers, hosted behavior, external writes, and release posture changes.
