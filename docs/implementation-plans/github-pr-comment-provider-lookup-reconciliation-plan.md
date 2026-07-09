# GitHub PR Comment Provider Lookup/Query Reconciliation Plan

Status: Implemented and reviewed for the first explicit model/helper slice. This follows the accepted [GitHub PR Comment Provider Lookup/Query Reconciliation Plan Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_PLAN_REVIEW.md), is reported in [GitHub PR Comment Provider Lookup Reconciliation Model Report](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REPORT.md), and is accepted in [GitHub PR Comment Provider Lookup Reconciliation Model Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REVIEW.md).

The implemented slice adds an explicit injected-client lookup reconciliation helper and bounded response/result model. It does not implement automatic provider lookup, hidden auth loading, provider writes, workflow event append, state repair, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 1. Executive Summary

Workflow OS can now classify GitHub PR comment provider event-proof recovery posture when local provider disclosure and durable workflow event proof do not line up.

The next question is how a future implementation should perform explicit provider lookup/query reconciliation when a provider outcome is ambiguous, event proof is missing, or local state cannot prove what happened.

This plan defines a narrow lookup/query reconciliation boundary. The first implementation should be an explicit helper with caller-supplied auth and injected transport. It should gather bounded provider-side observation and produce a reconciliation result. It must not write comments, retry mutations, append workflow events, repair local state, write report artifacts, expose CLI behavior, or change workflow semantics.

## 2. Goals

- Define a conservative provider lookup/query reconciliation boundary.
- Preserve workflow events as the durable event-proof source.
- Preserve provider lookup results as bounded observations, not proof of local event append.
- Support explicit lookup after ambiguous provider/local states.
- Help operators distinguish "remote comment exists" from "Workflow OS has durable event proof."
- Prevent accidental duplicate provider writes after ambiguous outcomes.
- Prepare for future manual repair and artifact-write composition without implementing either.
- Keep all errors, summaries, and debug/serialization paths redaction-safe.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
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
- automatic report artifact writing;
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

## 4. Current Baseline

Implemented and reviewed:

- GitHub PR comment provider-call trait/input model.
- Concrete injected-transport GitHub PR comment provider client.
- Provider write reconciliation candidate model/helper.
- Executor-integrated live provider-write helper with explicit supplied provider.
- Provider write event append helper for eligible reconciled completed/failed outcomes.
- Provider reconciliation disclosure projection into in-memory WorkReports.
- Strict report artifact event-proof gates.
- Denied-posture gate matrix hardening.
- Local event-proof recovery classification helper.

Implemented after this plan:

- explicit injected-client provider lookup/query reconciliation model/helper;

Still not implemented:

- automatic provider lookup;
- automatic recovery workflow;
- automatic event repair;
- manual state repair helper;
- CLI recovery command;
- schema-declared recovery policy.

## 5. Problem Statement

After a provider write attempt, Workflow OS can observe split-brain or ambiguous states:

- provider response was ambiguous;
- provider call may have succeeded but local lifecycle transition failed;
- provider/local disclosure says a comment may exist, but durable event proof is missing;
- a strict artifact gate denies artifact write because event proof is missing;
- recovery classification says manual provider lookup is required.

Without a lookup/query reconciliation boundary, the operator has a safe denial but no bounded way to answer whether the remote comment exists.

The next layer should answer only:

- was a matching provider-side comment observed;
- was no matching provider-side comment observed;
- was provider-side lookup unavailable, denied, ambiguous, or unsafe;
- what bounded next action should follow.

It must not convert observation into durable workflow event proof by itself.

## 6. Source-Of-Truth Boundaries

| Surface | Role |
| --- | --- |
| Workflow event log | Durable proof that Workflow OS recorded a provider write lifecycle event. |
| SideEffectRecordStore | Local side-effect lifecycle state. |
| Provider lookup/query result | Bounded remote observation. |
| Provider write reconciliation candidate | Provider/local reconciliation posture. |
| Event-proof recovery result | Local recovery posture and next-action guidance. |
| WorkReport | Handoff/reporting surface, not event proof. |
| Report artifact gate | Artifact write enforcement boundary. |

Provider lookup must not become a substitute for workflow event proof. It may support later operator decisions or a separately approved repair path.

## 7. Recommended First Implementation Target

Implement a small explicit lookup/query reconciliation helper.

Candidate shape:

```text
reconcile_github_pr_comment_provider_lookup(...)
```

or equivalent repository-named API that accepts:

- expected GitHub PR comment target;
- expected side-effect ID;
- expected provider reference if available;
- expected idempotency key or bounded idempotency marker if already modeled;
- explicit provider lookup client/transport supplied by the caller;
- caller-supplied auth material through the existing explicit auth boundary;
- bounded recovery classification context;
- sensitivity and redaction metadata.

The helper should return:

- lookup reconciliation posture;
- observed provider-side reference, if safely available;
- whether retry remains blocked;
- whether manual state repair may be planned;
- whether artifact write remains blocked;
- bounded next action;
- stable non-leaking error or warning posture.

Avoid first:

- event append;
- side-effect record mutation;
- report artifact writes;
- CLI command;
- automatic executor integration;
- hidden auth lookup;
- automatic retry.

## 8. Lookup Reconciliation Posture

Recommended stable vocabulary:

- `remote_comment_observed`: matching provider-side comment was found.
- `remote_comment_absent`: lookup completed and no matching comment was found.
- `remote_comment_ambiguous`: lookup found conflicting or insufficient matches.
- `lookup_not_authorized`: auth or policy did not permit lookup.
- `lookup_unavailable`: provider lookup could not run or returned unavailable state.
- `lookup_rate_limited`: provider rate limit prevented a conclusion.
- `lookup_target_invalid`: target identity failed validation before lookup.
- `lookup_response_untrusted`: response shape failed validation or redaction checks.

The posture must be enum vocabulary or stable codes, not natural-language parsing.

## 9. Matching Policy

The helper should match provider-side observations conservatively.

Allowed match signals:

- exact provider comment ID/reference when already known;
- exact pull request target identity;
- exact side-effect ID or idempotency marker only if it was intentionally embedded in a bounded provider-safe marker;
- bounded managed marker created by prior approved provider write path.

Rejected match signals:

- raw comment body matching;
- fuzzy text matching;
- model interpretation of comments;
- repository path inference;
- unbounded natural-language summaries;
- matching on secrets, tokens, or private payloads.

If the helper cannot match deterministically, it must return `remote_comment_ambiguous` or `lookup_response_untrusted`.

## 10. Provider Client Boundary

The future implementation should use injected transport/client patterns already established for GitHub PR comment provider work.

Requirements:

- caller supplies explicit auth/config;
- caller supplies injected transport or provider client;
- no hidden environment lookup unless a separately approved auth-loading phase extends the boundary;
- no default live network behavior;
- tests use fixtures or injected fake clients;
- live smoke tests, if ever added, must be explicit opt-in and non-default.

## 11. Error Handling

Errors must use stable non-leaking codes.

Candidate codes:

- `github_pr_comment_provider_lookup_reconciliation.invalid_input`;
- `github_pr_comment_provider_lookup_reconciliation.target_invalid`;
- `github_pr_comment_provider_lookup_reconciliation.auth_denied`;
- `github_pr_comment_provider_lookup_reconciliation.lookup_unavailable`;
- `github_pr_comment_provider_lookup_reconciliation.rate_limited`;
- `github_pr_comment_provider_lookup_reconciliation.response_untrusted`;
- `github_pr_comment_provider_lookup_reconciliation.redaction_invalid`;

Errors must not include:

- raw GitHub responses;
- comment bodies;
- PR bodies, diffs, review threads, or file contents;
- authorization headers;
- tokens or credentials;
- environment variable values;
- CI logs;
- command output;
- raw specs;
- parser payloads;
- repository paths or private URLs;
- secret-like IDs or metadata.

## 12. Workflow Semantics

Lookup/query reconciliation must not change workflow pass/fail status.

It must not:

- mutate `WorkflowRun`;
- append workflow events;
- emit audit events;
- emit observability events;
- mutate side-effect records;
- write report artifacts;
- retry provider writes;
- create filesystem artifacts;
- expose CLI output.

The first helper should be an explicit input-to-result function with optional injected provider lookup.

## 13. Relationship To Event-Proof Recovery

Event-proof recovery classification says what kind of recovery is needed.

Provider lookup/query reconciliation should answer only whether remote provider state can be observed.

Recommended flow:

1. Strict artifact gate denies missing or ambiguous event proof.
2. Recovery classifier returns bounded posture and next action.
3. Lookup reconciliation may be invoked explicitly when the posture calls for provider lookup.
4. Lookup result may inform a future manual repair plan.
5. No durable event proof exists until a separately approved event append or repair path records it.

## 14. Relationship To Manual Repair

Manual state repair remains deferred.

Provider lookup may support future repair, but it must not repair anything. A later manual repair plan should define:

- who may authorize repair;
- what evidence is required;
- what event, side-effect record, or artifact changes are allowed;
- how repair is audited;
- how duplicate provider writes are prevented.

## 15. Relationship To Artifact Writes

Report artifact gates remain the enforcement boundary.

Provider lookup result alone should not permit strict artifact write if durable workflow event proof is still missing. A later policy may allow a disclosure-only artifact, but that requires separate planning and review.

## 16. Privacy And Redaction

The lookup helper should be reference-first and posture-first.

Allowed:

- stable provider comment ID/reference if already bounded and validated;
- stable lookup posture;
- bounded count of candidate matches;
- bounded reason and next-action codes;
- redaction metadata after validation.

Forbidden:

- comment bodies;
- raw provider payloads;
- PR bodies, diffs, files, or review text;
- tokens or credentials;
- auth headers;
- environment values;
- command output;
- CI logs;
- raw specs;
- parser payloads;
- private paths or URLs;
- unbounded operator notes.

Debug, display, serialization, and deserialization errors must remain safe.

## 17. Test Plan

Future implementation tests should cover:

- remote comment observed by exact provider reference;
- remote comment observed by approved bounded managed marker;
- remote comment absent;
- ambiguous multiple matches;
- lookup not authorized;
- lookup unavailable;
- lookup rate limited;
- invalid target rejected before provider lookup;
- untrusted response rejected without leaking;
- secret-like auth/config/metadata rejected without leaking;
- no raw comment body copied;
- no raw provider response copied;
- retry remains blocked for ambiguous or split-brain states;
- artifact write remains blocked without durable workflow event proof;
- helper does not append events;
- helper does not mutate side-effect records;
- helper does not write artifacts;
- helper does not emit CLI output;
- debug and serialization are redaction-safe;
- existing provider write, reconciliation, event-proof recovery, report artifact gate, executor, WorkReport, and docs tests still pass.

## 18. Proposed Implementation Sequence

1. Add lookup reconciliation posture and next-action vocabulary.
2. Add explicit lookup input/result model.
3. Add injected lookup client trait or reuse an existing injected transport pattern if compatible.
4. Add fixture/fake-client tests for observed, absent, ambiguous, unauthorized, unavailable, and rate-limited outcomes.
5. Add non-leakage and non-mutation tests.
6. Create an implementation report.
7. Review before any event repair, artifact write composition, CLI command, schema exposure, example, hosted runtime, or broad write adapter expansion.

## 19. Deferred Work

- Automatic provider lookup.
- Hidden auth loading.
- Manual state repair helper.
- Workflow event repair.
- Event append from lookup result.
- Artifact write composition from lookup result.
- CLI recovery command.
- Workflow schema-declared recovery policy.
- Examples.
- Hosted/distributed runtime.
- Broader write-capable adapters.
- Reasoning lineage.
- Approval-presentation enforcement.
- Release posture changes.

## 20. Final Recommendation

Proceed next to a small implementation phase: **GitHub PR comment provider lookup/query reconciliation model/helper, explicit injected client only**.

Do not build automatic provider lookup, hidden auth, provider writes, retries, event append, state repair, artifact write composition, CLI behavior, schemas, examples, hosted behavior, broader write adapters, reasoning lineage, approval-presentation enforcement, or release posture changes in that phase.
