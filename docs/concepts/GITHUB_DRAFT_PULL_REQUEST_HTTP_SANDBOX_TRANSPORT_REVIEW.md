# GitHub Draft Pull Request HTTP Sandbox Transport Review

Review date: 2026-08-14

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The concrete transport is acceptable for one explicit, compile-time
allowlisted sandbox repository after blocker fixes. It does not authorize a
general GitHub write adapter, executor or CLI wiring, another repository,
automatic retry, hidden authentication, or production use.

The ignored live smoke is implemented but was not run during this phase.
Accordingly, this review accepts the bounded implementation and scripted proof,
not a completed live provider proof.

## 2. Scope Verification

The phase stayed within the accepted transport slice. It added one private
GitHub.com transport behind the existing provider trait, one opaque public
factory bound to `rcs2153/workflow-os-sandbox`, scripted tests, an ignored live
smoke, and security/documentation updates.

It added no automatic executor path, CLI mutation, workflow schema, SDK
surface, Git transport, branch or commit creation, non-draft PR operation,
additional provider family, hosted behavior, production access management, or
release-posture change.

## 3. Architecture And Target Assessment

The accepted helper retains governance, policy, approval, authority-object,
SideEffect, artifact, evidence, and reconciliation ownership. The provider
maps only ref observation, exact lookup, and one create operation. The private
transport performs one bounded HTTP exchange.

Initial review found that the factory accepted any otherwise authorized GitHub
repository while only the smoke test was allowlisted. That exceeded the
sandbox phase boundary. The implementation now rejects every owner/repository
except the compile-time sandbox before URL or authorization-header
construction and before transport invocation. Same-repository branches remain
mandatory.

## 4. REST And Network Assessment

The transport fixes the base URL to `https://api.github.com`, pins API version
`2026-03-10`, sets the reviewed media type and user agent, verifies TLS, sets
bounded connect/read/write/total timeouts, disables redirects, and constructs
paths and queries through `url::Url`.

Initial review found that a pooled `ureq` connection can be replayed after a
stale-connection failure. Pooling is now disabled globally on this agent, so
the library cannot enter its recycled-connection retry branches. The create
body is non-empty and no retry middleware is configured.

Initial review also found that `ureq` debug logs include full URLs. Workflow
Core now enables the `log` compile-time maximum-level features at `Info`,
removing dependency debug and trace logging from debug and release builds.
`ureq` has no URL-bearing info/warn path in the reviewed version. This is an
intentional package-wide security tradeoff and should be reconsidered only
with an HTTP boundary that can guarantee redacted structured logging.

## 5. Request And Response Assessment

The create request contains exactly title, namespaced head, base, bounded body
plus managed marker, `draft=true`, and `maintainer_can_modify=false`. Ref and
lookup operations use fixed methods and reviewed endpoint shapes.

Response bodies are bounded before JSON parsing. Private wire structs retain
only PR number, draft posture, head/base identity, and bounded managed-content
match posture. Created responses now require a non-zero PR number plus exact
head branch, base branch, head SHA, base SHA, draft posture, and managed body
commitment. Missing or drifting identity cannot complete the SideEffect.

## 6. Failure, Retry, And Ambiguity Assessment

Validation and request construction fail before the HTTP call. Once create
enters the client boundary, transport loss and malformed or truncated success
remain may-have-started ambiguity. Only `401`, `403`, `404`, and `422` are
known rejection; every other non-`201` status remains ambiguous.

Initial review found that known rejection was disclosed as retryable. It now
sets `retry_blocked=true`, requiring remediation and a fresh governed attempt.
Ambiguous outcomes remain attempted and operator-reconcilable without retry.

## 7. Authentication And Privacy Assessment

Authentication remains caller-supplied, non-serializable, and redacted. The
shared auth wrapper now rejects non-ASCII-graphic bytes before request
construction, preventing control characters or invalid header bytes from
crossing the call boundary. Errors never include the rejected value.

Request and response Debug implementations redact URL, auth, repository,
branches, SHAs, content, marker, provider reference, and idempotency material.
Raw request/response payloads do not enter Workflow OS persistence, events,
evidence, reports, diagnostics, or errors.

## 8. Governance Boundary Assessment

Governance, approval-presentation, policy, artifact, authority-object, and
current-fact failures occur before provider observation. Tests now count every
provider method, not only create, and prove zero transport-facing provider calls
for those rejected paths.

The scoped authority input is still a trusted caller-supplied proof object.
This helper validates its consistency and binding; it does not authenticate an
issuer or query a durable enterprise authority store. Documentation now states
that limitation and does not portray the model as protection from malicious
in-process callers.

## 9. Live Smoke Assessment

The ignored smoke uses an exact enable flag, a dedicated access environment
variable, pre-existing branch/SHA fixtures, a fixed content marker, and the
compile-time sandbox repository. Running the ignored test without exact opt-in
now fails loudly instead of producing a false passing smoke.

The smoke invokes the full governed helper twice with independent local stores,
requires completed SideEffect state, evidence and report citations, and proves
the second call reconciles the exact managed draft without create. It leaves
the draft as disclosed provider state. It was not run in this phase.

## 10. Test Assessment

Focused coverage includes:

- exact target allowlisting and same-repository rejection before transport;
- branch encoding and exact ref SHA mapping;
- exact lookup filters and not-found/existing/conflict/ambiguity behavior;
- reviewed create fields and draft-only posture;
- exact created-response branch/SHA/body identity;
- malformed, incomplete, and oversized success handling;
- accepted rejection and ambiguity status matrices;
- pre-call and post-call attempt posture;
- no transparent retry posture;
- zero provider calls before governance acceptance;
- auth-header safety and non-leakage;
- Debug and dependency-log safety; and
- ignored full-helper create-or-reconcile behavior.

The concrete client configuration remains primarily code-reviewed rather than
socket-instrumented in ordinary tests. The disabled pool and compile-time log
level are directly asserted where the dependency exposes stable posture.

The final validation matrix passed: workspace formatting, workspace clippy
with warnings denied, the complete Rust workspace suite, repository checks,
integration checks, documentation checks, and diff whitespace validation. The
live sandbox smoke remained ignored because this phase did not provision or
use write access.

## 11. Documentation Assessment

The roadmap, implementation plan/report, token-scope guidance, threat model,
and security review now distinguish:

- accepted provider-neutral helper behavior;
- implemented but newly reviewed concrete transport behavior;
- scripted proof from unexecuted live smoke;
- trusted authority-object validation from issuer authentication; and
- explicit sandbox mutation from ordinary read-only/default-no-write posture.

Stale claims that no HTTP transport existed or that two live proofs had already
completed were removed.

## 12. Blockers

None after fix-forward hardening.

## 13. Non-Blocking Follow-Ups

- Run the ignored smoke only after the dedicated sandbox repository, branch
  fixtures, and least-privilege access are provisioned and reviewed.
- Preserve the compile-time repository binding until a separately reviewed
  target-classification or sandbox-registration model exists.
- Revisit package-wide debug-log suppression only when a replacement HTTP
  boundary can guarantee redacted structured logs.
- Replace trusted caller-supplied authority proof objects with a reviewed
  authenticated authority source before production or adversarial in-process
  security claims.
- Keep cross-repository PR identity and all additional mutation families
  separately scoped.

## 14. Governed Review Evidence

- Workflow: `dg/review`
- Run ID: `run-1786712566319995000-2`
- Approval ID:
  `approval/run-1786712566319995000-2/review-scope-approved`
- Approval presentation ID: `presentation/872832372f255368`
- Approval presentation content hash:
  `872832372f2553687c63eaa1286a8072a6bb6bdf270a06c334655b5fee8e76b8`
- Approval outcome: granted through the proof-enforced path by the delegated
  maintainer
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations

Out-of-kernel work: Codex and three independent reviewers inspected code,
tests, dependency behavior, plans, reports, roadmap, and security documents.
Codex applied blocker fixes, authored this review, and ran validation. The
kernel governed scope and approval but did not edit files, run checks, call
GitHub, use access material, or perform git/PR actions.

## 15. Recommended Next Phase

Provision and execute the dedicated ignored GitHub sandbox smoke as a separate
governed proof phase, then reconcile its persistent draft and record the live
result. Do not broaden executor/CLI writes, repository targets, mutation
families, schemas, hidden auth, hosted behavior, or release posture first.
