# Agent Harness Onboarding Plan

Status: Implemented as documentation and repository-agent-instruction onboarding. Agent harness CLI scaffold planning is documented in [Agent Harness CLI Scaffold Plan](agent-harness-cli-scaffold-plan.md), and future dbt-style hook integration planning is documented in [Agent Harness Hook Integration Plan](agent-harness-hook-integration-plan.md). The first hook contract model is implemented as vocabulary and validation only. Runtime harness auto-generation, runtime hook invocation, workflow schema fields, automatic local check execution, hosted orchestration, writes, reasoning lineage, side-effect modeling, and release posture changes are not implemented.

## 1. Executive Summary

User feedback shows a strong pattern: evaluators often pull down Workflow OS and start by manually writing YAML and testing the kernel. The lightbulb moment comes when they connect a coding agent such as Codex or Claude Code to the local kernel and instruct it to use Workflow OS as the governing layer.

This phase makes that intended operating loop explicit and easy to adopt. It adds a canonical agent-harness quickstart, root-level agent instructions, README guidance, user-guide links, dogfood guidance, and roadmap positioning.

The current scaffold is an orientation layer, not an enforcement layer. The future maturity path is dbt-style named hooks that an agent harness invokes at governed checkpoints. The hook contract model exists, but runtime hook invocation is planned separately and remains unimplemented.

The intended default mental model is:

```text
Agent executes. Workflow OS governs.
```

This plan does not implement runtime automation, CLI scaffold commands, automatic report generation, automatic local check execution, persistence, schema changes, examples, side-effect modeling, writes, recursive agents, agent swarms, or release posture changes.

## 2. Goals

- Make kernel-governed agent use the obvious onboarding path.
- Give users a copy/paste prompt for Codex, Claude Code, or a similar coding agent.
- Preserve Workflow OS's product boundary as a governed work runtime, not a generic multi-agent framework.
- Explain that YAML specs are the contract the agent operates inside, not the whole user experience.
- Make the dogfood pattern discoverable from README and user guide docs.
- Document a future `init-agent-harness` style setup path without implementing it.
- Keep all automation phase-bounded, approval-aware, and validation-first.

## 3. Non-Goals

This phase does not implement:

- runtime automatic report generation;
- runtime harness generation;
- CLI `init-agent-harness`;
- CLI rendering or export;
- workflow schema fields;
- example integration changes;
- automatic local check execution;
- default local command handler registration;
- command-output evidence;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary modeling;
- write behavior;
- hosted or distributed runtime behavior;
- recursive agents;
- agent swarms;
- self-governing agents;
- Level 3 or Level 4 autonomy claims;
- release posture changes.

## 4. Product Framing

Workflow OS should teach users that the agent is the worker and the kernel is the governing layer.

Preferred language:

- agent harness;
- kernel-governed agent;
- governed execution envelope;
- phase-bounded automation;
- approval checkpoint;
- validation-first workflow;
- typed handoff;
- final work report.

Avoid:

- recursive agents;
- agent swarms;
- agents managing agents;
- self-governing AI;
- fully autonomous software factory;
- magic orchestration.

## 5. Onboarding Surface

Implemented docs:

- `AGENTS.md`: root repository instructions for coding agents.
- `docs/user-guide/agent-harness-quickstart.md`: canonical quickstart and copy/paste prompt.
- `README.md`: first-run pointer to kernel-governed agent usage.
- `docs/user-guide/README.md`: guide index entry.
- `dogfood/workflow-os-self-governance/README.md`: dogfood-specific usage framing.
- `ROADMAP.md`: P0 roadmap placement.

## 6. Canonical Agent Prompt

The quickstart defines a copy/paste prompt that tells an agent to:

- use Workflow OS as the governing layer;
- validate the project before work;
- start or resume a governed workflow;
- treat approval checkpoints as mandatory;
- stay inside the requested phase scope;
- avoid inventing workflow state, approvals, evidence, reports, or validation results;
- run checks requested by the phase;
- produce a structured report of completed work, deferred work, and validation.

## 7. Current Manual Setup Path

Current setup remains explicit:

1. Build the local CLI.
2. Validate the Workflow OS project or dogfood project.
3. Start or resume the governed workflow.
4. Paste the agent harness prompt into Codex or Claude Code.
5. Approve checkpoints when the kernel pauses.
6. Let the agent perform repository edits outside the kernel while the kernel governs phase boundaries.
7. Inspect the run and review the final implementation report.

This is intentionally not automatic runtime generation.

## 8. Future Magic Setup Path

Future planning may introduce a CLI command such as:

```sh
workflow-os init-agent-harness
```

Such a command should generate or update:

- `AGENTS.md`;
- a copy/paste session prompt;
- a starter governed workflow scaffold;
- safe phase-boundary instructions;
- validation/check instructions;
- report expectations.

That future command must not silently enable command execution, writes, hosted behavior, side-effecting adapters, or Level 3/4 autonomy.

## 9. Safety Boundary

The agent harness instructions must preserve these rules:

- Workflow OS governs; the coding agent executes.
- The agent must not bypass validation failures, policy denials, missing approvals, or failed checks.
- The agent must not invent workflow state, approvals, evidence references, audit events, or work reports.
- The agent must not treat natural-language self-review as a replacement for deterministic validation.
- The agent must not claim automatic kernel execution of build commands unless explicitly registered and reviewed.
- The agent must not widen phase scope without explicit user approval.

## 10. Documentation Updates

Docs must state:

- the kernel-governed agent harness onboarding path is documented;
- root `AGENTS.md` instructions are available for coding agents;
- manual YAML remains supported but is not the intended whole experience;
- runtime harness auto-generation is not implemented;
- CLI `init-agent-harness` is not implemented;
- automatic local check execution is not implemented;
- writes, side-effect modeling, recursive agents, agent swarms, hosted runtime, and Level 3/4 autonomy remain unsupported.

## 11. Test/Validation Plan

Validation for this docs/onboarding phase:

- `npm run check:docs`

If code changes are introduced in a future implementation, also run:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

## 12. Recommended Next Phase

Recommended next phase: **agent harness onboarding review**.

Agent harness CLI scaffold planning is documented in [Agent Harness CLI Scaffold Plan](agent-harness-cli-scaffold-plan.md). The planned command remains unimplemented and must stay documentation/scaffold-only without automatic command execution, writes, schemas, hosted behavior, or release posture changes.
