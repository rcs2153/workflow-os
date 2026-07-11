# GitHub PR Comment Live Sandbox Validation Plan

Status: Accepted with non-blocking follow-ups in
[GitHub PR Comment Live Sandbox Validation Plan Review](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_PLAN_REVIEW.md).
The first local sandbox target proof model/helper slice is implemented in
[GitHub PR Comment Sandbox Target Proof Helper Report](../concepts/GITHUB_PR_COMMENT_SANDBOX_TARGET_PROOF_HELPER_REPORT.md).
The first explicit injected live sandbox validation helper is implemented in
[GitHub PR Comment Live Sandbox Validation Helper Report](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HELPER_REPORT.md).
The helper review is documented in
[GitHub PR Comment Live Sandbox Validation Helper Review](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HELPER_REVIEW.md).
Focused helper-specific test hardening is documented in
[GitHub PR Comment Live Sandbox Validation Hardening Report](../concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HARDENING_REPORT.md).

## 1. Executive Summary

Workflow OS now has the local provider-write foundations needed to plan a
single disposable live sandbox validation path for GitHub pull request comments.
The auth/source hardening phase confirmed that concrete provider and lookup
clients compare the full validated caller-supplied auth wrapper before
transport, and sandbox readiness denies hidden, ambient, or unknown auth
postures.

The next question is how to validate one live sandbox provider path without
weakening the current product contract. This plan defines that future validation
boundary.

This plan does not implement provider writes, run a live provider call, add
hidden auth loading, add CLI mutation behavior, add workflow schema fields,
update examples, add hosted behavior, broaden adapters, add automatic executor
writes, add report artifact writes, implement reasoning lineage, or change
release posture.

## 2. Goals

- Define one future, explicit, disposable GitHub PR comment live sandbox
  validation path.
- Keep the validation local, injected, caller-supplied, and non-default.
- Require explicit sandbox target proof before any provider transport.
- Require explicit caller-supplied auth and reject hidden or ambient auth.
- Preserve the rule that token possession is not write authority.
- Preserve current executor semantics and default write-denied behavior.
- Use existing provider-write, SideEffect, approval, event-proof, and report
  disclosure primitives rather than introducing a parallel write model.
- Ensure provider-call success and failure are observable through bounded,
  non-leaking references and event/report posture.
- Keep user-facing documentation honest that production write support is not
  implemented.

## 3. Non-Goals

This planning phase does not authorize:

- provider writes;
- live sandbox mutation;
- production mutation;
- hidden auth loading from environment, keychain, GitHub CLI, git credentials,
  browser sessions, config files, OAuth state, or secret managers;
- automatic executor provider writes;
- automatic report generation or artifact writes;
- CLI mutation commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- broad GitHub writes beyond a disposable PR comment sandbox;
- Jira, CI, filesystem, HTTP, or arbitrary provider writes;
- automatic retries, repair, or recovery mutation;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- reasoning lineage;
- release posture changes.

## 4. Implemented Preconditions

The future live sandbox validation path may rely on reviewed local primitives:

- GitHub PR comment write request and response models;
- explicit caller-supplied provider auth wrapper;
- injected provider-call trait;
- concrete injected-transport GitHub PR comment provider client;
- provider lookup HTTP client;
- full validated auth wrapper matching before provider or lookup transport;
- SideEffect proposal, attempted, completed, and failed lifecycle helpers;
- approval-side-effect linkage;
- approval presentation proof;
- provider event-proof gate;
- provider lookup/recovery helpers;
- provider reconciliation helpers;
- report disclosure and artifact integrity helper paths;
- sandbox readiness helper that denies hidden, ambient, or unknown auth posture.

These foundations remain explicit and opt-in. They do not make writes default,
automatic, CLI-facing, schema-declared, or production-ready.

## 5. Sandbox Target Proof

A future disposable live sandbox validation must require a bounded target proof
before transport.

For the GitHub PR comment lane, target proof should include:

- repository owner;
- repository name;
- pull request number;
- sandbox classification, such as disposable, test, preview, or maintainer
  sandbox;
- statement that the target is not production-like;
- expected capability, limited to GitHub PR comment creation;
- intended actor or system actor;
- correlation ID;
- idempotency key;
- sensitivity;
- redaction metadata;
- optional evidence reference to the maintainer-approved sandbox setup.

Target proof must not include:

- raw pull request body text;
- raw issue or review comments;
- repository file contents;
- provider tokens;
- command output;
- CI logs;
- browser session state;
- secret-like values.

Production-looking, unknown, ambiguous, or unclassified targets must fail before
provider transport.

## 6. Auth Source And Authority Policy

Auth material must remain explicit caller input.

The future validation path must not read credentials from:

- environment variables;
- shell profiles;
- keychains;
- GitHub CLI state;
- git credential helpers;
- git remotes;
- repository config;
- OAuth state;
- browser sessions;
- secret managers.

Auth material must not be serialized, debug-formatted, copied into errors,
stored in workflow events, stored in SideEffect records, included in WorkReport
content, written to report artifacts, or emitted through CLI output.

Possessing a token is not authority. A future sandbox validation path must still
require the configured authority signals:

- supported provider capability;
- explicit sandbox target proof;
- policy allowance;
- SideEffect proposal and attempted lifecycle posture;
- approval-side-effect linkage when required;
- approval-presentation proof when required;
- high-assurance approval posture when configured;
- event-proof and report/artifact policy when configured.

Missing, stale, ambiguous, or unsupported authority signals must block transport.

## 7. Future Validation Trigger

The first implementation should be explicit and non-default.

Recommended trigger shape:

- an internal test/helper path, not automatic executor behavior;
- explicit caller-supplied provider client or transport;
- explicit caller-supplied auth wrapper;
- explicit sandbox target proof;
- explicit opt-in flag for integration testing if a real network call is
  exercised;
- no CLI mutation command in the first implementation.

An opt-in environment variable may gate an ignored integration test, but it must
not supply auth or target state. Auth and target proof must still be passed
through validated input.

## 8. Provider Call Flow

A future validation implementation should follow this sequence:

1. Validate explicit sandbox target proof.
2. Validate explicit caller-supplied auth posture.
3. Validate policy and authority signals.
4. Validate SideEffect proposal and attempted lifecycle state.
5. Validate approval linkage and approval-presentation proof when required.
6. Validate idempotency and correlation binding.
7. Invoke only the injected provider boundary.
8. Transition attempted SideEffect lifecycle to completed or failed from the
   classified provider result.
9. Return bounded event-proof and report-disclosure obligations.
10. Leave workflow event append and artifact writes to separately reviewed
    explicit boundaries.

The validation path must not mutate `WorkflowRun`, append workflow events by
default, write report artifacts, persist provider payloads, or emit CLI output.

## 9. Failure Behavior

Failure must be conservative and non-leaking.

The future validation path should fail before transport when:

- sandbox target proof is missing;
- target classification is ambiguous;
- target appears production-like;
- auth posture is hidden, ambient, unknown, missing, or mismatched;
- required policy allowance is absent;
- required approval linkage is absent;
- required approval-presentation proof is absent or stale;
- attempted SideEffect lifecycle state is missing or invalid;
- idempotency binding is missing or invalid;
- provider capability is unsupported.

Provider transport failures should be classified with stable reason codes and
bounded references. Raw provider error payloads, headers, tokens, snippets,
paths, command output, and secret-like values must not be copied.

Ambiguous provider/local outcomes must not trigger automatic retry or repair
mutation. They should produce explicit recovery posture for a later reviewed
lookup or reconciliation path.

## 10. Privacy And Redaction

The future validation path must preserve the existing privacy posture:

- no raw provider payloads;
- no raw PR bodies;
- no raw issue comments;
- no repository file contents;
- no CI logs;
- no command output;
- no parser payloads;
- no environment variable values;
- no credentials;
- no authorization headers;
- no private keys;
- no token-like values;
- no browser/session state.

Debug, Display, serialization, deserialization, and error paths must remain
redaction-safe.

Reports and report artifacts, if later attached, should cite stable references
and bounded summaries only. They must not copy provider payloads.

## 11. Test Plan For Future Implementation

Future tests should cover:

- explicit sandbox target proof is required;
- production-like target proof is rejected before transport;
- unknown target proof is rejected or deferred before transport;
- hidden auth posture is denied;
- ambient auth posture is denied;
- unknown auth posture is denied;
- auth mismatch fails before transport;
- full auth wrapper matching remains enforced;
- missing policy allowance fails before transport;
- missing approval linkage fails before transport when required;
- missing or stale approval-presentation proof fails before transport when
  required;
- missing attempted SideEffect lifecycle state fails before transport;
- missing idempotency binding fails before transport;
- provider success stores only bounded provider references;
- provider failure stores only stable failure codes and bounded references;
- provider/local ambiguity does not retry automatically;
- no workflow events are appended unless an explicit append helper is invoked;
- no report artifacts are written unless an explicit artifact helper is invoked;
- no CLI mutation command is introduced;
- errors and Debug output do not leak tokens, paths, provider payloads, or
  secret-like values;
- default executor paths remain write-denied.

If a real integration test is added later, it should be ignored by default and
require explicit maintainer opt-in plus disposable target proof.

## 12. Documentation And Product Contract

Documentation must continue to state:

- live sandbox validation is planned, not implemented by this plan;
- production provider writes are not implemented;
- hidden auth loading is not implemented;
- automatic executor writes are not implemented;
- CLI mutation commands are not implemented;
- workflow schema fields are not implemented;
- examples are not updated;
- hosted behavior is not implemented;
- broad write-capable adapters are not implemented;
- reasoning lineage is not implemented;
- release posture is unchanged.

The user-facing promise remains: Workflow OS governs work by making evidence,
policy, approvals, side effects, and reports inspectable. Live provider writes
must remain subordinate to that governance boundary.

## 13. Proposed Implementation Sequence

1. Review this live sandbox validation plan.
2. Add a small explicit sandbox target proof model/helper if the review accepts
   the boundary.
3. Add a non-default, injected live sandbox validation helper that validates
   target proof, auth posture, authority signals, SideEffect lifecycle, and
   idempotency before transport.
4. Add focused non-network tests with injected provider transport.
5. Add an ignored, maintainer-only disposable integration test only after the
   helper review passes.
6. Review the live sandbox validation helper before any CLI or executor-facing
   expansion.

Do not start with CLI mutation behavior or automatic executor integration.

## 14. Open Questions

- What is the smallest acceptable proof that a GitHub PR target is disposable?
- Should approval-presentation proof be mandatory for all live sandbox provider
  validation, even when the target is disposable?
- Should a sandbox target proof include repository visibility or owner
  relationship, or is that too close to hidden provider lookup?
- Should the first live validation use a real GitHub PR in this repository or a
  separate disposable repository?
- Should ignored integration tests live in `workflow-core` or a separate
  adapter-validation harness?
- How should duplicate comments be cleaned up after a maintainer-run sandbox
  validation without adding automatic deletion behavior?

## 15. Final Recommendation

Proceed next with a maintainer review of this plan.

Do not implement provider writes, live sandbox mutation, hidden auth loading,
CLI mutation commands, schemas, examples, hosted behavior, broad adapters,
automatic executor writes, report artifact writes, reasoning lineage, or release
posture changes before that review accepts the boundary.
