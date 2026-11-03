# External Action Fixture Pull-Request Lifecycle Proof Plan

Status: accepted after focused maintainer/security review and blocker
correction.

## Executive Decision

Implement one credential-free, Core-private, fixture-backed pull-request
lifecycle proof for the already accepted delegated-authority connector model.
The proof composes the existing immutable authorization projection, durable
transition lifecycle, current-authority source, approval-presentation proof,
and same-call connector admission into one exact sequence:

1. create one draft pull request for the reviewed head;
2. derive and persist the exact successor pull-request binding;
3. wait until the exact required check set is terminal green;
4. mark that exact pull request ready; and
5. squash-merge only the unchanged reviewed head.

The proof is a deterministic local conformance boundary. It uses no
credentials, network, live GitHub behavior, provider writes, browser
automation, public connector API, schema, SDK, CLI mutation command, scheduler,
or broader provider family.

The objective is not to simulate GitHub convincingly. It is to prove that
Workflow OS can preserve one scoped authorization across an explicitly
approved external lifecycle without asking for a second human authorization,
while still re-resolving current authority and current evidence before every
transition and failing closed on drift, ambiguity, refusal, expiry, or
revocation.

## Why This Phase Is Next

The authoritative roadmap queue accepts four prerequisite boundaries:

- immutable external-action projection and closed transition vocabulary;
- exact successor-resource binding;
- durable transition lifecycle with reservation, attempt, outcome,
  reconciliation, wait, consumption, denial, and revocation posture;
- Core-private same-call current-authority verification and one borrowed
  connector-use capability.

The accepted same-call admission slice still receives approval, policy,
execution-window, revocation, resource, and evidence facts through
`ExternalActionConnectorFixtureFacts`. Those inputs are exact-validated, but
they are caller-injected fixture claims rather than an end-to-end composition
from authoritative sources. The next proof must close that composition gap and
exercise the complete lifecycle, not introduce another primitive family.

## Governing Invariants

1. One approval may authorize several named transitions, but it does not make
   later transitions automatically lawful.
2. Before every connector entry, Core freshly reloads and validates the exact
   projection, approval-presentation proof, approval decision, capability
   grant, policy/governance commitments, execution window, revocation posture,
   resource facts, successor binding, and required evidence.
3. Reconstructed context and fixture observations are orientation and evidence,
   never authority. Material execution still crosses the existing private,
   borrowed, same-call capability boundary.
4. Draft creation is the only transition that may derive a successor PR
   identity. Later PR transitions require the exact persisted successor
   binding; callers cannot substitute a PR number.
5. Provider-operation identity is stable across reconciliation and retry.
   Local attempt identity is distinct for each lawful attempt.
6. Terminal success is replayable but never redispatched.
7. Ambiguous provider outcome blocks redispatch until deterministic lookup and
   reconciliation establish what happened.
8. Pending evidence and connector refusal become durable, inspectable,
   resumable waits rather than agent termination or fabricated completion.
9. Known failure may be retried only after fresh authority and evidence
   resolution and with a distinct attempt identity.
10. A serialized projection, successor binding, lifecycle snapshot, fixture
    observation, or report is not a bearer token.
11. The fixture provider state is authoritative only for this local proof. It
    must not be described as GitHub truth or production provider state.
12. Owner-conducted focused review is repository assurance, not independent
    external security review.

## Closed Lifecycle Scope

### Included transitions

- `GitHubPullRequestCreateDraft`;
- `GitHubPullRequestMarkReadyAfterTerminalGreen`; and
- `GitHubPullRequestSquashMergeUnchangedHead`.

### Excluded transitions

- `GitHubGitRefFastForwardExact`: the fixture begins with the exact reviewed
  head already present on the governed branch, so a ref mutation is not needed
  to prove the PR lifecycle;
- `GitHubBranchDeleteAfterMerge`: branch deletion remains separately
  authorizable and is intentionally absent from this proof.

The immutable projection must list all three included transitions, their exact
payload commitments, and their required evidence before the first connector
entry. Later discovery cannot widen that set.

## Core-Private Proof Architecture

### 1. Authoritative lifecycle input

Add one crate-private proof input that identifies only stable lookup keys and
source interfaces. It may identify the exact project, workflow, run, actor,
projection, and requested lifecycle operation. It must not accept a caller-
constructed aggregate of current facts.

The composition function must reload or derive current facts from the accepted
Core-owned stores and fixture provider source in the same call that admits each
transition.

### 2. Closed source composition

The proof composes these source classes:

| Required fact | Authoritative source for this proof | Freshness boundary |
| --- | --- | --- |
| immutable run and exact operation | immutable bundle/source already accepted by current-authority composition | exact run root and operation binding |
| approval scope and strict non-goals | durable approval-presentation record | exact presentation identity and content commitment |
| granted approval | durable approval decision/proof source | exact approval and presentation binding |
| current capability | registered current-authority source | same-call use and one-time consumption |
| policy and proportional-governance posture | existing accepted decision sources or explicit closed fixture source where no non-consuming source exists | exact source revision and commitment |
| execution window | existing authorized-execution continuity source | open at transition evaluation time |
| revocation | existing grant/revocation source | current and unrevoked at transition evaluation time |
| repository, base, branch, and reviewed head | fixture provider observation source | observation revision read before admission |
| required checks | fixture provider check observation source | exact check set for exact reviewed head |
| pull-request state | fixture provider successor observation source | exact provider revision and successor binding |
| transition history | local external-action lifecycle store | exact operation cursor and atomic append |

Phase 5A must name the exact source contract used for every row before a
transition implementation begins. If an accepted source does not expose a
complete non-consuming read suitable for this proof, Phase 5A may add one
narrow crate-private in-memory proof source with an explicit revision fence and
closed conformance semantics. There is no generic fallback to a caller-provided
fact aggregate. The proof must derive and return one source-coverage commitment
covering every required source class and disposition. It must not relabel
injected facts as authoritative production data. Missing, unsupported, denied,
failed, stale, or revision-drifting required source coverage fails closed.

### 3. Fixture provider state

Add a deterministic in-memory fixture provider store with atomic revisioned
state for exactly one repository lifecycle. The fixture state may contain:

- repository identity and base branch;
- governed head branch and reviewed head SHA;
- next deterministic pull-request number;
- zero or one created pull request bound to the reviewed head;
- draft/ready/merged posture;
- exact required check names and terminal result for the reviewed head;
- exact merge result and resulting merge SHA commitment;
- deterministic provider-operation receipts keyed by stable operation ID; and
- injected refusal or ambiguity posture for focused tests.

The store must provide bounded observation and operation interfaces. Debug and
error output must not expose payload contents. It is a conformance fixture, not
a production transport or provider emulator.

Every provider mutation request must carry the exact provider-state revision
observed during composition. The fixture provider atomically compares that
revision while applying the operation. Revision drift rejects without mutation
and without being classified as provider ambiguity. This compare-and-operate
boundary closes the read-to-mutation race; a pre-call observation check alone
is insufficient.

### 4. Successor binding

Successful draft creation returns the fixture PR number and observation time.
Core derives `ExternalActionSuccessorBinding` from the validated projection,
persists it through a narrow Core-owned create-only exact successor-binding
store, and rereads the exact binding before reporting success. The provider
fixture never owns or authorizes the Core successor binding.

Provider creation success followed by missing, failed, or ambiguous local
successor persistence is not a known failure and must never trigger blind
creation retry. The operation remains ambiguous until reconciliation finds the
stable provider-operation receipt, reconstructs the exact observed PR identity,
derives the same successor binding, and completes idempotent Core persistence.
Conflicting successor persistence fails closed as a security error.

Mark-ready and merge composition must load that binding by projection identity
and reject:

- missing binding;
- a different PR number;
- a different repository;
- a different reviewed head;
- a different create-operation commitment;
- changed or corrupt binding commitment; and
- caller-supplied successor substitution.

### 5. Transition admission

For each transition, the end-to-end composition must:

1. load the immutable projection and exact transition;
2. load the current source revisions and fixture provider observation;
3. verify approval presentation and approval decision cover the complete
   lifecycle scope;
4. verify policy, proportional-governance, capability, delegation, execution
   window, expiry, and revocation posture;
5. load and validate the exact successor binding where required;
6. evaluate the transition-specific evidence predicates;
7. inspect durable lifecycle state for replay, wait, ambiguity, denial, or
   terminal posture;
8. obtain one borrowed same-call connector-use capability;
9. atomically admit one local attempt;
10. call the fixture provider once;
11. record success, known failure, or ambiguity durably; and
12. return a payload-free proof outcome derived from reread state.

No caller-provided boolean such as `checks_green`, `window_open`, `revoked`, or
`head_matches` may decide admission in the new end-to-end entrypoint.

The composition must capture a pre-read source anchor, load all required
sources, recapture the source anchor before admission, and reject any drift.
The connector then performs the provider operation with the exact expected
provider revision as described above. Source-anchor equality does not replace
the connector's atomic provider compare-and-operate requirement.

## Transition-Specific Evidence

### Create draft

Required evidence:

- reviewed head still matches;
- base target still matches; and
- exact draft payload commitment still matches.

Success creates exactly one fixture PR and one successor binding. Exact replay
returns the existing binding and does not allocate another PR number.

### Mark ready

Required evidence:

- exact successor binding remains valid;
- reviewed head and base target still match;
- exact ready-state payload commitment matches; and
- the complete required check set is terminal green for the reviewed head.

Pending checks register an evidence wait. A failed required check blocks the
current evidence revision and also registers a typed evidence wait unless the
governing policy explicitly makes that failure terminal. A later green
observation may resolve the wait only through an explicit resume call that
rehydrates all current sources. No failed-check posture may be silently treated
as approval or as permanent transition denial without the policy distinction.

### Squash merge

Required evidence:

- exact successor binding remains valid;
- reviewed head and base target still match;
- exact merge payload commitment matches;
- required checks remain terminal green;
- the pull request is ready; and
- review and policy predicates remain satisfied.

Success records the exact merge result. Replay returns the existing terminal
outcome and never invokes the fixture provider again.

## Wait And Resume Semantics

The proof supports two bounded wait classes:

1. evidence wait, such as required checks still pending; and
2. connector refusal wait, where the connector proves before provider dispatch
   that it cannot currently accept the otherwise lawful operation.

The wait record binds the exact run, projection, transition operation, attempt
where applicable, execution window, source revisions, reason class, and
payload-free detail commitment.

Resume is explicit and local. It must:

- reload the current projection and all current sources;
- reject expired, revoked, changed-head, changed-successor, or widened scope;
- verify the wait is still the active wait for the exact operation;
- resolve the prior wait atomically; and
- perform a fresh same-call admission if the operation is now lawful.

This phase does not add a scheduler, polling loop, process owner, automatic
agent continuation, or background retry.

An expired or closed execution window is not resumable under the old
authorization and must not be represented as an active wait. It rejects the
transition and requires a separately governed successor authorization. A
connector response that cannot prove provider dispatch did not begin is not a
refusal wait; it is an ambiguous outcome and follows reconciliation rules.

## Ambiguity And Reconciliation

The fixture connector can return an ambiguous result after the fixture
provider operation may or may not have committed. On ambiguity:

- lifecycle state records `OutcomeAmbiguous`;
- redispatch is blocked;
- the stable provider operation ID remains unchanged;
- reconciliation performs a bounded fixture lookup by that operation ID;
- a found provider receipt reconciles to succeeded or known failed;
- an authoritative not-found result may reconcile to known failed only when
  the fixture source revision proves the lookup covered the attempted window;
- an unavailable or inconclusive lookup preserves ambiguity; and
- any lawful retry after known failure receives a distinct local attempt ID.

No timeout alone may be interpreted as provider failure.

Draft-create reconciliation must also repair the Core-owned successor binding
idempotently when the provider receipt proves creation but the first local
binding write did not complete. Ready and merge reconciliation must preserve
the existing exact successor binding and reject any provider receipt for a
different PR or head.

## Payload-Free Proof Outcome

The end-to-end proof returns a crate-private result that may disclose:

- projection, transition, operation, and successor-binding commitments;
- current source-revision commitment;
- lifecycle status and cursor;
- attempt posture without payload;
- wait, refusal, reconciliation, denial, and consumption posture;
- remaining projected transitions; and
- whether the fixture provider was entered.

It must not contain credentials, raw approval text, raw policy content, raw PR
body/title, logs, provider response payloads, or a reusable capability.

## Failure And Threat Matrix

Focused tests must cover at least:

| Condition | Required behavior |
| --- | --- |
| changed reviewed head | reject before connector entry |
| changed base target | reject before connector entry |
| stale or missing approval presentation | reject before connector entry |
| approval scope omits a later transition | reject that transition; no scope widening |
| expired or revoked grant | reject before connector entry |
| execution window closed or expired | reject old authorization; no active wait and no dispatch |
| changed policy/governance commitment | reject before connector entry |
| missing or substituted successor binding | reject mark-ready/merge |
| checks pending | durable evidence wait; no connector entry |
| checks failed | typed evidence wait/block for that revision unless policy makes it terminal; no connector entry |
| connector refusal with proof dispatch did not begin | durable resumable wait |
| connector response without no-dispatch proof | ambiguous; reconciliation required |
| known connector failure | durable failure; fresh lawful retry uses new attempt ID |
| ambiguous create/ready/merge | block redispatch until reconciliation |
| exact replay after each success | return existing outcome; no duplicate provider entry |
| replay after merge | no duplicate merge |
| corrupt fixture or lifecycle state | fail closed |
| concurrent admission | at most one admitted provider attempt for the operation |
| source revision changes during composition | reject and require fresh composition |
| provider revision changes between observation and mutation | atomic compare-and-operate rejection; no mutation |
| provider create succeeds but successor persistence does not | ambiguous; reconcile receipt and idempotently persist exact binding |
| serialized projection or proof replay | no authority without fresh same-call resolution |

## Implementation Sequence

### Phase 5A: Source and fixture contracts (implemented and review-accepted)

- define the closed source-composition input;
- define revision-fenced fixture provider observation and operation contracts;
- define Core-owned create-only exact successor-binding persistence;
- define complete source-coverage and pre/post source-anchor commitments; and
- add conformance tests for drift, corruption, and substitution.

### Phase 5B: Draft creation and successor binding

- compose authoritative inputs for create-draft;
- execute one fixture create operation under same-call authority;
- derive, persist, and reread the successor binding; and
- prove exact replay without duplicate PR creation, including recovery when
  provider creation succeeds before local successor persistence completes.

### Phase 5C: Evidence wait and ready transition

- compose required-check observations for the exact head;
- persist and resolve pending-check and connector-refusal waits;
- mark only the exact successor PR ready; and
- prove failed, stale, and changed-head paths never enter the connector.

### Phase 5D: Merge, replay, and ambiguity recovery

- compose the exact unchanged-head merge transition;
- prove one successful squash merge and terminal replay;
- exercise ambiguous outcomes for create, ready, and merge;
- reconcile from deterministic fixture provider receipts; and
- prove known-failure retry uses distinct attempt identity without changing
  provider operation identity.

### Phase 5E: End-to-end conformance and review

- run one complete create-draft to ready to squash-merge proof under one
  explicitly approved projection;
- verify no second human authorization is required inside the approved scope;
- inspect the durable lifecycle and successor records;
- document exact source ownership and remaining fixture-only boundaries;
- run focused and complete validation; and
- complete focused maintainer/security review before any live connector plan.

Each implementation phase must remain independently reviewable. A blocker in
an earlier subphase prevents later subphases from being claimed complete.

## Validation

At minimum:

```text
cargo fmt --all -- --check
cargo test -p workflow-core external_action
cargo test -p workflow-core --all-features --lib
cargo clippy -p workflow-core --all-targets --all-features -- -D warnings
npm run check:docs
git diff --check
```

Focused tests must assert connector entry counts, lifecycle cursors, stable
provider operation identity, distinct attempt identity, exact successor
binding, wait resolution, reconciliation, replay, and redaction-safe debug/error
surfaces.

## Acceptance Criteria

This phase is accepted only when:

- one exact fixture-backed PR lifecycle completes create-draft, mark-ready, and
  squash-merge under one complete, proof-backed authorization projection;
- every transition freshly resolves current authority and current evidence;
- authoritative source composition replaces caller-authored current-fact
  booleans at the end-to-end entrypoint;
- complete source coverage, stable pre/post source anchors, and atomic provider
  revision compare-and-operate close both source-read and provider-mutation
  races;
- the created PR identity is derived once and later transitions bind exactly to
  it;
- provider-success/local-successor-persistence ambiguity is reconciled without
  duplicate draft creation;
- pending evidence and connector refusal produce inspectable, resumable waits;
- ambiguous outcomes block redispatch and reconcile without duplicate provider
  action;
- exact replay at every successful transition does not re-enter the connector;
- changed head, stale evidence, expiry, revocation, missing presentation proof,
  source drift, and out-of-scope transitions fail closed;
- the proof discloses fixture-only authority and does not claim live GitHub,
  network, credential, scheduler, public runtime, or production connector
  support;
- all required validation passes; and
- focused maintainer/security review finds no unresolved blocker.

## Focused Review Corrections

The focused review found and corrected three blockers:

1. source reads were revision-fenced but the provider operation did not require
   an atomic expected-revision comparison, leaving a read-to-mutation race;
2. successor-binding persistence was described as fixture-owned rather than a
   Core-owned governance record; and
3. provider create success followed by local successor persistence failure had
   no explicit ambiguity and repair path.

It also corrected three important semantic defects: closed execution windows
are not resumable waits, connector refusal is a wait only with proof that
provider dispatch did not begin, and failed checks are evidence blockers rather
than permanent denial unless policy explicitly says otherwise.

## Exit And Next Decision

Acceptance of this proof closes delivery-sequence Phase 5 only. It does not
automatically authorize Phase 6.

A separate governed decision must assess whether the existing GitHub connector
and platform permission boundary can consume these contracts without creating
a second authorization system. That decision must account for credentials,
live provider observations, platform permissions, reconciliation guarantees,
rate limits, and operational failure posture before any live write path is
implemented.
