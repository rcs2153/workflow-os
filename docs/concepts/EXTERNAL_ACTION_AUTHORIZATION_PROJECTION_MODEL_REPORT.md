# External Action Authorization Projection Model Report

## 1. Executive Summary

The first model-only slice for projecting delegated Workflow OS authority into
external connector actions is implemented.

Workflow OS now has an immutable, payload-free authorization projection model
for a closed GitHub pull-request lifecycle. The model binds governed approval,
approval-presentation proof, scoped capability and execution-window references,
an exact repository and reviewed head, canonical transitions, transition-specific
payload commitments, exact evidence predicates, delegation, time bounds,
revocation references, and reporting posture into one deterministic commitment.

This slice remains non-authorizing and model-only. A serialized projection or
successor binding cannot admit a connector call. No connector execution,
credentials, provider calls, GitHub writes, durable transition lifecycle,
same-call use capability, workflow schema, CLI mutation behavior, scheduler, or
automatic continuation was added.

## 2. Scope Completed

- Added `ExternalActionAuthorizationProjection` and validated definition types.
- Added a closed GitHub transition vocabulary for draft creation, exact ref
  fast-forward, ready transition, unchanged-head squash merge, and separately
  authorized branch deletion.
- Added exact, payload-free GitHub repository, base, branch, and reviewed-head
  resource binding.
- Added a canonical approval-presentation action-scope commitment over exact
  connector, resource, transitions, evidence, payload, and strict non-goals.
- Added transition-specific payload-class commitments and source-bound,
  subject-bound, freshness-bound evidence predicates.
- Added canonical transition and evidence ordering, duplicate rejection, and
  transition dependency validation.
- Added deterministic projection commitments and projection identifiers.
- Added derived successor PR binding for the provider identity returned after
  draft creation.
- Added deterministic successor-binding commitments and identifiers.
- Added fail-closed deserialization that re-enters validation for nested models,
  projections, and successor bindings.
- Added redaction-safe debug and stable, payload-free error behavior.
- Added focused tests covering canonicalization, substitution, tampering,
  evidence completeness, presented-scope binding, transition dependencies,
  resource syntax, successor identity and timing, serde validation, and
  redaction.

## 3. Closed Transition Vocabulary

The first action set is deliberately closed:

- `github_pull_request_create_draft`;
- `github_git_ref_fast_forward_exact`;
- `github_pull_request_mark_ready_after_terminal_green`;
- `github_pull_request_squash_merge_unchanged_head`; and
- `github_branch_delete_after_merge`.

Draft creation does not imply any later transition. Ref updates remain distinct
from PR transitions. Branch deletion requires its own listed transition and the
merge dependency.

## 4. Authority Boundary

The projection is an immutable claim and commitment, not bearer authority.

It contains references to governed approval, approval presentation, capability,
policy, execution window, immutable run bundle, revocation source, actor,
delegator, and delegate. It does not resolve their current truth. It does not
carry credentials, provider payloads, raw approval prose, raw evidence, access
material, or a reusable connector capability.

A future Core-private admission boundary must reload current authority and
evidence, reserve the exact transition atomically, and issue a one-use same-call
capability. This phase does not implement or simulate that boundary.

Focused review corrected the presentation boundary so the projection requires
an exact canonical action-scope commitment covering connector, account,
resource, transitions, evidence, payload, and strict non-goals. The helper and
stored value still do not prove human presentation or approval. Future Core
admission must compare them with a current persisted presentation proof.

## 5. Resource And Successor Binding

Before draft creation, the projection binds the exact repository, base ref,
base commitment, head ref, and 40-character lowercase reviewed Git commit.

After draft creation, `derive_external_action_successor_binding(...)` derives a
payload-free binding for the exact provider PR identity returned by the
connector. The successor commitment includes the projection, repository,
provider PR identity, and create-transition operation commitment. Later model
phases can require that exact binding rather than accepting a caller-substituted
PR number.

The derived successor remains non-authorizing and is not durable in this phase.

## 6. Validation And Commitment Summary

Construction and deserialization fail closed for:

- malformed identifiers and references;
- invalid GitHub repository, ref, and reviewed-head shapes;
- empty, excessive, duplicate, or dependency-incomplete transition sets;
- duplicate, missing, excessive, or stale-bound evidence declarations;
- payload-class and transition mismatches;
- actor/delegate mismatch;
- invalid execution and projection time bounds;
- projection commitment or identifier tampering;
- successor repository, projection, PR identity, operation, commitment, or ID
  substitution; and
- unknown serialized fields.

Canonical ordering makes equivalent transition and evidence sets produce the
same projection commitment. Semantic changes produce different commitments.

## 7. Redaction And Privacy Summary

Debug output redacts repository commitments, payload commitments, evidence
commitments, projection commitments, and successor commitments. Validation
errors use bounded classifications and do not echo caller-controlled values.

The model stores no credentials, authorization headers, tokens, private keys,
provider payloads, raw evidence, approval prose, command output, logs, source
contents, or local filesystem paths.

## 8. Scope Explicitly Not Completed

This phase does not implement:

- durable `ExternalActionAuthorizationState` lifecycle;
- transition reservation, consumption, revocation, wait, or ambiguity records;
- current-authority or current-evidence resolution;
- a Core-owned same-call connector capability;
- connector admission or execution;
- GitHub API calls or provider mutation;
- platform credential discovery, loading, storage, or validation;
- provider result reconciliation;
- event, audit, SideEffect, or WorkReport projection;
- workflow YAML, schemas, TypeScript SDK, or CLI mutation surfaces;
- scheduler, supervisor, automatic resumption, or hosted behavior; or
- broader provider mutation families.

## 9. Validation Performed

- `cargo fmt --all --check` - passed.
- Focused external-action authorization unit tests - 10 passed after review
  corrections.
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings` -
  passed.
- `cargo test -p workflow-core --all-features --lib` - 629 passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

An earlier full `workflow-core` run passed 628 tests and reported one failure in
the pre-existing local timer duplicate-wake test: it observed
`trusted_host_supervisor.yield_not_committed` where the test expected
`trusted_host_time_window_caller.operation_replay_conflict`. The failing test
passed immediately in isolation, and the complete 629-test library rerun passed.
No authority-model code touches that timer path. This remains disclosed as an
order/timing-sensitive residual test risk rather than being represented as a
deterministic regression or silently omitted.

Focused review added two model tests. After those corrections, 10 focused tests
passed, warnings-denied Clippy passed, and the complete 631-test library suite
passed serially. Two default-parallel runs exposed the same existing timer-path
reliability family with three and one failures respectively; all affected tests
passed in isolation. See the [focused
review](EXTERNAL_ACTION_AUTHORIZATION_PROJECTION_MODEL_REVIEW.md) for the exact
validation posture.

Documentation and diff validation are completed at governed phase close and
recorded in the phase report.

## 10. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1793576143899106000-2`
- approval: `approval/run-1793576143899106000-2/implementation-approved`
- approval presentation: `presentation/1c48c956abf843e7`
- approval outcome: granted by delegated maintainer
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations
- approval-presentation enforcement: proof enforced with one persisted
  presentation record and event marker
- approved scope: model-only immutable projection, closed PR transitions,
  exact resource and payload commitments, successor binding, canonicalization,
  deterministic commitment, validation, tests, report, and roadmap update
- out-of-kernel work: source editing, test execution, documentation editing,
  and git operations were performed by the agent execution layer

The kernel coordinated phase governance and persisted the approval and event
trail. It did not execute repository edits, tests, documentation updates, git
operations, or provider actions.

## 11. Recommended Next Phase

Focused maintainer/security review accepted the corrected model. See the
[review](EXTERNAL_ACTION_AUTHORIZATION_PROJECTION_MODEL_REVIEW.md).

Recommended next phase: **durable external-action transition lifecycle model and
local state composition**.

That phase should add availability, reservation, attempt, success, denial,
wait, ambiguity, expiry, revocation, and consumption state composed with
authorized-execution continuity. It must not add connector execution,
credentials, live GitHub behavior, provider writes, or scheduler claims.
