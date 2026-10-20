# GitHub Draft Pull Request Live Sandbox Smoke Blocker Report

Report date: 2026-10-20

## 1. Executive Summary

The first governed execution of the ignored GitHub draft pull request live
sandbox smoke failed closed before any provider call. The failure is a test
fixture integration drift, not a GitHub transport failure and not a provider
mutation ambiguity.

The ignored smoke constructs distinct live-sandbox content but uses the shared
`terminal_run()` fixture. That fixture embeds an approval proof marker derived
from the ordinary `content()` fixture. The hardened provider-write helper
correctly rejects the mismatch with stable code
`github_draft_pull_request.approval.proof_marker_mismatch`.

No draft pull request was created. The dedicated sandbox repository still had
zero open pull requests after the failed proof.

## 2. Governed Phase

- workflow: `dg/implement`
- run: `run-1792531346664964000-2`
- approval: `approval/run-1792531346664964000-2/implementation-approved`
- presentation: `presentation/ac8c6e6bac58e063`
- approval outcome: granted under delegated maintainer authority
- approved purpose: execute the already-reviewed allowlisted live smoke

## 3. Verified Sandbox Inputs

The pre-call inspection verified:

- repository: private dedicated `rcs2153/workflow-os-sandbox` sandbox;
- base branch: `main` at `aa520c7d51a05ab72c12121161acb03d2c6dc210`;
- proof branch: `workflow-os-sandbox-proof-v1` at
  `dd8718cb44ba7eb076cceb6bcc3b074971bdedaf`; and
- open pull requests before and after the failed attempt: zero.

The repository credential was obtained from the operating-system credential
store and injected only into the test process. Its value was not printed,
persisted, or included in diagnostics.

## 4. Command And Result

The exact ignored test was selected:

```text
cargo test -p workflow-core --test github_draft_pull_request \
  live_github_com_transport_creates_or_reconciles_one_exact_managed_draft \
  -- --ignored --exact
```

The test failed with:

```text
github_draft_pull_request.approval.proof_marker_mismatch
```

The failure occurred during pre-provider approval validation. The transport did
not receive a request, so no provider write was attempted and no ambiguous
external state exists.

## 5. Root Cause

`approval_decision()` derives its persisted proof marker from
`presentation(&content())`. The live smoke creates `live_content`, then creates
`presentation(&live_content)`, but its run still comes from `terminal_run()` and
therefore carries the ordinary fixture's proof marker.

The mismatch demonstrates that the production boundary is enforcing exact
presentation scope as intended. Weakening the helper or bypassing proof
validation would be incorrect.

## 6. Scope Explicitly Not Completed

This phase did not:

- alter production code;
- weaken approval-presentation enforcement;
- call the GitHub provider transport;
- create, edit, merge, close, or clean up a pull request;
- broaden the sandbox target or mutation family;
- add automatic executor or CLI write behavior; or
- claim completion of the live provider proof.

## 7. Recommended Blocker Fix

Add a content-bound approval-decision fixture and construct the smoke's terminal
run from the exact live content and matching presentation proof. Preserve the
existing ordinary fixture behavior and add a regression proving mismatched
content still fails closed before provider use.

After focused tests pass, rerun the same ignored smoke with the same exact
allowlisted target. A successful phase must prove the first create-or-reconcile
result and the second independent lookup-before-create reconciliation, then
inspect and disclose the persistent draft pull request by stable reference.

## 8. Recommended Next Phase

**Focused GitHub draft pull request live-smoke fixture blocker fix.**

This is the smallest correction that can unblock the already-accepted provider
proof without changing production behavior or widening provider authority.

## 9. Blocker-Fix Follow-Up

The focused fixture correction is implemented. The integration test now offers
content-bound approval-decision and terminal-run helpers, and the ignored smoke
uses its exact live content for both the persisted approval proof and presented
scope. A new regression substitutes separately presented content and proves
the helper returns `github_draft_pull_request.approval.proof_marker_mismatch`
with zero provider calls.

Focused validation passed:

```text
cargo fmt --all --check
cargo test -p workflow-core --test github_draft_pull_request \
  -- --skip live_github_com_transport_creates_or_reconciles_one_exact_managed_draft
```

Result: 11 passed, 0 failed.

The exact ignored smoke was rerun. It passed approval validation and reached
the real provider pre-observation boundary, then failed closed with stable code
`github_draft_pull_request.provider.pre_observation_failed`.

Read-only status probes established the remaining prerequisite without
printing access material:

- the installed repository-scoped credential receives `200` for
  `rcs2153/workflow-os`;
- the same credential receives `404` for
  `rcs2153/workflow-os-sandbox`; and
- the GitHub connector independently confirms that the private sandbox and
  both exact refs exist.

The credential is therefore valid but scoped to the main repository rather
than the dedicated sandbox. This is not evidence of token expiry or transport
failure. The phase does not silently broaden that credential.

The sandbox still has zero open pull requests. The next action is explicit
least-privilege credential provisioning for `rcs2153/workflow-os-sandbox` with
contents read and pull requests write, followed by the unchanged ignored smoke.
