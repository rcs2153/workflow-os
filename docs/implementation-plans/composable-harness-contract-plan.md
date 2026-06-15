# Composable Harness Contract Plan

Status: Core model implemented. Composable Harness Contracts are not implemented as runtime behavior, schemas, CLI behavior, domain packs, nested execution, hosted execution, distributed workers, side-effect modeling, writes, or Level 3/4 autonomy.

## 1. Executive Summary

Workflow OS now has enough governed-work foundation to plan the next contract layer:

- workflow and run identity;
- deterministic validation;
- durable state and append-only event-log concepts;
- EvidenceReference core model and selected attachment paths;
- WorkReportContract and WorkReport models;
- explicit terminal local report helper/result/artifact APIs;
- local check result references and WorkReport local check citations.

The next roadmap question is how Workflow OS should eventually model bounded harnesses inside workflows.

Composable Harness Contracts are governed execution-envelope contracts. They should allow a workflow to decompose complex work into specialized harnesses with typed inputs, typed outputs, scoped authority, evidence requirements, approval rules, failure semantics, and traceable handoffs.

The first implementation adds a model-only `HarnessContract` family with validation, serde, redaction-safe debug behavior, and focused tests. It does not implement nested harness execution or runtime scheduling.

## 2. Goals

- Define the domain-neutral core concept for Composable Harness Contracts.
- Keep Workflow OS positioned as a governed work runtime, not a generic multi-agent framework.
- Separate harness contracts from nested harness execution.
- Define what a future harness contract must declare.
- Preserve scoped authority and typed handoffs.
- Require evidence, policy, approval, validation, and work-report relationships to remain explicit.
- Identify dependencies that must remain stable before implementation.
- Recommend a small future implementation phase.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- arbitrary recursive agent spawning;
- agent swarm positioning;
- nested harness execution;
- runtime scheduling of harnesses;
- hosted or distributed runtime behavior;
- schema changes;
- CLI behavior;
- examples;
- domain packs;
- reasoning lineage implementation;
- side-effect boundary implementation;
- live write integrations;
- write-capable adapters;
- production compliance systems;
- Level 3 or Level 4 autonomy claims;
- replacement of deterministic governance with model self-review;
- release posture changes.

## 4. Governance Check

This planning phase was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-composable-harness-contract-plan`
- Run ID: `run-1781543759473578000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781543759473578000-2/d`
- Final status: `Completed`

The governed run completed before documentation edits were made.

## 5. Why This Belongs In Workflow OS

Enterprises will increasingly decompose AI-assisted work across specialized systems, tools, deterministic checks, human reviews, and reasoning actors.

The hard problem is not agent-to-agent delegation. The hard problem is governed delegation:

- who or what has authority;
- what context is required;
- what state and evidence are carried forward;
- what side effects are allowed;
- which policy gates and approvals apply;
- how failures are represented;
- how handoffs remain typed and auditable;
- how final reports cite what happened without copying raw payloads.

Workflow OS should be the governed substrate for that delegation. It should not become agents managing agents.

## 6. What A Harness Is

A harness is a bounded execution envelope inside a workflow.

A harness is not synonymous with an agent. A harness may contain:

- deterministic code;
- a skill;
- a model-backed actor;
- adapter reads;
- local checks;
- policy checks;
- validation;
- human approval;
- work report generation;
- a future side-effect boundary.

The contract defines the envelope. It should not imply that the implementation inside the envelope is autonomous, recursive, distributed, or write-capable.

## 7. Candidate Core Model

A future model-only phase may introduce domain-neutral types such as:

- `HarnessContract`;
- `HarnessContractId`;
- `HarnessContractVersion`;
- `HarnessPurpose`;
- `HarnessInputRequirement`;
- `HarnessContextRequirement`;
- `HarnessToolAllowance`;
- `HarnessAuthorityScope`;
- `HarnessSideEffectAllowance`;
- `HarnessOutputRequirement`;
- `HarnessEvidenceRequirement`;
- `HarnessApprovalRequirement`;
- `HarnessExecutionPolicy`;
- `HarnessFailureSemantics`;
- `HarnessHandoffRequirement`.

The exact names should follow repository conventions at implementation time. The first implementation should add only the smallest set needed to validate a contract shape.

## 8. Required Contract Fields

A future harness contract should eventually define:

- harness name or ID;
- contract version;
- purpose;
- allowed inputs;
- required context;
- allowed tools or skill capabilities;
- scoped authority;
- allowed side effects;
- output schema or output requirement;
- evidence requirements;
- approval policy;
- timeout, budget, and retry policy;
- failure semantics;
- handoff requirements;
- sensitivity;
- redaction policy.

The first model-only phase should prefer explicit enums and validated bounded strings over flexible maps.

## 9. Typed Handoff Boundary

Typed handoffs are the primary reason harness contracts belong in Workflow OS.

A handoff should eventually carry structured references such as:

- artifacts or outputs produced by the previous harness;
- evidence references;
- validation diagnostics;
- local check result references;
- work report citations;
- known limitations;
- incomplete work disclosures;
- risks;
- approval or policy decision references;
- next obligations.

Handoffs must not be unbounded natural-language summaries. Natural-language notes may be allowed as bounded, redacted annotations, but they should not be the source of truth for context transfer.

## 10. Scoped Authority Boundary

Harness authority must be explicit.

A future contract should distinguish:

- read-only context access;
- local deterministic checks;
- adapter reads;
- approval requests;
- report generation;
- side effects, when side-effect modeling exists;
- writes, only after write-capable adapter policy is separately accepted.

No harness should inherit ambient authority from the parent workflow. Authority should be delegated explicitly by contract and enforced by validation and runtime boundaries.

## 11. Failure Semantics

A future harness contract should define how failure behaves:

- fail the parent workflow;
- produce a blocked handoff;
- require approval;
- retry according to bounded policy;
- skip with explicit disclosure;
- produce partial output with known limitations;
- cancel downstream harnesses;
- escalate.

Failure semantics must remain deterministic and auditable. A model self-review should not be treated as a substitute for policy, validation, or human approval.

## 12. Relationship To Existing Concepts

Recommended relationships:

- Workflow OS: governed work runtime.
- Workflow: authored unit of governed work.
- Harness: bounded execution envelope within a workflow.
- Agent: reasoning or execution actor inside a harness.
- Tool: capability exposed under policy.
- Evidence: durable proof or citation pointer attached to a claim, validation, decision, or report.
- Handoff: typed transfer of artifacts, claims, risks, and next obligations.
- Work Report: final auditable summary.
- Reasoning Lineage: future provenance graph for claims and derivations, not required for the first harness contract model.

Harness contracts should build on EvidenceReference and WorkReport foundations without implementing reasoning lineage.

## 13. Why Not Implement Too Early

Composable Harness Contracts should not be implemented before the underlying primitives are stable because they add coordination overhead and security risk.

Premature implementation could:

- create false governance if reviews are just model opinions;
- create context drift if handoffs are natural-language summaries;
- create security risk if authority is ambient;
- create parallel write conflicts;
- blur the line between workflow runtime and generic agent orchestration;
- overfit core to software engineering examples;
- imply nested execution support that does not exist;
- create schema commitments before the model is reviewed.

## 14. Initial Practical Use Case

The first illustrative future pattern is AI-assisted software engineering.

A workflow might eventually decompose into:

- spec harness;
- planning harness;
- implementation harness;
- test/verification harness;
- review harness;
- security/risk harness;
- final work report harness.

This is illustrative only. It is not an immediate implementation promise and should not imply production nested execution support.

## 15. Implementation Status

Implemented:

- `HarnessContract`;
- `HarnessContractId`;
- `HarnessContractVersion`;
- harness input, context, tool, output, evidence, approval, and handoff requirement types;
- harness authority scope vocabulary;
- side-effect allowance vocabulary without enabling writes;
- failure semantics vocabulary;
- conservative execution policy model;
- validation for required fields, duplicate declarations, bounded text, redaction metadata, and secret-like values;
- serde validation and redaction-safe `Debug`;
- focused Rust tests.

Not implemented:

- nested harness execution;
- runtime scheduling;
- workflow schema fields;
- CLI behavior;
- examples;
- reasoning lineage;
- side-effect boundary modeling;
- writes;
- domain packs;
- hosted or distributed runtime behavior.

## 16. Proposed Implementation Sequence

Recommended future phases:

1. Review the implemented Composable Harness Contract core model. Completed.
2. Plan typed handoff model only after the contract model is reviewed. Completed in [Typed Handoff Plan](typed-handoff-plan.md).
3. Implement typed handoff core model only. Completed.
4. Plan nested harness execution patterns only after typed handoffs, scoped authority, and side-effect boundaries are understood.
5. Keep reasoning lineage separate until the harness and report boundaries are stable.

## 17. Future Test Plan

A future model implementation should test:

- valid minimal harness contract;
- invalid harness ID;
- invalid version;
- missing purpose;
- empty input requirements where inputs are required;
- required context validation;
- authority scope validation;
- side-effect allowance is representable without enabling writes;
- evidence requirement validation;
- approval requirement validation;
- failure semantics validation;
- handoff requirement validation;
- sensitivity and redaction policy validation;
- serde round trip;
- invalid serialized contract fails closed;
- debug output does not leak secret-like values;
- no runtime execution behavior is introduced;
- no schema, CLI, domain pack, or write support is introduced.

## 18. Open Questions

- What is the smallest useful harness contract for Workflow OS core?
- Should harness contracts live in workflow specs eventually, or remain separate model objects first?
- Should typed handoffs be modeled before or after harness contracts?
- How should harness contracts cite WorkReportContract requirements?
- Should a harness contract reference local check command contracts?
- How should approval requirements compose across parent workflow and child harness boundaries?
- What failure semantics are needed before nested execution is safe?
- How should authority inheritance be prevented?
- Should reasoning lineage cite harness handoffs later?
- What should remain in core versus future domain packs?

## 19. Final Recommendation

The next phase should be: typed handoff core model review.

It should not implement nested execution, runtime scheduling, workflow schema fields, CLI behavior, examples, reasoning lineage, side-effect boundaries, writes, domain packs, hosted/distributed runtime behavior, or release posture changes.
