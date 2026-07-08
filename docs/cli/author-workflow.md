# author workflow

`workflow-os author workflow --from-recommendation <id> --dry-run` previews inactive workflow authoring obligations for one existing first-run recommendation.

`workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml` writes one inactive draft workflow file for review.

`workflow-os author workflow preflight --draft workflows/drafts/<name>.workflow.yml` inspects whether an inactive draft has promotion blockers.

`workflow-os author workflow steward-review --draft workflows/drafts/<name>.workflow.yml --decision <decision> --reviewer <actor> --reason <reason>` previews steward review of a preflight-passing inactive draft.

`workflow-os author workflow promote --draft workflows/drafts/<name>.workflow.yml --reviewer <actor> --reason <reason>` promotes one reviewed draft into the active `workflows/` surface.

`workflow-os author workflow catalog-status [--catalog-root <path>] [--strict-catalog-coverage]` inspects active workflow, draft, archived draft, and optional local catalog-store inventory without writing files.

This command is an authoring surface. It is designed to help a maintainer or agent understand what would need to be authored before a recommendation can become active governance. Draft file output is explicit and review-only; active promotion is a separate explicit mutation boundary.

## Usage

```sh
workflow-os author workflow --from-recommendation first_run.repo_implementation --dry-run
workflow-os --json author workflow --from-recommendation first_run.assign_ownership --dry-run
workflow-os author workflow --from-recommendation first_run.repo_implementation --output workflows/drafts/repo-implementation.workflow.yml
workflow-os author workflow --from-recommendation first_run.repo_implementation --output workflows/drafts/repo-implementation.workflow.yml --dry-run
workflow-os author workflow preflight --draft workflows/drafts/repo-implementation.workflow.yml
workflow-os --json author workflow preflight --draft workflows/drafts/repo-implementation.workflow.yml
workflow-os author workflow steward-review --draft workflows/drafts/repo-implementation.workflow.yml --decision approved-for-promotion --reviewer user/workflow-steward --reason bounded-review-reason
workflow-os --json author workflow steward-review --draft workflows/drafts/repo-implementation.workflow.yml --decision needs-changes --reviewer user/workflow-steward --reason bounded-review-reason
workflow-os author workflow promote --draft workflows/drafts/repo-implementation.workflow.yml --reviewer user/workflow-steward --reason bounded-review-reason --dry-run
workflow-os --json author workflow promote --draft workflows/drafts/repo-implementation.workflow.yml --reviewer user/workflow-steward --reason bounded-review-reason
workflow-os author workflow catalog-status
workflow-os --json author workflow catalog-status --strict-catalog-coverage
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
- when `steward-review --draft` is provided, derives fresh preflight context in the same process, calls the bounded in-memory steward-review helper, and prints a review card and decision result.
- when `promote --draft` is provided, derives fresh preflight context, validates the candidate as an active workflow in memory, requires same-process steward approval input, refuses active-path overwrites, and writes exactly one active workflow file unless `--dry-run` is present.
- when `catalog-status` is provided, derives active workflow, draft, archived draft, and optional local catalog-store summaries, calls the pure catalog index helper, and prints bounded inventory and conflict counts.

Generated draft files are intentionally review-only. They are nested under `workflows/drafts/`, which the current project loader does not treat as active workflow specs. They also include inactive posture fields such as `disabled_by_default: true`, empty triggers, empty steps, and authoring-obligation comments.

Preflight is also review-only. It does not move a draft into `workflows/`, does not register or activate a workflow, does not create runtime state, and does not approve promotion. Passing preflight means the draft is ready for steward review before any separately planned active-promotion step.

Steward review is a preview-only decision surface. It requires a preflight-passing draft, explicit reviewer, explicit decision, and bounded reason. `approved-for-promotion` authorizes only a separate active promotion command for the exact unchanged draft. It does not move files, persist approval records, create runtime state, execute commands, call providers, write artifacts, or approve future draft changes.

Active promotion is the first explicit mutation boundary. It promotes one preflight-passing draft from `workflows/drafts/<name>.workflow.yml` to `workflows/<name>.workflow.yml`, preserving the draft file. It does not persist the approval, start a run, create runtime state, execute commands, call providers, write report artifacts, update schemas, add examples, or authorize external writes. Promotion validates the candidate in active-placement context before writing and reloads the project after writing.

Catalog status is review-only. By default it reads `.workflow-os/catalog` only when that directory exists and otherwise reports `catalog_store: not_available` without creating it. `--catalog-root <path>` may point at an explicit safe repository-relative local catalog root. `--strict-catalog-coverage` turns missing catalog records for active workflows into blocker conflicts for the status command only; it does not change promotion or archive behavior.

## What It Does Not Do

The command does not:

- persist workflow catalog records beyond the active workflow file placement;
- create a workflow catalog root during status inspection;
- automatically promote or activate workflows;
- enforce catalog status in promotion or archive commands;
- persist steward approval records;
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

The command does not write files unless `--output` is explicitly supplied for inactive draft output or `author workflow promote` is invoked without `--dry-run`. Draft output is limited to one inactive file under `workflows/drafts/`. Active promotion is limited to one active file under `workflows/` and refuses overwrites.

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
- steward review is attempted against a draft with preflight blockers;
- steward review has an unknown decision;
- steward review has an invalid reviewer;
- steward review has a missing, too-long, or secret-like reason;
- active promotion is attempted without an explicit reviewer or reason;
- active promotion is attempted for a draft with preflight blockers;
- active promotion would overwrite an existing active workflow path;
- active promotion fails active-context validation before writing;
- active promotion fails post-write project validation;
- catalog-status is supplied an unsafe, absolute, traversal-shaped, or secret-like catalog root;
- catalog-status finds blocker conflicts;
- catalog-status cannot read valid local catalog, stewardship, or archive records;
- catalog-status cannot parse a draft or archived draft;
- project validation fails;
- proposal construction fails;
- draft writing fails.

Errors are bounded and do not echo unsafe recommendation ids, unsafe output paths, private directory material, or raw repository payloads.

## Compatibility

The JSON output is preview-only through `0.2.0-preview.1`. It is intended for local tooling and tests, not as a stable integration contract.

The draft file format is also preview-only. Draft output is a repository authoring aid, not a stable workflow-generation contract.
