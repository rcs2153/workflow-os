# Governed Workflow Draft Proposal Implementation Report

## 1. Executive Summary

This phase implements the first governed workflow authoring slice: an internal inactive draft proposal helper for first-run recommendations.

The helper is model/helper-only. It is used by `workflow-os first-run --recommendation <id>` to show a bounded inactive proposal summary with required authoring decisions, validation expectations, missing fields, non-goals, and privacy posture.

It does not write files, register workflows, execute commands, create runtime state, call providers, generate active workflow specs, or promote recommendations.

## 2. Scope Completed

- Added an internal `GovernedWorkflowDraftProposal` model in the CLI first-run recommendation boundary.
- Added `governed_workflow_draft_proposal_from_recommendation`.
- Added bounded validation for source recommendation ids.
- Added inactive proposal fields to human recommendation detail output.
- Added inactive proposal fields to preview JSON recommendation detail output.
- Added focused unit tests for inactive proposal shape, secret-like id rejection, side-effect non-write posture, and report/handoff closure obligations.
- Added focused CLI tests for human and JSON proposal output.
- Updated CLI docs and the authoring plan status.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- workflow file generation;
- workflow registration;
- active workflow promotion;
- repository file writes;
- local check registration;
- local check execution;
- command execution;
- provider calls;
- runtime state creation;
- catalog storage;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Helper/API Summary

The helper consumes one already-computed `WorkflowDiscoveryRecommendation` and returns an inactive `GovernedWorkflowDraftProposal`.

The proposal includes:

- source recommendation id;
- inactive status;
- draft lifecycle posture;
- proposal kind;
- proposed purpose code;
- required authoring decisions;
- validation expectations;
- missing required fields;
- explicit non-goals;
- privacy boundary.

The helper rejects unsafe recommendation ids with `cli.workflow_authoring.unsafe_payload_rejected` without echoing the unsafe id.

## 5. User-Facing Behavior

`workflow-os first-run --recommendation <id>` now includes an inactive draft proposal summary.

This is intentionally not a new authoring command. The output helps a maintainer or agent see what would need to be authored before a recommendation can become active governance.

## 6. Privacy And Redaction Summary

The helper uses bounded recommendation fields and static Workflow OS vocabulary only.

It does not copy:

- source contents;
- manifest bodies;
- package script command bodies;
- dependency values;
- CI logs;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings.

Secret-like recommendation ids are rejected without leaking the supplied id.

## 7. Test Coverage Summary

Added or updated tests cover:

- inactive draft proposal shape;
- required authoring decisions;
- validation expectations;
- missing required fields;
- explicit non-goals;
- secret-like recommendation id rejection without leakage;
- side-effect proposals do not enable writes;
- report/handoff proposals require closure obligations;
- human recommendation detail includes inactive proposal fields;
- preview JSON includes inactive proposal fields;
- no runtime state creation from detail output.

## 8. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783396058363156000-2`.
- Approval ID: `approval/run-1783396058363156000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations.
- Scope: model/helper-only inactive draft proposals for recommendation-to-workflow authoring.

## 9. Validation Commands Run

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783396058363156000-2 --phase implementation`: passed with 39 events, 1 approval, 0 retries, and 0 escalations.

## 10. Remaining Known Limitations

- No standalone authoring CLI command exists yet.
- No draft proposal persistence exists.
- No workflow file-writing path exists.
- No promotion or activation path exists.
- No catalog conflict detection exists beyond planned future posture.

## 11. Recommended Next Phase

Recommended next phase: governed workflow draft proposal implementation review.

The helper is small but user-facing through recommendation detail output, so it should be reviewed before any future authoring CLI, file-writing, catalog, or promotion work.
