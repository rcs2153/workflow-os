# GitHub Draft Pull Request HTTP Sandbox Transport Plan Report

## 1. Executive Summary

Planning is complete for one concrete, opt-in GitHub.com HTTP transport and an
environment-gated live sandbox smoke behind the accepted managed draft pull
request provider boundary.

The plan closes the gap between the reviewed injected provider helper and a
real provider call without enabling automatic writes. It defines exact REST
operations, API/version/header posture, caller-supplied least-privilege access,
bounded parsing, create-attempt ambiguity, lookup-first reconciliation, and a
single persistent managed-draft smoke target. No network call or provider
mutation was performed by this phase.

## 2. Scope Completed

- Defined the concrete GitHub.com-only transport boundary.
- Defined exact ref observation, open-PR lookup, and draft creation operations.
- Defined structured URL/query construction and bounded response parsing.
- Defined fixed API-version, media-type, user-agent, TLS, redirect, timeout,
  host, and retry posture.
- Defined caller-supplied, non-serializable access handling and least-privilege
  repository permissions.
- Defined known-rejection, not-started, and may-have-started classifications.
- Preserved lookup-before-create and no automatic mutation retry.
- Defined one ignored, environment-gated live smoke against a dedicated
  repository and already-pushed branch.
- Defined persistent managed-draft disclosure rather than adding a cleanup
  mutation.
- Defined one integrated implementation milestone rather than another
  model-only phase chain.
- Updated the authoritative roadmap to identify planning status and the next
  focused review.

## 3. Scope Explicitly Not Completed

This phase did not:

- call GitHub;
- load or generate access material;
- create, update, close, merge, or delete a pull request;
- create a branch, commit, or push;
- implement a transport or test;
- add executor or CLI wiring;
- add schemas, SDK changes, or examples;
- add default provider writes;
- add another mutation family;
- add hosted expansion, enterprise identity, OpenShell, reasoning lineage, or
  release changes.

## 4. Architecture Decision

The accepted `execute_github_draft_pull_request_mutation` helper remains the
only governance and lifecycle orchestration boundary. A future
`GitHubDraftPullRequestHttpProvider<T>` will map its provider operations onto a
dedicated injected HTTP transport.

The transport will perform one bounded network exchange and parse reviewed
fields. It will not own governance, state transitions, event append, report
artifacts, workflow mutation, CLI output, or hidden access loading.

## 5. HTTP And Provider Posture

The first implementation targets exact `https://api.github.com`, pins one REST
API version, disables redirects and retries, uses bounded timeouts and response
sizes, and accepts no caller-selected endpoint, method, headers, or body shape.

The provider supports only:

1. observing exact head/base refs;
2. looking up an exact open managed draft; and
3. attempting one draft creation.

All unneeded response fields and raw provider payloads remain outside Core.

## 6. Ambiguity And Recovery

The plan explicitly separates failure before request start from uncertainty
after create begins. Timeout, connection loss after send, truncated success,
5xx, or malformed success becomes `MayHaveStarted` and maps to the existing
ambiguous outcome. The provider must not retry.

Recovery remains lookup-based and operator-visible. Exact existing managed
state may be reconciled. Conflict, multiple candidates, pagination, incomplete
identity, or moved refs fail closed or require operator action.

## 7. Live Smoke Posture

The future smoke is ignored by default and requires an exact enable flag,
dedicated repository allowlist, caller-supplied access, pre-existing branches,
exact SHAs, and a full governed fixture context.

It may create the one managed draft once. Later invocations must reuse it. The
draft remains externally visible until a maintainer removes it manually; the
test will not add a separate close/delete mutation under the guise of cleanup.

## 8. Privacy And Security

The future transport must not expose access values, headers, request bodies,
response bodies, provider messages, repository/branch/SHA values, managed
markers, or idempotency values through `Debug`, serialization, errors,
telemetry, evidence summaries, or report summaries.

The plan also requires an implementation-time update to the GitHub permission
guide so the single sandbox write identity is not confused with ordinary
read-only/default adapter posture.

## 9. Validation

Required planning validation:

- `npm run check:docs`
- `git diff --check`
- manual diff review for capability overclaims and sensitive-value leakage

Results:

- `npm run check:docs`: passed
- `git diff --check`: passed
- manual diff review: passed; the plan does not claim a transport or live
  provider proof exists, and it keeps access values and provider payloads out
  of durable/report surfaces

## 10. Governed Phase Evidence

- Workflow: `dg/d`
- Phase: `planning`
- Run ID: `run-1786707314943783000-2`
- Approval ID:
  `approval/run-1786707314943783000-2/planning-approved`
- Approval presentation ID: `presentation/fa4ba281a6cc3932`
- Approval presentation content hash:
  `fa4ba281a6cc393271d1b9625b37cd9110d880496bfa7cee18de1c9df9851b08`
- Approval outcome: granted through the proof-enforced path by the delegated
  maintainer
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations;
  `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`,
  `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`,
  `RunValidated:1`, `SkillInvocationRequested:6`,
  `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, and
  `StepScheduled:6`

Out-of-kernel work: Codex inspected the engineering standard, accepted provider
helper and review, prior GitHub comment HTTP boundary, security guidance, and
official GitHub REST documentation; authored planning documents; and will run
documentation validation and git/PR operations. The kernel governed scope and
approval but did not browse documentation, edit files, run checks, call GitHub,
or perform git operations.

## 11. Remaining Limitations

- No concrete draft-PR HTTP provider or transport exists.
- The implementation must reverify the selected GitHub REST API version
  `2026-03-10` against official documentation.
- Cross-repository pull requests are intentionally unsupported in the first
  transport slice because the target lacks a separate head-repository name.
- HTTP-library failures after the call boundary are conservatively
  `MayHaveStarted`; no pre-send guarantee is assumed.
- No live smoke has been run.
- The sandbox draft will require a dedicated repository and manual lifecycle
  policy.
- No executor, CLI, schema, example, or default write path is authorized.

## 12. Recommended Next Phase

The focused maintainer/security review is complete. Proceed to implement the
request/response types, concrete GitHub.com transport, provider mapping,
scripted tests, security-document update, and ignored live smoke as one bounded
vertical milestone.

See [GitHub Draft Pull Request HTTP Sandbox Transport Plan
Review](GITHUB_DRAFT_PULL_REQUEST_HTTP_SANDBOX_TRANSPORT_PLAN_REVIEW.md).
