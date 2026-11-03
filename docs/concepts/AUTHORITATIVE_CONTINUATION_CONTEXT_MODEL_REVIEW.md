# Authoritative Continuation Context Model Review

## Decision

Accepted after focused blocker fixes.

The first comprehensive-context increment is a sound Core-private,
non-authorizing model boundary. It may proceed to separately governed collector
planning. It does not authorize collectors, persistence, accepted-intent
capture, consumer-visible projection, CLI or schema exposure, runtime
integration, compaction signaling, provider work, or Salsa adoption.

## Reviewed Scope

The review covered:

- the closed continuation purpose, source, authority, posture, and gap models;
- private inventory and purpose-bound bundle derivation;
- accepted-intent supersession and active-run precedence;
- source-coverage completeness and collector-outcome consistency;
- deterministic commitments and input-order independence;
- access, sensitivity, freshness, integrity, and budget behavior;
- understanding-claim provenance and conflict posture;
- Debug and projection leakage boundaries;
- non-authorizing behavior; and
- the roadmap and implementation report claims.

## Blocking Findings Resolved

### Source class could claim a stronger authority domain

The initial implementation validated claim-kind/domain compatibility but did
not bind a source class to its exact authority domain. An advisory source could
therefore have claimed approval-domain precedence. Core now enforces a closed
one-to-one source-class/authority-domain mapping before claim derivation.

### Collector coverage could omit an unimplemented source class

The initial implementation required coverage only for supplied sources. That
could allow an incomplete collector set to appear complete. Coverage now must
account for every closed source class exactly once. Required, optional, and
inapplicable posture remains explicit, and observed/empty/failure posture must
agree with whether a source was supplied.

### Coverage was not independently purpose-bound

The decision bound purpose indirectly, but the coverage artifact and its
commitment did not. Coverage now carries the exact purpose and commits to the
purpose plus the complete collector matrix.

### Later intent could override an active immutable objective

Accepted intent initially outranked immutable run input for objective claims.
That would let later conversational direction rewrite an active run's bound
objective. Objective and amendment precedence is now scope-aware: immutable run
input wins when a run identity exists, while accepted intent remains primary
before a run exists. Priority and stop/pause/resume orientation remain separate
and non-authorizing.

### Bundle and understanding commitments under-bound selected context

The initial bundle commitment included source IDs and materialized byte counts,
but not the selected source definitions. Changed context could therefore retain
the same bundle commitment. The bundle now commits to the complete selected
source definitions, and the understanding commitment binds to the bundle
commitment plus derived claims.

### Partial run scope was accepted

A run identity without a workflow identity was representable. Core now rejects
that partial scope before derivation.

## Accepted Security Properties

- The module is private to `workflow-core` and is not re-exported.
- The complete inventory and semantic values have no public consumer surface.
- Every output is explicitly non-authorizing; no authority token is created.
- Required sources are admitted before optional sources under budget pressure.
- Denied and above-ceiling sources do not expose source identity through the
  bundle or understanding claim.
- Equal best-authority values conflict deterministically instead of being
  resolved by discovery order or model preference.
- Current accepted intent is bounded and superseded intent cannot revive old
  direction.
- Active immutable input outranks later objective/amendment intent.
- Complete closed-class collector accounting is mandatory.
- Purpose, scope, actor, selected content, claims, and final posture are bound
  through deterministic domain-separated commitments.
- The same explicit inputs produce the same result independent of source order.

## Non-Blocking Follow-Ups

Before collectors are implemented, Core must define or validate the trusted
purpose-specific expectation profile that marks each closed collector class as
required, optional, or inapplicable. A host must not be allowed to choose a
weaker profile. The current model proves complete accounting and purpose
binding; it does not yet select that profile because no collector boundary
exists.

`ReferenceOnly` remains planned disposition vocabulary and is not emitted by
the first selection algorithm. Consumer-visible inventory, if introduced,
requires a separate capability-filtered and redacted projection review.

Derived products are serializable orientation data only. Any future
deserialization or persistence path must recompute or verify commitments and
must never treat a serialized decision, bundle, or understanding projection as
execution authority.

## Validation

The accepted tree must pass:

- `cargo test -p workflow-core --lib continuation_context`;
- `cargo clippy -p workflow-core --all-targets -- -D warnings -A dead-code`;
- `cargo test --workspace`;
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`;
- `cargo fmt --all -- --check`;
- `npm run check:docs`; and
- `git diff --check`.

Exact repository CI remains required on the final pull-request head.

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793695730838798000-2`
- approval ID:
  `approval/run-1793695730838798000-2/review-scope-approved`
- presentation ID: `presentation/3517ed616ed9d26b`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations
- approval-presentation enforcement: proof-enforced

## Out-Of-Kernel Work

The delegated maintainer inspected source and design documents, identified and
fixed model blockers, added regressions, ran validation, and wrote this review
outside the kernel. The kernel governed scope and approval but did not edit
files, execute checks, create commits, or mutate GitHub.
