# Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Plan Report

## 1. Executive Summary

This planning phase defines the next narrow bridge for workflow-declared high-assurance report artifact requirements.

The new plan recommends a pure runtime derivation helper that maps loaded workflow declarations from `report_artifact_requirements.high_assurance_approval` into the existing explicit `WorkReportArtifactHighAssuranceDisclosurePolicy`. The plan keeps current validation honest: enforcement postures remain semantically rejected until the derivation helper is implemented, reviewed, and explicitly integrated into artifact-capable runtime paths.

## 2. Scope Completed

- Created [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Plan](../implementation-plans/workflow-declared-high-assurance-artifact-requirement-runtime-derivation-plan.md).
- Defined the source-of-truth boundary between workflow declarations, semantic validation, derivation, artifact gate policy, and artifact persistence.
- Recommended a pure, local, deterministic derivation helper.
- Recommended keeping current semantic validation rejection unchanged in the first implementation.
- Defined future policy composition posture for explicit caller policy and workflow-declared policy.
- Updated adjacent roadmap, report artifact, high-assurance disclosure, workflow artifact requirement, schema, and workflow spec docs to link to the plan.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- runtime derivation from workflow specs;
- semantic validation relaxation;
- executor artifact path integration;
- automatic report generation;
- automatic artifact writing;
- CLI behavior;
- examples;
- runtime config;
- workflow-declared high-assurance approval controls;
- governance profile policy composition;
- RBAC/IdP;
- quorum approval;
- revocation enforcement;
- approval evidence attachment;
- workflow event or audit projection for derivation;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage;
- release posture changes.

## 4. Plan Summary

The plan recommends the next implementation phase:

```text
pure workflow-declared artifact gate derivation helper
```

The helper should derive:

- absent requirement -> disabled policy;
- `not_required` -> disabled policy;
- `disclosure_required` -> require disclosure policy;
- `validated_disclosure_required` -> require validated disclosure policy;
- `validated_fail_closed_disclosure_required` -> require validated fail-closed disclosure policy.

The helper must not generate reports, write artifacts, read state backends, inspect approval payloads, inspect workflow events, mutate runtime state, append events, or alter workflow pass/fail semantics.

## 5. Validation Summary

Validation commands run:

- `npm run check:docs` passed.
- `git diff --check` passed.

No Rust code changed in this planning phase, so Rust checks were not rerun for the plan-only edits.

## 6. Dogfood Governance

This planning phase was governed by the local Workflow OS dogfood runner.

- workflow phase: planning
- workflow ID: `dg/d`
- run ID: `run-1783136588378262000-2`
- approval ID: `approval/run-1783136588378262000-2/planning-approved`
- approval outcome: approved by the maintainer before planning work continued
- close status: completed
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`

The dogfood runner coordinated governance only. Documentation edits and validation commands were performed by the executor.

## 7. Remaining Known Limitations

- Workflow enforcement postures remain semantically rejected with `validation.workflow.report_artifact_requirement.runtime_not_enforced`.
- No runtime path derives workflow declarations into artifact gate inputs.
- No executor path combines workflow-declared policy with caller-supplied artifact gate policy.
- No artifact is generated or written automatically.

## 8. Recommended Next Phase

Recommended next phase: **pure workflow-declared artifact gate derivation helper implementation**.

That phase should add only the derivation helper and focused tests. It should keep semantic validation rejection for enforcement postures unchanged until the helper is reviewed and explicit executor artifact-path integration is planned.
