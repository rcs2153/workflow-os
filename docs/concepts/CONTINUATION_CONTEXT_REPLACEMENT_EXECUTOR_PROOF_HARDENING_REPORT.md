# Continuation Context Replacement-Executor Proof Hardening Report

## Status

Implemented and accepted after focused re-review. This correction remains
private, local, in-process, and non-authorizing.

## Governed Scope

- workflow: `dg/blocker`
- run: `run-1793719470436356000-2`
- approval: `approval/run-1793719470436356000-2/fix-approved`
- presentation: `presentation/1358b7d4cc0b3679`
- approval outcome: granted by the delegated maintainer for the three bounded
  replacement-proof blockers

## Corrections

The private authoritative WorkReport source and accepted-intent record wrapper
can now carry payload-free stable work identities with one of two typed
dispositions: `completed` or `lawful_next`. Identity is no longer projected as
an independent top-level collector assertion. Only the WorkReport source may
carry completed identity, and only one current resume record may carry
lawful-next identity. Completed identity is bound to the exact completed claim;
lawful-next identity is bound to the exact accepted-intent record.

Core derives work-identity candidates from those authoritative private source
records and projects them only when the exact source is selected by the normal
context decision. Access denial, stale or superseded freshness, invalid or
unverified integrity, sensitivity exclusion, and materialization-budget
omission cannot bypass that decision. Admitted identities are canonicalized,
bound into the aggregate proof, carried through exact delivery, and compared
between the original and replacement orientations.

The private aggregate now preserves the exact current control disposition as
`none`, `pause`, `stop`, or `resume`. Pause and stop never reach fresh authority
resolution. A pause or stop combined with a lawful-next identity fails closed.
A ready orientation without a current resume also fails closed.

Resume requires exactly one stable lawful-next identity. If the same identity
is already present with completed disposition, the proof rejects the action
even when its human-readable description differs. This replaces the previous
exact-text comparison in the conformance path.

Fresh authority resolution now receives one payload-free private request bound
to:

- the exact scope commitment;
- accepted aggregate proof commitment;
- exact continuation decision commitment;
- replacement recipient actor;
- stable lawful-next operation identity; and
- an explicit non-authorizing posture.

The proof validates aggregate equality, delivered understanding, typed gaps,
control disposition, stable work identities, recipient separation, and proof
commitment construction before invoking the injected resolver. The resolver
returns no authority payload, and the proof remains non-authorizing.

## Adversarial Coverage

Focused tests now prove:

- pause and stop perform no authority resolution;
- a superseded resume is not revived by a current pause or stop;
- a rephrased next action with a completed stable identity is rejected;
- operation-identity substitution changes the aggregate proof and fails before
  authority resolution;
- the exact expected operation identity reaches the resolver;
- stable identity is carried by the authoritative private source or accepted-
  intent wrapper rather than by a parallel top-level assertion;
- stale, unauthorized, invalid-integrity, and over-budget WorkReport sources
  cannot project completed-work identity;
- an unrelated claim binding cannot lend authority to a completed-work
  identity;
- original and replacement recipient identities must differ;
- blocked context still discloses matching typed gaps and performs no authority
  resolution; and
- another aggregate still fails before delivery or authority resolution.

## Explicit Boundaries

The stable work-identity records are private conformance inputs carried by the
authoritative collected source shape. Existing production-shaped collectors do
not invent or infer these identities. No runtime consumer is enabled until a
real authoritative producer can persist and supply a durable identity shared
across the completed-work and lawful-next domains.

This phase does not add public schema or API fields, CLI behavior, persistence,
provider operations, a host compaction signal, model or process replacement,
automatic WorkReport production, a reusable authority artifact, hosted
behavior, nested harnesses, an agent memory system, or Salsa.

## Validation

- focused continuation collection tests: passed, 53 tests;
- strict `workflow-core` Clippy with all targets and features: passed;
- formatting check: passed;
- full workspace tests with all features: passed;
- warnings-denied workspace rustdoc: passed;
- documentation checks: passed; and
- diff hygiene check: passed.

## Review Outcome

Focused re-review accepted the closed producer/disposition mapping, exact
source-selection gate, stable identity binding, pause/stop fail-closed
behavior, exact authority-request commitment, resolver ordering,
non-authorizing posture, and truthful limitation to private conformance. See
the [hardening focused
review](CONTINUATION_CONTEXT_REPLACEMENT_EXECUTOR_PROOF_HARDENING_REVIEW.md).
