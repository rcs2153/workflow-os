# Generated Agent Guidance Portability P1 Report

Status: Implemented and accepted after focused maintainer review.

## Summary

Fresh-repository evaluation found that generated `AGENTS.md` guidance referred
to `docs/user-guide/agent-harness-quickstart.md`, a path that exists in the
Workflow OS source checkout but is not scaffolded into downstream repositories.
The generated guidance therefore contained a dead instruction even though the
rest of the onboarding scaffold was valid.

This correction keeps the scaffold boundary unchanged. Generated guidance is
now context-aware: `workflow-os init-agent-harness` references only its local
`.workflow-os/agent-harness-prompt.md`, while
`workflow-os init-repo-governance` references that prompt and its additional
local `.workflow-os/README.md` setup note. The source-checkout-only guide is no
longer emitted by either scaffold path.

## Governed Phase

- Workflow: `dg/implement`
- Run: `run-1793722316101656000-2`
- Approval: `approval/run-1793722316101656000-2/implementation-approved`
- Approval presentation: `presentation/a350c051d043e0e5`
- Approval outcome: granted

## Implemented Scope

- Replaced the source-checkout-only quickstart reference in generated
  `AGENTS.md` content with context-aware references to files created by the
  selected scaffold command.
- Added regression assertions for both scaffold entry points proving that:
  - the source-checkout-only path is absent;
  - the generated local prompt reference is present; and
  - the referenced local prompt file exists after scaffolding.
- Preserved the existing managed-block, existing-guidance preservation, and
  force-replacement behavior.

## Non-Goals

This phase does not:

- add another scaffolded guide;
- depend on remote documentation availability;
- broaden generated workflow or policy content;
- change schemas, runtime execution, providers, continuation behavior, hosted
  behavior, or release posture; or
- redesign the broader onboarding experience.

## Validation

- Focused `init_agent_harness_creates_scaffold_files`: passed.
- Focused `init_repo_governance_creates_valid_local_project`: passed.
- `cargo test -p workflow-cli --test cli`: passed, 176 tests.
- `cargo clippy -p workflow-cli --all-targets --all-features -- -D warnings`:
  passed.
- `cargo fmt --all -- --check`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Focused review initially found one blocker: the shared template still referenced
`.workflow-os/README.md` for `init-agent-harness`, which does not create that
file. The review-driven correction made local guidance context-aware and the
complete validation set passed again. See [Generated Agent Guidance
Portability P1 Review](GENERATED_AGENT_GUIDANCE_PORTABILITY_P1_REVIEW.md).

## Next Authorized Work

After focused review accepts this correction, the next authoritative P1 queue
item is provider-fixture-aware `workflow-os next-action` routing.
