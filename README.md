# Workflow OS

Workflow OS is a governance kernel for work performed by AI agents.

```text
Agent executes. Workflow OS governs.
```

Agents are increasingly capable of writing code, calling tools, and changing
external systems. The missing layer is not another agent framework. It is a
durable answer to:

- What was this agent authorized to do?
- Which workflow, policy, and inputs governed the action?
- What evidence and checks supported the decision?
- Who or what approved it?
- What side effects actually occurred?
- What may happen next?

Workflow OS puts those facts in a deterministic kernel instead of leaving them
inside prompts, chat history, or agent memory.

The project is an Apache-2.0 preview. It is useful today for local governed
workflows and engineering evaluation. It is not yet a production enterprise
control plane.

## How It Works

```text
workflow + policy + authority + evidence obligations
                         |
                         v
                  Workflow OS kernel
                         |
       validate -> authorize -> record -> reconcile
                         |
                         v
             agent / handler / adapter / sandbox
                         |
                         v
              events + evidence + work report
```

For each material action, Workflow OS:

1. validates the declared workflow and policy;
2. binds the run to immutable definitions and inputs;
3. evaluates current authority, checks, evidence, and approval requirements;
4. allows, discloses, blocks for approval, or denies the action;
5. records ordered durable events and side-effect outcomes;
6. produces an inspectable report and derives the lawful next action.

Workflow OS does not model every internal reasoning step. The agent remains
free to explore and execute inside the authorized envelope. The kernel governs
the boundaries where work consumes authority, changes durable state, invokes a
tool, creates a side effect, or claims completion.

## Try It In A Repository

Build the CLI:

```sh
cargo build -p workflow-cli --bin workflow-os
```

From a repository you want to govern:

```sh
/path/to/workflow-os init-repo-governance --agent codex --dry-run
/path/to/workflow-os init-repo-governance --agent codex
/path/to/workflow-os validate
/path/to/workflow-os first-run
```

This creates a local governance envelope and reports what Workflow OS can
establish safely from repository metadata. It does not read arbitrary source
contents, execute project commands, call providers, or invent evidence.

`first-run` identifies missing ownership, checks, evidence, approval, reporting,
and side-effect decisions. It also recommends candidate workflows. A candidate
does not become active until it is authored, reviewed, and promoted.

To exercise the approval and event-log path with a deterministic mock handler:

```sh
/path/to/workflow-os --mock-all-local-skills run local/first-run-governance
/path/to/workflow-os --mock-all-local-skills approve <run-id> <approval-id> \
  --actor user/evaluator --reason reviewed-first-run
/path/to/workflow-os inspect <run-id>
```

The mock proves kernel sequencing, approval, persistence, and audit behavior.
It does not prove that repository work was executed.

## What Is Real Today

The current kernel includes:

- deterministic YAML loading and validation;
- sequential multi-step local workflows;
- immutable run inputs and approval-resume integrity;
- policy decisions and proof-bound approvals;
- durable event history, retries, escalation, cancellation, and rehydration;
- evidence references, side-effect records, and terminal work reports;
- proportional governance: quiet capture, visible disclosure, approval, or
  denial;
- scoped capability, current-authority, required-context, and check models;
- local filesystem, SQLite continuity, and PostgreSQL state implementations;
- fixture-first read-only GitHub, Jira, and GitHub Actions adapters;
- narrow, explicit GitHub write proofs for comments and managed draft pull
  requests;
- a single-tenant hosted no-write alpha and optional OpenShell boundary.

Not every Core capability is exposed through the CLI. Some are deliberately
kept as explicit, reviewed integration paths while their security contracts
harden.

## What It Does Not Claim

Workflow OS is not:

- an agent runtime, prompt router, or agent swarm;
- a replacement for a coding agent, CI system, Temporal, or Airflow;
- a general shell-execution or provider-write framework;
- a production multi-tenant service;
- enterprise RBAC, identity, or policy administration;
- proof that mocked or caller-asserted work actually happened.

Missing handlers and unsupported authority fail closed. A model or schema in
the repository is not a runtime capability unless an explicit execution path
enforces it.

## Use It With An Agent

Point the agent at the repository and give it one operating instruction:

```text
Use Workflow OS as the governing layer for this task.
Treat kernel state, not conversation history, as authoritative.
Execute only the work currently authorized by the kernel.
Do not bypass validation, policy, approval, evidence, check, or scope failures.
At closure, report checks, evidence, side effects, incomplete work, and the
kernel-authorized next action.
```

The agent still performs edits and ordinary tool calls unless a reviewed
Workflow OS handler owns that execution. Workflow OS governs the work; it does
not gain control merely because an instruction file says so.

## Read Next

- [Current Product Contract](docs/user-guide/current-product-contract.md): what
  is implemented, experimental, and unavailable.
- [Workflow OS Explainer](docs/concepts/WORKFLOW_OS_EXPLAINER.md): the engineering
  architecture and execution model.
- [Agent Harness Quickstart](docs/user-guide/agent-harness-quickstart.md): using
  Workflow OS with a coding agent.
- [Engineering Standard](docs/ENGINEERING_STANDARD.md): invariants for changing
  the kernel.
- [Roadmap](ROADMAP.md): accepted sequence and unfinished work.

## Repository

```text
crates/workflow-core/    canonical Rust kernel
crates/workflow-cli/     local CLI and onboarding
crates/workflow-hosted/  single-tenant hosted alpha
packages/sdk-typescript/ spec-authoring helpers
examples/                portable evaluation projects
dogfood/                 this repository's governed workflows
docs/                    contracts, architecture, plans, and reports
```

The `dogfood/` workflows are specific to building Workflow OS. They are
reference implementations, not defaults installed into downstream projects.

## Develop

Use Node.js 20 and a Rust toolchain with `rustfmt` and `clippy`:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm ci
npm run check
npm run check:integrations
```

## License

Apache License 2.0. See [LICENSE](LICENSE).
