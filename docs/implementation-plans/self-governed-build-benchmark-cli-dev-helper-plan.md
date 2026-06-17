# Self-Governed Build Benchmark CLI/Dev-Helper Plan

Status: Implemented as a repo-local development helper in `scripts/self-governed-benchmark.mjs` with npm alias `dogfood:benchmark`. The implementation is reported in [Self-Governed Build Benchmark CLI/Dev-Helper Report](../concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_REPORT.md), accepted in [Self-Governed Build Benchmark CLI/Dev-Helper Review](../concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_REVIEW.md), hardened in [Self-Governed Build Benchmark CLI/Dev-Helper Hardening Report](../concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_HARDENING_REPORT.md), and accepted in [Self-Governed Build Benchmark CLI/Dev-Helper Hardening Review](../concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_HARDENING_REVIEW.md). This plan follows the accepted [Self-Governed Build Benchmark Behavior Test Review](../concepts/SELF_GOVERNED_BUILD_BENCHMARK_BEHAVIOR_TEST_REVIEW.md). It does not add stable public CLI behavior, change runtime semantics, register default local check handlers, write report artifacts, add schemas, execute arbitrary commands, add writes, implement reasoning lineage, or change release posture.

## 1. Executive Summary

The self-governed build benchmark is now documented, tested through explicit core APIs, and accepted with non-blocking follow-ups.

The next usability gap is local operation. Today a maintainer or agent must manually stitch together:

- building the CLI;
- validating the dogfood project;
- choosing a state directory;
- starting the dogfood workflow;
- copying the `run_id` and `approval_id`;
- approving the planning checkpoint;
- inspecting the run;
- remembering which boundaries are real kernel behavior and which checks remain manual.

This plan defines a conservative CLI/dev-helper path to reduce that friction without changing Workflow OS runtime behavior. The recommended first implementation is a **repo-local development helper script** with an npm alias, not a new stable product CLI subcommand.

The helper should make the dogfood loop easier to run, but it must preserve the core boundary:

```text
Agent executes. Workflow OS governs.
```

## 2. Goals

- Make the self-governed build benchmark easier for maintainers and agents to operate locally.
- Use existing `workflow-os` CLI commands instead of new runtime APIs.
- Keep the helper repo-local and clearly development-only.
- Preserve explicit approval behavior.
- Preserve existing workflow pass/fail semantics.
- Preserve existing local state behavior.
- Avoid hidden global state.
- Avoid automatic kernel control of agents.
- Avoid automatic local check execution.
- Avoid automatic report generation or artifact writing.
- Print bounded, copy/pasteable commands and status context for agent use.
- Keep unsupported behavior explicit and visible.

## 3. Non-Goals

Do not implement or authorize:

- a stable public Workflow OS CLI command in this phase;
- automatic runtime report generation;
- runtime result exposure changes;
- CLI report rendering;
- report artifact writing;
- automatic report artifact writing;
- automatic local check execution;
- default `DocsCheckLocalHandler` registration;
- arbitrary shell execution;
- command-output evidence attachment;
- workflow schema changes;
- workflow-declared benchmark behavior;
- workflow-declared hooks;
- runtime hook configuration;
- hook warning/skipped continuation;
- approval evidence attachment;
- reasoning lineage or claim graph;
- side-effect boundary enforcement;
- write-capable adapters;
- repository writes from inside the kernel;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- production self-hosting claims;
- Level 3 or Level 4 autonomy claims;
- release posture changes.

## 4. Current Surface

Current generic CLI commands:

- `workflow-os validate`
- `workflow-os run <workflow-id> [--run-id <run-id>]`
- `workflow-os status <run-id>`
- `workflow-os approve <run-id> <approval-id> [--deny] [--actor <actor>] [--reason <reason>]`
- `workflow-os inspect <run-id>`
- `workflow-os doctor`
- `workflow-os doctor state`
- `workflow-os init-agent-harness`

Current benchmark docs require maintainers to combine those commands manually for:

- `--project-dir dogfood/workflow-os-self-governance`;
- `--state-dir <local benchmark state>`;
- `--mock-all-local-skills`;
- workflow ID `dg/d`;
- explicit approval actor and reason.

Current behavior-test coverage proves the dogfood loop through explicit core APIs, but normal CLI usage still requires manual orchestration.

## 5. Candidate Helper Options

| Option | Assessment | Recommendation |
| --- | --- | --- |
| Public `workflow-os dogfood ...` command | Too product-specific for the core CLI and risks implying stable product behavior. | Reject for now. |
| Public `workflow-os benchmark ...` command | More generic, but still premature without report/handler/runtime decisions. | Defer. |
| Repo-local script under `scripts/` | Fits existing repo tooling and can be clearly development-only. | Accept as first implementation target. |
| npm alias wrapping the repo-local script | Improves discoverability without changing Rust CLI contracts. | Accept. |
| Docs-only command snippets | Already exists and is still too manual. | Insufficient. |

Recommended first target: `scripts/self-governed-benchmark.mjs` plus npm aliases such as `dogfood:benchmark` or `dogfood:self-governed`.

The exact script name can be adjusted during implementation to match repository conventions, but it should remain obviously repo-local and dogfood-specific.

## 6. Helper Boundary

The helper should be a thin, explicit wrapper around existing CLI commands.

It may:

- locate the repository root;
- locate or build `target/debug/workflow-os`;
- accept an explicit `--state-dir`;
- default to a repo-documented temporary state directory only when no state directory is supplied;
- validate the dogfood project using `workflow-os validate`;
- start the dogfood workflow using `workflow-os run dg/d`;
- inspect or status a supplied run ID;
- approve a supplied run ID and approval ID with explicit actor and reason;
- print the benchmark agent prompt or a pointer to the runbook;
- print next-step guidance after successful commands.

It must not:

- bypass the Workflow OS CLI;
- mutate workflow state except by invoking existing CLI commands the user explicitly requested;
- approve automatically;
- infer approval IDs and approve them silently;
- register default local check handlers;
- run `DocsCheckLocalHandler` implicitly;
- run `npm`, `cargo`, shell, provider, or adapter commands beyond the bounded CLI build/validate/run/approve/status/inspect helper operations;
- write report artifacts;
- render reports;
- create EvidenceReference, typed handoff, hook, local check result, or WorkReport values;
- call live adapters or external systems;
- fabricate run IDs, approval IDs, audit events, evidence, local check results, reports, or command output.

## 7. Proposed Command Shape

First implementation should prefer a small repo-local helper with subcommands:

```sh
npm run dogfood:benchmark -- commands
npm run dogfood:benchmark -- validate
npm run dogfood:benchmark -- start [--state-dir <path>] [--run-id <run-id>]
npm run dogfood:benchmark -- status <run-id> [--state-dir <path>]
npm run dogfood:benchmark -- inspect <run-id> [--state-dir <path>]
npm run dogfood:benchmark -- approve <run-id> <approval-id> --reason <reason> [--actor <actor>] [--state-dir <path>]
npm run dogfood:benchmark -- prompt
```

The helper should also support:

- `--dry-run` to print commands without executing them;
- `--json` only if it can transparently pass through existing CLI JSON output without creating a new machine contract;
- explicit failure when required arguments are missing.

The helper should not implement a one-shot `go` command that validates, starts, approves, and inspects in one ambient sequence. That would make the approval boundary feel ceremonial rather than governed.

## 8. State Directory Policy

The helper should make state explicit.

Recommended behavior:

- accept `--state-dir <path>`;
- if omitted, use a documented temporary default under the OS temp directory, such as `<tmp>/workflow-os-self-governance-state`;
- print the resolved state directory before running commands;
- never delete state automatically;
- never reset state automatically;
- never write outside the state directory except for normal CLI build artifacts if a build step is invoked;
- avoid hidden dotfile state for the first implementation.

If a future helper needs named benchmark sessions, that should be separately planned.

## 9. Build And Toolchain Policy

The helper may build the local CLI if `target/debug/workflow-os` is missing.

Rules:

- use `cargo build -p workflow-cli --bin workflow-os`;
- do not install Rust, Node, npm packages, or other dependencies;
- do not run `npm ci`;
- do not modify lockfiles;
- do not fetch network dependencies beyond whatever the caller's local toolchain already performs;
- clearly print when a build is being run;
- support `--no-build` to require an existing binary.

The helper must not treat build success as benchmark validation beyond producing the CLI binary.

## 10. Approval Policy

Approval must remain explicit.

The helper may provide an `approve` subcommand only when the caller supplies:

- `run_id`;
- `approval_id`;
- `--reason`;
- optional `--actor`.

It must not:

- auto-discover the latest approval and grant it;
- approve immediately after `start`;
- use an empty or generic hidden reason;
- approve denied or terminal runs silently;
- convert approval failure into success.

This keeps the benchmark useful as governance instead of theatre.

## 11. Output Policy

The helper output should be bounded and operator-oriented.

It may print:

- command being run;
- resolved project directory;
- resolved state directory;
- next recommended command;
- run ID and approval ID when the underlying CLI prints them;
- links or paths to the benchmark runbook and agent prompt section.

It must not:

- copy raw command transcripts into evidence;
- create WorkReports;
- render report payloads;
- print secret-like values;
- print raw local state file contents;
- claim manual checks were kernel-executed;
- claim real docs check execution happened when using `--mock-all-local-skills`.

## 12. Error Handling

Errors should be stable, bounded, and honest.

The helper should:

- exit non-zero when the wrapped CLI command fails;
- preserve the existing CLI error code where practical;
- add only bounded helper context;
- avoid printing raw command output beyond normal CLI stderr/stdout;
- avoid leaking paths beyond the explicitly supplied project/state paths;
- fail fast on unknown subcommands or missing required arguments;
- make unsupported operations explicit.

Recommended helper-level error labels:

- `dogfood.helper.usage`
- `dogfood.helper.binary_missing`
- `dogfood.helper.command_failed`
- `dogfood.helper.unsupported`

These labels are script-facing, not new Workflow OS core error codes.

## 13. Test Plan

Future implementation tests should cover:

- `commands` prints bounded command guidance;
- `validate --dry-run` prints the expected CLI command without executing;
- `start --dry-run` prints the expected CLI command with dogfood project, state dir, mock flag, workflow ID, and optional run ID;
- `approve --dry-run` requires run ID, approval ID, and reason;
- missing approval reason fails closed;
- helper does not auto-approve after start;
- helper does not create report artifacts;
- helper does not add CLI report rendering;
- helper does not register default local check handlers;
- helper does not run arbitrary commands;
- helper does not fabricate run IDs or approval IDs;
- helper output does not include secret-like test values;
- helper can run validate/start/approve/inspect against the dogfood project using existing CLI commands, if the test environment permits;
- existing CLI tests still pass;
- `npm run check:docs` passes.

If integration-style helper tests would be too slow or brittle, the first implementation can use dry-run command-shape tests plus one bounded smoke path that relies on the existing CLI binary build pattern.

## 14. Documentation Updates For Implementation

Future implementation should update:

- `package.json` with a repo-local npm alias;
- `docs/user-guide/self-governed-build-benchmark.md` with helper usage;
- `dogfood/workflow-os-self-governance/README.md` with helper usage;
- `docs/implementation-plans/self-governed-build-benchmark-plan.md` with status;
- `ROADMAP.md` with the helper implementation status;
- a phase report under `docs/concepts/`.

Docs must clearly state:

- the helper is repo-local development tooling;
- it does not add public product CLI behavior;
- it does not make report generation automatic;
- it does not run real local checks by default;
- it does not approve automatically;
- it does not persist report artifacts;
- it does not implement writes, schemas, reasoning lineage, recursive agents, hosted execution, or production self-hosting.

## 15. Proposed Implementation Sequence

1. Add `scripts/self-governed-benchmark.mjs` with `commands`, `validate`, `start`, `status`, `inspect`, `approve`, and `prompt`.
2. Add an npm alias such as `dogfood:benchmark`.
3. Add dry-run command-shape tests if a script test harness is practical.
4. Add one bounded dogfood smoke test only if it can run without adding network, persistence beyond explicit state dir, or default handlers.
5. Update the benchmark runbook and dogfood README.
6. Create an implementation report.
7. Review before adding any public CLI command or report-bearing helper behavior.

## 16. Deferred Work

Deferred until separately planned and reviewed:

- stable product CLI benchmark commands;
- public CLI report-bearing dogfood execution;
- automatic report generation;
- automatic report artifact writing;
- default local check handler registration;
- live `DocsCheck` execution from normal CLI paths;
- real cargo/npm check broadening;
- local check result propagation into report inputs;
- runtime-produced typed handoffs;
- runtime-produced hook disclosures;
- command-output evidence;
- workflow schema changes;
- workflow-declared hooks;
- side-effect boundary enforcement;
- write-capable adapters;
- reasoning lineage;
- hosted/distributed execution.

## 17. Final Recommendation

Next implementation phase: **repo-local self-governed benchmark dev-helper implementation**.

Start with a repo-local Node helper and npm alias that wraps existing CLI commands for the dogfood benchmark. Do not add a stable public Rust CLI command yet. Do not add automatic approval, automatic local checks, automatic reports, report artifacts, schema changes, writes, reasoning lineage, recursive agents, hosted execution, or release posture changes.
