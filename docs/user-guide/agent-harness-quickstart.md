# Agent Harness Quickstart

This guide explains the intended first-run loop for using Workflow OS with a coding agent such as Codex or Claude Code.

The important shift is:

```text
Agent executes. Workflow OS governs.
```

Workflow OS is not just YAML to hand-author and test. The YAML specs are the governed contract that an agent should operate inside. The kernel provides validation, durable run state, policy gates, approval checkpoints, auditability, and report posture while the agent performs repository work under those boundaries.

This guide does not add runtime automation. It does not implement recursive agents, agent swarms, hosted execution, writes, automatic local checks, CLI report rendering, workflow schema changes, or Level 3/4 autonomy.

## When To Use This

Use this flow when you want an AI coding agent to help with repository work while Workflow OS governs the task.

Good fits:

- planning a bounded implementation phase;
- editing documentation under an approved scope;
- implementing a narrow kernel feature after a plan is accepted;
- running validation commands and reporting results;
- preserving explicit approval checkpoints and final work reports.

Do not use this flow to bypass policy, approvals, validation, human review, or the repository engineering standard.

## One-Minute Setup

From the repository root, build the local CLI:

```sh
cargo build -p workflow-cli --bin workflow-os
```

Validate the self-governance dogfood project:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  validate
```

Start a governed dogfood run using a temporary state directory:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  run dg/d
```

The run should pause for approval. Copy the `run_id` and `approval_id`.

## Copy/Paste Agent Prompt

Paste this into Codex, Claude Code, or another coding agent in the repository:

```text
You are working in this repository with Workflow OS installed locally.

Use Workflow OS as the governing layer for this task.

Before making changes:
1. Read the repository engineering standard and relevant roadmap/planning docs.
2. Validate the Workflow OS project or the relevant Workflow OS dogfood project.
3. Start or resume the appropriate governed workflow when the task requires it.
4. Treat workflow approvals as mandatory checkpoints.
5. Do not bypass failed validation, denied policy, missing approvals, failed checks, or explicit scope limits.

While working:
1. Stay inside the approved phase scope.
2. Do not invent workflow state, approvals, evidence references, audit events, work reports, validation results, or command outputs.
3. Do not claim runtime automation, write support, hosted execution, recursive agents, or agent swarms unless those are explicitly implemented and reviewed.
4. Use existing Workflow OS commands, tests, and docs as the source of truth.
5. Preserve deterministic validation, durable state, auditability, and human approval boundaries.

Before finishing:
1. Run the validation commands required by the phase.
2. Report completed scope, explicitly deferred scope, validation results, and the recommended next phase.
3. If a governed run was used, include the run status and approval/checkpoint context.
```

## Approve The Kernel Checkpoint

When the kernel pauses, approve with a bounded reason:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-governance-task
```

Inspect the completed run:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  inspect <run-id>
```

## What The Agent Should Do

The agent should:

- use Workflow OS as the governance wrapper;
- keep the work phase-bounded;
- ask for approval when the governed workflow requires it;
- run requested local checks outside the kernel unless a real handler is explicitly registered and reviewed;
- produce a structured implementation or review report;
- distinguish implemented behavior from planned or deferred behavior.

## What The Agent Must Not Do

The agent must not:

- treat YAML authoring as the entire Workflow OS experience;
- bypass Workflow OS validation, policy, approvals, or failed checks;
- mutate workflow state manually;
- fabricate run IDs, approval IDs, evidence references, audit events, or reports;
- claim automatic local check execution when only explicit/manual checks are available;
- claim production self-hosting;
- frame Workflow OS as recursive agents, agent swarms, or arbitrary multi-agent orchestration;
- claim write-capable adapters, hosted runtime, production backend, or Level 3/4 autonomy.

## Setup Helper

Use the scaffold helper to create or update local agent instruction files:

```sh
workflow-os init-agent-harness
```

The command generates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md`. Existing Workflow OS managed blocks are updated in place; unmanaged files fail closed unless `--force` is supplied.

This helper is explicit and safe: no silent command execution, no workflow runs, no approvals, no local check execution, no handler registration, no writes, no hosted behavior, no schema change, and no default Level 3/4 autonomy.

The command is documented in [Agent Harness CLI Scaffold Plan](../implementation-plans/agent-harness-cli-scaffold-plan.md) and [CLI init-agent-harness](../cli/init-agent-harness.md). Dogfood and adoption review planning is documented in [Agent Harness Scaffold Dogfood And Adoption Plan](../implementation-plans/agent-harness-scaffold-dogfood-adoption-plan.md). The root `AGENTS.md` and this quickstart remain the canonical human-readable setup path.

## Future Hook Layer

The scaffold is the `dbt_project.yml` equivalent for human/agent orientation: useful for declaring conventions, expectations, and structure, but not itself an enforcement layer.

The next maturity layer is dbt-style hooks: deterministic, named checkpoints that the harness invokes before or after important phases of work. Hook integration is planned in [Agent Harness Hook Integration Plan](../implementation-plans/agent-harness-hook-integration-plan.md), the first hook contract model is implemented as vocabulary and validation only, and the in-memory invocation helper model is implemented for explicit context validation. Hook audit/event semantics planning is documented in [Agent Harness Hook Audit/Event Semantics Plan](../implementation-plans/agent-harness-hook-audit-event-semantics-plan.md), and the hook audit record core model is implemented as model-only vocabulary and validation. Runtime hook execution is not implemented yet.

Future hooks should make governance less dependent on the agent remembering prose instructions. They should not silently enable command execution, workflow runs, approvals, local checks, writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.
