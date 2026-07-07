# First-Run Recommendation Detail Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a bounded `workflow-os first-run --recommendation <id>` detail surface for already-computed first-run recommendations. It stays inside the approved local, read-only, review-only scope and does not introduce workflow generation, command execution, runtime state, provider calls, writes, schemas, examples, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved implementation scope.

Confirmed in scope:

- bounded human recommendation detail output;
- bounded preview JSON detail output;
- fail-closed unknown recommendation handling;
- focused CLI tests;
- CLI docs, roadmap, plan status, and phase report updates.

No accidental implementation was found for:

- automatic workflow generation;
- workflow registration;
- local check registration;
- command execution;
- local check execution;
- provider calls;
- source-content inspection;
- runtime state creation;
- report artifact writing;
- persistence;
- schema changes;
- examples;
- side-effect execution;
- writes;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 3. CLI Surface Assessment

The selected surface, `workflow-os first-run --recommendation <id>`, is appropriate for the current maturity level.

It keeps recommendation detail attached to the `first-run` command that computes the recommendation set, avoids implying there is a persisted recommendation registry, and preserves existing first-run validation behavior before rendering detail output.

The detail view is explicit enough for a maintainer or agent to understand a recommendation before authoring a real workflow, but it does not cross into automatic authoring.

## 4. Detail Output Assessment

The human output includes the expected bounded fields:

- recommendation id;
- kind;
- target ordinal;
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

The output is intentionally code-oriented rather than prose-heavy. That is acceptable for this slice because the goal is safe inspectability, not a polished authoring wizard.

## 5. JSON Compatibility Assessment

Preview JSON output mirrors the same bounded detail. The JSON shape is sensible for preview use and does not claim stable schema exposure.

No workflow spec schema changes were introduced, and no persistence contract was implied.

## 6. Error Handling Assessment

Unknown recommendation ids fail closed with `cli.first_run.recommendation_not_found`.

The error is stable and non-leaking: it does not echo the unknown caller-supplied recommendation id. Project validation failures continue to use the existing first-run validation boundary before recommendation lookup.

## 7. Privacy And Redaction Assessment

The implementation uses bounded recommendation fields already computed by first-run. It does not print or serialize raw repository payloads.

Review confirmed the detail surface does not copy:

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

Tests include secret-like script and dependency values and verify they are not present in recommendation detail output.

## 8. Behavior Preservation

Default `workflow-os first-run` behavior remains concise.

`workflow-os first-run --verbose` remains the full posture matrix path.

`workflow-os first-run --recommendation <id>` renders one selected recommendation and does not print the full summary or full recommendation list. The command does not create runtime state and does not emit run or approval ids.

## 9. Test Quality Assessment

Focused tests cover:

- bounded text detail for a known recommendation;
- review-only posture;
- safe metadata-signal output;
- JSON detail for ownership recommendation detail;
- unknown recommendation id failure;
- no runtime state creation;
- no diagnostic echo of unknown recommendation id;
- no package script body leakage;
- no dependency-value leakage;
- existing first-run summary, verbose, JSON, metadata, and validation behavior through the broader CLI test set.

No blocker-level test gaps were found.

Non-blocking test follow-ups:

- Add a small focused assertion for Rust/Python/Go/GitHub Actions recommendation detail if future changes expand metadata-specific detail text.
- Consider parsing detail JSON structurally in tests if the preview JSON surface begins to stabilize.

## 10. Documentation Review

Docs correctly state that recommendation detail is implemented and remains explanatory.

Docs continue to state that the lane does not implement:

- automatic workflow generation;
- command execution;
- local check execution;
- provider calls;
- writes;
- hosted behavior;
- schemas;
- examples;
- recursive agents;
- agent swarms;
- Level 3/4 autonomy.

The implementation report and roadmap are aligned with the actual code boundary.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Keep recommendation ids preview-scoped unless and until a future compatibility policy makes them stable.
- Consider a future human-polish pass for detail output once recommendation-to-workflow authoring is planned.
- Add ecosystem-specific detail tests if future implementation adds recommendation-specific prose beyond bounded codes.

## 13. Recommended Next Phase

Recommended next phase: governed workflow authoring planning.

The current first-run loop can now detect safe repo metadata, produce concrete review-only recommendations, show bounded next-action hints, and explain individual recommendations. The next product question is how a maintainer or agent should safely turn a recommendation into a draft workflow/check/ownership update without pretending that generation is already enforced or silently mutating the repository.

That next phase should remain planning only and should preserve the current boundary: no automatic workflow generation, no writes without explicit authoring scope, no command execution, no provider calls, no schemas, no examples, no hosted behavior, and no release posture changes.

## 14. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783394825841921000-2`.
- Approval ID: `approval/run-1783394825841921000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations.
- Scope approved: create the maintainer review document for the first-run recommendation detail implementation.
- Out-of-kernel work disclosed: file editing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 15. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783394825841921000-2 --phase review`: passed with 39 events, 1 approval, 0 retries, and 0 escalations.
