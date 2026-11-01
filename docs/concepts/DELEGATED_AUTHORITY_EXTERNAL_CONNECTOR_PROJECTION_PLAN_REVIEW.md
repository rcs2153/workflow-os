# Delegated Authority External Connector Projection Plan Review

## Decision

Accepted after security-critical plan corrections.

The corrected plan defines one extension of the existing Workflow OS authority
and continuity system. It does not create connector-owned authority, treat a
serialized projection as a bearer token, weaken platform permission, or enable
provider mutation.

## Reviewed Boundary

The review verified that the plan now:

- separates immutable authorization projection from mutable transition state;
- binds approval-presentation proof to the exact transition-scope commitment;
- requires Core-owned current-authority reassessment immediately before every
  connector mutation;
- permits connector entry only through a non-serializable same-call use
  capability;
- derives the exact PR identity after draft creation rather than accepting a
  caller-substituted PR number;
- distinguishes exact Git ref fast-forward from pull-request transitions;
- binds payload classes through commitments without storing raw payloads;
- requires exact head-bound, source-bound, freshness-bound evidence;
- composes authorized-execution continuity and provider reconciliation rather
  than inventing a second consumption ledger;
- preserves refusal as durable wait or terminal denial posture; and
- keeps platform credentials and provider permission separate from kernel
  authority.

## Blocking Findings Corrected

### Projection and lifecycle state were conflated

The initial plan placed consumed transition records and remaining transitions
inside the projection. That would make the purported immutable authority
envelope mutable and could allow caller-authored remaining scope.

The corrected plan keeps one immutable projection and derives remaining
authority from it plus Core-owned durable transition lifecycle state.

### A serialized projection could be mistaken for authority

The initial connector-verification language did not state strongly enough that
a deserialized projection is only a claim. The corrected boundary requires
Core to reload current sources, reserve the exact operation, and pass a
non-reusable same-call use capability into the connector.

### PR identity did not exist before draft creation

The initial plan allowed an optional PR identity but did not define how later
transitions become bound to the provider-created PR. The corrected plan binds
the create request to exact repository/base/head and operation commitments,
then records the returned PR identity as a Core-derived successor binding.

### Provider mutation ambiguity was underspecified

The initial replay rule did not fully address remote-success/local-failure or
transport ambiguity. The corrected plan composes existing reconciliation rules,
blocks blind redispatch, and requires exact replay or explicit lookup/recovery.

### Evidence and payload commitments were too broad

`all named checks` and `payload class` were not sufficient security boundaries.
The corrected plan requires exact check-set, source, head, observation time,
freshness, and payload-content commitments while remaining payload-free.

### Automatic continuation was ahead of the scheduler

The initial plan said the scheduler resumes automatically without an accepted
scheduler integration. The corrected plan defines durable wait semantics now
and keeps automatic scheduling as a later, separately proven consumer.

## Security Properties Preserved

- Standing delegation alone cannot authorize a connector mutation.
- Approval alone cannot create platform permission.
- A projection cannot be used as a credential or bearer token.
- Every transition is exact-resource, exact-action, and exact-head bound.
- Revocation, expiry, changed head, stale evidence, scope expansion, and
  ambiguous prior use fail closed.
- Branch deletion remains separately authorized and cannot target protected or
  default branches.
- Provider success without local closure remains reconciliation-required.

## Accepted Next Phase

Implement only the model-level immutable authorization projection, closed PR
transition vocabulary, successor-resource binding vocabulary, deterministic
commitment, and validation rules.

Do not implement connector execution, provider calls, credentials, live GitHub
behavior, durable transition storage, scheduler behavior, CLI mutation paths,
schemas, or broader provider writes in that phase.

## Validation

- `npm run check:docs`: required before phase close.
- `git diff --check`: required before phase close.

## Governance Record

- workflow: `dg/review`;
- run: `run-1793561258366071000-2`;
- approval: `approval/run-1793561258366071000-2/review-scope-approved`;
- presentation proof: `presentation/8dea0b4bc8abdc78`;
- approval outcome: granted under delegated maintainer authority;
- phase scope: plan review and correction only; and
- out-of-kernel work: document inspection, edits, validation, git operations,
  and later PR actions remain executor work.
