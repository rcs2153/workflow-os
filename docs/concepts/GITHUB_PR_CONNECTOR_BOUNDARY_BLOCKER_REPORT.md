# GitHub PR Connector Boundary Blocker Report

## 1. Executive Summary

A governed blocker review investigated a GitHub PR creation failure encountered while preparing a Workflow OS planning PR.

The failure was initially described too broadly as a stale or expired token. That was imprecise and misleading. The evidence shows a narrower boundary:

- `git push` to `rcs2153/workflow-os` succeeded from the local repository credential.
- The Codex GitHub connector returned `401 token_expired` before a Workflow OS provider path was invoked.
- The local REST attempt using the git credential reached GitHub but lacked PR-creation API authority and returned `403 Resource not accessible by personal access token`.
- The repo-owned Workflow OS GitHub provider implementation is for explicit GitHub pull request comment provider calls, not PR creation, and it requires caller-supplied auth plus injected transport.

This report fixes the repo-owned diagnosis gap: connector errors must not be attributed to Workflow OS provider auth unless a Workflow OS provider path actually ran.

## 2. Blocker Fixed

The immediate blocker was not a code defect in the Workflow OS GitHub PR comment provider.

The blocker was a governance and maintainer-ops diagnosis failure:

- an external connector failure was treated as if it proved credential expiry;
- the repo-owned provider boundary was not distinguished from the Codex GitHub connector boundary;
- the next action risked drifting into browser/tool workarounds instead of accurately identifying the failing system.

The fix is this documented boundary and operating rule:

```text
Connector-auth failures, browser-auth failures, git-credential failures, and Workflow OS provider-auth failures are distinct evidence classes.
Do not collapse them into one "token expired" diagnosis.
```

## 3. Evidence Reviewed

Observed during the blocker phase:

- Current branch push succeeded for `codex/real-repo-onboarding-ux-plan`.
- GitHub connector PR creation failed with `401 token_expired`.
- `gh` was unavailable locally.
- Local REST PR creation using the git credential failed with `403 Resource not accessible by personal access token`.
- Google Chrome was open to the GitHub compare URL, but the Computer Use accessibility bridge returned `cgWindowNotFound`.
- Workflow OS repo-owned GitHub provider code and tests show the implemented provider path is scoped to GitHub PR comment writes, not PR creation.

Repo-owned files inspected:

- `crates/workflow-core/src/provider_write.rs`
- `crates/workflow-core/tests/provider_write.rs`
- `docs/implementation-plans/github-pr-comment-provider-client-auth-loading-plan.md`
- `docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REVIEW.md`

## 4. Boundary Assessment

### Codex GitHub Connector

The connector is external to this repository. Its `401 token_expired` response may reflect connector session state, connector app auth, or connector service state. It does not prove that the local git credential is expired, and it does not prove that Workflow OS provider auth failed.

### Local Git Credential

The local credential can push branches. It is not sufficient evidence for REST PR creation scope. The observed `403` indicates the credential used for git transport did not authorize the PR creation API request through that path.

### Workflow OS Provider Code

Workflow OS currently implements a narrow GitHub PR comment provider-call boundary:

- explicit caller-supplied auth;
- injected transport;
- bounded status classification;
- no hidden auth discovery;
- no automatic PR creation;
- no CLI mutation behavior;
- no default executor writes.

The failing PR creation action did not invoke this provider path.

## 5. Operating Rule

For future governed maintainer work:

1. Treat GitHub UI, GitHub connector, local git credential, REST API, and Workflow OS provider paths as separate systems.
2. Record which system actually produced the error.
3. Do not state that a token is expired unless the failing credential source is identified.
4. Do not attribute external connector errors to Workflow OS provider auth.
5. Do not fall back to Safari when the requested path is Chrome.
6. If PR creation is blocked, preserve pushed branches and report the exact failing boundary.

## 6. Scope Explicitly Not Changed

This blocker report does not implement:

- connector token refresh;
- GitHub app reauthorization;
- browser automation changes;
- GitHub PR creation inside Workflow OS;
- hidden auth loading;
- automatic provider writes;
- broad GitHub write support;
- CLI PR commands;
- schemas;
- examples;
- hosted behavior;
- release posture changes.

## 7. Validation

Commands run:

- `npm run dogfood:benchmark -- phase-start --phase blocker --state-dir /private/tmp/workflow-os-github-pr-path-blocker-state --no-build ...`: passed after work-context wording was sanitized.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-github-pr-path-blocker-state --mock-all-local-skills approve run-1783313086148366000-2 approval/run-1783313086148366000-2/fix-approved --actor user/dogfood-reviewer --reason approved-github-pr-path-reliability-blocker`: passed.
- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783313086148366000-2 --phase blocker --state-dir /private/tmp/workflow-os-github-pr-path-blocker-state --no-build`: passed.

Governed blocker run:

- Workflow: `dg/blocker`.
- Run: `run-1783313086148366000-2`.
- Approval: `approval/run-1783313086148366000-2/fix-approved`.
- Approval outcome: granted under delegated maintainer authority after the complete approval handoff block was surfaced.
- Terminal status: `Completed`.
- Events: 39.
- Approvals: 1.

## 8. Remaining Known Limitations

- Codex GitHub connector auth remains outside this repository.
- Chrome accessibility bridge failure remains outside Workflow OS core.
- GitHub PR creation remains an external maintainer operation, not a Workflow OS runtime capability.
- The previously pushed onboarding UX planning branch still needs PR creation once GitHub UI/connector access is available.

## 9. Recommended Next Phase

Recommended next phase: continue the real-repo onboarding UX lane after PR creation access is restored.

The next implementation slice remains existing agent-instruction preservation. This connector-boundary report should prevent future diagnosis drift while the build continues.
