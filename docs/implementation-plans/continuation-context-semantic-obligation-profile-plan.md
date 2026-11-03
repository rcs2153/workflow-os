# Continuation Context Semantic-Obligation Profile Plan

Status: Accepted after focused maintainer/security review. Implementation has
not started.

Related foundations:

- [Authoritative Agent Continuation Context And Rehydration Plan](authoritative-agent-continuation-context-rehydration-plan.md)
- [Continuation Context Coverage Profile Plan](continuation-context-coverage-profile-plan.md)
- [Authoritative Continuation Context Model Report](../concepts/AUTHORITATIVE_CONTINUATION_CONTEXT_MODEL_REPORT.md)
- [Continuation Context Coverage Profile Report](../concepts/AUTHORITATIVE_CONTINUATION_CONTEXT_COVERAGE_PROFILE_REPORT.md)
- [Continuation Context Semantic-Obligation Profile Plan Review](../concepts/CONTINUATION_CONTEXT_SEMANTIC_OBLIGATION_PROFILE_PLAN_REVIEW.md)

## 1. Executive Summary

The private continuation-context model now owns collector expectations for all
eighteen closed source classes. That closes one caller-downgrade path, but it
does not yet make the meaning of an authoritative understanding projection a
Core-owned contract.

Two caller-carried inputs still control semantic strictness:

- `ContinuationContextDecisionInput.required_claims` decides which understanding
  claims must exist; and
- `ContinuationContextSourceDefinition.required_for_purpose` decides whether a
  stale, unauthorized, invalid, or over-budget source blocks.

A caller can therefore request a run-bound material-continuation decision,
clear every required claim, label a material source optional, and receive a
ready projection even though Core's collector-coverage profile was complete.
That is incompatible with authoritative post-compaction understanding.

The next P0 increment must add one private, closed, purpose- and scope-bound
semantic-obligation profile. Core, not the caller, must derive:

- which claim kinds require at least one value;
- which claim kinds require complete accounting even when the result is empty;
- which claim kinds are optional or inapplicable;
- which source classes are mandatory, supplemental, advisory-only, or
  inapplicable;
- whether a claim kind is singular or set-valued; and
- how equal-authority values are resolved.

This phase remains model-only. It must not add collectors, persistence,
accepted-intent capture, CLI output, runtime consumers, schemas, Salsa, provider
actions, or execution authority.

## 2. Required Invariant

```text
For one validated continuation purpose and scope, Core owns the complete
semantic-obligation profile. A caller may supply observations, but it cannot
weaken required understanding, relabel a material source as optional, collapse
set-valued work into one value, or turn missing understanding into readiness.
```

Collector coverage and semantic completeness are separate proofs:

```text
collector coverage: did every required collector participate with an allowed
outcome?

semantic completeness: did the admitted observations satisfy every Core-owned
claim and source obligation for this purpose and scope?
```

An `Empty` collector result can satisfy collector coverage. It satisfies a
semantic obligation only when the obligation allows an authoritative empty
accounting result. It can never synthesize a value that was not observed.

## 3. Problem Detail

### 3.1 Caller-owned required claims

`required_claims` is currently a public field on the private decision input.
Tests can clear it, and production composition could do the same. The projection
then has no reason to emit a missing-claim gap for objective, scope, remaining
work, validation, reporting, or lawful next options.

The field must be removed from caller control. Core should derive claim
obligations after validating the purpose and whether the scope is pre-run or
run-bound. A caller substitution API is unnecessary. If a temporary internal
compatibility path is required during refactoring, supplied obligations must be
compared with the exact Core profile and rejected on any mismatch.

### 3.2 Caller-owned source necessity

`required_for_purpose` is currently supplied on each source definition. That
field controls selection priority and whether source gaps block. A caller can
therefore relabel an admitted policy, event-history, accepted-intent, required-
context, evidence, or WorkReport source as optional.

Core must derive necessity from the semantic profile and source class. The
source definition may describe what was observed, but it must not describe how
important the observation is to governance.

### 3.3 Required value versus required accounting

Some claim kinds require a positive value. An objective, scope, remaining
obligation, or lawful next option cannot be complete when absent.

Other claim kinds require complete accounting but may legitimately be empty.
A run may have no blocker, no registered wait, no accepted amendment, no open
question, or no promised follow-up. Requiring callers to invent semantic text
such as `none` would create unauditable negative facts and invite model
fabrication.

The profile must distinguish:

- `RequiredValue`: at least one admissible value is required;
- `RequiredAccounting`: an authoritative value set or an authoritative empty
  observation is required;
- `Optional`: absence does not block but remains disclosed when relevant; and
- `Inapplicable`: the claim kind must not contribute to this profile.

An authoritative empty result must come from collector and source posture that
Core recognizes for that claim kind. It is typed absence, not free-form prose.

### 3.4 Singular versus set-valued claims

The current derivation treats different equal-rank values as a conflict. That
is correct for singular claims such as objective or scope, but incorrect for
legitimate sets such as:

- accepted amendments;
- strict non-goals;
- decisions and rationale references;
- completed, in-progress, deferred, and remaining obligations;
- blockers, waits, questions, assumptions, and uncertainties;
- evidence and check requirements;
- validation and reporting commitments;
- promised follow-ups; and
- lawful next options.

The profile must assign claim cardinality:

- `Singleton`: the highest-authority value must be unique; different values at
  the same winning authority rank are a typed conflict; or
- `Set`: all distinct values at the winning authority rank are retained in
  deterministic order with per-value provenance.

Set union must not guess whether two natural-language values contradict each
other. Contradiction requires typed source posture or a separately reviewed
reconciliation model. The first increment preserves both values and their
provenance.

## 4. Proposed Private Model

The first implementation should remain private to `workflow-core` and use
closed enums similar to the coverage-profile increment.

Candidate types:

```text
ContinuationContextSemanticProfileId
ContinuationClaimRequirement
ContinuationClaimCardinality
ContinuationClaimResolution
ContinuationClaimObligation
ContinuationSourceNecessity
ContinuationSourceObligation
ContinuationContextSemanticProfile
ContinuationClaimAccountingOutcome
```

`ContinuationContextSemanticProfileId` should have twelve variants: one for
each existing continuation purpose in pre-run and run-bound scope. The identity
must be derived only after `ContinuationContextScope` validation.

Each `ContinuationClaimObligation` should bind:

- claim kind;
- requirement level;
- cardinality;
- resolution rule;
- permitted authority domains, reusing the existing closed authority mapping;
  and
- whether a typed authoritative empty accounting result is allowed.

Each `ContinuationSourceObligation` should bind:

- source class;
- one Core-owned necessity posture;
- whether supplied sources in that class must survive selection; and
- whether the class may satisfy required values, required accounting, optional
  support, or advisory context only.

The first source-necessity vocabulary should be:

- `Mandatory`: each admitted source in the class must be selected or produce a
  blocking typed gap;
- `Supplemental`: selection failure is disclosed and blocks only when a
  required claim cannot otherwise be satisfied;
- `AdvisoryOnly`: the source may contribute only advisory posture and can never
  satisfy a required claim; and
- `Inapplicable`: sources in the class are rejected for this profile.

Core must reject a supplied source class that the semantic profile marks
inapplicable. It must not silently ignore the source or allow the caller to use
it to affect commitments.

## 5. Claim-Obligation Matrix

The implementation should encode the matrix in Rust, not parse it from YAML or
accept it from a caller. The table below is the reviewed semantic contract.

Legend:

- `V`: required value;
- `A`: required accounting, including an authoritative empty result;
- `O`: optional; and
- `I`: inapplicable.

### 5.1 Pre-run profiles

| Claim kind | Planning | Implementation | Approval review | Provider reconciliation | Child handoff | Material continuation |
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

### 5.2 Run-bound profiles

Run-bound profiles use the same matrix with these stricter overrides:

- completed, in-progress, and deferred obligations remain required accounting;
- remaining obligation is a required value for every purpose except approval
  review, where complete accounting is sufficient;
- evidence requirement, check requirement, SideEffect posture, validation
  commitment, and reporting commitment are required values for implementation,
  approval review, child handoff, and material continuation;
- evidence requirement and SideEffect posture are required values for provider
  reconciliation;
- registered waits, blockers, questions, assumptions, uncertainty, and promised
  follow-ups require authoritative accounting even when empty; and
- lawful next option remains a required value. A terminal run must use a typed
  terminal disposition rather than omit the claim silently.

The implementation report must print the exact encoded matrix. Reviewers should
not have to infer it from nested match expressions.

## 6. Cardinality And Resolution Matrix

The first implementation should use `Singleton` plus highest-authority unique
resolution for:

- objective;
- priority; and
- scope.

All other current claim kinds should be `Set` plus deterministic union of
distinct values at the winning authority rank. Each retained value must keep
its own ordered provenance list. A lower-rank value may remain available only
as disclosed supporting or advisory context; it must not replace or merge into
the authoritative value set silently.

`SideEffectPosture` is set-valued because one bounded work unit may contain
multiple disclosed effects. `LawfulNextOption` is set-valued because more than
one independently lawful option may exist. Selection of one exact material
operation remains a separate same-call authority decision.

## 7. Source-Necessity Matrix

The semantic profile should initially align source necessity with the accepted
coverage profile while keeping the two proofs separate.

### 7.1 Pre-run

`Mandatory`:

- validated declaration;
- policy;
- required context;
- accepted intent;
- Git observation; and
- operating document.

`Supplemental`:

- evidence;
- check;
- WorkReport;
- typed handoff; and
- provider observation.

`AdvisoryOnly`:

- agent advisory.

`Inapplicable`:

- immutable run bundle;
- durable run state;
- event history;
- approval;
- capability; and
- SideEffect.

### 7.2 Run-bound

Every source class is `Mandatory` except:

- agent advisory, which is `AdvisoryOnly`; and
- provider observation, which is `Mandatory` for provider reconciliation and
  `Supplemental` for every other purpose.

This matrix governs admitted source objects. Collector coverage separately
proves whether the source class was observed, empty, denied, failed,
unsupported, skipped, omitted, or inapplicable. A mandatory class with an
allowed empty collector result does not require a fabricated source object;
claim accounting still determines whether the empty result is semantically
sufficient.

## 8. Commitment And Readiness Rules

The semantic profile must be included in deterministic commitments for:

- source selection;
- private inventory;
- bounded bundle;
- understanding projection; and
- overall continuation-context decision.

At minimum, the commitment must bind:

- semantic profile identity and algorithm version;
- purpose and validated scope posture;
- every claim obligation;
- every source obligation;
- accounting outcomes;
- cardinality and resolution outcomes;
- retained value commitments and provenance; and
- typed gaps.

Readiness requires all of the following:

1. collector coverage is complete;
2. every mandatory admitted source is selected or represented by a blocking
   gap;
3. every required-value claim has at least one admissible retained value;
4. every required-accounting claim has an admissible value set or a typed
   authoritative empty result;
5. every singleton claim has one unique winning value;
6. no inapplicable source or claim affected the decision;
7. no required value is supplied only by advisory input; and
8. all existing sensitivity, integrity, freshness, access, budget, precedence,
   and conflict checks pass.

The understanding projection remains non-authorizing even when ready.

## 9. Error Posture

The implementation should add stable, payload-free errors or typed gaps for:

- caller semantic-profile substitution;
- caller source-necessity substitution;
- inapplicable source class;
- inapplicable claim kind;
- missing required value;
- missing required accounting proof;
- advisory-only satisfaction attempt;
- singleton equal-authority conflict;
- unsupported claim cardinality; and
- semantic-profile commitment mismatch.

Errors must not echo claim text, source identity, repository paths, prompts,
provider payloads, or protected source topology.

## 10. Implementation Sequence

1. Add private closed semantic-profile types and the twelve profile identities.
2. Derive the profile only after purpose and scope validation.
3. Remove caller control over `required_claims` and
   `required_for_purpose`. If a temporary migration field remains, reject any
   substitution before source selection.
4. Derive source necessity from the profile before ordering or budget
   selection.
5. Add typed required-accounting outcomes so empty posture is proven rather
   than represented as prose.
6. Refactor claim derivation to honor singleton and set cardinality while
   preserving per-value provenance.
7. Bind the full semantic profile and derivation outcomes into every dependent
   commitment.
8. Add focused in-memory tests.
9. Produce an implementation report that prints the encoded matrices and
   discloses every deferred runtime surface.
10. Perform focused maintainer/security review before collector or runtime
    integration.

## 11. Required Tests

Focused tests must prove:

- all twelve purpose/scope profiles are selected deterministically;
- every claim kind appears exactly once in every profile;
- every source class appears exactly once in every profile;
- clearing or substituting required claims is impossible or rejected;
- relabeling a mandatory source as optional is impossible or rejected;
- a complete collector profile does not imply semantic completeness;
- required-value absence blocks;
- required-accounting accepts a typed authoritative empty result;
- free-form `none` text does not substitute for accounting proof;
- advisory-only input cannot satisfy a required claim;
- optional or supplemental sources cannot weaken mandatory posture;
- multiple equal-rank set values are retained deterministically rather than
  treated as a conflict;
- multiple equal-rank singleton values produce a typed conflict;
- lower-rank values cannot replace winning values;
- per-value provenance survives set union;
- source and claim input order does not change commitments;
- profile identity and all obligations affect commitments;
- inapplicable sources and claims fail closed;
- required source gaps survive budget, sensitivity, freshness, integrity, and
  access handling;
- accepted intent remains non-authorizing;
- ready understanding remains non-authorizing; and
- existing continuation-context and coverage-profile tests remain green.

## 12. Non-Goals

This increment does not authorize:

- collectors or repository discovery;
- accepted-intent persistence or capture;
- runtime event, state, approval, capability, or SideEffect integration;
- a CLI preview or public schema;
- agent-visible inventory or understanding projection;
- host compaction integration or delivery receipts;
- a Salsa dependency, prototype, or cache;
- provider reads or writes;
- nested harness execution;
- model-generated negative facts;
- natural-language contradiction detection;
- workflow-authored semantic profiles; or
- execution authority from a ready projection.

## 13. Acceptance Criteria

- Core owns one closed semantic profile for every purpose and scope posture.
- Callers cannot reduce required claims or source necessity.
- Required values and required accounting are distinct typed obligations.
- Legitimate multi-valued work is retained with deterministic provenance.
- Singular equal-authority disagreement fails closed.
- Collector coverage and semantic completeness remain separate commitments.
- Semantic readiness remains orientation only and grants no authority.
- The first implementation remains private and model-only.
- Focused tests and the full repository suite pass on the exact reviewed head.
- Focused security review accepts the implementation before collectors or
  runtime consumers proceed.

## 14. Recommended Next Phase

Implement this private model-only profile in
`crates/workflow-core/src/continuation_context.rs`, with focused tests, an
implementation report, and focused maintainer/security review. Do not begin
collectors, accepted-intent persistence, CLI exposure, Salsa evaluation, or a
runtime consumer in that phase.
