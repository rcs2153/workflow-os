# Real-Repo Onboarding UX Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation addresses the two approved P0 onboarding UX slices:

- existing `AGENTS.md` guidance is preserved by default when Workflow OS scaffolds into an existing repository;
- `workflow-os first-run` now uses bounded safe repository metadata to make package/TypeScript recommendations more concrete without executing commands or inspecting source contents.

The phase stayed within local onboarding UX scope. It did not introduce automatic workflow generation, source-content inspection, local command execution, provider calls, external writes, schema changes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Scope Verification

The phase remained focused on existing-repository onboarding.

Implemented:

- `init-repo-governance` preserves unmanaged `AGENTS.md` content by default.
- `init-agent-harness` preserves unmanaged `AGENTS.md` content by default.
- Existing Workflow OS managed blocks are updated in place.
- Explicit `--force` replacement remains available and is disclosed.
- Dry-run output reports preservation/update/replacement posture without echoing existing file contents.
- `first-run` detects safe package/repository metadata using bounded file presence, selected manifest keys, allowlisted script names, TypeScript markers, and counts.
- `first-run` adds review-only package/TypeScript workflow recommendations.

Not implemented:

- source-content inspection;
- script execution;
- local check execution;
- provider calls;
- automatic workflow generation;
- workflow registration;
- external writes;
- schema changes;
- examples;
- hosted/distributed behavior;
- recursive agents;
- agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 3. Agent Instruction Preservation Assessment

The scaffold behavior is appropriately conservative for real repositories that already contain agent guidance.

When `AGENTS.md` exists without a Workflow OS block, the implementation appends a bounded managed block and preserves existing unmanaged text. When the managed block already exists, only that block is refreshed. When `--force` is supplied, replacement remains explicit and observable.

This fixes the sharp onboarding path where users previously had to choose between adoption and replacing existing project-specific agent guidance.

The implementation remains bounded:

- it does not print existing `AGENTS.md` content;
- it does not inspect or transform unmanaged instructions;
- it does not merge arbitrary instruction formats;
- unmanaged `.workflow-os/agent-harness-prompt.md` still fails closed unless explicitly forced.

## 4. Safe Repo Metadata Assessment

The `first-run` metadata slice uses a safe, bounded inspection model.

It reports:

- `package.json` presence;
- package-manager posture from lockfile presence;
- allowlisted package script keys;
- TypeScript markers from dependency names and `tsconfig.json` presence;
- conventional source/test directory labels;
- `.github/workflows` YAML count;
- common repository document presence.

It does not copy:

- package script bodies;
- dependency versions;
- raw source files;
- raw test files;
- CI workflow contents;
- command output;
- provider payloads;
- environment values;
- token-like strings.

The review-only recommendations are useful and do not overclaim enforcement. They make `first-run` feel more concrete for package repositories while preserving the current observe/report posture.

## 5. Validation And UX Semantics

The phase preserves existing semantics:

- `first-run` still produces report-ready context, not a terminal `WorkReport`.
- `first-run` still does not create runtime state.
- Recommendations remain review-only.
- Safe metadata does not make package scripts required.
- Safe metadata does not register handlers.
- Safe metadata does not execute `npm`, `pnpm`, `yarn`, `bun`, or any other command.
- The optional mock first-run workflow remains separate from real first-run posture analysis.

## 6. Privacy And Redaction Assessment

Privacy posture is appropriate for the phase.

The tests include secret-like markers in existing `AGENTS.md`, package script values, and dependency names. Output assertions verify those values are not emitted in human or JSON output where they would be unsafe.

The implementation uses bounded labels and allowlisted keys. It avoids raw path disclosure beyond known repository-relative names and avoids arbitrary manifest payload copying.

Remaining privacy limitation: dependency names used as TypeScript markers are currently allowlisted only for `typescript`, `ts-node`, and `tsx`. That is acceptable for this phase, but future ecosystem expansion should keep the same allowlist posture.

## 7. Test Quality Assessment

Focused tests cover:

- unmanaged `AGENTS.md` preservation for `init-agent-harness`;
- unmanaged `AGENTS.md` preservation for `init-repo-governance`;
- managed block update behavior;
- dry-run behavior;
- explicit `--force` replacement disclosure;
- no leakage of existing unmanaged content in scaffold output;
- safe package metadata output;
- bounded JSON metadata output;
- non-copying of script bodies and dependency payloads;
- no runtime state creation from `first-run`.

Validation evidence for this PR includes:

- `cargo test -p workflow-cli --test cli init_repo_governance -- --nocapture`: passed;
- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed;
- GitHub required checks on PR head `db9cd5b428ef3fb696c5a55559ab66ade0ab2a6d`: all passed.

No blocker test gaps were found.

## 8. Documentation Review

Documentation now states:

- existing agent guidance preservation is implemented;
- safe metadata-aware first-run recommendations are implemented;
- recommendations are review-only;
- source-content inspection is not implemented;
- command execution is not implemented;
- automatic workflow generation is not implemented;
- provider calls and writes are not implemented;
- schemas, examples, hosted behavior, recursive agents, agent swarms, and release posture changes are not introduced.

A stale roadmap sentence that still described safe metadata-aware first-run recommendations as planned was corrected before this review.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add a more polished short default `first-run` summary while keeping detailed output available.
- Consider an explicit `--merge-agent-instructions` alias or wording even though preservation is now default.
- Expand safe metadata detection to other ecosystems only with the same bounded allowlist approach.
- Separate the optional mock approval/audit demo more visibly from real first-run posture analysis in CLI copy.
- Add a future review for whether `first-run` should emit suggested validation obligations as structured machine-readable recommendations.

## 11. Recommended Next Phase

First-run summary polishing and mock-demo separation.

Why: the two highest-risk real-repo onboarding blockers are addressed. The next most valuable adoption improvement is making the already-useful `first-run` output easier to scan and making the difference between real posture analysis and optional mock approval/audit demonstration impossible to miss.

Do not move into automatic workflow generation, command execution, source inspection, provider calls, writes, schemas, hosted behavior, recursive agents, agent swarms, or release posture changes as part of that phase.
