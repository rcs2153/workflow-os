# Authoritative Agent Continuation Context And Rehydration Plan Review

## 1. Executive Verdict

**Plan accepted after focused security corrections; proceed to one local
`BeforeSkillInvocation` continuation vertical slice.**

The P0 priority is justified. Durable run state cannot govern a material next
action when an external agent may instead continue from compacted conversation
memory or an untyped delegation summary. The plan now makes the brief
orientation-only, requires fresh Core-owned resolution in the consumer call,
and adds the durable cursor-bound single-consumption protocol needed for
parallel-agent safety.

## 2. Scope Verification

The phase remained planning and review only. It did not add:

- continuation runtime types or behavior;
- a CLI command;
- executor, hook, dogfood, or typed-handoff runtime wiring;
- provider writes or another mutation family;
- nested harness execution or recursive agents;
- schemas, SDKs, examples, hosted behavior, or enterprise administration;
- reusable permission leases or model self-approval; or
- release posture changes.

The private sandbox repository and one fixture branch created under the
preempted provider-smoke phase remain external setup only. No draft pull request
was created and no live provider transport smoke ran.

## 3. P0 And Product-Boundary Assessment

The invariant is correct:

```text
The agent may remember or propose the next step.
Only the kernel may declare and authorize the next material action.
```

This does not turn Workflow OS into an agent orchestrator. The kernel declares
an allowed governed operation from current durable facts; the agent or handler
still performs execution. It also does not require enumerating model reasoning
or every tool edge.

The first slice must be described honestly. It enforces this invariant for one
integrated local consumer. It cannot mechanically intercept arbitrary shell,
editor, git, browser, or provider actions performed outside that consumer.
Those surfaces remain procedurally governed until explicit integrations exist.

## 4. Existing-Foundation Assessment

The plan composes real foundations rather than inventing parallel governance:

- immutable run bundles bind the exact workflow inputs and content root;
- durable run events rehydrate current status and ordered sequence;
- approval-resume paths already reject changed resolved context;
- proportional-governance assessment fingerprints bind current facts;
- capability and governed-context models provide scoped authority vocabulary;
- required-context consumption fails closed for required gaps;
- the private current-authority same-call resolver recomputes from complete
  Core-owned inventories;
- typed handoffs provide bounded identity and obligation vocabulary; and
- `BeforeSkillInvocation` is an existing fail-closed executor checkpoint.

The plan correctly reuses these identities and commitments. A continuation
brief must not become a second authority model, approval model, or context
projection system.

## 5. Continuation-Brief Assessment

The brief is appropriately bounded and explicitly non-authoritative. Its
useful role is to orient agents and operators after compaction, restart, or
delegation while exposing blockers and strict non-goals.

Freshness cannot depend on `generated_at` or a caller assertion. The first
version must bind the exact immutable root, last event sequence, last event ID,
current step, and current governance commitments. Any later projection is a new
brief, not a refreshed permission.

Debug, serialization, errors, events, and reports must continue to exclude raw
prompts, source contents, command output, provider payloads, credentials,
environment values, paths, and model reasoning.

## 6. Same-Call Consumer Assessment

The selected first consumer is correctly narrowed to the existing local
`BeforeSkillInvocation` path. The operation vocabulary for the first slice is
only `invoke_current_step_skill`.

Core must perform rehydration, immutable binding checks, approval proof checks,
proportional reassessment, current-authority resolution, governed-context
projection, required-context consumption, obligation checks, exact operation
matching, and consumer invocation as one private composition. No generic public
callback or reusable authority object is acceptable.

The existing hook input currently carries caller-supplied invocation posture.
The continuation slice must not interpret that input as proof of fresh kernel
authority. It must insert the new Core-owned resolution boundary before the
skill can be invoked.

## 7. Atomicity And Parallel-Agent Assessment

The draft plan originally relied on a non-cloneable same-call value but did not
specify durable single consumption. That was a blocker because two processes
can independently construct non-cloneable values from the same stale event
position.

The corrected plan requires one durable idempotency claim bound to the
immutable root, last sequence and event ID, step, invocation key, next-action
code, and current governance commitments. Only the first writer proceeds. Core
then re-reads the event cursor before invocation. A mismatch burns the stale
claim and requires a new cursor-bound claim.

This can build on the existing atomic `IdempotencyStore` contract. It does not
create a bearer credential or ambient permission. The implementation review
must verify that no second eligible material action can race at the same cursor
in the first slice.

## 8. Event And Rehydration Assessment

The first event-position version should use the durable last event sequence and
last event ID together. Sequence continuity is already enforced by state
backends, while the event ID prevents a same-position substitution from being
treated as equivalent.

A later version may add an aggregate event commitment, but that is not required
for the first local proof. Snapshot status remains derived posture and must not
replace event-log rehydration.

## 9. Proportional-Governance Assessment

Rehydration is not a human interruption. Eligible low-risk reads may continue
quietly when current facts produce an accepted quiet posture. Visible
disclosure, blocking approval, and denial remain consequences of the current
assessment, not separate ways to bypass current-state resolution.

Stale, missing, ambiguous, approval-sensitive, or mutation-capable posture must
fail closed or retain its stronger governance mode. Old context may never lower
strictness.

## 10. Typed-Handoff Assessment

Typed handoff identity is necessary before child launch or result acceptance
can become governed runtime behavior. However, adding child start/result
runtime wiring in the same implementation as the first continuation consumer
would make the security proof too broad.

The corrected sequence defers typed-handoff runtime integration until the local
consumer is accepted. Natural-language child summaries remain untrusted and do
not establish governed completion.

## 11. Dogfood Assessment

One existing dogfood skill path should exercise the selected continuation
consumer after its Core tests pass. The phase runner should disclose the brief
identity and consumed next-action outcome.

This does not prove that the kernel intercepts all repository development. The
phase report must continue listing edits, shell commands, checks, git actions,
browser actions, and provider actions performed outside the kernel.

## 12. Test Quality Assessment

The corrected test plan is phase-ready and must prove:

- exact durable run and immutable-root projection;
- orientation-only brief behavior;
- current-state recomputation before invocation;
- one exact next action and consumer;
- stale event-position rejection;
- durable first-writer behavior under concurrent consumption;
- post-claim cursor-change rejection;
- approval, authority, context, evidence, check, sensitivity, and SideEffect
  invalidation;
- terminal and superseded run rejection;
- no duplicate skill invocation or continuation events;
- bounded non-leaking errors, Debug, serialization, events, and reporting; and
- regressions across executor, state, approval, immutable-run, proportional
  governance, authority, context, hook, typed-handoff, SideEffect, and report
  tests.

## 13. Blockers

None after correcting two findings in the plan:

1. durable cursor-bound single consumption was missing; and
2. the first slice combined public CLI, dogfood, typed-handoff runtime, and
   executor enforcement too broadly.

Both corrections are now explicit in the plan.

## 14. Non-Blocking Follow-Ups

- Add the read-only `next-action` JSON preview after the consumer is accepted.
- Add typed child start/result acceptance as a separately reviewed runtime
  slice before nested harness execution.
- Consider a versioned aggregate event commitment after the sequence-plus-ID
  proof has real operational evidence.
- Production multi-worker behavior will eventually need backend-specific
  contention, crash recovery, and claim-retention analysis.
- Expand checkpoint coverage only when each additional consumer has an exact
  operation vocabulary and independent tests.

## 15. Implementation Guardrails

- Keep the continuation authority boundary private to Core.
- Do not export a reusable authority or permission object.
- Permit only `invoke_current_step_skill` in the first version.
- Bind the claim to exact run, immutable root, cursor, step, invocation, and
  current governance commitments.
- Re-read the cursor after first-write claim and before invocation.
- Reuse current capability, context, approval, evidence, check, SideEffect, and
  idempotency sources of truth.
- Do not treat the agent-facing brief, hook input, or model judgment as proof.
- Do not add provider writes, schemas, nested execution, or hosted behavior.
- Disclose all material execution that still occurs outside the integrated
  boundary.

## 16. Recommended Next Phase

Implement one local end-to-end authoritative continuation slice for a selected
`BeforeSkillInvocation` skill path.

The slice should add the minimal model, brief projection, durable cursor-bound
claim, private same-call consumer, bounded events, focused executor and
concurrency tests, one dogfood proof, documentation, implementation report, and
focused security review.

## 17. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Plan claims were checked against current immutable-run, event-state,
  idempotency, current-authority, governed-context, required-context,
  typed-handoff, hook, approval-resume, and local-executor surfaces.

## 18. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1786770435830043000-2`
- approval ID:
  `approval/run-1786770435830043000-2/review-scope-approved`
- presentation ID: `presentation/164e93ee9aeeba8b`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- event posture: approval-presentation proof persisted and enforced; matching
  presentation marker present at phase close
- out-of-kernel work: source inspection, plan correction, review writing, and
  validation were performed by the delegated maintainer; the kernel governed
  scope and approval but did not edit files or run documentation checks
