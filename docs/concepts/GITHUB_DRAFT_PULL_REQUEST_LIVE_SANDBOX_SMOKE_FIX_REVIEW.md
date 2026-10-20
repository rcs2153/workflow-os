# GitHub Draft Pull Request Live Sandbox Smoke Fix Review

Review date: 2026-10-20

## 1. Executive Verdict

**Fixture blocker fixed; live provider proof remains blocked on dedicated
sandbox access.**

The test-only correction is accepted. It restores exact approval-presentation
binding for the ignored smoke without weakening production enforcement or
changing provider behavior. This verdict does not accept the live GitHub write
proof, because no provider mutation has yet completed.

## 2. Scope Verification

The correction stayed within the approved blocker-fix scope:

- content-bound approval-decision fixture construction;
- content-bound terminal-run fixture construction;
- exact use by the ignored live smoke;
- direct mismatch regression coverage; and
- truthful roadmap and phase reporting.

No production provider logic, target allowlist, HTTP behavior, retry posture,
runtime integration, CLI, schema, SDK, hosted behavior, or release posture
changed.

## 3. Binding Assessment

`approval_decision_for(content)` derives the persisted proof marker from the
same `ApprovalPresentationRecord` that the mutation helper receives.
`terminal_run_for(content)` embeds that exact decision in the durable run
fixture. The ignored smoke now constructs both from `live_content`.

The existing parameterless helpers delegate to the ordinary fixture and
preserve existing tests. The production helper continues to compare the
durable proof marker with the supplied presentation identifier, content hash,
validation policy, and decision timestamp.

## 4. Negative-Path Assessment

The new regression deliberately presents different content from the content
bound into the durable run. It requires stable error
`github_draft_pull_request.approval.proof_marker_mismatch` and asserts zero
provider transport calls. This directly covers the failure that blocked the
first live-smoke attempt.

The first attempt therefore provides useful security evidence: exact scope
substitution was rejected before external observation or mutation.

## 5. Validation Assessment

- formatting check: passed;
- focused ordinary integration tests: 11 passed, 0 failed;
- substitution regression: passed;
- documentation check: passed;
- diff check: passed; and
- sandbox open pull requests after both attempts: zero.

The rerun passed the corrected approval boundary and reached provider
pre-observation. It then failed closed with
`github_draft_pull_request.provider.pre_observation_failed`.

## 6. Access And Privacy Assessment

Read-only probes prove that the installed access is valid for
`rcs2153/workflow-os` but not visible to the private
`rcs2153/workflow-os-sandbox` repository. The GitHub connector independently
confirms that the sandbox and exact refs exist. This supports a repository-
scope prerequisite rather than an expired-access or missing-target diagnosis.

No access value appeared in output, Debug, diagnostics, reports, or repository
files. No separate sandbox Keychain entry was created because no valid
sandbox-scoped value was available to store.

## 7. Blockers

There is no remaining code blocker in the fixture correction.

The live provider proof remains blocked until a separate least-privilege access
grant for `rcs2153/workflow-os-sandbox` is available with contents read and
pull requests write. The main-repository access must not be silently broadened
or replaced.

## 8. Non-Blocking Follow-Ups

- Consider a small operator preflight that distinguishes inaccessible sandbox
  target from generic bounded pre-observation failure outside Core logs.
- Preserve the ignored smoke's exact create-then-independent-reconcile shape.
- Keep the created draft persistent until a separately governed cleanup path
  exists.

## 9. Recommended Next Phase

**Provision dedicated sandbox access and rerun the unchanged ignored smoke.**

If that proof succeeds, document the stable draft pull request reference and
the second lookup-only reconciliation. If it fails with correct sandbox access,
open a focused transport/provider-contract blocker phase before any new
provider mutation family.

## 10. Governed Review Evidence

- workflow: `dg/review`
- run: `run-1792537312580257000-2`
- approval: `approval/run-1792537312580257000-2/review-scope-approved`
- presentation: `presentation/b9d78c1dafe9df4f`
- approval outcome: granted under delegated maintainer authority
- phase close: `Completed`, 39 events, one approval, zero retries, zero
  escalations
- presentation enforcement: proof-enforced with persisted event marker

Out-of-kernel review work comprised commit inspection, focused Rust validation,
read-only sandbox-state inspection, documentation validation, and report edits.
No provider call, access provisioning, or git mutation was performed by the
kernel itself.
