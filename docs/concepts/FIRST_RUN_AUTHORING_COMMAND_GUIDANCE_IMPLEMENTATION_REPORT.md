# First-Run Authoring Command Guidance Implementation Report

## 1. Executive Summary

This phase implements bounded default `workflow-os first-run` command guidance
for the existing recommendation-to-authoring bridge.

Default human first-run output now selects one already-computed recommendation
and shows how to inspect it and preview inactive workflow authoring without
mutating the repository:

```sh
workflow-os first-run --recommendation <id>
workflow-os author workflow --from-recommendation <id> --dry-run
```

This closes a first-use UX gap from real-repository testing: Workflow OS already
had recommendation detail and authoring dry-run surfaces, but users had to know
those commands existed.

## 2. Scope Completed

- Added bounded default `authoring_command_guidance` output to `workflow-os
  first-run`.
- Selected one existing recommendation using a deterministic priority order:
  TypeScript, Rust, Python, Go, generic repo implementation, validation
  obligations, ownership, and report handoff.
- Printed recommendation detail guidance for the selected recommendation.
- Printed authoring dry-run guidance only when the selected recommendation is a
  workflow-creation candidate.
- Preserved the review-only and non-mutating boundary.
- Added focused CLI tests for generic and TypeScript recommendation selection.
- Updated CLI docs and roadmap.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic workflow generation;
- file writing by default;
- workflow promotion;
- workflow execution from first-run;
- approval execution;
- local check execution;
- provider calls;
- runtime state creation;
- report artifact writing;
- schema changes;
- examples;
- hosted or distributed behavior;
- write-capable adapters;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Behavior Added

Default first-run output now includes:

```text
authoring_command_guidance:
  - inspect_recommendation: workflow-os first-run --recommendation first_run.typescript_implementation
  - preview_authoring: workflow-os author workflow --from-recommendation first_run.typescript_implementation --dry-run
  - posture: review_only_non_mutating
```

For repositories without more concrete ecosystem metadata, the generic
`first_run.repo_implementation` recommendation is selected.

The output is constructed from static command tokens and existing bounded
recommendation IDs only.

## 5. Safety And Privacy

The guidance:

- does not fabricate recommendation IDs;
- does not print package script bodies;
- does not print dependency values;
- does not print source contents;
- does not print command output;
- does not include provider payloads;
- does not include environment values or credentials;
- does not suggest file output, promotion, workflow execution, approval, local
  check execution, or provider commands.

## 6. Tests Added

Focused tests cover:

- default first-run output includes recommendation detail command guidance for
  generic repos;
- default first-run output includes authoring dry-run guidance for generic repo
  implementation;
- TypeScript metadata selects `first_run.typescript_implementation`;
- secret-like package script bodies are not copied;
- default guidance does not suggest draft file output or promotion;
- no runtime state is created.

## 7. Dogfood Governance

- Workflow: `dg/implement`
- Run ID: `run-1783738600799185000-2`
- Approval ID: `approval/run-1783738600799185000-2/implementation-approved`
- Approval presentation ID: `presentation/cec6f35f743b3a9e`
- Approval presentation hash:
  `cec6f35f743b3a9e2f70c77fba37063db179f304ed2db2159c74e6e19e4ad11d`
- Approval outcome: delegated maintainer approved.

## 8. Commands Run And Results

All commands passed:

```sh
cargo test -p workflow-cli --test cli first_run_default_authoring_guidance_selects_typescript_recommendation
cargo test -p workflow-cli --test cli first_run
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

## 9. Remaining Limitations

- Preview JSON is intentionally unchanged in this slice.
- The guidance shows one selected recommendation, not a full command menu.
- File output remains explicit and separate.
- Promotion, steward review, catalog writes, runtime execution, local checks,
  provider calls, report artifacts, and writes remain separate phases.

## 10. Recommended Next Phase

First-run authoring command guidance review.

The review should verify that the output is helpful, bounded, and
non-mutating, and that it does not imply workflow generation, file writing,
promotion, runtime execution, local check execution, provider calls, schemas,
examples, hosted behavior, writes, or release posture changes.
