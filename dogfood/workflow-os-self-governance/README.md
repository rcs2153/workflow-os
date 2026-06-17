# Workflow OS Self-Governance Dogfood Project

This project is the first Workflow OS dogfooding slice for building Workflow OS itself.

It uses the local Workflow OS kernel as a sequential multi-step governance wrapper for a planning/docs task. The kernel validates the dogfood specs, creates a durable local run, records a scope checkpoint, pauses for human approval, records policy and approval events, resumes the run, completes downstream checkpoints, and leaves inspectable event history.

This is **kernel-governed, Codex-executed** dogfooding. Codex or a human still performs the actual repository edits outside the kernel. The dogfood workflow can exercise the docs check only when a caller explicitly supplies and registers `DocsCheckLocalHandler`; it is not default, CLI-enabled, schema-driven, or automatic. The dogfood workflow does not execute arbitrary build commands, mutate repository files, call external systems, run recursive agents, or replace human review.

Use this project as the reference pattern for kernel-governed agent work:

```text
Agent executes. Workflow OS governs.
```

The agent should use the kernel to validate, start or resume the governed workflow, respect approval checkpoints, execute only the scoped repository work, run required validation commands outside the kernel unless an explicit handler exists, and report completed/deferred work honestly.

## What It Demonstrates

- Workflow OS can govern Workflow OS planning/docs work through its own local kernel.
- The workflow is Level 2 and approval-gated.
- The workflow uses sequential local multi-step execution.
- The run is local, deterministic, auditable, and inspectable.
- The dogfood checkpoints separate scope, planning approval, implementation handoff, validation disclosure, explicit docs-check execution, and review/report posture.

The conversion is documented in [Self-Governance Dogfood Multi-Step Conversion Plan](../../docs/implementation-plans/self-governance-dogfood-multi-step-conversion-plan.md).

## What It Does Not Do

- It does not execute arbitrary shell commands.
- It does not run Rust or npm checks from inside the kernel unless a reviewed explicit local check handler is supplied by the caller.
- It does not perform code or documentation edits.
- It does not use live adapters or external services.
- It does not execute branching, parallel, or nested harness behavior.
- It does not implement recursive agents, agent swarms, or generic orchestration.
- It does not claim production self-hosting.

## Run It

From the repository root:

```sh
cargo build -p workflow-cli --bin workflow-os
```

Validate the dogfood project:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  validate
```

Start the governance workflow using an external state directory:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  run dg/d
```

The run should execute the `scope-requested` checkpoint, pause at `planning-approved`, and print a `run_id` plus `approval_id`.

Approve and resume the downstream placeholder checkpoints:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-governance-task
```

Inspect the completed multi-step governance run:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  inspect <run-id>
```

## Use It With Codex Or Claude Code

After starting the governed run, paste the setup prompt from [Agent Harness Quickstart](../../docs/user-guide/agent-harness-quickstart.md) into Codex or Claude Code.

The agent should:

- read the required engineering and roadmap context;
- treat the dogfood run as the governing wrapper for the task;
- ask for approval when the kernel pauses;
- stay inside the approved phase scope;
- run validation commands requested by the phase outside the kernel unless a real local handler is explicitly registered;
- avoid inventing run state, approvals, evidence, audit events, or reports;
- produce the structured implementation or review report expected by the phase.

This dogfood path is not automatic build execution, recursive agent orchestration, agent swarming, production self-hosting, or Level 3/4 autonomy.

## Operating Boundary

Use this project to prove governance around Workflow OS work, not automation ownership of Workflow OS work. The docs-check checkpoint is explicit-handler-only: ordinary registry construction still fails closed, CLI mock runs still use deterministic mocks, and broader validation/build commands remain outside the kernel until separately scoped.
