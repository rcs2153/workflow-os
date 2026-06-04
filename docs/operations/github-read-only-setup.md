# GitHub Read-Only Setup

The GitHub read-only adapter is optional. Workflow OS local kernel use does not require GitHub credentials.

This runbook is for `0.2.0-preview.1` GitHub read-only adapter preview evaluation. GitHub read-only live mode is opt-in and not required for local kernel use or normal CI.

## Fixture Mode

Fixture mode is used by normal tests and CI. It does not call GitHub and does not require credentials.

Use fixture mode for:

- unit tests
- contract tests
- offline development
- CI validation

## Live Read-Only Mode

Live read-only mode calls GitHub REST read APIs. It is opt-in.

Set a read-only token in one of:

```sh
export WORKFLOW_OS_GITHUB_TOKEN=...
```

or:

```sh
export GITHUB_TOKEN=...
```

`WORKFLOW_OS_GITHUB_TOKEN` is preferred when both are present.

Do not put tokens in Workflow OS specs.

## Health Checks

GitHub adapter health reports:

- adapter ID
- adapter kind
- operation mode
- configured/unconfigured
- credential present or absent
- last checked timestamp
- warnings

Health output must never print the token value.

## Live Test Opt-In

Live tests are skipped by default. To run them manually:

```sh
export WORKFLOW_OS_LIVE_GITHUB_TESTS=1
export WORKFLOW_OS_GITHUB_TOKEN=...
cargo test -p workflow-core --test github_adapter -- --ignored
```

Use a token with the least privilege needed for the repository being read.

Maintainers can also run the documented smoke wrapper:

```sh
npm run smoke:github-live
```

Use only maintainer-approved non-sensitive repositories for live smoke testing. The current smoke test reads public `octocat/Hello-World` repository metadata.

## Troubleshooting

Common failures:

- `auth_failure`: token missing, expired, revoked, or invalid
- `permission_failure`: token lacks access to the repository
- `not_found`: repository, pull request, file, or ref does not exist or is not visible to the token
- `rate_limited`: GitHub rate limit exhausted
- `timeout`: GitHub did not respond within the adapter timeout
- `malformed_response`: response shape was not valid JSON for JSON endpoints

See [integration troubleshooting](integration-troubleshooting.md) for generic adapter diagnostics.
