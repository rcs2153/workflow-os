# Agent Harness CLI Scaffold Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

`workflow-os init-agent-harness` is implemented as a narrow documentation/scaffold command. It creates or updates local agent instruction files, preserves the "Agent executes. Workflow OS governs." posture, and does not expand runtime authority.

## 2. Scope Verification

The phase stayed within the approved scaffold-only scope.

Confirmed not introduced:

- runtime harness auto-generation;
- automatic runtime report generation;
- workflow execution from the scaffold command;
- approval decisions from the scaffold command;
- local check execution;
- local check handler registration;
- report artifact generation;
- persistence or StateBackend writes;
- CLI report rendering;
- example updates;
- workflow spec schema changes;
- reasoning lineage;
- side-effect boundary modeling;
- write behavior;
- hosted or distributed runtime behavior;
- recursive-agent or agent-swarm positioning;
- Level 3/4 autonomy enablement;
- release posture changes.

The command writes only `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` under the requested scaffold output directory.

## 3. CLI API Assessment

Implemented command:

```sh
workflow-os init-agent-harness [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]
```

The API is appropriately small and follows the existing hand-rolled CLI parser style. `--project-dir` remains the inherited default target root, while `--output-dir` provides explicit override behavior. `--agent` changes only the generated audience label and does not imply runtime capability differences.

The command prints bounded status lines only. It does not produce CLI report output or runtime summaries.

## 4. Generated File Assessment

The command generates the two approved files:

- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

Generated content includes:

- `Agent executes. Workflow OS governs.`
- engineering-standard and phase-document reading posture;
- validation and governed workflow posture;
- approval checkpoint language;
- scope-boundary language;
- non-invention rules for workflow state, approvals, evidence, audit events, work reports, validation results, and command outputs;
- unsupported capability warnings;
- quickstart pointer.

Generated content avoids claims that Workflow OS executes coding agents directly, enables automatic local checks, supports writes, provides hosted/distributed runtime behavior, or enables recursive agents, agent swarms, or Level 3/4 autonomy.

## 5. File Safety And Overwrite Assessment

The implementation uses the approved managed markers:

```text
<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
<!-- END WORKFLOW OS AGENT HARNESS -->
```

Confirmed behavior:

- absent target files are created;
- existing files with a managed block are updated only within that block;
- surrounding user content is preserved;
- existing unmanaged files fail closed unless `--force` is supplied;
- `--force` replaces the generated target content;
- `--dry-run` writes no files.

This is conservative enough for a first scaffold slice.

## 6. Runtime And Authority Boundary Assessment

The command is isolated from runtime execution paths.

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

Tests also verify that a scaffold invocation does not create `.workflow-os/state`.

## 7. Privacy And Redaction Assessment

The generated content is static and does not copy project files, spec contents, provider payloads, command output, environment values, credentials, tokens, private repository metadata, or local absolute paths.

Error behavior is appropriately bounded:

- unmanaged-file errors name only the logical scaffold target;
- read/write/create-dir errors do not echo file content;
- invalid `--agent` errors do not echo the rejected value;
- tests cover secret-like unmanaged content and invalid agent values.

## 8. Test Quality Assessment

Focused tests cover:

- scaffold file creation;
- generated slogan and governance language;
- unsupported-capability warnings;
- recursive-agent and agent-swarm language avoidance;
- unmanaged `AGENTS.md` fail-closed behavior;
- `--force` replacement without leaking existing content;
- managed-block update preserving surrounding content;
- `--dry-run` writing no files;
- scaffold-only behavior with no runtime state directory;
- invalid agent rejection without leaking the rejected value;
- CLI help visibility.

Existing workspace tests continue to cover CLI validation, execution, approval, inspection, doctor, local check, WorkReport, EvidenceReference, Diagnostic, adapter telemetry, and runtime behavior.

One planned test is only indirectly covered: there is no explicit assertion that `.workflow-os/agent-harness-prompt.md` unmanaged content fails closed independently from `AGENTS.md`. The shared helper and creation tests make this low risk, but a prompt-file-specific overwrite test would improve coverage.

## 9. Documentation Review

Documentation now states:

- `workflow-os init-agent-harness` is implemented;
- it generates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md`;
- it is documentation/scaffold-only;
- it does not run workflows;
- it does not approve checkpoints;
- it does not execute local checks;
- it does not register handlers;
- it does not create report artifacts;
- it does not enable writes, hosted execution, recursive agents, agent swarms, or Level 3/4 autonomy.

Docs updated or added:

- README;
- ROADMAP;
- CLI overview and command reference;
- user guide index;
- agent harness quickstart;
- governed work pattern;
- evidence reference concept;
- scaffold plan;
- implementation report.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a prompt-file-specific unmanaged overwrite test for `.workflow-os/agent-harness-prompt.md`.
- Consider moving the generated scaffold text out of Rust string literals if the content grows materially.
- Consider making `--dry-run` report whether files would be created, replaced, or managed-block-updated once users start relying on it.
- Consider adding a future `--agent claude` content variant beyond the current audience label if field testing shows that agent-specific wording improves adoption.

## 12. Recommended Next Phase

Recommended next phase: agent harness scaffold dogfood and adoption review.

The scaffold command is accepted; the next useful phase is to dogfood the generated files in a clean temporary project and collect whether the command makes the kernel-governed agent setup feel obvious without implying runtime automation. That should remain documentation/adoption work unless a separate plan approves runtime behavior.

## 13. Validation

Review validation run:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

No broader checks were required for this scaffold-only review.
