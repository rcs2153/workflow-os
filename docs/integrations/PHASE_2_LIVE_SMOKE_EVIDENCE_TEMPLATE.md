# Phase 2 Live Smoke Evidence Template

Use this template after completing the [Phase 2 live smoke environment checklist](PHASE_2_LIVE_SMOKE_ENVIRONMENT_CHECKLIST.md).

Do not include secrets, tokens, authorization headers, raw Jira issue bodies, raw PR comments, raw private file contents, or raw CI logs. Record references and summaries only.

## 1. Review Metadata

- Date:
- Maintainer:
- Repository commit SHA:
- Working tree status:
- Smoke environment:
  - Local machine or CI/manual runner:
  - Operating system:
  - Rust toolchain:
  - Node/npm version:
- Approved resource inventory:
  - GitHub repository:
  - GitHub pull request:
  - GitHub Actions workflow run:
  - Jira project:
  - Jira issue:
  - Approval reference for using these resources:

## 2. Pre-Flight Checks

- Fixture tests passed:
  - Command:
  - Result:
- Integration gate passed:
  - Command:
  - Result:
- Docs check passed:
  - Command:
  - Result:
- Normal test run confirmed live tests are skipped by default:
  - Command:
  - Result:
- No secrets found in specs, fixtures, logs, or docs:
  - Command or review method:
  - Result:
- Credentials loaded from environment only:
  - Confirmation:
- No credentials stored in specs or fixtures:
  - Confirmation:

## 3. GitHub Read-Only Smoke

- Command:
- Environment variables used, names only:
  - `WORKFLOW_OS_LIVE_GITHUB_TESTS`
  - `WORKFLOW_OS_GITHUB_TOKEN` or `GITHUB_TOKEN`
- Resource identifiers, redacted:
  - Repository:
  - Pull request, if exercised:
- Operations exercised:
  - Repository metadata:
  - Default branch:
  - File reference/content metadata:
  - Pull request metadata:
  - Changed files:
  - Pull request comments:
  - Check summaries:
- Result:
- Output summary:
- No write confirmation:
- No secret exposure confirmation:
- Failure details, if any:
  - Classification:
  - Summary:
  - Follow-up:

## 4. Jira Read-Only Smoke

- Command:
- Auth mode used:
  - Atlassian Cloud Basic auth:
  - Bearer auth:
- Environment variables used, names only:
  - `WORKFLOW_OS_LIVE_JIRA_TESTS`
  - `WORKFLOW_OS_JIRA_BASE_URL`
  - `WORKFLOW_OS_JIRA_EMAIL`
  - `WORKFLOW_OS_JIRA_API_TOKEN`
  - `JIRA_EMAIL`
  - `JIRA_API_TOKEN`
  - `WORKFLOW_OS_JIRA_BEARER_TOKEN`
  - `WORKFLOW_OS_JIRA_TEST_ISSUE_KEY`
- Resource identifiers, redacted:
  - Jira project:
  - Jira issue:
- Operations exercised:
  - Issue metadata:
  - Issue summary:
  - Description reference:
  - Comments reference:
  - Status:
  - Priority:
  - Labels:
  - Assignee/reporter display data:
  - Project metadata:
- Result:
- Output summary:
- No write confirmation:
- No secret exposure confirmation:
- Failure details, if any:
  - Classification:
  - Summary:
  - Follow-up:

## 5. GitHub Actions / CI Read-Only Smoke

- Command:
- Environment variables used, names only:
  - `WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS`
  - `WORKFLOW_OS_GITHUB_ACTIONS_TOKEN` or `GITHUB_TOKEN`
  - `WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER`
  - `WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO`
  - `WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID`
- Resource identifiers, redacted:
  - Repository:
  - Workflow run:
  - Job, if exercised:
- Operations exercised:
  - Workflow run metadata:
  - Workflow jobs:
  - Check summaries:
  - Failure summary:
  - Log reference:
  - Bounded redacted log excerpt:
- Result:
- Output summary:
- No rerun/dispatch confirmation:
- No secret exposure confirmation:
- Failure details, if any:
  - Classification:
  - Summary:
  - Follow-up:

## 6. Post-Flight Checks

- Credentials unset:
  - Confirmation:
- No provider mutation observed:
  - Confirmation:
- Local output reviewed for secrets:
  - Confirmation:
- Tokens rotated if needed:
  - Confirmation or not applicable:
- Failures recorded:
  - Confirmation or not applicable:
- Sensitive local smoke output deleted if present:
  - Confirmation or not applicable:

## 7. Verdict

Choose one:

- Ready for public read-only integration preview.
- Not ready for public read-only integration preview.

Blockers:

- None recorded yet.

Accepted limitations:

- None recorded yet.

Recommended next step:

- None recorded yet.
