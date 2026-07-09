# GitHub PR Comment Provider Lookup Operator Recovery CLI Plan

Status: Implemented as an explicit local summary-input CLI. This follows the accepted
[GitHub PR Comment Provider Lookup Operator Recovery Summary Helper Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_SUMMARY_HELPER_REVIEW.md)
and is accepted in
[GitHub PR Comment Provider Lookup Operator Recovery CLI Plan Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_CLI_PLAN_REVIEW.md).
The implementation is documented in
[GitHub PR Comment Provider Lookup Operator Recovery CLI Implementation Report](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_CLI_IMPLEMENTATION_REPORT.md).

## 1. Executive Summary

Workflow OS now has an in-memory operator recovery summary helper for GitHub
pull request comment provider lookup recovery posture.

The next implementation question is how a local operator should inspect that
posture from the CLI without changing runtime behavior.

This plan defined a future local, explicit, read-only CLI surface that renders
the existing summary posture as a human recovery card and optional bounded JSON.
That surface is now implemented as
`workflow-os provider github-pr-comment recovery-summary --summary <path>`.
The command consumes explicit serialized summary input only.

The CLI must preserve the central boundary:

```text
Provider lookup can inform recovery, but it is not durable workflow event proof.
```

## 2. Goals

- Expose provider lookup operator recovery posture to a local maintainer.
- Render a concise, human-readable recovery card.
- Support bounded machine-readable output later if consistent with CLI patterns.
- Consume existing validated helper/model outputs rather than recreating logic.
- Keep lookup, recovery, retry, repair, and artifact posture explicit.
- Make event-proof gaps visible.
- Make retry and artifact-write blocks understandable.
- Preserve redaction-safe output.
- Avoid raw provider payloads, comments, diffs, logs, command output, source
  contents, paths, credentials, tokens, and private keys.
- Prepare for later high-assurance repair/retry planning without implementing it.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- hidden auth loading;
- automatic provider lookup;
- automatic provider writes;
- automatic retries;
- manual state repair;
- workflow event append from recovery;
- side-effect record mutation;
- report artifact writes;
- treating lookup observations as durable workflow event proof;
- workflow schema changes;
- examples;
- hosted/distributed behavior;
- reasoning lineage;
- approval-presentation enforcement;
- release posture changes.

## 4. Proposed CLI Boundary

The first CLI surface should be local and operator-facing.

Illustrative command shape:

```sh
workflow-os provider github-pr-comment recovery-summary \
  --lookup-recovery-result <path-or-id> \
  --format text
```

This shape is intentionally tentative. The implementation phase should inspect
existing CLI conventions before naming the command.

The command should:

- load or receive an already produced lookup/recovery summary source through an
  explicit local input;
- validate that input through existing model constructors;
- render bounded posture only;
- exit non-zero on invalid or unsafe input;
- avoid provider calls;
- avoid repository mutation;
- avoid workflow event append;
- avoid report artifact writes.

It should not:

- read browser sessions, git credentials, keychains, or environment tokens by
  default;
- infer provider state by searching GitHub;
- create missing provider references;
- repair state;
- retry writes;
- make report artifacts eligible without durable event proof.

## 5. Input Policy

The first implementation should accept explicit local input only.

Allowed inputs:

- serialized `GitHubPullRequestCommentProviderLookupOperatorRecoverySummary`,
  if serialization remains part of the accepted model boundary;
- explicit lookup/recovery integration result, if a safe local loading path
  already exists;
- explicit run/side-effect identifiers only as display references, not as
  permission to query state unless separately scoped.

Forbidden inputs:

- raw provider responses;
- raw comment bodies;
- pull request bodies;
- diffs;
- review threads;
- CI logs;
- command output;
- source file contents;
- ambient credentials;
- token-like strings;
- private keys;
- unbounded paths;
- hidden state searches.

If the CLI cannot get a validated summary without hidden lookup or hidden state
loading, the first implementation should stop at a command shape that validates
explicit serialized summary input.

## 6. Output Policy

Default human output should be a concise recovery card.

Example:

```text
Provider lookup recovery posture

remote_comment: observed
local_event_proof: missing
retry: blocked
artifact_write: blocked
operator_action: required
next_action: plan_manual_state_repair

Why:
- A bounded provider lookup found a matching remote comment.
- Workflow OS does not have accepted durable event proof for the provider outcome.
- Lookup observations cannot replace workflow event proof.

What this command did not do:
- did not call GitHub
- did not write to GitHub
- did not repair state
- did not append events
- did not write report artifacts
```

JSON output, if added, should expose only stable bounded fields from the summary
model. It must not include raw provider references, raw errors, auth details, or
raw metadata strings that have not passed report-safe validation.

## 7. Event-Proof And Artifact Policy

The CLI must state artifact-write posture clearly.

Rules:

- provider lookup observed does not equal durable event proof;
- missing event proof keeps artifact writes blocked;
- accepted event proof may only be reflected if the validated summary already
  reports an artifact-allowed posture;
- the CLI must not compute new artifact eligibility by inspecting provider data;
- the CLI must not append missing events to make artifact writes possible.

## 8. Retry And Repair Policy

The CLI may show retry or repair recommendations from the existing summary
vocabulary.

It must not:

- run retry logic;
- enqueue retry logic;
- repair state;
- mutate side-effect records;
- create events;
- mark a provider write as completed;
- present manual repair as already approved.

Any future repair command should be separately planned with high-assurance
approval, event projection, audit disclosure, and report disclosure.

## 9. Error Handling

Errors must fail closed and remain stable.

Recommended stable error-code families:

- `provider_lookup_operator_recovery_cli.input.missing`
- `provider_lookup_operator_recovery_cli.input.invalid`
- `provider_lookup_operator_recovery_cli.input.unsafe`
- `provider_lookup_operator_recovery_cli.render.invalid_format`
- `provider_lookup_operator_recovery_cli.unsupported`

Errors must not include:

- raw provider payloads;
- comments;
- diffs;
- source snippets;
- command output;
- credentials;
- tokens;
- private keys;
- raw redaction metadata;
- sensitive file paths.

## 10. UX Requirements

The command should be useful to an operator under stress.

Human output should:

- lead with the recovery posture;
- show retry and artifact gates;
- show the next action;
- state why lookup is not event proof;
- state what the command did not do;
- avoid marketing language;
- avoid implying automatic repair.

If verbose output is later added, the default should remain concise and
operator-readable.

## 11. Test Plan

Future implementation tests should cover:

- observed remote comment renders operator-action-required posture;
- missing event proof renders artifact-write blocked;
- accepted event proof renders artifact posture only from the validated summary;
- remote absent renders retry-review posture when represented by the summary;
- unauthorized, unavailable, rate-limited, invalid, ambiguous, and untrusted
  postures render safely;
- text output includes the "lookup is not event proof" boundary;
- JSON output, if implemented, contains only bounded fields;
- invalid serialized input fails closed;
- secret-like input fails without leakage;
- raw provider/comment/diff/log/source markers are not copied;
- the command performs no provider calls;
- the command appends no workflow events;
- the command mutates no side-effect records;
- the command writes no report artifacts;
- existing provider write, lookup, recovery, event-proof, report artifact,
  executor, CLI, and side-effect tests continue to pass.

## 12. Documentation Updates For Future Implementation

Future implementation should update:

- CLI command reference if the command becomes public or semi-public;
- provider lookup operator recovery docs;
- roadmap current-state text;
- implementation report and review docs.

Documentation must state:

- the CLI is local and explicit;
- hidden auth loading is not implemented;
- automatic lookup is not implemented;
- provider writes are not implemented by the command;
- repair is not implemented;
- event append is not implemented;
- artifact writes remain blocked without durable event proof.

## 13. Proposed Implementation Sequence

1. Review this CLI plan.
2. Implement a local explicit CLI surface that consumes already validated summary
   input.
3. Add focused rendering and non-leakage tests.
4. Review the CLI surface.
5. Plan hidden-auth or state-loading behavior separately only if still needed.
6. Plan manual repair separately with high-assurance approval controls.

## 14. Open Questions

- Should the first CLI input be a serialized summary file, a run/side-effect
  state lookup, or both?
- Should JSON output be included in the first implementation or deferred?
- Should the command live under `provider github-pr-comment`, `side-effect`, or
  `operator` CLI namespace?
- Should the command require an explicit `--acknowledge-lookup-is-not-proof`
  flag for recovery postures that are easy to misread?
- Should state-loading be allowed before approval-presentation enforcement is
  implemented?

## 15. Final Recommendation

The next phase should be a maintainer review of this CLI plan.

The first implementation after review should be the smallest local CLI surface
that renders an already validated operator recovery summary. It must not add
hidden auth loading, automatic provider lookup, provider writes, retries, repair,
workflow event append, side-effect mutation, report artifact writes, schemas,
examples, hosted behavior, reasoning lineage, approval-presentation enforcement,
or release posture changes.
