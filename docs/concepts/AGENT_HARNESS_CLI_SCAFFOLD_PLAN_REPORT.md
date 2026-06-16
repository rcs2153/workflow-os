# Agent Harness CLI Scaffold Plan Report

Report date: 2026-06-16

## 1. Executive Summary

The agent harness CLI scaffold planning phase is complete.

The plan defines a future documentation/scaffold-only command, likely `workflow-os init-agent-harness`, that can generate or update `AGENTS.md` and a local session prompt for kernel-governed coding-agent work.

No CLI command, runtime automation, workflow schema field, automatic local check execution, write behavior, hosted behavior, recursive agent behavior, agent swarm behavior, or release posture change was implemented.

## 2. Scope Completed

- Added `docs/implementation-plans/agent-harness-cli-scaffold-plan.md`.
- Defined the proposed command shape.
- Defined generated file targets.
- Defined safe generated content policy.
- Defined overwrite and managed-block behavior.
- Defined runtime/authority non-goals.
- Defined privacy/redaction rules.
- Defined future implementation tests and docs updates.

## 3. Scope Explicitly Not Completed

- No `workflow-os init-agent-harness` command.
- No CLI behavior.
- No runtime harness generation.
- No workflow schema fields.
- No automatic local check execution.
- No default local command handler registration.
- No report artifact writing.
- No persistence.
- No command-output evidence.
- No approval evidence attachment.
- No reasoning lineage.
- No side-effect boundary modeling.
- No writes.
- No hosted/distributed runtime behavior.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy enablement.
- No release posture change.

## 4. Command Planning Summary

The planned command is:

```sh
workflow-os init-agent-harness
```

The first implementation should be a narrow scaffold command that writes documentation/instruction files only. Candidate generated files:

- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

The command should not execute workflows, approve checkpoints, register handlers, run local checks, create report artifacts, or touch state backends.

## 5. Safety Boundary Summary

The plan requires conservative overwrite behavior:

- create files when absent;
- update only recognizable managed blocks;
- fail closed on unmanaged existing files unless explicitly forced;
- support dry-run behavior without writes.

Generated content must preserve the product boundary:

```text
Agent executes. Workflow OS governs.
```

## 6. Test Coverage Summary

The future implementation test plan covers:

- file creation;
- managed-block updates;
- dry-run behavior;
- overwrite protection;
- generated content assertions;
- non-execution boundaries;
- non-leaking errors;
- CLI help documentation;
- existing CLI and workspace regression tests.

## 7. Commands Run And Results

Validation commands for this planning phase:

- `npm run check:docs`
  - Passed.

## 8. Remaining Known Limitations

- The scaffold command is not implemented.
- Agent setup remains manual through existing docs and root `AGENTS.md`.
- Generated prompt variants for specific tools are not designed beyond a possible future option.
- The interaction between repository root, project root, and generated file location needs final implementation design.

## 9. Recommended Next Phase

Recommended next phase: **agent harness CLI scaffold plan review**.

The review should verify the plan is documentation/scaffold-only, preserves product boundaries, and does not authorize runtime automation, local check execution, writes, schemas, hosted behavior, recursive agents, agent swarms, or release posture changes.
