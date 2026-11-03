# Authoritative Agent Continuation Context And Rehydration Plan

Status: Accepted after focused maintainer/security review. The first opt-in
local immutable-run `BeforeSkillInvocation` continuation slice is implemented
and focused-review accepted. The read-only local Core and CLI preview is also
implemented and focused-review accepted. The registered-current-authority
consumer and its final composition test blocker fix are implemented and
focused-review accepted in [Authoritative Continuation Registered
Current-Authority Consumer Plan](authoritative-continuation-registered-current-authority-consumer-plan.md).
The authorized-execution continuity core model is implemented and accepted.
Its first atomic-state slice now implements the separately versioned support
contract, private authoritative model, explicit unsupported backend posture,
and a test-only executable reference conformance store. Durable backend
transactions, runtime events, a supervisor, typed child runtime behavior, and
operational source configuration remain deferred.

P0 comprehensive-context status: implementation in progress under the
authoritative Active Delivery Queue. The pure model, closed collector model,
store-backed collectors, bounded local observers, accepted-intent capture,
aggregate proof, integrated private conformance composition, and exact private
projection delivery and recipient acknowledgment are implemented and
focused-review accepted.
The local end-to-end replacement-executor proof remains unimplemented. A live
post-compaction failure showed that an agent can receive or reconstruct stale
conversation context and answer it as current. The required correction is
broader than conversation replay detection. Workflow OS must reconstruct the
maximum relevant, authorized, provenance-preserving local context available,
resolve which source is authoritative for each domain, expose material gaps and
conflicts, and provide a bounded continuation bundle before material work can
continue. Directive freshness and durable suspension remain required hardening
inside that larger context system; they are not the system's center of gravity.

Related foundations:

- [Governed Work Pattern](../concepts/governed-work-pattern.md)
- [Typed Handoff Plan](typed-handoff-plan.md)
- [Required Context Contract Consumption Plan](required-context-contract-consumption-plan.md)
- [Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md)
- [Current-Authority One-Time-Use And Replay Posture Plan](current-authority-one-time-use-replay-posture-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)
- [BeforeSkillInvocation Required Checkpoint Plan](before-skill-required-checkpoint-plan.md)
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md)

## 1. Executive Summary

Long-running agent sessions can compact conversation history. Parallel agents
and subagents can start from isolated or summarized context. In both cases, a
model may infer its next action from stale conversational understanding instead
of current Workflow OS state.

This is not primarily a model-quality or prompt-quality defect. Conversation
context is probabilistic working memory. Workflow OS durable state, immutable
run inputs, current authority, policy, approval, evidence, checks, SideEffects,
and event history are the governance source of truth.

The P0 must separate three products:

1. a `ContinuationContextInventory` that accounts for every eligible context
   source with authority domain, provenance, freshness, sensitivity,
   availability, and conflict posture; and
2. a `ContinuationContextBundle` that materializes only the bounded subset
   required for the current workflow, phase, actor, and proposed action; and
3. a `ContinuationUnderstandingProjection` that derives an evidence-backed,
   non-authorizing account of what the work means: accepted intent, decisions,
   progress, remaining obligations, blockers, uncertainty, and lawful next
   options, with every material claim linked to inventory provenance.

The complete inventory is a kernel-internal decision input. Any inventory view
exposed to an agent, operator, hook, report, CLI, or child harness is a separate
governed projection. Lack of access to a source must not reveal its identity,
locator, authority domain, revision, or other sensitive existence metadata.

The understanding projection is not a generated conversation summary. It is a
typed derivation from authorized inventory entries and bundle projections. It
must distinguish authoritative, supported, advisory, conflicting, unknown, and
withheld claims. It restores orientation only; it cannot grant approval,
capability, policy satisfaction, or execution authority.

"Maximum context" means maximum relevant and authorized context with complete
inventory and explicit omissions. It does not mean copying every local file,
conversation turn, provider payload, or tool result into model context.

The future runtime invariant is:

```text
The agent may remember or propose the next step.
Only the kernel may declare and authorize the next material action.
```

The corresponding post-compaction understanding invariant is:

```text
A replacement executor with no conversational memory receives the same bounded,
evidence-backed understanding of lawful work as the prior executor, without
inheriting authority from the reconstruction.
```

The first implementation should compose existing foundations into one local
end-to-end continuation boundary. It should expose a bounded continuation brief
for agent orientation, then freshly re-resolve and consume one exact material
operation through a private Core-owned same-call boundary. It must not create a
reusable authority token or treat a serialized brief as permission.

## 2. Problem Statement

The current repository provides durable run state and selected proof-enforced
runtime paths, but an agent can still choose a subsequent repository action
from compacted local context between governed checkpoints. A delegated agent
may receive only a parent-authored summary and may not receive current dynamic
run state at all.

The problem also exists when all relevant information is local. Workflow YAML,
immutable run records, append-only events, approval proofs, WorkReports, Git
state, repository instructions, roadmap documents, and provider observations
can all be available while the agent sees only a compacted narrative. Reading
those sources directly is not enough: they have different authority domains,
freshness rules, sensitivity, and conflict semantics. A current workflow YAML
file, for example, is authoritative for a new declaration but cannot replace
the immutable definition bound to an already-started run.

That creates several failure classes:

- stale run status causes already-completed or denied work to continue;
- a pending, expired, revoked, or superseded approval is treated as valid;
- a changed policy, check, evidence, authority, or SideEffect fact is missed;
- two parallel agents act from the same old event position;
- a natural-language handoff widens scope or omits strict non-goals;
- a compacted summary becomes more influential than durable kernel state;
- a parent accepts a child summary without proving which governed task produced
  it;
- an agent reuses an earlier next-action result for a different operation.

Static `AGENTS.md` or harness prompts can require kernel use, but they cannot
prove current run state or enforce one exact continuation. Better summaries are
useful UX, not a correctness boundary.

## 3. Goals

- Make current durable kernel state authoritative for every material
  continuation.
- Rehydrate current run, event, approval, assessment, authority, context,
  evidence/check, and SideEffect posture before one exact operation.
- Give agents a bounded machine-readable summary of current posture without
  making that summary an authority credential.
- Bind delegated work to typed handoff identity, scope, inputs, non-goals,
  expected outputs, validation, evidence, and failure semantics.
- Reject stale, incomplete, mismatched, replayed, or ambiguous continuation.
- Preserve quiet execution for eligible low-risk reads through proportional
  governance.
- Keep model and harness vendors replaceable.
- Record enough bounded evidence to explain which kernel state governed an
  operation.
- Make compaction and parent/child context loss harmless to correctness.
- Inventory every eligible context source without requiring every source to be
  materialized into the agent prompt.
- Resolve authority by domain instead of treating all discovered text as equal.
- Distinguish discovered, selected, materialized, omitted, stale, unavailable,
  conflicting, and unauthorized context.
- Give a fully context-free replacement agent enough bounded information to
  orient correctly without making the projection an execution credential.
- Preserve the semantic effect of attributable accepted user direction as typed
  intent records before conversational context can disappear, without storing
  raw conversation or treating requested intent as execution authority.
- Derive a bounded understanding projection covering objective, accepted
  amendments, decisions and rationale references, completed and remaining work,
  blockers, uncertainty, validation/reporting commitments, and lawful next
  options.
- Prove collector coverage for the declared rehydration purpose so an
  unregistered, unsupported, failed, or skipped collector cannot silently appear
  as source absence.
- Trigger immediate rehydration when a trusted host reports compaction or
  executor replacement, while retaining unconditional current-state resolution
  at every material continuation boundary.
- Reuse existing immutable-run, governed-context, required-context,
  current-authority, evidence, WorkReport, SideEffect, and continuation
  contracts rather than building another state system.
- Preserve an inspectable explanation of why each source was selected or
  omitted and what must be refreshed before material continuation.

## 4. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- model chain-of-thought capture or persistence;
- treating conversation history as durable workflow state;
- reusable bearer tokens or TTL-based authority leases;
- model self-approval or authority inferred from model identity;
- arbitrary recursive agent spawning or agent swarms;
- nested harness runtime execution;
- provider mutation broadening or default writes;
- automatic command execution outside an accepted handler boundary;
- universal enforcement over arbitrary shell, editor, git, browser, or provider
  actions performed outside an integrated Core consumer;
- workflow schema changes in the first slice;
- hosted or distributed execution;
- enterprise identity, RBAC, or administration;
- raw prompt, source, tool-output, provider-payload, or credential persistence;
- detecting every vendor-specific compaction event as a correctness dependency.
- indiscriminate repository crawling or automatic inclusion of arbitrary files;
- treating operating docs, current YAML, generated summaries, or model memory
  as authority outside their declared domains;
- creating a second event ledger, workflow store, approval store, evidence
  store, policy engine, or capability system;
- allowing a serialized inventory or bundle to authorize execution;
- using token budget pressure to silently omit required context;
- persisting raw conversation history, hidden reasoning, secret values, or
  unbounded source contents as continuation state;
- requiring every supported host to expose a native compaction event;
- blocking unrelated bounded validation, read-only, release-admission, or
  maintenance work that does not claim compaction-safe autonomous continuation.

## 5. Core Invariants

1. A conversational summary is advisory and never sufficient for a material
   operation.
2. A serialized continuation brief is orientation, not authority.
3. Every material operation resolves current facts in the same Core-owned call
   that reaches the concrete consumer.
4. The operation must match one exact allowed next-action code and resource
   scope.
5. Current facts may preserve or increase governance strictness; stale context
   cannot weaken it.
6. Required context gaps, source failure, stale facts, unknown state, or
   identity mismatch block before consumer invocation.
7. Continuation authority is non-cloneable, non-serializable, non-reusable, and
   operation-specific.
8. Subagent results without an exact accepted typed handoff remain untrusted
   supporting input.
9. Low-risk refresh may be quiet, but it must remain durable and inspectable.
10. Correctness must hold even when the model never reports that compaction or
    context loss occurred.
11. Context authority is source- and domain-specific; there is no global
    "latest text wins" rule.
12. For an active run, the canonical immutable run bundle outranks subsequently
    edited declarative YAML for run inputs.
13. Append-only run events and authoritative stores determine dynamic run state;
    generated summaries and projections do not.
14. Required context is never silently dropped because of a model context or
    token budget. It is materialized, represented by an allowed bounded
    projection, or reported as a blocking gap.
15. Optional context omitted from the bundle remains visible in inventory and
    omission posture.
16. Conflicting authoritative facts produce a typed conflict or reconciliation
    requirement rather than an agent-selected winner.
17. A bundle is purpose-bound, actor-bound, scope-bound, freshness-bounded, and
    non-authorizing. Material execution still uses the existing same-call
    current-authority boundary.
18. A replacement agent with no prior conversation must reach the same kernel
    disposition as an agent with full conversational memory when given the same
    authoritative sources and request.

## 6. Existing Foundations And Remaining Gap

Workflow OS already has:

- immutable run bundles and exact run-definition binding;
- durable event history and run rehydration;
- approval-presentation proof and selected approval-resume reassessment;
- proportional-governance assessment fingerprints;
- scoped capability and governed-context projection models;
- required-context contract consumption;
- registered current-authority source and same-call one-time-use boundaries;
- typed handoff core vocabulary;
- selected `BeforeSkillInvocation` fail-closed checkpoints;
- a governed dogfood phase runner with bounded approval handoff output.

These foundations do not yet create a general continuation contract for an
agent or child harness. Typed handoffs do not have broad runtime behavior.
Automatic executor checkpoints are limited. The dogfood runner governs phase
start and close, but it does not mechanically require fresh kernel context
before every material continuation performed by the external executor.

The additional P0 gap is context assembly. Existing components prove individual
facts, but Core does not yet produce one complete inventory of eligible sources,
resolve source-specific authority and conflicts, select the context required by
the current operation, or explain omissions as a single bounded continuation
contract. The agent can therefore receive a valid next-action projection while
still missing important repository, operating, report, or external-resource
context that would change how the work should be understood.

### 6.1 Existing-Foundation Reconciliation

The P0 must extend, not replace, these foundations:

| Existing foundation | Reused responsibility | P0 addition |
| --- | --- | --- |
| loader and validated specs | parse declarations and compute deterministic content identity | inventory validated declaration sources and classify whether they apply to a new run or an active immutable run |
| immutable run bundle and record | preserve exact definitions used by a run | make the bundle authoritative for active-run inputs and inventory later YAML edits as non-applicable or conflicting context |
| durable run state and event ledger | establish current dynamic run posture | expose current cursor, terminal/waiting posture, and source revision through inventory references |
| governed context access | enforce typed reference access, sensitivity, and bounded projection | use it to authorize materialization instead of inventing context access rules |
| required-context contracts | define required and optional context obligations | drive bundle selection and fail closed when required context cannot be represented lawfully |
| current-authority resolution | resolve whether one exact operation remains lawful | consume the bundle only as input context; retain same-call authority as the execution boundary |
| evidence, checks, SideEffects, approvals, and WorkReports | preserve proof and reporting posture | inventory stable references and materialize bounded summaries only when required and permitted |
| typed handoffs | bind delegated scope and expected output | bind child context selection to the accepted handoff rather than parent-authored prose |
| proportional governance | determine interruption and disclosure posture | permit quiet context refresh without weakening context completeness or execution authority |
| first-run and trusted-host metadata discovery | perform bounded safe repository inspection | supply explicit local collectors without turning Core into a filesystem crawler |

No P0 type may duplicate an existing identifier, decision, projection, or store.
The focused architecture review must reject any proposed field that cannot name
its existing authoritative source or explain why a new source is necessary.

### 6.2 Source Classes And Authority Domains

The inventory must use a closed first-slice source vocabulary. Candidate source
classes and their authority boundaries are:

| Source class | Authoritative for | Not authoritative for |
| --- | --- | --- |
| validated project declarations | current declarations for future validation or run creation | changing inputs of an active immutable run |
| immutable run bundle and record | exact workflow, skill, policy, check, and context definitions bound to an active run | live provider state or later repository edits |
| run state and append-only events | current execution, wait, retry, approval, and terminal posture | repository contents or provider truth not captured by an accepted observation |
| policy, approval, capability, and authority records | current governance decisions inside their exact scope and validity window | broader user intent or undeclared resources |
| evidence, validation, check, SideEffect, artifact, and WorkReport records | recorded proof, disclosure, and report posture within their typed boundaries | facts that were never observed or have exceeded freshness requirements |
| Git and worktree observation | current repository identity, head, branch, cleanliness, and bounded diff posture at observation time | workflow authority or remote provider state |
| allowlisted operating documents | engineering guidance, roadmap intent, runbooks, ownership, and declared local instructions | approval, policy, capability, or run-state authority |
| external provider observation | current remote resource state for the observed resource and revision | kernel authorization to mutate that resource |
| current user or delegated-maintainer directive | requested intent and priority, subject to attributable identity | bypass of immutable scope, failed policy, missing evidence, denied authority, or required checks |
| accepted intent record | bounded semantic effect of one attributable accepted objective, amendment, priority, pause, stop, or resume direction | execution permission, policy satisfaction, approval, capability, or facts absent from the accepted direction |
| agent memory or generated summary | advisory orientation and candidate queries | any durable fact or execution authority |

Authority is evaluated per fact domain. Two sources may both be valid while one
is authoritative for the active run and the other is authoritative for a future
run. A conflict is not resolved by file modification time, discovery order, or
the confidence of a generated summary.

### 6.3 Continuation Context Inventory

The inventory is a complete account of eligible sources, not a raw payload
archive. Each entry should carry only bounded metadata needed to decide whether
and how the source may contribute:

- stable inventory-entry identity and closed source class;
- authoritative source reference or locator class without leaking sensitive
  local paths by default;
- project, workflow, run, phase, step, actor, resource, and handoff scope where
  applicable;
- authority domain and precedence rule identifier;
- source revision, event cursor, immutable root, content commitment, provider
  revision, or equivalent freshness anchor;
- observed-at time, maximum accepted age when applicable, and freshness posture;
- availability, integrity, sensitivity, redaction, and access posture;
- required, optional, advisory, or inapplicable obligation posture;
- selected, materialized, reference-only, omitted, stale, unauthorized,
  unavailable, conflicting, or superseded disposition;
- bounded reason code for selection, omission, conflict, or refresh need; and
- collector and inventory algorithm version.

The inventory must be deterministic for the same explicit inputs. It must not
scan arbitrary files from Core, infer authority from prose, or store raw source
contents. Local hosts and adapters collect candidate observations through
explicit interfaces; Core validates, classifies, and reconciles them.

Inventory completeness must be evaluated against a purpose-bound
`ContinuationContextSourceCoverage` contract. That contract identifies the
closed collector classes required, optional, unsupported, or inapplicable for
the declared purpose; records which collectors were invoked; and distinguishes
successful empty observation, access denial, collector failure, unsupported
collection, and intentional omission. A host cannot claim a complete inventory
merely because it registered only the collectors it happened to implement.

Completeness applies to the private Core decision inventory, not to every
consumer-visible projection. An exposed inventory projection must pass through
existing governed-context access and redaction rules. When even source
existence is sensitive, the projection may disclose only an opaque withheld or
incomplete posture, without entry identity, locator, source class, authority
domain, revision, or content commitment. Counts must also be coarsened or
omitted when they would reveal protected topology.

### 6.4 Continuation Context Bundle

The bundle is the bounded materialized context supplied to an agent, operator,
hook, or child harness for one declared purpose. It should include:

- bundle identity, algorithm version, creation time, and content commitment;
- exact project, workflow, run, phase, step, actor, handoff, and proposed-action
  binding where applicable;
- current immutable-run root and event cursor;
- selected inventory-entry references and their authority domains;
- bounded typed projections already permitted by governed-context access;
- required-context satisfaction and explicit blocking gaps;
- optional omissions and the reasons they were omitted;
- conflict, stale-source, refresh-required, and unsupported-source posture;
- strict scope, non-goals, validation obligations, expected outputs, and next
  lawful disposition when available;
- sensitivity ceiling, redaction posture, and materialization budget; and
- an explicit statement that the bundle is orientation, not authority.

The bundle must be purpose-specific. A planning bundle, implementation bundle,
approval-review bundle, provider-reconciliation bundle, and child-handoff bundle
may select different context from the same inventory. Reuse requires exact
purpose, scope, actor, immutable root, event cursor, freshness, and access
equivalence; otherwise Core rebuilds it.

### 6.4.1 Durable Semantic Capture And Understanding Projection

Full post-compaction understanding requires durable semantic inputs that cannot
be reconstructed from run posture alone. When a host can provide attributable
directive identity, Workflow OS should preserve only the accepted semantic
effect of material user or delegated-maintainer direction as a typed
`AcceptedIntentRecord`. The record should bind the attributable directive,
project and run or phase scope, objective or amendment kind, bounded accepted
meaning, supersession posture, and provenance commitment. It must not copy raw
conversation, hidden reasoning, or unbounded prose, and it must not grant
execution authority.

The `ContinuationUnderstandingProjection` is derived, not independently
authored. It should include bounded claims for:

- current objective, priority, accepted amendments, scope, and strict non-goals;
- accepted decisions and bounded rationale references;
- completed, in-progress, deferred, and remaining obligations;
- blockers, registered waits, unresolved questions, assumptions, and explicit
  uncertainty;
- required evidence, checks, SideEffects, validation, reporting, and promised
  follow-up posture;
- current lawful next options and the source of each option; and
- differences from an earlier projection caused by newer authoritative facts,
  refresh, supersession, access changes, or permitted omission.

Every material claim must cite one or more inventory entries and identify its
authority domain and claim posture. Conflicting or unknown claims remain
conflicting or unknown; a model cannot convert narrative coherence into kernel
truth. Agent memory and generated summaries may contribute advisory candidate
claims that are retained only when supported by authorized sources or labeled
advisory with explicit provenance.

### 6.4.2 Incremental Derivation Engine Evaluation

After the pure canonical inventory, source-coverage, bundle, and understanding
models are accepted, evaluate Salsa as one possible internal incremental query
engine for those derived products. Candidate Salsa inputs may include immutable
run roots, event cursors, accepted-intent records, policy and approval
commitments, evidence and check posture, SideEffect posture, Git observations,
allowlisted document revisions, provider-observation revisions, access posture,
collector versions, and algorithm versions. A relevant input change should
invalidate only dependent derivations; an irrelevant change should not churn
the canonical understanding commitment.

Salsa is not selected by this plan and must not shape the canonical model. Add
an internal derivation-engine boundary only after a straightforward
deterministic implementation defines expected outputs independently. A bounded
prototype may then compare clean derivation, incremental invalidation, restart
reconstruction, and canonical result equality. Adoption requires evidence that
it materially improves dependency tracking or recomputation without changing
semantics or the supported Rust toolchain posture.

Any Salsa database is a discardable derived cache. It must never own durable
state, the event ledger, accepted intent, approvals, capabilities, policy,
SideEffects, audit records, authorization, execution, or the same-call authority
boundary. Rebuilding from an empty cache and the same authoritative inputs must
produce the same canonical projection commitment. Cache contents cannot grant
authority, satisfy evidence, or make an operation executable.

### 6.5 Selection And Materialization Algorithm

The first pure decision layer should operate only on explicit validated inputs:

1. enumerate eligible source metadata from registered collectors and existing
   authoritative stores;
2. classify authority domain, scope applicability, sensitivity, freshness, and
   integrity without reading arbitrary payloads;
3. derive required and optional context from immutable run bindings, required-
   context contracts, typed handoffs, current phase, and proposed action;
4. resolve source-specific precedence and surface unresolved conflicts;
5. authorize each projection through existing governed-context access and
   capability rules;
6. select the least data needed to satisfy all required obligations and useful
   optional context within the bounded materialization budget;
7. represent omitted optional sources and every required gap explicitly;
8. produce the inventory and non-authorizing bundle commitments;
9. derive the non-authorizing understanding projection and claim-level source
   map; and
10. pass material execution to the existing fresh same-call authority boundary,
   which rechecks current facts before consumer invocation.

Budget exhaustion cannot silently remove a required source. The algorithm must
prefer typed summaries, reference-only projections, and deterministic bounded
metadata before raw content. If no permitted representation can satisfy a
required obligation, the result is blocked with a stable gap code.

The private inventory commitment may bind protected entries for integrity, but
consumer-visible commitments must be domain-separated and scoped so they cannot
be used to correlate hidden sources across actors, projects, or runs.

### 6.6 Conflict, Freshness, And Gap Semantics

At minimum, the P0 model must distinguish:

- `current`: the authoritative source satisfies its declared freshness rule;
- `stale_refreshable`: a permitted collector may refresh before continuation;
- `stale_blocking`: refresh is unavailable or failed and the fact is required;
- `superseded`: a newer authoritative record exists;
- `inapplicable`: the source is valid but does not govern this run or action;
- `conflicting`: two sources claim incompatible facts in the same authority
  domain and no deterministic rule resolves them;
- `unauthorized`: the source exists but current context-access authority does
  not permit materialization;
- `unavailable_required`: a required source cannot be loaded or represented;
- `unavailable_optional`: an optional source is omitted with disclosure; and
- `unsupported`: the collector or source class is not implemented.

Required conflicts and required unknown posture fail closed before the bundle is
declared actionable. Advisory conflicts remain visible and cannot be silently
converted into kernel facts by the agent.

### 6.7 Replay, Directive Freshness, And Durable Suspension

The observed stale-message incident remains an important subproblem. Once the
inventory and bundle boundaries exist, a host integration may add payload-free
directive receipts, stable host/turn identity or an equivalent monotonic cursor,
duplicate-delivery posture, and durable stop/pause suspension. These records
belong in the inventory as requested-intent and control-state sources; they do
not replace run state or become a parallel permission system.

Where a host cannot provide stable directive identity, Workflow OS may still
assemble bounded context and allow read-only orientation. It must not claim
replay-safe automatic intent changes. A resume request remains a request and
must pass fresh kernel admission against current authority, policy, evidence,
checks, SideEffects, repository state, and external-resource state.

Accepted-intent capture is required for the first full-understanding proof when
material user intent is not already represented by an immutable run, typed
handoff, accepted plan, approval presentation, or another existing authoritative
record. Broader host replay protection and durable suspension remain subordinate
hardening, but the proof cannot claim semantic continuity by relying on
conversation that the replacement executor does not receive.

### 6.7.1 Explicit Post-Compaction Rehydration

When a trusted host exposes a reliable compaction, context replacement, worker
replacement, or session-resume signal, the host must immediately invalidate the
prior orientation projection and request a fresh inventory, bundle, source-
coverage result, and understanding projection. The refresh outcome and new
commitments should be recorded through bounded payload-free audit posture.
Material continuation remains blocked until required context and understanding
claims are current and the replacement executor has received and acknowledged
the fresh projection. Acknowledgment proves delivery of an exact projection
commitment, not human-like comprehension or execution authority.

Host signaling improves UX and observability but is not the correctness
dependency. Hosts that compact silently, omit lifecycle signals, or deliver a
stale generated summary remain safe because every material boundary rebuilds
current context and performs the existing same-call authority check.

### 6.8 Disruption Boundary

This P0 is intentionally additive. It does not invalidate accepted immutable
run, current-authority, continuation, approval, proportional-governance,
provider-sandbox, or trusted-host work. It creates a composition layer that
names and projects those facts consistently.

The P0 preempts claims of compaction-safe delegated continuation, automatic
cross-agent handoff, or broader autonomous continuation. It also precedes new
nested-harness and broad provider-mutation families that depend on reliable
continuation. It does not block bounded read-only work, validation, release
admission, or maintenance that uses existing explicit governance boundaries and
does not claim context-independent autonomous continuation.

## 7. Candidate Core Model

The first model review should consider the smallest set of types needed for a
future continuation path:

- `ContinuationContextInventoryId`
- `ContinuationContextInventory`
- `ContinuationContextInventoryEntry`
- `ContinuationContextSourceClass`
- `ContinuationContextAuthorityDomain`
- `ContinuationContextDisposition`
- `ContinuationContextBundleId`
- `ContinuationContextBundle`
- `ContinuationContextGap`
- `ContinuationContextConflict`
- `ContinuationContextSourceCoverage`
- `AcceptedIntentRecordId`
- `AcceptedIntentRecord`
- `ContinuationUnderstandingProjectionId`
- `ContinuationUnderstandingProjection`
- `ContinuationUnderstandingClaim`
- `ContinuationUnderstandingClaimPosture`
- `ContinuationUnderstandingDeliveryReceipt`
- `GovernedContinuationBriefId`
- `GovernedContinuationBrief`
- `GovernedContinuationBinding`
- `GovernedNextAction`
- `GovernedContinuationBlocker`
- `GovernedContinuationContextPosture`
- `GovernedDelegationBinding`
- `GovernedContinuationUseOutcome`

The names are provisional. The model must reuse existing IDs, immutable bundle
bindings, assessment bindings, required-context results, authority source
commitments, approval proof markers, check/evidence references, SideEffect
references, and typed handoff IDs rather than defining parallel substitutes.

## 8. Governed Continuation Brief

The brief is a bounded, redaction-safe projection intended for agents,
operators, hooks, and machine consumers. It should include only stable identity
and posture:

- workflow, run, phase, and current step identity;
- immutable bundle identity and root;
- current event cursor, sequence, or state revision;
- terminal, waiting, running, blocked, or failed run posture;
- pending approval identity and presentation-proof posture;
- current proportional-governance assessment identity and fingerprint;
- authority source identity, freshness posture, and bounded watermark identity;
- required-context satisfaction and explicit gaps;
- required evidence and check posture;
- SideEffect and retry posture;
- typed handoff identity and expected target when delegated;
- allowed next-action codes;
- explicit blockers and strict non-goals;
- current understanding-projection identity and commitment;
- generated-at time and brief algorithm/version identity.

The brief must not contain raw prompts, source contents, command output,
provider payloads, tokens, credentials, authorization headers, environment
values, private keys, model reasoning, or unbounded summaries.

Brief validation must detect field substitution, non-canonical ordering,
duplicate obligations, mismatched execution scope, and stale binding. A valid
brief remains non-authoritative for execution.

## 9. Same-Call Continuation Use Boundary

Immediately before one material operation, Core should:

1. load and validate the exact durable run;
2. load the exact immutable run bundle;
3. read the current event position and terminal posture;
4. verify approval decision and presentation proof when applicable;
5. recompute proportional governance from current registered facts;
6. resolve current capability and authority;
7. rebuild governed context projection;
8. consume the exact required-context contract;
9. validate current evidence, checks, sensitivity, and SideEffect posture;
10. derive allowed next-action vocabulary;
11. require the requested operation and target to match exactly; and
12. invoke one concrete Core-owned consumer through the existing non-reusable
    same-call pattern.

The first consumer should be narrow and local. It should not expose a generic
callback that turns the boundary into reusable ambient authority.

### 9.1 Atomic Continuation Consumption

A non-cloneable in-process value is not sufficient when two workers or agents
can observe the same durable run position. The first slice must therefore bind
one exact `BeforeSkillInvocation` operation to:

- the immutable run root;
- the current last event sequence and event ID;
- the current step and invocation idempotency key;
- the selected next-action code; and
- the current assessment, authority, context, approval, evidence, check, and
  SideEffect commitments.

Core must create one durable idempotency claim for that exact binding. Only the
first writer may continue. Before invoking the consumer, Core must re-read the
event position and reject the operation when it no longer matches the claimed
cursor. A cursor mismatch burns that exact claim and requires a fresh
rehydration with a new cursor-bound claim; it must not retry under the stale
binding. This gives the first local slice deterministic single-consumption
behavior without creating a reusable permission lease.

The first slice should allow exactly one material next action:
`invoke_current_step_skill`. Expanding the vocabulary requires a separate
review once another concrete consumer exists.

## 10. Required Rehydration Checkpoints

Correctness should use unconditional current-state resolution at material
boundaries rather than relying on compaction detection. Candidate checkpoints:

- trusted host notification of compaction, context replacement, executor
  replacement, or session resume;
- session or worker resume;
- run retry;
- approval resume;
- before each governed skill invocation;
- before required check execution;
- before report or artifact production;
- before SideEffect attempt or provider call;
- before subagent or child harness launch;
- before accepting a child result;
- before workflow or phase completion.

Vendor-specific compaction, interruption, or subagent lifecycle events may be
recorded when available, but absence of those events must not weaken the
boundary.

## 11. Typed Subagent And Harness Handoffs

Delegated execution must not rely on a parent-generated prose summary alone.
The kernel should bind each child task to:

- parent workflow, run, step, event position, and immutable root;
- child actor or harness identity;
- exact approved scope and strict non-goals;
- required and optional governed context references;
- allowed capabilities, tools, and SideEffect posture;
- expected output and artifact schema or stable vocabulary;
- evidence and validation requirements;
- timeout, retry, cancellation, and failure semantics;
- acceptance authority and next obligation.

The child begins from a kernel-produced continuation brief. Material child
operations still require fresh same-call resolution. The parent accepts the
result only after exact typed handoff validation. Missing, stale, superseded,
or mismatched handoffs fail closed without converting a natural-language report
into fake evidence.

## 12. Proportional Governance And Quiet Refresh

Current-state rehydration is not synonymous with a human interruption.

- Eligible bounded reads may refresh under `Proceed + Quiet` and record the
  resulting posture without verbose output.
- Review-worthy changes may use `Proceed + Visible` disclosure.
- Approval-sensitive actions must preserve proof-enforced blocking approval.
- Denied, stale, ambiguous, missing-context, or source-failure posture blocks.

This keeps automation fast while preventing stale conversational state from
becoming an authorization shortcut.

## 13. Event, Audit, Evidence, And Report Posture

Future continuation events should remain payload-free and may record:

- brief identity and algorithm version;
- immutable bundle root reference;
- source snapshot or watermark identity;
- event cursor or state revision;
- requested and selected next-action code;
- typed handoff identity;
- assessment, approval-proof, evidence, check, and SideEffect references;
- blocked, consumed, failed, or ambiguous outcome;
- actor and correlation identity; and
- compaction or replacement trigger class, prior orientation commitment, source-
  coverage commitment, and replacement understanding commitment without raw
  conversation or projected claim payloads.

The WorkReport should disclose continuation gaps, stale-context rejection,
delegation posture, skipped refreshes, and work performed outside the governed
boundary. It must not copy conversation history or subagent transcripts.

## 14. Failure Behavior

- Missing current source: fail closed before material consumer access.
- Stale brief: regenerate for orientation; do not reuse it as authority.
- Event cursor mismatch: rehydrate and reassess.
- Terminal or superseded run: reject continuation.
- Pending, denied, expired, or mismatched approval: block.
- Revoked or expired authority: block.
- Missing required context, evidence, or checks: block or disclose only when the
  exact contract marks the obligation optional.
- Child result without an accepted handoff: reject as governed completion.
- Consumer may-have-started ambiguity: preserve existing conservative recovery
  and retry-blocking semantics.

Errors must use stable codes and must not echo IDs, paths, prompts, summaries,
payloads, tokens, or secret-like values.

## 15. Proposed Implementation Sequence

Complete the enforcement proof as one reviewed local vertical slice rather
than an extended model-only sequence:

1. Focused maintainer/security plan review.
2. Add the minimal continuation brief, binding, next-action, blocker, and
   outcome model.
3. Add one pure brief projection from an exact immutable run and current
   registered facts.
4. Add one durable cursor-bound idempotency claim and private Core-owned
   same-call continuation consumer for the local `BeforeSkillInvocation` path.
5. Add bounded continuation events and focused tests proving stale-cursor and
   parallel-consumption rejection before skill invocation.
6. Integrate only the selected dogfood `BeforeSkillInvocation` path so one
   material dogfood continuation proves current brief projection and
   authoritative same-call consumption. Disclose that arbitrary repository,
   shell, editor, git, browser, and provider actions outside this path remain
   governed procedurally rather than intercepted mechanically.
7. Perform focused maintainer/security review before any provider-write or
   nested-harness expansion resumes.

The read-only `workflow-os next-action <run-id>` preview is now implemented for
exact local immutable runs whose current hook and SideEffect context is fully
reconstructable. Human and JSON output expose the same bounded brief and state
explicitly that preview is non-authoritative and non-consuming. Projection
does not claim idempotency, append events, invoke handlers, write artifacts, or
change run state. Typed child handoff start/result acceptance remains deferred.

The P0 comprehensive-context follow-up should proceed in these independently
reviewed increments:

8. Complete a focused architecture/security review that maps every proposed
   inventory and bundle field to an existing authoritative source, or rejects
   it as a parallel source of truth.
9. Implement a pure model-only inventory, source-coverage, selection, accepted-
   intent, and understanding-projection decision layer from explicit validated
   inputs. Include closed source classes, authority domains, dispositions, gaps,
   conflicts, purpose binding, claim provenance, claim posture, and deterministic
   commitments. Do not add collectors or persistence in this increment.
10. Add in-memory conformance tests proving source-specific precedence,
    immutable-run priority for active runs, required-context blocking, optional
    omission disclosure, collector-coverage completeness, sensitivity
    enforcement, deterministic selection, supported/advisory/conflicting/unknown
    understanding claims, and non-authorizing bundle and projection behavior.
    Define canonical expected outputs without an incremental framework. Only
    after these outputs are accepted may a separately reviewed Salsa-backed
    prototype compare canonical equality, precise invalidation, irrelevant-input
    stability, and empty-cache restart reconstruction. No dependency addition is
    authorized by this plan alone.
11. Add explicit local collectors behind interfaces for validated project
    declarations, immutable run records, durable run/event posture, WorkReport
    and evidence references, accepted-intent records, Git/worktree posture, and
    an allowlisted operating-document set. Collectors return bounded
    observations; they do not decide authority or crawl arbitrary repository
    content. The detailed boundary, source-ownership matrix, stable-anchor
    algorithm, phased implementation, and conformance requirements are now
    specified in the [Continuation Context Trusted Local Collectors
    Plan](continuation-context-trusted-local-collectors-plan.md). Focused
    architecture/security review of that plan is the next phase. No collector
    implementation or operational composition is authorized by planning.
12. Add one narrow attributable accepted-intent capture boundary before the
    full-understanding proof. Persist only the typed semantic effect, provenance,
    scope, and supersession posture; do not persist raw conversation or make the
    record an authority credential. When accepted intent is already represented
    by an immutable run, typed handoff, accepted plan, or approval presentation,
    reference that existing authoritative source instead of duplicating it.
13. Compose collector observations through existing governed-context access and
    required-context consumption. Unsupported or unauthorized sources remain
    typed inventory entries rather than disappearing.
14. Add one read-only `continuation-context` preview for an exact local run or
    governed phase. Human output should lead with current posture, blockers,
    objective, accepted decisions, progress, remaining obligations, uncertainty,
    lawful next options, selected source classes, conflicts, and omissions; JSON
    may expose the full bounded inventory, coverage result, bundle, and
    understanding projection. Preview remains non-consuming and non-authorizing.
15. Integrate one existing same-call material boundary so the bundle is rebuilt
    and required gaps are checked immediately before the already-supported
    operation. Do not add a new operation family in this increment.
16. Add one induced total-conversation-loss dogfood proof: a replacement agent
    receives no prior transcript, reads only the bounded bundle and referenced
    permitted sources, reconstructs the same bounded objective, accepted
    decisions, progress, remaining obligations, blockers, uncertainty,
    validation/reporting commitments, and lawful next material action as the
    original executor, completes only the approved scope without repeating
    completed work, and produces a WorkReport disclosing coverage, inventory,
    omissions, conflicts, refreshes, and out-of-kernel work.
17. Add one trusted-host compaction-signal proof that invalidates an earlier
    orientation commitment, produces a fresh projection, records bounded audit
    posture and an exact delivery acknowledgment, and blocks material
    continuation until refresh succeeds. Retain a companion test proving silent
    compaction is still safe at the next material boundary.
18. After the context proof is accepted, review the subordinate payload-free
    directive-receipt and durable-suspension slice for hosts that can provide
    stable attributable message identity. Do not make that host feature a
    prerequisite for context inventory or read-only rehydration.
19. Complete adversarial review before claiming compaction-safe delegated
    continuation, cross-agent continuation, or context-independent autonomous
    maintenance.

## 16. Test Plan

Future tests must cover:

- a current brief projects the exact durable run and immutable root;
- a brief cannot authorize an operation by itself;
- exact allowed next action reaches the selected consumer once;
- two concurrent consumers at the same cursor produce one durable first writer
  and one fail-closed duplicate outcome;
- stale event cursor blocks before consumer invocation;
- compacted or intentionally incomplete agent context cannot bypass rehydrate;
- the inventory accounts for every explicit eligible source and records why
  each source is selected, reference-only, omitted, stale, unavailable,
  unauthorized, inapplicable, superseded, or conflicting;
- the source-coverage contract names every collector class required for the
  purpose and distinguishes successful empty results, skipped optional
  collection, access denial, collector failure, and unsupported collection;
- the same validated inputs produce byte-stable canonical inventory and bundle
  commitments independent of discovery order;
- current YAML cannot replace an older immutable definition bound to an active
  run, while the current YAML remains eligible for a future run;
- event-ledger state outranks a stale generated summary for dynamic run posture;
- operating docs can supply guidance and constraints but cannot grant approval,
  capability, or execution authority;
- a provider observation can establish remote resource posture but cannot
  authorize provider mutation;
- required context cannot be omitted because of a materialization budget;
- optional omitted context remains visible with a stable reason;
- conflicting facts in one authority domain block when required and remain
  visibly advisory when optional;
- unauthorized sensitive context is not materialized and cannot be summarized
  by the model as though it were available;
- unauthorized source identity, locator, class, revision, commitment, and exact
  counts are absent from consumer-visible inventory projections;
- collector failure is distinguishable from source absence;
- arbitrary unallowlisted repository files are not discovered or loaded;
- no raw conversation, hidden reasoning, credential, environment value,
  provider payload, or unbounded source content appears in inventory, bundle,
  logs, errors, events, reports, or debug output;
- a bundle built for one actor, purpose, run root, event cursor, or handoff is
  rejected for a different binding;
- a replacement agent with no conversation reaches the same disposition as an
  agent with full conversational memory from the same authoritative inputs;
- the replacement agent reconstructs the same bounded objective, accepted
  amendments and decisions, progress, remaining obligations, blockers,
  uncertainty, validation/reporting commitments, and lawful next material
  action without repeating completed work or reviving superseded direction;
- every material understanding claim cites inventory provenance and remains
  authoritative, supported, advisory, conflicting, unknown, or withheld rather
  than being silently completed by the model;
- a trusted compaction signal invalidates prior orientation and forces a fresh
  projection plus exact delivery acknowledgment before material continuation,
  while absent signaling remains safe through unconditional material-boundary
  rehydration;
- replacement and original executors receive the same canonical understanding-
  projection commitment for the same authorized source set and purpose;
- approval granted after brief creation requires fresh resolution;
- approval denial, expiry, revocation, or supersession blocks;
- authority revocation after child launch blocks the child operation;
- changed policy, check, evidence, sensitivity, or SideEffect posture
  invalidates the earlier continuation;
- completed, canceled, failed, or superseded runs reject continuation;
- a cursor change after the durable claim but before consumer invocation burns
  the stale claim and requires a fresh cursor-bound claim;
- parallel agents cannot consume the same operation authorization twice;
- child results require exact typed handoff identity and parent binding;
- natural-language-only child summaries are not accepted as governed results;
- unrelated definition changes do not churn the immutable execution binding;
- relevant definition changes cannot alter an active immutable bundle;
- low-risk quiet refresh remains auditable without human interruption;
- debug, serialization, errors, events, and reports do not leak forbidden
  content;
- existing executor, approval, immutable-run, proportional-governance,
  authority, required-context, typed-handoff, hook, SideEffect, and WorkReport
  tests remain green.

## 17. Acceptance Criteria

- Current kernel state, not model memory, determines the next material action.
- One complete bounded inventory explains every eligible source and its
  authority, provenance, freshness, sensitivity, access, and disposition.
- Consumer-visible inventory is a separately authorized, redacted projection;
  internal completeness never implies disclosure of protected source existence.
- One purpose-bound bundle contains all required permitted context, explicitly
  records optional omissions, and blocks on unresolved required gaps.
- Authority precedence is defined per source domain and covered by tests; no
  global latest-text or latest-file rule exists.
- Active-run immutable inputs cannot be replaced by subsequently edited YAML.
- Context inventory and bundle reuse existing stores, identifiers, access
  controls, required-context contracts, and same-call authority boundaries.
- Inventory and bundle serialization cannot authorize execution.
- Context-free agent replacement and induced compaction produce the same kernel
  disposition and equivalent bounded understanding from the same authoritative
  sources.
- Equivalent bounded understanding covers objective, accepted decisions,
  progress, remaining obligations, blockers, uncertainty, validation/reporting
  commitments, and lawful next options with claim-level provenance.
- Material accepted intent that would otherwise exist only in conversation is
  durably represented as a typed, attributable, non-authorizing semantic record
  before the full-understanding proof may pass.
- A purpose-bound source-coverage result proves which collectors were required,
  invoked, unavailable, denied, unsupported, or intentionally omitted.
- Trusted host compaction or executor-replacement signals trigger immediate
  orientation invalidation, refresh, and exact projection-delivery
  acknowledgment; material-boundary rehydration remains mandatory when no signal
  exists.
- Every selected material consumer has a fresh same-call rehydration boundary.
- The selected operation has a durable cursor-bound single-consumption claim.
- The continuation brief is explicitly non-authoritative.
- Missing or stale context fails closed before consumer invocation.
- Subagent launch and result acceptance use exact typed handoff identity.
- Compaction detection is optional observability, not a correctness dependency.
- Low-risk work can remain quiet under proportional governance.
- No provider mutation broadening, nested harness runtime, schema expansion, or
  hosted claim is introduced by the first slice.

## 18. Open Questions

- Which additional next-action code should follow
  `invoke_current_step_skill`, once another concrete consumer is approved?
- Should later event-position versions replace the first slice's last sequence
  plus last event ID with a versioned aggregate commitment?
- Which existing dogfood `BeforeSkillInvocation` step is the safest first
  concrete consumer?
- Should the read-only CLI preview return one strict next action or a bounded
  set of independently eligible actions?
- How should parent and child completion race without granting either ambient
  mutation authority?
- Which agent platforms expose lifecycle hooks useful for observability, while
  preserving unconditional kernel resolution as the portable invariant?
- Which allowlisted operating documents belong in the first local collector:
  `AGENTS.md`, engineering standard, active roadmap item, accepted plan, and
  current report only, or a smaller purpose-specific subset?
- Which repository observations should be promoted from bounded metadata to
  typed governed-context reference kinds rather than represented as collector
  metadata?
- What materialization budget model is deterministic across models while never
  allowing required context to disappear?
- When two authoritative sources disagree because one has not yet observed the
  other's transition, which domains permit deterministic refresh and which
  require explicit reconciliation?
- Which bundle fields are safe for default human output, and which belong only
  in bounded JSON because they reveal sensitive repository topology?

## 19. Final Recommendation

The first atomic local consumer and its read-only preview are implemented and
focused-review accepted. The registered-current-authority-backed consumer is
now implemented as one crate-private composition of fresh source resolution,
exact required-context consumption, the durable cursor-bound continuation
claim, and the current local skill invocation. It awaits focused
maintainer/security review and does not create trusted runtime source
configuration or a public authority API.

Do not treat `next-action` output as permission, add typed child-handoff runtime
behavior, or resume broader provider mutation or nested harness execution until
the separately reviewed source-backed continuation boundary exists.

Treat comprehensive provenance-aware context rehydration as the next P0
hardening layer for autonomous continuation. Begin with the pure inventory and
bundle decision model, not persistence or broad collectors. Preserve the
existing same-call authority boundary as the only path to material execution.
Do not claim compaction-safe or cross-agent autonomous continuation until the
context-free replacement-agent proof and focused adversarial review pass.
