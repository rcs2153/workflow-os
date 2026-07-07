# First-Run Recommendation Next-Action Report

## 1. Executive Summary

The first-run workflow recommendation next-action refinement slice is implemented.

`workflow-os first-run` now emits bounded next-action guidance for review-only workflow recommendations. The default human output includes a short `recommendation_next_actions` group, verbose output includes per-recommendation `next_action` codes, and preview JSON exposes both surfaces for agents and tooling.

This makes first-run recommendations easier to act on without generating workflows automatically, executing commands, calling providers, registering checks, or changing runtime state.

## 2. Scope Completed

- Added bounded per-recommendation `next_action` codes.
- Added default human `recommendation_next_actions` grouping.
- Added verbose text output for recommendation next actions.
- Added preview JSON output for recommendation next actions.
- Added focused tests for default output, verbose output, JSON output, and ecosystem-specific candidate ordering.
- Updated first-run CLI documentation.
- Updated the roadmap.

## 3. Scope Explicitly Not Completed

- No automatic workflow generation.
- No automatic workflow registration.
- No command execution.
- No local check execution.
- No provider calls.
- No source-content inspection.
- No manifest-body interpretation beyond existing bounded metadata.
- No schemas.
- No examples.
- No writes.
- No hosted or distributed behavior.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy.
- No release posture changes.

## 4. Behavior Added

Default `workflow-os first-run` output now includes:

```text
recommendation_next_actions:
  - review_only: recommendations are not active workflows until authored and reviewed
  - start_with: first_run.assign_ownership
  - workflow_candidate: first_run.repo_implementation
  - validation_candidate: first_run.evidence_check_requirements
  - safety_candidate: first_run.side_effect_posture
  - closure_candidate: first_run.report_handoff_obligations
```

When safe metadata supports a more specific candidate, the workflow and validation candidates point to the most concrete detected recommendation, such as:

- `first_run.typescript_implementation`;
- `first_run.package_validation_obligations`;
- `first_run.rust_implementation`;
- `first_run.rust_validation_obligations`.

Verbose recommendation rows now include bounded action codes such as:

- `review_and_author_workflow_spec`;
- `replace_placeholder_owner_and_escalation`;
- `define_evidence_and_validation_obligations`;
- `define_side_effect_posture_before_writes`;
- `define_report_and_handoff_obligations`.

Preview JSON exposes the same bounded action vocabulary.

## 5. Privacy And Redaction

The new next-action surface is static vocabulary derived from existing recommendation IDs and kinds. It does not copy:

- raw source contents;
- manifest bodies;
- script command bodies;
- dependency values;
- GitHub Actions workflow names or bodies;
- command output;
- provider payloads;
- environment values;
- credentials;
- token-like values.

The implementation continues to use existing safe repository metadata detection and review-only recommendation construction.

## 6. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783391165613527000-2`.
- Approval ID: `approval/run-1783391165613527000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the complete approval handoff block was surfaced.
- Scope: first-run workflow recommendation next-action refinement.

## 7. Tests Added Or Updated

Focused tests now cover:

- default output includes `recommendation_next_actions`;
- generic first-run recommendations point at stewardship, workflow, validation, side-effect, and report/handoff candidates;
- verbose output includes per-recommendation `next_action` codes;
- JSON output includes per-recommendation `next_action` codes;
- JSON output includes the bounded `recommendation_next_actions` array;
- TypeScript/package metadata selects TypeScript/package workflow and validation candidates;
- Rust metadata selects Rust workflow and validation candidates;
- existing first-run non-leakage and no-runtime-state tests still pass.

## 8. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase implementation ...`: passed.
- `./target/debug/workflow-os ... approve run-1783391165613527000-2 ...`: passed.
- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed during implementation.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783391165613527000-2 --phase implementation ...`: passed with 39 events, 1 approval, 0 retries, and 0 escalations.

## 9. Remaining Known Limitations

- Recommendations remain review-only.
- Next-action hints are static guidance, not workflow generation.
- No workflow files are authored or registered automatically.
- No local check handlers are activated by the first-run command.
- Missing-citation records and workflow catalog storage remain outside this slice.

## 10. Recommended Next Phase

Recommended next phase: first-run recommendation next-action refinement review.

The review should verify that the new action hints make first-run output more actionable while preserving the metadata-only, review-only, non-executing boundary.
