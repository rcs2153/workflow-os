# Onboarding Retest P0 Fixes Plan

Status: Implemented. The implementation is documented in [Onboarding Retest P0 Fixes Report](../concepts/ONBOARDING_RETEST_P0_FIXES_REPORT.md). It updates CLI guidance and generated downstream agent instructions only; it does not change schemas, execute commands automatically, create workflows automatically, add runtime config, add hosted behavior, add write-capable adapters, or change release posture.

## 1. Executive Summary

A clean existing-repository retest confirmed that the current onboarding path is materially better: `workflow-os init-repo-governance`, `workflow-os validate`, `workflow-os first-run`, and the mock first-run workflow can take a normal repository into a valid local governance envelope.

The retest also found two P0 onboarding blockers:

- `init-repo-governance` output skips `workflow-os first-run`, even though `first-run` is the strongest explanation of why Workflow OS is valuable before custom workflows exist.
- generated downstream `AGENTS.md` tells agents to read `docs/ENGINEERING_STANDARD.md`, which exists in Workflow OS but not in ordinary target repositories.

Two additional small fixes should be included if they stay narrow:

- pre-scaffold `workflow-os validate` missing-manifest output should suggest `workflow-os init-repo-governance`;
- `workflow-os doctor` should not report optional schema lookup as scary `schemas: failed` when the install is otherwise usable.

This plan defines the next implementation phase to fix those onboarding gaps without expanding runtime authority.

## 2. Goals

- Make the existing-repo onboarding command sequence obvious.
- Ensure generated agent instructions fit downstream repositories, not only Workflow OS itself.
- Preserve the "Agent executes. Workflow OS governs." adoption path.
- Make missing-manifest validation errors actionable for first-time users.
- Avoid misleading health output for optional schema checks.
- Keep all changes local, bounded, and documentation/CLI-output oriented.
- Preserve current validation, runtime, approval, report, state, and adapter semantics.

## 3. Non-Goals

Do not implement in this phase:

- automatic runtime report generation;
- automatic workflow execution;
- automatic command execution;
- automatic local check execution;
- workflow generation or registration;
- workflow schema changes;
- runtime config;
- report artifacts;
- persistence changes;
- CLI report rendering;
- RBAC, IdP, admin controls, or escalation routing;
- provider calls or write-capable adapters;
- hosted/distributed runtime;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Retest Findings

The retest simulated a clean non-WorkflowOS repository and ran the expected evaluator path.

Worked:

- `init-repo-governance --dry-run` showed the files that would be written.
- `init-repo-governance` produced a valid local project envelope.
- `validate` passed after scaffolding.
- `first-run` clearly disclosed missing evidence, skipped checks, unsupported side effects, governance profile, ownership/escalation posture, approvals, policy gates, and review-only recommendations.
- the generated local workflow paused for approval and completed after explicit approval.
- no report artifacts, provider writes, or side-effect records were created.

Confused or failed:

- pre-scaffold `validate` reported missing `workflow-os.yml` without suggesting the scaffold command;
- scaffold output jumped from `validate` to mock `run`, skipping `first-run`;
- generated downstream `AGENTS.md` referenced `docs/ENGINEERING_STANDARD.md`, which ordinary repos do not have;
- `doctor` printed `schemas: failed`, which reads like an unhealthy install even when validation and runtime work.

## 5. Fix 1: Scaffold Next Steps Include First-Run

`workflow-os init-repo-governance` should print the safe evaluator sequence:

```text
next_step: workflow-os validate
next_step: workflow-os first-run
next_step: workflow-os --mock-all-local-skills run local/first-run-governance
```

Reason:

- `first-run` is the immediate Governed Work Pattern ledger/report posture.
- It shows value before mature custom workflows exist.
- It explains missing evidence, skipped checks, side-effect posture, approval posture, ownership/escalation warnings, and review-only workflow recommendations.

Boundaries:

- do not run `first-run` automatically;
- do not run the mock workflow automatically;
- do not create runtime state from the scaffold command;
- do not create report artifacts.

## 6. Fix 2: Downstream AGENTS.md Must Not Require Workflow OS Internal Docs

Generated downstream `AGENTS.md` should not say:

```text
Read docs/ENGINEERING_STANDARD.md
```

unless such a file exists in the target repo or the generator can truthfully scope the sentence as optional.

Recommended wording:

```text
Read this repository's engineering standard or contribution guide if one exists.
Read .workflow-os/README.md and .workflow-os/agent-harness-prompt.md before governed work.
```

For Workflow OS's own repo, the root `AGENTS.md` may continue to reference `docs/ENGINEERING_STANDARD.md`. The generated scaffold for ordinary repositories should be portable and should not pull the agent back into Workflow OS internals.

Boundaries:

- do not remove the engineering-standard requirement from Workflow OS's own root `AGENTS.md`;
- do not assume downstream repos have `docs/`, `CONTRIBUTING.md`, or internal roadmap docs;
- do not ask agents to copy Workflow OS dogfood workflows into downstream projects.

## 7. Fix 3: Missing Manifest Validation Guidance

When `workflow-os validate` is run in a normal repository with no `workflow-os.yml`, the error should remain bounded but actionable.

Recommended output should include:

```text
No workflow-os.yml was found.
Run `workflow-os init-repo-governance` to scaffold local governance files, or pass --project-dir to an existing Workflow OS project.
```

Boundary:

- do not convert missing manifest into success;
- do not silently scaffold files;
- do not inspect raw repository source contents;
- do not emit paths beyond existing bounded CLI behavior unless already part of the diagnostic contract.

## 8. Fix 4: Doctor Schema Posture

If `workflow-os doctor` reports schema availability as `schemas: failed` for optional or non-blocking schema lookup, the wording should be made less alarming.

Candidate wording:

```text
schemas: unavailable_optional
```

or:

```text
schemas: not_available
```

The implementation should first inspect current doctor semantics. If schema absence truly blocks a supported command, keep a failure. If it is optional in the local preview, use a non-failing posture label.

Boundaries:

- do not hide real failures;
- do not change validation semantics;
- do not introduce schema generation or schema downloads;
- do not add network access.

## 9. User Experience After Fix

A user in an ordinary repo should see:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
workflow-os --mock-all-local-skills run local/first-run-governance
```

The user should understand:

- Workflow OS can be useful immediately after setup;
- `first-run` is the ledger/report posture, not a workflow run;
- the explicit mock run is separate and approval-gated;
- the generated agent instructions apply to the user's repo;
- dogfood workflows are Workflow OS internal reference patterns, not community defaults.

## 10. Test Plan

Future implementation should add focused tests for:

- `init-repo-governance` text output includes `workflow-os first-run` between `validate` and mock `run`;
- `init-repo-governance --dry-run` either includes the same next-step sequence or clearly reports planned writes only;
- generated downstream `AGENTS.md` does not contain `docs/ENGINEERING_STANDARD.md`;
- generated downstream `AGENTS.md` points to `.workflow-os/README.md` and `.workflow-os/agent-harness-prompt.md`;
- Workflow OS's own root `AGENTS.md` remains unchanged;
- `workflow-os validate` without `workflow-os.yml` suggests `workflow-os init-repo-governance`;
- missing-manifest validation error does not leak raw paths, source contents, environment values, tokens, or provider payloads;
- `workflow-os doctor` schema output is non-alarming when schema availability is optional;
- existing `init-agent-harness`, `init-repo-governance`, `first-run`, validation, runtime, docs, and CLI tests still pass.

## 11. Documentation Updates

Future implementation should update:

- `docs/cli/init-repo-governance.md`;
- `docs/cli/overview.md`;
- `docs/user-guide/agent-harness-quickstart.md`;
- `docs/implementation-plans/existing-repo-governance-onboarding-plan.md`;
- `ROADMAP.md`;
- implementation and review reports for this fix phase.

Docs must say:

- first-run is the recommended step after scaffold validation;
- generated downstream agent instructions are portable and do not assume Workflow OS internal docs exist;
- dogfood workflows are internal/reference patterns, not user defaults;
- no automatic workflow execution, command execution, report artifacts, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy are introduced.

## 12. Proposed Implementation Sequence

1. Update `init-repo-governance` next-step output.
2. Update generated downstream `AGENTS.md` and agent prompt wording.
3. Add missing-manifest guidance to `validate` if absent in the current code path.
4. Inspect and adjust `doctor` schema posture only if it is optional/non-blocking.
5. Add focused CLI tests.
6. Update docs and create an end-of-phase report.
7. Run full relevant validation.
8. Run a clean temporary existing-repo onboarding smoke test.

## 13. End-Of-Phase Report

Future implementation should create:

- `docs/concepts/ONBOARDING_RETEST_P0_FIXES_REPORT.md`

The report should include:

1. executive summary;
2. retest findings addressed;
3. behavior changed;
4. behavior explicitly not changed;
5. generated AGENTS.md wording summary;
6. validation/missing-manifest guidance summary;
7. doctor posture summary;
8. tests added;
9. commands run and results;
10. clean temporary onboarding smoke test result;
11. remaining limitations;
12. recommended next phase.

## 14. Validation For Future Implementation

Run:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Also run a clean temporary existing-repo onboarding smoke:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
workflow-os --mock-all-local-skills run local/first-run-governance
workflow-os init-agent-harness
```

## 15. Acceptance Criteria

- Scaffold output includes `workflow-os first-run` as the next step after validation.
- Generated downstream `AGENTS.md` does not reference Workflow OS internal `docs/ENGINEERING_STANDARD.md`.
- Missing-manifest validation output points users to `workflow-os init-repo-governance`.
- Optional schema doctor posture no longer reads as a hard failure.
- Existing validation and runtime semantics are unchanged.
- No automatic runtime generation, command execution, workflow generation, report artifacts, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy are introduced.

## 16. Final Recommendation

The next implementation prompt should be:

```text
Existing-repo onboarding retest P0 fixes.
```

This is the right next phase because it closes the concrete evaluator friction found in the corrected real onboarding retest, while keeping the kernel's current local/no-write/no-automatic-command boundary intact.
