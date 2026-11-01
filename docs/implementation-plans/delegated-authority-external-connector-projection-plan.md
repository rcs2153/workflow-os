# Delegated Authority Projection Into External Connectors Plan

## Status

P0 planning requirement. This plan preempts broader provider mutation families,
nested harness work, and claims of continuous delegated-maintainer autonomy.
It extends existing scoped runtime authority, authoritative continuation, and
proportional-governance contracts; it does not create a parallel permission
system.

## Problem

A governed PR phase can hold current proof-backed authority for an exact PR
lifecycle while an external connector independently asks the human to authorize
an intermediate transition. Both systems fail closed, but lawful work stalls
because the connector cannot verify the kernel's scoped authority.

No typed capability projection currently binds kernel authority to a connector
action. An approval handoff can also name only an initial mutation such as draft
creation without enumerating later intended transitions such as ready-for-review
and squash merge.

## Invariant

A current scoped kernel authorization may project into an external connector
capability grant. The connector executes only listed transitions while the
exact resource, immutable head, evidence predicates, execution window,
delegation, expiry, revocation, and consumption constraints remain satisfied.

No additional human approval is required inside that approved scope. Missing,
stale, changed, incomplete, or expanded scope fails closed.

## Core Model

Define a typed, payload-free projection with:

- projection ID and version;
- governed run ID, approval ID, and approval-presentation proof reference;
- authority source and scoped capability grant reference;
- connector kind and exact repository/resource identity;
- optional exact PR identity when it exists;
- immutable approved head SHA;
- closed permitted-action set;
- required evidence predicates and evidence-reference posture;
- execution-window identity and validity bounds;
- actor, delegator, delegate, and delegation scope;
- issued-at, expires-at, revoked-at, and revocation reference;
- single-use or reusable-within-window consumption posture;
- consumed transition records and remaining transition set; and
- redaction and sensitivity metadata.

The projection contains no credential, provider payload, conversation token,
or broad repository permission.

## Pull Request Transition Vocabulary

The first closed action set is:

- `github.pull_request.create_draft`;
- `github.pull_request.update_exact_head`;
- `github.pull_request.mark_ready_after_terminal_green`;
- `github.pull_request.squash_merge_unchanged_head`; and
- `github.branch.delete_after_merge` only when separately listed.

Draft creation does not imply ready transition, merge, or branch deletion. A
phase intending the full lifecycle must present and approve every anticipated
transition.

## Connector Verification

Before every mutation the connector verifies:

1. the requested action is in the unconsumed permitted-action set;
2. repository, PR, branch, and head SHA match the projection exactly;
3. all action-specific evidence predicates are currently satisfied;
4. approval-presentation proof is valid and matches projected scope;
5. capability and execution window are current and unrevoked;
6. actor and delegation remain valid;
7. the action does not broaden target, payload class, or side effects; and
8. replay is an exact idempotent reconciliation or is rejected.

Connector/platform credentials remain a separate platform permission. Kernel
approval does not create credentials or bypass platform authorization.

## Evidence Predicates

- create draft: reviewed commit equals approved head and branch target matches;
- update exact head: update remains on the bound branch and exact approved SHA;
- mark ready: all named required checks are terminal green for the same head;
- squash merge: PR is ready, checks remain green, review and policy predicates
  remain satisfied, and head is unchanged; and
- delete branch: merge is terminal and deletion was separately authorized.

Evidence references are payload-free and reassessed immediately before the
mutation.

## Refusal And Continuation

A connector refusal is a first-class resumable wait condition, not agent
termination. Persist the attempted action, bounded refusal class, failed
predicate or missing platform permission, projection and remaining-authority
posture, safe next action, and whether automatic resumption is lawful.

The scheduler resumes automatically only when the wait condition is satisfied
and all authority facts are reassessed. Human restart is not required for a
connector-local transient refusal inside still-current authority.

## Reporting

WorkReports and inspect output distinguish authorized and still-available
transitions, consumed transitions, denied or out-of-scope transitions, expired
or revoked authority, connector/platform permission failures, and resumable
wait conditions. Documentation separately explains kernel approval, adapter
capability, and platform permission.

## Required Tests

- complete exact green PR lifecycle without a second human prompt;
- changed-head rejection at every later transition;
- missing, pending, failed, or stale required checks;
- expired and revoked grants;
- unlisted transition rejection;
- exact replay without duplicate draft, ready transition, or merge;
- connector refusal persisted as inspectable resumable wait;
- projection scope cannot broaden repository, PR, branch, or payload class;
- branch deletion requires separate authority; and
- reports classify consumed, denied, and remaining actions truthfully.

## Delivery Sequence

1. focused maintainer/security review of this plan;
2. model-only capability projection and closed PR transition vocabulary;
3. durable projection, consumption, revocation, and wait-condition records;
4. connector-side verifier with injected fixture transport only;
5. one end-to-end fixture-backed PR lifecycle proof;
6. separately governed integration with the existing GitHub connector; and
7. only then reconsider broader provider mutation or delegated-maintainer
   continuity claims.

## Non-Goals

- no weakening or bypass of connector safety;
- no browser automation as a connector-control bypass;
- no broad repository mutation authority;
- no unlimited standing delegation;
- no credential issuance or storage;
- no new provider mutation family in the model phase;
- no enterprise IdP, organization administration, or hosted control plane;
- no nested harness runtime; and
- no parallel authorization system.

## Acceptance

One governed PR phase completes every explicitly approved transition without a
second human authorization. Every external mutation is attributable to a
current scoped kernel capability and durable approval proof. Stale,
changed-head, evidence-incomplete, expired, revoked, and out-of-scope actions
fail closed. Connector refusal is durable and resumable. Reports expose exact
authorization, consumption, denial, and remaining-authority posture.
