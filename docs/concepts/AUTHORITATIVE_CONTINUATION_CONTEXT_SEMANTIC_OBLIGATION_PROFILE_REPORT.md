# Authoritative Continuation Context Semantic-Obligation Profile Report

## Status

Implemented as a Core-private, model-only continuation-context increment.

## Delivered

Core now derives one closed semantic-obligation profile from the exact
continuation purpose and validated pre-run or run-bound scope. Callers can no
longer supply required claim kinds or label an admitted source as required or
optional.

Each of the twelve profiles commits to:

- all 23 closed understanding claim kinds;
- required-value, required-accounting, optional, or inapplicable posture;
- singleton or set cardinality;
- the `HighestAuthority` resolution rule;
- the complete permitted authority-domain set for each claim;
- all 18 source classes and their Core-owned necessity; and
- the exact purpose and run-binding posture.

The semantic-profile commitment is transitively bound into the private
inventory, bounded bundle, understanding projection, and final decision.

## Exact Claim Matrices

Legend: `V` is required value and `A` is required accounting. The first
increment does not assign `O` or `I` to a claim, but those closed postures remain
reserved for a later reviewed profile change.

### Pre-run

| Claim | Planning | Implementation | Approval review | Provider reconciliation | Child handoff | Material continuation |
| --- | --- | --- | --- | --- | --- | --- |
| Objective | V | V | V | V | V | V |
| Priority | A | A | A | A | A | A |
| Accepted amendment | A | A | A | A | A | A |
| Scope | V | V | V | V | V | V |
| Strict non-goal | A | A | A | A | A | A |
| Decision | A | A | A | A | A | A |
| Rationale reference | A | A | A | A | A | A |
| Completed obligation | A | A | A | A | A | A |
| In-progress obligation | A | A | A | A | A | A |
| Deferred obligation | A | A | A | A | A | A |
| Remaining obligation | A | V | A | A | V | V |
| Blocker | A | A | A | A | A | A |
| Registered wait | A | A | A | A | A | A |
| Open question | A | A | A | A | A | A |
| Assumption | A | A | A | A | A | A |
| Uncertainty | A | A | A | A | A | A |
| Evidence requirement | A | A | V | V | A | A |
| Check requirement | A | V | V | A | A | V |
| SideEffect posture | A | V | V | V | A | V |
| Validation commitment | A | V | V | A | V | V |
| Reporting commitment | A | V | V | A | V | V |
| Promised follow-up | A | A | A | A | A | A |
| Lawful next option | V | V | V | V | V | V |

### Run-bound

| Claim | Planning | Implementation | Approval review | Provider reconciliation | Child handoff | Material continuation |
| --- | --- | --- | --- | --- | --- | --- |
| Objective | V | V | V | V | V | V |
| Priority | A | A | A | A | A | A |
| Accepted amendment | A | A | A | A | A | A |
| Scope | V | V | V | V | V | V |
| Strict non-goal | A | A | A | A | A | A |
| Decision | A | A | A | A | A | A |
| Rationale reference | A | A | A | A | A | A |
| Completed obligation | A | A | A | A | A | A |
| In-progress obligation | A | A | A | A | A | A |
| Deferred obligation | A | A | A | A | A | A |
| Remaining obligation | V | V | A | V | V | V |
| Blocker | A | A | A | A | A | A |
| Registered wait | A | A | A | A | A | A |
| Open question | A | A | A | A | A | A |
| Assumption | A | A | A | A | A | A |
| Uncertainty | A | A | A | A | A | A |
| Evidence requirement | A | V | V | V | V | V |
| Check requirement | A | V | V | A | V | V |
| SideEffect posture | A | V | V | V | V | V |
| Validation commitment | A | V | V | A | V | V |
| Reporting commitment | A | V | V | A | V | V |
| Promised follow-up | A | A | A | A | A | A |
| Lawful next option | V | V | V | V | V | V |

## Source Necessity

Pre-run profiles mark validated declarations, policy, required context,
accepted intent, Git observation, and operating documents `Mandatory`.
Evidence, checks, WorkReports, typed handoffs, and provider observations are
`Supplemental`; agent advisory context is `AdvisoryOnly`; immutable run bundle,
durable run state, event history, approval, capability, and SideEffect sources
are `Inapplicable`.

Run-bound profiles mark every class `Mandatory` except agent advisory context,
which remains `AdvisoryOnly`, and provider observation, which is
`Supplemental` outside provider reconciliation. Run-bound provider
reconciliation makes provider observation `Mandatory`.

An inapplicable class is rejected. Advisory-only context can preserve nuance,
but it cannot satisfy a required value or required accounting obligation.

## Accounting And Cardinality

Required accounting is represented by a typed claim/source observation with an
`Empty` outcome. It counts only when every Core-selected best accounting source
has an observed or explicit-empty collector outcome. Missing, failed, denied,
unsupported, or intentionally omitted collection cannot manufacture negative
knowledge.

Objective, priority, and scope are singleton claims. Different values at the
same winning authority rank produce a typed conflict. Every other claim is
set-valued: distinct values at the winning authority rank are retained in
deterministic order with per-value provenance. No natural-language conflict is
inferred.

## Security Properties

- Purpose and validated scope select the profile; the caller cannot select it.
- Required claims and source necessity are no longer caller-carried fields.
- Authority domains and the resolution rule are part of the canonical profile.
- Missing required values, incomplete accounting, conflicts, withheld values,
  and advisory-only satisfaction attempts block.
- Required accounting can be explicitly empty without invented prose.
- Source selection gives mandatory context budget priority.
- Semantic profile changes invalidate inventory, bundle, understanding, and
  decision commitments.
- Debug output remains redacted and errors remain payload-free.
- Inventory, bundle, projection, and decision remain non-authorizing.

## Focused Coverage

Forty continuation-context tests pass. Added regressions cover:

- all twelve complete, distinct semantic profiles;
- all 23 claim obligations and all 18 source obligations;
- explicit authority-domain and resolution binding;
- reviewed purpose and pre-run/run-bound matrix differences;
- caller inability to clear a required value;
- typed empty accounting and missing accounting;
- authoritative empty accounting precedence over advisory-only prose;
- rejection of non-authoritative accounting;
- advisory-only insufficiency;
- deterministic set-valued retention with per-value provenance;
- singleton conflict preservation;
- inapplicable pre-run source rejection; and
- profile-sensitive canonical commitments.

## Boundary Preserved

This phase did not add collectors, persistence, trusted composition,
accepted-intent capture, redacted public projection, delivery acknowledgment,
compaction lifecycle handling, runtime consumers, CLI or schema exposure,
provider actions, child harnesses, or Salsa. It does not prove replacement-
executor equivalence and does not authorize execution.

## Governed Implementation

- workflow: `dg/implement`
- run ID: `run-1793699751868140000-2`
- approval ID:
  `approval/run-1793699751868140000-2/implementation-approved`
- presentation ID: `presentation/adaa6e72ef82a3d0`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused continuation-context tests: 40 passed
- strict `workflow-core` clippy: passed with warnings denied and the existing
  narrow `dead_code` allowance for future-facing private surfaces
- full workspace tests: passed
- rustdoc with warnings denied: passed
- formatting: passed

- documentation checks: passed
- diff checks: passed

## Out-Of-Kernel Work

The delegated maintainer edited the private Core model, added focused tests,
ran validation, updated roadmap documentation, and authored this report outside
the kernel. The kernel governed scope and approval but did not edit files,
execute checks, create commits, or mutate GitHub.
