# Existing Repo Governance Scaffold Report

## 1. Executive Summary

The first existing-repo governance onboarding slice is implemented. `workflow-os init-repo-governance` creates a minimal valid local Workflow OS project envelope in an existing repository so a user can validate the project and run an approval-gated first-run mock workflow without copying Workflow OS's internal dogfood workflows.

This phase implements in-repo scaffolding only. It does not implement first-run ledger/report mode, automatic workflow recommendations, sidecar external-repo governance, arbitrary command execution, real local handlers, report artifact writing, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

## 2. Scope Completed

- Added `workflow-os init-repo-governance`.
- Added `--output-dir`, `--agent`, `--force`, and `--dry-run` handling for the scaffold.
- Generated a minimal `workflow-os.yml`.
- Generated a first-run approval-gated local workflow.
- Generated a mockable local first-run report skill spec.
- Generated a conservative default governance policy.
- Generated a declarative test spec.
- Generated `.workflow-os/README.md`.
- Reused managed-block agent instruction scaffolding for `AGENTS.md` and `.workflow-os/agent-harness-prompt.md`.
- Added CLI tests for scaffold creation, validation, approval-gated mock execution, dry-run behavior, fail-closed existing file behavior, and `--force`.
- Updated CLI/user docs and the onboarding plan status.

## 3. Scope Explicitly Not Completed

- No first-run ledger/report command.
- No automatic WorkReport generation from the scaffold command.
- No automatic workflow recommendation generation.
- No sidecar external-repo scaffold.
- No arbitrary command execution.
- No real local skill handler registration.
- No runtime state writes during scaffold creation.
- No report artifact writes.
- No GitHub/Jira/CI writes.
- No provider calls.
- No schema changes.
- No hosted or distributed runtime behavior.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy.

## 4. CLI API Summary

```sh
workflow-os init-repo-governance [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]
```

Generated files:

- `workflow-os.yml`
- `workflows/first-run-governance.workflow.yml`
- `skills/first-run-report.skill.yml`
- `policies/default-governance.policy.yml`
- `tests/first-run-governance.test.yml`
- `.workflow-os/README.md`
- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

## 5. Generated Project Behavior

The generated project validates through the existing Rust validator. The generated workflow can be run with:

```sh
workflow-os --mock-all-local-skills run local/first-run-governance
```

It pauses for human approval before the mock first-run report step completes. `--mock-all-local-skills` remains a preview convenience and does not prove a real handler exists.

## 6. Safety And Boundary Summary

The scaffold fails closed when plain scaffold targets already exist unless `--force` is supplied. Agent instruction files use managed-block behavior and preserve surrounding user content when possible.

The command writes only scaffold files. It does not run workflows, append runtime events, create state, execute commands, call providers, or create report artifacts.

## 7. Test Coverage Summary

Focused CLI tests cover:

- help text includes the new command;
- scaffold creates expected files;
- generated project validates;
- generated workflow runs to approval and completes after approval with mock local skills;
- dry run writes no files or state;
- existing project file fails closed without leaking contents;
- `--force` intentionally replaces existing scaffold targets.

## 8. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- First-run ledger/report mode is not implemented.
- The generated first-run skill is mockable only unless a real handler is separately implemented and reviewed.
- Generated workflow recommendations are documented as future posture, not generated automatically.
- Sidecar governance for external repositories is deferred.
- Capability-aware blocked-vs-failed classification is deferred.

## 10. Recommended Next Phase

Proceed to **existing repo governance scaffold review**.

If accepted, the next implementation lane should be **first-run governed ledger/report mode planning**, focused on producing an evidence-aware WorkReport or report-ready context from the generated governance envelope without executing arbitrary commands or fabricating evidence.
