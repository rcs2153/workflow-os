# GitHub Draft Pull Request HTTP Sandbox Transport Plan

Status: Plan accepted with fix-forward hardening; implementation not started.

## 1. Executive Summary

Workflow OS now has an accepted Core helper that can create or reconcile one
managed draft GitHub pull request from an already-pushed branch after exact
governance, authority, approval-presentation, report-artifact, SideEffect, and
current-fact gates pass. The helper consumes an injected
`GitHubDraftPullRequestProvider`; it does not contain a GitHub HTTP transport.

This plan defines the next bounded milestone: one concrete, opt-in GitHub REST
provider and transport plus one environment-gated live sandbox smoke. The
transport will implement the accepted provider trait without changing the
governance helper, loading authentication from hidden state, retrying provider
mutation, or enabling any default executor or CLI write path.

The implementation will remain local and explicit. Ordinary tests will use a
scripted injected transport and perform no network I/O. The live smoke will be
ignored by default, require an explicit enable flag and caller-supplied access
material at the test-harness boundary, and target one dedicated repository with
an already-pushed branch. It may create or exactly reconcile one managed draft
pull request. It will not create a branch, push commits, merge, close, delete,
label, assign, request review, or modify repository contents.

## 2. Why This Phase Is Next

The provider-neutral draft pull request helper and its security review are
accepted. The remaining proof gap is concrete transport behavior:

- exact REST endpoints and API version;
- request construction and URL encoding;
- bounded response parsing;
- provider error and ambiguity classification;
- no-redirect, no-retry network posture;
- least-privilege caller-supplied access;
- an actual sandbox-repository smoke that proves the provider seam; and
- truthful disclosure of persistent provider state.

Implementing another mutation family before this proof would repeat model and
helper work without proving that the current boundary survives a real provider
call. Adding automatic executor or CLI wiring would be premature because the
transport itself has not yet been tested against GitHub.

## 3. Goals

- Implement `GitHubDraftPullRequestProvider` through a dedicated injected HTTP
  transport.
- Support exact ref observation, exact open-pull-request lookup, and one draft
  pull request create request.
- Pin the GitHub REST API version and required headers.
- Construct URLs and query parameters without raw branch-name interpolation.
- Parse only bounded fields required by the accepted provider model.
- Classify create failures into proven not-started, known rejection, or
  may-have-started ambiguity.
- Preserve lookup-before-create and no automatic mutation retry.
- Keep access material non-serializable, redacted, caller-supplied, and outside
  durable Workflow OS records.
- Add comprehensive scripted-transport tests that run without network access.
- Add one ignored, environment-gated live smoke against a dedicated sandbox
  repository and pre-provisioned branch.
- Preserve the accepted Core helper, SideEffect lifecycle, evidence, and
  report-ready citation behavior.

## 4. Strict Non-Goals

This phase does not authorize:

- automatic executor integration;
- a CLI pull request creation command or flag;
- workflow schema or SDK changes;
- default provider writes;
- hidden environment, keychain, GitHub CLI, git-remote, or config discovery in
  Core;
- authentication persistence or refresh;
- Git branch creation, commit creation, push, force-push, or other Git
  transport;
- non-draft pull requests;
- pull request merge, close, reopen, edit, review, label, assignment, or
  reviewer requests;
- repository content, settings, checks, releases, actions, or administration
  mutation;
- automatic provider retries;
- automatic cleanup through an additional GitHub mutation;
- Jira or another provider mutation family;
- hosted or distributed execution expansion;
- production access-material management;
- enterprise identity or administration;
- OpenShell integration;
- reasoning lineage, recursive agents, agent swarms, or Level 3/4 autonomy;
- examples or release-posture changes.

## 5. Accepted Baseline

The implementation must reuse, not duplicate, the accepted:

- `GitHubDraftPullRequestTarget`;
- `GitHubDraftPullRequestContent` and content commitment;
- `GitHubDraftPullRequestProviderRequest`;
- `GitHubDraftPullRequestProvider`;
- ref observation, lookup, create-outcome, and provider-observation models;
- `execute_github_draft_pull_request_mutation` orchestration helper;
- exact terminal WorkReport artifact identity validation;
- current V3 proportional-governance reassessment;
- current scoped capability resolution;
- adapter-write policy semantics;
- proof-enforced granted approval marker;
- SideEffect persistence and lifecycle transitions;
- lookup-first idempotency and no-retry ambiguity posture; and
- evidence and WorkReport citation construction.

The transport must not become an alternate path around those gates.

## 6. Architecture Boundary

The future call chain is:

```text
explicit caller
  -> execute_github_draft_pull_request_mutation
  -> accepted governance and SideEffect gates
  -> GitHubDraftPullRequestHttpProvider
  -> injected GitHubDraftPullRequestHttpTransport
  -> GitHub REST API
  -> bounded typed response
  -> accepted reconciliation and lifecycle logic
```

Core continues to own whether the mutation may be attempted and how its result
is reconciled. The HTTP transport owns one network exchange at a time. GitHub
owns the provider state. Possession of technical access material is never
treated as Workflow OS authority.

The provider must not write Workflow OS state, append workflow events, write a
WorkReport artifact, print CLI output, or mutate a `WorkflowRun`. Those remain
the responsibility of already-reviewed explicit boundaries.

## 7. GitHub REST Contract

The first implementation targets GitHub.com only:

- base URL: exact `https://api.github.com`;
- accepted media type: `application/vnd.github+json`;
- explicit `User-Agent` identifying Workflow OS preview behavior;
- exact `X-GitHub-Api-Version: 2026-03-10`, recorded in code and tests;
- TLS certificate verification enabled;
- redirects disabled; and
- bounded connect and response timeouts.

GitHub Enterprise Server and caller-selected base URLs remain deferred. This
prevents access-header forwarding to an unreviewed host and avoids claiming
compatibility with a different API surface.

GitHub currently documents `2026-03-10` as a supported REST API version. The
implementation review must reverify that pin against official documentation;
compatibility must not be inferred from the repository's older read-only
client or from an unversioned request default.

## 8. Required Operations

### 8.1 Observe Head And Base Refs

Use the Git references endpoint to observe:

```text
GET /repos/{owner}/{repository}/git/ref/heads/{encoded-branch}
```

The first transport proof supports same-repository pull requests only. It must
require `head_owner == owner` and observe both refs in the target repository.
Cross-repository pull requests require a distinct head-repository identity and
remain deferred rather than being guessed from the base repository name. Only
the commit SHA required by
`GitHubDraftPullRequestRefObservation` is retained. The transport must reject:

- missing or malformed commit SHA;
- non-commit ref objects;
- response-body overflow;
- unexpected success shapes;
- redirects; and
- unclassified transport failures.

Branch names must be encoded as path data. A slash in a branch name must not be
interpreted as an unvalidated URL path assembled by string concatenation.

### 8.2 Lookup Existing Open Pull Requests

Use the pull request list endpoint with exact filters:

```text
GET /repos/{owner}/{repository}/pulls
  ?state=open
  &head={head-owner}:{head-branch}
  &base={base-branch}
  &per_page=100
```

The transport parses only bounded fields needed to construct the existing
observation model: pull request number/reference, draft posture, head SHA, base
SHA, and managed-marker/content-match posture.

Rules:

- zero exact candidates becomes `NotFound`;
- one exact managed candidate becomes `Existing`;
- a non-managed or mismatched candidate becomes `Conflict`;
- multiple candidates, pagination beyond the bounded page, malformed entries,
  or incomplete identity becomes `Ambiguous`;
- lookup never edits an existing pull request; and
- raw titles, bodies, URLs, user objects, labels, reviews, and provider payloads
  are not retained or emitted.

The managed marker may be inspected only inside the redacted provider boundary.
It must not appear in `Debug`, errors, telemetry, evidence summaries, or report
summaries.

### 8.3 Create One Draft Pull Request

Use exactly:

```text
POST /repos/{owner}/{repository}/pulls
```

The bounded request body contains only:

- validated title;
- namespaced head identity;
- validated base branch;
- validated body containing the managed marker;
- `draft: true`; and
- `maintainer_can_modify: false` for the first sandbox proof.

The transport must not add labels, reviewers, assignees, projects, milestones,
auto-merge, or merge configuration.

A successful response must provide enough bounded data to construct a
`GitHubDraftPullRequestObservation`. Success without a pull request number,
draft posture, exact head/base identity, or marker match is invalid and must not
be reported as completed.

## 9. Proposed Rust Boundary

Add the smallest types required behind the existing module boundary, likely:

- `GitHubDraftPullRequestHttpProvider<T>`;
- `GitHubDraftPullRequestHttpTransport`;
- a non-serializable `GitHubDraftPullRequestHttpRequest`;
- a bounded `GitHubDraftPullRequestHttpResponse` or operation-specific response
  variants; and
- a transport attempt-posture error/result that distinguishes `NotStarted`
  from `MayHaveStarted` conservatively.

The exact names may follow local conventions, but the responsibilities must
remain separate:

- provider: maps accepted provider requests to exact HTTP operations and maps
  bounded responses to existing provider outcomes;
- transport: performs one request and parses only reviewed fields;
- orchestration helper: retains all governance, persistence, reconciliation,
  evidence, and report behavior.

Request types that contain access headers, title/body content, branches, SHAs,
markers, or idempotency values must not implement serialization and must use
fully redacted `Debug` output.

Response types must not retain raw response bodies or unrestricted headers.

## 10. Access And Permission Posture

Core accepts access material only through the existing explicit,
non-serializable wrapper. Core must not read environment variables, files,
keychains, GitHub CLI state, or operating-system credential stores.

The live-smoke test harness may read one dedicated environment variable and
immediately construct the wrapper. It must not print the variable, value,
prefix, header, length, or parse failure details.

The dedicated GitHub identity should be repository-scoped and grant only the
permissions required by the three operations:

- repository metadata/read posture required by GitHub;
- contents read for exact ref observation; and
- pull requests write for draft creation.

It must not grant contents write, administration, actions write, checks write,
secrets write, workflow dispatch, or broader organization access.

Before implementation, update `docs/security/github-token-scopes.md` so it no
longer implies that the accepted sandbox write proof is part of the old Phase 2
read-only posture. The security document must continue to state that ordinary
adapters and default operation remain read-only/no-write.

## 11. HTTP Safety Requirements

- Use the existing HTTP dependency; add no network crate unless review proves
  it necessary.
- Disable redirects so the access header cannot cross origins.
- Accept only the exact GitHub.com API host and HTTPS scheme.
- Apply bounded connect, read, and total-operation timeouts.
- Bound every response body before JSON parsing.
- Parse JSON into private wire structs, then validate into existing Core types.
- Ignore unneeded provider fields rather than retaining them.
- Reject response success accompanied by malformed required fields.
- Do not log request URLs, query values, request bodies, response bodies, or
  headers.
- Do not expose provider messages in `WorkflowOsError`.
- Do not treat GitHub rate-limit or request identifiers as authority.
- Do not follow provider links from response payloads.
- Do not allow caller-selected methods, endpoints, headers, or body fields.

## 12. Failure And Attempt Posture

Read-only ref observation and lookup failures occur before create and return
stable, non-leaking provider errors.

Create requires stricter classification. The concrete client may report
`NotStarted` only for validation, serialization, or request construction that
fails before invoking the HTTP library. Once the library's send/call boundary
is invoked, any transport-layer error is `MayHaveStarted` unless the selected
library exposes and tests a stronger no-bytes-sent guarantee. The first slice
must not depend on such an unproven guarantee.

The status matrix is:

- local validation or request-construction failure: proven `NotStarted`;
- HTTP `201` with exact required identity: `Created`;
- explicit HTTP `401`, `403`, `404`, or `422`: bounded `KnownRejected` because
  the endpoint returned a terminal non-success response;
- every other non-success HTTP status, including `408`, `409`, `429`, and all
  `5xx` responses: `MayHaveStarted`;
- timeout, connection loss after send, truncated success, 5xx response,
  malformed success, or any uncertainty after request start:
  `MayHaveStarted` mapped to the existing `Ambiguous` outcome;
- redirect response: rejected and treated as `MayHaveStarted` after the POST
  boundary; and
- the provider response body is never used to widen the status classification
  or copied into an error.

An ambiguous create must never be automatically retried. The accepted helper
leaves the SideEffect attempted and requires operator/provider reconciliation.
The transport must preserve that posture rather than collapse ambiguity into a
generic failure.

## 13. Idempotency And Reconciliation

GitHub's create-pull-request endpoint is not assumed to provide a native
idempotency guarantee. The first implementation therefore relies on:

- the existing durable Workflow OS idempotency binding;
- exact open-PR lookup before create;
- a bounded managed marker and content commitment;
- exact target, head, and base identity;
- a single create attempt; and
- lookup-based operator reconciliation after ambiguity.

The HTTP provider must not implement automatic retry middleware. Repeating a
transport call behind the Core helper would violate the accepted mutation
semantics even if a generic HTTP library considers the request retryable.

## 14. Live Sandbox Smoke

The live smoke is a separate ignored integration test. It is not part of
ordinary `cargo test --workspace`, CI, examples, or the public CLI.

It requires all of the following:

1. An exact opt-in environment flag.
2. Caller-supplied access material through a dedicated environment variable.
3. A dedicated sandbox repository allowlisted in test code by exact owner and
   repository. Environment variables may enable the smoke and supply access,
   branch, and SHA fixtures, but may not select an arbitrary repository.
4. A pre-existing base branch and already-pushed head branch.
5. Exact expected head and observed base SHAs supplied to the governed fixture.
6. A fixed template/marker version reserved for the smoke.
7. A full accepted test-fixture run, report artifact, authority, policy,
   approval presentation, and SideEffect store context exercising the real
   helper. Fixture approval proves model composition only; it must not be
   described as production human or enterprise authority.
8. A post-call provider lookup proving one exact managed draft.
9. Redaction assertions over errors, `Debug`, and test output.
10. A final disclosure that the draft remains provider state.

The smoke may create the managed draft once. Later runs must reconcile that
same open draft rather than create duplicates. The test must not close or
delete the draft because that would add a separate mutation family. Manual
cleanup, if desired, is outside Workflow OS and must be disclosed.

The test must exit before network access when any allowlist, branch, SHA,
access, or explicit opt-in value is absent or invalid.

## 15. Evidence And Reporting

The concrete transport must preserve the existing evidence boundary:

- stable pull request reference only after exact reconciled completion;
- SideEffect lifecycle and report citations produced by the accepted helper;
- bounded status and operator-action disclosure for ambiguity/conflict;
- no raw request or response payload;
- no title/body/marker copy into evidence summaries;
- no branch, SHA, repository, access, or provider-message leakage through
  errors; and
- no claim that the provider created a draft when the outcome is ambiguous.

The live-smoke report should disclose:

- whether the draft was created or exactly reused;
- whether lookup and post-create observation ran;
- whether retry was blocked;
- the stable provider reference only through the accepted bounded result;
- validation commands; and
- that the draft remains externally visible provider state.

## 16. Test Plan

Focused scripted-transport tests must cover:

1. Valid head/base ref observations map exact SHAs.
2. Branch names are safely encoded.
3. Exact lookup with no candidate returns not found.
4. One exact managed draft returns existing.
5. Mismatched marker/content returns conflict.
6. Multiple candidates or pagination returns ambiguous.
7. Create request is always draft and disables maintainer modification.
8. Create request includes only the reviewed fields.
9. Successful create returns an exact bounded observation.
10. Success missing required identity is ambiguous/invalid, never completed.
11. Known provider rejection is classified without raw payloads.
12. Pre-send failure remains not started.
13. Post-send timeout or truncation becomes may-have-started ambiguity.
14. No create retry occurs.
15. Redirects are rejected.
16. Non-GitHub hosts and non-HTTPS base URLs are rejected.
17. Oversized response bodies fail closed.
18. Malformed JSON fails closed without payload leakage.
19. Request/response `Debug` is redaction-safe.
20. Serialization cannot carry access material or provider payloads.
21. Existing helper governance gates still fail before transport invocation.
22. Existing draft-provider tests remain green.
23. Existing GitHub PR comment provider tests remain green.
24. Workspace tests remain green.

The ignored live smoke must prove create-or-reconcile behavior, exact ref
observation, no duplicate draft, bounded evidence/report output, and persistent
draft disclosure. It must be separately invoked and separately reported.

## 17. Proposed Implementation Sequence

Implement the next milestone as one reviewed vertical slice:

1. Focused maintainer/security review of this plan.
2. Dedicated request, response, attempt-posture, and injected transport types.
3. `GitHubDraftPullRequestHttpProvider<T>` implementing the accepted provider
   trait.
4. Scripted-transport tests for all three operations and ambiguity semantics.
5. One concrete GitHub.com transport with fixed host, headers, API version,
   timeouts, bounded parsing, no redirects, and no retry.
6. Updated GitHub access-scope security documentation.
7. One ignored environment-gated live smoke and operator run instructions.
8. Full maintainer/security review before any executor, CLI, schema, default,
   or additional provider-write work.

Do not split this into another long sequence of model-only phases. Request
types, concrete transport, provider mapping, scripted proof, and the ignored
smoke belong in one implementation milestone after plan acceptance.

## 18. Acceptance Criteria

- The concrete provider implements the accepted injected provider trait.
- Ordinary tests make no network calls.
- All request and response surfaces are bounded and redaction-safe.
- GitHub.com host, API version, headers, TLS, timeout, and redirect posture are
  explicit.
- Lookup-before-create remains mandatory.
- Create is attempted at most once.
- Ambiguous post-send outcomes never become known failure or automatic retry.
- The live smoke is ignored by default and requires exact opt-in and allowlist.
- The smoke uses an already-pushed branch and creates or reuses only a draft.
- The smoke supports same-repository branches only; cross-repository identity
  remains unsupported until the target model is explicitly extended.
- The smoke does not close, merge, edit, or otherwise clean up provider state.
- Access material never enters Core persistence, events, evidence, reports,
  diagnostics, logs, or serialized models.
- No executor, CLI, schema, example, hosted, default-write, or broader mutation
  behavior is introduced.

## 19. Review Decisions And Deferred Questions

The focused review fixes the first implementation posture as follows:

- pin GitHub REST API version `2026-03-10` and reverify it during implementation
  review;
- keep the concrete transport private to Core for the first slice;
- classify all HTTP-library errors after the call boundary as
  `MayHaveStarted` unless a tested stronger guarantee exists;
- require the existing dependency to prove redirects and retries are disabled;
  otherwise use a narrow configuration wrapper without adding a second client
  abstraction;
- omit GitHub request identifiers from evidence in the first slice;
- support same-repository branches only; and
- require a compile-time exact sandbox repository allowlist, with the concrete
  repository selected and reviewed in the implementation PR.

Cross-repository pull requests, public transport API stability, request-ID
evidence, automated cleanup, and broader sandbox lifecycle management remain
separately scoped future questions.

## 20. Final Recommendation

Proceed next to a focused maintainer and security review of this plan. If
accepted, implement the request/response boundary, concrete GitHub.com
transport, provider mapping, scripted tests, security documentation, and one
ignored live smoke as a single vertical milestone.

Continue to reject automatic executor or CLI writes, hidden access loading,
Git transport, non-draft pull requests, merge/close/edit operations, additional
provider mutations, schemas, examples, hosted expansion, and release changes.

## 21. References

- [GitHub REST pull request endpoints](https://docs.github.com/en/rest/pulls/pulls)
- [GitHub REST Git reference endpoints](https://docs.github.com/en/rest/git/refs)
- [GitHub REST API versioning](https://docs.github.com/en/rest/about-the-rest-api/api-versions)
- [GitHub Draft Pull Request Provider Mutation Plan](github-draft-pull-request-provider-mutation-plan.md)
- [GitHub Draft Pull Request Provider Mutation Review](../concepts/GITHUB_DRAFT_PULL_REQUEST_PROVIDER_MUTATION_REVIEW.md)
