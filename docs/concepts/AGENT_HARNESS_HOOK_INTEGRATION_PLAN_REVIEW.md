# Agent Harness Hook Integration Plan Review

Review date: 2026-06-16

## 1. Executive Verdict

Plan accepted; proceed to agent harness hook contract model implementation, model-only.

The plan correctly responds to user feedback that relying only on `AGENTS.md` is brittle. It preserves the current scaffold as a useful orientation layer while defining a future dbt-style hook layer for deterministic, named governed checkpoints. The plan stays planning-only and does not authorize runtime hook execution, workflow schema fields, CLI hook commands, automatic local checks, persistence, report artifacts, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not implement or authorize:

- runtime hook execution;
- automatic workflow execution;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- workflow schema fields;
- CLI hook commands;
- runtime harness generation;
- nested harness execution;
- recursive agents;
- agent swarms;
- hosted or distributed agent execution;
- side-effect modeling;
- writes;
- approval evidence attachment;
- reasoning lineage;
- persistence changes;
- report artifact auto-writing;
- examples;
- release posture changes.

## 3. Product Framing Assessment

The plan uses the right framing.

It treats the current scaffold as analogous to `dbt_project.yml`: a project orientation and convention layer that helps humans and agents understand how to work, but not an enforcement layer. It then frames hooks as the next maturity layer: deterministic checkpoints invoked by the harness or agent before or after important work phases.

This is the correct product direction because it strengthens adoption without changing Workflow OS into a generic agent orchestration platform.

## 4. Scaffold Versus Hooks Assessment

The plan clearly separates:

- scaffold files as documentation and prompt setup;
- hook contracts as future explicit checkpoint declarations;
- hook runtime invocation as deferred.

This distinction is important. It acknowledges the user concern without undermining the current scaffold work. The scaffold remains useful for onboarding; hooks are the future integration boundary where governance becomes less dependent on prose-only instruction following.

## 5. Candidate Hook Assessment

The candidate hook names are reasonable for planning:

- `before_plan`;
- `after_plan`;
- `before_implementation`;
- `after_implementation`;
- `before_validation`;
- `after_validation`;
- `before_review`;
- `after_review`;
- `before_report`;
- `after_report`.

The plan correctly marks them as illustrative and recommends selecting a very small initial set later. That avoids turning the first implementation into a broad lifecycle framework.

## 6. Contract Shape Assessment

The planned hook contract shape is appropriately bounded.

It covers:

- hook name;
- purpose;
- invocation timing;
- required and optional inputs;
- allowed outputs;
- policy checks;
- approval requirements;
- evidence requirements;
- local check requirements;
- failure semantics;
- redaction policy;
- sensitivity;
- report or handoff obligations.

This gives enough structure for a model-only implementation while still leaving runtime invocation, schema exposure, and CLI behavior deferred.

## 7. Authority Boundary Assessment

The authority boundary is strong.

The plan explicitly states that hooks must not grant ambient authority to agents and must not silently authorize workflow state mutation, approval decisions, local command execution, external provider calls, report artifact writes, side effects, or Level 3/4 autonomy.

It also correctly defers any side-effecting hook behavior until side-effect boundary modeling, high-assurance approval controls where needed, and write-capable adapter policy are separately accepted.

## 8. Relationship To Existing Primitives

The plan builds on existing Workflow OS primitives instead of inventing a parallel governance lane.

It correctly ties hooks to:

- workflow and run identity;
- durable runtime state;
- policy gates;
- event-sourced approvals;
- local check result references;
- EvidenceReference;
- typed handoffs;
- WorkReports;
- audit records.

This relationship keeps the hook concept aligned with Workflow OS as a governed work runtime, not a generic agent framework.

## 9. Error Handling And Privacy Assessment

The error-handling and privacy guidance is appropriate.

The plan requires stable error codes, fail-closed behavior for missing governance context, and non-leaking errors. It forbids raw prompts, command output, provider payloads, environment values, paths, tokens, credentials, raw specs, parser payloads, raw CI logs, Jira/GitHub bodies, and secret-like values.

This is enough to drive a model-only implementation prompt.

## 10. Test Plan Assessment

The future model-only test plan is focused and sufficient.

It covers:

- valid minimal hook contract;
- invalid hook ID/name rejection;
- required input and output validation;
- duplicate requirement rejection;
- redaction policy validation;
- sensitivity validation;
- failure semantics validation;
- side-effect authorization rejection;
- serde round trip;
- invalid serde failure;
- redaction-safe `Debug`;
- serialization non-leakage;
- regression checks that runtime execution, CLI behavior, schemas, persistence, artifacts, writes, and hosted behavior are not introduced.

The plan correctly defers runtime hook tests until after a model-only contract has been reviewed.

## 11. Documentation Review

Documentation updates are honest and aligned.

The roadmap, quickstart, onboarding plan, and Governed Work Pattern now say:

- the scaffold is an orientation layer, not an enforcement layer;
- agent harness hooks are planned, not implemented;
- hook runtime execution is not implemented;
- CLI hook commands are not implemented;
- workflow schema fields are not implemented;
- automatic local checks are not implemented;
- writes, hosted behavior, recursive agents, and agent swarms remain non-goals.

No current capability is overclaimed.

## 12. Planning Blockers

No planning blockers.

## 13. Non-Blocking Follow-Ups

- Decide whether the first hook contract should be workflow-level, harness-level, step-level, or phase-level.
- Decide whether the first model should use generic `before_*`/`after_*` vocabulary or a smaller domain-neutral checkpoint vocabulary.
- Decide whether hook failure semantics should model warning-only outcomes or fail-closed outcomes first.
- Keep hook runtime invocation separate from the first hook contract model implementation.
- Keep hook schema exposure deferred until model vocabulary is reviewed.

## 14. Recommended Next Phase

Recommended next phase: **agent harness hook contract model implementation, model-only**.

That phase should add the smallest Rust model needed to represent and validate hook contracts. It should not implement runtime hook execution, CLI hook commands, workflow schema fields, automatic local checks, persistence, report artifacts, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 15. Validation

Validation commands for this review:

- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.
