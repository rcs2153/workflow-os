# Cross-Harness Authoritative Continuation Milestone Preregistration Review

Status: amended local Claude Desktop preregistration; same-actor fixture bridge
merged; distinct-principal corpus and live proof pending

## Review Verdict

The milestone should proceed, but only as a narrow, fixture-first proof of
authority portability. The product question is not whether two frameworks can
pass a message. It is whether a replacement execution host can recover the same
lawful action from kernel-owned state, prove that its actor and delegation remain
authorized, execute one already-reviewed bounded effect, and leave enough durable
truth for an independent reviewer to distinguish authority from execution.

The amended primary trial is Codex to Claude Desktop on the same Mac around the
existing GitHub PR-comment sandbox family. The original Codex-to-LangGraph
selection remains historical evidence and an optional comparator. It cannot
satisfy the primary trial. This is a test of independent harnesses, not
cross-machine transport. The review rejects a generic provider-write command,
conversation-based handoff, benchmark-only hidden state, and live execution
before the fixture corpus is accepted.

## Selected Harnesses

- Codex starts the governed run and reaches the preregistered interruption point.
- Claude Desktop in a fresh session acts as the replacement harness. Verify
  the actual app version, model, supported public tool surface, and how its
  isolated identity and credentials are provisioned before the corpus.
- The integrations must be independently implemented against the same
  documented public Workflow OS contract. If the app cannot use it within
  scope, report an integration blocker rather than substitute another harness.
- Preserve setup steps, versions, redacted configuration, dependencies, and
  integration source line counts with the evaluation evidence.

The replacement receives only documented public identifiers and inputs. No
chat transcript, workspace-file side channel, copied private Core or Codex
state, inherited approval assertion, or evaluator answer key may cross. Claude
must authenticate a principal distinct from Codex and present a kernel-verified
delegation for the exact run, approval proof, action, request/target, and window.
The kernel re-resolves expiry, revocation, consumption and current evidence at
use. An actor label or shared OS account does not prove identity. Only a later,
separately approved live phase may provide a scoped provider credential through
the approved transport.

## Selected Effect And Target

Use the existing GitHub PR-comment live-sandbox provider-write family because it
already has bounded request, target, policy, evidence, SideEffect, idempotency,
provider-observation, and report semantics. Do not add another mutation family.

The eventual live proof targets one maintainer-owned disposable pull request in a
non-production posture. The authorization must bind the exact repository, pull
request, immutable head where relevant, comment request, action, execution window,
and idempotency commitment. Fixture tests must precede any live credential use.

## Interruption Point

Interrupt after all of the following are durable:

- immutable run input and bundle identity;
- approval request and approval-presentation proof;
- granted approval or other authoritative governance disposition;
- actor and delegation posture;
- required evidence and check outcomes;
- current execution window;
- exact provider target and request commitment; and
- attempted SideEffect or equivalent pre-effect intent.

Interrupt before provider HTTP dispatch. This forces the replacement harness to
re-resolve authority and current facts without making an already-observed effect
artificially easy to recover.

## Public Continuation Contract

The merged public fixture bridge is a no-transport, same-actor baseline. Keep
its fail-closed behavior. Before using it for a distinct-principal claim,
implement and review the public delegation verification boundary. A candidate
live invocation, not available on the merged fixture path, is:

```text
workflow-os --json provider github-pr-comment continue-sandbox \
  --request <bounded-request.json> --auth-stdin
```

The implementation review may refine naming and request encoding. It may not
weaken these requirements:

1. Load the exact durable run and immutable bundle.
2. Authenticate the distinct replacement actor and re-resolve its exact
   delegation in the same call, not from a claimed actor label or checkpoint.
3. Reassess approval-presentation proof, evidence, checks, execution window,
   target commitment, attempted SideEffect, and idempotency posture.
4. Reuse the reviewed GitHub PR-comment sandbox composition.
5. Fail closed on missing, stale, changed, expired, revoked, denied, or ambiguous
   authority.
6. Preserve post-effect/pre-ack ambiguity without automatic repetition or false
   completion.
7. Persist eligible SideEffect event proof and bounded report disclosure.
8. Emit stable, redacted machine-readable output.
9. Accept authentication explicitly through standard input and never persist,
   echo, report, or log it.

`next-action`, a compact checkpoint, and a continuation preview remain
non-authorizing. They may explain what could be lawful; they may not execute the
provider effect.

## Preregistered Conditions

The fixture corpus must exercise the following in Claude Desktop without live
credentials or provider calls:

- exact approved continuation;
- denied continuation;
- changed action or resource;
- unauthorized replacement actor;
- missing, expired, or revoked delegation;
- stale approval or execution window;
- missing or changed required evidence;
- replay after terminal completion;
- a second replacement-harness attempt; and
- conflicting idempotency or fixture provider observation.

For the live slice, use the same Claude Desktop integration for one separately
approved disposable target. Interrupt after provider acceptance and before
local acknowledgement, reconcile without dispatching again, and have an
independent reviewer reconstruct the single effect and authority chain. A
fixture result is not evidence of a real provider outcome.

Each result must identify whether procedure authority, invocation authority, or
execution truth passed or failed. A correct provider outcome does not cure invalid
authority. A valid approval does not prove provider execution.

## Independent Review And Measurements

An independent reviewer must use only public integration code and bounded kernel
records. The reviewer must not receive hidden reasoning or the expected verdict
before recording the assessment.

Preserve per condition:

- setup steps, app/model and dependency versions, integration source and
  redacted configuration;
- custom state or glue outside Workflow OS;
- recovery turns and evidence lookups;
- gross, cached, uncached, and output tokens where available;
- authoritative cost only when supplied by the provider;
- continuation and completion latency;
- human or delegated-maintainer interruptions;
- decision correctness;
- duplicate or missing effects; and
- reviewer reconstruction time.

The prior roughly `49.9%` and `50.1%` gross recovery-token observations motivate
measurement but do not establish dollar savings, broad generalization, or a moat.
The principal claim under test is that one authoritative record remains usable
across independently implemented execution hosts.

## Delivery Sequence

1. Fixture-only, same-actor public continuation bridge: merged at `eb92306e`.
2. Implement and review the distinct-principal public authority boundary, then
   run the Codex-to-Claude Desktop fixture corpus with measurements and
   independent reviewer tooling. No live mutation.
3. Govern and execute one exact disposable live-sandbox proof, then obtain an
   independent rerun and review.

Each slice requires its own governed scope, validation, report, and acceptance.
Failure in one slice must not be hidden by success in a later demonstration.

## Non-Goals

This review does not authorize generic provider writes, a new mutation family,
nested harness runtime, recursive agents, agent teams, shared memory, enterprise
identity administration, hosted orchestration, broad autonomy defaults, or a
production interoperability claim.

## Amendment Governance Record

- workflow: `dg/d`
- run: `run-1794034836480821000-2`
- approval: `approval/run-1794034836480821000-2/planning-approved`
- approval presentation: `presentation/b02e81daefff05c3`
- approval outcome: granted under delegated maintainer authority after full
  scope presentation; documentation only, no app or provider execution

## Original Governance Record

- workflow: `dg/review`
- run: `run-1794022978943775000-2`
- approval: `approval/run-1794022978943775000-2/review-scope-approved`
- approval presentation: `presentation/3a4bcb78ae20ae07`
- approval outcome: granted under delegated maintainer authority after the full
  scope, non-goals, touched surfaces, validation expectations, and rationale were
  presented
- kernel role: coordination and durable governance record only
- execution disclosure: documentation inspection and editing occurred outside the
  kernel; no provider mutation or runtime implementation occurred
