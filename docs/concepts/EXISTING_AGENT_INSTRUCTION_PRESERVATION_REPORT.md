# Existing Agent Instruction Preservation Report

## 1. Executive Summary

The existing agent-instruction preservation slice is implemented.

`workflow-os init-repo-governance` and `workflow-os init-agent-harness` now preserve existing repository-specific `AGENTS.md` content by default. When an existing file has no Workflow OS managed block, the CLI appends the managed Workflow OS block instead of failing and pushing users toward destructive `--force`.

This keeps real-repo onboarding safe while making adoption smoother for repositories that already contain useful agent guidance.

## 2. Scope Completed

- Preserved unmanaged `AGENTS.md` content by default.
- Appended the Workflow OS managed block when `AGENTS.md` exists without one.
- Continued updating existing Workflow OS managed blocks in place.
- Kept explicit `--force` replacement behavior.
- Added bounded dry-run and write-path messages for preserve, append, update, and replace behavior.
- Added focused CLI tests for `init-agent-harness` and `init-repo-governance`.
- Updated roadmap, onboarding planning, and user-guide documentation.

## 3. Scope Explicitly Not Completed

- No safe repo metadata inspection.
- No metadata-aware `first-run` recommendations.
- No source-content inspection.
- No command execution.
- No automatic local check execution.
- No provider calls.
- No automatic workflow generation.
- No workflow schema changes.
- No examples.
- No hosted or distributed runtime behavior.
- No write-capable adapters.
- No recursive agents, agent swarms, or Level 3/4 autonomy.
- No release posture changes.

## 4. Behavior Added

For `AGENTS.md`:

- Missing file: create the managed Workflow OS file.
- Existing file with managed block: replace only the managed block.
- Existing file without managed block: preserve existing content and append the managed block.
- `--force`: replace the file and print a bounded replacement notice.
- `--dry-run`: print bounded notices showing whether unmanaged content would be preserved and a managed block would be appended.

Other unmanaged scaffold targets remain fail-closed unless `--force` is supplied.

## 5. Privacy And Redaction

The CLI does not echo existing `AGENTS.md` content in preserve, append, update, force, or dry-run messages.

Focused tests include secret-like markers in existing files and verify that command output does not leak those markers.

## 6. Tests Added

- `init_agent_harness_unmanaged_agents_file_is_preserved_without_force`
- `init_agent_harness_dry_run_preserves_unmanaged_agents_file`
- `init_repo_governance_preserves_existing_agents_file_by_default`
- `init_repo_governance_dry_run_preserves_existing_agents_file`
- `init_repo_governance_force_replaces_existing_agents_file_with_warning`

Existing managed-block update, force, dry-run, scaffold, validation, and approval-gated generated workflow tests continue to pass.

## 7. Validation Commands Run

- `cargo test -p workflow-cli --test cli init_agent_harness -- --nocapture`: passed.
- `cargo test -p workflow-cli --test cli init_repo_governance -- --nocapture`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 8. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run: `run-1783315755158355000-2`.
- Approval: `approval/run-1783315755158355000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer judgment after the full approval handoff was surfaced.
- Event summary: 39 total events, including one approval request, one approval grant, eight policy decisions, six scheduled skill invocations, and terminal completion.
- Out-of-kernel work disclosed: repo edits, shell validation commands, formatting, docs updates, git/PR operations, and this phase report were performed by Codex/human execution outside the kernel while the kernel governed the phase boundary.

## 9. Remaining Known Limitations

- First-run recommendations remain generic until safe repo metadata inspection is implemented.
- The generated mock first-run workflow remains an approval/audit demonstration, not repository analysis.
- Existing non-`AGENTS.md` unmanaged scaffold files still fail closed unless `--force` is explicit.

## 10. Recommended Next Phase

Recommended next phase: safe repo metadata-aware first-run recommendations.

Start with bounded `package.json`/TypeScript detection, script-key posture, and review-only workflow/check recommendations. Do not inspect raw source contents, execute commands, generate workflows automatically, call providers, add schemas, add examples, enable hosted behavior, add writes, or change release posture.
