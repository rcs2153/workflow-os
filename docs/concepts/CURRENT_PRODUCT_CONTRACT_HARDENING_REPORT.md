# Current Product Contract Hardening Report

## 1. Executive Summary

The current-product contract hardening phase is complete.

External kernel review confirmed that Workflow OS is credible as a local
governance kernel, while also identifying preview-readiness risks in CLI
identity discoverability, docs drift, scaffold-file documentation, first-run
operator clarity, and the bridge from review-only recommendations to governed
workflow authoring.

This phase audited those surfaces, patched current docs where they lagged
implemented behavior, added explicit CLI version documentation, verified that
existing focused CLI tests already cover the core behavior, and preserved the
local preview boundary.

## 2. Scope Completed

- Added explicit `docs/cli/version.md` documentation for `workflow-os
  --version`, `workflow-os version`, and preview JSON version output.
- Updated CLI indexes and overview docs so version/build identity is visible as
  current product contract.
- Corrected `init-repo-governance` file-safety wording for `AGENTS.md`:
  unmanaged repo-specific agent guidance is preserved by default, while
  `--force` remains the explicit replacement boundary.
- Clarified the agent harness quickstart so normal downstream adoption starts
  from `init-repo-governance` and `first-run`, while `dg/*` workflows remain
  Workflow OS internal dogfood benchmark workflows rather than community
  defaults.
- Updated the roadmap and implementation plan status to reflect implementation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- provider writes;
- automatic workflow generation;
- automatic workflow promotion;
- automatic local check execution;
- hidden skill handler registration;
- hosted or distributed runtime behavior;
- schemas;
- examples;
- reasoning lineage or claim graph;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Audit Findings

The audit found that several external-review findings were already implemented
and covered by tests:

- `workflow-os --version` works outside a Workflow OS project.
- `workflow-os version` works outside a Workflow OS project.
- `workflow-os --json version` emits bounded identity and release posture.
- `init-repo-governance` includes `policies/local.policy.yml`.
- `init-repo-governance` preserves existing unmanaged `AGENTS.md` content by
  default.
- `init-repo-governance --dry-run` discloses preservation without writing files
  or leaking existing content.
- `init-repo-governance --force` replaces existing `AGENTS.md` only at an
  explicit replacement boundary.
- `first-run` has concise default text and `--verbose` detailed posture.
- `first-run` separates real bounded posture analysis from the optional mock
  approval/audit demo.
- `first-run` safe metadata detection avoids raw source, manifest, workflow,
  command, dependency, and script-body payloads.
- `first-run` distinguishes scaffold-only `tests/` from user test metadata.

The audit found stale documentation around `AGENTS.md` preservation in
`docs/cli/init-repo-governance.md`; this phase corrected that current-product
contract claim.

## 5. Tests And Coverage

Existing tests already cover the behavior this hardening phase needed to lock:

- `version_command_reports_cli_version_without_project`
- `version_subcommand_reports_cli_version_without_project`
- `version_json_is_bounded_without_project`
- `init_repo_governance_creates_valid_local_project`
- `init_repo_governance_preserves_existing_agents_file_by_default`
- `init_repo_governance_dry_run_preserves_existing_agents_file`
- `init_repo_governance_force_replaces_existing_agents_file_with_warning`
- first-run report-ready context, summary, safe metadata, and recommendation
  output tests in `crates/workflow-cli/tests/cli.rs`

No runtime code changes were required.

## 6. Privacy And Security

The phase preserves the existing privacy boundary:

- no raw source contents;
- no package script bodies;
- no GitHub Actions workflow bodies;
- no command output;
- no provider payloads;
- no environment values;
- no credentials, authorization headers, tokens, or private keys.

Documentation now more accurately states the preservation and replacement
boundary for agent guidance files.

## 7. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start ...`: passed, created governed run
  `run-1783766503798053000-2`.
- Proof-enforced governed approval for `run-1783766503798053000-2`: granted.
- `npm run check:docs`: passed.
- `cargo test -p workflow-cli --test cli
  version_command_reports_cli_version_without_project`: passed.
- `cargo test -p workflow-cli --test cli
  init_repo_governance_preserves_existing_agents_file_by_default`: passed.
- `cargo test -p workflow-cli --test cli
  init_repo_governance_dry_run_preserves_existing_agents_file`: passed.
- `cargo test -p workflow-cli --test cli
  first_run_after_repo_governance_outputs_report_ready_context`: passed.

Full workspace cargo checks were not rerun because this phase changed
current-product documentation only and existing focused CLI tests already cover
the behavior being documented.

## 8. Remaining Known Limitations

- CLI JSON remains experimental and is not yet a stable versioned machine
  contract.
- `first-run` recommendations remain review-only until explicit workflow
  authoring, preflight, stewardship review, and promotion.
- Automatic workflow generation and automatic local check execution remain
  unimplemented.
- The generated mock first-run workflow remains an approval/audit demo unless a
  real local handler is separately implemented and registered.
- Historical phase reports may contain older observations and should not be
  read as the current product contract.

## 9. Recommended Next Phase

Recommended next phase: current product contract hardening review.

Reason: this phase intentionally changed current user-facing documentation and
should receive a focused maintainer review before returning to broader
write-adjacent runtime composition work.
