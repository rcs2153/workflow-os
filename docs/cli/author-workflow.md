# author workflow

`workflow-os author workflow --from-recommendation <id> --dry-run` previews inactive workflow authoring obligations for one existing first-run recommendation.

`workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml` writes one inactive draft workflow file for review.

`workflow-os author workflow preflight --draft workflows/drafts/<name>.workflow.yml` inspects whether an inactive draft has promotion blockers.

This command is an authoring surface. It is designed to help a maintainer or agent understand what would need to be authored before a recommendation can become active governance. File output is explicit and review-only.

## Usage

```sh
workflow-os author workflow --from-recommendation first_run.repo_implementation --dry-run
workflow-os --json author workflow --from-recommendation first_run.assign_ownership --dry-run
workflow-os author workflow --from-recommendation first_run.repo_implementation --output workflows/drafts/repo-implementation.workflow.yml
workflow-os author workflow --from-recommendation first_run.repo_implementation --output workflows/drafts/repo-implementation.workflow.yml --dry-run
workflow-os author workflow preflight --draft workflows/drafts/repo-implementation.workflow.yml
workflow-os --json author workflow preflight --draft workflows/drafts/repo-implementation.workflow.yml
```

`--dry-run` is required unless `--output workflows/drafts/<name>.workflow.yml` is provided.

With `--output`, the path must be a relative draft path under `workflows/drafts/` and must end in `.workflow.yml`. Existing files are never overwritten.

## What It Does

- loads and validates the local Workflow OS project;
- recomputes the bounded first-run recommendation set;
- finds the requested recommendation id;
- builds the existing inactive draft proposal summary;
- prints required authoring decisions, validation expectations, missing fields, non-goals, privacy posture, and next action;
- when `--output` is provided without `--dry-run`, writes one inactive draft file under `workflows/drafts/`;
- when `--output` is provided with `--dry-run`, previews the proposed file path and workflow id without writing.
- when `preflight --draft` is provided, parses one inactive draft in isolation, compares its workflow id against active workflows, validates it as a candidate, and reports bounded blocker/warning codes.

Generated draft files are intentionally review-only. They are nested under `workflows/drafts/`, which the current project loader does not treat as active workflow specs. They also include inactive posture fields such as `disabled_by_default: true`, empty triggers, empty steps, and authoring-obligation comments.

Preflight is also review-only. It does not move a draft into `workflows/`, does not register or activate a workflow, does not create runtime state, and does not approve promotion. Passing preflight means the draft is ready for steward review before any separately planned active-promotion step.

## What It Does Not Do

The command does not:

- register workflows;
- promote or activate workflows;
- execute commands;
- register or execute local checks;
- call providers;
- create runtime state;
- append events;
- inspect raw source contents;
- copy manifest bodies, package script bodies, dependency values, CI logs, provider payloads, parser payloads, environment values, credentials, or token-like values;
- create examples;
- change schemas;
- enable writes.

The command does not write files unless `--output` is explicitly supplied. That write is limited to one inactive draft file under `workflows/drafts/`.

## Failure Behavior

The command fails closed when:

- `--dry-run` is missing;
- `--dry-run` is missing and no `--output` path is supplied;
- `--from-recommendation <id>` is missing;
- the recommendation id is unknown;
- the recommendation id is invalid or secret-like;
- the output path is absolute, traverses outside the draft boundary, contains unsafe or secret-like segments, or does not end in `.workflow.yml`;
- the output path already exists;
- the proposed workflow id conflicts with an active workflow id;
- the preflight draft path is missing, unsafe, or not under `workflows/drafts/`;
- the preflight draft cannot be parsed;
- preflight detects blockers such as a still-draft workflow id, incomplete owner/escalation posture, missing triggers, missing steps, active workflow id conflicts, or validation errors;
- project validation fails;
- proposal construction fails;
- draft writing fails.

Errors are bounded and do not echo unsafe recommendation ids, unsafe output paths, private directory material, or raw repository payloads.

## Compatibility

The JSON output is preview-only through `0.2.0-preview.1`. It is intended for local tooling and tests, not as a stable integration contract.

The draft file format is also preview-only. Draft output is a repository authoring aid, not a stable workflow-generation contract.
