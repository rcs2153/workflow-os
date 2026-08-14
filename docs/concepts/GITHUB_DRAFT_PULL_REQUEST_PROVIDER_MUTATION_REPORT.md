# GitHub Draft Pull Request Provider Mutation Report

## 1. Executive Summary

Workflow OS now has one explicit Core helper for a second bounded provider
mutation family: creating or reconciling a managed draft GitHub pull request
from an already-pushed branch through an injected provider.

The helper composes the existing immutable-run, proportional-governance,
policy, current-authority, approval-presentation, SideEffect, idempotency,
evidence, and WorkReport citation foundations. It is not wired into the
default executor, CLI, hosted runtime, or a concrete GitHub HTTP transport.

## 2. Scope Completed

- Added `GitHubPullRequestCreate` readiness support through a separate
  draft-only sandbox policy; the default preview policy remains unchanged.
- Added a bounded repository target with explicit head/base branch names and
  full lowercase SHA observations.
- Added versioned bounded title/body/marker content with a deterministic
  SHA-256 commitment.
- Added a non-serializable explicit-auth provider request and injected provider
  trait for ref observation, exact lookup, and one create call.
- Added one integrated mutation helper that returns an in-memory result.
- Required a coherent terminal run with immutable bundle binding.
- Required a validated terminal WorkReport artifact with the exact run,
  workflow, version, schema, and spec-hash identity.
- Required the accepted V3 current-runtime-fact proportional-governance binding
  recorded in that run plus a matching reassessment evaluated at provider-use
  time, with complete facts and visible blocking-approval posture.
- Required durable policy events for adapter invocation with both external-write
  and adapter-invoke capabilities, fresh exact repository-scoped
  `github.pull_request.create` authority, a matching approval request, and a
  granted decision carrying the exact approval-presentation proof marker.
- Persisted and linked a proposed SideEffect before mutation, transitioned it
  to attempted before the create call, and produced completed or failed
  terminal records only for known outcomes.
- Added lookup-before-create reconciliation, known pre-create drift blocking,
  post-create ref observation, and no automatic retry after ambiguity.
- Added bounded provider evidence and report-ready citations for reconciled
  completion.
- Expanded provider-content secret-shape rejection for common GitHub, GitLab,
  and Slack token prefixes.

## 3. Scope Explicitly Not Completed

This phase does not add:

- Git commit, branch, push, force-push, fetch, or other Git transport;
- a concrete GitHub HTTP client or live network smoke;
- automatic executor or hosted-runtime invocation;
- public CLI mutation behavior;
- non-draft creation, ready-for-review conversion, merge, close, labels,
  reviewers, assignees, comments, checks, releases, or workflow dispatch;
- Jira or another provider mutation;
- automatic credential discovery, hidden auth, or credential persistence;
- workflow schemas, SDK changes, examples, or release posture changes;
- generic provider-write abstraction or default write enablement.

## 4. API Summary

`execute_github_draft_pull_request_mutation(...)` accepts an explicit local
SideEffect store, injected provider, and `GitHubDraftPullRequestMutationInput`.
The input carries only typed governance objects, a validated terminal report
artifact, bounded target/content, explicit caller-supplied auth, and stable
identities.

The provider request deliberately has no serialization implementation. Debug
output redacts auth, repository, branches, SHAs, content, idempotency identity,
and correlation context.

## 5. Governance Boundary

Provider invocation requires all of the following:

1. coherent terminal run and immutable bundle;
2. exact accepted V3 proportional-governance binding from the run and a
   matching current reassessment evaluated at provider-use time;
3. durable allowed adapter-invocation policy decision requiring approval and
   carrying external-write plus adapter-invoke capabilities;
4. exact current capability resolution for
   `github.pull_request.create` and the exact repository;
5. valid terminal WorkReport artifact bound to the exact run;
6. durable approval request and granted decision with the matching
   approval-presentation proof marker;
7. approval presentation bound to the exact rendered-content commitment;
8. draft-only preflight with SideEffect and idempotency identities;
9. valid SideEffect approval linkage;
10. exact pre-create head/base provider observations;
11. lookup-before-create reconciliation.

Broad `ExternalWrite`, generic GitHub access, an approval decision by itself,
or possession of a token is insufficient.

## 6. Provider And Reconciliation Behavior

- Exact existing managed draft: no create call; reconcile as completed.
- No existing draft and exact refs: perform exactly one create call.
- Known provider rejection: transition SideEffect to failed.
- Ambiguous lookup: do not call create; require operator review.
- Ambiguous create outcome or provider error: retain attempted lifecycle,
  block automatic retry, and require reconciliation.
- Known pre-create ref drift: fail before lookup/create.
- Ref movement observed after creation: retain attempted lifecycle and surface
  concurrent-ref-change reconciliation posture. Do not retry or auto-close.

The helper does not claim GitHub can atomically create a PR against immutable
SHAs. Branch SHAs are governed pre/post observations of mutable refs.

## 7. Evidence And Reporting

Known reconciled completion returns a bounded `AdapterResponseSummary`
EvidenceReference and report-ready citations to the EvidenceReference and
SideEffect identity. The validated input artifact is referenced from the
SideEffect record. The helper does not create or persist another WorkReport or
report artifact, and it does not append returned lifecycle events to the run.

## 8. Privacy And Redaction

Raw provider payloads, response bodies, logs, diffs, source contents, command
output, environment values, tokens, authorization headers, and rendered PR
content are not persisted by this helper. Errors are stable and non-leaking.
Ambiguous provider codes are bounded and redacted from Debug output.

## 9. Test Coverage

Focused integration tests cover:

- successful single-call creation with completed SideEffect, evidence, and
  report citations;
- exact existing managed-draft reconciliation without create;
- ambiguous create posture with no retry and no false post-observation claim;
- pre-create ref drift blocking before create;
- exact approval content-commitment binding;
- granted-decision approval-presentation proof-marker enforcement;
- policy action and external-write/adapter-invoke capability enforcement;
- stale current-runtime-fact reassessment rejection;
- validated terminal report-artifact acceptance;
- secret-like content rejection and non-leaking errors.

The fixture uses a real local SideEffect store and reconstructs a terminal run
with immutable bundle, authoritative governance binding, durable policy and
approval events, exact capability authority, and canonical approval
presentation.

## 10. Validation

Phase validation targets:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed

## 11. Governed Phase Evidence

- Workflow: `dg/implement`
- Run ID: `run-1786698277941768000-2`
- Approval ID:
  `approval/run-1786698277941768000-2/implementation-approved`
- Approval presentation ID: `presentation/56ac79f4d7018f4f`
- Approval outcome: granted through the proof-enforced path
- Phase status: completed
- Event summary: 39 events, one approval, zero retries, zero escalations

Out-of-kernel work: Codex edited source/tests/docs and ran validation. The
kernel governed scope and approval but did not edit files, invoke a provider,
perform Git operations, or create a pull request.

## 12. Remaining Limitations

- No concrete GitHub transport or live sandbox proof exists for this family.
- The helper is explicit and internal; no executor or CLI path invokes it.
- The caller must obtain the current V3 assessment through the existing
  current-runtime-fact assessment path; this helper validates but does not
  discover runtime facts itself.
- Returned event payloads and report citations require an explicit later
  persistence/composition path.
- The provider API cannot atomically freeze mutable branch refs during create.
- This slice does not authorize another write family.

## 13. Recommended Next Phase

The focused maintainer and security review accepted this integrated slice. The
next provider step should be a concrete opt-in GitHub HTTP sandbox wiring and
live-smoke phase, not a third mutation family or default write behavior.
