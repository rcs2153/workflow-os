# Governed Workflow Authoring File Output Plan

Status: Implemented in [Governed Workflow Authoring File Output Implementation Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_FILE_OUTPUT_IMPLEMENTATION_REPORT.md).

This plan follows the accepted dry-run implementation reviewed in [Governed Workflow Authoring CLI Dry-Run Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_CLI_DRY_RUN_IMPLEMENTATION_REVIEW.md). It defined the explicit, inactive draft workflow file-output boundary for `workflow-os author workflow`.

The implementation writes only explicit inactive draft files under `workflows/drafts/`. It does not implement workflow registration, workflow promotion, command execution, provider calls, runtime state creation, schemas, examples, hosted behavior, write-capable adapters, or release posture changes.

## 1. Executive Summary

`workflow-os author workflow --from-recommendation <id> --dry-run` now lets an operator preview inactive workflow authoring obligations from a bounded first-run recommendation.

The next product question is whether Workflow OS should support an explicit file-output path that writes an inactive draft workflow file for review. This would be the first authoring feature that mutates repository files, so it needs a stricter boundary than the dry-run preview.

The implemented file-output command is opt-in and writes one inactive draft workflow file only when the caller explicitly requests an output path under `workflows/drafts/`. The draft remains inactive, preserves existing files by default, fails closed on conflicts, and never registers, promotes, executes, or activates the workflow.

## 2. Goals

- Provide a safe bridge from recommendation preview to reviewable draft workflow files.
- Keep workflow authoring deterministic, local, and explicit.
- Preserve existing repository files by default.
- Write inactive draft files only through an opt-in command.
- Require the source recommendation id.
- Reuse the accepted inactive draft proposal helper.
- Require conflict checks before writing.
- Make owner, escalation, policy, evidence/check, side-effect, and report/handoff obligations visible in the draft.
- Keep drafts review-only until a future promotion path exists.
- Avoid raw source contents, raw command output, provider payloads, parser payloads, environment values, credentials, and token-like strings.
- Prepare future steward review and promotion without implementing them.

## 3. Non-Goals

Do not implement in this phase or the first file-output slice:

- automatic workflow generation;
- active workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- command execution;
- local check execution;
- provider calls;
- runtime state creation;
- approval decisions;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Recommended CLI Shape

Implemented command:

```sh
workflow-os author workflow \
  --from-recommendation <id> \
  --output workflows/drafts/<workflow-id>.workflow.yml
```

Implemented dry-run for the same file-output path:

```sh
workflow-os author workflow \
  --from-recommendation <id> \
  --output workflows/drafts/<workflow-id>.workflow.yml \
  --dry-run
```

The current dry-run-only command should continue to work:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

The first file-output implementation does not invent interactive prompts. Explicit flags keep the boundary auditable and testable.

## 5. Output Location Policy

The preferred output location is an explicit caller-provided path under a draft-only area such as:

```text
workflows/drafts/<workflow-id>.workflow.yml
```

Rules:

- require a relative path inside the project directory;
- reject absolute paths;
- reject parent-directory traversal;
- reject paths outside `workflows/` or a future approved draft directory;
- reject paths that do not end in `.workflow.yml`;
- reject existing files unless an explicit future `--replace-draft` flag is planned and approved;
- create parent draft directories only if that is explicitly in scope for the implementation;
- never overwrite unmanaged existing workflow files.

## 6. Draft Lifecycle Policy

Generated draft workflow files must be inactive.

Implemented draft posture:

- lifecycle status: `experimental`, because the current v0 workflow schema does not define a `draft` lifecycle status;
- draft marker: nested path under `workflows/drafts/`, `disabled_by_default: true`, empty triggers, empty steps, and explicit authoring-obligation comments;
- owner and escalation: explicit placeholders or required fields, never fabricated real people;
- policy gates: bounded suggested gates from proposal vocabulary;
- evidence/check obligations: bounded labels only;
- side-effect posture: `none`, `skipped`, or `unsupported` unless explicitly authored later;
- report/handoff posture: required obligations, not generated final reports;
- execution posture: not loaded by the current project loader while nested under `workflows/drafts/`; if moved into the active workflow directory, the empty triggers/steps posture fails validation until promotion authors the missing fields.

The generated file should include comments or fields that make inactive status clear only if comments are already acceptable in repository YAML conventions. If comments are used, they must not become the only enforcement mechanism.

## 7. Conflict Handling

The implementation fails closed on conflicts.

Required checks before writing:

- output path already exists;
- output path is outside the allowed draft boundary;
- output path is absolute or contains traversal;
- workflow id already exists in loaded project workflows;
- proposed workflow id is invalid;
- proposed workflow purpose conflicts with an existing active workflow if a deterministic check exists;
- recommendation id is unknown or unsafe;
- proposal helper rejects the recommendation;
- generated content would require raw payload copying.

If conflict detection is incomplete, the command must disclose what was not checked and keep the output inactive. It must not silently write active governance.

## 8. Input Policy

The first file-output implementation accepts only:

- `--from-recommendation <id>`;
- `--output <relative-path>`;
- `--dry-run` for preview mode.

Defer:

- owner values;
- escalation values;
- policy body input;
- command/check input;
- provider identifiers;
- approval assignments;
- raw YAML snippets;
- natural-language workflow bodies.

Those inputs need separate validation and redaction rules before being accepted.

## 9. Draft Content Policy

Allowed content:

- stable Workflow OS schema version;
- proposed workflow id if explicitly supplied and validated;
- source recommendation id;
- lifecycle status `draft`;
- bounded purpose code;
- required authoring decisions as structured obligations;
- validation expectation labels;
- missing required field labels;
- non-goal labels;
- side-effect posture labels;
- report/handoff obligation labels.

Forbidden content:

- raw source contents;
- raw package script command bodies;
- raw dependency values;
- raw CI logs;
- provider payloads;
- issue or pull request bodies;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, token-like strings;
- existing agent instruction bodies.

## 10. Promotion Boundary

File output must not mean promotion.

Promotion remains future and should require:

- project validation;
- draft workflow validation;
- explicit steward or delegated maintainer approval;
- owner and escalation completion;
- policy/evidence/check posture completion;
- side-effect posture completion;
- report/handoff posture completion;
- conflict checks against existing workflows;
- an auditable work report or equivalent governed handoff.

The first file-output slice must not add promotion flags.

## 11. Error Handling

Errors must use stable codes and avoid leaking unsafe values.

Candidate error codes:

- `cli.workflow_authoring.output_required`;
- `cli.workflow_authoring.output_path_rejected`;
- `cli.workflow_authoring.output_exists`;
- `cli.workflow_authoring.workflow_id_required`;
- `cli.workflow_authoring.workflow_id_invalid`;
- `cli.workflow_authoring.workflow_id_conflict`;
- `cli.workflow_authoring.draft_write_failed`;
- `cli.workflow_authoring.unsafe_payload_rejected`.

Errors must not echo raw paths if a path looks secret-like or contains private directory material. Prefer bounded labels such as `output_path_rejected`.

## 12. Privacy And Redaction

The file-output path must remain safe for public repositories.

Requirements:

- use bounded safe metadata only;
- write no raw source, manifest, script, dependency, CI, provider, parser, environment, or credential payloads;
- avoid private absolute paths;
- reject secret-like ids and output path segments;
- avoid embedding current user names, emails, machine names, home directories, or local temp paths;
- keep generated draft content review-only and inactive.

## 13. Documentation Requirements

The implementation updated:

- CLI docs for `author workflow`;
- governed workflow authoring plan;
- roadmap;
- implementation report.

The command remains preview-stage. Runtime registration, promotion, active workflow generation, and schema changes remain deferred.

Docs must state:

- file output is explicit and opt-in;
- drafts are inactive;
- no workflow registration is performed;
- no promotion is performed;
- no commands, providers, checks, or writes beyond the explicit draft file are run;
- no runtime state is created;
- examples, schemas, hosted behavior, write-capable adapters, and release posture changes remain unimplemented.

## 14. Test Plan

Future implementation tests should cover:

- dry-run without output still works as today;
- output path is required for file-writing mode;
- absolute output path is rejected;
- traversal output path is rejected;
- unsupported extension is rejected;
- existing output file is not overwritten;
- unknown recommendation id fails closed without leakage;
- secret-like recommendation id fails closed without leakage;
- invalid workflow id is rejected if `--workflow-id` is added;
- duplicate workflow id is rejected;
- generated draft is lifecycle `draft`;
- generated draft includes required authoring obligations;
- generated draft includes side-effect and report/handoff obligations;
- generated draft is not registered in runtime state;
- generated draft does not execute commands;
- generated draft does not call providers;
- generated draft does not copy raw package script bodies;
- generated draft does not copy source contents;
- generated draft does not copy dependency values;
- generated draft validates only if the intended draft schema posture supports it;
- existing CLI, validation, scaffold, runtime, and docs tests still pass.

## 15. Proposed Implementation Sequence

1. Add output path validation helper.
2. Add workflow id validation or require explicit workflow id only if existing primitives support it.
3. Add deterministic inactive draft rendering from the existing proposal helper.
4. Add dry-run preview for file-output mode.
5. Add write path that fails closed on existing files and unsafe paths.
6. Add focused tests for path safety, non-overwrite, inactivity, and non-leakage.
7. Update docs and create an end-of-phase report.
8. Review before any promotion or registration planning.

## 16. Open Questions

- Should the first file-output slice require `--workflow-id`, or derive a bounded suggested id from the recommendation?
- Should generated draft files live under `workflows/drafts/` or directly under `workflows/` with `lifecycle_status: draft`?
- Should parent directory creation be allowed, or must the draft directory already exist?
- Should the first implementation write YAML comments, or only structured fields?
- Should generated drafts be valid workflow specs immediately, or a separate proposal artifact until promotion exists?
- How should enterprise stewardship alter defaults for owner, escalation, and approval posture?

## 17. Final Recommendation

Proceed next to governed workflow authoring file-output implementation only after this plan is reviewed.

The first implementation should be narrow: explicit output path, inactive draft only, path safety checks, no overwrite, no registration, no promotion, no command execution, no provider calls, no runtime state, no schemas, no examples, no hosted behavior, no write-capable adapters, and no release posture changes.
