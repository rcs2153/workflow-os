# GitHub Draft Pull Request Provider Mutation Plan Report

## 1. Executive Summary

Planning is complete for one bounded second provider mutation candidate: draft
GitHub pull request creation from an already-pushed branch.

The plan keeps Git transport outside Workflow OS provider mutation, requires an
exact remote head/base commitment, composes accepted governance primitives,
and proposes one integrated future vertical slice. No provider call or runtime
write behavior is implemented by this phase.

## 2. Scope Completed

- Selected draft GitHub pull request creation as the single next mutation
  candidate.
- Defined the boundary between already-completed Git transport and GitHub PR
  metadata creation.
- Defined exact immutable run, report, capability, target, policy,
  proportional-governance, authority, approval, SideEffect, idempotency,
  reconciliation, evidence, and WorkReport gates.
- Defined bounded templated PR content and non-leaking provider receipt posture.
- Defined failure and ambiguous-outcome semantics with no automatic retry.
- Defined one integrated implementation milestone and acceptance matrix.
- Updated the authoritative roadmap after the merged durable approval-route
  milestone.

## 3. Scope Explicitly Not Completed

This phase did not add a provider call, pull request, Git operation, mutation
adapter, runtime path, CLI command, workflow schema, SDK surface, example,
hosted production behavior, enterprise identity, Jira integration, reasoning
lineage, or release change.

It does not authorize commits, branches, pushes, merges, labels, reviewers,
assignees, checks, releases, or another provider mutation family.

## 4. Candidate Decision

Selected candidate:

```text
Create one draft GitHub pull request from an already-pushed exact head SHA.
```

Why:

- it closes a real dogfood gap;
- it is externally visible but cannot merge work;
- it has stable provider identities and lookup surfaces;
- it can compose existing authority, approval, SideEffect, reconciliation,
  evidence, and report boundaries;
- it does not require Workflow OS to own Git transport.

## 5. Governance Boundary

The first slice requires a fresh accepted proportional-governance assessment,
fresh exact-resource current authority, and a proof-enforced blocking approval.
The GitHub credential remains technical access, not authority.

Stored approval routes remain historical routing evidence. They do not grant
pull request creation permission.

## 6. Idempotency And Recovery

The plan requires lookup before create by exact repository, head, and base;
reconciliation against exact SHAs, draft posture, and a bounded marker; and no
automatic retry after provider ambiguity.

Exact equivalent provider state may be reconciled without duplicate creation.
Conflict or incomplete provider facts fail closed.

## 7. Privacy And Redaction

The future boundary must not persist or expose tokens, auth headers, rendered
PR bodies, raw provider payloads, diffs, source, logs, command output,
environment values, or approval/policy payloads.

Core records bounded stable references, content commitments, classifications,
and evidence/report links. Debug and errors redact repository, branch, SHA,
content, marker, and idempotency values.

## 8. Validation

Required planning validation:

- `npm run check:docs`
- `git diff --check`
- manual diff review for capability honesty and scope boundaries

- `npm run check:docs`: passed
- `git diff --check`: passed
- Manual diff review: passed; the documents do not claim provider execution or
  broaden ambient Git or GitHub authority.

## 9. Governed Phase Evidence

- Workflow: `dg/d`
- Run ID: `run-1786697760687795000-2`
- Approval ID:
  `approval/run-1786697760687795000-2/planning-approved`
- Approval presentation ID: `presentation/946a90d77eae4256`
- Approval outcome: granted through the proof-enforced path
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations;
  `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`,
  `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`,
  `RunValidated:1`, `SkillInvocationRequested:6`,
  `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, and
  `StepScheduled:6`

Out-of-kernel work: Codex inspected repository plans, ADRs, accepted reviews,
roadmap state, and implementation vocabulary; authored planning documents;
ran documentation validation; and will perform git/PR operations. The kernel
governed scope and approval but did not edit files, invoke GitHub, or perform
git operations.

## 10. Remaining Limitations

- No request/response model exists for draft PR creation.
- The default preview write policy still rejects `GitHubPullRequestCreate`.
- No injected provider client or live smoke exists.
- Exact authority capability mapping and the first bounded template remain
  implementation-review questions.
- Broad provider writes remain blocked.

## 11. Recommended Next Phase

Run a focused maintainer and security review of the plan. If accepted, proceed
to one integrated implementation milestone for draft-only GitHub pull request
creation from an already-pushed exact branch.
