# GitHub PR Comment Provider Lookup Operator Recovery Plan

Status: Accepted. This follows the accepted [GitHub PR Comment Provider Lookup Recovery Integration Helper Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECOVERY_INTEGRATION_HELPER_REVIEW.md) and is accepted in [GitHub PR Comment Provider Lookup Operator Recovery Plan Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_PLAN_REVIEW.md).

## 1. Executive Summary

Workflow OS now has an explicit in-memory helper that composes GitHub PR comment provider lookup reconciliation with event-proof recovery classification.

The next question is how an operator should use that posture when a provider write outcome is ambiguous or when local event proof is missing.

This plan defines a future operator-facing recovery path. It does not implement CLI behavior, automatic lookup, hidden auth loading, retries, manual repair, event append, side-effect record mutation, report artifact writes, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

The operator recovery path must keep the central safety rule intact:

```text
Provider lookup can inform recovery, but it is not durable workflow event proof.
```

## 2. Goals

- Give maintainers an explicit way to inspect provider-side recovery posture.
- Keep lookup caller-supplied and bounded.
- Use the existing lookup recovery integration helper.
- Preserve strict event-proof gates for report artifact writes.
- Make retry, repair, and artifact-write posture understandable to humans.
- Require explicit operator action before any future repair or retry path.
- Avoid hidden auth, automatic lookup, automatic retry, and event fabrication.
- Avoid raw provider payloads, comment bodies, PR bodies, diffs, review threads, CI logs, command output, source contents, credentials, tokens, and private keys.
- Prepare for a later CLI/operator command without implementing it now.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this prompt;
- CLI commands or CLI rendering;
- automatic provider lookup;
- hidden or ambient auth loading;
- provider writes;
- retry or backoff behavior;
- manual repair;
- workflow event append from recovery;
- side-effect record mutation;
- report artifact writes;
- treating lookup observations as event proof;
- workflow schema changes;
- examples;
- hosted/distributed behavior;
- reasoning lineage;
- approval-presentation enforcement;
- release posture changes.

## 4. Operator Recovery Boundary

The operator recovery boundary should answer a narrow question:

```text
Given an attempted GitHub PR comment side-effect record and explicit lookup/recovery context, what should a maintainer do next?
```

It should not answer by doing the next action. It should return a bounded recovery posture that can be reviewed, cited, and later used to drive separately approved repair or retry phases.

Allowed future output:

- lookup posture;
- recovery posture;
- retry-blocked posture;
- artifact-write-blocked posture;
- operator-action-required posture;
- bounded next-action vocabulary;
- stable references to existing records/events;
- bounded non-leaking warnings.

Forbidden output:

- raw provider payloads;
- raw comments;
- raw diffs;
- raw CI logs;
- raw command output;
- hidden credential state;
- fabricated event IDs;
- fabricated provider references;
- claims that report artifact writes are safe without durable event proof.

## 5. Candidate User Experience

A future operator path may be exposed as a local command or internal service, but both should keep the same posture:

```text
workflow-os provider github-pr-comment recover --side-effect-id <id> --run-id <run-id> --explicit-auth <configured-reference>
```

This command shape is illustrative only. It is not implemented by this plan.

The human-facing output should lead with a concise card:

```text
Provider lookup recovery posture

remote_comment: observed
local_event_proof: missing
retry: blocked
artifact_write: blocked
operator_action: required
next_action: plan_manual_state_repair

Why:
- A bounded provider lookup found a matching remote comment.
- Workflow OS does not have accepted local event proof for the provider outcome.
- Lookup observations cannot replace workflow event proof.

What this does not do:
- does not write to GitHub
- does not repair state
- does not append events
- does not write report artifacts
```

The JSON output, if later added, should mirror the existing bounded model vocabulary and should not include raw provider payloads.

## 6. Input Policy

Future operator recovery should accept only explicit inputs:

- attempted `SideEffectRecord` identity;
- run/workflow identity;
- expected provider reference, if already known;
- expected managed marker, if available;
- explicit provider lookup auth supplied by caller;
- explicit recovery disclosure context;
- optional accepted event-proof reference, if already available.

It must not:

- read hidden global auth;
- search all repositories by default;
- inspect arbitrary provider payloads;
- infer provider references from raw comment bodies;
- load command output;
- read source files;
- mutate state to make recovery easier.

## 7. Auth And Provider Lookup Policy

Auth must remain explicit.

The first operator-facing path should use caller-supplied auth or an explicit named credential reference only after separate auth-loading planning. Until then, the existing injected client model is the safe boundary.

No hidden environment variables, keychains, browser sessions, git credentials, or ambient tokens should be read by default.

Lookup should remain bounded:

- one known repository;
- one known pull request;
- expected provider reference and/or managed marker;
- bounded page/observation count;
- stable classification for not authorized, unavailable, rate limited, invalid, ambiguous, absent, observed, and untrusted responses.

## 8. Event-Proof Policy

Provider lookup observations are evidence for operator review, not durable workflow event proof.

Strict report artifact gates must remain blocked unless accepted workflow event proof exists through the existing event-proof gate path.

Future operator recovery may recommend:

- manual state repair planning;
- retry eligibility reevaluation;
- authorized lookup retry later;
- fixing lookup inputs;
- collecting missing event proof.

It must not:

- append provider outcome events;
- backfill event proof silently;
- mark side effects completed;
- allow report artifact writes solely because the remote provider object exists.

## 9. Retry And Repair Posture

Retry posture must remain conservative.

Remote observed posture should generally block retry because repeating a provider write can duplicate external effects.

Remote absent posture may support retry eligibility review, but only after the event-proof recovery posture also permits it.

Manual repair remains deferred. A later repair phase should require:

- explicit operator intent;
- stable side-effect record identity;
- stable provider observation identity;
- high-assurance approval where configured;
- durable audit/event projection;
- report disclosure;
- non-leaking validation errors;
- no raw provider payload copying.

## 10. Failure Behavior

Operator recovery failures should fail closed and remain non-leaking.

Failure cases should include:

- missing attempted record;
- non-GitHub PR comment record;
- target mismatch;
- invalid auth input;
- lookup unavailable;
- lookup unauthorized;
- lookup rate limited;
- ambiguous provider observations;
- invalid recovery disclosure;
- secret-like inputs.

Failures must not:

- mutate runtime state;
- append events;
- write artifacts;
- retry provider calls;
- leak paths, tokens, provider payloads, comment text, or command output.

## 11. Documentation And Disclosure Requirements

Future operator recovery documentation must state:

- lookup is explicit and bounded;
- lookup is not event proof;
- recovery posture is advisory until a later approved repair/retry path exists;
- report artifact writes remain gated by durable event proof;
- hidden auth loading is not implemented;
- provider writes are not performed;
- state repair is not performed;
- CLI behavior is not implemented until separately approved.

## 12. Test Plan For Future Implementation

Future implementation tests should cover:

- observed remote comment returns operator-action-required posture;
- remote absent returns retry-eligibility-review posture only when recovery posture permits;
- missing event proof keeps artifact writes blocked;
- accepted event proof can allow artifact posture only through strict gate rules;
- unauthorized, unavailable, rate-limited, invalid, ambiguous, and untrusted lookup postures are represented;
- no provider writes occur;
- no workflow events are appended;
- no side-effect records are mutated;
- no report artifacts are written;
- no CLI output is emitted from internal helper paths;
- raw provider/comment/diff/log/command/source payloads are not copied;
- secret-like inputs are rejected without leakage;
- Debug and serialization remain redaction-safe;
- existing provider write, lookup, recovery, event-proof gate, report artifact, executor, and side-effect tests continue to pass.

## 13. Proposed Implementation Sequence

1. Review this operator recovery plan.
2. Add an internal operator recovery summary model/helper, in memory only.
3. Add focused tests for posture mapping and redaction.
4. Review.
5. Only after review, consider a local CLI/operator command.
6. Only after separate planning, consider manual state repair.

## 14. Deferred Work

- CLI/operator command.
- Hidden or configured auth loading.
- Automatic provider lookup.
- Automatic retry/backoff.
- Manual state repair.
- Workflow event append from recovery.
- Side-effect record mutation.
- Report artifact writes.
- Live lookup smoke tests.
- Examples.
- Schemas.
- Hosted behavior.
- Reasoning lineage.
- Approval-presentation enforcement.
- Release posture changes.

## 15. Open Questions

- Should the first operator recovery helper return only existing integration result types, or add a concise operator-facing summary type?
- Should CLI exposure wait until auth-loading posture is separately planned?
- How much remote lookup pagination is acceptable before the result becomes ambiguous?
- Should remote absent posture ever permit retry without an explicit operator confirmation?
- Should manual repair require high-assurance approval controls even for local-only repair?
- Should report artifact writes require a separately cited recovery decision even when event proof is present?

## 16. Final Recommendation

Recommended next phase: **provider lookup operator recovery summary helper review**.

The internal in-memory operator recovery summary helper is implemented as a bounded projection over an already validated lookup/recovery integration result. It does not add CLI behavior, hidden auth, automatic lookup, retry, repair, event append, side-effect mutation, artifact writes, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 17. Validation

- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783570140390043000-2 --phase planning`: passed.

## 18. Governed Dogfood Run

- workflow_id: `dg/d`
- run_id: `run-1783570140390043000-2`
- approval_id: `approval/run-1783570140390043000-2/planning-approved`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-provider-lookup-operator-recovery-planning-scope`
- event summary: 39 events total; 1 approval; 0 retries; 0 escalations.
- event kinds: ApprovalGranted, ApprovalRequested, PolicyDecisionRecorded, RunCompleted, RunCreated, RunResumed, RunStarted, RunValidated, SkillInvocationRequested, SkillInvocationStarted, SkillInvocationSucceeded, StepScheduled.
- out-of-kernel work disclosed: planning doc edits, docs validation, and git/PR actions are performed by the executor outside the kernel.
