# GitHub Draft Pull Request Live Sandbox Proof Report

Report date: 2026-10-23

## 1. Executive Summary

Workflow OS completed the dedicated GitHub draft pull request live sandbox
proof through the accepted explicit provider-write helper.

The unchanged ignored test ran against exact pinned refs in the private
`rcs2153/workflow-os-sandbox` repository. It created one managed draft pull
request, then used a second independent lookup-before-create path to reconcile
that same draft without issuing a second create. The persistent provider
reference is
[`rcs2153/workflow-os-sandbox#1`](https://github.com/rcs2153/workflow-os-sandbox/pull/1).

This proof does not enable writes by default and does not authorize another
repository, provider, mutation family, merge, close, cleanup, CLI write path,
or automatic runtime use.

## 2. Governed Phase

- workflow: `dg/implement`
- run: `run-1792705388668478000-2`
- approval: `approval/run-1792705388668478000-2/implementation-approved`
- presentation: `presentation/a20e85e7b5daff48`
- presentation hash:
  `a20e85e7b5daff489dc98049c332b757560737f5d6d7a480000483f043c27b13`
- approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof
- approved scope: rerun the unchanged ignored smoke against the exact sandbox
  target, record the proof, validate the repository, and prepare the governed
  change for review
- close status: completed with 39 events, one approval, zero retries, and zero
  escalations
- presentation enforcement: proof enforced; the approval event trail includes
  the persisted presentation marker

## 3. Exact Sandbox Preconditions

- repository: private `rcs2153/workflow-os-sandbox`
- base branch: `main`
- base SHA: `aa520c7d51a05ab72c12121161acb03d2c6dc210`
- head branch: `workflow-os-sandbox-proof-v1`
- head SHA: `dd8718cb44ba7eb076cceb6bcc3b074971bdedaf`
- open pull requests before the successful proof: zero
- head posture: one commit ahead of the pinned base

The test's compile-time allowlist, branch names, and expected SHAs were not
changed for this proof.

## 4. Credential Posture

The credential was created as a fine-grained GitHub token with:

- resource owner: `rcs2153`;
- repository access: only `rcs2153/workflow-os-sandbox`;
- contents: read-only;
- metadata: required read-only; and
- pull requests: read and write.

It is stored under the dedicated macOS Keychain service
`workflow-os-github-draft-pr-sandbox`. It was retrieved into the live test
process without printing or writing its value, and the clipboard used during
initial storage was cleared. Workflow OS Core does not discover or load this
credential.

## 5. Command And Result

The existing ignored test was run unchanged:

```text
cargo test -p workflow-core --test github_draft_pull_request \
  live_github_com_transport_creates_or_reconciles_one_exact_managed_draft \
  -- --ignored --exact
```

Result: one test passed, zero failed.

The explicit environment supplied only the smoke enablement flag, Keychain-
retrieved token, exact head and base branch names, and exact pinned SHAs.

## 6. Provider Outcome And Reconciliation

The first operation found no existing managed draft and created:

- pull request: `rcs2153/workflow-os-sandbox#1`;
- state: open;
- draft: true;
- title: `Workflow OS governed draft pull request sandbox proof`;
- base: `main` at the pinned base SHA;
- head: `workflow-os-sandbox-proof-v1` at the pinned head SHA; and
- managed marker: `workflow-os-github-draft-pr-sandbox-v1`.

The same test then constructed a second independent provider path, repeated the
lookup, selected the exact managed draft, and returned the same stable pull
request reference without calling create again. The test requires exactly one
create request across both operations.

## 7. Scope Explicitly Not Completed

This phase did not:

- change production Rust behavior or the ignored test;
- enable any write by default;
- add hidden credential loading to Core;
- add an executor or CLI write path;
- merge, close, edit, or clean up the draft;
- push a branch or mutate repository contents;
- authorize another target, provider, or mutation family;
- add retry or redirect behavior;
- claim production provider-write readiness;
- add schema, SDK, example, hosted, or release behavior; or
- weaken approval, policy, SideEffect, report, or immutable-input boundaries.

## 8. Privacy And Redaction

- The token value is absent from repository files, command output, reports, and
  diagnostics.
- The concrete transport continues to redact authorization headers and reduce
  provider responses to bounded modeled fields.
- The report records branch names, commit SHAs, permission posture, and the
  stable draft reference, but no raw provider payload.
- The provider write occurred only in the dedicated private sandbox.

## 9. Validation

The live smoke passed before repository-wide validation:

```text
cargo test -p workflow-core --test github_draft_pull_request \
  live_github_com_transport_creates_or_reconciles_one_exact_managed_draft \
  -- --ignored --exact
```

End-of-phase validation passed:

```text
cargo fmt --all --check                                      passed
cargo clippy --workspace --all-targets -- -D warnings        passed
cargo test --workspace                                      passed
npm run check:docs                                          passed
git diff --check                                            passed
```

The ordinary workspace suite kept environment-gated live tests ignored. The
single authorized live smoke was run separately and passed as described above.

## 10. Workflow Semantics And External State

The proof used the explicit helper path and did not mutate a Workflow OS run,
append runtime events, or change workflow pass/fail semantics. The external
draft remains intentionally persistent so its stable reference can be reviewed.
Cleanup requires a separately governed action.

The accepted transport still treats uncertainty after the create-call boundary
as may-have-started. This successful proof did not exercise that ambiguity.

## 11. Remaining Known Limitations

- The proof is an ignored, environment-gated integration test rather than a
  public executor or CLI surface.
- The credential is an operator-managed sandbox prerequisite, not a Workflow OS
  secret-provider integration.
- Reconciliation is limited to the exact managed marker, title, repository,
  head, and base contract.
- No cleanup path is implemented or authorized.
- No claim is made for multi-tenant, production, hosted, or additional provider
  mutation readiness.

## 12. Recommended Next Phase

Perform a focused maintainer and security review of this completed live proof.
The review should verify least privilege, exact target binding, idempotent
reconciliation, post-call ambiguity, redaction, and non-broadening scope before
the Active Delivery Queue selects any additional provider mutation.

## 13. Out-Of-Kernel Work

The Workflow OS kernel governed phase scope and approval. Codex and the
maintainer performed the browser authentication, token creation and Keychain
storage, provider smoke execution, external pull request inspection,
documentation edits, validation commands, and Git/PR operations. The kernel
did not execute shell commands, manipulate credentials, call GitHub, edit
files, or create the external draft itself.
