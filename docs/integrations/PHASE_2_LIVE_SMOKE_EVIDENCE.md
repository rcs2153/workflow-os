# Phase 2 Live Smoke Evidence

Phase 2 live smoke evidence was recorded on 2026-06-04 for GitHub read-only, GitHub Actions / CI read-only, and Jira read-only adapter paths.

This document records maintainer-provided live smoke output summaries. It does not include tokens, authorization headers, raw provider payloads, raw issue bodies, raw pull request content, raw CI logs, or private credential values.

This evidence does not itself announce public read-only integration preview readiness. It provides input for a follow-up maintainer readiness review.

## 1. Review Metadata

- Date: 2026-06-04.
- Maintainer: repository owner/operator.
- Repository commit SHA at start of evidence capture: `a1de00f434f073711a72da76d76c578f8e81c93d`.
- Working tree status: documentation evidence file was updated during this capture; no runtime or adapter source change was made as part of smoke execution.
- Smoke environment:
  - Local machine: maintainer local shell.
  - Node/npm path: repository-bundled Node/npm under `.tools/node-v20.19.5-darwin-arm64`.
  - Live credentials: loaded through environment variables by the human operator; values were not recorded in this document.

Approved resource inventory:

- GitHub repository: `rcs2153/AGT` approved by the human operator for GitHub/GitHub Actions smoke testing.
- GitHub pull request: not exercised by the current live smoke tests.
- GitHub Actions workflow run: `rcs2153/AGT` workflow run `26415289853`.
- Jira site: `https://segar.atlassian.net`.
- Jira issue: `KAN-1`.
- Jira project: inferred from issue key prefix `KAN`.
- Approval reference: human operator approved use of the listed GitHub/GitHub Actions and Jira sandbox resources in the local evaluation thread.

## 2. Pre-Flight Checks

- Fixture tests passed:
  - Command: `npm run check:integrations`.
  - Result: passed earlier in the same local evaluation sequence after the tracked CI log fixture fix.
- Normal test run confirmed live tests are skipped by default:
  - Command: `cargo test --workspace`.
  - Result: passed earlier in the same local evaluation sequence; live GitHub, Jira, and GitHub Actions tests remained ignored by default.
- Credentials loaded from environment only:
  - Confirmation: smoke wrappers required environment variables and failed closed when they were missing.
- No credentials stored in specs or fixtures:
  - Confirmation: no spec or fixture edits were made to carry live credentials.

## 3. GitHub Read-Only Smoke

- Command: `npm run smoke:github-live`.
- Environment variables used, names only:
  - `WORKFLOW_OS_LIVE_GITHUB_TESTS`.
  - `WORKFLOW_OS_GITHUB_TOKEN`.
- Resource identifiers:
  - Repository exercised by current test implementation: `octocat/Hello-World`.
  - Repository approved by operator for future targeted smoke: `rcs2153/AGT`.
- Operations exercised:
  - Repository metadata: exercised.
  - Default branch: not exercised by this live smoke test.
  - File reference/content metadata: not exercised by this live smoke test.
  - Pull request metadata: not exercised by this live smoke test.
  - Changed files: not exercised by this live smoke test.
  - Pull request comments: not exercised by this live smoke test.
  - Check summaries: not exercised by this live smoke test.
- Result: pass.
- Output summary:
  - `live_github_repo_metadata_read_is_opt_in ... ok`.
  - `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out`.
- No write confirmation: the smoke test used the read-only repository metadata adapter path only.
- No secret exposure confirmation: no token value was included in the output summary provided for this evidence.
- Failure details: none.

Limitation:

- The current GitHub live smoke test is hardcoded to `octocat/Hello-World`. Testing repository metadata specifically against `rcs2153/AGT` requires parameterizing the GitHub live smoke target or adding a separately reviewed smoke path.

## 4. Jira Read-Only Smoke

- Command: `npm run smoke:jira-live`.
- Auth mode used:
  - Atlassian Cloud Basic auth with email and API token.
- Environment variables used, names only:
  - `WORKFLOW_OS_LIVE_JIRA_TESTS`.
  - `WORKFLOW_OS_JIRA_BASE_URL`.
  - `WORKFLOW_OS_JIRA_EMAIL`.
  - `WORKFLOW_OS_JIRA_API_TOKEN`.
  - `WORKFLOW_OS_JIRA_TEST_ISSUE_KEY`.
- Resource identifiers:
  - Jira site: `https://segar.atlassian.net`.
  - Jira issue: `KAN-1`.
- Operations exercised:
  - Issue metadata: exercised.
  - Issue summary: included in metadata path where provider response supports it.
  - Description reference: not exercised by this live smoke test.
  - Comments reference: not exercised by this live smoke test.
  - Status: included in metadata path where provider response supports it.
  - Priority: included in metadata path where provider response supports it.
  - Labels: included in metadata path where provider response supports it.
  - Assignee/reporter display data: included in metadata path where provider response supports it.
  - Project metadata: not separately exercised by this live smoke test.
- Result: pass.
- Output summary:
  - `live_jira_issue_metadata_read_is_opt_in ... ok`.
  - `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 20 filtered out`.
- No write confirmation: the smoke test used the read-only Jira issue metadata adapter path only.
- No secret exposure confirmation: no token value or authorization header was included in the output summary provided for this evidence.
- Failure details:
  - Initial attempt failed with `jira.http.404` because the placeholder token string was still configured. The retry with the real sandbox API token passed.

## 5. GitHub Actions / CI Read-Only Smoke

- Command: `npm run smoke:ci-live`.
- Environment variables used, names only:
  - `WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS`.
  - `WORKFLOW_OS_GITHUB_ACTIONS_TOKEN`.
  - `WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER`.
  - `WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO`.
  - `WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID`.
- Resource identifiers:
  - Repository: `rcs2153/AGT`.
  - Workflow run: `26415289853`.
  - Job: not directly exercised by this live smoke test.
- Operations exercised:
  - Workflow run metadata: exercised.
  - Workflow jobs: not exercised by this live smoke test.
  - Check summaries: not exercised by this live smoke test.
  - Failure summary: not exercised by this live smoke test.
  - Log reference: not exercised by this live smoke test.
  - Bounded redacted log excerpt: not exercised by this live smoke test.
- Result: pass.
- Output summary:
  - `live_github_actions_workflow_run_read_is_opt_in ... ok`.
  - `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out`.
- No rerun/dispatch confirmation: the smoke test used the read-only workflow run metadata adapter path only.
- No secret exposure confirmation: no token value was included in the output summary provided for this evidence.
- Failure details:
  - Initial attempt failed with `ci.github_actions.http.404` because the wrong identifier was used. The retry with workflow run ID `26415289853` passed.

## 6. Post-Flight Checks

- Credentials unset:
  - Confirmation: the human operator was instructed to unset smoke-test environment variables after execution.
- No provider mutation observed:
  - Confirmation: all passing smoke tests exercised read-only adapter methods only. No write, comment, transition, rerun, dispatch, cancel, merge, assignment, or mutation path was part of these commands.
- Local output reviewed for secrets:
  - Confirmation: pasted output summaries contained no token values or authorization headers.
- Tokens rotated if needed:
  - Confirmation: not recorded. The Jira token was pasted into the local evaluation thread before smoke execution; rotating it is recommended even though the Jira site is a sandbox.
- Failures recorded:
  - Confirmation: initial GitHub Actions wrong-ID 404 and initial Jira placeholder-token 404 are recorded above.
- Sensitive local smoke output deleted if present:
  - Confirmation: not applicable from the recorded output summaries.

## 7. Verdict

Evidence status: live smoke evidence is recorded for GitHub read-only, Jira read-only, and GitHub Actions / CI read-only smoke paths.

This evidence supports a follow-up public read-only integration preview readiness review. It does not itself mark Phase 2 ready for public preview.

Accepted limitations:

- GitHub live smoke currently exercises repository metadata for `octocat/Hello-World`, not the approved `rcs2153/AGT` repository.
- GitHub Actions live smoke exercised workflow run metadata only.
- Jira live smoke exercised issue metadata only.
- Live tests remain opt-in and skipped by default.
- No write-capable adapters, webhook ingestion, OAuth flow, production backend, distributed workers, hosted integration service, or Level 3/4 autonomy behavior is implied by this evidence.

Recommended next step:

- Perform a follow-up maintainer readiness review using this evidence.
- Consider parameterizing the GitHub repository metadata live smoke target before any claim that `rcs2153/AGT` has been exercised through the GitHub read-only adapter path.
