# Vertical Slice Approval Example

This is the first complete local Workflow OS vertical slice. It demonstrates the v0 kernel with a generic enterprise workflow:

> Review a structured internal business request and require approval before producing a deterministic recommendation output.

The example uses no external services, no secrets, and no real adapters.

## What Is Real

- Project loading from `workflow-os.yml`.
- Rust semantic validation.
- CLI-driven local execution.
- Event-sourced run creation and rehydration.
- Durable local filesystem state.
- Approval pause and resume.
- Conservative policy checks before approval/resume/skill execution.
- Audit event emission through the runtime audit sink.
- Observability event emission through the runtime observability sink.
- CLI `validate`, `run`, `status`, `approve`, and `inspect`.

## What Is Mocked

The skill `local/rec` is a deterministic local mock skill handled only when the CLI is run with `--mock-all-local-skills` or when tests explicitly register an `ExampleHandler` in `LocalSkillRegistry`. It does not call an AI model, external API, GitHub, Jira, CI, or any enterprise system.

The mock output proves the kernel path. It is not a production business recommendation engine.

## Run It

From the repository root:

```sh
cargo build -p workflow-cli --bin workflow-os
```

Validate the project:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  validate
```

Start the workflow:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --mock-all-local-skills \
  run ex/review
```

The run pauses before skill execution:

```text
run_id: run-...
status: WaitingForApproval
approval_id: approval/run-.../rec
```

Check status:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  status <run-id>
```

Approve and resume:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/example-approver \
  --reason reviewed-local-example
```

Inspect the completed run:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  inspect <run-id>
```

The event history includes `RunCreated`, `PolicyDecisionRecorded`, `ApprovalRequested`, `ApprovalGranted`, `RunResumed`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, and `RunCompleted`.

## Local State

By default, state is written under:

```text
examples/vertical-slice-approval/.workflow-os/state
```

Use `--state-dir <path>` to keep example runs outside the working tree:

```sh
target/debug/workflow-os \
  --project-dir examples/vertical-slice-approval \
  --state-dir /tmp/workflow-os-vertical-slice-state \
  run ex/review
```

## Product Boundary

This example is intentionally generic. It is not a ticketing workflow, pull request workflow, CI workflow, or SaaS integration. Future adapters can reuse the same kernel guarantees after adapter contracts exist.
