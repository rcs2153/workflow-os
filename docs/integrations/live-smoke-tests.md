# Maintainer Live Smoke Tests

Phase 2 live smoke tests are maintainer-only checks for the GitHub, Jira, and GitHub Actions read-only adapters. They are opt-in, skipped by default, and not required for normal CI.

These smoke tests prove that the fixture-tested read-only adapter contract can reach real providers with maintainer-approved read-only credentials. They must never create, update, comment, rerun, dispatch, cancel, merge, transition, assign, or otherwise mutate provider state.

Before setting credentials or running any live smoke command, complete the [Phase 2 live smoke environment checklist](PHASE_2_LIVE_SMOKE_ENVIRONMENT_CHECKLIST.md). The checklist defines approved resources, credential handling, pre-smoke validation, evidence capture, failure handling, and cleanup.

## Safety Rules

- Use only maintainer-approved test repositories, Jira issues, and workflow runs.
- Do not use production-sensitive repos, issues, workflow runs, logs, or private customer data unless explicitly approved.
- Use least-privilege read-only credentials.
- Do not paste credentials into specs, fixtures, docs, shell history examples, screenshots, issues, or pull requests.
- Do not run live smoke tests from normal CI.
- Do not capture raw provider payloads when sharing failures.
- Treat leaked tokens, authorization headers, private issue bodies, private repo contents, or raw CI logs as security incidents.

## Commands

The npm wrappers fail clearly when required environment variables are missing and redact token-like values from command failure output.

Run one provider:

```sh
npm run smoke:github-live
npm run smoke:jira-live
npm run smoke:ci-live
```

Run all live smoke tests:

```sh
npm run smoke:integrations-live
```

Equivalent direct Rust commands are documented in each provider setup guide:

- [GitHub read-only setup](../operations/github-read-only-setup.md)
- [Jira read-only setup](../operations/jira-read-only-setup.md)
- [GitHub Actions read-only setup](../operations/github-actions-read-only-setup.md)

## GitHub Read-Only Smoke

Purpose: verify live read access to repository metadata.

Required environment:

```sh
export WORKFLOW_OS_LIVE_GITHUB_TESTS=1
export WORKFLOW_OS_GITHUB_TOKEN=...
```

`GITHUB_TOKEN` is accepted as a fallback. `WORKFLOW_OS_GITHUB_TOKEN` takes precedence.

Test data:

- The current live smoke test reads public repository metadata for `octocat/Hello-World`.
- Maintainers may use a low-privilege token that can read public metadata.
- If this test is changed to target a private repository later, use a dedicated non-sensitive test repository.

Expected output:

- The command exits `0`.
- The Rust test reports `live_github_repo_metadata_read_is_opt_in ... ok`.
- No token value or token prefix appears in output.

Cleanup:

- Unset `WORKFLOW_OS_LIVE_GITHUB_TESTS`.
- Unset `WORKFLOW_OS_GITHUB_TOKEN` or revoke the temporary token if one was created.

## Jira Read-Only Smoke

Purpose: verify live read access to Jira issue metadata.

Required environment for Atlassian Cloud Basic auth:

```sh
export WORKFLOW_OS_LIVE_JIRA_TESTS=1
export WORKFLOW_OS_JIRA_BASE_URL=https://example.atlassian.net
export WORKFLOW_OS_JIRA_EMAIL=person@example.com
export WORKFLOW_OS_JIRA_API_TOKEN=...
export WORKFLOW_OS_JIRA_TEST_ISSUE_KEY=OPS-42
```

Fallback Basic auth names are `JIRA_EMAIL` and `JIRA_API_TOKEN`. Bearer auth is supported only for Jira deployments that explicitly accept bearer tokens:

```sh
export WORKFLOW_OS_JIRA_BEARER_TOKEN=...
```

When complete Basic auth and bearer auth are both configured, Basic auth takes precedence.

Test data:

- Use a non-sensitive Jira project and issue created for adapter smoke testing.
- The issue may contain a simple summary, status, priority, labels, assignee, reporter, and non-sensitive description.
- Do not use issues with customer data, secrets, private incident details, HR data, financial data, or regulated data.

Expected output:

- The command exits `0`.
- The Rust test reports `live_jira_issue_metadata_read_is_opt_in ... ok`.
- No email address, token value, authorization header, description body, or comment body appears in output.

Cleanup:

- Unset `WORKFLOW_OS_LIVE_JIRA_TESTS`.
- Unset Jira auth environment variables.
- Revoke temporary API tokens when they were created only for smoke testing.

## GitHub Actions / CI Read-Only Smoke

Purpose: verify live read access to GitHub Actions workflow run metadata.

Required environment:

```sh
export WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS=1
export WORKFLOW_OS_GITHUB_ACTIONS_TOKEN=...
export WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER=acme
export WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO=widgets
export WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID=12345
```

`GITHUB_TOKEN` is accepted as a fallback. `WORKFLOW_OS_GITHUB_ACTIONS_TOKEN` takes precedence.

Test data:

- Use a maintainer-approved repository and workflow run with non-sensitive logs and metadata.
- Prefer a purpose-built test repository with a successful or intentionally simple failed workflow run.
- Do not use production incident runs, private customer logs, secret-bearing logs, or sensitive deployment pipelines unless explicitly approved.

Expected output:

- The command exits `0`.
- The Rust test reports `live_github_actions_workflow_run_read_is_opt_in ... ok`.
- No token value, token prefix, raw log body, or authorization header appears in output.

Cleanup:

- Unset `WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS`.
- Unset GitHub Actions smoke-test environment variables.
- Revoke temporary tokens when they were created only for smoke testing.

## Default CI Posture

Normal CI must remain fixture-based. The default CI workflow runs `npm run check:integrations`, which verifies adapter contracts and examples without live credentials. It must not run `npm run smoke:*`.

Live smoke tests may be run manually by maintainers before or after a public read-only integration preview, or by a future explicitly manual workflow if maintainers choose to add one with protected secrets. They remain opt-in and must not run in normal CI.

## Failure Handling

If a smoke test fails:

- Confirm the required environment variables are present without printing their values.
- Confirm the credential is read-only and has access to the test object.
- Confirm the test repository, issue, or workflow run exists.
- Check provider rate limits and service status.
- Do not retry write operations; they are intentionally unsupported.
- Share only classified error kind, adapter action, provider object reference, and redacted summaries.
