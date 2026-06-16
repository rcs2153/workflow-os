# Agent Harness CLI Scaffold Plan

Status: Implemented as a documentation/scaffold-only CLI command. `workflow-os init-agent-harness` creates or updates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` with managed Workflow OS scaffold blocks. Runtime harness auto-generation, workflow schema fields, automatic local check execution, writes, hosted behavior, recursive agents, agent swarms, side-effect modeling, and release posture changes are not implemented.

## 1. Executive Summary

The Agent Harness Onboarding phase made the desired local adoption loop explicit:

```text
Agent executes. Workflow OS governs.
```

Users can now read the quickstart and manually use `AGENTS.md` plus a copy/paste prompt to connect Codex, Claude Code, or another coding agent to the Workflow OS kernel as the governing layer.

The next question was how to make this feel automatic without silently increasing authority. This plan defined a documentation/scaffold-only CLI command, `workflow-os init-agent-harness`, that creates or updates local agent instruction files and a session prompt. The command is now implemented, and it remains scaffold-only.

## 2. Goals

- Make kernel-governed agent setup stupid simple for local users.
- Generate or update safe local agent instruction artifacts.
- Preserve the current explicit governance boundary.
- Make the generated files useful for Codex, Claude Code, and similar coding agents.
- Keep generated content documentation-only and non-executing.
- Avoid automatic local check execution or default command handler registration.
- Avoid workflow schema changes.
- Preserve manual YAML/project authoring for users who want direct control.
- Prepare a small implementation prompt for a future CLI scaffold command.

## 3. Non-Goals

Do not implement:

- the CLI command in this planning phase;
- runtime harness generation;
- automatic runtime report generation;
- automatic local check execution;
- default local command handler registration;
- workflow schema fields;
- workflow-declared agent harnesses;
- persistence or report artifacts;
- command-output evidence;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary modeling;
- writes;
- hosted or distributed runtime behavior;
- recursive agents;
- agent swarms;
- self-governing agents;
- Level 3 or Level 4 autonomy enablement;
- release posture changes.

## 4. Proposed Command Shape

Future command:

```sh
workflow-os init-agent-harness
```

Candidate options:

- `--project-dir <path>`: existing CLI project directory convention.
- `--output-dir <path>`: optional target directory, defaulting to project root or current directory.
- `--agent codex|claude|generic`: optional flavor, default `generic`.
- `--force`: overwrite generated files when safe.
- `--dry-run`: print planned file changes without writing.

The first implementation should prefer the smallest safe set:

```sh
workflow-os init-agent-harness --project-dir .
```

If CLI conventions make `--project-dir` awkward for a scaffold command, the implementation may use the repository root/current directory after documenting the choice.

## 5. Generated Files

Recommended first generated files:

- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

Optional future files:

- `.workflow-os/agent-harness/README.md`
- `.workflow-os/agent-harness/codex-prompt.md`
- `.workflow-os/agent-harness/claude-code-prompt.md`

The first implementation should generate only the minimal files needed to make setup obvious. `AGENTS.md` plus one prompt file is enough.

## 6. Generated AGENTS.md Policy

Generated `AGENTS.md` should include:

- `Agent executes. Workflow OS governs.`
- required engineering-standard and roadmap reading posture;
- validation-before-work instructions;
- governed workflow start/resume instructions;
- mandatory approval checkpoint language;
- scope-boundary language;
- non-invention rules for workflow state, approvals, evidence, audit events, reports, validation results, and command outputs;
- current unsupported capability list;
- pointer to `docs/user-guide/agent-harness-quickstart.md` if present.

Generated `AGENTS.md` must not:

- claim Workflow OS executes coding agents directly;
- claim automatic local check execution;
- claim write support;
- claim hosted/distributed runtime behavior;
- claim recursive agents, agent swarms, or production nested harness execution;
- claim Level 3/4 autonomy.

## 7. Generated Prompt Policy

Generated `.workflow-os/agent-harness-prompt.md` should contain a copy/paste session prompt.

It should tell the agent to:

- use Workflow OS as the governing layer;
- validate before making changes;
- start or resume the appropriate governed workflow when required;
- treat approvals as mandatory;
- stay inside approved phase scope;
- run required validation commands;
- report completed scope, deferred scope, validation results, and next phase.

It must tell the agent not to:

- bypass validation, policy, approvals, or failed checks;
- invent workflow state, approvals, evidence, audit events, reports, validation results, or command outputs;
- mutate workflow state files by hand;
- replace deterministic governance with model self-review;
- claim unsupported capabilities.

## 8. File Safety And Overwrite Behavior

The scaffold command must be conservative with existing user files.

Recommended behavior:

- If `AGENTS.md` does not exist, create it.
- If `AGENTS.md` exists and does not contain a recognizable Workflow OS managed block, fail closed unless `--force` is provided.
- If `AGENTS.md` contains a recognizable Workflow OS managed block, update only that block.
- If `.workflow-os/agent-harness-prompt.md` exists, fail closed unless it has a recognizable generated header or `--force` is provided.
- Create `.workflow-os/` only for documentation scaffold files, not runtime state.

Generated files should include a short marker such as:

```text
<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
...
<!-- END WORKFLOW OS AGENT HARNESS -->
```

This keeps later updates bounded and reviewable.

## 9. Runtime And Authority Boundary

The scaffold command must not:

- validate and run workflows automatically;
- approve workflow checkpoints;
- register local check handlers;
- execute local checks;
- create or mutate workflow runs;
- write report artifacts;
- alter state backends;
- call external systems;
- add workflow schema fields;
- change policy behavior.

It only writes documentation/instruction files.

## 10. Privacy And Redaction

Generated content must not include:

- local absolute paths unless explicitly supplied and safe;
- environment variable values;
- tokens or credentials;
- provider payloads;
- raw command output;
- raw spec contents;
- private repository metadata.

Errors must use stable codes and must not echo secret-like paths, prompt contents, tokens, or file contents.

## 11. Relationship To Existing Docs

The command should draw language from:

- `AGENTS.md`;
- `docs/user-guide/agent-harness-quickstart.md`;
- `docs/concepts/governed-work-pattern.md`;
- `dogfood/workflow-os-self-governance/README.md`.

The implementation should avoid duplicating large blocks manually across Rust string literals if a small template helper or included template file is more maintainable. Any template file should be treated as product documentation and covered by docs checks where practical.

## 12. Test Plan For Future Implementation

Future implementation should add tests for:

- command creates `AGENTS.md` when absent;
- command creates `.workflow-os/agent-harness-prompt.md` when absent;
- generated content includes `Agent executes. Workflow OS governs.`;
- generated content includes validation, approval, and scope-boundary instructions;
- generated content includes unsupported capability warnings;
- generated content avoids recursive-agent and agent-swarm framing;
- existing unmanaged `AGENTS.md` fails closed without `--force`;
- managed block update preserves user content outside the block;
- `--dry-run` writes no files;
- command does not validate, run, approve, or inspect workflows;
- command does not register local check handlers;
- command does not create report artifacts;
- command does not touch `StateBackend`;
- command emits no secret-like paths or prompt contents in errors;
- CLI help documents the command as documentation/scaffold-only;
- existing CLI tests still pass;
- `npm run check:docs` passes.

If the implementation changes Rust CLI code, also run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`.

## 13. Documentation Updates For Future Implementation

Future implementation should update:

- `README.md`;
- `docs/user-guide/agent-harness-quickstart.md`;
- `docs/user-guide/README.md`;
- `docs/cli/overview.md` or the closest CLI command reference;
- `ROADMAP.md`;
- `docs/concepts/governed-work-pattern.md`.

Docs must state:

- `workflow-os init-agent-harness` is documentation/scaffold-only;
- it generates agent instructions and a session prompt;
- it does not run workflows;
- it does not approve checkpoints;
- it does not execute local checks;
- it does not register handlers;
- it does not create report artifacts;
- it does not enable writes, hosted execution, recursive agents, agent swarms, or Level 3/4 autonomy.

## 14. Open Questions

- Should the first implementation support `--agent codex|claude|generic`, or should one generic prompt be used until user testing demands variants?
- Should generated files live at repository root or under project root when those differ?
- Should `.workflow-os/` be used for documentation scaffold files before any runtime config exists?
- Should `workflow-os validate` eventually print a non-default hint pointing to the quickstart?
- Should the scaffold command be available before side-effect boundary planning is accepted, given that it is documentation-only?
- How should managed blocks interact with existing user-maintained `AGENTS.md` files?

## 15. Acceptance Criteria For Future Implementation

- CLI scaffold command exists.
- Generated files are documentation/instruction only.
- Existing unmanaged files are not overwritten silently.
- Generated content is safe, bounded, and product-boundary accurate.
- No workflow runs, approvals, local checks, report artifacts, state backend writes, schemas, side effects, writes, hosted behavior, recursive agents, or release posture changes are introduced.
- Tests cover create, update, dry-run, overwrite protection, and non-execution boundaries.
- Documentation remains honest.

## 16. Final Recommendation

The next implementation phase should be: **agent harness CLI scaffold implementation, docs/template-only**.

It should generate `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` from conservative templates, with safe overwrite behavior and no runtime side effects. Do not add automatic execution, workflow schema fields, local check registration, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.
