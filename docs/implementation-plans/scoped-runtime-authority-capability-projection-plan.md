# Scoped Runtime Authority And Capability Projection Plan

Status: The capability-grant and availability core model is implemented in
[Capability Grant And Availability Core Model Report](../concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_REPORT.md).
Its availability source-of-truth blocker is fixed in
[Capability Grant And Availability Core Model Blocker Fix Report](../concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_BLOCKER_FIX_REPORT.md)
and accepted in
[Capability Grant And Availability Core Model Blocker Fix Review](../concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_BLOCKER_FIX_REVIEW.md).
Runtime resolution, capability requests, projections, authority receipts,
enforcement, schemas, CLI behavior, connectors, provider writes, hosted
administration, and enterprise identity integration remain unimplemented.

Related foundations:

- [SideEffect Boundary Core Model](../adr/0011-side-effect-boundary.md)
- [Composable Harness Contract Plan](composable-harness-contract-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [High-Assurance Approval Controls Plan](high-assurance-approval-controls-plan.md)
- [EvidenceReference](../concepts/evidence-reference.md)
- [Governed Work Pattern](../concepts/governed-work-pattern.md)

## 1. Executive Summary

Workflow OS already governs workflow state, policy decisions, approvals,
evidence, reports, and selected SideEffect authority. The next authority-layer
question is broader than whether one proposed mutation was approved: which
actor or harness may discover, receive, and invoke which capability against
which resource and context for the current step, and what durable proof should
travel with an authorized invocation.

The future product invariant is:

```text
Expose only the capabilities and context authorized for the current governed
step. Treat discovery, visibility, authority, approval, invocation, and outcome
as distinct states. Carry stable authority proof into consequential actions.
```

This plan defines a sequence for scoped capability grants, explicit capability
availability, step-scoped tool projection, governed context projection, and
bounded authority receipts. It preserves Workflow OS as a governed work
runtime. It does not make Core an agent framework, general memory platform,
hosted control plane, connector marketplace, or multi-agent orchestrator.

## 2. Why This Belongs In Workflow OS

An agent instruction that says "do not use this tool" is not an authority
boundary. A runtime that exposes every available capability and rejects only
some invocations still increases prompt surface, accidental misuse risk, and
ambiguity about what the actor was authorized to see.

Governed work needs deterministic answers to separate questions:

- Does the capability exist?
- Is it connected and operational?
- Is it visible to this actor or harness?
- Is it granted for this workflow, run, step, and resource?
- Does invocation require approval or additional evidence?
- Is the grant current, expired, or revoked?
- Was an invocation attempted, completed, denied, or left ambiguous?
- What stable authority record can a downstream adapter verify?

Workflow OS already owns the adjacent sources of truth. Policy evaluates rules;
approval records deliberate authorization; SideEffect records mutation intent
and outcome; EvidenceReference cites proof; WorkReport discloses governed work;
and Composable Harness Contracts will define bounded execution envelopes. A
capability authority layer should compose these primitives rather than create a
parallel permission system.

## 3. Goals

- Model scoped, actor-bound, resource-bound capability grants.
- Make grant issuance, expiry, revocation posture, and delegation constraints
  explicit and auditable.
- Distinguish capability existence, connection, visibility, authorization, and
  executable readiness.
- Project only authorized tool vocabulary into a future harness or execution
  step.
- Govern context and evidence visibility as carefully as tool execution.
- Let missing capabilities produce explicit request or remediation posture
  instead of fabricated identifiers or silent substitution.
- Define a bounded authority receipt that can accompany selected downstream
  invocations without copying sensitive payloads.
- Integrate deterministic authority facts with proportional governance.
- Preserve local-first operation and provider independence.
- Prepare for later enterprise stewardship without requiring hosted
  administration in Core.

## 4. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- a generic MCP gateway or connector marketplace;
- automatic connector installation or credential discovery;
- a transcript, vector-memory, RAG, or company-knowledge platform;
- agent teams, recursive agents, agent swarms, or arbitrary delegation;
- default tool execution or automatic local command execution;
- new provider-write families or broader mutation defaults;
- workflow schema or SDK changes;
- hosted control-plane, tenant, UI, RBAC, IdP, OIDC, SSO, SCIM, or SIEM work;
- cryptographic signing, notarization, or distributed receipt verification;
- replacement of policy decisions or approvals with model judgment;
- Level 3 or Level 4 autonomy defaults;
- release posture changes.

## 5. Source-Of-Truth Boundaries

| Concept | Source of truth | Must not be confused with |
| --- | --- | --- |
| Capability definition | Validated capability registry or contract | Permission to use it |
| Capability availability | Current bounded runtime/adapter inventory | Authorization or successful connectivity forever |
| Capability grant | Durable scoped authority record | Approval decision, policy decision, or execution outcome |
| Policy decision | Policy engine result for requested action/context | Durable grant or completed action |
| Approval decision | Human or delegated decision at a specific gate | General capability ownership |
| Step projection | Derived set of visible tools/context for one step | Global registry or ambient authority |
| SideEffect | Proposed/attempted external mutation and lifecycle | Capability grant or approval |
| EvidenceReference | Citation pointer | Payload access authorization |
| Authority receipt | Bounded proof references for an invocation | Raw approval, policy, provider, or evidence payload |
| WorkReport | Governed terminal handoff | Permission source or audit-event replacement |

These boundaries are mandatory. A granted approval does not imply a reusable
capability grant. A capability grant does not prove policy passed for every
invocation. A visible tool is not necessarily executable. A completed
SideEffect does not prove that future repetitions are authorized.

## 6. Candidate Core Contracts

The first model phase should add only types justified by concrete validation
rules. Candidate vocabulary includes:

### Capability Grant

- `CapabilityGrant`
- `CapabilityGrantId`
- subject or principal reference;
- capability reference;
- resource scope;
- workflow, run, step, or harness scope;
- issuer reference;
- issued-at and optional expiry posture;
- revocation posture and optional revocation reference;
- delegation posture and maximum delegation depth;
- required policy, approval, evidence, or check references;
- sensitivity ceiling;
- redaction metadata.

The model must not store credentials, tokens, raw policy payloads, raw evidence,
provider responses, arbitrary source contents, or unrestricted metadata maps.

### Capability Availability

Use a small deterministic inventory/connectivity taxonomy rather than one
boolean:

- `available`;
- `declared_not_connected`;
- `known_unsupported`;
- `unknown`.

Authorization, denial, expiry, and revocation are resolution outcomes derived
from grants and independent decisions; availability records must not assert
them. Unknown and unsupported states must remain fail-closed. Catalog
visibility must never imply execution authority.

### Step-Scoped Capability Projection

A projection should contain only stable capability/tool identifiers plus
bounded decision references. It should be derived from validated workflow step,
policy, grant, actor, resource, approval, and runtime availability inputs.

It must not load tools, connect adapters, discover credentials, execute
commands, invoke providers, or mutate runtime state. The first implementation
should be a pure deterministic helper.

### Governed Context Projection

Context authority should describe:

- allowed source/reference kinds;
- resource and repository scope;
- sensitivity ceiling;
- reference-only, summary, or payload-access posture;
- required redaction posture;
- evidence dereference posture;
- retention and report-disclosure posture.

An `EvidenceReference` being citeable must not automatically make its target
readable by every actor. Raw prompts, transcripts, provider payloads, source
contents, command output, environment values, and credentials remain excluded
unless a separately governed capability explicitly permits bounded access.

### Authority Receipt

A future `AuthorityReceipt` should be a payload-free envelope of stable
references, potentially including:

- receipt identity and version;
- workflow, run, step, and harness identity;
- actor/principal reference;
- capability grant reference;
- resource scope commitment;
- policy decision reference;
- approval decision and presentation-proof references when required;
- SideEffect reference for mutations;
- evidence/check posture references;
- issued-at, freshness, and expiry posture;
- correlation and audit-event references;
- redaction metadata.

The first receipt must be local and unsigned. Cryptographic signatures,
cross-service verification, notarization, and distributed trust roots require
separate threat modeling and planning.

## 7. Capability Request Posture

When required authority is missing, the runtime should return a structured
posture rather than inventing a tool, widening access, or silently selecting a
substitute. A future request record should identify:

- requested capability and bounded purpose;
- requesting actor, workflow, run, and step;
- requested resource scope;
- availability posture;
- why current authority is insufficient;
- required steward, policy, approval, or connector action;
- expiration and review posture;
- explicit non-authority until granted.

Capability requests are not grants and must not activate connectors, attach
credentials, modify workflows, or resume blocked execution automatically.

## 8. Tool Visibility And Invocation

The long-term runtime path should be:

```text
authored step requirements
  -> current capability availability
  -> scoped grant resolution
  -> policy and proportional-governance decision
  -> step-scoped visible tool projection
  -> approval/evidence/check gate when required
  -> authority receipt
  -> invocation
  -> SideEffect/event/audit/report outcome
```

Tool visibility should be narrower than the global adapter or MCP inventory.
Invocation must revalidate authority; a stale projection must not become a
time-of-check/time-of-use bypass. The immutable run bundle must commit to the
relevant capability and context requirements before this path becomes an
enforcement default.

## 9. Relationship To Proportional Governance

Capability and context posture become deterministic inputs to the proportional
governance selector:

- available, read-only, adequately evidenced work may remain quiet;
- incomplete optional context may produce visible non-blocking disclosure;
- a consequential capability, elevated sensitivity, missing authority, or
  externally visible SideEffect may require blocking approval;
- unknown, revoked, expired, mismatched, or unsupported authority must deny.

Proportional governance may select the interaction mode. It must not mint a
grant, broaden a resource scope, revive expired authority, or convert unknown
capability posture into permission.

## 10. Relationship To Composable Harness Contracts

Composable Harness Contracts should eventually declare:

- required and optional capabilities;
- allowed tool identifiers;
- required context classes;
- resource and sensitivity scope;
- SideEffect classes;
- approval, evidence, check, and report obligations;
- delegation constraints;
- typed input and output handoffs.

Runtime projection then resolves the narrow set available for one harness
invocation. This is governed composition, not arbitrary agent spawning. Nested
harness execution remains deferred until contract, grant, context, handoff, and
receipt boundaries are independently accepted.

## 11. Security And Privacy Requirements

- Authority must fail closed on unknown, stale, revoked, expired, malformed, or
  mismatched state.
- Grant and receipt validation errors must use stable codes and omit raw
  caller-supplied values.
- Debug, Display, serialization, and deserialization errors must be
  redaction-safe.
- Resource scopes must be bounded and canonicalized without leaking private
  local paths by default.
- Grants must reference secrets; they must never contain credentials.
- Projection must be deterministic from explicit validated inputs.
- Runtime invocation must revalidate freshness and exact run/step/resource
  bindings.
- Approval denial must remain possible even when a grant exists.
- Revocation must be monotonic and auditable once enforcement is implemented.
- Delegation must be explicit, bounded, and disabled by default.

## 12. Operator Experience

The operator-facing decision trace should lead with human meaning:

```text
Capability                     Decision             Reason
Read pull request metadata     Quiet capture        Scoped read grant active
Publish pull request comment   Approval required    External write
Delete repository branch       Denied                Capability not granted
```

Technical references should remain available for inspection: workflow, run,
step, actor, grant, policy decision, approval, SideEffect, receipt, and audit
event IDs. Quiet success must still leave a durable record. Missing capability
posture must identify the next governed action without presenting a request as
already authorized.

## 13. Proposed Implementation Sequence

Do not begin this sequence until approval/resume resolved-context integrity and
the required immutable run-input boundary are accepted.

1. **Core capability-grant and availability model only. Implemented.** The
   validated, redaction-safe Rust types and focused tests are documented in
   [Capability Grant And Availability Core Model Report](../concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_REPORT.md).
   No runtime consumption was added.
2. **Pure capability resolution helper.** Resolve explicit definitions,
   availability, grants, actor, resource, and run/step scope without side
   effects.
3. **Capability request model and review-only projection.** Represent missing
   or insufficient authority without auto-granting or connector mutation.
4. **Pure step-scoped tool projection.** Produce authorized tool identifiers
   only; do not execute or alter provider payloads yet.
5. **Governed context-access model and projection.** Begin with references and
   safe metadata, not raw source or provider payloads.
6. **Local unsigned authority-receipt model.** Bind stable references and
   freshness posture without cryptographic claims.
7. **One opt-in harness or adapter integration.** Enforce projection and receipt
   on one reviewed local/read-only path before any write-path adoption.
8. **Selected provider-write adoption.** Only after read-only enforcement,
   TOCTOU, immutable-bundle, approval, SideEffect, and receipt reviews pass.
9. **Enterprise stewardship planning.** Add organization minimums, identity,
   shared grant state, revocation, and source binding only after local contracts
   stabilize.

Every implementation and review remains a separate governed phase.

## 14. Test Plan

Future phases should cover:

- valid narrowly scoped grants;
- invalid actor, capability, resource, issuer, expiry, and revocation posture;
- exact workflow/run/step/harness binding;
- expired and revoked grants fail closed;
- unknown and unsupported capability states fail closed;
- visibility does not imply authorization;
- authorization does not imply invocation success;
- approval and policy requirements remain independently enforced;
- projection contains only authorized tool identifiers;
- stale projection cannot authorize invocation;
- context sensitivity and dereference limits are preserved;
- capability requests cannot be used as grants;
- authority receipts contain references, not raw payloads;
- receipt freshness and resource mismatch fail closed;
- debug/serde/error non-leakage;
- deterministic ordering and serialization;
- proportional-governance escalation is monotonic;
- SideEffect, EvidenceReference, WorkReport, approval, policy, and runtime
  regressions continue to pass.

## 15. Deferred Enterprise Layer

Later enterprise stewardship may add:

- shared durable grant and capability catalogs;
- organization, department, team, repository, and workflow minimums;
- RBAC/IdP principal resolution;
- OIDC/SSO/SCIM integration;
- role-bound issuers and separation of duties;
- centralized revocation and emergency kill switches;
- source-bound context policies and quarantine review;
- immutable or cryptographically verifiable receipt chains;
- SIEM/audit export;
- tenant and resource isolation.

None of these are Core v0 claims. Local model quality, deterministic enforcement,
restart safety, and inspectable evidence must precede hosted administration.

## 16. Final Recommendation

The next implementation prompt should be the **pure capability resolution
helper**. It should consume only explicit validated
definitions, availability records, grants, actor, resource, and run/step scope
and return a deterministic decision without runtime mutation or side effects.

Do not build capability requests, tool/context projection, tool execution,
provider writes, connector installation, memory infrastructure, agent teams,
hosted administration, enterprise identity, authority receipts, cryptographic
claims, or schema exposure in that next phase.
