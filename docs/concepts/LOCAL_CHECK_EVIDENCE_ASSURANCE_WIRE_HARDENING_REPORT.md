# Local Check Evidence Assurance Wire Hardening Report

## Status

Implemented; focused maintainer/security review is required before acceptance.

## Summary

The bounded follow-up from independent assurance is implemented. Serialized
local-check evidence-assurance projections and minimums now reject unknown
fields, and `LocalCheckEvidenceProducerPosture` no longer derives ordering
traits that future code could accidentally reuse as policy.

The existing minimum gate still evaluates producer posture through its
explicit closed comparison table. This phase does not change any reachable
producer posture, minimum semantics, runtime route, persistence behavior, or
authorization decision.

## Closed Findings

### Unknown fields fail closed

`LocalCheckEvidenceAssuranceProjectionWire` and
`LocalCheckEvidenceAssuranceMinimum` now use serde's closed-field posture.
Direct regressions add an unrecognized assurance claim to each valid wire
record and require deserialization to fail.

This closes the independent review's low-severity finding that ignored data
could travel with otherwise valid records. Recognized fields and the existing
projection commitment remain authoritative inside the disclosed local trust
boundary.

### Producer declaration order is not policy vocabulary

`LocalCheckEvidenceProducerPosture` no longer derives `Ord` or `PartialOrd`.
The active minimum gate already used an explicit closed producer-assurance
table; this change removes the informational future-policy risk that unrelated
code could compare producer postures by declaration order.

## Preserved Boundary

This hardening does not establish:

- producer registration or producer authentication;
- validator semantic trustworthiness;
- trusted-host independence;
- cryptographic signatures or external witnessing;
- resistance to a privileged storage owner that can replace all records and
  recompute unkeyed commitments;
- a new assurance upgrade path;
- new policy, provider, or execution behavior; or
- broader release or security assurance.

Serialized assurance vocabulary remains non-authorizing. A stronger producer
posture is still unreachable through the current closed runtime route.

## Validation

Passed:

- the focused serde tampering and unknown-vocabulary regression;
- `cargo check -p workflow-core --all-features`;
- the complete `cargo test -p workflow-core --all-features` suite, including
  792 library tests, 351 local-executor tests with one opt-in live test ignored,
  all remaining non-live integration targets, and 6 documentation tests;
- warnings-denied Clippy for all Workflow Core targets and features;
- repository-wide Rust formatting;
- documentation and release-state checks; and
- diff validation.

No live provider, credential-backed, hosted, or external-witness test was run
because this phase changes only closed local wire validation and type traits.

## Governed Implementation

- Workflow: `dg/implement`
- Run: `run-1793938203052922000-2`
- Approval: `approval/run-1793938203052922000-2/implementation-approved`
- Presentation: `presentation/bec77eeabe1e61bf`
- Presentation hash:
  `bec77eeabe1e61bfae6db9fb27d49dab0957cb901ce1ffa693cf13d5a0bee0ae`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- Approval-presentation enforcement: proof enforced

Repository edits, tests, and documentation were performed outside the kernel
under the approved scope. The kernel coordinated the phase and approval; it did
not execute commands or manufacture validation evidence.

## Next Phase

Run focused maintainer/security review of the exact hardening diff and its
validation evidence. Do not make a stronger producer posture reachable during
that review.
