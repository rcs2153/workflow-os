# Dogfood Workflow Discovery Workflow Report

## 1. Executive Summary

The dogfood suite now includes `dg/workflow-discovery`, a local, approval-gated Workflow OS workflow for recommendation-only workflow discovery.

This phase responds to a concrete dogfood lesson: `dg/branch-cleanup` was a workflow the kernel should have recommended from repeated PR, merge, branch-protection, and cleanup friction. Workflow OS should not only govern workflows humans already know to run. It should help identify when repeated work patterns deserve a governed workflow boundary.

The implemented workflow is intentionally conservative. It discovers and recommends; it does not generate workflow files, register workflows, mutate specs, modify roadmap state, inspect GitHub, approve itself, or resolve workflow conflicts automatically.

## 2. Scope Completed

- Added `dogfood/workflow-os-self-governance/workflows/workflow-discovery.workflow.yml`.
- Added the `dg/workflow-discovery` workflow to the self-governance dogfood suite.
- Documented workflow discovery in the dogfood README.
- Documented workflow discovery in the self-governed build benchmark runbook.
- Documented workflow discovery in the agent harness quickstart.
- Updated the roadmap to mark the first local recommendation-only workflow discovery step as implemented.
- Updated dogfood test documentation to include workflow discovery project validation coverage.

## 3. Scope Explicitly Not Completed

- No automatic workflow file generation.
- No automatic workflow registration.
- No workflow catalog model.
- No workflow conflict taxonomy implementation.
- No workflow proposal model.
- No roadmap mutation.
- No spec mutation.
- No GitHub inspection.
- No command execution.
- No repository edits from inside the kernel.
- No automatic approval.
- No recursive agents, agent swarms, hosted behavior, production self-hosting, or Level 3/4 autonomy.

## 4. Workflow Summary

`dg/workflow-discovery` is a Level 2 manual workflow with the same local/d skill and approval/d policy posture as the existing dogfood workflows.

It contains these checkpoints:

- `discovery-scope-readiness`
- `signal-source-inventory`
- `repeated-pattern-analysis`
- `workflow-overlap-review`
- `recommendation-draft`
- `stewardship-approved`
- `recommendation-handoff`
- `discovery-report`

The `stewardship-approved` checkpoint requires explicit approval before recommendations become roadmap or implementation handoff material.

## 5. Governance Boundary

The workflow governs recommendation quality, not workflow creation.

The kernel records the governed discovery path, approval checkpoint, and report posture. Codex or a human still performs any accepted follow-up planning, roadmap update, or workflow implementation outside the kernel.

## 6. Recommendation Model Summary

The workflow asks the operator or agent to inventory bounded signals:

- recent reports;
- roadmap changes;
- dogfood runs;
- repeated manual handoffs;
- recurring review findings;
- missing gates;
- missing evidence;
- unclear handoffs;
- branch, PR, validation, report, or cleanup friction.

It then asks for recommendation review across:

- create workflow;
- change workflow;
- split workflow;
- merge or relate workflows;
- retire workflow;
- add policy gate;
- add approval requirement;
- add evidence requirement;
- add typed handoff;
- add report requirement;
- flag conflict or overlap.

Recommendations must remain bounded and reviewable. They should not become active workflows without explicit follow-up implementation and review.

## 7. Validation Summary

Validation commands run:

- `npm run dogfood:benchmark -- validate --no-build`
- `npm run check:docs`
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-workflow-discovery-smoke --mock-all-local-skills run dg/workflow-discovery --run-id run/wd`
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-workflow-discovery-smoke --mock-all-local-skills approve run/wd approval/run/wd/stewardship-approved --actor user/dogfood-reviewer --reason smoke-approved-workflow-discovery`

Results:

- Dogfood project validation passed with expected experimental lifecycle warnings.
- Docs check passed.
- Smoke run reached `WaitingForApproval` at `approval/run/wd/stewardship-approved`.
- Smoke approval completed the run with status `Completed`.

## 8. Privacy And Safety Posture

The workflow uses bounded literal checkpoint disclosures. It does not store raw git logs, GitHub payloads, tokens, credentials, command output, private notes, or workflow-run internals as evidence. Any discovery inventory remains a human/agent disclosure unless a future scoped evidence, report artifact, or workflow catalog path is implemented.

## 9. Remaining Limitations

- Discovery is manual and recommendation-only.
- No workflow catalog model exists yet.
- No conflict taxonomy is implemented yet.
- No automated signal extraction from audit records, reports, hooks, approvals, side-effect records, or run history is implemented.
- No draft workflow proposal model exists yet.
- No promotion workflow exists yet.
- No collaboration registry exists yet.

## 10. Recommended Next Phase

Run `dg/workflow-discovery` periodically as part of the self-governed build benchmark. Use it to identify repeated governance friction and propose next workflow boundaries, then implement accepted recommendations as separate scoped phases.
