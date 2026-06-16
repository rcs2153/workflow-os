# Agent Harness CLI Scaffold Plan Review

Review date: 2026-06-16

## 1. Executive Verdict

Plan accepted; proceed to agent harness CLI scaffold implementation.

The plan defines a narrow documentation/scaffold-only command path for `workflow-os init-agent-harness` and preserves the current Workflow OS product boundary. It does not authorize runtime automation, automatic local check execution, workflow schema fields, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Scope Verification

The plan stayed within planning-only scope.

Planned scope:

- future CLI scaffold command shape;
- generated file targets;
- generated `AGENTS.md` policy;
- generated session prompt policy;
- conservative overwrite behavior;
- runtime/authority non-goals;
- privacy/redaction constraints;
- future test plan and documentation updates.

No accidental implementation or authorization was found:

- no CLI command implemented;
- no runtime harness generation;
- no workflow schema fields;
- no automatic local check execution;
- no default local command handler registration;
- no report artifact writing;
- no persistence;
- no command-output evidence;
- no approval evidence attachment;
- no reasoning lineage;
- no side-effect boundary modeling;
- no writes;
- no hosted/distributed runtime behavior;
- no recursive agents or agent swarms;
- no Level 3/4 autonomy enablement;
- no release posture change.

## 3. Command Shape Assessment

The proposed command shape is appropriate:

```sh
workflow-os init-agent-harness
```

Candidate options are reasonable:

- `--project-dir <path>`;
- `--output-dir <path>`;
- `--agent codex|claude|generic`;
- `--force`;
- `--dry-run`.

The plan correctly recommends that the first implementation prefer the smallest safe surface. A generic prompt plus explicit project/output behavior is enough for the first implementation.

## 4. Generated File Assessment

The recommended first generated files are appropriate:

- `AGENTS.md`;
- `.workflow-os/agent-harness-prompt.md`.

This gives users the “magic setup” feel while keeping the output inspectable and reviewable. The plan correctly treats `.workflow-os/` as documentation scaffold storage only, not runtime state or hidden authority.

## 5. File Safety Assessment

The overwrite policy is conservative and suitable for implementation.

Accepted safety requirements:

- create missing files;
- fail closed on unmanaged existing files unless `--force` is provided;
- update only recognizable Workflow OS managed blocks;
- preserve user content outside managed blocks;
- support `--dry-run` without writes.

The managed block marker is a good implementation constraint because it limits future updates to a bounded region.

## 6. Runtime And Authority Boundary Assessment

The plan keeps the scaffold command non-executing.

It explicitly forbids the command from:

- validating and running workflows automatically;
- approving checkpoints;
- registering local check handlers;
- executing local checks;
- creating or mutating workflow runs;
- writing report artifacts;
- altering state backends;
- calling external systems;
- adding workflow schema fields;
- changing policy behavior.

This is the right boundary. The command should make onboarding easier, not silently make the kernel more autonomous.

## 7. Product Boundary Assessment

The plan preserves the intended product framing:

```text
Agent executes. Workflow OS governs.
```

Generated content is required to avoid claims that Workflow OS executes coding agents directly, runs checks automatically, supports writes, provides hosted/distributed runtime behavior, implements recursive agents or agent swarms, or enables Level 3/4 autonomy.

The plan also avoids confusing agent harness onboarding with Composable Harness Contracts or nested harness runtime execution.

## 8. Privacy And Redaction Assessment

The privacy posture is adequate for a scaffold command.

The plan prohibits generated content and errors from including:

- unsafe local absolute paths;
- environment variable values;
- tokens or credentials;
- provider payloads;
- raw command output;
- raw spec contents;
- private repository metadata;
- secret-like prompt contents.

Future implementation should preserve stable non-leaking error codes for overwrite and file-write failures.

## 9. Test Plan Assessment

The future test plan is strong and phase-ready.

It covers:

- file creation;
- managed-block update behavior;
- overwrite protection;
- dry-run no-write behavior;
- generated content assertions;
- unsupported capability warnings;
- avoidance of recursive-agent and agent-swarm framing;
- non-execution boundaries;
- non-leaking errors;
- CLI help documentation;
- existing CLI regression tests;
- docs checks.

One useful implementation detail: tests should use temporary directories and avoid depending on the repository's checked-in `AGENTS.md`.

## 10. Documentation Review

Documentation links are coherent:

- onboarding plan points to scaffold planning;
- quickstart points to scaffold planning while stating the command is unimplemented;
- roadmap marks scaffold work as planned, not implemented;
- plan report records planning-only scope.

The docs do not overclaim current capabilities.

## 11. Planning Blockers

No planning blockers.

## 12. Non-Blocking Follow-Ups

- Decide during implementation whether `--agent codex|claude|generic` should ship immediately or remain future.
- Decide whether `.workflow-os/agent-harness-prompt.md` should be the first prompt path or whether `.workflow-os/agent-harness/prompt.md` is cleaner.
- Ensure implementation tests cover existing unmanaged `AGENTS.md` behavior in temporary directories, not only the current repository state.
- Consider a later, separate plan for a non-default `workflow-os validate` hint after the scaffold command is implemented and reviewed.

## 13. Recommended Next Phase

Recommended next phase: agent harness CLI scaffold implementation.

The implementation should add the smallest documentation/scaffold-only command that can create or update `AGENTS.md` and a prompt file with safe managed-block behavior. It must not add runtime automation, automatic local check execution, workflow schema fields, report artifacts, state mutations, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 14. Validation

Validation commands for this review:

- `npm run check:docs`
  - Passed.
