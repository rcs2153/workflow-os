# Agent Harness Quickstart

This guide explains the intended first-run loop for using Workflow OS with a coding agent such as Codex or Claude Code.

The important shift is:

```text
Agent executes. Workflow OS governs.
```

Workflow OS is not just YAML to hand-author and test. The YAML specs are the governed contract that an agent should operate inside. The kernel provides validation, durable run state, policy gates, approval checkpoints, auditability, and report posture while the agent performs repository work under those boundaries.

You do not need the Workflow OS repository's `dg/*` dogfood workflows in your own project. They are internal benchmark workflows for building Workflow OS itself. New and existing repos should start from the scaffold and first-run path below.

The agent keeps its speed and flexibility. Workflow OS makes the work inspectable. The kernel should not force the agent to model every internal reasoning edge; it should present the steps, gates, stops, approvals, evidence requirements, side-effect disclosures, validation/check obligations, handoffs, and reports that make autonomous work reviewable.

This guide does not add runtime automation. It does not implement recursive agents, agent swarms, hosted execution, writes, automatic local checks, CLI report rendering, workflow schema changes, or Level 3/4 autonomy.

For normal repository adoption, start with `workflow-os init-repo-governance`
and `workflow-os first-run` in the user's own repo. Workflow OS's `dg/*`
dogfood workflows remain useful for learning how this repository governs its
own build, but they are not downstream defaults and should not be copied into a
user project as plug-and-play assets. The first existing-repo governance
scaffold is implemented as documented in [Existing Repo Governance Onboarding
Plan](../implementation-plans/existing-repo-governance-onboarding-plan.md).
First-run Governed Work Pattern reporting posture is implemented as
`workflow-os first-run`, which emits a bounded report-ready context without
running workflows or writing artifacts.

## When To Use This

Use this flow when you want an AI coding agent to help with repository work while Workflow OS governs the task.

Good fits:

- planning a bounded implementation phase;
- editing documentation under an approved scope;
- implementing a narrow kernel feature after a plan is accepted;
- running validation commands and reporting results;
- gathering evidence and report context without slowing every agent action;
- preserving explicit approval checkpoints and final work reports.

Do not use this flow to bypass policy, approvals, validation, human review, or the repository engineering standard.

## One-Minute Setup

From the repository root, build the local CLI:

```sh
cargo build -p workflow-cli --bin workflow-os
```

For a normal existing repository that is not already a Workflow OS project, initialize a minimal local governance envelope first:

```sh
target/debug/workflow-os init-repo-governance
target/debug/workflow-os validate
target/debug/workflow-os first-run
target/debug/workflow-os --mock-all-local-skills run local/first-run-governance
```

The `first-run` command produces the immediate ledger/report posture: missing evidence, skipped checks, unsupported side effects, bounded risks, and review-only workflow recommendations. The explicit `run` command remains separate and pauses for approval. The generated skill is a mockable placeholder until a real local handler is implemented, registered, and reviewed.

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

For PR preparation or conflict-avoidance work, use the PR hygiene workflow instead:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-pr-hygiene-state \
  --mock-all-local-skills \
  run dg/pr
```

That workflow governs main-sync disclosure, hot-file risk scoping, validation disclosure, conflict-resolution disclosure, and PR readiness reporting. It does not run git, inspect GitHub, resolve conflicts, push branches, or open PRs.

For accepted implementation phases, use:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-implementation-state \
  --mock-all-local-skills \
  run dg/implement
```

For maintainer review phases, use:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-review-state \
  --mock-all-local-skills \
  run dg/review
```

For runtime-composition phases, use:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-runtime-composition-state \
  --mock-all-local-skills \
  run dg/runtime-composition
```

For focused blocker fixes, use:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-blocker-fix-state \
  --mock-all-local-skills \
  run dg/blocker
```

For release hygiene phases, use:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-release-hygiene-state \
  --mock-all-local-skills \
  run dg/release
```

For merged-branch cleanup readiness, use:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-branch-cleanup-state \
  --mock-all-local-skills \
  run dg/branch-cleanup
```

That workflow governs branch inventory, deletion-candidate review, explicit cleanup approval, post-cleanup validation, and cleanup reporting. It does not run git, delete branches, inspect GitHub, force-push, or bypass branch protection.

For recommendation-only workflow discovery, use:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-workflow-discovery-state \
  --mock-all-local-skills \
  run dg/workflow-discovery
```

That workflow governs discovery of repeated work patterns, missing gates, overlap/conflict risk, workflow split/merge/retirement candidates, stewardship approval, and recommendation handoff. It does not generate workflow files, register workflows, mutate specs, modify roadmap state, inspect GitHub, or approve itself.

These workflows govern the lifecycle and approval/checkpoint posture. They do not execute repository edits, validation commands, GitHub operations, or PR actions on behalf of the agent.

They are also not community defaults. Use them to understand the pattern; do not copy them blindly into a downstream repository.

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
- keep useful automation moving inside the approved boundaries;
- keep the work phase-bounded;
- ask for approval when the governed workflow requires it;
- gather and cite evidence through implemented Workflow OS surfaces where available;
- run requested local checks outside the kernel unless a real handler is explicitly registered and reviewed;
- produce a structured implementation or review report;
- distinguish implemented behavior from planned or deferred behavior.

## What The Agent Must Not Do

The agent must not:

- treat YAML authoring as the entire Workflow OS experience;
- treat Workflow OS as a brittle graph that must model every internal agent edge;
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

The command generates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md`. Existing Workflow OS managed blocks are updated in place. If `AGENTS.md` already contains repository-specific guidance, Workflow OS preserves that content and appends or updates only the managed Workflow OS block. Other unmanaged scaffold targets still fail closed unless `--force` is supplied.

This helper is explicit and safe: no silent command execution, no workflow runs, no approvals, no local check execution, no handler registration, no writes, no hosted behavior, no schema change, and no default Level 3/4 autonomy.

The command is documented in [Agent Harness CLI Scaffold Plan](../implementation-plans/agent-harness-cli-scaffold-plan.md) and [CLI init-agent-harness](../cli/init-agent-harness.md). Dogfood and adoption review planning is documented in [Agent Harness Scaffold Dogfood And Adoption Plan](../implementation-plans/agent-harness-scaffold-dogfood-adoption-plan.md). The root `AGENTS.md` and this quickstart remain the canonical human-readable setup path.

This helper is not the existing-repo governance scaffold. It does not create `workflow-os.yml`, workflows, policies, skills, or a runnable Workflow OS project for a normal repository. Use `workflow-os init-repo-governance` for the current minimal in-repo project scaffold.

## Future Hook Layer

The scaffold is a human/agent orientation layer: useful for declaring conventions, expectations, and structure, but not itself an enforcement layer.

The next maturity layer is governed hooks: deterministic, named checkpoints that the harness invokes before or after important phases of work. Hook integration is planned in [Agent Harness Hook Integration Plan](../implementation-plans/agent-harness-hook-integration-plan.md), the first hook contract model is implemented as vocabulary and validation only, and the in-memory invocation helper model is implemented for explicit context validation. Hook audit/event semantics planning is documented in [Agent Harness Hook Audit/Event Semantics Plan](../implementation-plans/agent-harness-hook-audit-event-semantics-plan.md), and the hook audit record core model is implemented as model-only vocabulary and validation. Runtime hook execution is not implemented yet.

Future hooks should make governance less dependent on the agent remembering prose instructions. They should not silently enable command execution, workflow runs, approvals, local checks, writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.
