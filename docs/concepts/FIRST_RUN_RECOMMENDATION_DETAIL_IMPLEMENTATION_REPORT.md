# First-Run Recommendation Detail Implementation Report

## 1. Executive Summary

This phase implements a bounded first-run recommendation detail view:

```sh
workflow-os first-run --recommendation <recommendation-id>
workflow-os --json first-run --recommendation <recommendation-id>
```

The detail view explains one already-computed first-run recommendation using stable ids, rationale codes, metadata-signal codes, coverage codes, ownership issue codes, and next-action codes. It remains local, read-only, and review-only.

## 2. Scope Completed

- Added `first-run --recommendation <id>` parsing.
- Added bounded text output for one selected recommendation.
- Added bounded preview JSON output for one selected recommendation.
- Added fail-closed handling for unknown recommendation ids.
- Added focused tests for text output, JSON output, metadata non-leakage, unknown ids, and no runtime state creation.
- Updated CLI docs, roadmap, and the recommendation detail plan status.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic workflow generation;
- workflow registration;
- local check registration;
- command execution;
- local check execution;
- provider calls;
- source-content inspection;
- report artifact writing;
- persistence;
- schema changes;
- examples;
- side-effect execution;
- writes;
- hosted behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Behavior Added

`workflow-os first-run --recommendation <id>` now prints:

- recommendation id;
- kind;
- target;
- status;
- review-only posture;
- summary code;
- rationale codes;
- metadata-signal codes;
- coverage codes;
- ownership issue codes;
- next-action code;
- authoring requirement;
- explicit non-execution boundary;
- privacy boundary.

`workflow-os --json first-run --recommendation <id>` emits the same bounded detail in preview JSON.

## 5. Error Handling

Unknown recommendation ids fail closed with `cli.first_run.recommendation_not_found`.

The error does not echo the unknown id. The command still validates the project before producing detail output, preserving current first-run validation behavior.

## 6. Privacy And Redaction Summary

The detail view prints bounded codes only. It does not copy:

- source contents;
- raw manifest bodies;
- raw package script command bodies;
- dependency values;
- CI logs;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings.

Focused tests cover secret-like package script and dependency values.

## 7. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783393627553708000-2`.
- Approval ID: `approval/run-1783393627553708000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after full approval handoff review.
- Scope: first-run recommendation detail implementation.

## 8. Test Coverage Summary

Added or updated tests cover:

- bounded text detail for `first_run.repo_implementation`;
- bounded metadata detail for `first_run.typescript_implementation`;
- bounded preview JSON detail for `first_run.assign_ownership`;
- unknown recommendation id failure without state creation;
- existing first-run summary, verbose, JSON, metadata, and non-leakage behavior.

## 9. Validation Commands Run

- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783393627553708000-2 --phase implementation`: passed with 39 events, 1 approval, 0 retries, and 0 escalations.

## 10. Remaining Known Limitations

- Recommendation detail remains a CLI preview-era surface.
- It explains recommendations but does not author workflow specs.
- It does not persist recommendation state.
- It does not provide interactive remediation or an authoring wizard.

## 11. Recommended Next Phase

Recommended next phase: first-run recommendation detail review.

The implementation is small and user-facing enough to warrant a maintainer review before considering any workflow authoring or generation planning.
