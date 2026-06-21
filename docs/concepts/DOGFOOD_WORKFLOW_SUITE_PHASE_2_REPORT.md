# Dogfood Workflow Suite Phase 2 Report

## 1. Executive Summary

Dogfood Workflow Suite Phase 2 adds three new self-governance workflows for Workflow OS development:

- `dg/runtime-composition`
- `dg/blocker`
- `dg/release`

These workflows make the kernel more useful for the next implementation-heavy lanes: composing existing primitives into explicit runtime paths, fixing blockers without scope creep, and preparing release/readiness handoffs. They are local, approval-gated, sequential, and kernel-governed while Codex or a human remains the executor.

This phase does not add runtime automation, command execution, git automation, release publishing, write-capable adapters, hosted behavior, schemas, recursive agents, agent swarms, or Level 3/4 autonomy.

## 2. Scope Completed

Implemented workflow files:

- `dogfood/workflow-os-self-governance/workflows/runtime-composition.workflow.yml`
- `dogfood/workflow-os-self-governance/workflows/blocker-fix.workflow.yml`
- `dogfood/workflow-os-self-governance/workflows/release-hygiene.workflow.yml`

Updated documentation:

- `dogfood/workflow-os-self-governance/README.md`
- `dogfood/workflow-os-self-governance/tests/README.md`
- `docs/user-guide/self-governed-build-benchmark.md`
- `docs/user-guide/agent-harness-quickstart.md`
- `ROADMAP.md`

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic workflow generation;
- workflow catalog mutation;
- local command execution from the new workflows;
- automatic check execution;
- git automation;
- GitHub or PR automation;
- release tagging or publishing;
- package publishing;
- report artifact writing;
- persistence changes;
- CLI behavior changes;
- workflow schema changes;
- runtime side-effect execution;
- provider writes;
- write-capable adapters;
- recursive agents or agent swarms;
- hosted/distributed runtime behavior;
- release posture changes.

## 4. Workflow Summary

`dg/runtime-composition` governs phases that connect existing primitives into explicit runtime paths. It requires primitive inventory, explicit integration path scoping, approval before composition, validation disclosure, and a composition report.

`dg/blocker` governs focused blocker fixes. It requires the original blocker to be restated, the fix boundary to remain narrow, approval before edits, regression validation, and a fix report.

`dg/release` governs release hygiene and public-preview readiness. It requires release scope confirmation, public docs checks, approval before release handoff, validation disclosure, and a readiness report.

## 5. Governance Boundary

The new workflows are governed checklists with durable run identity and approval checkpoints. They do not own execution.

The intended operating model remains:

```text
Agent executes. Workflow OS governs.
```

Codex, Claude Code, or a human still performs repository edits, validation commands, git operations, PR actions, release actions, and maintainer judgment outside the kernel unless a reviewed explicit handler exists.

## 6. Validation Summary

Validation performed before edits:

- `npm run dogfood:benchmark -- validate --no-build` passed with expected experimental lifecycle warnings.
- `dg/implement` governance run `run/dg2` completed after approval `approval/run/dg2/implementation-approved`.

Validation performed after edits:

- `npm run dogfood:benchmark -- validate --no-build` passed with expected experimental lifecycle warnings.
- `npm run check:docs` passed.
- `dg/runtime-composition` smoke run `run/rc` paused at `approval/run/rc/composition-approved` and completed after approval.
- `dg/blocker` smoke run `run/bf` paused at `approval/run/bf/fix-approved` and completed after approval.
- `dg/release` smoke run `run/rh` paused at `approval/run/rh/release-approved` and completed after approval.

## 7. Privacy And Safety Posture

The new workflows only use bounded literal checkpoint text and existing `local/d` mockable dogfood skill references. They do not copy raw command output, provider payloads, secrets, private paths, report contents, or external system data.

The workflow definitions preserve the existing Level 2 approval posture and do not broaden autonomy.

## 8. Remaining Known Limitations

- These workflows are validation-covered only; there are no dedicated `.test.yml` dogfood tests yet.
- The repo-local helper still defaults to the legacy `dg/d` path.
- The new workflows do not execute real local checks; validation commands remain agent/human-executed outside the kernel unless a reviewed handler is supplied.
- The derived idempotency-key long-run-ID blocker found during this phase is fixed in [Derived Idempotency Key Bound Blocker Fix Report](DERIVED_IDEMPOTENCY_KEY_BOUND_BLOCKER_FIX_REPORT.md).

## 9. Recommended Next Phase

Recommended next phase: use `dg/runtime-composition` for the next runtime-composition implementation lane.

That is the best next step because it keeps the project focused on closing the gap between documented governance and enforced runtime paths without starting write-capable adapters or adding new primitive families.
