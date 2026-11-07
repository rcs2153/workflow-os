# Cross-Harness Authoritative Continuation Milestone Plan

Status: preregistration selections accepted for focused review; runtime and live
provider execution not started

## 1. Product Question

Can one harness begin consequential governed work, a separately implemented
harness resume it after interruption, and an independent reviewer use the same
kernel-owned record to prove what was approved, what occurred, what remains
lawful, and whether the selected effect happened exactly once?

This is the most direct near-term test of the Workflow OS product thesis:

```text
Agent executes. Workflow OS governs.
```

The milestone is not a nested-agent or agent-team feature. It tests whether
independent execution hosts can consume the same public governance boundary
without recreating authority from transcript memory or custom hidden state.

Cross-harness governance and composition are first-class platform properties.
They are not optional presentation glue around a single preferred harness.
Workflow OS must remain the shared kernel that carries lawful continuation
between independently implemented execution hosts.

## 2. Three-Layer Authority Contract

The milestone must preserve and test three distinct layers:

1. **Procedure authority** governs who may define, revise, activate, or retire
   the workflow procedure and its policy, evidence, check, approval, and
   effect requirements.
2. **Invocation authority** governs who authorized this exact run and action,
   for which resource and execution window, and whether the replacement actor
   or harness may lawfully continue it.
3. **Execution truth** records what was attempted or observed at the effect
   boundary, whether the outcome is accepted, denied, ambiguous, or awaiting
   reconciliation, and whether repetition is lawful.

These layers may refer to one another, but they must not collapse into one
generic approval or audit record. Procedure stewardship does not authorize a
run. Run approval does not prove an effect. Provider or host observation does
not retroactively grant authority.

## 3. Preconditions

Execution is blocked until:

- hosted partial-run creation recovery is accepted on exact `main`;
- the authoritative continuation and compact-checkpoint contracts used by the
  selected path are stable enough for the evaluation;
- the selected consequential effect already has a reviewed bounded execution,
  SideEffect, idempotency, evidence, and report path;
- no new provider mutation family is required; and
- the benchmark protocol, expected decisions, interruption point, and review
  rubric are preregistered.

## 4. Harness Boundary

Use two independently implemented thin harness integrations. Each integration
may translate its host's invocation and display conventions, but both must use
the same documented Workflow OS CLI, API, or SDK contract.

Neither harness may:

- read private Core storage directly;
- infer approval from conversational history;
- copy authority into private session state;
- rewrite Workflow OS events, projections, reports, or checkpoints;
- use a hidden side channel to tell the second harness the expected answer; or
- contain benchmark-only logic that performs the governed decision itself.

The second harness must identify the acting principal and any delegation used
for continuation. Same-human, delegated-agent, service-actor, and unauthorized
actor substitution are distinct cases. The checkpoint or continuation record
is non-authorizing and must not upgrade the replacement harness's authority.

Preserve and publish the integration code, dependency versions, configuration,
and line count. Distinguish framework-native behavior from evaluator-authored
wiring.

## 5. Prior Evaluation Evidence And Claim Boundary

An evaluator-authored integration has already demonstrated the intended
shape: Codex started a governed run, LangGraph resumed it without conversation
history, one approved disposable Git commit occurred exactly once, denial and
authority drift produced no commit, and a fresh reviewer reconstructed the
approval and next action from the shared kernel record. A post-effect/pre-ack
fault remained `Running` instead of manufacturing success, and a dedicated
provider path recorded ambiguity.

That evidence is useful but bounded. The integrations were evaluator-written,
identities were synthetic, and the ambiguous target was simulated. It does
not establish packaged cross-harness support, authenticated enterprise
delegation, or a production provider-write contract.

## 6. Milestone Scenario

The selected workflow must include:

1. exact immutable run creation;
2. at least one policy or evidence obligation;
3. a real approval or a proportional-governance route that is authoritative
   for the selected profile;
4. interruption after authority is established but before the bounded effect;
5. replacement by the second harness with no conversational handoff;
6. kernel-owned continuation or checkpoint retrieval;
7. execution of one already-supported bounded consequential effect;
8. durable SideEffect/evidence/report reconciliation; and
9. terminal independent inspection.

Prefer an isolated existing write sandbox or another already-reviewed effect
that can be proven exactly once. Do not add a mutation family merely to make
the demonstration more impressive.

## 7. Required Assertions

The evaluation must prove:

- the second harness selects the same current objective and lawful next action;
- the second harness's actor and delegation are authorized for that exact next
  action at the time of use;
- granted approval remains bound to the exact immutable action;
- denied, expired, revoked, stale, or changed authority is not reused;
- completed work is recognized and not repeated;
- the consequential effect has one accepted attempt and one reconciled outcome
  at its existing idempotency and provider-observation boundary, with no
  duplicate Workflow OS SideEffect or completion claim;
- the WorkReport and evidence belong to the same run and effect;
- interruption does not require manual Workflow OS state repair;
- the independent reviewer can explain the complete decision and effect from
  the kernel record; and
- no host claims authority that the kernel record does not provide.

The assertions must separately identify which layer failed or succeeded:
procedure authority, invocation authority, or execution truth. A correct final
effect cannot cure invalid procedure or invocation authority.

Run adversarial variants for changed action, changed resource, stale approval,
missing evidence, terminal replay, and a second replacement attempt.

## 8. Measurements

Record per condition:

- harness-specific integration source lines and configuration;
- custom state or glue code outside Workflow OS;
- recovery turns and evidence lookups;
- gross input, cached input, output, and total tokens;
- uncached input plus output tokens;
- authoritative billed cost only when available from the provider;
- latency to lawful continuation and end-to-end completion;
- user or delegated-maintainer interruptions;
- decision correctness;
- duplicate or missing effects; and
- reviewer time to explain the run.

The existing controlled benchmark replicated roughly `49.9%` and `50.1%`
fewer gross recovery tokens while both arms made all nine tested decisions
correctly. That result does not establish dollar savings, broad model/task
generalization, or a moat by itself. Cache hits narrowed the uncached-token
difference. This milestone should test the stronger claim: Workflow OS supplies
one reusable authoritative record across execution hosts.

## 9. Productization Gates

The cross-harness property is not productized until all of these gates pass:

- authenticated actor and delegation binding rather than synthetic identity;
- packaged, versioned, stable public harness contracts;
- one real sandboxed provider write resumed across harnesses;
- post-effect/pre-ack reconciliation with no false success;
- duplicate prevention across retry, interruption, and replacement; and
- independent reconstruction from public kernel records alone.

Passing only the evaluator-authored fixture remains integration evidence, not
a general product capability.

## 10. Independent Review

An independent reviewer receives:

- the public integration code;
- the immutable run/checkpoint identifiers;
- bounded inspect, event, evidence, SideEffect, and WorkReport surfaces;
- the preregistered expected decisions; and
- the effect provider's bounded reconciliation evidence.

The reviewer must not receive the agents' hidden reasoning or an evaluator
answer key before recording the verdict. The review must distinguish what
Workflow OS proved, what the execution host asserted, and what the external
provider or storage owner controls.

## 11. Acceptance

The milestone passes only when:

- both harness integrations use the documented public boundary;
- replacement continuation is correct without conversational handoff;
- every adversarial authority case fails closed;
- the selected bounded effect has one accepted, reconciled outcome at the
  declared idempotency and provider-observation boundary;
- the terminal report and evidence remain correctly bound;
- an independent reviewer reconstructs the same approval and effect history;
- integration code and measurements are preserved; and
- all claims remain within the measured evidence.

A successful demonstration is adoption evidence, not proof of production
multi-harness orchestration, hosted SaaS readiness, or general agent autonomy.

## 12. Non-Goals

This milestone does not implement:

- nested harness runtime;
- recursive agents or agent teams;
- model routing or general orchestration;
- shared agent memory infrastructure;
- a new provider mutation family;
- enterprise identity administration;
- hidden automatic approvals;
- cross-tenant execution;
- a UI requirement; or
- a general comparative superiority claim.

## 13. Accepted Preregistration Selections

The focused planning review selected:

- **starting harness:** Codex;
- **replacement harness:** a pinned, independently implemented LangGraph Python
  adapter;
- **consequential effect:** the existing bounded GitHub PR-comment live-sandbox
  provider-write family;
- **target:** one maintainer-owned disposable pull request in a non-production
  repository posture, bound to the exact repository, pull request, head, comment
  request, and idempotency commitment;
- **interruption point:** after the immutable run bundle, approval-presentation
  proof, granted approval, actor/delegation posture, required evidence and checks,
  execution window, exact target/request commitment, and attempted SideEffect or
  pre-effect intent are durable, but before provider HTTP dispatch; and
- **review boundary:** the replacement harness receives no transcript, private
  session state, expected answer, or copied authority. It must identify its actor
  and delegation and recover lawful continuation through public Workflow OS
  surfaces.

`next-action` remains non-authorizing. The first implementation must add a narrow,
explicit continuation bridge rather than implying that a preview or checkpoint
can execute the effect. The candidate public shape is:

```text
workflow-os --json provider github-pr-comment continue-sandbox \
  --request <bounded-request.json> --auth-stdin
```

The exact spelling may change during implementation review, but the contract must:

- load the exact durable run and immutable run bundle;
- resolve actor and delegation in the same invocation;
- verify approval presentation, evidence and checks, execution window, target
  commitment, attempted SideEffect, and idempotency posture;
- call the existing reviewed GitHub PR-comment sandbox composition rather than
  introducing a generic provider-write path;
- preserve post-effect/pre-ack ambiguity without retry or manufactured success;
- persist eligible SideEffect event proof and bounded WorkReport disclosure;
- emit stable, redacted JSON; and
- accept provider authentication explicitly through standard input without
  storing or logging the credential.

## 14. Implementation Sequence

Implement the milestone in three separately governed slices:

1. **Public continuation bridge, fixture first.** Add the narrow CLI/JSON bridge,
   reuse the existing GitHub PR-comment sandbox composition, and prove fail-closed
   behavior without a live credential or provider mutation.
2. **Independent harness corpus.** Add the pinned Codex and LangGraph adapters,
   adversarial fixtures, measurement capture, and reviewer tooling. Keep this
   slice fixture-only.
3. **Live disposable-sandbox proof.** After the first two slices and their
   focused reviews pass, authorize one exact live target, perform the interrupted
   handoff, reconcile the effect, and obtain an independent rerun and review.

The fixture and live evaluation must cover approved exact continuation, denial,
changed target or action, unauthorized actor, missing/expired/revoked delegation,
stale approval or execution window, missing evidence, post-effect/pre-ack
ambiguity, terminal replay, a second replacement attempt, and conflicting
idempotency or provider observation.

No slice may claim fixed token savings, production interoperability, general
provider writes, nested harness runtime, or production multi-agent orchestration.

The accepted preregistration review is recorded in
[Cross-Harness Authoritative Continuation Milestone Preregistration Review](../concepts/CROSS_HARNESS_AUTHORITATIVE_CONTINUATION_MILESTONE_PREREGISTRATION_REVIEW.md).
