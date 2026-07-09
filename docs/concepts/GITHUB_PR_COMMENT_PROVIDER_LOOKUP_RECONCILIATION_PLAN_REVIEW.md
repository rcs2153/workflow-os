# GitHub PR Comment Provider Lookup/Query Reconciliation Plan Review

## 1. Executive Verdict

Plan accepted; proceed to GitHub PR comment provider lookup/query reconciliation model/helper implementation.

The plan defines a conservative, explicit, injected-client lookup/query reconciliation boundary. It keeps provider lookup separate from event proof, state repair, report artifact writes, CLI behavior, schemas, examples, hosted runtime, reasoning lineage, and release posture changes.

No planning blockers were found.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- implementation in the planning phase;
- automatic provider lookup;
- hidden auth loading;
- live provider calls by default;
- GitHub comment creation, update, or deletion;
- automatic retries;
- workflow event append;
- workflow event repair;
- side-effect record mutation;
- report artifact writes;
- automatic report generation;
- default executor behavior changes;
- CLI behavior;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- broad write-capable adapter expansion;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- approval-presentation enforcement;
- release posture changes.

## 3. Boundary Assessment

The plan correctly identifies the next boundary after event-proof recovery classification.

The proposed helper should answer bounded provider-side observation questions:

- whether a matching provider-side comment was observed;
- whether no matching comment was observed;
- whether lookup was ambiguous, unavailable, unauthorized, rate-limited, invalid, or untrusted.

It does not allow that observation to become durable workflow event proof.

## 4. Source-Of-Truth Assessment

The plan preserves the right source-of-truth hierarchy.

Verified:

- workflow event log remains durable provider event proof;
- side-effect store remains local lifecycle state;
- provider lookup result is remote observation only;
- recovery result is operator guidance only;
- WorkReport is a handoff/reporting surface, not an event log;
- artifact gate remains the artifact-write enforcement boundary.

This distinction is essential. Provider lookup can inform a later repair or operator decision, but it must not make a report artifact claim that Workflow OS recorded an event when no such event exists.

## 5. Provider Client Boundary Assessment

The plan uses the right provider boundary for this stage.

Verified:

- caller-supplied auth/config is required;
- injected transport or provider client is required;
- hidden environment lookup is deferred;
- default live network behavior is not authorized;
- fixture/fake client testing is expected;
- opt-in live smoke tests remain separately scoped.

This aligns with the existing GitHub PR comment provider client/auth and injected transport posture.

## 6. Matching Policy Assessment

The matching policy is appropriately conservative.

Allowed matching signals are deterministic:

- exact provider comment ID/reference;
- exact pull request target identity;
- intentionally bounded provider-safe managed marker.

Rejected matching signals are correctly excluded:

- raw comment body matching;
- fuzzy text matching;
- model interpretation;
- repository path inference;
- unbounded natural-language summaries;
- matching on secrets, tokens, or private payloads.

The plan's fail-closed `remote_comment_ambiguous` and `lookup_response_untrusted` postures are important and should be implemented before any operator-facing recovery output expands.

## 7. Lookup Posture Assessment

The planned posture vocabulary is sufficient for a first implementation:

- `remote_comment_observed`;
- `remote_comment_absent`;
- `remote_comment_ambiguous`;
- `lookup_not_authorized`;
- `lookup_unavailable`;
- `lookup_rate_limited`;
- `lookup_target_invalid`;
- `lookup_response_untrusted`.

The vocabulary is stable, bounded, and specific enough to drive later WorkReport, CLI, repair, or artifact planning without using free-form string interpretation.

## 8. Error-Handling Assessment

The planned error codes are stable and non-leaking:

- `github_pr_comment_provider_lookup_reconciliation.invalid_input`;
- `github_pr_comment_provider_lookup_reconciliation.target_invalid`;
- `github_pr_comment_provider_lookup_reconciliation.auth_denied`;
- `github_pr_comment_provider_lookup_reconciliation.lookup_unavailable`;
- `github_pr_comment_provider_lookup_reconciliation.rate_limited`;
- `github_pr_comment_provider_lookup_reconciliation.response_untrusted`;
- `github_pr_comment_provider_lookup_reconciliation.redaction_invalid`.

The plan explicitly forbids raw GitHub responses, comment bodies, PR data, auth headers, tokens, environment values, CI logs, command output, raw specs, parser payloads, private URLs, and secret-like values in errors.

## 9. Privacy And Redaction Assessment

The privacy posture is appropriate.

Allowed:

- stable provider comment reference;
- stable lookup posture;
- bounded candidate counts;
- bounded reason and next-action codes;
- validated redaction metadata.

Forbidden:

- comment bodies;
- raw provider payloads;
- PR bodies, diffs, files, or review text;
- tokens, credentials, or auth headers;
- environment values;
- command output;
- CI logs;
- raw specs;
- parser payloads;
- private paths or URLs;
- unbounded operator notes.

The future implementation must keep `Debug`, display, serialization, and deserialization errors redaction-safe.

## 10. Relationship Assessment

The plan correctly separates lookup/query reconciliation from adjacent phases.

Event-proof recovery:

- recovery classification says lookup may be needed;
- lookup reconciliation observes provider state;
- neither creates durable event proof.

Manual repair:

- deferred;
- must later define authority, evidence, repair shape, audit posture, and duplicate-write prevention.

Artifact writes:

- still blocked without durable workflow event proof;
- provider lookup alone should not satisfy strict artifact gates.

CLI:

- deferred until model/helper and review exist.

## 11. Test Plan Assessment

The future test plan is strong enough for implementation.

It covers:

- observed remote comment by exact reference;
- observed remote comment by bounded managed marker;
- absent remote comment;
- ambiguous multiple matches;
- unauthorized lookup;
- unavailable lookup;
- rate-limited lookup;
- invalid target before lookup;
- untrusted response;
- secret-like input rejection;
- no raw comment body copying;
- no raw provider response copying;
- retry remains blocked for ambiguous or split-brain states;
- artifact write remains blocked without durable event proof;
- no event append;
- no side-effect record mutation;
- no artifact writes;
- no CLI output;
- redaction-safe debug and serialization;
- existing adjacent test suites.

Non-blocking implementation note: include an explicit test proving a successful remote observation still does not set `artifact_write_may_proceed` when durable workflow event proof is missing.

## 12. Documentation Review

The documentation is clear and honest.

Verified:

- plan states lookup/query reconciliation is planned, not implemented;
- plan states provider calls are not automatic;
- plan states writes, retries, event append, repair, artifacts, CLI, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, and release posture are not implemented;
- roadmap now links the lookup reconciliation plan and plan report.

## 13. Planning Blockers

None.

## 14. Non-Blocking Follow-Ups

- During implementation, consider whether lookup result should include a bounded candidate count even when ambiguous, without exposing comment bodies or IDs beyond allowed references.
- During implementation, test that remote observation cannot satisfy strict artifact gates without separate event proof.
- After implementation and review, plan manual state repair only if lookup result semantics are stable.

## 15. Recommended Next Phase

Recommended next phase: GitHub PR comment provider lookup/query reconciliation model/helper implementation, explicit injected client only.

The implementation should add vocabulary, explicit input/result types, an injected lookup client or compatible transport boundary, fixture/fake-client tests, and an implementation report. It must not implement automatic provider lookup, hidden auth loading, provider writes, retries, workflow event append, state repair, artifact write composition, CLI behavior, schemas, examples, hosted behavior, broader write adapters, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 16. Validation

Validation run:

- `npm run check:docs`: passed.

## 17. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run: `run-1783560061011575000-2`.
- Approval: `approval/run-1783560061011575000-2/review-scope-approved`.
- Approval outcome: granted under delegated maintainer authority after the complete approval handoff block was emitted and preserved.
