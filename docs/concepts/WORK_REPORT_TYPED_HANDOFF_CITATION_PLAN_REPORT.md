# WorkReport Typed Handoff Citation Plan Report

## 1. Executive Summary

This planning phase defines how future WorkReports should cite typed handoffs.

The plan recommends a narrow model-only implementation: add WorkReport citation vocabulary for typed handoff references without changing report generation, report artifacts, runtime behavior, persistence, CLI behavior, schemas, nested harness execution, side-effect modeling, writes, reasoning lineage, or release posture.

## 2. Scope Completed

Completed:

- created WorkReport typed handoff citation plan;
- defined citation boundary;
- defined source-of-truth rules;
- defined privacy and redaction requirements;
- defined validation and test plan;
- recommended the next implementation phase.

## 3. Scope Explicitly Not Completed

Not implemented:

- WorkReport typed handoff citation target;
- report helper typed handoff inputs;
- runtime handoff generation;
- automatic report citation;
- typed handoff persistence;
- nested harness execution;
- CLI rendering;
- schemas;
- artifact behavior changes;
- side-effect modeling;
- writes;
- reasoning lineage.

## 4. Governance Summary

This planning phase was governed by the self-governance dogfood workflow before documentation edits.

- State directory: `/tmp/workflow-os-workreport-typed-handoff-citation-plan`
- Run ID: `run-1781546166681750000-2`
- Approval ID: `approval/run-1781546166681750000-2/d`
- Final status: `Completed`

## 5. Recommended Next Phase

WorkReport typed handoff citation target implementation, model vocabulary only.

The implementation should add citation vocabulary and tests only. It should not change report generation helpers, executor behavior, artifact writing, persistence, CLI, schemas, runtime handoff behavior, nested harness execution, side effects, writes, reasoning lineage, or release posture.

## 6. Commands Run

- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate`
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-workreport-typed-handoff-citation-plan --mock-all-local-skills run dg/d`
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-workreport-typed-handoff-citation-plan --mock-all-local-skills approve run-1781546166681750000-2 approval/run-1781546166681750000-2/d --actor codex --reason workreport-typed-handoff-citation-planning`

Docs validation is recorded in the final implementation report.
