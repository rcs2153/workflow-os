# author workflow

`workflow-os author workflow --from-recommendation <id> --dry-run` previews inactive workflow authoring obligations for one existing first-run recommendation.

This command is a dry-run authoring surface. It is designed to help a maintainer or agent understand what would need to be authored before a recommendation can become active governance.

## Usage

```sh
workflow-os author workflow --from-recommendation first_run.repo_implementation --dry-run
workflow-os --json author workflow --from-recommendation first_run.assign_ownership --dry-run
```

`--dry-run` is required. The command fails closed without it.

## What It Does

- loads and validates the local Workflow OS project;
- recomputes the bounded first-run recommendation set;
- finds the requested recommendation id;
- builds the existing inactive draft proposal summary;
- prints required authoring decisions, validation expectations, missing fields, non-goals, privacy posture, and next action.

## What It Does Not Do

The command does not:

- write workflow files;
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

## Failure Behavior

The command fails closed when:

- `--dry-run` is missing;
- `--from-recommendation <id>` is missing;
- the recommendation id is unknown;
- the recommendation id is invalid or secret-like;
- project validation fails;
- proposal construction fails.

Errors are bounded and do not echo unsafe recommendation ids or raw repository payloads.

## Compatibility

The JSON output is preview-only through `0.2.0-preview.1`. It is intended for local tooling and tests, not as a stable integration contract.
