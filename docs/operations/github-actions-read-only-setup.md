# GitHub Actions Read-Only Setup

The GitHub Actions read-only adapter is optional. Workflow OS local kernel use does not require CI credentials.

This runbook is for `0.2.0-preview.1` GitHub Actions read-only adapter preview evaluation. GitHub Actions read-only live mode is opt-in and not required for local kernel use or normal CI.

## Fixture Mode

Fixture mode is used by normal tests and CI. It does not call GitHub Actions and does not require credentials.

Use fixture mode for:

- unit tests
- contract tests
- offline development
- CI validation

## Live Read-Only Mode

Live read-only mode calls GitHub REST read APIs for GitHub Actions and checks. It is opt-in.

Set a read-only token in one of:

```sh
export WORKFLOW_OS_GITHUB_ACTIONS_TOKEN=...
```

or:

```sh
export GITHUB_TOKEN=...
```

`WORKFLOW_OS_GITHUB_ACTIONS_TOKEN` is preferred when both are present.

Do not put tokens in Workflow OS specs.

## Health Checks

GitHub Actions adapter health reports:

- adapter ID
- adapter kind
- operation mode
- configured/unconfigured
- credential present or absent
- last checked timestamp
- warnings

Health output must never print the token value or token prefix.

## Live Test Opt-In

Live tests are skipped by default. To run them manually:

```sh
export WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS=1
export WORKFLOW_OS_GITHUB_ACTIONS_TOKEN=...
export WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER=acme
export WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO=widgets
export WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID=12345
cargo test -p workflow-core --test ci_adapter -- --ignored
```

Use a token with the least privilege needed for the repository and workflow run being read.

Maintainers can also run the documented smoke wrapper:

```sh
npm run smoke:ci-live
```

Use only maintainer-approved repositories and workflow runs with non-sensitive logs and metadata for live smoke testing.

For the `0.2.0-preview.1` release posture, recorded live smoke evidence exercised workflow run metadata only. Broader live GitHub Actions coverage remains future opt-in smoke work.

## Troubleshooting

Common failures:

- `auth_failure`: token missing, expired, revoked, or invalid
- `permission_failure`: token lacks access to the repository, checks, or workflow run
- `not_found`: repository, workflow run, job, check run, or ref does not exist or is not visible to the token
- `rate_limited`: GitHub rate limit exhausted
- `timeout`: GitHub did not respond within the adapter timeout
- `malformed_response`: response shape was not valid JSON for JSON endpoints

See [integration troubleshooting](integration-troubleshooting.md) for generic adapter diagnostics.
