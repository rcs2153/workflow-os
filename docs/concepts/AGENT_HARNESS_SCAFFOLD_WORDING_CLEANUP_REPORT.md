# Agent Harness Scaffold Wording Cleanup Report

## 1. Executive Summary

The scaffold adoption wording cleanup is implemented. The generated `.workflow-os/agent-harness-prompt.md` now repeats the core adoption slogan:

```text
Agent executes. Workflow OS governs.
```

The CLI also prints a bounded next-step hint telling users to paste `.workflow-os/agent-harness-prompt.md` into their coding agent. A prompt-file-specific unmanaged overwrite regression test was added.

This remains documentation/scaffold-only. No runtime harness generation, workflow execution, approvals, local check execution, handler registration, persistence, report artifacts, schemas, examples, reasoning lineage, side effects, writes, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes were introduced.

## 2. Scope Completed

- Added the exact slogan to the generated prompt file.
- Added a post-command next-step line pointing users to `.workflow-os/agent-harness-prompt.md`.
- Added a regression test for unmanaged `.workflow-os/agent-harness-prompt.md` fail-closed behavior.
- Preserved existing managed-block behavior.
- Preserved existing `--force`, `--dry-run`, `--agent`, and `--output-dir` behavior.

## 3. Scope Explicitly Not Completed

- Runtime harness auto-generation is not implemented.
- Workflow execution from the scaffold command is not implemented.
- Approval decisions from the scaffold command are not implemented.
- Automatic local check execution is not implemented.
- Local check handler registration is not implemented.
- Workflow schema fields are not implemented.
- Persistence and report artifacts are not implemented.
- CLI report rendering is not implemented.
- Example integration updates are not implemented.
- Reasoning lineage is not implemented.
- Side-effect boundary modeling is not implemented.
- Write behavior is not implemented.
- Hosted or distributed runtime behavior is not implemented.
- Recursive agents and agent swarms are not implemented or positioned.
- Level 3 or Level 4 autonomy enablement is not implemented.
- Release posture changes are not implemented.

## 4. Behavior Added

Generated prompt files now include:

```text
Agent executes. Workflow OS governs.
```

Successful scaffold runs now print:

```text
next_step: paste .workflow-os/agent-harness-prompt.md into your coding agent
```

The command still writes only `AGENTS.md` and `.workflow-os/agent-harness-prompt.md`.

## 5. Safety And Runtime Boundary

The cleanup did not add executor calls, state backend usage, workflow validation, workflow execution, approvals, event appends, local check execution, local check registration, report generation, report artifact writes, provider calls, or schema changes.

Unmanaged prompt-file content now has explicit regression coverage. The error remains stable and does not echo secret-like prompt contents.

## 6. Test Coverage Summary

Added or strengthened tests for:

- generated prompt includes the slogan;
- successful CLI output points to the prompt file;
- unmanaged prompt files fail closed without `--force`;
- unmanaged prompt file contents are not leaked in errors;
- unmanaged prompt file contents are not overwritten on failure.

Existing scaffold tests still cover creation, unmanaged `AGENTS.md`, `--force`, managed-block updates, `--dry-run`, no runtime state, invalid agent handling, and help text.

## 7. Commands Run And Results

Validation run:

- `cargo test -p workflow-cli --test cli init_agent_harness` - passed.
- `cargo fmt --all --check` - passed after applying rustfmt.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 8. Remaining Known Limitations

- `--dry-run` output remains safe but sparse.
- `--agent codex` and `--agent claude` still only change the generated audience label.
- Generated scaffold text remains embedded in Rust string literals.

## 9. Recommended Next Phase

Recommended next phase: scaffold wording cleanup review.

After review, return to roadmap kernel implementation unless field testing justifies a separately planned agent-specific prompt variant phase.
