# External Action Authorization Projection Model Review

## 1. Decision

Accepted after four security corrections.

The corrected model is a bounded, immutable, payload-free claim describing one
closed GitHub pull-request action set. It remains non-authorizing, non-persistent,
and no-write. It is ready to support planning and implementation of durable
transition lifecycle state, but it does not admit connector execution.

## 2. Reviewed Boundary

The review covered:

- exact workflow, run, approval, presentation, capability, policy, execution
  window, immutable bundle, actor, delegation, expiry, revocation, and
  sensitivity references;
- GitHub repository, base, branch, and reviewed-head binding;
- closed transition and payload-class vocabulary;
- exact evidence predicate requirements;
- approval-presentation action-scope binding;
- transition dependency rules;
- deterministic canonicalization and commitments;
- draft-creation successor PR binding;
- deserialization revalidation;
- diagnostics, redaction, and tests; and
- explicit non-goals and next-phase boundaries.

## 3. Blocking Findings Corrected

### Approval proof and action scope were only adjacent commitments

The initial model committed the approval-presentation content hash and the
external action set inside the same projection, but it did not expose a
canonical action-scope commitment that presentation handling could carry and a
future current-authority verifier could compare. It also omitted a strict
non-goals commitment from that action scope.

The corrected model adds a canonical
`compute_external_action_approval_scope_commitment(...)` contract over exact
connector, account, resource, transitions, evidence predicates, payload
commitments, and strict non-goals. Projection construction requires the claimed
approval-presentation scope commitment to match that canonical value.

This remains a claim-only model. It does not prove presentation, persistence,
freshness, approval, or current authority. A future Core admission boundary must
load the persisted presentation proof and require its typed scope binding to
match before issuing same-call connector authority.

### Squash merge did not require the ready transition

Draft creation was required for later PR transitions, but squash merge could be
listed without the explicit ready transition. Because the closed lifecycle
always creates a draft, that would permit a transition set that could not lawfully
reach merge readiness.

The corrected validator requires
`github_pull_request_mark_ready_after_terminal_green` whenever
`github_pull_request_squash_merge_unchanged_head` is listed. Branch deletion
continues to require squash merge separately.

### Resource identifiers admitted ambiguous GitHub names

Repository and ref validation previously accepted whitespace and several
characters rejected by Git/GitHub reference rules. Exact resource binding is not
meaningful if ambiguous or invalid identifiers enter the commitment.

The corrected model constrains repository owner/name syntax, rejects whitespace
and invalid ref characters, rejects leading dash, dot components, `.lock`
components, `@`, traversal-like patterns, and malformed separators, and rejects
whitespace in connector account references.

### Successor observations were not time-bound to authority

A successor PR binding could be derived with an observation timestamp before
the projection's not-before time or after expiry. Although successor bindings
remain non-authorizing, accepting an out-of-window provider identity would
weaken later lifecycle composition.

The corrected derivation rejects successor observations outside the exact
authorized window.

## 4. Security Properties Preserved

- A serialized projection or successor binding is never bearer authority.
- Projection commitments cover all immutable references, exact resource,
  transitions, evidence requirements, payload commitments, strict non-goals,
  actors, time bounds, revocation source, and presentation scope.
- Equivalent unordered transition/evidence inputs canonicalize identically.
- Scope, resource, head, payload, evidence, non-goal, actor, and successor
  substitution changes a commitment or fails validation.
- Draft creation does not imply ready, merge, ref update, or branch deletion.
- Branch deletion remains separately authorized and merge-dependent.
- Debug and validation errors do not expose repositories, refs, SHAs,
  commitments, provider PR identity, or caller-controlled rejected values.

## 5. Important Non-Claim

The approval-scope helper does not prove that a human saw or approved the scope.
It defines the exact canonical value that future presentation persistence and
Core-owned connector admission must compare. Runtime work must fail closed when
the persisted presentation lacks this typed binding, is stale, or differs.

The model does not reload authority, evaluate evidence, reserve a transition,
issue a same-call capability, call a connector, reconcile provider outcomes, or
resume waits.

## 6. Validation

- focused external-action model tests: 10 passed;
- `cargo fmt --all --check`: required and passed at phase close;
- warnings-denied all-target/all-feature `workflow-core` Clippy: required and
  passed at phase close;
- complete all-feature `workflow-core` library tests with
  `--test-threads=1`: 631 passed;
- `npm run check:docs`: required and passed at phase close; and
- `git diff --check`: required and passed at phase close.

Default-parallel all-feature library runs exposed a pre-existing trusted-host
timer-test reliability defect. One run failed three timer-family tests and a
second run failed a different timer-family test; all affected focused tests
passed in isolation, and the complete serial 631-test run passed. The review
found no coupling from the new model into that existing SQLite/timer path. This
is disclosed as an unresolved repository validation reliability issue rather
than hidden behind the successful serial run. It does not change the security
verdict for this isolated model, but should receive a separately governed
blocker-fix phase before relying on parallel full-suite runs as deterministic
evidence.

## 7. Governance Record

- workflow: `dg/review`
- run: `run-1793612798061739000-2`
- approval: `approval/run-1793612798061739000-2/review-scope-approved`
- presentation proof: `presentation/87b556a0d5a2c014`
- approval outcome: granted under delegated maintainer authority
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations
- approval-presentation enforcement: proof enforced with one persisted record
  and event marker
- approved scope: model security review and blocker correction only
- out-of-kernel work: code inspection, edits, tests, docs, git operations, and
  later PR actions remain executor work

The kernel coordinated phase governance and persisted the approval and event
trail. It did not execute source edits, tests, documentation updates, git
operations, or provider actions.

## 8. Accepted Next Phase

Implement the model and local durable state for transition availability,
reservation, attempt, success, denial, wait, ambiguity, expiry, revocation, and
consumption, composed with authorized-execution continuity.

Do not add connector execution, credentials, live GitHub behavior, provider
writes, same-call connector capability, workflow schemas, CLI mutation,
scheduler claims, or broader provider mutation families in that phase.
