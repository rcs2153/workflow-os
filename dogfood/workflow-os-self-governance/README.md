# Workflow OS Self-Governance Dogfood Project

This project is the first Workflow OS dogfooding slice for building Workflow OS itself.

It uses the local Workflow OS kernel as a governance wrapper for a planning/docs task. The kernel validates the dogfood specs, creates a durable local run, pauses for human approval, records policy and approval events, resumes the run, and leaves inspectable event history.

This is **kernel-governed, Codex-executed** dogfooding. Codex or a human still performs the actual repository edits and validation commands outside the kernel. The dogfood workflow does not execute build commands, mutate repository files, call external systems, run recursive agents, or replace human review.

## What It Demonstrates

- Workflow OS can govern Workflow OS planning/docs work through its own local kernel.
- The workflow is Level 2 and approval-gated.
- The run is local, deterministic, auditable, and inspectable.
- The first self-governance slice uses the current v0 single-step executor honestly.

## What It Does Not Do

- It does not execute arbitrary shell commands.
- It does not run Rust or npm checks from inside the kernel.
- It does not perform code or documentation edits.
- It does not use live adapters or external services.
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

The run should pause at `WaitingForApproval` and print a `run_id` plus `approval_id`.

Approve and resume:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/dogfood-reviewer \
  --reason reviewed-governance-task
```

Inspect the completed governance run:

```sh
target/debug/workflow-os \
  --project-dir dogfood/workflow-os-self-governance \
  --state-dir /tmp/workflow-os-self-governance-state \
  inspect <run-id>
```

## Operating Boundary

Use this project to prove governance around Workflow OS work, not automation ownership of Workflow OS work. Until real local build/check skill handlers are separately designed, the kernel records the governed workflow and Codex or humans still perform implementation and validation.
