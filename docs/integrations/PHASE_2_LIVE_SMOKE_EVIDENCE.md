# Phase 2 Live Smoke Evidence

Live smoke evidence has not yet been recorded.

The Phase 2 read-only adapters remain fixture-gated and ready for internal read-only integration use, but public read-only integration preview readiness must not be claimed until maintainer-owned live smoke evidence is captured.

## Current Evidence Status

- Date checked: 2026-05-25T13:43:27Z.
- Repository commit SHA: `ee28a58ae7829ec4ba9f9865a841ab740f61654c`.
- Working tree status: dirty; Phase 2 adapter, documentation, example, and smoke-test files are present as uncommitted changes.
- Live tests run: no.
- Reason live tests were not run: required maintainer-owned live smoke preconditions were not met in this shell. This was rechecked and the required opt-in flags, credentials, and live resource identifiers were still missing.

## Missing Preconditions

The following live smoke environment variables were checked by name only. Values were not printed.

### GitHub Read-Only

- `WORKFLOW_OS_LIVE_GITHUB_TESTS`: missing.
- `WORKFLOW_OS_GITHUB_TOKEN` or `GITHUB_TOKEN`: missing.

### Jira Read-Only

- `WORKFLOW_OS_LIVE_JIRA_TESTS`: missing.
- `WORKFLOW_OS_JIRA_BASE_URL`: missing.
- `WORKFLOW_OS_JIRA_EMAIL` plus `WORKFLOW_OS_JIRA_API_TOKEN`: missing.
- `JIRA_EMAIL` plus `JIRA_API_TOKEN`: missing.
- `WORKFLOW_OS_JIRA_BEARER_TOKEN`: missing.
- `WORKFLOW_OS_JIRA_TEST_ISSUE_KEY`: missing.

### GitHub Actions / CI Read-Only

- `WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS`: missing.
- `WORKFLOW_OS_GITHUB_ACTIONS_TOKEN` or `GITHUB_TOKEN`: missing.
- `WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER`: missing.
- `WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO`: missing.
- `WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID`: missing.

## Result

No provider calls were made. No GitHub, Jira, or GitHub Actions resources were read or mutated. No credentials were printed, stored, or unset by this check because no live credentials were present in the environment.

Public read-only integration preview remains blocked until maintainers provide approved non-sensitive resources, load read-only credentials through environment variables, run the documented live smoke tests, and record evidence using the template.

## Maintainer Instructions

Maintainers should:

1. Complete the [Phase 2 live smoke environment checklist](PHASE_2_LIVE_SMOKE_ENVIRONMENT_CHECKLIST.md).
2. Run the documented [maintainer live smoke tests](live-smoke-tests.md) with approved non-sensitive resources and read-only credentials.
3. Record results using [the live smoke evidence template](PHASE_2_LIVE_SMOKE_EVIDENCE_TEMPLATE.md).
4. Confirm no provider writes occurred and no secrets appeared in output.

Do not mark Phase 2 ready for public read-only integration preview until evidence is captured and reviewed.
