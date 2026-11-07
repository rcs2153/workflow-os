# Workflow OS

[![CI](https://github.com/rcs2153/workflow-os/actions/workflows/ci.yml/badge.svg)](https://github.com/rcs2153/workflow-os/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/rcs2153/workflow-os?include_prereleases)](https://github.com/rcs2153/workflow-os/releases)
[![License](https://img.shields.io/github/license/rcs2153/workflow-os)](LICENSE)

Workflow OS is a local-first continuity-of-authority kernel for agentic work.
It converts ephemeral conversation and execution context into a durable,
policy-governed run whose intent, authority, execution state, evidence, and
next permissible action can be recovered by another person, process, or agent.
It does this without replacing the agent runtime.

```text
Agent executes. Workflow OS governs.
```

Agents can already write code, call tools, and change external systems. Most
systems preserve context, traces, or task state. Workflow OS is designed to
preserve whether the work is still legitimate to continue. It maintains four
connected truths:

- **Intent:** what exact work was declared.
- **Authority:** which exact action was permitted, under which policy and
  restrictions.
- **Execution:** what actually ran, completed, failed, or remains blocked.
- **Continuity:** what the next authorized executor may safely do without
  recreating permission from chat history.

That produces durable, bounded, and fail-closed answers to:

- What was the agent authorized to do?
- Which workflow, policy, and inputs governed the action?
- What evidence and checks supported the decision?
- Who or what approved it?
- What side effects occurred?
- What may happen next?

Workflow OS puts those facts in a deterministic, fail-closed kernel instead of
leaving them inside prompts, chat history, model traces, or agent memory.
Evidence references, events, audit records, SideEffects, and WorkReports make
the governed state inspectable; they are outputs of the control function, not
the product category by themselves.

The current source tree is preparing **Workflow OS `0.4.0-preview.1` - Governed
Authority and Repository Adoption Preview**. It is useful today for local
governed workflows and engineering evaluation. It is not yet a production
enterprise control plane.

## See The Kernel Work

A workflow declares ordered work, the skill or adapter used at each step, its
policy requirements, and any approval boundary. This abridged example requires
approval before a repository metadata read:

```yaml
schema_version: workflowos.dev/v0
id: example/github-repository-metadata-fixture
version: v1
autonomy_level: level_2
steps:
  - id: read
    skill_ref:
      id: adapter/github-repository-metadata
      version: v1
    policy_requirements: [{ id: external/read }]
    approval_policy:
      policy: { id: approval/github-read }
    terminal_behavior: fail_workflow
approval_requirements:
  - id: repository-read
    reason: Approval is required before the metadata read.
```

The [complete example](examples/github-repository-metadata-fixture/workflows/read.workflow.yml)
includes ownership, trigger, timeout, audit, and observability declarations.

Starting an approval-gated run produces an explicit checkpoint rather than
silently invoking the skill:

```text
run_id: run-...
status: WaitingForApproval
approval_id: approval/run-.../read
```

After approval, `workflow-os inspect <run-id>` exposes the ordered history:

```text
RunCreated -> PolicyDecisionRecorded -> ApprovalRequested -> ApprovalGranted
-> RunResumed -> SkillInvocationStarted -> SkillInvocationSucceeded
-> RunCompleted
```

For a qualifying low-risk local path, proportional governance can proceed
without an approval interruption while still persisting the governed check
result and report:

```text
status: Completed
governance: quiet_success
report: persisted report/run-...
inspect: workflow-os inspect run-...
```

That is the core product loop: execution stays flexible inside a governed work
envelope, while the exact authority-to-execution relationship and the next
lawful action remain recoverable. Evidence posture, side-effect records, and
completion claims make that continuity inspectable.

## How It Works

```text
workflow + policy + authority + evidence requirements
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
        events + evidence references + WorkReport
```

For each material action, Workflow OS:

1. validates the workflow and policy;
2. binds the run to immutable definitions and inputs;
3. evaluates authority, checks, evidence, and approval requirements;
4. proceeds quietly, discloses, blocks for approval, or denies;
5. records durable events and supported side-effect outcomes; and
6. on supported paths, produces an inspectable WorkReport and derives the next
   permitted action.

Workflow OS does not model every reasoning step or tool-selection edge. The
agent remains free to explore inside the authorized envelope. The kernel
governs the boundaries where work consumes authority, changes durable state,
invokes a capability, creates a side effect, or claims completion.

## Why Workflow OS

Most agent infrastructure governs one layer of the problem:

| Category | What it does well | What Workflow OS adds |
| --- | --- | --- |
| Agent frameworks and graph orchestrators | Run model loops, tools, handoffs, and durable execution | A separate authority boundary that does not require every reasoning or execution edge to be encoded in a graph |
| Sandboxes and security runtimes | Contain processes, files, networks, credentials, and tool access | The work-level contract that explains why execution was authorized, which evidence was required, and what the result may claim |
| Action-governance runtimes and gateways | Enforce identity, capability, policy, and approval around individual agent actions | A repository-adoptable contract spanning declared work, immutable inputs, required evidence and checks, side-effect accounting, terminal WorkReports, and lawful continuation |
| Policy and authorization engines | Decide whether a principal may perform an action | Durable workflow identity, approval context, evidence and check obligations, side-effect records, and terminal reporting |
| Observability and evaluation platforms | Trace, inspect, score, and debug what ran | Fail-closed decisions before material actions plus an assurance-labeled record of what was approved and completed |
| Workflow OS | Governs the complete unit of work across those layers | One inspectable chain from declared intent and authority to evidence references, side-effect records, outcome, WorkReport, and next permitted action |

This makes Workflow OS complementary to agent SDKs, deterministic workflow
engines, policy systems, observability platforms, and sandboxes. Those systems
can execute, contain, authorize, or observe individual parts of a run. Workflow
OS keeps the governed work contract durable across them.

The distinction is deliberate:

- **Govern the work, not the model loop.** Agents may plan and adapt without
  forcing every internal decision into a brittle graph.
- **Authority is not a prompt.** Scope, policy, approval, and capability state
  live outside model context and are reassessed at material boundaries.
- **A trace is not proof of permission.** Events are connected to the exact
  workflow, immutable inputs, checks, approvals, side-effect records, and
  evidence references whose assurance remains explicit.
- **Completion is a governed claim.** A WorkReport records what happened, what
  did not happen, what remains incomplete, and what may happen next.
- **Low risk does not mean no record.** Proportional governance can proceed
  quietly while preserving an inspectable governance and evidence-reference
  trail.

## Build And Try It

### Prerequisites

- Rust stable
- Node.js 20 for repository checks and SDK development

Build the current CLI from source:

```sh
git clone https://github.com/rcs2153/workflow-os.git
cd workflow-os
cargo build --release -p workflow-cli --bin workflow-os
export WORKFLOW_OS="$PWD/target/release/workflow-os"
```

Published preview archives and checksums are available from
[GitHub Releases](https://github.com/rcs2153/workflow-os/releases). The current
source tree may be newer than the latest published preview.

From a repository you want to govern:

```sh
cd /path/to/your-repository
"$WORKFLOW_OS" init-repo-governance --agent codex --dry-run
"$WORKFLOW_OS" init-repo-governance --agent codex
"$WORKFLOW_OS" validate
"$WORKFLOW_OS" first-run
```

`init-repo-governance` adds a local governance envelope while preserving
existing unmanaged `AGENTS.md` content. `first-run` safely inspects allowlisted
repository metadata and reports:

- what governance is already configured;
- missing ownership, checks, evidence, and side-effect decisions;
- concrete workflow candidates for the detected repository; and
- what Workflow OS did not inspect, execute, or change.

It does not read arbitrary source files, execute project commands, contact
providers, or invent evidence.

To persist one governed local WorkReport for the bounded first-run analysis:

```sh
"$WORKFLOW_OS" first-run --write-report-artifact
```

This path validates the generated contract, binds the analysis to immutable
run identity, executes the declared Workflow OS project-validation check,
selects the required governance level, and writes one create-only report after
completion. It still does not execute repository build or test commands, read
arbitrary source, access the network, or create external side effects.

For a fully local approval and event-history demonstration, see the
[vertical-slice approval example](examples/vertical-slice-approval/README.md).

## What Is Implemented

- Deterministic YAML loading and validation.
- Sequential multi-step local workflows.
- Immutable run inputs and approval-resume integrity.
- Policy decisions and proof-bound approvals.
- Durable event history, retries, escalation, cancellation, and rehydration.
- Evidence references, side-effect records, and terminal WorkReports, with
  path-specific assurance and explicit local storage-owner limits.
- Proportional governance: quiet capture, visible disclosure, approval, or
  denial.
- Scoped capability, current-authority, required-context, and check models.
- Local filesystem, SQLite continuity, and PostgreSQL state implementations.
- Fixture-first read-only GitHub, Jira, and GitHub Actions adapters.
- Narrow GitHub comment and managed draft-pull-request execution paths.
- A single-tenant hosted no-write alpha and optional OpenShell boundary.

Not every Core capability is exposed through the CLI. The
[Current Product Contract](docs/user-guide/current-product-contract.md)
distinguishes public execution paths from reviewed internal foundations.

## Current Boundaries

Workflow OS is not an agent runtime, prompt router, agent swarm, CI system, or
general automation engine. The current preview does not provide production
multi-tenant hosting, enterprise identity administration, broad provider
writes, arbitrary shell execution, or proof that caller-asserted work happened.

Missing handlers, unsupported authority, and ambiguous actions fail closed. A
model or schema in the repository is not a runtime capability until a tested
execution path enforces it.

Evidence assurance is path-specific. Content-bound dispatch proves which
selected executable bytes and request were used; it does not authenticate an
operator-selected executable as an official validator. Supported WorkReport
stores enforce exact artifact identity and a retained complete-body commitment
across accepted read, inspect, recovery, reconciliation, and replay paths. That
detects report-only substitution while the local anchor remains intact; local
storage still does not provide cryptographic tamper evidence against an owner
who can rewrite both record and anchor. See the [Current Product
Contract](docs/user-guide/current-product-contract.md) for the exact trust
boundary.

## Use It With An Agent

Give the coding agent one operating instruction:

```text
Use Workflow OS as the governing layer for this task.
Treat kernel state, not conversation history, as authoritative.
Execute only the work currently authorized by the kernel.
Do not bypass validation, policy, approval, evidence, check, or scope failures.
At closure, report checks, evidence, side effects, incomplete work, and the
kernel-authorized next action.
```

The agent still edits files and uses ordinary tools unless a reviewed Workflow
OS handler owns that execution. Workflow OS governs the work; an instruction
file alone does not give the kernel control of the agent.

## Read Next

- [Current Product Contract](docs/user-guide/current-product-contract.md): the
  implemented, experimental, and unavailable product surfaces.
- [Agent Harness Quickstart](docs/user-guide/agent-harness-quickstart.md): using
  Workflow OS with a coding agent.
- [Workflow OS Explainer](docs/concepts/WORKFLOW_OS_EXPLAINER.md): architecture
  and execution model.
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

The `dogfood/` workflows govern development of Workflow OS itself. They are
reference implementations, not defaults installed into downstream projects.

## Develop

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm ci
npm run check
npm run check:integrations
```

See [CONTRIBUTING.md](CONTRIBUTING.md) and
[SECURITY.md](SECURITY.md) before opening a change or reporting a vulnerability.

## License

Apache License 2.0. See [LICENSE](LICENSE).
