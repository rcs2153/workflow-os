# GitHub Draft Pull Request Provider Mutation Plan

Status: Integrated Core helper implemented and accepted after phase-level
maintainer and security review. The implementation provides one explicit
injected provider boundary for draft pull request creation. It does not add Git
transport, a GitHub HTTP transport, automatic executor integration, CLI
behavior, schemas, or default write behavior.

## 1. Executive Summary

The first accepted Workflow OS provider-write sandbox can post a bounded GitHub
pull request comment through an explicit, approval-linked, SideEffect-governed
path. The durable project approval-route milestone is also merged and proven in
live PostgreSQL CI. The authoritative roadmap therefore permits choosing at
most one additional provider mutation family without enabling broad writes.

This plan selected **draft GitHub pull request creation from an already-pushed
branch** as that bounded candidate. The integrated Core helper is now
implemented behind explicit inputs and an injected provider trait.

The future slice creates only a draft pull request. It does not create or
modify commits, branches, refs, files, labels, reviewers, assignees, milestones,
checks, releases, or merges. Git transport remains outside the provider
mutation boundary. The caller must supply an existing remote head branch, its
expected head commit SHA, an exact base branch and observed base commit SHA,
and all governance facts explicitly. GitHub pull requests track mutable branch
refs, so those SHAs are governed observations rather than a false claim that
the provider can atomically freeze pull request content.

The candidate is useful because it closes a real Workflow OS dogfood gap while
remaining reviewable and reversible. It is more consequential than a comment,
so the first slice requires explicit current authority and a proof-enforced
blocking approval. This conservative first-slice requirement does not declare
that every future draft pull request must interrupt a human. Any future quiet
or delegated path must be separately justified by the accepted proportional-
governance model, current scoped authority, policy, evidence, and stewardship
minimums.

## 2. Why This Candidate Is Next

Draft pull request creation is the narrowest useful provider mutation that
advances the self-governed build loop after comments:

- Workflow OS already governs implementation, review, validation, PR hygiene,
  and release phases.
- Maintainers already push reviewed branches outside the kernel and then create
  draft pull requests through a separate GitHub client.
- A draft pull request is externally visible but does not merge work, alter the
  base branch, or authorize deployment.
- GitHub exposes stable repository, pull request, branch, and commit
  identifiers suitable for reconciliation.
- The operation can be exercised in this repository without inventing a second
  provider or a generic mutation interface.
- It composes existing authority, approval-presentation, SideEffect,
  idempotency, event, evidence, and WorkReport foundations.

The candidate is intentionally not pull request merge, branch push, repository
file mutation, workflow dispatch, check rerun, Jira issue mutation, or a generic
HTTP write.

## 3. Goals

- Define one exact future capability: `GitHubPullRequestCreate`.
- Keep Git transport and GitHub pull request metadata creation separate.
- Require an already-pushed head branch and an exact expected head observation
  before provider invocation.
- Bind the proposed mutation to one repository, head branch and SHA, base
  branch and observed SHA, run, report, policy decision, authority decision, approval,
  SideEffect, and idempotency key.
- Use the accepted proportional-governance assessment rather than a caller-
  selected interaction mode.
- Require current scoped authority at the time of provider use.
- Require proof that the exact approval scope was presented and granted.
- Reconcile duplicate, timeout, and provider/local ambiguity without automatic
  retry.
- Emit bounded provider outcome, event, evidence, and WorkReport references.
- Preserve explicit opt-in and sandbox-only posture for the first live proof.
- Deliver the future implementation as one integrated vertical slice with
  focused internal sequencing and one phase-level review.

## 4. Strict Non-Goals

This plan does not implement or authorize:

- a concrete GitHub HTTP transport or live network call;
- automatic pull request creation;
- non-draft pull request creation;
- local commits, branch creation, ref updates, pushes, force pushes, or Git
  credential handling;
- merge, close, reopen, convert-to-ready, review submission, labels, reviewers,
  assignees, milestones, projects, comments, or release publication;
- GitHub Actions dispatch, rerun, cancellation, or check mutation;
- Jira or another provider mutation;
- a generic provider-write adapter;
- automatic credential discovery or hidden auth loading;
- workflow schema or SDK changes;
- a public CLI mutation command in the first implementation;
- default executor integration;
- hosted production mutation behavior;
- enterprise identity, RBAC, SCIM, SSO, or administrative policy UI;
- recursive agents, agent swarms, or Level 3/4 autonomy defaults;
- reasoning lineage;
- examples or release posture changes.

## 5. Architecture Boundary

The future flow is:

```text
governed terminal run and immutable run bundle
  -> exact pushed head/base facts
  -> proportional-governance reassessment
  -> current scoped authority resolution
  -> proof-enforced approval when required
  -> proposed and attempted SideEffect record
  -> exact provider lookup/reconciliation
  -> injected GitHub draft-PR provider call when no equivalent PR exists
  -> provider outcome reconciliation
  -> Core-owned event, evidence, report, and artifact disclosure
```

Workflow OS owns the governance decision, immutable facts, authority,
approval, SideEffect lifecycle, event history, reconciliation posture,
evidence references, and report disclosure. The injected GitHub provider owns
only the bounded API lookup and draft pull request creation request.

The provider must not grant authority, approve work, create Core events, mutate
run state directly, or declare the workflow successful.

## 6. Git Transport Separation

Pull request creation is not permission to change source content.

Before the provider boundary is reached, the future caller must prove:

- the head branch already exists on the target GitHub repository;
- the supplied expected head SHA matches a current provider observation;
- the base branch already exists;
- the supplied base SHA matches the provider base observation used for the
  governed assessment;
- the immutable run bundle and report refer to the same governed work identity;
- no local push is requested or implied.

GitHub's create-pull-request API accepts branch names, not immutable commit
SHAs, and does not expose a conditional create primitive. The future approval
presentation must therefore disclose that the draft tracks mutable refs. A
pre-create read blocks when the observed head or base already differs from the
approved observation. A post-create read records the provider's actual head
and base observations. Movement in the unavoidable interval is surfaced as a
concurrent-ref-change reconciliation posture after an externally visible draft
may exist; it must not be described as atomic prevention, retried, hidden, or
automatically closed.

The provider adapter must never run `git`, infer a branch from the current
checkout, or push content. Readiness or merge of the draft is a later governed
action that must reassess the then-current commits.

## 7. Required Request Boundary

The future validated request should include only bounded typed values and
stable references:

- provider adapter identity and version;
- correlation ID;
- organization or trust-domain identity where applicable;
- project and repository identity;
- workflow ID and version;
- immutable run ID, bundle ID, and definition-root commitment;
- terminal WorkReport and report-artifact references;
- actor and current authority-snapshot reference;
- capability `GitHubPullRequestCreate`;
- repository owner and name or stable repository reference;
- exact head owner and branch reference plus expected commit SHA observation;
- exact base branch reference plus observed commit SHA;
- `draft = true` as a fixed first-slice invariant;
- bounded title template input;
- bounded report-derived body template references;
- policy decision reference;
- proportional-governance assessment and freshness binding;
- approval decision and approval-presentation proof when required;
- proposed SideEffect ID and persisted record reference;
- deterministic idempotency key;
- sensitivity and redaction metadata;
- bounded sandbox target proof for the first live smoke.

The request must not carry raw diffs, source files, command output, check logs,
provider payloads, tokens, authorization headers, environment values, prompts,
approval prose, or unbounded report text.

## 8. Pull Request Content Policy

The first implementation should not accept arbitrary free-form pull request
content.

Use one versioned bounded template that may render:

- a bounded work summary;
- explicit scope and non-goals;
- validation command names and pass/fail/skipped posture, not raw output;
- stable WorkReport, evidence, approval, and SideEffect references;
- known limitations and deferred work;
- one explicit marker that the pull request was created as a governed draft;
- one opaque, non-secret idempotency/reconciliation marker.

The template input must use validated WorkReport fields or explicit bounded
values. Diagnostic messages, raw logs, source contents, file diffs, provider
responses, secrets, and authorization material must not be copied by default.

Core may pass the rendered bounded title and body to the provider, but it
should persist only the template version, content commitment, stable
references, and bounded disclosure. Debug output and errors must never echo the
rendered body.

## 9. Governance Gates

Every future provider invocation must satisfy these gates in order:

1. **Immutable run gate:** the exact terminal run and bundle are durable and
   internally coherent.
2. **Report gate:** the terminal WorkReport and required report artifact exist,
   validate, and reference the exact run.
3. **Capability gate:** the requested capability is exactly
   `GitHubPullRequestCreate`.
4. **Target gate:** repository, head, base, and draft-only posture are bounded
   and match the approved sandbox target.
5. **Current provider-fact gate:** a bounded pre-create provider read observes
   the expected head and base SHAs, with the non-atomic branch-tracking boundary
   disclosed.
6. **Policy gate:** the exact policy set authorizes this capability and target.
7. **Proportional-governance gate:** a fresh accepted assessment is derived
   from authoritative workload and runtime facts.
8. **Authority gate:** the actor has fresh exact-project and exact-repository
   authority for capability reference `github.pull_request.create` at time of
   use. Generic `ExternalWrite`, `GitHubWrite`, or `ApprovalDecide` authority is
   insufficient.
9. **Approval gate:** the first slice requires a linked granted decision with
   fresh approval-presentation proof for the exact target and content
   commitment.
10. **SideEffect gate:** a durable proposed record exists and transitions to
    attempted before provider mutation.
11. **Idempotency gate:** a deterministic key and provider lookup strategy are
    present.
12. **Redaction gate:** all template inputs, metadata, Debug output, and errors
    are bounded and non-leaking.
13. **Mode gate:** the first live invocation is explicit, sandbox-bound, and
    disabled from default executor paths.

The token or GitHub installation credential proves only technical access. It
does not satisfy policy, authority, or approval.

## 10. Proportional Governance Posture

The caller must not select `Quiet`, `Visible`, `RequireApproval`, or `Denied`.
The accepted selector and runtime reassessment boundary derive execution and
disclosure posture from authoritative profile, policy, capability, authority,
evidence/check, sensitivity, SideEffect, and runtime facts.

For the first draft-PR slice, externally visible mutation plus the absence of
enterprise stewardship produces a blocking approval minimum. A later phase may
allow a sufficiently authorized policy to permit non-blocking draft creation,
but only after the selector, authority source, evidence obligations, and
stewardship minimums prove that posture. Quiet success would still require a
durable SideEffect, evidence, event, report, and provider receipt.

Visible disclosure remains an operator-presentation axis, not a separate
execution-permission category.

## 11. Current Authority And Approval

Stored project approval routes are historical routing evidence, not authority
to create a pull request. Before provider invocation, the future composition
must independently resolve current authority for:

- the exact actor;
- the exact organization/project/repository;
- exact capability reference `github.pull_request.create`;
- the exact head/base resource scope;
- the exact current authority revision and fingerprint;
- an expiry and revocation posture when the active authority model supports it.

The accepted generic `SideEffectCapability::GitHubWrite` may classify the
SideEffect family but must not grant authority. The accepted adapter capability
`GitHubPullRequestCreate` identifies preflight behavior but likewise does not
grant authority. If the current authority source cannot resolve the exact
capability reference and repository resource, implementation must stop rather
than map a broader grant implicitly. The first implementation is local and
explicit; extending the closed hosted project capability vocabulary is a later
separately reviewed boundary.

The approval must bind the complete mutation subject: repository, head/base
branches, observed SHAs, explicit mutable-ref tracking disclosure, draft-only
posture, template version and content commitment, run, report, policy,
capability, SideEffect, idempotency key, and actor.

The policy decision must be reconstructed from the exact run/step event trail
and validated policy set. A generic `external.write` policy allowance only
permits evaluation to continue; it does not grant target authority or satisfy
the exact capability gate. A caller-authored `Allowed` enum or policy reference
without durable decision proof is insufficient.

Approval-presentation proof must show that scope, non-goals, touched external
surface, validation/evidence posture, and next action were presented before the
decision. Missing, stale, mismatched, or unlinked proof blocks before provider
lookup or creation.

## 12. SideEffect Lifecycle

The future vertical slice should reuse existing SideEffect persistence and
transition helpers:

- create a durable `Proposed` record before approval or provider work;
- bind approval and authority to that exact record;
- transition to `Attempted` immediately before the provider create call;
- transition to `Completed` only after provider identity, draft posture,
  repository, head/base facts, and response reference reconcile;
- transition to `Failed` only when provider non-creation is known;
- preserve an explicit reconciliation-required posture when the provider may
  have created the pull request but Core cannot yet prove it.

No automatic retry is allowed after an ambiguous provider outcome.

## 13. Idempotency And Reconciliation

GitHub pull request creation is not assumed to be provider-idempotent.

Before creation, the injected provider must perform a bounded lookup by exact
repository, head owner/branch, and base branch. The result is reconciled with
the expected head/base observations, draft posture, and opaque Workflow OS
marker:

- no match: creation may proceed after all gates pass;
- one matching managed draft: return an existing-managed outcome and do not
  create a duplicate. If its current head or base differs from the approved
  observation, surface ref-drift posture and require reassessment before any
  later readiness or merge action;
- one non-equivalent match: fail closed with conflict posture;
- multiple matches or incomplete provider facts: fail closed with ambiguous
  reconciliation posture.

After timeout or transport ambiguity, perform one explicit lookup through the
same bounded interface. Do not automatically issue another create call.

The idempotency key must bind the exact project, repository, head/base refs,
approved SHA observations, run, SideEffect, content commitment, and operation
version. Reusing the key with different content or target facts is a conflict.
The managed marker establishes creation identity even if the branch later
moves; SHA drift is evidence and reassessment posture, not permission to create
a duplicate PR.

## 14. Provider Outcome And Evidence

A successful or reconciled result may record only bounded provider facts:

- provider pull request ID and number;
- stable pull request reference;
- provider repository reference;
- confirmed head/base SHAs;
- confirmed `draft = true` posture;
- provider creation or existing-equivalent classification;
- provider request/response timing and adapter version;
- content commitment and idempotency reference;
- bounded denied/conflict/reconciliation reason codes;
- referenced provider observation, event, evidence, and artifact identities.

Do not persist raw provider response bodies, rendered PR body text, access
material, source content, diffs, logs, or error payloads.

Core owns terminal event append, EvidenceReference construction, WorkReport
disclosure, and report artifact integrity. A provider success alone does not
prove local completion.

## 15. Failure Semantics

Fail closed before provider creation when any immutable-run, report,
capability, target, provider-fact, policy, proportional-governance, authority,
approval, SideEffect, idempotency, redaction, or sandbox gate is absent,
stale, contradictory, or invalid.

Pre-provider failure must not append a provider-attempt event or claim a pull
request exists.

After provider invocation:

- known provider rejection may produce a failed SideEffect and bounded failure
  event;
- known provider success must reconcile before local completion;
- transport timeout, local persistence failure, or incomplete provider
  response produces reconciliation-required posture;
- report or artifact failure does not retroactively change the governed
  workflow result, but it prevents the mutation phase from claiming complete
  evidence closure;
- no automatic retry or repair occurs.

Errors use stable codes and do not contain repository names, branch names,
SHAs, title/body text, paths, provider payloads, or access material.

## 16. Privacy And Security

The implementation must reject or avoid storing:

- GitHub tokens, installation credentials, authorization headers, and cookies;
- raw pull request bodies, issue bodies, comments, diffs, patches, source files,
  logs, command output, and provider payloads;
- environment variable values;
- raw approval or policy payloads;
- secret-like branch, title, summary, limitation, risk, or handoff values;
- private repository URLs where a stable redacted reference is sufficient.

Debug implementations redact target, branch, SHA, content, marker, and
idempotency values. Serialization exposes only the explicitly reviewed bounded
contract. Deserialization fails closed without echoing rejected values.

The first live proof must use a maintainer-controlled non-sensitive sandbox
target and a least-privilege credential scoped only to pull request creation on
that target.

## 17. Proposed Integrated Implementation Milestone

After plan review, implement one governed vertical slice with this internal
sequence:

1. Add the validated draft-PR target, request, provider outcome, and
   reconciliation models using existing capability vocabulary.
2. Add a draft-PR-specific readiness policy that does not broaden the default
   comments-only policy.
3. Add pure preflight composition and fixture provider tests with no network.
4. Add an injected transport interface for exact branch/PR lookup and draft PR
   creation.
5. Compose immutable run/report proof, proportional-governance reassessment,
   current authority, approval-presentation proof, SideEffect persistence, and
   idempotency gates.
6. Add provider lookup/create/reconciliation orchestration with no automatic
   retry.
7. Append Core-owned bounded events and create evidence/report/artifact
   references through existing gates.
8. Add one ignored-by-default live sandbox test or maintained smoke harness
   against a non-sensitive maintainer target.
9. Add runtime/security documentation, phase report, and focused maintainer and
   security review.

The milestone should not be split into routine model/helper micro-phases.
Blocker fixes remain separately governed when review finds an authority,
idempotency, provider-outcome, privacy, migration, or recovery defect.

## 18. Test Plan

Future tests must prove at least:

- valid draft-only request construction;
- non-draft request rejection;
- unsupported default policy rejection;
- exact capability and target binding;
- head/base provider fact equality and movement rejection;
- branch movement between the pre-create read and post-create read produces
  concurrent-ref-change reconciliation posture without duplicate creation or
  automatic cleanup;
- immutable run/report mismatch rejection;
- stale proportional-governance assessment rejection;
- missing, expired, revoked, wrong-project, wrong-resource, or wrong-capability
  authority rejection;
- missing, denied, stale, or mismatched approval-presentation proof rejection;
- missing or unlinked proposed SideEffect rejection;
- attempted transition before provider invocation;
- lookup-before-create ordering;
- exact existing-equivalent reconciliation without a create call;
- conflicting or ambiguous existing PR rejection;
- provider create invoked exactly once when all gates pass;
- timeout lookup recovery without automatic create retry;
- provider success/local persistence ambiguity preservation;
- deterministic idempotency and conflict detection;
- bounded template construction and content commitment;
- no raw diff, source, log, command, provider, or access-material storage;
- non-leaking Debug, serialization, deserialization, and errors;
- Core-owned event/evidence/report/artifact closure;
- no mutation through existing executor or CLI defaults;
- GitHub PR comment provider-write regression coverage;
- local executor, authority, approval, SideEffect, WorkReport, hosted, and
  PostgreSQL regression coverage;
- one maintained sandbox smoke before any capability is described as live.

## 19. Acceptance Criteria

The future implementation is acceptable only when:

- only a draft PR can be created;
- the branch already exists remotely and exact head/base SHAs remain current;
- the operation is explicit and sandbox-bound;
- current exact-capability authority, policy, proportional governance, and proof-enforced
  approval are checked immediately before provider mutation;
- a durable proposed/attempted SideEffect exists before the create call;
- provider lookup prevents or exposes duplicate/conflicting creation;
- ambiguity never triggers automatic retry;
- provider success is reconciled into Core-owned events, evidence, report, and
  artifact posture;
- default executor and CLI paths remain no-write;
- no raw payload or access material leaks;
- focused maintainer/security review accepts the result;
- all required local and live CI checks pass.

## 20. Open Questions For Implementation Review

- Should the first template expose a short visible Workflow OS marker or only
  an opaque non-rendered reconciliation marker?
- Which GitHub installation or fine-grained token permission is the least
  privilege needed for lookup plus draft creation?
- Should an existing exact equivalent draft PR complete the SideEffect as
  reconciled success or produce a distinct terminal lifecycle classification?
- Which report-artifact gate is mandatory before the first live create call?
- What exact sandbox repository and branch-retention policy should maintain the
  live proof without creating public noise?

## 21. Final Recommendation

Proceed next to a focused maintainer and security review of this plan.

If accepted, implement the integrated draft GitHub pull request creation
vertical slice in one governed milestone. Do not implement another provider
mutation family, Git transport, non-draft PR creation, PR merge, Jira mutation,
default runtime writes, workflow schemas, or enterprise administration.
