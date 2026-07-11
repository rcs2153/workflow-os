# Current Product Contract Hardening Review

## 1. Executive Verdict

Phase accepted; proceed to broader write-adjacent roadmap continuation.

The implementation stayed within the approved current-product contract
hardening scope. It corrected current user-facing documentation drift, made CLI
version identity discoverable, documented the `AGENTS.md` preservation behavior
that tests already prove, clarified downstream onboarding versus internal
dogfood workflows, and did not broaden runtime behavior.

## 2. Scope Verification

The phase stayed within docs/report hardening scope.

No accidental implementation was found for:

- provider writes;
- write-capable adapters;
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

Historical phase reports were not broadly rewritten. The current-product report
was updated as the implementation closeout, which is appropriate because it is
the active report for this phase.

## 3. CLI Identity Assessment

The phase adds [version](../cli/version.md), and the CLI command indexes now
surface `workflow-os --version`, `workflow-os version`, and preview JSON
version output.

The documentation correctly states that the version command:

- does not require a Workflow OS project;
- does not create runtime state;
- does not load provider credentials;
- does not call external services;
- does not inspect repository source contents;
- emits bounded identity and release posture only.

Existing tests cover:

- `version_command_reports_cli_version_without_project`;
- `version_subcommand_reports_cli_version_without_project`;
- `version_json_is_bounded_without_project`.

## 4. Documentation Truth Assessment

The implementation fixes the important stale documentation claim in
[init-repo-governance](../cli/init-repo-governance.md): unmanaged `AGENTS.md`
content is preserved by default and the managed Workflow OS block is appended
or updated in place. `--force` remains the explicit replacement boundary.

The generated-file list includes `policies/local.policy.yml`, matching current
scaffold behavior.

The roadmap and current-product hardening plan now mark the phase as
implemented and keep non-goals explicit.

## 5. Onboarding Boundary Assessment

The quickstart now leads normal downstream users toward:

```text
workflow-os init-repo-governance
workflow-os first-run
```

It also states that repo-local `dg/*` workflows are internal Workflow OS
dogfood benchmark workflows, not downstream defaults or plug-and-play
community assets. That aligns with the user feedback from real-repo onboarding
tests.

## 6. First-Run And Recommendation Bridge Assessment

The phase did not change runtime behavior, but it correctly documents the
already implemented bridge:

```text
first-run -> recommendation detail -> author workflow dry-run -> explicit draft output -> preflight -> steward review -> promote
```

Existing tests cover the core user-facing behavior:

- concise `first-run` output;
- optional mock approval/audit demo separation;
- bounded safe metadata detection;
- scaffold-only `tests/` separation;
- recommendation detail and authoring dry-run behavior.

The review confirms that recommendations remain review-only until explicitly
authored, preflighted, reviewed, and promoted.

## 7. Privacy And Redaction Assessment

The phase preserves the privacy boundary:

- no raw source contents;
- no package script bodies;
- no GitHub Actions workflow bodies;
- no command output;
- no provider payloads;
- no environment values;
- no credentials, authorization headers, tokens, or private keys.

Docs accurately describe that safe metadata and current-product contract
outputs are bounded.

## 8. Test Quality Assessment

The report cites focused tests that map directly to the external review
feedback:

- version commands outside a project;
- JSON version posture;
- `init-repo-governance` valid scaffold creation;
- unmanaged `AGENTS.md` preservation by default;
- dry-run preservation without leakage;
- `--force` replacement boundary;
- first-run report-ready context.

No new tests were necessary because no runtime behavior changed and existing
coverage already protected the documented behavior.

## 9. Validation

Reviewed validation:

- `npm run check:docs`: passed.
- `cargo test -p workflow-cli --test cli
  version_command_reports_cli_version_without_project`: passed.
- `cargo test -p workflow-cli --test cli
  init_repo_governance_preserves_existing_agents_file_by_default`: passed.
- `cargo test -p workflow-cli --test cli
  init_repo_governance_dry_run_preserves_existing_agents_file`: passed.
- `cargo test -p workflow-cli --test cli
  first_run_after_repo_governance_outputs_report_ready_context`: passed.
- `git diff --check`: passed.
- GitHub required checks for PR #300: passed before merge.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Consider adding a short `workflow-os version` mention to first-use
  troubleshooting in a future docs polish pass.
- Consider a generated or centrally checked current-product contract index if
  the documentation surface keeps growing.
- Continue to treat historical phase reports as historical, not current
  product contract.

## 12. Recommended Next Phase

Recommended next phase: return to broader write-adjacent roadmap continuation,
starting from the next unreviewed accepted helper or plan in the roadmap.

Reason: the current-product contract hardening lane has addressed the preview
trust gap identified by external testing. The next useful work is to continue
runtime composition or write-readiness work without adding new primitive
families unless they unblock enforcement.
