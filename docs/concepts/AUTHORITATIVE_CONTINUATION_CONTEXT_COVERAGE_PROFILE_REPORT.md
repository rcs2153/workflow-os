# Authoritative Continuation Context Coverage Profile Report

## Status

Implemented as a Core-private, model-only continuation-context increment.

## Delivered

The private continuation-context decision now derives a closed coverage
profile identity from:

- the exact `ContinuationContextPurpose`; and
- whether the already-validated scope is pre-run or bound to an exact run.

Core owns the expected collector posture for all 18 closed source classes.
Caller-carried coverage rows remain an input for this increment, but every
expectation must match the derived profile before collector outcomes are
evaluated. A required-to-optional, inapplicable-to-required, or other
expectation substitution fails with the stable payload-free code
`continuation_context.coverage.expectation_mismatch`.

Focused review also closed outcome relabeling. Required collection cannot be
reported as skipped-optional or inapplicable, optional collection cannot be
reported as inapplicable, and an inapplicable class requires the exact
inapplicable outcome. Contradictions fail with
`continuation_context.coverage.outcome_expectation_mismatch`.

The coverage result and commitment now bind:

- purpose;
- closed profile identity;
- run-binding posture;
- all source-class expectations; and
- all collector outcomes.

Input ordering remains irrelevant because coverage rows are canonicalized
before validation and commitment derivation.

## Trusted Profiles

Pre-run profiles require accounting for validated declarations, policy,
required context, accepted intent, Git observation, and operating documents.
Evidence, checks, WorkReports, typed handoffs, provider observations, and agent
advisory context are optional. Dynamic run facts are inapplicable.

Run-bound profiles require accounting for every source class except agent
advisory context and, for non-reconciliation purposes, provider observation.
Run-bound provider reconciliation requires provider observation. Required
explicit empty posture remains complete because it proves the collector
accounted for the class; failure, access denial, unsupported collection, and
intentional omission remain incomplete.

## Security Properties

- Scope shape is validated before profile derivation.
- A caller cannot select run-bound posture independently of an exact valid run
  scope.
- A host cannot downgrade a required collector to optional or inapplicable.
- A host cannot activate an inapplicable pre-run class through expectation
  substitution.
- Pre-run and run-bound profiles have distinct identities and commitments.
- Purpose changes produce distinct profile identities and commitments even
  where expectation matrices happen to match.
- Provider reconciliation cannot relabel provider observation as optional.
- Errors contain no source identities, revisions, semantic values, paths, or
  provider payloads.
- The model remains non-authorizing and process-free.

## Focused Coverage

Thirty-one continuation-context tests pass, including regressions for:

- all 12 purpose/scope profile identities;
- the reviewed pre-run and run-bound matrices;
- order-independent commitments;
- required-to-optional substitution;
- inapplicable-to-required substitution;
- expectation/outcome relabeling;
- pre-run versus run-bound commitment separation;
- provider-observation requirements;
- required empty versus required failure;
- optional degraded collection;
- complete closed-class accounting; and
- source/outcome contradictions.

## Explicit Remaining Blocker

This increment closes collector-expectation substitution only. It does not
complete authoritative post-compaction understanding.

The current decision input still accepts caller-carried `required_claims` and
each source definition's `required_for_purpose` posture. Core must own those
semantic obligations in a separately reviewed profile before an operational
consumer may rely on a ready decision as authoritative understanding. A
required collector returning explicit empty posture proves collection was
attempted; it does not prove a purpose-specific semantic value exists.

Collectors, persistence, trusted composition, redacted projection, delivery
proof, compaction adapters, runtime consumers, and induced context-loss proof
also remain unimplemented.

## Boundary Preserved

This phase did not add:

- source collectors or file/provider reads;
- storage or runtime wiring;
- CLI, schema, SDK, report, hook, or child-harness exposure;
- accepted-intent recording;
- compaction detection;
- execution grants or provider operations;
- Salsa or another derivation framework;
- dependencies or Rust toolchain changes; or
- public Core exports.

## Governed Implementation

- workflow: `dg/implement`
- run ID: `run-1793697341944897000-2`
- approval ID:
  `approval/run-1793697341944897000-2/implementation-approved`
- presentation ID: `presentation/54ebdad523e673fc`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused continuation-context tests: 31 passed
- strict `workflow-core` clippy: passed with warnings denied and a narrow
  `dead_code` allowance for the repository's pre-existing future-facing private
  surface
- full workspace tests: passed
- rustdoc with warnings denied: passed
- formatting: passed
- documentation checks: passed
- diff checks: passed

## Out-Of-Kernel Work

The delegated maintainer edited the private Core model, added focused tests,
updated roadmap documentation, ran formatting and focused tests, and authored
this report outside the kernel. The kernel governed scope and approval but did
not edit files, execute validation, create commits, or mutate GitHub.
