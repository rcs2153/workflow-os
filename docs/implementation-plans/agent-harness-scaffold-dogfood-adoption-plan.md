# Agent Harness Scaffold Dogfood And Adoption Plan

Status: Planned. This phase dogfoods and reviews the implemented `workflow-os init-agent-harness` scaffold command in clean local projects. It does not implement runtime harness auto-generation, workflow execution, automatic local checks, handler registration, persistence, report artifacts, CLI report rendering, schemas, examples, reasoning lineage, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

The agent harness onboarding and CLI scaffold phases are implemented. Users can now run:

```sh
workflow-os init-agent-harness
```

to generate `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` with the intended local adoption posture:

```text
Agent executes. Workflow OS governs.
```

The next question is whether the generated scaffold is clear, safe, and effective when used in a clean local project. This plan defines a dogfood and adoption review phase. It does not add runtime automation or broaden Workflow OS authority.

## 2. Goals

- Dogfood `workflow-os init-agent-harness` in a clean temporary local project.
- Verify the generated files make the kernel-governed agent setup obvious.
- Verify generated content does not overclaim runtime behavior.
- Verify unmanaged-file and managed-block behavior remains safe in realistic local use.
- Verify `--dry-run`, `--force`, `--output-dir`, and `--agent` behavior from a user perspective.
- Identify wording gaps that make users fall back to hand-writing YAML instead of using the agent harness posture.
- Produce a bounded adoption review report with concrete follow-ups.

## 3. Non-Goals

Do not implement:

- runtime harness auto-generation;
- automatic runtime report generation;
- workflow execution from the scaffold command;
- approval decisions from the scaffold command;
- automatic local check execution;
- local check handler registration;
- workflow schema fields;
- workflow-declared agent harnesses;
- persistence or report artifacts;
- CLI report rendering;
- example integration updates;
- reasoning lineage;
- side-effect boundary modeling;
- write behavior;
- hosted or distributed runtime behavior;
- recursive agents;
- agent swarms;
- self-governing agents;
- Level 3 or Level 4 autonomy enablement;
- release posture changes.

## 4. Dogfood Setup

Use a clean temporary project directory outside the repository, such as:

```sh
/private/tmp/workflow-os-agent-harness-dogfood
```

The dogfood should use the local built CLI binary or cargo invocation. It should not require publishing, installing global packages, network access, provider credentials, live adapters, or a production state backend.

The setup should cover:

- an empty directory with no existing scaffold files;
- a directory with an unmanaged `AGENTS.md`;
- a directory with managed Workflow OS scaffold blocks;
- a directory using `--output-dir`;
- one run each for `--agent generic`, `--agent codex`, and `--agent claude`;
- `--dry-run` behavior.

## 5. Review Questions

The dogfood review should answer:

- Does the command make the next user action obvious?
- Does `AGENTS.md` explain the operating model without becoming too long?
- Does `.workflow-os/agent-harness-prompt.md` work as a copy/paste prompt?
- Does the generated content keep the agent from inventing state, approvals, evidence, reports, validation results, or command output?
- Does the generated content avoid implying automatic local checks, runtime harness execution, writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy?
- Does the command fail closed in a way a new user can understand?
- Does `--dry-run` provide enough information for cautious users?
- Does `--force` feel appropriately explicit and risky?
- Do the agent labels help adoption, or are real prompt variants needed?
- Does the quickstart still match the generated scaffold?

## 6. Acceptance Criteria

The phase is successful if:

- clean scaffold generation succeeds;
- generated files contain the intended slogan and governance posture;
- generated files include validation, approval, scope-boundary, and unsupported-capability language;
- generated files avoid recursive-agent and agent-swarm framing;
- unmanaged existing files fail closed without `--force`;
- managed-block update preserves surrounding content;
- `--dry-run` writes no files;
- `--force` replaces unmanaged files only when explicitly requested;
- the command does not create runtime state, run workflows, approve checkpoints, execute checks, register handlers, or write report artifacts;
- docs accurately describe the command as scaffold-only;
- an adoption review report captures findings and next recommended phase.

## 7. Test And Check Plan

The dogfood phase should run:

- `workflow-os init-agent-harness` in a clean temporary directory;
- `workflow-os init-agent-harness --dry-run` in a clean temporary directory;
- `workflow-os init-agent-harness --force` against unmanaged scaffold files;
- `workflow-os init-agent-harness --agent codex`;
- `workflow-os init-agent-harness --agent claude`;
- `workflow-os init-agent-harness --output-dir <path>`;
- `cargo test -p workflow-cli --test cli init_agent_harness`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

If the dogfood phase makes documentation-only wording updates, `npm run check:docs` is the required minimum validation. If it changes CLI code or tests, run the full Rust validation set.

## 8. Privacy And Redaction

The dogfood phase must not copy secrets, environment values, provider payloads, raw command output, raw specs, local absolute paths, or private repository metadata into generated docs or reports.

Temporary dogfood paths may be mentioned in command logs or the review report only as bounded local test paths, not as user-specific proof of production behavior.

Errors should be checked for non-leakage of unmanaged file contents and rejected flag values.

## 9. Documentation Outputs

Create:

- `docs/concepts/AGENT_HARNESS_SCAFFOLD_DOGFOOD_ADOPTION_REPORT.md`

The report should include:

1. executive summary;
2. scope completed;
3. scope explicitly not completed;
4. dogfood environment;
5. commands run and results;
6. generated file assessment;
7. overwrite and dry-run assessment;
8. adoption clarity assessment;
9. runtime boundary assessment;
10. privacy/redaction assessment;
11. discovered issues;
12. recommended next phase.

Update documentation only if dogfood reveals unclear or inaccurate wording. Do not use this phase to add new runtime behavior.

## 10. Recommended Next Phase Options

The dogfood report should choose one:

- scaffold adoption wording cleanup;
- prompt-file overwrite regression test;
- agent-specific prompt variant planning;
- broader onboarding automation planning;
- return to roadmap kernel implementation;
- defer.

## 11. Final Recommendation

Execute the dogfood and adoption review next. Keep it bounded to local scaffold behavior and user clarity. Do not implement runtime harness generation, automatic workflow execution, local check automation, schemas, writes, or hosted behavior.
