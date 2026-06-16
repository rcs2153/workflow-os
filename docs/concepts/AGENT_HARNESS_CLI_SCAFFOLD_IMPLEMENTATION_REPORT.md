# Agent Harness CLI Scaffold Implementation Report

## 1. Executive Summary

The agent harness CLI scaffold phase is implemented. `workflow-os init-agent-harness` now creates or updates local agent instruction scaffolding so users can connect Codex, Claude Code, or another coding agent to Workflow OS as the governing layer.

The command is documentation/scaffold-only. It does not run workflows, approve checkpoints, execute local checks, register handlers, mutate runtime state, persist reports, write report artifacts, call providers, add schemas, implement writes, or change release posture.

## 2. Scope Completed

- Added `workflow-os init-agent-harness`.
- Added `--output-dir`, `--agent generic|codex|claude`, `--force`, and `--dry-run`.
- Generated `AGENTS.md`.
- Generated `.workflow-os/agent-harness-prompt.md`.
- Added Workflow OS managed-block replacement.
- Added fail-closed behavior for unmanaged existing files unless `--force` is supplied.
- Added dry-run behavior that writes no files.
- Added focused CLI tests.
- Updated README, roadmap, user guide, CLI docs, governed work pattern docs, evidence-reference docs, and the scaffold plan.

## 3. Scope Explicitly Not Completed

- Runtime harness auto-generation is not implemented.
- Automatic runtime report generation is not implemented.
- Workflow execution from the scaffold command is not implemented.
- Approval decisions from the scaffold command are not implemented.
- Local check execution and handler registration are not implemented.
- Report artifact generation is not implemented.
- Persistence and StateBackend writes are not implemented.
- CLI report rendering is not implemented.
- Examples are not updated.
- Workflow spec schema changes are not implemented.
- Reasoning lineage is not implemented.
- Side-effect boundary modeling is not implemented.
- Write behavior is not implemented.
- Hosted or distributed runtime behavior is not implemented.
- Recursive agents, agent swarms, and Level 3/4 autonomy enablement are not implemented.

## 4. CLI Summary

The implemented command is:

```sh
workflow-os init-agent-harness [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]
```

By default, the command writes under `--project-dir`. `--output-dir` can target a different directory. `--agent` changes the generated audience label only; it does not change runtime behavior. `--dry-run` prints planned scaffold writes and writes nothing.

## 5. Generated Files

The command creates or updates:

- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

Both generated files include the managed markers:

```text
<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
<!-- END WORKFLOW OS AGENT HARNESS -->
```

Generated content preserves the operating model:

```text
Agent executes. Workflow OS governs.
```

## 6. Safety And Overwrite Behavior

If a target file does not exist, the command creates it.

If a target file exists with the Workflow OS managed block, the command updates only that block and preserves surrounding content.

If a target file exists without the managed block, the command fails closed unless `--force` is supplied.

The command does not inspect, validate, or execute workflow specs. It does not create `.workflow-os/state`.

## 7. Runtime Boundary

The scaffold command does not touch runtime state or executor APIs. It does not create runs, append events, approve checkpoints, inspect state, register local check handlers, run local checks, generate reports, or write artifacts.

## 8. Privacy And Redaction

Generated content does not include local absolute paths, environment values, provider payloads, raw command output, raw spec contents, private repository metadata, tokens, credentials, or secret-like values.

Errors use stable codes and avoid echoing existing file contents, prompt text, or rejected flag values.

## 9. Test Coverage Summary

Focused CLI tests cover:

- scaffold file creation;
- required generated wording;
- unsupported-capability warnings;
- unmanaged file fail-closed behavior;
- force replacement;
- managed-block update while preserving surrounding content;
- dry-run with no writes;
- no runtime state creation;
- invalid agent rejection without leaking the rejected value;
- CLI help visibility.

Existing CLI tests continue to cover validation, execution, approval, inspect, doctor, and JSON behavior.

## 10. Commands Run And Results

- `cargo test -p workflow-cli --test cli init_agent_harness` - passed.
- `cargo fmt --all --check` - passed after applying rustfmt.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 11. Remaining Known Limitations

- The scaffold command is intentionally generic and does not detect a user’s preferred coding agent automatically.
- It does not install hooks, commands, skills, runtime configuration, or schema fields.
- It does not scaffold workflow YAML.
- Existing unmanaged files require explicit `--force`.
- Agent-specific variants only change the generated audience label in this first slice.

## 12. Recommended Next Phase

Recommended next phase: agent harness CLI scaffold implementation review.

The implementation should be reviewed for overwrite safety, runtime non-interference, documentation honesty, generated wording, and test coverage before broader onboarding automation is considered.
