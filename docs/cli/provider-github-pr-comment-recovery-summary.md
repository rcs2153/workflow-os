# provider github-pr-comment recovery-summary

`workflow-os provider github-pr-comment recovery-summary --summary <path>` renders
a bounded local operator recovery card for GitHub pull request comment provider
lookup posture.

This command is for maintainers inspecting an already produced, serialized
`GitHubPullRequestCommentProviderLookupOperatorRecoverySummary`.

## Usage

```sh
workflow-os provider github-pr-comment recovery-summary --summary recovery-summary.json
workflow-os --json provider github-pr-comment recovery-summary --summary recovery-summary.json
```

The legacy input flag `--lookup-recovery-result <path>` is accepted as an alias
for `--summary <path>`.

## Behavior

The command:

- reads one explicit local summary file;
- validates the summary through the existing Workflow OS model boundary;
- renders bounded posture for remote lookup, durable event proof, retry,
  artifact-write, operator-action, and next-action state;
- supports bounded JSON output through the global `--json` flag;
- fails closed on missing, invalid, or unsafe input.

The default text output is a recovery card:

```text
Provider lookup recovery posture
remote_lookup: observed
local_event_proof: event_proof_missing
observed_match_count: 1
observed_provider_reference: present
provider_error_code: absent
retry: blocked
artifact_write: blocked
operator_action: required
next_action: review_remote_provider_state, collect_event_proof

Why:
- Provider lookup posture is bounded summary vocabulary only.
- Workflow OS requires durable workflow event proof for provider outcomes.
- Provider lookup observations cannot replace workflow event proof.

What this command did not do:
- did not call GitHub
- did not write to GitHub
- did not retry provider writes
- did not repair state
- did not append events
- did not mutate side-effect records
- did not write report artifacts
```

## Boundaries

The command does not:

- call GitHub;
- read browser sessions, git credentials, keychains, or environment tokens;
- perform provider lookup;
- write to providers;
- retry provider writes;
- repair state;
- append workflow events;
- mutate side-effect records;
- write report artifacts;
- treat provider lookup observations as durable workflow event proof;
- expose schemas, examples, hosted behavior, approval-presentation enforcement,
  reasoning lineage, or release behavior.

Provider lookup can inform recovery. It is not durable workflow event proof.

## Failure Behavior

Missing or unreadable input fails with
`provider_lookup_operator_recovery_cli.input.missing`.

Invalid or unsafe input fails with
`provider_lookup_operator_recovery_cli.input.invalid`.

Both errors intentionally avoid echoing file paths, raw payloads, provider
responses, comments, diffs, command output, credentials, tokens, private keys,
or raw redaction metadata.
