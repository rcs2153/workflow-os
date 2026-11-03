# External Action Fixture Pull-Request Lifecycle Proof Plan Review

Status: accepted after blocker correction.

## Scope

This owner-conducted focused maintainer/security review evaluates the
[External Action Fixture Pull-Request Lifecycle Proof
Plan](../implementation-plans/external-action-fixture-pr-lifecycle-proof-plan.md)
against the accepted external-action authorization projection, durable
transition lifecycle, same-call current-authority connector admission,
authoritative continuation, approval-presentation proof, and proportional-
governance boundaries.

This is repository assurance, not independent external security review.

## Findings

### Blocker: provider observation did not close the mutation race

The draft required revision-fenced provider observation but did not require the
connector operation to atomically compare the exact observed provider revision.
State could therefore change after evidence composition and before fixture
mutation.

Correction: every provider operation now carries the exact observed revision,
and the fixture provider atomically compares it while applying the operation.
Drift rejects without mutation and is distinct from ambiguous provider outcome.

### Blocker: successor binding ownership was misplaced

The draft described compare-and-replace successor persistence through a
fixture store. The provider fixture may observe or return a PR identity, but the
binding from that identity to the immutable projection is a Core governance
record.

Correction: successor persistence is explicitly Core-owned. The provider
fixture cannot issue, replace, or authorize the binding.

### Blocker: create success followed by local persistence failure was undefined

A provider may create the PR successfully while Core fails before durably
persisting the successor binding. Treating that as ordinary failure could
duplicate draft creation.

Correction: that split outcome is ambiguous. Reconciliation looks up the
stable provider operation receipt, reconstructs the exact PR observation,
derives the same binding, and completes idempotent Core persistence. Conflict
fails closed.

### Important: closed execution windows are not resumable

The draft allowed a closed window to become a wait. A wait cannot extend expired
authority.

Correction: closed or expired windows reject the old authorization and require
a separately governed successor authorization.

### Important: refusal and ambiguity require dispatch evidence

A connector refusal is safely resumable only if the connector proves provider
dispatch did not begin. Otherwise the outcome may have happened remotely.

Correction: proof-backed pre-dispatch refusal becomes a wait; absence of that
proof becomes ambiguity and blocks redispatch until reconciliation.

### Important: failed checks are evidence posture, not automatically terminal

The draft described failed checks as transition denial, which could prevent a
later fresh green check result from satisfying the same still-current
projection.

Correction: failed checks create a typed evidence block/wait for that revision
unless governing policy explicitly declares the failure terminal.

## Accepted Architecture

The corrected plan preserves these critical separations:

- immutable authorization projection versus mutable lifecycle state;
- non-authorizing context/evidence versus same-call execution authority;
- provider observation versus Core-owned successor binding;
- stable provider operation identity versus per-attempt local identity;
- pre-dispatch refusal versus may-have-started ambiguity;
- evidence wait versus expired authorization;
- one complete approved transition set versus fresh admission at every
  transition; and
- local conformance fixture versus production GitHub truth.

The plan's source-coverage table is acceptable only with its correction that
Phase 5A name each exact source contract and return a complete coverage
commitment. There is no generic caller-authored current-fact fallback.

## Threat Coverage

The corrected plan requires focused proof for changed head/base, stale or
missing presentation proof, omitted transition scope, expiry, revocation,
policy drift, missing or substituted successor binding, pending/failed checks,
pre-dispatch refusal, known failure, ambiguous outcomes, provider revision
drift, local persistence ambiguity, exact replay, concurrent admission, corrupt
state, and serialized-claim replay.

The full lifecycle proof must verify connector entry counts and durable state,
not merely returned status values.

## Remaining Limits

The accepted plan does not authorize:

- live GitHub behavior or network access;
- access-material loading or storage;
- production provider writes;
- ref updates or branch deletion;
- public connector APIs, schemas, SDKs, or CLI mutation commands;
- scheduler, polling, or automatic continuation;
- broader provider mutation families;
- Salsa implementation; or
- a claim of independent security assurance.

## Verdict

Accepted after correction. Phase 5A source and fixture contract implementation
is the next authorized increment. Later subphases remain contingent on focused
validation and review of the preceding slice.
