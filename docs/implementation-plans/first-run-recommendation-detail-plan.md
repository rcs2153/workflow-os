# First-Run Recommendation Detail Plan

Status: Implemented as a bounded `workflow-os first-run --recommendation <id>` detail view in [First-Run Recommendation Detail Implementation Report](../concepts/FIRST_RUN_RECOMMENDATION_DETAIL_IMPLEMENTATION_REPORT.md). This follows the accepted first-run recommendation next-action review in [First-Run Recommendation Next-Action Review](../concepts/FIRST_RUN_RECOMMENDATION_NEXT_ACTION_REVIEW.md).

This plan is planning only. It does not implement a new CLI command, UI, SDK surface, workflow generation, workflow registration, command execution, local check execution, provider calls, source-content inspection, schema changes, examples, writes, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

`workflow-os first-run` now emits safe repository metadata, review-only recommendations, and bounded next-action hints. The next product question is how an operator or agent should inspect an individual recommendation deeply enough to author a real Workflow OS workflow, check obligation, ownership change, side-effect posture, or report obligation safely.

The recommendation detail surface should explain why a recommendation exists, what evidence supported it, what is still missing, what the safe next authoring step is, and what Workflow OS explicitly has not done. It should preserve the core first-run boundary: recommendations remain review-only until a human or governed authoring path creates and reviews actual workflow specs.

The first implementation should be a bounded local detail view over already-computed first-run recommendation data. It should not auto-generate workflow files.

## 2. Goals

- Make individual first-run recommendations easier to inspect and act on.
- Explain the recommendation rationale using stable bounded codes.
- Show safe detected metadata that influenced the recommendation.
- Show missing inputs, ownership gaps, validation gaps, side-effect posture gaps, and report/handoff obligations.
- Preserve review-only recommendation semantics.
- Help a maintainer or agent author a workflow manually or through a later governed authoring path.
- Avoid raw source contents, raw command bodies, raw provider payloads, raw CI logs, environment values, credentials, and token-like strings.
- Keep `first-run` deterministic, local, and non-mutating.
- Prepare for future governed workflow authoring without implementing it in this phase.

## 3. Non-Goals

Do not implement in this lane:

- automatic workflow generation;
- automatic workflow registration;
- automatic local check registration;
- command execution;
- local check execution;
- provider calls;
- source-content inspection;
- raw manifest body inspection beyond existing bounded metadata;
- report artifact writing;
- persistence;
- hosted or distributed runtime behavior;
- CLI rendering beyond a future bounded detail surface;
- workflow schema changes;
- examples;
- side-effect execution;
- write-capable adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Recommendation Inputs

The current first-run recommendation model already has enough bounded inputs for a detail view:

- recommendation id;
- recommendation kind;
- target ordinal;
- status;
- bounded summary code;
- rationale codes;
- spec-field coverage codes;
- ownership/escalation issue codes;
- next-action code;
- safe repository metadata labels;
- governance profile and posture fields;
- evidence/check/side-effect/report disclosure posture.

The detail surface should compose those inputs. It should not add raw repository reads or external calls.

## 5. Candidate User Experience

The first implementation should prefer one of these bounded forms:

1. Extend `workflow-os first-run --verbose` with a compact detail block for each recommendation.
2. Add a focused command such as `workflow-os first-run recommendation <recommendation-id>`.
3. Add a `--recommendation <recommendation-id>` filter to `workflow-os first-run`.

Recommended first choice: add a filter-style first-run detail view, such as `workflow-os first-run --recommendation first_run.typescript_implementation`, if it fits current CLI conventions.

Why:

- it keeps the detail surface attached to the first-run command that produced the recommendation;
- it can reuse first-run validation and safe metadata collection;
- it avoids inventing a broader recommendation registry;
- it avoids implying recommendations are persisted or active workflows.

The default first-run output should remain concise. Detail output should be explicitly requested.

## 6. Required Detail Content

For a selected recommendation, show:

- recommendation id;
- recommendation kind;
- recommendation status;
- review-only posture;
- safe summary;
- why it was recommended;
- metadata signals used;
- ownership/escalation dependencies;
- relevant spec-field posture codes;
- suggested next action;
- what must be authored or reviewed before it becomes active;
- what Workflow OS did not do;
- privacy and non-execution boundary.

For workflow candidates, detail should say:

- the workflow does not exist yet unless separately authored;
- detected metadata may inform validation obligations;
- local commands are suggested obligations only and are not executed;
- any future workflow must define owner, escalation, policy, evidence, checks, side effects, and report posture explicitly.

For ownership candidates, detail should say:

- placeholders should be replaced;
- enterprise stewardship and local automation posture are separate concerns;
- this does not implement RBAC, IdP integration, paging, or escalation notifications.

For validation/evidence candidates, detail should say:

- check obligations are not active until configured;
- evidence should cite stable references rather than copy payloads;
- local check execution remains explicit and policy-governed.

For side-effect candidates, detail should say:

- writes remain unsupported;
- side-effect posture should be decided before write-capable adapters;
- side-effect disclosures are governance posture, not provider mutation.

## 7. Output Boundary

The detail surface may print:

- stable recommendation ids;
- static bounded summary strings;
- stable rationale codes;
- stable coverage and posture codes;
- safe metadata labels and counts already exposed by first-run;
- bounded next-action code;
- explicit missing/not-available text.

The detail surface must not print:

- source contents;
- raw manifest bodies;
- raw package script command bodies;
- dependency values;
- CI log content;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings;
- generated workflow YAML.

## 8. Error Handling

Recommended error behavior:

- Unknown recommendation id fails with a stable non-leaking code such as `cli.first_run.recommendation_not_found`.
- Recommendation detail requested when project validation fails should preserve current first-run validation failure behavior.
- Internal construction failures should fail closed with stable non-leaking errors.
- Error messages should not echo user-supplied raw ids beyond bounded validated ids, and should not echo repo metadata values.

## 9. Relationship To Workflow Authoring

Recommendation detail is not workflow authoring.

It can prepare future authoring by making the authoring obligations explicit:

- what workflow/check/policy/report posture should be considered;
- what data supported the suggestion;
- what fields need human choice;
- what side-effect and approval boundaries must be decided;
- what validation should be run after authoring.

Automatic workflow generation should remain deferred until governed workflow authoring and review semantics are planned. A future authoring lane should decide whether generated workflow drafts are allowed, where they are written, how they are reviewed, and how conflicts with existing workflow specs are handled.

## 10. Test Plan For Future Implementation

Future implementation tests should cover:

- default `workflow-os first-run` remains concise;
- detail view for a known recommendation shows id, kind, status, rationale, next action, and review-only posture;
- TypeScript/package recommendation detail uses safe metadata labels only;
- Rust/Python/Go/GitHub Actions detail uses safe bounded labels only;
- ownership recommendation detail shows placeholder posture without raw owner values;
- validation recommendation detail does not copy script command bodies;
- side-effect recommendation detail states writes remain unsupported;
- unknown recommendation id fails closed with a stable non-leaking error;
- detail view does not create runtime state;
- detail view does not execute commands;
- detail view does not generate workflow files;
- preview JSON remains bounded if detail JSON is added;
- existing first-run, scaffold, validation, runtime, and docs tests continue to pass.

## 11. Documentation Plan

Future implementation should update:

- `docs/cli/first-run.md`;
- `ROADMAP.md`;
- a phase report under `docs/concepts/`;
- tests documenting the no-state/no-execution boundary.

Docs must keep saying:

- recommendations are review-only;
- detail output is explanatory, not active workflow state;
- automatic workflow generation is not implemented;
- command execution and local check execution are not implemented by `first-run`;
- provider calls, writes, hosted behavior, schemas, examples, recursive agents, agent swarms, and Level 3/4 autonomy are not implemented by this lane.

## 12. Open Questions

- Should the first implementation use `--recommendation <id>` or a subcommand-style surface?
- Should detail output be human-only first, or should preview JSON include selected recommendation detail too?
- Should detail output include suggested workflow section names, or would that feel too close to automatic generation?
- Should recommendation ids become a stricter public CLI contract in this preview release?
- How should detail output relate to future governed workflow authoring?

## 13. Final Recommendation

Proceed next to a small implementation slice: add an explicit first-run recommendation detail view for already-computed recommendations, local and read-only, with no workflow generation and no command execution.

Use existing first-run validation and metadata collection. Keep the output bounded, review-only, and redaction-safe.
