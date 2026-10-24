# Provider-Neutral Trusted-Host Input Observer Plan Review

Status: focused maintainer/security review complete; one planning blocker
requires fix-forward before implementation.

Fix-forward note: the documentation-only blocker fix is implemented. The plan
now requires exactly one existing `TrustedHostRootObservation` for the
repository input root, exact static repository commitment binding, read-only
and non-invocation-scoped posture, coherent method/object/continuity facts, and
the focused regression cases identified below. The original finding is
preserved for auditability; focused re-review remains required before model
implementation.

Reviewed plan:
[Provider-Neutral Trusted-Host Input Observer
Plan](../implementation-plans/provider-neutral-trusted-host-input-observer-plan.md).

## 1. Executive Verdict

**Needs planning blocker fixes.**

The plan is directionally correct, provider-neutral, privacy-preserving, and
appropriately non-authorizing. It defines a coherent closed result for all nine
host-input obligations left unavailable by static discovery and preserves the
separation between discovery, observation, assessment, and admission.

One boundary is not explicit enough for implementation: the observer result
does not require an independently observed repository-root object binding. The
accepted workspace proof binds a caller-supplied repository commitment but
does not independently prove Git/object identity. Closing the nine host
obligations without binding the exact repository snapshot would leave the
planned observer unable to support its stated immutable-object-binding claim.

## 2. Scope Verification

The plan stayed within planning-only scope. It does not authorize:

- host inspection or environment reads;
- Cargo, compiler, linker, build-script, procedural-macro, or shell execution;
- a runner, sandbox, provider selection, or OpenShell integration;
- runtime admission, approvals, policy decisions, or authority grants;
- persistence, workflow events, reports, artifacts, schemas, SDKs, CLI
  behavior, or examples;
- credentials, provider mutations, writes, or release changes.

No implementation code or runtime behavior was introduced by the planning
phase.

## 3. Architecture Assessment

The plan correctly establishes four distinct layers:

1. static discovery proves only the accepted repository metadata graph;
2. host observation reports independently observed host facts;
3. a later pure assessment determines whether static and observed facts satisfy
   exact requirements; and
4. a separate admission path combines that assessment with current authority,
   policy, proportional governance, approvals, and durable attempt state.

This separation matches the accepted trusted-host live-security architecture.
In particular, neither observation construction nor a successful observation
result becomes reusable execution authority.

## 4. Exact Binding Assessment

The request binding is otherwise strong. It requires commitments for the
immutable run bundle, workflow/run/step, closed operation contract, static
discovery request/result, repository selection, security requirements,
target/features, attempt identity, and observer contract version.

The planned fixed-width canonical framing, pinned vectors, and substitution
tests are appropriate. Raw commands, paths, environment values, and mutable
repository files are correctly excluded as identity.

### Blocking gap: repository object observation

The result closes the nine `TrustedHostBuildInputHostObligationKind` values and
adds object-binding posture to each host fact. That does not independently
bind the repository root itself.

The plan mentions a repository root identity claim in the request and asks in
an open question how Git-independent binding should work. A request claim is
not observation evidence. The result must therefore include or bind an
existing `TrustedHostRootObservation` for
`TrustedHostRootRole::RepositoryInput`, or a narrowly equivalent composition,
with:

- the exact static-discovery repository commitment;
- read-only access posture;
- independently observed object-binding posture;
- continuity posture for the relevant interval;
- one closed observation method; and
- a stable accepted evidence reference.

The fix must reuse the accepted root-observation vocabulary unless a concrete
incompatibility is documented. It must not create a parallel root-binding
hierarchy.

## 5. Host Obligation Assessment

The plan represents all nine required host obligations exactly once:

- toolchain executable;
- target specification;
- SDK or sysroot;
- linker or archiver;
- native compiler tools;
- dynamic libraries;
- registry and package sources;
- effective environment inputs; and
- runtime base.

Requiring explicit unavailable, unsupported, or uncertain entries prevents
silent omission. The per-obligation guidance is sufficiently concrete for a
model-only phase while correctly deferring provider-specific observation
methods.

## 6. Trust And Provenance Assessment

The plan correctly states that `Observed` describes what an identified channel
reported, not that Core has accepted the producer implementation. Producer
identity, version, assurance, evidence reference, and observation interval are
required without making caller-supplied facts authoritative.

The later provider review remains responsible for deciding whether a producer
and method are trusted for live use. This prevents the model-only phase from
laundering assertions into evidence.

## 7. Freshness And TOCTOU Assessment

The plan correctly rejects canonical paths and preflight hashes as sufficient
live identity. It requires object identity, continuity, freshness, drift
detection, and reobservation after material request changes.

The exact freshness representation may remain an implementation-review topic
because the first model is non-authorizing. It must use ordered revisions or
bounded timestamps without turning wall-clock recency alone into authority.

## 8. Privacy And Redaction Assessment

The privacy boundary is appropriately strict. Public values are limited to
commitments, counts, closed postures, bounded identifiers, and accepted stable
references. Host paths, environment names/values, commands, logs, dynamic
library details, source/package payloads, endpoints, and credentials remain
excluded.

Debug output, serialization, validation, and deserialization are explicitly
non-leaking. The future implementation tests correctly include strict unknown
field/value rejection and secret-like input regressions.

## 9. OpenShell And Provider Boundary

The plan correctly keeps OpenShell optional and outside Core. It requires an
adapter to map only authoritative, version-pinned, exactly bound upstream facts
and forbids treating repeated requested configuration or sandbox labels as
proof.

The no-fork posture remains sound. A fork would require a separate ADR and a
demonstrated upstream governance-hook blocker plus ownership of the resulting
security surface.

## 10. Test Plan Assessment

The planned model tests cover canonicalization, substitution, completeness,
bounds, serde, privacy, and authority non-exposure. The blocker fix must add
explicit tests for:

- required repository-root observation presence;
- repository-root role uniqueness;
- static repository commitment substitution;
- incompatible root access/method/posture combinations;
- unavailable repository binding preventing a future satisfied assessment;
  and
- reuse of existing `TrustedHostRootObservation` semantics.

No live-host or provider test belongs in the model-only phase.

## 11. Planning Blocker

Before implementation, revise the plan so the observer result explicitly
binds one independently observed repository input root using the accepted
root-observation model. The request's repository claim alone is insufficient.

This is a narrow documentation fix. It does not require implementation,
provider selection, Git coupling, or choosing the eventual live observation
mechanism.

## 12. Non-Blocking Follow-Ups

- Decide whether the first contract version is explicitly Cargo/Rust while
  keeping its composition provider-neutral.
- Select or add a semantically correct hosted evidence reference kind during
  model implementation review.
- Pin environment influence classes before any live provider can claim
  completeness.
- Decide whether dynamic-library completeness requires runtime loader evidence
  before live admission.
- Define cache freshness without treating elapsed wall-clock time as sufficient
  authority.

## 13. Recommended Next Phase

Perform a focused planning blocker fix that adds exact repository-root
observation composition and its required tests to the plan. Then re-review only
that corrected boundary. If accepted, proceed to the provider-neutral observer
Core model.

Do not implement host observation, process execution, provider integration,
runtime admission, persistence, schemas, CLI behavior, examples, writes, or
release changes.

## 14. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792833481450938000-2`.
- Approval ID:
  `approval/run-1792833481450938000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/c5ffbe82ce9719aa`.
- Approval-presentation content hash:
  `c5ffbe82ce9719aa694b16462d9c85fde4536fcef3ccca46be10763cfefaba42`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker is present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected the accepted plan and
  relevant Core root-observation model, authored this review and roadmap
  update, and ran documentation validation. The kernel governed scope and
  approval but did not edit files, run validation, or perform git or
  pull-request operations.
