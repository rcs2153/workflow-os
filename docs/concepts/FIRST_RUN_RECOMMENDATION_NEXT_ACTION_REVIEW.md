# First-Run Recommendation Next-Action Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The first-run recommendation next-action refinement improves the usability of metadata-aware recommendations while preserving the correct Workflow OS boundary: recommendations remain review-only hints, not active workflows, generated files, command execution, or provider actions.

## 2. Scope Verification

The phase stayed within the approved scope.

Confirmed in scope:

- bounded `recommendation_next_actions` in default first-run output;
- per-recommendation `next_action` codes in verbose output;
- equivalent preview JSON fields;
- focused first-run tests;
- first-run documentation update;
- roadmap update;
- end-of-phase report.

No accidental implementation found for:

- automatic workflow generation;
- automatic workflow registration;
- command execution by `first-run`;
- local check execution;
- provider calls;
- source-content inspection;
- schema changes;
- examples;
- writes;
- hosted/distributed behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 3. Output Assessment

The new output is appropriately action-oriented.

Default first-run output now tells the operator or agent:

- recommendations are not active workflows until authored and reviewed;
- ownership/stewardship should be addressed first when placeholders remain;
- the most concrete workflow candidate should be reviewed next;
- the most concrete validation/evidence candidate should be reviewed next;
- side-effect posture and report/handoff obligations remain explicit follow-ups.

This addresses the accepted review's concern that metadata-aware recommendations were useful but still required too much interpretation before a user or agent could act on them.

## 4. Recommendation Model Assessment

The implementation remains conservative.

`WorkflowDiscoveryRecommendation` now carries a static `next_action` code derived from the recommendation kind. This is a good fit for the current model because:

- it is bounded vocabulary;
- it does not copy repository content;
- it does not imply the recommendation is already active;
- it can be consumed by agents or future UI surfaces without introducing schemas in this slice;
- it keeps recommendation construction local and deterministic.

The default `recommendation_next_actions` ordering is also reasonable: stewardship first, then workflow candidate, validation candidate, side-effect posture, and report/handoff closure.

## 5. Privacy And Redaction Assessment

The privacy boundary is preserved.

The new fields are static strings derived from recommendation IDs and kinds. They do not copy:

- source contents;
- manifest bodies;
- script command bodies;
- dependency values;
- GitHub Actions workflow names or bodies;
- command output;
- provider payloads;
- environment values;
- credentials or token-like values.

Existing non-leakage tests for first-run metadata still cover the relevant sensitive payload paths, and the new assertions do not weaken those checks.

## 6. Behavior Preservation

The phase does not change first-run execution semantics.

`workflow-os first-run` still:

- validates the local project;
- emits a report-ready context;
- creates no runtime state;
- appends no workflow events;
- runs no checks;
- executes no commands;
- calls no providers;
- generates no workflows.

The optional mock approval/audit demo remains clearly separated from real first-run posture analysis.

## 7. Test Quality Assessment

Test coverage is focused and adequate.

Reviewed coverage includes:

- default output contains the next-action group;
- generic projects select repo implementation and evidence/check candidates;
- TypeScript/package metadata selects TypeScript/package candidates;
- Rust metadata selects Rust candidates;
- verbose recommendation rows include `next_action` codes;
- preview JSON includes per-recommendation `next_action` codes;
- preview JSON includes the bounded `recommendation_next_actions` array;
- existing first-run no-state and non-leakage tests still pass.

Non-blocking gap: future tests could cover Python-only, Go-only, and GitHub-Actions-only next-action candidate selection explicitly. The current broader-ecosystem test proves those recommendations exist, while Rust verifies candidate prioritization for the multi-ecosystem case.

## 8. Documentation Review

Docs accurately describe the new behavior.

Reviewed docs state:

- recommendation next actions are guidance only;
- recommendations are not active workflows until authored and reviewed;
- next-action codes do not create files, mutate state, approve gates, or execute checks;
- first-run does not execute commands, call providers, generate workflows, register checks, write files, or change release posture.

The roadmap links the new phase report and preserves the surrounding no-overclaim language.

## 9. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783392272403906000-2`.
- Approval ID: `approval/run-1783392272403906000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the complete approval handoff block was surfaced.
- Scope: first-run recommendation next-action refinement review.

## 10. Validation Commands Run

- Focused implementation, test, docs, and roadmap inspection: passed.
- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783392272403906000-2 --phase review ...`: passed with 39 events, 1 approval, 0 retries, and 0 escalations.

Prior implementation validation recorded in the phase report:

- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add explicit Python-only, Go-only, and GitHub-Actions-only next-action candidate tests if future edits touch first-run recommendation ranking.
- Consider a future bounded recommendation detail command or view, but only after deciding whether it belongs in CLI or a later UI/SDK surface.
- Keep automatic workflow generation out of scope until governed workflow authoring and review semantics are separately planned.

## 13. Recommended Next Phase

Recommended next phase: first-run recommendation detail planning.

The current output now tells users what to review next, but the next product question is how to inspect an individual recommendation in enough detail to author a real workflow safely. That should be planned before adding any workflow generation, workflow registration, or command execution behavior.
