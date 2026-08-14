# GitHub Draft Pull Request HTTP Sandbox Transport Report

## 1. Executive Summary

Workflow OS now has one concrete, explicit, local GitHub.com transport for the
accepted managed draft-pull-request provider boundary. The implementation
executes only after the existing helper has validated caller-supplied current scoped authority,
write-policy, proof-enforced approval, terminal report-artifact, SideEffect,
current-fact, and exact-ref gates.

The transport remains private to Core. Callers receive an opaque provider from
`github_com_draft_pull_request_http_provider()` and cannot select a base URL,
method, endpoint, header set, retry posture, or response model. No executor,
CLI, schema, example, hosted path, or default write behavior was added.

## 2. Scope Completed

- Added an opaque concrete GitHub.com provider factory.
- Bound that factory to the compile-time `rcs2153/workflow-os-sandbox` target.
- Added a private typed request, response, candidate, and attempt-posture boundary.
- Added exact head/base ref observation through the Git references endpoint.
- Added exact open-PR lookup with bounded pagination and candidate handling.
- Added exactly one draft-PR create call with reviewed request fields only.
- Pinned GitHub REST API version `2026-03-10`.
- Added exact media type, user agent, HTTPS host, and timeout posture.
- Disabled redirects and connection pooling so the HTTP client cannot replay a
  pooled request.
- Compile-disabled dependency debug/trace logging so request URLs and query
  values cannot enter logs through the HTTP dependency.
- Added bounded response reading and private wire deserialization.
- Added deterministic known-rejection and may-have-started classification.
- Added scripted ordinary tests and one ignored full-helper live smoke.
- Updated token-scope, threat-model, security-review, roadmap, and plan posture.

## 3. Scope Explicitly Not Completed

This phase did not add automatic executor writes, CLI mutation behavior,
workflow schemas, SDK changes, examples, hidden auth loading, Git operations,
cross-repository PRs, non-draft PRs, PR edits, merges, closes, labels, reviews,
assignments, automatic cleanup, provider retries, hosted execution, production
credential management, another provider family, or release-posture changes.

## 4. Provider And Transport API Summary

`github_com_draft_pull_request_http_provider()` returns an opaque implementation
of the existing `GitHubDraftPullRequestProvider`. The concrete transport and
its typed HTTP request/response models are private to Core. Authentication is
read only from the explicit non-serializable provider request produced by the
accepted governed helper.

The concrete transport targets only `https://api.github.com` and rejects every
repository except `rcs2153/workflow-os-sandbox` before transport use. URL path segments
and query pairs are constructed through `url::Url`; branch slashes are encoded
as path data. The client uses bounded connect/read/write/total timeouts, TLS
verification through the existing HTTP dependency, zero redirects, and one
HTTP call per requested operation.

The authority objects consumed by the helper are validated proof objects from
trusted local callers. This slice does not authenticate an authority issuer or
consult a durable enterprise authority store, so it is not a defense against
malicious in-process callers.

## 5. Endpoint And Request Behavior

The provider performs:

1. `GET /repos/{owner}/{repo}/git/ref/heads/{branch}` for head and base refs.
2. `GET /repos/{owner}/{repo}/pulls` with exact open/head/base filters and
   `per_page=100`.
3. `POST /repos/{owner}/{repo}/pulls` with title, namespaced head, base, bounded
   body plus managed marker, `draft=true`, and
   `maintainer_can_modify=false`.

Same-repository head branches are required. The transport does not create or
push branches.

## 6. Failure And Ambiguity Behavior

Local validation, URL construction, and serialization fail before the HTTP
call. Once the create call enters the HTTP library boundary, transport loss,
timeout, malformed/truncated success, response overflow, redirects, and all
unclassified outcomes are treated as may-have-started ambiguity.

Only HTTP `401`, `403`, `404`, and `422` are known rejections. HTTP `201`
completes only when the response proves an exact managed draft with the expected
head/base identity. Every other status is ambiguous. The provider never
automatically retries create.

## 7. Privacy And Redaction

Request and response debug output redacts auth, URL, branches, SHAs, body,
marker, provider identity, and idempotency material. Raw provider bodies and
headers do not enter Core models, events, evidence, reports, diagnostics, or
errors. Private wire models retain only the fields needed to construct bounded
existing Core observations, and discard titles, users, URLs, and unrelated
provider fields. The Core build compile-disables dependency debug/trace logs,
and the HTTP agent disables pooling to prevent transparent replay.

## 8. Test Coverage

Focused tests cover the compile-time repository allowlist, encoded branch paths, exact ref mapping, exact lookup
filters, not-found/existing/conflict/ambiguity outcomes, pagination, reviewed
create fields, successful create, known rejection, redirect/rate-limit/server
ambiguity, pre-send versus post-send attempt posture, no retry, malformed and
oversized payloads, redaction, ignored provider fields, cross-repository
rejection, header-safe auth, dependency logging posture, and opaque
concrete-provider construction.

The existing full helper test fixture now includes an ignored live smoke. It
requires explicit opt-in, a dedicated token, pre-existing branches and exact
SHAs, and the compile-time allowlisted `rcs2153/workflow-os-sandbox` repository.
It runs the complete governed helper twice with independent local stores: the
first creates or reconciles the managed draft, and the second must reconcile it
without another create.

## 9. Live Smoke Invocation

The smoke is not part of ordinary tests or CI. Its required inputs are:

```text
WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_SMOKE=1
WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_TOKEN=<caller-supplied token>
WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_HEAD_BRANCH=<pre-pushed branch>
WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_HEAD_SHA=<full lowercase SHA>
WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_BASE_BRANCH=<existing base branch>
WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_BASE_SHA=<full lowercase SHA>
```

The token value must not be placed in shell history or committed files. The
managed draft remains external provider state after the smoke; cleanup is
manual and outside Workflow OS.

## 10. Commands Run And Results

Completed during implementation:

- `cargo fmt --all --check`: passed.
- focused HTTP transport unit tests: 13 passed after security hardening.
- `cargo test -p workflow-core --test github_draft_pull_request`: 10 passed,
  1 ignored live smoke.
- focused provider-auth safety tests: 2 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; the explicit live-provider smokes remained
  ignored by design.
- `npm run check`: passed, including docs, dogfood helper, integration helper,
  TypeScript, and contract checks.
- `npm run check:integrations`: passed.
- `git diff --check`: passed.

The ignored live smoke was not invoked because no pre-provisioned allowlisted
sandbox fixture was supplied during ordinary implementation.

## 11. Workflow Semantics And Side Effects

The transport does not mutate WorkflowRun state, append events, write report
artifacts, persist SideEffect records, or print CLI output. Those responsibilities
remain in the accepted explicit helper and stores. Any caller of the opaque
factory can affect only the compile-time sandbox repository, and only through
the accepted explicit helper. The ignored smoke is the maintained live proof
path, but it was not run during ordinary implementation.

## 12. Remaining Limitations

- The live smoke requires a separately provisioned sandbox repository and token.
- The caller-supplied authority proof is not an authenticated authority-store boundary.
- GitHub Enterprise Server and caller-selected API hosts are unsupported.
- Cross-repository PRs are unsupported.
- Git refs remain mutable observations rather than an atomic GitHub transaction.
- No request-ID evidence is retained.
- No automatic reconciliation or cleanup occurs after ambiguity.
- The transport is not wired into executor or CLI defaults.

## 13. Governed Implementation Evidence

- Workflow: `dg/implement`
- Run ID: `run-1786708518378558000-2`
- Approval ID:
  `approval/run-1786708518378558000-2/implementation-approved`
- Approval presentation ID: `presentation/6b9635a32c8976aa`
- Approval presentation content hash:
  `6b9635a32c8976aa1e378a9e06f9b100c59a7c9b34ff6a8ad9258a3f26b6032d`
- Approval outcome: granted through the proof-enforced path by the delegated
  maintainer
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations

Out-of-kernel work: Codex implemented the provider and transport, edited tests
and documentation, and ran the validation commands. Three independent
reviewers inspected code, tests, dependency behavior, and documentation. The
kernel governed scope and approval but did not edit files, run commands, use
GitHub access, call the live provider, or perform git and pull-request actions.
The live smoke was skipped because its separately governed sandbox fixture and
access were not provisioned in this phase.

## 14. Recommended Next Phase

Provision and execute the dedicated ignored GitHub sandbox smoke as a separate
governed proof phase, then reconcile its persistent draft and record the live
result. Do not broaden executor/CLI writes, repository targets, mutation
families, schemas, hidden auth, hosted behavior, or release posture first.
