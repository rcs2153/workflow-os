# Continuation Context Coverage Profile Plan Review

## Decision

Accepted for one Core-private, model-only implementation after clarification.

The proposed matrix is appropriately conservative. Core, rather than a host or
execution provider, should decide whether every closed continuation-context
source class is required, optional, or inapplicable for an exact purpose and
valid scope posture. Required explicit empty posture is correctly distinct from
collector failure, access denial, unsupported collection, or omission.

This acceptance does not claim that authoritative post-compaction
understanding is complete.

## Findings

### The matrix closes a real caller-downgrade path

The first model requires complete source-class accounting, but each input row
still carries its own expectation. A caller can therefore label a class
optional or inapplicable unless Core compares the row with a trusted profile.
Deriving that profile from purpose and validated pre-run or run-bound posture
is the correct correction.

The broad run-bound baseline is intentional. Requiring a collector means the
kernel must account for the class; it does not require an object to exist.
Explicit empty posture is therefore a valid complete collector result, while
failure, denial, unsupported collection, and omission remain distinguishable.

### Profile posture must follow scope validation

Run-bound posture cannot be a caller-selected flag. Core must first validate
the exact scope shape and then derive profile identity from the presence of a
valid run binding plus the continuation purpose. The plan now states this
ordering explicitly.

### Provider reconciliation needs semantic sufficiency after coverage

Provider observation is correctly required for run-bound provider
reconciliation. However, a required collector may lawfully return explicit
empty posture. That proves the collector ran; it does not prove the kernel has
the provider fact needed for reconciliation.

The existing required-claim decision can represent this distinction, but its
claim set is still supplied by the caller. A later Core-owned semantic-
obligation profile must require the relevant provider claim and keep the
decision blocked when no current provider value exists.

### Two caller-controlled downgrade paths remain

The current decision input still carries:

- `required_claims`; and
- each source definition's `required_for_purpose` value.

Either can weaken the resulting readiness decision even after collector
expectations become Core-owned. This phase must not absorb those semantics
casually because they require a purpose-specific claim and source-necessity
design. It must instead report the remaining boundary precisely and keep all
operational consumers blocked until the follow-up is accepted.

## Required Implementation Conditions

The implementation must:

1. remain crate-private and process-free;
2. validate scope shape before deriving a profile;
3. derive one closed profile identity from purpose and valid run-binding
   posture;
4. compare every caller-carried expectation with the Core-derived matrix before
   outcome evaluation;
5. bind purpose, run-binding posture, profile identity, expectations, and
   outcomes into the coverage commitment;
6. use stable payload-free errors for expectation substitution;
7. preserve required-empty versus unavailable semantics; and
8. state in its report that semantic-obligation ownership remains a blocking
   follow-up.

## Security Assessment

Accepted within the stated boundary. The phase removes host control over
collector expectation classification without authorizing execution, exposing
context, reading sources, or creating a configurable policy surface.

The implementation is not sufficient evidence for compaction-safe delegated
continuation. That claim remains blocked until Core also owns semantic claim
requirements, source necessity, collectors, trusted composition, bounded
projection, delivery proof, and one induced context-loss vertical slice.

## Validation

- `npm run check:docs`
- `git diff --check`

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793697243809161000-2`
- approval ID:
  `approval/run-1793697243809161000-2/review-scope-approved`
- presentation ID: `presentation/0d7ad4b6b47b1c83`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Out-Of-Kernel Work

The delegated maintainer inspected the private Core model, challenged the
trusted matrix, refined the plan and roadmap, and wrote this review outside the
kernel. The kernel governed scope and approval but did not edit files, run
documentation checks, create commits, or mutate GitHub.
