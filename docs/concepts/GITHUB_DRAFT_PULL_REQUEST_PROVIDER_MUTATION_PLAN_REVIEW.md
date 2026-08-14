# GitHub Draft Pull Request Provider Mutation Plan Review

Review date: 2026-08-14

## 1. Executive Verdict

**Plan accepted after fix-forward corrections; proceed to one integrated local
sandbox implementation milestone.**

The plan selects one useful bounded mutation family without authorizing Git
transport or broad writes. Review found two planning blockers: it overstated
GitHub's ability to bind PR creation atomically to immutable SHAs, and it left
room to interpret broad GitHub/write capability vocabulary as creation
authority. Both are corrected in the plan.

## 2. Scope Verification

The phase stayed within planning and review scope. It added no provider call,
PR, Git mutation, runtime code, schema, CLI mutation, example, Jira write,
generic adapter, hosted production behavior, enterprise administration, or
release change.

The future slice remains explicit, draft-only, local, sandbox-bound, and
disabled from default executor and CLI paths.

## 3. Candidate Assessment

Draft pull request creation from an already-pushed branch is an appropriate
second candidate:

- it closes a repeated dogfood boundary;
- it is consequential enough to prove authority and reconciliation composition;
- it is less dangerous than merge, ref mutation, CI dispatch, or issue-field
  mutation;
- it has stable provider identities and read surfaces;
- draft posture preserves a later human or governed readiness/merge boundary.

The candidate is not low-risk merely because the PR is a draft. It is an
externally visible provider mutation and may notify users or expose bounded
metadata. The first slice correctly requires blocking approval.

## 4. Git And Provider Boundary Assessment

The plan now clearly separates Git transport from GitHub metadata creation.
The provider adapter cannot commit, create refs, push, infer the current branch,
or mutate source.

The original draft incorrectly implied that exact head/base SHA checks could
atomically protect provider creation. GitHub creates a PR from mutable branch
names. The corrected plan treats SHAs as pre/post observations, discloses the
branch-tracking semantics in approval, blocks known pre-create drift, and
surfaces interval drift after creation as reconciliation-required posture. It
does not promise impossible atomic prevention or automatically close the draft.

This is the correct provider-semantic boundary. A later readiness or merge
action must reassess current commits independently.

## 5. Governance And Authority Assessment

The gate order is complete: immutable run, report, exact capability, target,
provider facts, durable policy decision, proportional-governance reassessment,
current authority, approval presentation, SideEffect, idempotency, redaction,
and explicit sandbox mode.

The corrected authority rule is essential. `SideEffectCapability::GitHubWrite`
is classification vocabulary, `AdapterWriteCapability::GitHubPullRequestCreate`
is preflight vocabulary, and `HostedProjectCapability::ApprovalDecide` governs
approval decisions. None grants PR creation authority.

The first implementation must resolve exact capability reference
`github.pull_request.create` against an exact repository resource through the
scoped capability-authority model. It must not implicitly map broad
`ExternalWrite` or `GitHubWrite` grants to this operation. Hosted capability
extension remains later.

The policy rule is also now explicit: the exact policy decision must come from
the durable run/step event trail. A caller-authored allowed posture or bare
reference is not sufficient.

## 6. Proportional Governance Assessment

The caller does not choose interaction mode. The accepted selector derives
execution and disclosure posture from authoritative facts.

Requiring blocking approval for the first externally visible draft creation is
appropriate. The plan does not turn that conservative sandbox minimum into a
permanent product rule. A future non-blocking path would still require exact
authority, policy, evidence, SideEffect, event, report, and stewardship proof.

Visible disclosure remains a presentation axis rather than an execution mode,
which incorporates external product feedback correctly.

## 7. SideEffect And Reconciliation Assessment

The proposed lifecycle correctly requires durable `Proposed` and `Attempted`
states before provider mutation, known success before `Completed`, known
non-creation before `Failed`, and explicit reconciliation posture for ambiguity.

Lookup-before-create and no automatic retry are required. The corrected
idempotency rule distinguishes creation identity from evolving branch content:
an existing managed PR is not duplicated when its head later moves. Instead,
drift is disclosed and later actions require reassessment.

This avoids both duplicate PRs and the incorrect claim that a draft PR is an
immutable artifact.

## 8. Evidence, Report, And Privacy Assessment

The plan records only bounded provider identity, observed commit posture,
classification, content commitment, and stable evidence/report references.
Core remains the owner of events, EvidenceReference construction, WorkReport
disclosure, and artifact integrity.

Tokens, auth headers, rendered bodies, provider payloads, source, diffs, logs,
command output, environment values, approval prose, and policy payloads are
excluded. Debug and error contracts redact target, ref, SHA, content, marker,
and idempotency values.

The bounded template rule is preferable to arbitrary model-authored PR text for
the first slice.

## 9. Test Assessment

The planned matrix covers the important pre-provider, provider, concurrency,
recovery, privacy, and non-regression boundaries. The fix-forward addition for
movement between pre-create and post-create reads is required and present.

The live sandbox proof must remain ignored by default or environment-gated and
must use a non-sensitive maintainer-controlled target. Unit and fixture tests
cannot substitute for that provider-semantic proof before claiming live
support.

## 10. Blockers

None after the documented corrections.

Implementation must stop if the exact scoped capability reference cannot be
resolved without broadening authority, or if the provider cannot return enough
bounded head/base/draft facts for reconciliation. Those are implementation
gates, not permission to weaken the plan.

## 11. Non-Blocking Follow-Ups

- Select the least-privilege GitHub App or fine-grained token permission for the
  maintained sandbox proof.
- Decide whether the bounded managed marker is visibly rendered or hidden in
  the PR body source.
- Specify the distinct terminal classification for an existing managed draft
  with ref drift.
- Add hosted capability vocabulary only in a separately reviewed collaborative
  mutation phase.
- Treat readiness, merge, and post-creation branch updates as separate governed
  actions.

## 12. Recommended Next Phase

Implement one integrated **local sandbox draft GitHub pull request creation
vertical slice** following the accepted plan.

The milestone should include model, policy/preflight, exact scoped authority,
approval-presentation proof, SideEffect persistence, injected provider lookup
and create, reconciliation, event/evidence/report closure, tests, runbook, and
phase-level maintainer/security review.

Do not add Git transport, non-draft creation, merge, Jira mutation, default
runtime writes, workflow schemas, hosted mutation behavior, or another provider
family.

## 13. Validation

- `npm run check:docs`: passed
- `git diff --check`: passed
- Manual scope/security review: passed after the two fix-forward corrections

## 14. Governed Review Evidence

- Workflow: `dg/review`
- Run ID: `run-1786698020878709000-2`
- Approval ID:
  `approval/run-1786698020878709000-2/review-scope-approved`
- Approval presentation ID: `presentation/96f367c06d03bdd7`
- Approval outcome: granted through the proof-enforced path
- Event summary: 39 events, one approval, zero retries, zero escalations;
  `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`,
  `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`,
  `RunValidated:1`, `SkillInvocationRequested:6`,
  `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, and
  `StepScheduled:6`

Out-of-kernel work: Codex inspected implementation vocabulary and accepted
plans, reviewed and corrected the candidate plan, authored this review, ran
documentation validation, and will perform git/PR operations. The kernel
governed scope and approval but did not edit files, call GitHub, or perform git
operations.
