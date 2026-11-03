# Continuation Context Coverage Profile Plan

## Status

Planned P0 follow-up to the accepted Core-private authoritative continuation
context model.

## Problem

The first model requires one coverage entry for every closed source class and
binds the resulting matrix to the continuation purpose. That prevents silent
collector omission, but the input still carries each class's `Required`,
`Optional`, or `Inapplicable` expectation.

A future host must not be able to weaken completeness by labeling a collector
optional or inapplicable. Expectation selection is governance policy, not an
execution-provider preference.

## Required Invariant

For one exact continuation purpose and scope posture, Core owns the complete
collector-expectation profile. A caller supplies observed outcomes only. Core
rejects missing classes, duplicate classes, substituted expectations, and
outcomes that contradict supplied observations.

`Required` means the collector must account for the class. It does not mean a
record must exist. `Observed` and explicit `Empty` are both complete outcomes
for a required collector. Access denial, failure, unsupported collection, and
intentional omission remain distinct and fail completeness when the trusted
profile requires that collector.

## Boundary

This phase is Core-private and model-only. It does not:

- implement or invoke a collector;
- read files, Git, state, events, providers, or conversation;
- persist accepted intent, inventory, bundles, or projections;
- add CLI, schema, SDK, report, hook, or child-harness exposure;
- authorize execution or satisfy approval, policy, capability, evidence, or
  check requirements;
- add a configurable YAML expectation matrix;
- add Salsa or another derivation framework; or
- broaden provider operations.

This increment closes caller substitution of collector expectations only. It
does not make the complete continuation decision authoritative by itself. The
current model still accepts caller-carried `required_claims` and per-source
`required_for_purpose` posture. A later Core-owned semantic-obligation profile
must remove those downgrade paths before any operational consumer can claim
authoritative post-compaction understanding.

## Model

Add a private closed profile identity derived only from:

- `ContinuationContextPurpose`; and
- whether the exact scope is pre-run or run-bound.

The profile exposes one deterministic expectation for every
`ContinuationContextSourceClass`. The decision input may continue to carry the
complete expectation/outcome rows for this increment, but validation must
compare every supplied expectation against the Core-derived profile before
evaluating outcomes.

The profile identity and complete expected matrix must be included in the
coverage commitment. A caller cannot reuse a pre-run profile for an active run
or reuse an implementation profile for approval review or provider
reconciliation.

Scope shape must be validated before profile derivation. Run-bound posture is
derived only from a valid exact run scope; a caller cannot select it directly.

## Trusted Expectation Matrix

### Pre-run baseline

For pre-run planning or preparation:

Required collector classes:

- validated declaration;
- policy;
- required context;
- accepted intent;
- Git observation; and
- operating document.

Optional collector classes:

- evidence;
- check;
- WorkReport;
- typed handoff;
- provider observation; and
- agent advisory.

Inapplicable collector classes:

- immutable run bundle;
- durable run state;
- event history;
- approval;
- capability; and
- SideEffect.

Pre-run purposes other than `Planning` remain representable for model
conformance, but a later operational caller must separately justify whether
they are lawful. This profile does not authorize an operational entry point.

### Run-bound baseline

Every run-bound purpose requires collectors for:

- validated declaration;
- immutable run bundle;
- durable run state;
- event history;
- policy;
- approval;
- capability;
- required context;
- evidence;
- check;
- SideEffect;
- WorkReport;
- accepted intent;
- typed handoff;
- Git observation; and
- operating document.

Agent advisory is always optional and can never make a required claim complete
by itself.

Provider observation is:

- required for `ProviderReconciliation`;
- optional for `Implementation`, `ApprovalReview`, `ChildHandoff`, and
  `MaterialContinuation`; and
- optional for run-bound `Planning`.

The broad run-bound baseline is deliberate. Required collector execution may
return explicit empty posture. The kernel must know that it looked for current
approval, capability, evidence, check, SideEffect, report, intent, handoff, Git,
and operating-document state before claiming comprehensive understanding.

Coverage completeness is not semantic sufficiency. In particular, an explicit
empty provider observation proves that the required collector accounted for
the class; it does not prove that provider reconciliation has enough current
provider state to proceed. A later Core-owned required-claim profile must make
the relevant claim unknown and keep the decision blocked when the purpose
requires a value that the collector did not observe.

## Validation Order

The decision path must:

1. validate purpose and scope shape;
2. derive the trusted profile identity;
3. require exactly one row for every closed source class;
4. reject any expectation that differs from the trusted profile;
5. reject source classes without coverage;
6. reject `Observed` without at least one supplied source;
7. reject supplied sources paired with `Empty`, failure, denial, unsupported,
   omitted, or inapplicable outcomes;
8. derive completeness from trusted expectation plus observed outcome;
9. bind purpose, scope posture, profile identity, expectations, and outcomes in
   the coverage commitment; and
10. proceed to selection and understanding derivation only after those checks.

Errors must use stable non-leaking codes and must not include source IDs,
semantic values, revisions, local paths, provider identities, or caller-supplied
payloads.

## Security Properties

- A host cannot downgrade a required collector to optional or inapplicable.
- A missing class cannot disappear from the inventory completeness decision.
- Required empty is distinguishable from collector failure or access denial.
- Run-bound work cannot use a pre-run collector profile.
- Provider reconciliation cannot omit current provider observation.
- Advisory memory remains optional and non-authoritative.
- The profile is not an authority credential and cannot authorize execution.
- No source existence or count is exposed through a consumer-visible surface.

## Focused Tests

Add direct in-memory regressions proving:

- every purpose and scope posture yields a complete deterministic matrix;
- input order does not change the profile or coverage commitment;
- a required-to-optional substitution fails closed;
- an inapplicable-to-required substitution fails closed;
- pre-run and run-bound profiles have distinct identities and commitments;
- provider observation is required only for run-bound provider reconciliation;
- all run-bound baseline classes are required;
- all pre-run dynamic run classes are inapplicable;
- required explicit empty remains complete;
- required access denial, failure, unsupported, or omission is incomplete;
- optional failure is degraded but does not make coverage incomplete;
- observed/outcome contradictions still fail closed; and
- profile derivation remains non-authorizing and process-free.

## Implementation Sequence

1. Complete focused maintainer/security review of this matrix and validation
   order.
2. Add the private profile identity and deterministic expectation derivation in
   `continuation_context.rs`.
3. Validate all caller-carried expectations against the derived profile.
4. Bind profile identity and scope posture into the coverage commitment.
5. Add the focused matrix and substitution regressions.
6. Run focused tests, strict clippy, full workspace tests, rustdoc, formatting,
   docs, and diff checks.
7. Complete focused implementation/security review before collector-interface
   planning begins.
8. Keep every operational consumer blocked until a separately reviewed
   Core-owned semantic-obligation profile removes caller control over
   `required_claims` and `required_for_purpose` posture.

## Acceptance

- Core, not the host, decides every collector expectation.
- Every closed source class is accounted for exactly once.
- The expectation matrix is deterministic for exact purpose and scope posture.
- Caller expectation substitution fails before inventory or claim derivation.
- Required empty and required unavailable remain semantically distinct.
- Coverage commitments change across purpose, run-binding posture,
  expectations, or outcomes.
- The implementation remains private, pure, in-memory, and non-authorizing.
- No collector, persistence, public surface, runtime behavior, or dependency is
  added.
- The report states precisely that expectation substitution is closed while
  semantic-obligation substitution remains blocked follow-up work.

## Governed Planning

- workflow: `dg/d`
- run ID: `run-1793697108294074000-2`
- approval ID: `approval/run-1793697108294074000-2/planning-approved`
- presentation ID: `presentation/135bcc8d3ae77406`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Out-Of-Kernel Work

The delegated maintainer inspected the accepted model and P0 plan and authored
this plan outside the kernel. The kernel governed scope and approval but did not
edit files, run documentation checks, create commits, or mutate GitHub.
