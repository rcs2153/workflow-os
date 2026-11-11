# Durable Authority Continuation And Proof-Carrying Work

## Decision And Status

Status: deferred strategic direction, recorded through governed planning;
not an accepted ADR, implemented protocol or production capability.

Extend the continuity-of-authority kernel toward vendor-neutral durable
authorization, continuation, reconciliation and provenance across independently
operated executors. This is an elevation of the existing architecture, not a
pivot into an orchestrator or a replacement for execution frameworks.

Internal working names are Authority Continuation Profile and Governed Action
Envelope. Naming needs review. Do not rename the product or announce a standard.
The future thesis is that intent, scoped authority, execution truth and lawful
continuation survive the original agent, process, credential and call chain.
If used, "settlement" means reconciliation and terminal proof, not financial
settlement.

The [Active Delivery Queue](../../ROADMAP.md#active-delivery-queue) remains the
only forward-work authority. Complete its P0 hosted recovery, protected-host,
custody and cross-harness gates first. Phase 1 below is the first strategic
planning phase after that complete exit gate unless a higher-severity
correctness issue preempts it. Later phases require accepted prior exits and
their own governed scope; this plan grants no provider-call right.

## Existing Foundation And New Work

The kernel already has procedure and invocation authority foundations, immutable
run/action/resource commitments, policy/check/evidence/approval obligations,
current-fact reassessment, execution windows, SideEffect identity, idempotency,
uncertainty classification, reconciliation, execution truth, continuation
derivation and WorkReport foundations. Their assurance is bounded by each
implemented public path and its evidence. Their existence does not establish
all the integrations or trust properties below.

New work includes a portable public abstraction, canonical envelope, signature
and provenance profile, external identity and authorization-context validation,
explicit federation, verifier tooling and interoperability conformance.

Keep these existing contracts connected:

- [Cross-harness milestone](cross-harness-authoritative-continuation-milestone-plan.md)
  and [live sandbox admission](live-sandbox-continuation-admission-plan.md).
- [Current product contract](../user-guide/current-product-contract.md).
- [Side-effect boundary ADR](../adr/0011-side-effect-boundary.md).
- [Evidence reference](../concepts/evidence-reference.md) and
  [WorkReports](../concepts/work-reports.md).
- [Reasoning Lineage ADR](../adr/0008-reasoning-lineage-claim-graph.md).
- [Threat model](../security/THREAT_MODEL.md) and
  [V1 readiness criteria](../release/V1_READINESS_CRITERIA.md).
- Composable Harness Contracts, scoped runtime authority and proportional
  governance: profiles must consume these boundaries, not invent parallel
  approval, delegation or execution semantics.

## Core Invariant

A replacement, without the original transcript or transferable provider
authority, authenticates independently and asks the kernel for the current
lawful action. It must learn the exact governing procedure/revision, initiating
principal and delegation, permitted replacement, action/target/request/limits,
required policy/check/evidence/presentation facts, current validity, effect
state, and terminal proof belonging to that operation.

**The portable record is not a bearer credential.** Possession, a valid
signature, transport delivery or a historical approval never grants execution.
Resolve current principal authority, attenuation, revocation, policy, time,
evidence, checks and execution state at admission. The atomic admission boundary
is the linearization point for any one-use executable right; that right remains
non-serializable. No provider material, ambient secret or reusable capability
belongs in the envelope. A summary cannot confer authority.

Offline verification proves historical commitments under explicitly selected
trust roots and evidence availability. It cannot prove current revocation or
authorize a new effect without current authority resolution. Expose that limit
rather than treating offline signature validity as permission.

## Complementary Standards

Primary sources inspected during planning; draft versions and mutable sources
must be pinned and re-reviewed in the ADR. These are integration candidates,
not claims that other projects omit equivalent capabilities or endorse us.

| Surface | Candidate boundary |
| --- | --- |
| [A2A specification](https://a2a-protocol.org/latest/specification/) | Discovery, task/message/artifact exchange and task lifecycle; delivery is not kernel admission. |
| [MCP authorization, 2025-06-18](https://modelcontextprotocol.io/specification/2025-06-18/basic/authorization) | OAuth-protected tool-server access; transport authorization is not durable work authority. |
| [SPIFFE overview](https://spiffe.io/docs/latest/spiffe-about/overview/) | Workload identity, trust domains and attestation; identity alone does not authorize an action. |
| [OAuth Transaction Tokens draft](https://datatracker.ietf.org/doc/draft-ietf-oauth-transaction-tokens/) | Live call-chain identity and authorization context within a trusted domain; re-resolve durable work authority. Inspected revision 11, not an RFC claim. |
| [Transaction Tokens For Agents draft](https://datatracker.ietf.org/doc/draft-araut-oauth-transaction-tokens-for-agents/) | Agent-chain context and security considerations; evaluate attenuation, identity laundering and isolation. Inspected revision 03. |
| [Intent Token draft](https://datatracker.ietf.org/doc/draft-chen-oauth-agent-intent-token/) | Delegated intent context distinct from specific grants; interpretation is not execution permission. Inspected revision 00. |
| [AP2 authorization](https://ap2-protocol.org/ap2/agent_authorization/) | Verifiable mandate chains and action authorization; evaluate one import profile, not a payment rail. |
| [SLSA provenance](https://slsa.dev/spec/v1.2/provenance) and [in-toto](https://in-toto.io/) | Provenance and attestation carriers; do not claim SLSA level compliance or signed truth merely by adopting a format. |

Our architectural hypothesis is complementary: task transport, workload
identity, authorization context, runtime scheduling and attestations can feed
durable work-level authority and reconciliation. Test that hypothesis; do not
market an exclusive standards gap as an established fact.

## Applications And Limits

1. Bounded mandates beyond payments: cloud changes, procurement, legal
   submissions, healthcare administration, data release and incident response.
   Domain rules and evidence need independent validation; no safety-critical
   certification or regulatory compliance is implied.
2. Proof-carrying work for code, documents, infrastructure, investigations and
   decisions, binding inputs, approvals, checks, effects, ambiguity and outcomes.
3. Cross-vendor replacement among agents, deterministic workers and humans,
   without carrying transcripts as permission or distributing blanket access.
4. Post-effect/pre-acknowledgement integrity: missing acknowledgement never by
   itself permits a retry. Insufficient observation preserves uncertainty.
5. Purpose-bound data use: bind access and downstream use to declared purpose
   and resulting work, not identity-wide entitlement. Minimize disclosed data.
6. Third-party agent contracting, disputes, warranties and insurance are
   strategic optionality only, not near-term features or legal guarantees.

## Threats And Assurance

The ADR must address confused deputy and identity laundering, wrong audience or
trust domain, delegation expansion, stolen/replayed envelopes, mutable targets,
TOCTOU at admission, stale/forged/revoked authority, lease and fencing races,
duplicated dispatch after ambiguous commit, receipt substitution, report
mix-and-match, compromised issuer/verifier, key rotation, clock rollback,
restored or forked state, malicious evidence references, availability loss and
sensitive metadata disclosure. Define attacker powers and trusted components,
not just negative examples.

Signatures bind statements to an issuer; they do not make a host or provider
honest. Disclose operator-controlled storage and host attestations separately
from independently obtained provider observations. Conflicting observations
remain unresolved until an explicit reconciliation rule applies. Federation
requires configured trust roots, issuer/audience constraints and revocation
policy; do not infer trust from an identity string or another domain's success.

## Phase 0: Complete Current Continuity Foundation

Finish the active queue's hosted creation/result interruption recovery,
protected-host preflight, receiver custody, authenticated distinct-principal
continuation, one bounded real provider effect, post-effect/pre-acknowledgement
recovery, independent reconstruction and product/release reconciliation.

Exit: replacement does not rely on conversational authority; current facts are
re-resolved; uncertainty cannot fabricate success or enable blind retry; one
accepted effect is reconciled without duplication; an independent reviewer
can reconstruct the outcome. The accepted synthetic and no-dispatch proofs
remain bounded evidence, not substitutes for the real effect or human review.

## Phase 1: Architecture Decision

Produce a focused ADR, implementation plan and complete threat model. Map
existing procedure/invocation/execution truth models to the profile; specify
trust domains, revocation/expiry, atomic admission, one-use rights, ambiguity,
privacy/redaction, compatibility, and the standards boundaries above. Explain
how WorkReports, EvidenceReferences, SideEffects, audit and provider receipts
relate without making report prose authoritative.

Compare an embedded harness plugin, task sidecar, local daemon, and remote
policy decision point or gateway. For each, document which component owns
canonical state, what the workload and cell must trust, how policy and
version compatibility are checked, and what happens on revocation, network
partition, upgrade, and rollback. Central authority means one canonical
decision and execution record, not a mandatory centralized hosting model.

A task cell may enforce finite delegated scope, mediate governed calls and
return attributable, operation-bound observations. It must not mint,
expand, renew, reinterpret or independently persist executable authority.
Only the canonical kernel may resolve and admit new consequential actions.
The harness remains responsible for execution and its VM, container or
sandbox remains responsible for isolation. A governed gateway owns any
provider access needed for the supported action; a wrapper alone cannot
close alternate credential, tool or egress paths.

Non-goals: generic orchestration, identity provider, OAuth server, payment
rail, sandbox, agent framework, blockchain requirement, private chain-of-thought
capture, production claim or assumed cross-domain trust.
Defer an optional first-party delegator, objective decomposition, automatic
harness selection, spawning and recursive delegation until independent
executors can receive, exercise, surrender and return finite authority.
Any delegator would sit above Core, not become its authority source.

Exit: accepted maintainer/security architecture review and a stable distinction
between portable proof and executable authority. Owner review must be labeled
as such; independent assurance cannot be inferred.

## Phase 2: Canonical Continuation Envelope

Define versioned deterministic commitments for procedure/revision; run, step,
operation and SideEffect; initiator, delegation references, replacement, issuer,
audience and domain; exact action/target/request/input/limits; policy revision
and approval-presentation proof; evidence/check commitments and assurance;
window/time posture; idempotency/reconciliation identity; effect and next-action
classifications; limitations and terminal provenance references.

Represent not-admitted, admitted, dispatch-unknown, externally observed,
completed, denied, failed and reconciliation-required postures without
collapsing external observation into kernel completion. A serialized next action
is historical or advisory until freshly resolved, never portable permission.

Design DSSE/in-toto carriage without making Core correctness depend on one
signature or transport. Define canonical encoding, domain-separated hashes,
size/cardinality bounds, unknown-field/version behavior, verification inputs
and content-addressed evidence access.

Exit: deterministic serialization/hashing/verification and adversarial
substitution, replay, mix-and-match, stale-state, wrong-run/target/principal,
revoked-authority and expired-window tests. No secrets or executable rights
may be serialized.

## Phase 3: One Identity And One Transport Profile

Select one bounded identity profile, preferably SPIFFE/SPIRE or an explicitly
justified signed workload identity. Separately choose one external context
import from Transaction Token, Intent Token or AP2-style mandate work.
Select A2A or MCP as one initial handoff transport.

Verify external context, bind its commitment to the run, authenticate the
replacement and independently resolve current kernel authority. The receiving
executor gets neither state-store access nor the prior transcript.

Exit: forged/replayed/wrong-audience/expired context and delegation beyond
parent scope fail closed. Transport success cannot manufacture admission.
Pin draft revisions, trust configuration and interoperability test vectors.

## Phase 3A: Reference Enforcement Plane

After accepted Phase 1 through Phase 3 exits, prove one task-scoped cell, one
governed credential/tool gateway, one bounded provider integration and one
public versioned protocol. Use the topology selected by the ADR, not four
implementations. The executor receives no reusable provider credential or
direct state-store access. At the supported boundary, the gateway obtains a
fresh decision and exercises only an exact, one-use admission. The kernel
retains action identity, ambiguity, reconciliation and reporting posture.
Cell observations are bound to the operation and distinguishable from
provider evidence. Do not add a generic gateway, scheduler or sandbox.

Exit: prove both the admitted path and denial when current authority cannot
be resolved. Preregister expected results for stale policy, revocation during
disconnection, expired or consumed leases, duplicate cells and replacement
executors, forked or restored state, replayed mandates, wrong audience or
domain, network partition, clock rollback, conflicting provider observations,
and interruption after acceptance but before acknowledgement. Unavailable
authority denies new effects. Insufficient or contradictory observations
preserve uncertainty without a resend. Neither a signed envelope nor cached
policy may become a bearer credential. Record distinct results for initial
admission, recovery and terminal reporting.

## Phase 4: Cross-Host Consequential-Action Proof

Preregister a bounded remediation simulation. A human issues an exact mandate;
one harness reaches approval; a separately authenticated harness or Linux
workload replaces it through the public profile. Re-resolve delegation, policy,
evidence, checks, time, request, target and idempotency facts. Admit one sandbox
cloud-style action or existing GitHub mutation. Kill the worker after provider
acceptance but before success persistence. A third executor must observe and
reconcile the exact effect without resending it.

A blind verifier receives only exported proof and provider history, not private
database access or transcript authority. The trial must disclose any custom
integration and operator action, trust assumptions and unresolved evidence.

Run the same governed task across two materially different compute substrates
and two independently implemented harnesses. A shared kernel and protocol are
expected; two configurations of the same harness or two copies of one host
image do not prove these properties. The replacement independently
authenticates and recovers from kernel-owned state without the original
transcript or transferable provider authority.

Exit: zero unauthorized or duplicate accepted effects, zero transcript-derived
authority, correct decisions at every preregistered fault point, and explicit
unresolved posture for insufficient provider observation. Measure recovery
turns, latency, lookups, tokens, custom integration code, operator overhead
and verifier success. Before a distributed authority-plane claim, require one
real bounded consequential effect, verified revocation and expiry, successful
post-effect reconciliation without blind retry, and independent reconstruction
from public proof surfaces. Synthetic and no-dispatch evidence may qualify
the protocol but cannot close the real-effect gate.
Do not promise exactly-once provider execution absent its own contract.

## Phase 5: Terminal Work Provenance And Verifier

Create a signed terminal artifact separate from operational storage, bound to
the exact continuation envelope, operation and terminal state. Cite sensitive
evidence rather than copying it. Disclose assurance, limitations and denied,
skipped, attempted, failed, completed and ambiguous effects. Include offline
verification where supported and a small verifier CLI/library.

Develop minimal verifier test vectors early enough for Phase 4; Phase 5 delivers
the supported terminal profile and tool, not a circular prerequisite. Reasoning
Lineage can enrich claim/evidence links later. Base provenance must not wait for
it or capture private chain of thought.

Exit: independent verification rejects wrong-run reports, wrong-target receipts,
altered bodies, missing required observations, stale approval and broken
delegation, while valid proof verifies under explicit trust configuration.
Neither signature nor WorkReport prose replaces execution truth.

## Phase 6: Narrow Domain Pilots

After the generic proof, run cloud/incident remediation with shift replacement,
procurement with vendor/price bounds and escalation, and purpose-bound data
release with downstream-result binding and expiry/revocation. Evaluate AP2
compatibility only where applicable. Keep Core semantics unchanged; domain
rules belong in bounded profiles/adapters.

Exit: independent domain review and measured adoption/adapter effort,
investigation time saved, assurance limits and recovery correctness. Do not
build broad packs before these pilots establish common semantics.

## Phase 7: Open Profile And Standards Engagement

Only after the proofs, publish implementation-neutral terminology, threat
model, canonical envelope, conformance levels, identity/transport/mandate
profiles, verifier behavior, test vectors, privacy and ambiguity rules.
Engage the relevant A2A, MCP, SPIFFE/WIMSE/OAuth, AP2 and in-toto/SLSA communities
through complementary contributions and profiles, not incompatible replacements.
Do not claim endorsement or standard status.

## Delivery And Claims

Current public language remains an advanced local governance and continuity
preview. Packaged cross-harness interoperability, production cross-domain
delegation, cryptographic work provenance and a generalized protocol are not
current claims. Reconcile the product contract and V1 criteria through separate
governed changes only when accepted public-boundary evidence supports them.

Use three future deployment postures, separate from proportional governance:

- **Observe:** record available decisions and actions. Bypass may remain
  possible; the record is not proof that all consequential work was mediated.
- **Enforce:** require the governed gateway for consequential actions within
  the documented integration boundary. Name the actions and disclose paths
  outside that boundary.
- **Strict:** additionally verify that workloads have no ambient provider
  credentials or unrestricted consequential tool or egress paths in the
  declared environment. If the controls cannot be verified, withhold the
  Strict claim. A cooperative wrapper or audit-only adapter is not sufficient.

Claim only what can be shown: supported consequential actions are admitted,
authorized, identified, reconciled, and reported at the declared boundary.
Do not claim global mediation, non-bypassability outside the verified boundary,
or that every tool call is recorded.

This plan succeeds only when a replacement independently authenticates without
the original transcript or transferable provider authority, resolves current
permission, recovers exact execution truth without duplicating an uncertain
effect, and exports proof an independent verifier binds to the exact mandate,
operation, provider outcome and work product.

Planning run `run-1794367438224102000-2` has proof-backed approval
`approval/run-1794367438224102000-2/planning-approved`. Source inspection,
primary-source reading, documentation edits and checks are executor work outside
the preview kernel handlers. This record is not independent assurance.

The enforcement-plane additions follow read-only review
`run-1794379401146053000-2` and docs run `run-1794379495160632000-2`, with
proof-backed approvals at `review-scope-approved` and `planning-approved`,
respectively. They are deferred acceptance criteria, not an ADR decision,
implementation approval or a change to Phase 0.
