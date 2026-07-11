# First-Run Authoring Command Guidance Plan

## 1. Executive Summary

Real-repository testing shows that `workflow-os first-run` is the strongest
first-use product signal, but the bridge from recommendations to reviewed
workflow authoring remains under-presented in default human output.

Workflow OS already has the necessary authoring surfaces:

- `workflow-os first-run --recommendation <id>`;
- `workflow-os author workflow --from-recommendation <id> --dry-run`;
- explicit inactive draft output under `workflows/drafts/`;
- preflight, steward review, promotion, and catalog-status commands.

The next implementation should make the first two non-mutating commands visible
directly from `first-run` output. This plan does not implement workflow
generation, active workflow registration, command execution, provider calls,
schemas, examples, hosted behavior, writes, or release posture changes.

## 2. Goals

- Make the recommendation-to-authoring bridge obvious in default human
  `first-run` output.
- Show copyable, concrete next commands for recommendation detail and authoring
  dry-run.
- Preserve the review-only nature of recommendations.
- Preserve the non-mutating nature of recommendation detail and authoring
  dry-run.
- Avoid suggesting that Workflow OS has already generated, activated, or
  validated a production workflow.
- Keep output bounded and safe for real repositories.
- Keep preview JSON bounded and explicit.
- Improve the first 10-minute product loop without adding runtime behavior.

## 3. Non-Goals

Do not implement:

- automatic workflow generation;
- active workflow registration;
- automatic draft file writing;
- automatic workflow promotion;
- local command execution;
- local check handler registration;
- provider calls;
- runtime state creation from `first-run`;
- report artifact writing;
- schema changes;
- examples;
- hosted or distributed behavior;
- write-capable adapters;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Behavior

`workflow-os first-run` emits concise default output, detailed verbose output,
preview JSON, and recommendation next-action codes. It already separates the
real first-run posture analysis from the optional mock approval/audit demo.

The current authoring bridge exists, but users must already know the commands:

```sh
workflow-os first-run --recommendation <id>
workflow-os author workflow --from-recommendation <id> --dry-run
```

That is too implicit for new users who just saw their first recommendation.

## 5. Proposed UX

Default `first-run` human output should include a short `try_next` or equivalent
section with concrete commands for the top review-only recommendation.

Suggested shape:

```text
try_next:
  - inspect_recommendation: workflow-os first-run --recommendation first_run.typescript_implementation
  - preview_authoring: workflow-os author workflow --from-recommendation first_run.typescript_implementation --dry-run
```

If no concrete workflow recommendation exists, the section should point at the
most important available setup recommendation, such as ownership assignment, and
avoid authoring commands that would be misleading.

## 6. Recommendation Selection Policy

Select one bounded recommendation for default command guidance.

Recommended order:

1. concrete ecosystem implementation workflow recommendation, such as
   TypeScript, Rust, Python, or Go implementation;
2. generic repo implementation workflow recommendation;
3. package or ecosystem validation-obligation recommendation;
4. ownership/stewardship setup recommendation;
5. report handoff obligations.

The chosen recommendation must already exist in the computed first-run
recommendation set. The CLI must not fabricate recommendation IDs.

## 7. Command Guidance Rules

The output may show:

- `workflow-os first-run --recommendation <id>`;
- `workflow-os author workflow --from-recommendation <id> --dry-run`;
- optional text explaining that `--output workflows/drafts/<name>.workflow.yml`
  is a separate explicit file-writing step.

The output must not show:

- a command that writes a draft by default;
- a command that promotes a workflow by default;
- a command that runs a workflow;
- a command that approves a checkpoint;
- a command that executes local checks;
- provider commands;
- shell commands inferred from package scripts;
- raw package script bodies;
- raw source paths beyond already-bounded metadata posture.

## 8. JSON Posture

Preview JSON should expose bounded machine-readable guidance if the current JSON
shape already has a natural place for recommendation next actions.

Suggested fields:

- selected recommendation id;
- inspect command tokens or bounded command string;
- authoring dry-run command tokens or bounded command string;
- posture value such as `review_only_non_mutating`.

If adding JSON shape is too broad for the implementation slice, default human
output may be implemented first and JSON can remain unchanged with a documented
follow-up.

## 9. Safety And Redaction

Command guidance must be constructed only from validated recommendation IDs and
static command tokens.

It must not include:

- raw manifest contents;
- package script bodies;
- dependency values;
- source file contents;
- command output;
- provider payloads;
- environment values;
- credentials;
- token-like values;
- caller-supplied free-form text.

Errors must remain stable and non-leaking.

## 10. Tests

Future implementation should add focused tests for:

- default `first-run` output includes recommendation detail command guidance
  when a concrete recommendation exists;
- default `first-run` output includes authoring dry-run command guidance when a
  workflow-authoring recommendation exists;
- output does not include draft-writing, promotion, run, approval, local check,
  provider, or shell commands;
- output uses an existing recommendation id only;
- TypeScript metadata selects a concrete TypeScript recommendation;
- generic repositories fall back to repo implementation or ownership setup;
- `--verbose` preserves existing detailed posture while including or clearly
  locating command guidance;
- preview JSON is either intentionally unchanged or includes bounded command
  guidance;
- secret-like or raw payload markers are not printed;
- existing first-run, author-workflow, validation, and CLI tests still pass.

## 11. Documentation Updates

Update:

- `docs/cli/first-run.md`;
- `docs/cli/author-workflow.md` if needed;
- `docs/user-guide/current-product-contract.md` if the visible first-use loop
  changes;
- `ROADMAP.md`;
- a phase report under `docs/concepts/`.

Docs must say:

- recommendation command guidance is implemented only when it is implemented;
- recommendations remain review-only;
- authoring dry-run remains non-mutating;
- file output remains explicit;
- active promotion remains separate;
- no runtime execution, provider calls, local check execution, schemas, examples,
  writes, hosted behavior, or release posture changes are introduced.

## 12. Proposed Implementation Sequence

1. Add a small helper that selects one existing recommendation for command
   guidance.
2. Render bounded command guidance in default human `first-run` output.
3. Decide whether verbose output reuses the same section or keeps guidance in
   the concise summary only.
4. Decide whether preview JSON includes bounded guidance in the same PR or a
   follow-up.
5. Add focused CLI tests.
6. Update docs and create the implementation report.
7. Run full validation.

## 13. Open Questions

- Should preview JSON include command strings or token arrays?
- Should default output show one command pair or multiple top commands?
- Should ownership/stewardship recommendations suppress authoring dry-run
  guidance until owner/escalation placeholders are resolved?
- Should the command guidance include `--output` as an optional later step, or
  is that too close to suggesting file writes?
- Should the selected recommendation be stable across metadata changes, or is
  changing guidance acceptable when safer concrete metadata is detected?

## 14. Final Recommendation

Proceed to a small implementation phase: add default `first-run` command
guidance for recommendation detail and authoring dry-run, using existing
recommendation IDs only.

Still do not build automatic workflow generation, file writes by default,
promotion, runtime execution, local checks, provider calls, schemas, examples,
writes, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or
release posture changes.
