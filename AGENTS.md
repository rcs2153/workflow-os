# Workflow OS Agent Instructions

This repository uses Workflow OS as the governing layer for Workflow OS work.

If you are a coding agent working in this repository, treat the kernel as the governance boundary and Codex/Claude/human editing as the execution layer.

Core rule:

```text
Agent executes. Workflow OS governs.
```

## Required Posture

- Read `docs/ENGINEERING_STANDARD.md` before implementation or review work.
- Use the relevant roadmap, ADR, implementation plan, report, and review docs before changing files.
- Validate the relevant Workflow OS project or dogfood project before governed work where the task requires it.
- Start or resume the appropriate governed workflow when the task asks for kernel dogfooding or phase execution.
- Treat Workflow OS approval checkpoints as mandatory.
- Preserve deterministic validation, policy gates, durable state, auditability, and final reporting.
- Return structured implementation/review reports in the repository's established format.

## Do Not Bypass Governance

Do not:

- bypass failed validation, denied policy, missing approval, or failed checks;
- invent workflow state, run IDs, approval IDs, evidence references, audit events, work reports, validation results, or command outputs;
- widen phase scope without explicit user approval;
- silently enable local command execution;
- mutate Workflow OS state files by hand to force a result;
- replace deterministic validation with model self-review.

## Current Boundary

Workflow OS currently supports local kernel execution, sequential multi-step runs, approvals, durable local state, selected evidence/report foundations, and explicit local check handler infrastructure.

It does not currently provide:

- recursive agents;
- agent swarms;
- production nested harness execution;
- hosted/distributed runtime;
- automatic local check execution by default;
- write-capable adapters;
- side-effect boundary implementation;
- production self-hosting;
- Level 3/4 autonomy by default.

## Recommended Agent Loop

1. Read the relevant docs and current roadmap state.
2. Validate the relevant Workflow OS project.
3. Start or resume the governed workflow if this is a governed phase.
4. Pause for approval when required.
5. Implement only the approved scope.
6. Run required validation commands.
7. Report completed scope, deferred scope, validation results, and next recommended phase.

For the detailed user-facing setup path, see `docs/user-guide/agent-harness-quickstart.md`.
