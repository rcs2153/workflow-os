# Governed Workflow Authoring CLI Dry-Run Plan

Status: Planning only.

This plan follows the accepted inactive draft proposal helper reviewed in [Governed Workflow Draft Proposal Implementation Review](../concepts/GOVERNED_WORKFLOW_DRAFT_PROPOSAL_IMPLEMENTATION_REVIEW.md). It defines the next authoring boundary: an explicit CLI dry-run surface that can preview draft workflow authoring obligations without writing files, registering workflows, executing commands, calling providers, creating runtime state, changing schemas, adding examples, enabling hosted behavior, or enabling writes.

## 1. Executive Summary

`workflow-os first-run --recommendation <id>` now exposes inactive draft proposal obligations for review-only workflow recommendations.

The next product question is whether a maintainer or agent should be able to ask for a more explicit authoring preview from the CLI without making manual YAML authoring the only path and without silently mutating a repository.

The recommended next implementation is a dry-run-only CLI surface that turns one existing first-run recommendation into a bounded inactive authoring preview. The preview should make proposed workflow identity, required decisions, validation expectations, missing fields, non-goals, and privacy posture clearer. It must not create workflow files or active governance.

## 2. Goals

- Provide a clear CLI path from recommendation review to inactive authoring preview.
- Keep the preview deterministic, local, and non-mutating.
- Reuse the existing inactive draft proposal helper.
- Require explicit recommendation input.
- Make missing owner, escalation, evidence/check, policy, side-effect, and report/handoff decisions visible.
- Make the inactive status unmistakable.
- Preserve safe metadata boundaries.
- Prepare for a later file-writing plan without implementing file writing.
- Preserve compatibility with future steward review and workflow catalog governance.

## 3. Non-Goals

Do not implement in this phase:

- workflow file generation;
- repository file writes;
- workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- runtime state creation;
- local command execution;
- local check registration;
- local check execution;
- provider calls;
- hidden edits to `workflows/`, `skills/`, `policies/`, or tests;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Current State

The implemented helper is internal to the CLI first-run recommendation boundary.

Current user path:

```sh
workflow-os first-run
workflow-os first-run --recommendation <id>
```

The detail view shows the inactive proposal summary, but there is no dedicated authoring command and no stronger CLI affordance for users who want to move from "review this recommendation" to "show me the draft I would need to author."

## 5. Recommended CLI Shape

Recommended first dry-run command:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

Rationale:

- `author workflow` says the command is about workflow authoring, not workflow execution.
- `--from-recommendation` ties the command to an existing bounded first-run signal.
- `--dry-run` makes non-mutation explicit and should be required in the first implementation.

Acceptable alternative if the existing parser favors flatter commands:

```sh
workflow-os author-workflow --from-recommendation <id> --dry-run
```

The implementation should pick the smallest idiomatic parser shape for the current CLI, but the user-facing docs should keep the command clearly separated from `run`.

## 6. Inputs

Required inputs:

- recommendation id from `workflow-os first-run` output;
- explicit `--dry-run`;
- current project directory through existing CLI project resolution.

Optional future inputs, deferred unless trivial:

- `--json` for preview JSON;
- `--proposal-id` if proposal identity becomes useful before catalog storage;
- `--workflow-id` only after workflow id validation and conflict posture are designed.

The first implementation should not accept raw owner, escalation, command, policy, or YAML body values. Those belong in a later file-writing or interactive authoring phase after validation, redaction, and conflict rules are designed.

## 7. Output Policy

Human output should include:

- inactive proposal status;
- source recommendation id;
- proposed purpose code;
- proposed lifecycle posture;
- required authoring decisions;
- validation expectations;
- missing required fields;
- explicit non-goals;
- privacy boundary;
- next action.

Output must state:

- no files were written;
- no workflow was registered;
- no commands were executed;
- no providers were called;
- no runtime state was created;
- this is not active governance.

Suggested next action:

```text
next_action: review this preview, fill required authoring decisions in a separate implementation phase, then validate before promotion
```

## 8. JSON Output

If `--json` is included in the first implementation, it should be preview-only and clearly marked unstable.

Recommended JSON fields:

- `schema_version`;
- `mode: "author_workflow_dry_run"`;
- `status: "preview_only"`;
- `proposal`;
- `non_mutation`;
- `next_action`.

The JSON should reuse the existing proposal shape where possible and avoid introducing a durable public schema contract before compatibility rules exist.

## 9. Validation And Error Handling

The dry-run command should fail closed when:

- `--dry-run` is missing;
- the recommendation id is missing;
- the recommendation id is unknown;
- the recommendation id is invalid or secret-like;
- project loading or first-run recommendation derivation fails;
- the proposal helper rejects the recommendation;
- output would require raw payload copying.

Errors must use stable codes and avoid echoing unsafe input values.

Candidate error codes:

- `cli.workflow_authoring.dry_run_required`;
- `cli.workflow_authoring.recommendation_required`;
- `cli.workflow_authoring.recommendation_not_found`;
- `cli.workflow_authoring.unsafe_payload_rejected`;
- `cli.workflow_authoring.preview_failed`.

## 10. Privacy And Redaction

The command must use bounded safe metadata only.

It must not print or copy:

- raw source contents;
- manifest bodies;
- package script command bodies;
- dependency values;
- lockfile contents;
- CI logs;
- provider payloads;
- issue or pull request bodies;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings;
- existing agent instruction bodies.

If a repository has useful but private local metadata, the dry-run output should use labels and counts rather than payloads.

## 11. Behavior Preservation

The dry-run command must not change existing behavior for:

- `workflow-os first-run`;
- `workflow-os first-run --recommendation <id>`;
- `workflow-os validate`;
- `workflow-os run`;
- scaffold commands.

It must not create `.workflow-os/state`, append events, or touch workflow runtime state.

## 12. Test Plan

Future implementation tests should cover:

- command requires `--dry-run`;
- missing recommendation id fails closed;
- unknown recommendation id fails closed without leaking the id;
- secret-like recommendation id fails closed without leakage;
- known recommendation produces inactive authoring preview;
- preview includes required authoring decisions;
- preview includes missing owner and escalation obligations;
- preview includes evidence/check obligations;
- preview includes side-effect posture obligation;
- preview includes report/handoff obligation;
- preview includes explicit non-goals;
- preview states no files were written;
- preview states no workflow was registered;
- preview states no commands were executed;
- preview states no providers were called;
- no runtime state is created;
- no workflow, skill, policy, or test files are written;
- raw package script bodies are not copied;
- source contents are not copied;
- dependency values are not copied;
- JSON output is bounded if included;
- existing first-run recommendation detail tests still pass;
- existing CLI, scaffold, validation, runtime, and docs tests still pass.

## 13. Proposed Implementation Sequence

1. Add a CLI dry-run parser path for workflow authoring.
2. Require `--from-recommendation <id>` and `--dry-run`.
3. Recompute or reuse first-run recommendation data through the existing safe metadata boundary.
4. Reuse the existing inactive draft proposal helper.
5. Render bounded human preview output.
6. Add JSON preview only if it is small and clearly marked preview-only.
7. Add focused CLI tests.
8. Update docs and create an end-of-phase report.
9. Review before any file-writing or promotion planning.

## 14. Deferred Work

- Draft workflow file output.
- Conflict checks against existing workflow ids and authority surfaces.
- Owner/escalation input handling.
- Policy/evidence/check input handling.
- Workflow catalog storage.
- Workflow promotion and activation.
- Steward approval workflow for accepting proposals.
- Schema exposure.
- Examples.
- Runtime execution of generated workflows.

## 15. Open Questions

- Should the first CLI implementation support `--json`, or keep the first slice human-only?
- Should proposed workflow ids be generated, omitted, or required later?
- Should dry-run output include YAML-like preview, or only structured obligation text?
- How should future authoring collect owner and escalation values without leaking private identity data?
- When should catalog conflict checks become mandatory?
- What is the smallest promotion path that preserves steward review without making authoring feel manual again?

## 16. Final Recommendation

Proceed next to governed workflow authoring CLI dry-run implementation.

The implementation should add an explicit non-mutating command, require `--dry-run`, consume one bounded first-run recommendation id, reuse the existing inactive proposal helper, write no files, register no workflows, execute no commands, call no providers, create no runtime state, and preserve the review-only boundary.
