# Authoritative Continuation Context Semantic-Obligation Profile Review

## Decision

Accepted after one fix-forward correction.

The implementation closes the reviewed caller-controlled semantic strictness
paths without adding collection, persistence, runtime rehydration, public
projection, or execution authority. It remains a private deterministic model.

## Review Finding And Correction

The initial implementation correctly rejected advisory-only values as
satisfaction for required claims. It did not distinguish the case where a
Core-selected authoritative source had already supplied complete typed-empty
accounting for a required-accounting claim.

In that case, the mere presence of lower-authority advisory prose caused a
`ClaimAdvisoryOnly` gap even though the authoritative semantic obligation was
already satisfied as empty. This was safe but incorrect: agent memory was able
to turn authoritative negative knowledge into a false blocker.

The correction gives complete authoritative empty accounting precedence over
advisory-only prose for `RequiredAccounting` claims. The projection emits one
`AccountedEmpty` claim with no value and no advisory source citation. The
advisory source remains visible in the private inventory and bounded bundle,
but it does not override or satisfy the authoritative semantic result.

Required-value claims remain blocked by advisory-only input. Required-
accounting claims without complete authoritative accounting also remain
blocked. A focused regression proves all three boundaries.

## Accepted Properties

- Exact purpose and validated scope derive one of twelve closed profiles.
- Every profile accounts for all 23 claim kinds and all 18 source classes.
- The profile binds requirement, cardinality, resolution rule, permitted
  authority domains, and source necessity.
- Callers cannot clear required claims or relabel source necessity.
- Required value and required accounting remain distinct.
- Typed empty is explicit, source-bound, coverage-bound, and cannot be invented
  from prose.
- Objective, priority, and scope remain singleton claims with equal-authority
  conflict detection.
- Set-valued claims preserve every distinct winning value in deterministic
  order with per-value provenance.
- Advisory-only context cannot satisfy a required claim or override
  authoritative empty accounting.
- Inapplicable source classes fail closed before projection.
- Mandatory sources receive selection priority and their selection failures
  remain blocking.
- The semantic profile is transitively committed into inventory, bundle,
  understanding, and decision products.
- Debug and validation surfaces remain bounded and payload-safe.
- Every result remains non-authorizing.

## Remaining P0 Boundary

This acceptance establishes only the pure semantic contract. It does not make
the continuation decision operationally authoritative.

Claim accounting and source coverage are still supplied observations. The
trusted collectors and composition path that produce them are not implemented.
Accepted-intent persistence, complete source collection, redacted public
projection, exact delivery acknowledgment, compaction lifecycle handling,
unconditional material-boundary rehydration, runtime consumption, and the
replacement-executor proof also remain unimplemented.

Salsa remains a later bounded evaluation only. It cannot become durable state,
authority, or an execution boundary.

## Validation

- focused continuation-context tests: 40 passed
- strict `workflow-core` clippy: passed with warnings denied and the documented
  narrow pre-existing `dead_code` allowance
- full workspace tests: passed after the fix-forward correction
- rustdoc with warnings denied: passed
- formatting: passed
- documentation checks: passed
- diff checks: passed

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793700757192624000-2`
- approval ID:
  `approval/run-1793700757192624000-2/review-scope-approved`
- presentation ID: `presentation/cc6b03e1d0da2837`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Out-Of-Kernel Work

The delegated maintainer inspected the implementation, profile matrices,
authority binding, accounting semantics, commitments, tests, roadmap, and
report; identified and corrected the advisory-versus-authoritative-empty edge;
and reran focused and broad validation outside the kernel. The kernel governed
scope and approval but did not edit files, run checks, create commits, or
mutate GitHub.
