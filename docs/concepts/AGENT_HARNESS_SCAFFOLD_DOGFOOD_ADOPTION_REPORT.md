# Agent Harness Scaffold Dogfood And Adoption Report

## 1. Executive Summary

The `workflow-os init-agent-harness` scaffold command was dogfooded in clean temporary local directories. The command successfully created and updated the expected documentation scaffold files without creating runtime state, running workflows, approving checkpoints, executing local checks, registering handlers, or writing report artifacts.

The generated `AGENTS.md` clearly communicates the intended adoption model:

```text
Agent executes. Workflow OS governs.
```

The generated prompt is usable as a copy/paste session prompt, but the dogfood pass found one adoption clarity follow-up: the prompt file should probably repeat the exact slogan so users see the same mental model in both generated artifacts.

## 2. Scope Completed

- Ran `workflow-os init-agent-harness` in a clean temporary directory.
- Ran `workflow-os init-agent-harness --dry-run` in a clean temporary directory.
- Ran unmanaged-file fail-closed behavior.
- Ran unmanaged-file replacement with `--force`.
- Ran managed-block update behavior.
- Ran `--agent codex`.
- Ran `--agent claude`.
- Ran `--output-dir`.
- Checked generated files for expected governance language and unsupported-capability language.
- Checked that runtime state was not created.
- Checked that unmanaged secret-like file contents were not echoed by scaffold errors.
- Produced this adoption report.

## 3. Scope Explicitly Not Completed

- Runtime harness auto-generation is not implemented.
- Workflow execution from the scaffold command is not implemented.
- Approval decisions from the scaffold command are not implemented.
- Automatic local check execution is not implemented.
- Local check handler registration is not implemented.
- Workflow schema fields are not implemented.
- Workflow-declared agent harnesses are not implemented.
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

## 4. Dogfood Environment

Dogfood used local temporary directories under `/private/tmp` and the local `target/debug/workflow-os` binary.

Temporary directories exercised:

- clean scaffold directory;
- dry-run directory;
- unmanaged-file directory;
- managed-block directory;
- output-dir directory.

No network access, provider credentials, live adapters, production state backend, or global install path was required.

## 5. Commands Run And Results

- `target/debug/workflow-os --project-dir /private/tmp/workflow-os-agent-harness-dogfood.rpCY8e init-agent-harness` - passed.
- `target/debug/workflow-os --project-dir /private/tmp/workflow-os-agent-harness-dryrun.4kFNB0 init-agent-harness --dry-run` - passed and wrote no files.
- `target/debug/workflow-os --project-dir /private/tmp/workflow-os-agent-harness-unmanaged.o1bm8m init-agent-harness` - failed closed with `cli.init_agent_harness.unmanaged_file`.
- `target/debug/workflow-os --project-dir /private/tmp/workflow-os-agent-harness-unmanaged.o1bm8m init-agent-harness --force` - passed and replaced unmanaged scaffold targets.
- `target/debug/workflow-os --project-dir /private/tmp/workflow-os-agent-harness-managed.Eqoa3b init-agent-harness --agent codex` - passed and preserved surrounding managed-block content.
- `target/debug/workflow-os --project-dir /private/tmp/workflow-os-agent-harness-output.CNqdoX init-agent-harness --output-dir /private/tmp/workflow-os-agent-harness-output.CNqdoX/nested-target --agent claude` - passed and wrote only under the requested output directory.
- `rg` content checks confirmed expected slogan, agent labels, and unsupported-capability warnings.
- Runtime-state absence checks confirmed `.workflow-os/state` was not created by scaffold runs.

Validation after this report:

- `cargo test -p workflow-cli --test cli init_agent_harness` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 6. Generated File Assessment

`AGENTS.md` is short enough to be readable and direct enough to steer an agent. It includes:

- the slogan;
- engineering-standard and phase-doc reading posture;
- validation posture;
- governed workflow start/resume posture;
- approval checkpoint language;
- scope-boundary language;
- non-invention rules;
- unsupported capability warnings;
- quickstart pointer.

`.workflow-os/agent-harness-prompt.md` works as a copy/paste prompt. It includes:

- use Workflow OS as the governing layer;
- read engineering standard and active phase docs;
- validate when required;
- use governed workflow as source of truth for scope, approvals, checks, and reports;
- do not bypass validation, policy, approvals, or failed checks;
- do not mutate Workflow OS state by hand;
- do not replace deterministic governance with model self-review;
- report completed scope, deferred scope, validation commands, limitations, and next phase.

The prompt does not currently repeat the exact slogan. That is not a blocker, but adding it would improve adoption consistency.

## 7. Overwrite And Dry-Run Assessment

Unmanaged `AGENTS.md` fails closed without `--force`, and the error does not echo file contents.

`--force` replaces unmanaged content only when explicitly supplied.

Managed-block updates preserve surrounding user content and replace only the Workflow OS managed block.

`--dry-run` writes no files and identifies the two planned scaffold targets. It is safe, but it could be more useful if it later reported whether each file would be created, replaced, or managed-block-updated.

## 8. Adoption Clarity Assessment

The command makes setup materially simpler than telling users to manually copy the root `AGENTS.md` and quickstart prompt. The generated files make it clear that the coding agent performs repository work while Workflow OS governs validation, approvals, state, and reporting posture.

The generated artifacts avoid the bad frame of recursive agents, agent swarms, self-governing agents, or magic orchestration.

The strongest adoption improvement would be a small wording cleanup that repeats the slogan in the prompt file and adds one or two more explicit next-step pointers after scaffold generation, such as reading the prompt file or pasting it into Codex/Claude Code.

## 9. Runtime Boundary Assessment

The dogfood phase confirmed scaffold invocations do not create `.workflow-os/state` and do not emit runtime identifiers, approval IDs, workflow status, report artifact output, or CLI report rendering.

The command remains documentation-only:

- no `LocalExecutor` behavior;
- no StateBackend writes;
- no workflow validation or execution;
- no approval decision;
- no event append;
- no local check execution;
- no local check handler registration;
- no report generation;
- no report artifact write;
- no external provider call.

## 10. Privacy And Redaction Assessment

Generated content did not include secrets, environment values, provider payloads, raw command output, raw specs, private repository metadata, or local absolute paths.

The unmanaged-file test used secret-like file content. The fail-closed error did not echo that content.

Temporary local paths are included in this report only as bounded dogfood command context. They are not product behavior or production evidence.

## 11. Discovered Issues

No blockers.

Non-blocking issues:

- The generated prompt file does not repeat `Agent executes. Workflow OS governs.`
- `--dry-run` output is safe but sparse; it does not distinguish create, replace, or managed-block update.
- `--agent codex` and `--agent claude` currently change only the audience label. That is acceptable for the first slice, but field testing may justify true prompt variants.
- The implementation review already noted missing prompt-file-specific unmanaged overwrite test coverage.

Fix-forward note: the slogan and prompt-file-specific unmanaged overwrite coverage are addressed in [Agent Harness Scaffold Wording Cleanup Report](AGENT_HARNESS_SCAFFOLD_WORDING_CLEANUP_REPORT.md). The `--dry-run` detail and agent-specific prompt variant follow-ups remain deferred.

## 12. Recommended Next Phase

Recommended next phase: scaffold adoption wording cleanup.

Keep the next phase small and documentation/scaffold-only:

- add the exact slogan to `.workflow-os/agent-harness-prompt.md` generation;
- consider a slightly clearer post-command output line telling users where to paste the prompt;
- add prompt-file-specific unmanaged overwrite regression coverage;
- keep runtime harness generation, automatic workflow execution, local check automation, schemas, writes, hosted behavior, recursive agents, agent swarms, and release posture changes out of scope.
