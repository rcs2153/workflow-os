# Phase 2 Live Smoke Environment Checklist

This checklist is for maintainer-owned Phase 2 live smoke testing of the GitHub, Jira, and GitHub Actions read-only adapters.

Live smoke tests are opt-in, maintainer-only checks. They are not part of normal CI, must not run automatically, and must never create, update, comment, dispatch, rerun, merge, transition, assign, or otherwise mutate provider state.

Use this checklist before running [maintainer live smoke tests](live-smoke-tests.md).

## 1. Approved Resources

Before setting credentials, identify approved non-sensitive resources.

- [ ] Approved non-sensitive GitHub repository.
- [ ] Approved non-sensitive GitHub pull request if PR-context smoke coverage is needed.
- [ ] Approved non-sensitive GitHub Actions workflow run.
- [ ] Approved non-sensitive Jira project.
- [ ] Approved non-sensitive Jira issue.
- [ ] The resources contain no customer data, regulated data, secrets, private incident details, HR data, financial data, sensitive CI logs, or sensitive repository contents.
- [ ] The resources are documented by reference only, not by copying raw provider payloads into notes.

Do not use sensitive production resources unless a maintainer explicitly approves the exact resource, credential, purpose, and evidence-capture plan before the smoke test starts.

## 2. Credential Posture

Credentials must be read-only and must never be stored in Workflow OS specs, fixtures, docs, shell-history examples, screenshots, issues, pull requests, logs, or review notes.

### GitHub Read-Only

- [ ] Use a token with only the read permissions needed for the approved repository.
- [ ] Prefer a temporary or dedicated smoke-test token.
- [ ] Set `WORKFLOW_OS_LIVE_GITHUB_TESTS=1`.
- [ ] Set `WORKFLOW_OS_GITHUB_TOKEN` or fallback `GITHUB_TOKEN`.
- [ ] Confirm the token is not scoped for branch creation, commit writes, PR comments, review requests, label changes, merges, workflow dispatch, or check reruns.

### Jira Read-Only

- [ ] Set `WORKFLOW_OS_LIVE_JIRA_TESTS=1`.
- [ ] Set `WORKFLOW_OS_JIRA_BASE_URL`.
- [ ] For Atlassian Cloud, use Basic auth:
  - [ ] Set `WORKFLOW_OS_JIRA_EMAIL`.
  - [ ] Set `WORKFLOW_OS_JIRA_API_TOKEN`.
- [ ] If using fallback Basic auth names, set both `JIRA_EMAIL` and `JIRA_API_TOKEN`.
- [ ] Use `WORKFLOW_OS_JIRA_BEARER_TOKEN` only for Jira deployments that explicitly support bearer tokens.
- [ ] Set `WORKFLOW_OS_JIRA_TEST_ISSUE_KEY`.
- [ ] Confirm the account can read only the approved project/issue scope needed for the smoke test.
- [ ] Confirm partial Basic auth is not used.

### GitHub Actions / CI Read-Only

- [ ] Use a token with only the read permissions needed for the approved repository and workflow run.
- [ ] Set `WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS=1`.
- [ ] Set `WORKFLOW_OS_GITHUB_ACTIONS_TOKEN` or fallback `GITHUB_TOKEN`.
- [ ] Set `WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER`.
- [ ] Set `WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO`.
- [ ] Set `WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID`.
- [ ] Confirm the token cannot rerun workflows, dispatch workflows, cancel runs, upload artifacts, delete logs, or modify checks.

### Verify Presence Without Printing Secrets

Use shell checks that print only `set` or `missing`; do not echo values.

```sh
test -n "$WORKFLOW_OS_GITHUB_TOKEN" && echo "WORKFLOW_OS_GITHUB_TOKEN=set" || echo "WORKFLOW_OS_GITHUB_TOKEN=missing"
test -n "$WORKFLOW_OS_JIRA_API_TOKEN" && echo "WORKFLOW_OS_JIRA_API_TOKEN=set" || echo "WORKFLOW_OS_JIRA_API_TOKEN=missing"
test -n "$WORKFLOW_OS_GITHUB_ACTIONS_TOKEN" && echo "WORKFLOW_OS_GITHUB_ACTIONS_TOKEN=set" || echo "WORKFLOW_OS_GITHUB_ACTIONS_TOKEN=missing"
```

After testing, unset credentials in the same shell.

```sh
unset WORKFLOW_OS_LIVE_GITHUB_TESTS WORKFLOW_OS_GITHUB_TOKEN GITHUB_TOKEN
unset WORKFLOW_OS_LIVE_JIRA_TESTS WORKFLOW_OS_JIRA_BASE_URL WORKFLOW_OS_JIRA_EMAIL WORKFLOW_OS_JIRA_API_TOKEN WORKFLOW_OS_JIRA_BEARER_TOKEN WORKFLOW_OS_JIRA_TOKEN JIRA_EMAIL JIRA_API_TOKEN WORKFLOW_OS_JIRA_TEST_ISSUE_KEY
unset WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS WORKFLOW_OS_GITHUB_ACTIONS_TOKEN WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID
```

## 3. Safety Rules

- [ ] No provider write actions.
- [ ] No webhooks.
- [ ] No OAuth app flow.
- [ ] No workflow dispatch.
- [ ] No CI reruns.
- [ ] No CI cancellation.
- [ ] No issue updates.
- [ ] No Jira comments.
- [ ] No Jira status changes.
- [ ] No PR comments.
- [ ] No branch creation.
- [ ] No commits.
- [ ] No merges.
- [ ] No production resources without explicit approval.
- [ ] No broadening token permissions to make a smoke test pass.

If a command appears to require a write permission, stop. Do not retry with a broader credential.

## 4. Pre-Smoke Validation

Run offline checks first. Do not proceed to live testing if fixture tests fail.

- [ ] `npm run check:docs`
- [ ] `npm run check:integrations`
- [ ] `cargo test --workspace`
- [ ] Confirm live tests are skipped by default in the normal test output.
- [ ] Check `git status --short` and record whether the tree is clean or intentionally dirty.
- [ ] Search specs, fixtures, logs, and docs for accidental secrets before live testing:

```sh
rg -n "gh[pousr]_|xox[baprs]-|AKIA|BEGIN PRIVATE KEY|password|api[_-]?token|authorization:" workflow-os.yml workflows skills policies tests examples docs
```

If any match contains a real secret, stop and handle it as a credential exposure.

## 5. Smoke Execution Order

Run one provider at a time.

1. GitHub read-only smoke:

   ```sh
   npm run smoke:github-live
   ```

2. Jira read-only smoke:

   ```sh
   npm run smoke:jira-live
   ```

3. GitHub Actions / CI read-only smoke:

   ```sh
   npm run smoke:ci-live
   ```

4. Integration smoke summary:

   ```sh
   npm run smoke:integrations-live
   ```

The summary command should be run only after the provider-specific commands are understood. It must not replace per-provider evidence review.

## 6. Evidence Capture

For each provider, capture a short maintainer note with:

- [ ] Provider name.
- [ ] Command run.
- [ ] Timestamp with timezone.
- [ ] Resource identifier with sensitive values redacted.
- [ ] Result: pass or fail.
- [ ] Output summary, not full raw provider payload.
- [ ] Confirmation that no write occurred.
- [ ] Confirmation that no secret appeared in output.
- [ ] Classified failure details if the command failed.
- [ ] Confirmation that live credentials were unset or rotated after testing.

Example evidence shape:

```text
provider: github
command: npm run smoke:github-live
timestamp: 2026-05-25T14:30:00Z
resource: repo=<approved-non-sensitive-owner>/<approved-non-sensitive-repo>
result: pass
output_summary: live_github_repo_metadata_read_is_opt_in ... ok
write_confirmation: no provider writes observed
secret_output_confirmation: no token, authorization header, or token prefix appeared
cleanup: env vars unset
```

Do not paste raw tokens, authorization headers, raw issue descriptions, raw comments, raw private file contents, or raw CI logs into evidence.

## 7. Failure Handling

Stop immediately if any of these occur:

- A credential, token prefix, authorization header, API key, private key, or password appears in output.
- A provider resource is created, updated, commented on, rerun, dispatched, canceled, merged, transitioned, assigned, or otherwise mutated.
- A smoke test requires broader permissions than expected.
- Jira auth mode is ambiguous or partially configured.
- The provider returns a response that appears to include sensitive raw payload data in output.

When a smoke test fails:

- [ ] Classify the provider failure as authentication failure, permission failure, not found, rate limited, timeout, validation failure, malformed response, transient network failure, unsupported operation, policy denied, or unknown.
- [ ] Record the failure honestly.
- [ ] Do not retry by weakening permissions or switching to a broader token.
- [ ] Do not copy raw provider payloads into issues or review notes.
- [ ] If a secret leaked, rotate or revoke it before continuing.

## 8. Post-Smoke Cleanup

- [ ] Unset all smoke-test environment variables.
- [ ] Rotate or revoke any token that was accidentally exposed.
- [ ] Delete local smoke output if it contains sensitive provider data.
- [ ] Confirm no provider resources were mutated.
- [ ] Record any provider rate-limit, auth, permission, or redaction issue as a follow-up.
- [ ] Keep the default CI posture fixture-only.

## 9. Public Preview Gate

Do not announce a public read-only integration preview until:

- [ ] GitHub live smoke evidence is recorded.
- [ ] Jira live smoke evidence is recorded.
- [ ] GitHub Actions live smoke evidence is recorded.
- [ ] Evidence confirms no provider writes occurred.
- [ ] Evidence confirms no secrets appeared in output.
- [ ] Any failures are documented and either fixed or accepted as explicit limitations.
