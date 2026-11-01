# Delegated Authority Projection Into External Connectors Plan

## Status

P0 planning requirement, accepted after focused maintainer/security review in
[Delegated Authority External Connector Projection Plan Review](../concepts/DELEGATED_AUTHORITY_EXTERNAL_CONNECTOR_PROJECTION_PLAN_REVIEW.md).

This plan preempts broader provider mutation families, nested harness work, and
claims of continuous delegated-maintainer autonomy. It extends existing scoped
runtime authority, approval-presentation proof, current-authority same-call
resolution, authorized-execution continuity, provider reconciliation, and
proportional-governance contracts. It does not create a parallel permission
system.

## Problem

A governed PR phase can hold current proof-backed authority for an exact PR
lifecycle while an external connector independently asks the human to authorize
an intermediate transition. Both systems fail closed, but lawful work stalls
because the connector cannot verify the kernel's scoped authority.

No typed external-action projection currently binds kernel authority to a
connector action. An approval handoff can also name only an initial mutation,
such as draft creation, without enumerating later intended transitions such as
ready-for-review and squash merge.

## Invariant

A current scoped kernel authorization may project into a connector-verifiable
capability. The connector executes only listed transitions while the exact
resource, immutable commit, evidence predicates, execution window, delegation,
expiry, revocation, and consumption constraints remain satisfied.

No additional human approval is required inside that exact approved scope.
Missing, stale, changed, incomplete, ambiguous, or expanded scope fails closed.

A serialized projection is not bearer authority. Only Core may resolve current
authority and deliver one same-call use capability to the connector admission
boundary. Durable projection and lifecycle records prove attribution and
support recovery; they do not independently authorize a mutation.

## Source-Of-Truth Boundaries

| Concept | Source of truth | Must not be confused with |
| --- | --- | --- |
| Governed approval | Durable approval decision and presentation proof | Platform permission or connector execution |
| Capability grant | Existing scoped runtime authority source | One completed provider mutation |
| Authorization projection | Immutable payload-free commitment to a closed transition set | Mutable consumption state or a bearer token |
| Same-call use capability | Core-owned current-authority resolution immediately before connector entry | A deserialized projection or stale readiness result |
| Transition lifecycle | Durable authorized-execution continuity state | The immutable projection |
| Provider observation | External result or bounded provider reference | Local lifecycle completion by itself |
| Platform permission | Connector credential and provider authorization | Kernel approval |
| WorkReport | Governed disclosure and handoff | Authority or state reconciliation |

These boundaries are mandatory. Approval does not create credentials. A grant
does not prove every transition remains lawful. A projection does not execute
or carry secret material. Provider success does not prove local lifecycle
closure.

## Immutable Authorization Projection

Define a typed, payload-free `ExternalActionAuthorizationProjection` with:

- projection ID, version, and deterministic projection commitment;
- governed workflow, run, approval, and approval-presentation proof references;
- capability-grant, policy-decision, and proportional-governance references;
- authorized-execution window identity and immutable run-bundle reference;
- connector kind and canonical provider account/installation reference when
  required;
- exact repository, base ref, base commitment, head ref, and reviewed head SHA;
- a closed, canonically ordered permitted-transition set;
- transition-specific target and payload-class commitments;
- exact evidence-predicate set and evidence freshness requirements;
- actor, delegator, delegate, and bounded delegation scope;
- issued-at, not-before, expires-at, and revocation-source references;
- sensitivity, redaction, and report-disclosure posture; and
- projection derivation/version metadata.

The immutable projection does not contain consumed transitions, mutable
remaining-authority state, credentials, provider payloads, conversation tokens,
raw approval prose, raw evidence, or broad repository permission.

Approval-presentation proof must bind the exact projection commitment or the
same canonical transition, resource, evidence, payload-class, and non-goal
commitments. A proof reference without exact content binding is insufficient.

## Durable Transition Lifecycle

Track use separately in an `ExternalActionAuthorizationState` composed with
the existing authorized-execution continuity boundary. For each listed
transition, persist bounded posture such as:

- `available`;
- `reserved`;
- `attempted`;
- `succeeded`;
- `denied`;
- `waiting`;
- `ambiguous`; or
- `expired_or_revoked`.

Each transition has a deterministic operation/idempotency identity. Reservation,
attempt admission, and local lifecycle change must use an atomic Core-owned
boundary. Exact replay reconciles to the prior durable result and never repeats
the provider mutation. Ambiguous provider or local outcomes block redispatch
until explicit lookup/reconciliation resolves them.

`remaining transitions` is derived from the immutable projection plus durable
lifecycle state. It is never caller-authored authority.

## Pull Request Transition Vocabulary

The first closed action set is:

- `github.pull_request.create_draft`;
- `github.git_ref.fast_forward_exact` when an exact reviewed commit must be
  pushed to the bound head ref;
- `github.pull_request.mark_ready_after_terminal_green`;
- `github.pull_request.squash_merge_unchanged_head`; and
- `github.branch.delete_after_merge`, only when separately listed.

Draft creation does not imply a ref update, ready transition, merge, or branch
deletion. A phase intending the full lifecycle must present and approve every
anticipated transition.

### Resource Binding Across Creation

Before draft creation, no PR number exists. The create transition binds:

- exact repository;
- exact base ref and base commitment posture;
- exact head ref and reviewed head SHA;
- deterministic creation operation identity;
- title/body payload-class and content commitments; and
- draft-only creation posture.

Successful creation returns a bounded provider PR identity. Core records that
identity as a derived successor binding under the same immutable projection.
Later transitions must use that exact derived PR identity; callers may not
substitute a different PR number or repository.

If a ref update is listed, it binds exact expected-old SHA, exact new reviewed
SHA, non-force posture, repository, and ref. It is not modeled as a PR update.

Mark-ready binds the exact derived PR and unchanged reviewed head. Squash merge
also binds merge method plus merge-title/message payload commitments without
storing raw payload. Branch deletion binds the exact non-default,
non-protected head ref and may occur only after the exact merge transition is
durably reconciled as successful.

## Current-Authority Connector Admission

Immediately before every provider mutation, Core must:

1. reload the immutable run bundle, projection, current approval proof, grant,
   execution window, revocation source, transition state, and evidence sources;
2. verify that the requested action is listed and not already consumed or
   ambiguously attempted;
3. verify exact provider account, repository, base, PR successor binding, ref,
   expected-old SHA, reviewed head SHA, payload commitment, and transition
   identity as applicable;
4. reassess all action-specific evidence predicates for the exact head and
   required freshness window;
5. verify the approval-presentation proof binds the complete transition scope;
6. verify actor, delegation, capability, policy, proportional-governance,
   expiry, revocation, and execution-window facts remain current;
7. reserve the exact durable transition operation atomically; and
8. pass one non-serializable, non-reusable same-call use capability to the
   injected connector boundary.

The connector accepts only that Core-owned use capability plus the separately
resolved platform credential. It must not trust a caller-supplied projection,
conversation approval, serialized receipt, or cached readiness assessment.

Platform credentials remain a separate permission layer. Kernel authority
does not create credentials or bypass provider authorization.

## Evidence Predicates

- create draft: exact reviewed commit, branch, base target, payload commitment,
  and repository binding are current;
- ref fast-forward: provider ref equals expected-old SHA and update target is
  the exact reviewed SHA with force disabled;
- mark ready: the exact required check set is terminal green for the same head,
  observations are fresh, and no required review/policy predicate is missing;
- squash merge: PR is ready, the exact required checks remain green, review and
  policy predicates remain satisfied, base/head bindings are current, and the
  reviewed head is unchanged; and
- delete branch: the exact merge is durably reconciled as successful, deletion
  was separately authorized, and the ref is neither default nor protected.

Evidence predicates are typed, canonically committed, source-bound, and
reassessed immediately before mutation. `all checks passed` without an exact
required-check set, head binding, source, observation time, and freshness rule
is not sufficient evidence.

## Refusal, Wait, And Continuation

Classify connector admission and execution failures without collapsing them
into agent termination:

- `authority_denied`: terminal for the attempted transition until new authority;
- `evidence_incomplete`: resumable only when the named predicate can change;
- `platform_permission_missing`: resumable after credential/permission repair;
- `connector_unavailable`: resumable under bounded retry/wake policy;
- `rate_limited`: resumable at a bounded provider-supplied wake time;
- `external_state_conflict`: requires reassessment and may invalidate authority;
- `provider_outcome_ambiguous`: blocks redispatch until lookup/reconciliation;
  and
- `out_of_scope`: terminal under the current projection.

Persist the attempted transition, refusal class, failed predicate or bounded
platform-permission posture, exact projection/state references, safe next
action, wake condition when known, and remaining-authority posture.

The current phase defines durable wait semantics only. Automatic scheduler
resumption is a later integration and must not be claimed until a trusted
scheduler consumes the wait condition. When present, the scheduler may resume
only after the condition is satisfied and all authority facts are reassessed.
Human workflow restart is not required for connector-local transient refusal
inside still-current authority.

## Provider Outcome And Reconciliation

Connector execution must compose the existing provider-write reconciliation
rules:

- verify authority and reserve immediately before mutation;
- record attempted posture before provider entry;
- persist bounded success, classified failure, or ambiguity;
- never retry an ambiguous mutation blindly;
- use provider lookup/reconciliation before deciding whether a mutation may be
  redispatched; and
- treat remote success with local-transition failure as reconciliation-required,
  not as available authority.

The same operation identity must span projection transition, continuity state,
SideEffect, adapter telemetry, provider observation, events, audit, and report
references without storing raw provider payloads.

## Reporting

WorkReports and inspect output distinguish:

- projected and currently available transitions;
- reserved and attempted transitions;
- connector-admitted transitions;
- provider-observed outcomes;
- durably reconciled successes or failures;
- denied and out-of-scope transitions;
- expired or revoked authority;
- ambiguous outcomes and retry blocks;
- connector/platform permission failures; and
- resumable wait conditions.

Documentation separately explains kernel approval, Core-issued same-call
capability, adapter admission, platform permission, provider outcome, and local
reconciliation.

## Required Tests

- immutable projection canonicalization and deterministic commitment;
- approval-presentation scope commitment mismatch rejection;
- complete fixture-backed green PR lifecycle without a second human prompt;
- draft creation deriving one exact PR successor binding;
- caller substitution of repository, PR, branch, base, head, transition,
  payload commitment, actor, or delegate;
- changed-head rejection at every later transition;
- missing, pending, failed, stale, or wrong-head required checks;
- expired and revoked grants or execution windows;
- unlisted transition rejection;
- exact replay without duplicate ref update, draft, ready transition, merge, or
  branch deletion;
- provider-success/local-failure and transport ambiguity blocking redispatch;
- connector refusal persisted as an inspectable resumable wait;
- transition state cannot broaden immutable projection scope;
- branch deletion requires separate authority and rejects protected/default
  refs; and
- reports classify projected, reserved, attempted, admitted, observed,
  reconciled, denied, ambiguous, consumed, and remaining posture truthfully.

## Delivery Sequence

1. focused maintainer/security review of this plan: complete;
2. model-only immutable authorization projection, closed PR transition
   vocabulary, successor-resource binding, and validation: next;
3. durable transition lifecycle, reservation, consumption, revocation, and
   wait-condition records composed with authorized-execution continuity;
4. Core-private same-call verifier/use capability with injected fixture
   connector only;
5. one end-to-end fixture-backed PR lifecycle proof including replay,
   ambiguity, and wait recovery;
6. separately governed integration with the existing GitHub connector and
   platform permission boundary; and
7. only then reconsider broader provider mutation or delegated-maintainer
   continuity claims.

The model phase must not add connector execution, provider calls, credentials,
live GitHub behavior, workflow schemas, CLI mutation behavior, or scheduler
claims.

## Non-Goals

- no weakening or bypass of connector safety;
- no browser automation as a connector-control bypass;
- no broad repository mutation authority;
- no unlimited standing delegation;
- no credential issuance, discovery, loading, or storage;
- no trusted deserialized/bearer projection;
- no new live provider mutation family in the model phase;
- no automatic scheduler or supervisor in the model phase;
- no enterprise IdP, organization administration, or hosted control plane;
- no nested harness runtime; and
- no parallel authorization system.

## Acceptance

The fixture-backed acceptance proof completes every explicitly approved PR
transition without a second human authorization. Every mutation attempt is
attributable to a current scoped kernel capability, exact approval-presentation
proof, immutable projection, and durable transition operation. Stale,
changed-head, evidence-incomplete, expired, revoked, ambiguous, and out-of-scope
actions fail closed. Connector refusal is durable and resumable. Reports expose
exact projection, admission, attempt, outcome, reconciliation, denial, and
remaining-authority posture.

Live connector acceptance is separate and occurs only after fixture-backed
model, lifecycle, same-call admission, and reconciliation boundaries are
accepted.
