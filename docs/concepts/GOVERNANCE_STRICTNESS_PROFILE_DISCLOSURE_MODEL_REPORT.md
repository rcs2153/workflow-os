# Governance Strictness Profile Disclosure Model Report

## 1. Executive Summary

The first governance strictness profile implementation slice is complete.

Workflow OS now has typed core vocabulary for local governance strictness
profiles and a bounded first-run disclosure that uses that vocabulary for the
current `observe_and_report` posture. This makes the single-user speed versus
future enterprise stewardship separation a product contract instead of a pair
of CLI string literals.

## 2. Scope Completed

- Added `GovernanceStrictnessProfile`.
- Added `GovernanceProfilePosture`.
- Added `GovernanceProfileDisclosure`.
- Exported the profile types from `workflow-core`.
- Wired `workflow-os first-run` human and JSON output through the typed
  disclosure.
- Updated the governance strictness profile plan, current product contract,
  and roadmap.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- executor approval behavior changes;
- admin controls;
- enterprise stewardship enforcement;
- RBAC;
- IdP integration;
- hosted policy enforcement;
- workflow schema changes;
- automatic approval behavior changes;
- write-capable adapters;
- CLI mutation behavior;
- examples;
- release posture changes.

## 4. Model Summary

The initial profile vocabulary is:

- `observe_and_report`;
- `agent_assisted_gated`;
- `human_approval_gated`;
- `strict_enterprise`.

Only `observe_and_report` is used as the current local default disclosure. Its
posture is `disclosed_not_enforced`, meaning the profile is reported for
operator clarity but does not enforce profile-specific approval or admin
behavior.

## 5. First-Run Behavior

`workflow-os first-run` continues to emit:

- `governance_profile: observe_and_report`;
- `profile_posture: disclosed_not_enforced`.

The output is behaviorally unchanged for users, but the labels now come from
the core profile model.

## 6. Privacy And Redaction

The model stores only static vocabulary labels. It does not store user text,
paths, commands, provider payloads, source contents, environment values,
credentials, tokens, authorization headers, private keys, or secret-like
values.

## 7. Validation

Validation run for this phase:

- `cargo test -p workflow-core governance_profile` - passed.
- `cargo test -p workflow-cli --test cli first_run_verbose_outputs_full_posture_matrix` - passed.
- `cargo test -p workflow-cli --test cli first_run_json_is_bounded_and_report_ready` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed after the
  blocker fix removed identical match arms from `is_currently_enforced`.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 8. Remaining Known Limitations

- Governance profiles are disclosure vocabulary only in this slice.
- No workflow-declared profile selection exists.
- No enterprise stewardship store exists.
- No admin-controlled profile enforcement exists.
- Existing approval behavior remains governed by current workflow policy and
  executor paths, not by the profile model.

## 9. Recommended Next Phase

Recommended next phase: governance strictness profile disclosure model review.

Reason: profile vocabulary is a product-boundary concept. Review should verify
that the implementation keeps local automation fast, does not overclaim
enterprise stewardship, and preserves executor approval semantics.

## 10. Dogfood Governance

This implementation phase was governed by the local Workflow OS dogfood runner.

- workflow phase: implementation
- workflow ID: `dg/implement`
- run ID: `run-1783770121317303000-2`
- approval ID: `approval/run-1783770121317303000-2/implementation-approved`
- approval presentation ID: `presentation/7c6187dd31832063`
- approval presentation hash: `7c6187dd318320633df5eb2e67f1931b0229507c9aa8ce6762643d69d521e278`
- approval outcome: approved by delegated maintainer before implementation
- close status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- blocker fix workflow: `dg/blocker`
- blocker fix run ID: `run-1783771525521082000-2`
- blocker fixed: GitHub Rust CI clippy `match_same_arms` failure in
  `GovernanceStrictnessProfile::is_currently_enforced`
- out-of-kernel work: repository edits, tests, docs updates, and validation
  commands were performed by the executor under the governed phase boundary
