# Agent Harness Scaffold Wording Cleanup Review

## 1. Executive Verdict

Phase accepted; return to roadmap kernel implementation.

The wording cleanup fixed the adoption clarity issue found during dogfood: the generated prompt now repeats the exact mental model, `Agent executes. Workflow OS governs.` The phase also added prompt-file-specific fail-closed regression coverage without broadening runtime behavior.

## 2. Scope Verification

The phase stayed within the approved documentation/scaffold-only cleanup scope.

Confirmed not introduced:

- runtime harness auto-generation;
- workflow execution from the scaffold command;
- approval decisions from the scaffold command;
- automatic local check execution;
- local check handler registration;
- workflow schema fields;
- persistence or report artifacts;
- CLI report rendering;
- example integration updates;
- reasoning lineage;
- side-effect boundary modeling;
- write behavior;
- hosted or distributed runtime behavior;
- recursive agents;
- agent swarms;
- Level 3 or Level 4 autonomy enablement;
- release posture changes.

## 3. Cleanup Assessment

The generated `.workflow-os/agent-harness-prompt.md` now includes:

```text
Agent executes. Workflow OS governs.
```

This aligns the prompt file with generated `AGENTS.md`, the quickstart, and the roadmap adoption language.

Successful scaffold runs now print:

```text
next_step: paste .workflow-os/agent-harness-prompt.md into your coding agent
```

This is a bounded next-step hint and does not imply runtime execution, hosted behavior, local check automation, or higher autonomy.

## 4. Runtime Boundary Assessment

The implementation remains isolated to scaffold file content and CLI status output.

Confirmed:

- no `LocalExecutor` call;
- no `LocalStateBackend` creation;
- no workflow validation or execution;
- no approval decision;
- no run creation or event append;
- no local check handler registration;
- no local check execution;
- no report generation;
- no report artifact write;
- no external provider call.

## 5. File Safety Assessment

Existing managed-block behavior remains intact. The cleanup did not change marker strings or replacement semantics.

The new prompt-file-specific test verifies that an unmanaged `.workflow-os/agent-harness-prompt.md` fails closed without `--force`, does not leak secret-like prompt contents, and is not overwritten on failure.

## 6. Privacy And Redaction Assessment

The new generated prompt line is static and contains no path, environment value, provider payload, command output, spec content, token, credential, or private repository metadata.

The new success output line is static and relative-path-only. It does not echo user-supplied paths or file contents.

The new regression test confirms secret-like unmanaged prompt contents are not echoed in the error path.

## 7. Test Quality Assessment

Test coverage is now stronger for:

- generated prompt includes the slogan;
- successful output points users at the generated prompt file;
- unmanaged prompt file fails closed without `--force`;
- unmanaged prompt contents are not leaked in errors;
- unmanaged prompt contents are not overwritten on failure.

Existing scaffold tests still cover clean creation, unmanaged `AGENTS.md`, `--force`, managed-block update, `--dry-run`, no runtime state creation, invalid agent rejection, and help text.

No blocker-level test gaps remain for this phase.

## 8. Documentation Review

The wording cleanup report honestly states:

- what changed;
- what remains scaffold-only;
- what was not implemented;
- validation results;
- remaining known limitations.

The dogfood report keeps its original finding and adds a fix-forward note instead of erasing history.

## 9. Blockers

No blockers.

## 10. Non-Blocking Follow-Ups

- `--dry-run` output remains safe but sparse; future UX work could distinguish create, replace, and managed-block update.
- `--agent codex` and `--agent claude` still only change the generated audience label; field testing may justify true prompt variants later.
- Generated scaffold text remains embedded in Rust string literals; extracting templates can wait until content growth makes that worthwhile.

## 11. Recommended Next Phase

Recommended next phase: return to roadmap kernel implementation.

The P0 adoption scaffold is now implemented, dogfooded, cleaned up, and reviewed. Further adoption polish should be driven by field feedback. The project should return to kernel roadmap execution unless the user explicitly prioritizes another adoption slice.

## 12. Validation

Review validation run:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

No broader checks were required for this scaffold-only review.
