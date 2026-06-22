# Onboarding Retest P0 Fixes Report

## 1. Executive Summary

The onboarding retest P0 fixes are implemented. The changes close the concrete friction found when a clean non-WorkflowOS repository was initialized with Workflow OS:

- scaffold output now points users to `workflow-os first-run`;
- generated downstream `AGENTS.md` no longer assumes Workflow OS internal docs exist;
- `workflow-os validate` missing-manifest output now points users to `workflow-os init-repo-governance`;
- optional schema availability in `workflow-os doctor` no longer appears as `schemas: failed`.

The phase remains CLI-output and scaffold-wording only. It does not add automatic runtime generation, automatic workflow execution, command execution, workflow generation, report artifacts, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

## 2. Retest Findings Addressed

The corrected existing-repo retest found that `init-repo-governance`, `validate`, `first-run`, and the mock first-run workflow work for a normal repository, but the user-facing path still had gaps.

Addressed findings:

- `init-repo-governance` skipped `workflow-os first-run` in next-step output.
- generated downstream `AGENTS.md` referenced `docs/ENGINEERING_STANDARD.md`, which ordinary repos do not have.
- `workflow-os validate` in a normal repo without `workflow-os.yml` did not suggest the scaffold command.
- `workflow-os doctor` printed `schemas: failed` for optional schema availability.

## 3. Behavior Changed

`workflow-os init-repo-governance` now prints:

```text
next_step: workflow-os validate
next_step: workflow-os first-run
next_step: workflow-os --mock-all-local-skills run local/first-run-governance
```

Generated downstream `AGENTS.md` now asks agents to read the target repository's own engineering standard or contribution guide if one exists, plus `.workflow-os/README.md` and `.workflow-os/agent-harness-prompt.md`.

`workflow-os validate` now prints:

```text
next_step: workflow-os init-repo-governance
```

when the manifest is missing in human-readable output.

`workflow-os doctor` now prints:

```text
schemas: unavailable_optional
```

when schema directories are absent but schema availability is not the hard project-health failure.

## 4. Behavior Explicitly Not Changed

This phase does not:

- run workflows automatically;
- run `first-run` automatically;
- execute repository commands;
- execute local check handlers;
- register real skill handlers;
- create runtime state from scaffold commands;
- create WorkReport artifacts;
- generate or register workflows automatically;
- change workflow schemas;
- add runtime config;
- call providers;
- add write-capable adapters;
- add hosted/distributed behavior;
- add recursive agents, agent swarms, or Level 3/4 autonomy.

## 5. Generated AGENTS.md Wording Summary

The generated downstream `AGENTS.md` is now portable.

It no longer references Workflow OS's internal `docs/ENGINEERING_STANDARD.md`. That requirement remains appropriate for Workflow OS's own root `AGENTS.md`, but not for user repositories initialized by the scaffold.

The generated file now points agents to:

- the downstream repository's own engineering standard or contribution guide, if present;
- `.workflow-os/README.md`;
- `.workflow-os/agent-harness-prompt.md`.

## 6. Validation And Missing-Manifest Guidance Summary

Missing manifest remains a validation failure. The CLI does not silently scaffold files or convert a missing project into success.

The difference is that human-readable `validate` output now gives an actionable next step for normal repositories:

```text
next_step: workflow-os init-repo-governance
```

This keeps validation deterministic while making the onboarding path discoverable.

## 7. Doctor Posture Summary

`doctor` still exits non-zero when the local project is missing or cannot be loaded safely.

Schema directory availability is reported as optional in the local preview when absent. The human-readable label changed from `failed` to `unavailable_optional` so users do not read optional schema lookup as an unhealthy install by itself.

## 8. Tests Added

Focused CLI tests cover:

- missing manifest validation suggests `workflow-os init-repo-governance`;
- generated `AGENTS.md` uses portable downstream wording;
- generated `AGENTS.md` does not reference `docs/ENGINEERING_STANDARD.md`;
- `init-repo-governance` output includes `workflow-os first-run`;
- `doctor` reports `schemas: unavailable_optional` rather than `schemas: failed` for missing optional schemas.

Existing first-run, init-agent-harness, init-repo-governance, validation, runtime, state, and docs tests remain in scope for full validation.

## 9. Commands Run And Results

Commands run:

- `cargo fmt --all` - passed.
- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-cli --test cli` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

A clean temporary existing-repo onboarding smoke also passed:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
workflow-os --mock-all-local-skills run local/first-run-governance
workflow-os init-agent-harness
```

The explicit mock run paused for approval as expected. No command in this smoke created report artifacts, provider writes, hosted behavior, or real local skill handlers.

## 10. Remaining Limitations

- The generated first-run workflow still uses mockable local skills; real handlers remain future work.
- `first-run` emits report-ready context, not a terminal WorkReport from a completed workflow run.
- Missing ownership/escalation values are warning-only in the current local preview.
- Dogfood workflows remain internal/reference patterns, not downstream defaults.
- Sidecar external-repo mode, automatic workflow recommendations, catalog/store governance, and write-capable adapters remain future work.

## 11. Recommended Next Phase

Proceed to onboarding retest P0 fixes review.

Reason: these changes are intentionally small but user-facing. A focused review should verify that the CLI/scaffold wording fixes the retest blockers without expanding runtime authority or overclaiming automation.
