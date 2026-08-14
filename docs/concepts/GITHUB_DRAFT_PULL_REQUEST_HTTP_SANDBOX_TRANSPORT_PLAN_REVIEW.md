# GitHub Draft Pull Request HTTP Sandbox Transport Plan Review

## 1. Executive Verdict

Plan accepted with fix-forward hardening; proceed to the integrated transport
implementation milestone.

The plan selects the correct next execution step: prove the accepted governed
draft-pull-request provider boundary against one concrete GitHub.com transport
and one environment-gated sandbox smoke. The review found no reason to add
another model-only phase, executor wiring, CLI mutation surface, schema, or
second provider family first.

## 2. Scope Verification

The phase remained planning-only. It did not call GitHub, create access
material, create or edit a pull request, push a branch, add a transport, enable
executor or CLI writes, add schemas or examples, or change release posture.

The planned implementation remains opt-in, local, GitHub.com-only, draft-only,
and behind the accepted governance helper. Default execution remains no-write.

## 3. Architecture Assessment

The separation is appropriate:

- Core retains governance, authority, approval-presentation, policy,
  SideEffect, evidence, report, idempotency, and reconciliation ownership.
- `GitHubDraftPullRequestHttpProvider<T>` maps the accepted provider trait onto
  exact HTTP operations.
- The injected transport performs one bounded exchange and cannot select an
  arbitrary endpoint, method, header, or payload shape.
- The live smoke is test-only, ignored by default, and does not become runtime
  configuration or a hidden production path.

This prevents technical access from becoming implicit Workflow OS authority.

## 4. GitHub REST Contract Assessment

The fixed host, media type, user agent, TLS, redirect, timeout, response-bound,
and no-retry posture is sufficient for the first slice. The review selects
`X-GitHub-Api-Version: 2026-03-10`, which GitHub currently lists as supported.
Implementation review must reverify the pin rather than relying on an
unversioned default.

The exact operations are appropriately narrow: observe refs, list exact open
pull-request candidates, and create one draft pull request. No edit, merge,
close, review, label, assignment, branch, content, or repository mutation is
authorized.

## 5. Target And Identity Assessment

The current target includes base owner/repository plus head owner, but no
separate head repository name. Guessing that a fork shares the base repository
name would create an identity ambiguity.

The plan is therefore hardened to support same-repository branches only in the
first transport proof. It must require `head_owner == owner` and observe head
and base refs in the same repository. Cross-repository pull requests remain
unsupported until the target model explicitly carries their identity.

## 6. Attempt And Ambiguity Assessment

The first implementation must not claim that a generic HTTP client proves no
bytes were sent. Validation, serialization, and request construction failures
before invoking the client are `NotStarted`. Once the client call boundary is
entered, transport errors are conservatively `MayHaveStarted` unless a stronger
guarantee is explicitly supported and tested.

The accepted status matrix is:

- `201` plus exact identity: created;
- `401`, `403`, `404`, or `422`: known rejected;
- `408`, `409`, `429`, redirects, other non-success statuses, and all `5xx`:
  may have started;
- timeout, disconnect, truncated or malformed success, and every uncertain
  post-call result: may have started.

Ambiguous POST outcomes must never retry automatically. Recovery remains exact
lookup plus operator-visible reconciliation.

## 7. Lookup And Idempotency Assessment

Lookup-before-create, the durable Workflow OS idempotency binding, exact
head/base identity, managed marker, content commitment, and single create
attempt form an acceptable first provider proof. They are not claimed as a
provider-native idempotency guarantee.

Multiple candidates, pagination, marker mismatch, moved refs, malformed
identity, or incomplete observation fail closed as conflict or ambiguity.

## 8. Access And Privacy Assessment

Access remains caller-supplied through the existing non-serializable wrapper.
Core may not inspect environment variables, keychains, GitHub CLI state, git
configuration, or credential stores. Only the ignored test harness may read a
dedicated environment variable and immediately construct the wrapper.

The planned permission set is appropriately limited to metadata/read posture,
contents read for refs, and pull requests write. Access values, raw payloads,
provider messages, marker text, branch/SHA values, and request bodies remain
excluded from errors, Debug, serialization, telemetry, evidence summaries, and
reports.

## 9. Live Smoke Assessment

The smoke must target a repository allowlisted by exact owner and repository in
test code. Environment variables may enable the smoke and provide access,
branch, and SHA fixtures but may not redirect it to an arbitrary repository.

The full governed fixture proves composition, not production human or
enterprise authority. The smoke may create or exactly reuse one managed draft
and must disclose that it leaves provider state behind. Automated cleanup is a
different mutation and remains excluded.

## 10. Test Assessment

The planned scripted tests cover URL encoding, bounded parsing, exact lookup,
conflict and ambiguity, draft-only request shape, redacted errors, no retry,
redirect rejection, oversized and malformed responses, and pre-provider gate
preservation. Implementation should add explicit assertions for:

- same-repository-only rejection before transport invocation;
- every status in the accepted create matrix;
- compile-time sandbox target allowlisting;
- no arbitrary environment-selected repository; and
- fixture approval being described only as test composition evidence.

## 11. Documentation Assessment

The plan and roadmap state clearly that transport and live smoke are not yet
implemented. The implementation milestone must update
`docs/security/github-token-scopes.md`, add operator invocation instructions for
the ignored smoke, and preserve the default read-only/no-write posture.

## 12. Blockers

None after fix-forward hardening in this review.

## 13. Non-Blocking Follow-Ups

- Reverify GitHub REST API version `2026-03-10` during implementation review.
- Select the exact dedicated sandbox repository in the implementation PR.
- Keep the concrete transport private to Core until its compatibility surface
  has real usage evidence.
- Revisit cross-repository identity only through a separately reviewed target
  model change.
- Consider bounded provider request-ID evidence only after the first proof; do
  not add it speculatively.

## 14. Governed Review Evidence

- Workflow: `dg/review`
- Run ID: `run-1786707813485381000-2`
- Approval ID:
  `approval/run-1786707813485381000-2/review-scope-approved`
- Approval presentation ID: `presentation/0db42d7d45a74eeb`
- Approval presentation content hash:
  `0db42d7d45a74eeb370192f551e81f58a88cb7aa1ebd96fd35772e6e21fa629d`
- Approval outcome: granted through the proof-enforced path by the delegated
  maintainer
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations

Out-of-kernel work: Codex inspected the plan, current provider target and trait,
prior HTTP boundaries, security posture, and current official GitHub REST API
documentation; performed the maintainer/security assessment; authored this
review; and will run documentation checks and git/PR operations. The kernel
governed scope and approval but did not browse documentation, edit files, run
checks, call GitHub, or perform git operations.

## 15. Recommended Next Phase

Implement the concrete GitHub.com request/response boundary, private transport,
provider mapping, scripted tests, GitHub token-scope documentation update, and
one ignored environment-gated sandbox smoke as a single vertical milestone.

Do not add executor or CLI mutation wiring, hidden access loading, Git
transport, cross-repository pull requests, non-draft operations, additional
provider writes, schemas, examples, hosted expansion, or release changes.
