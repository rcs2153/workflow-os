# `workflow-os init-repo-governance`

`workflow-os init-repo-governance` scaffolds a minimal local Workflow OS project envelope in an existing repository.

```sh
workflow-os init-repo-governance
```

It creates:

- `workflow-os.yml`
- `workflows/first-run-governance.workflow.yml`
- `skills/first-run-report.skill.yml`
- `policies/default-governance.policy.yml`
- `policies/local.policy.yml`
- `tests/first-run-governance.test.yml`
- `.workflow-os/README.md`
- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

The generated project is intentionally conservative. It gives the repository a valid local Workflow OS contract and a first-run, approval-gated, mockable workflow that reflects the Governed Work Pattern posture: bounded goal, context, evidence, checks, approval, side-effect disclosure, risks, skipped work, final report closure, and future workflow recommendations.

## Options

```text
workflow-os init-repo-governance [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]
```

- `--output-dir <path>`: write scaffold files under the given directory. Defaults to `--project-dir`.
- `--agent generic|codex|claude`: tune the generated prompt label. Defaults to `generic`.
- `--force`: replace existing scaffold targets.
- `--dry-run`: show the files that would be written without writing them.

## First Run

After scaffolding:

```sh
workflow-os validate
workflow-os first-run
workflow-os --mock-all-local-skills run local/first-run-governance
```

`first-run` emits the immediate report-ready context: safe project counts, all v1 report section posture, explicit missing evidence, skipped checks, unsupported side effects, bounded risks, and review-only workflow recommendations.

The explicit `run` command remains separate and pauses for human approval before the mock first-run report step completes.

`--mock-all-local-skills` is a local preview convenience. It is not proof that the generated skill has a real handler.

First-run governed ledger/report posture is implemented in [First-Run Governed Ledger/Report Plan](../implementation-plans/first-run-governed-ledger-report-plan.md) as `workflow-os first-run`. It produces a report-ready context, not a terminal WorkReport from a completed workflow run.

## File Safety

The command fails closed if a plain scaffold target already exists, unless `--force` is supplied.

Generated `AGENTS.md` uses the same managed block behavior as
`init-agent-harness`: existing managed Workflow OS blocks are updated in place,
and unmanaged surrounding content is preserved by default. If no managed block
exists, Workflow OS appends its managed block instead of replacing existing
repo-specific agent guidance. `--force` is the explicit replacement boundary.

Generated `.workflow-os/agent-harness-prompt.md` remains a managed scaffold
target. Existing unmanaged prompt files fail closed unless `--force` is
supplied.

Errors use stable codes and avoid echoing file contents, prompt text, or secret-like values.

## Boundary

This command does not:

- run workflows;
- approve checkpoints;
- execute arbitrary shell commands;
- execute repository edits;
- register real local skill handlers;
- write runtime state;
- create report artifacts;
- call GitHub, Jira, CI, or other providers;
- create branches, pull requests, issues, comments, labels, or CI reruns;
- change workflow schemas;
- enable hosted behavior;
- enable recursive agents or agent swarms;
- enable Level 3/4 autonomy.

The boundary remains:

```text
Agent executes. Workflow OS governs.
```
