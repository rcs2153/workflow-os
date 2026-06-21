# Workflow OS

Workflow OS v0 is a **public local kernel preview** of a local-first framework for defining, validating, testing, executing, governing, observing, and eventually scaling AI-driven workflows.

Current preview version: `0.2.0-preview.1`.

This preview exposes the core kernel shape: declarative workflow and skill specs, deterministic validation, event-sourced local execution, policy gates, approval pause/resume, bounded retry and escalation semantics, local durable state, audit/observability signals, CLI commands, TypeScript spec-generation helpers, adapter contracts, evidence-reference foundations, and early in-memory work-report model/helper APIs.

It is **not** a production distributed runtime. It is not a hosted product, adapter-complete framework, enterprise deployment platform, or Level 3/4 autonomy system. The `0.1.0-preview.1` release established the local kernel preview. The `0.2.0-preview.1` posture adds a narrow public read-only integration preview for GitHub, Jira, and GitHub Actions / CI. No GitHub/Jira write behavior, CI rerun/dispatch/cancel behavior, generic live adapter execution, automatic runtime work-report generation, CLI report rendering, distributed workers, production database backend, UI, hosted service, or marketplace/package registry exists. Level 1 and Level 2 autonomy are the only default posture. Level 3 and Level 4 are declaration-only and denied by default.

## Product Boundary

Workflow OS Core is a local-first, declarative kernel for governed AI workflows through explicit local skills and future adapters.

In v0, Core does:

- Load `workflow-os.yml` projects and versioned YAML specs.
- Validate workflows, skills, policies, and tests before execution.
- Execute a narrow sequential local workflow path.
- Persist and rehydrate event-sourced local runs.
- Enforce conservative policy checks before meaningful runtime actions.
- Pause and resume approval-gated local runs.
- Record local audit and observability signals.
- Model evidence references and selected evidence attachment paths.
- Model work-report contracts/reports and provide explicit in-memory local report helper APIs.
- Provide CLI commands for local validation, execution, approval, status, inspection, and doctor checks.
- Provide a TypeScript SDK for spec generation only.
- Define adapter contracts. `0.2.0-preview.1` adds GitHub, Jira, and GitHub Actions read-only adapters for fixture-first and opt-in live evaluation.

Workflow OS Core is not:

- A replacement for Temporal, Airflow, GitHub Actions, or Zapier.
- A generic business process management engine.
- A chat agent framework.
- A one-off repository, ticketing, or CI automation tool.
- An enterprise SaaS application.
- A UI product in v0.
- A production distributed runtime.
- A GitHub/Jira write or CI write/rerun/dispatch integration layer in v0.
- A Level 3/4 autonomy runtime by default.

See [docs/PROJECT_CHARTER.md](docs/PROJECT_CHARTER.md) and [docs/ENGINEERING_STANDARD.md](docs/ENGINEERING_STANDARD.md) before contributing.

For a narrative overview of what Workflow OS is, what it does, and how the kernel fits together, read [Workflow OS Explainer](docs/concepts/WORKFLOW_OS_EXPLAINER.md).

## User Guide

The [Workflow OS User Guide](docs/user-guide/README.md) provides RC1 internal evaluation documentation: an agent harness quickstart, a rewritten field guide, a fillable workbook, and safe evaluation paths for the local kernel, vertical slice, read-only fixture adapters, and adapter telemetry inspection.

The user guide preserves the operating-model ideas from the earlier field guide while keeping the current implementation boundary explicit: local kernel preview, public read-only integration preview, early core evidence/work-report foundations, no write-capable adapters, no production backend, no distributed workers, no hosted service, no UI, and no Level 3/4 autonomy enablement.

## Use Workflow OS With A Coding Agent

The intended local adoption loop is not only hand-writing YAML and manually testing the kernel. The lightbulb path is to point Codex, Claude Code, or another coding agent at the repository and instruct it to use Workflow OS as the governing layer.

```text
Agent executes. Workflow OS governs.
```

Workflow OS validates the project, creates durable local run state, enforces policy and approval checkpoints, records inspectable event history, and preserves report posture. The coding agent still performs repository edits and normal validation commands unless a real local handler has been explicitly implemented, registered, and reviewed.

Start here:

- [Agent Harness Quickstart](docs/user-guide/agent-harness-quickstart.md)
- [Workflow OS self-governance dogfood](dogfood/workflow-os-self-governance/README.md)
- [Root agent instructions](AGENTS.md)

The self-governance dogfood workflows are Workflow OS's own build-governance workflows. They are useful reference patterns, but they are not community defaults, product templates, or workflows that every downstream user is expected to install. Portable examples live under `examples/`; user and team workflows should live in the user's own Workflow OS project or, in future phases, a governed workflow catalog/store.

You can scaffold local agent instructions with:

```sh
workflow-os init-agent-harness
```

This creates or updates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` only. It does not run workflows, approve checkpoints, execute local checks, register handlers, persist reports, write runtime state, or enable hosted or higher-autonomy behavior.

Copy/paste setup prompt:

```text
Use Workflow OS as the governing layer for this task.
Validate the relevant Workflow OS project before work.
Start or resume the governed workflow when the task requires it.
Treat approvals as mandatory checkpoints.
Do not bypass failed validation, denied policy, missing approvals, failed checks, or explicit scope limits.
Do not invent workflow state, approvals, evidence, audit events, reports, validation results, or command outputs.
Report completed scope, deferred scope, validation results, and the recommended next phase.
```

This is kernel-governed agent work, not recursive agents, agent swarms, hosted orchestration, production self-hosting, automatic local command execution, or Level 3/4 autonomy.

## Repository Layout

```text
crates/
  workflow-core/        Canonical Rust core home.
  workflow-cli/         Rust CLI home.
packages/
  sdk-typescript/       TypeScript SDK ergonomics layer.
examples/
  vertical-slice-approval/
  github-read-only-review-context/
  jira-read-only-intake-quality/
  ci-read-only-failure-summary/
dogfood/
  workflow-os-self-governance/  Workflow OS's own dogfood workflows, not community defaults.
docs/
  user-guide/           Field guide, workbook, and RC1 internal evaluation guide.
  adr/                  Architecture decision records.
  architecture/         Architecture documentation.
  concepts/             Public concepts.
  specs/                Spec documentation.
  runtime/              Runtime invariants and behavior.
  cli/                  CLI documentation.
  sdk/                  SDK documentation.
  operations/           Operational runbooks.
  security/             Security documentation.
  release/              Versioning and release process.
```

## Local Checks

Required local tools:

- Rust toolchain with `cargo`, `rustfmt`, and `clippy`.
- Node.js 20 or newer.
- npm 10 or newer.

Run the baseline checks:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
npm ci
npm run check
npm run check:integrations
```

`npm run check:integrations` runs the Phase 2 read-only adapter contract gate for GitHub, Jira, and CI/GitHub Actions using offline fixtures. It does not require live credentials.

The Phase 2 gate reflects `0.2.0-preview.1` read-only adapter preview work. It is fixture-first, credential-free in normal CI, and does not imply write support, production integration readiness, or broad live provider compatibility.

## Try The Vertical Slice

Build the CLI and run the local approval example:

```sh
cargo build -p workflow-cli --bin workflow-os
target/debug/workflow-os --project-dir examples/vertical-slice-approval validate
target/debug/workflow-os --project-dir examples/vertical-slice-approval --mock-all-local-skills run ex/review
```

The run pauses for approval and prints a `run_id` plus `approval_id`.

```sh
target/debug/workflow-os --project-dir examples/vertical-slice-approval --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-approver --reason reviewed-local-example
target/debug/workflow-os --project-dir examples/vertical-slice-approval inspect <run-id>
```

This example uses an explicitly enabled deterministic local mock skill. The CLI does not execute arbitrary declared `local/*` skills by default. The example exercises the v0 kernel path without external services, secrets, real adapters, distributed workers, production backends, or production deployment claims.

## Try The Workflow OS Self-Governance Dogfood

Workflow OS has begun dogfooding its own local kernel. The dogfood project uses the kernel as a sequential multi-step governance wrapper for Workflow OS planning/docs work while Codex or a human still performs repository edits outside the kernel.

```sh
target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate
target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-self-governance-state --mock-all-local-skills run dg/d
```

The run pauses at the planning approval step. After approval, the remaining governed placeholder steps complete and the run can be inspected:

```sh
target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-self-governance-state --mock-all-local-skills approve <run-id> <approval-id> --actor user/dogfood-reviewer --reason reviewed-governance-task
target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-self-governance-state inspect <run-id>
```

This is **kernel-governed, Codex-executed** dogfooding. It does not execute build commands, mutate repository files, run recursive agents, implement agent swarms, or claim production self-hosting.

## Try The Phase 2 GitHub Read-Only Example

This example exists to evaluate `0.2.0-preview.1` read-only adapter behavior. It is a fixture-backed preview example, not production GitHub automation.

The GitHub read-only reference example uses fixture data and does not require GitHub credentials:

```sh
target/debug/workflow-os --project-dir examples/github-read-only-review-context validate
target/debug/workflow-os --project-dir examples/github-read-only-review-context --mock-all-local-skills run ex/gh
```

The run pauses for approval before producing the final recommendation summary.

```sh
target/debug/workflow-os --project-dir examples/github-read-only-review-context --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-reviewer --reason reviewed-fixture-context
target/debug/workflow-os --project-dir examples/github-read-only-review-context inspect <run-id>
```

This example reads fixture-backed pull request metadata and changed files through the GitHub read-only adapter contract. It does not post comments, create branches, open pull requests, update GitHub, or require live credentials.

## Try The Phase 2 Jira Read-Only Example

This example exists to evaluate `0.2.0-preview.1` read-only adapter behavior. It is a fixture-backed preview example, not production Jira automation.

The Jira read-only reference example uses fixture data and does not require Jira credentials:

```sh
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality validate
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality --mock-all-local-skills run ex/jira
```

The run pauses for approval before producing the final intake-quality recommendation.

```sh
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-reviewer --reason reviewed-fixture-intake
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality inspect <run-id>
```

This example reads fixture-backed issue metadata, description presence, and comments through the Jira read-only adapter contract. It does not update Jira, add comments, change status, assign users, change labels, or require live credentials.

## Try The Phase 2 CI Read-Only Example

This example exists to evaluate `0.2.0-preview.1` read-only adapter behavior. It is a fixture-backed preview example, not production CI automation.

The CI read-only reference example uses fixture-backed GitHub Actions data and does not require GitHub credentials:

```sh
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary validate
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary --mock-all-local-skills run ex/ci
```

The run pauses for approval before producing the final failure diagnosis summary.

```sh
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-reviewer --reason reviewed-fixture-ci-context
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary inspect <run-id>
```

This example reads fixture-backed workflow run metadata, job status, failure context, and bounded redacted log excerpts through the GitHub Actions read-only adapter contract. It does not rerun CI, dispatch workflows, cancel workflows, modify checks, write comments, attempt auto-repair, or require live credentials.

## Current Status

Workflow OS currently has a local-first v0 kernel foundation: declarative specs, validation, event-sourced local execution, approvals, policy checks, durable local state, audit/observability signals, CLI commands, TypeScript spec-generation helpers, selected evidence-reference attachment paths, early work-report model/helper APIs, and a first self-governance dogfood project. `0.2.0-preview.1` adds GitHub/Jira/GitHub Actions read-only adapter preview work. GitHub/Jira writes, CI rerun/dispatch/cancel behavior, generic live adapter execution, automatic report generation for every run, CLI report rendering, production integration readiness, distributed workers, production deployment backends, hosted services, and Level 3/4 execution by default have not been implemented.

See [docs/release/V0_READINESS.md](docs/release/V0_READINESS.md) and [docs/release/V0_KNOWN_LIMITATIONS.md](docs/release/V0_KNOWN_LIMITATIONS.md) for the current readiness assessment.

## License

Workflow OS is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
