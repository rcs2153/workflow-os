# Governed Context Access Projection Plan

Status: Core model and pure step-scoped projection helper implemented and
accepted with non-blocking follow-ups in the
[focused maintainer review](../concepts/GOVERNED_CONTEXT_ACCESS_PROJECTION_REVIEW.md).
Payload dereference, runtime enforcement, persistence, events, receipts,
schema, SDK, CLI, provider, sandbox, SideEffect execution, and write behavior
remain unimplemented.

Related foundations:

- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Governed Context Access Projection Review](../concepts/GOVERNED_CONTEXT_ACCESS_PROJECTION_REVIEW.md)
- [Step-Scoped Capability Projection Review](../concepts/STEP_SCOPED_CAPABILITY_PROJECTION_REVIEW.md)
- [EvidenceReference](../concepts/evidence-reference.md)
- [Typed Handoff Plan](typed-handoff-plan.md)
- [Composable Harness Contract Plan](composable-harness-contract-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Audit Redaction](../security/audit-redaction.md)

## 1. Executive Summary

Workflow OS can now resolve scoped capability authority and project authorized
capability references for one workflow step. The next authority boundary is
context: which stable references may be exposed to an actor or harness for the
current step, at what access level, and with which sensitivity and redaction
constraints.

The future invariant is:

```text
A reference being known, citeable, or present in a handoff does not make its
target readable. Project only the context references authorized for the current
step, and re-resolve authority before any later dereference.
```

The first implementation should remain a pure model/helper slice. It should
project typed stable references and bounded enumerated metadata only. It should
not read source files, evidence targets, reports, event payloads, transcripts,
provider responses, command output, memory systems, or external services.

The planning document now records the implemented model boundary. It still does
not authorize any target dereference or runtime consumer.

## 2. Goals

- Define a domain-neutral governed context-reference model.
- Distinguish reference visibility from payload access.
- Reuse scoped capability authority rather than create a parallel permission
  system.
- Bind context projection to one actor, workflow, run, step, optional harness,
  evaluation time, and sensitivity ceiling.
- Represent reference-only and bounded-metadata access explicitly.
- Project only references backed by fresh, exact-context authorized capability
  resolutions.
- Preserve deterministic ordering, validated serde, and redaction-safe Debug.
- Make missing, unavailable, unauthorized, or sensitivity-incompatible context
  explicit without fabricating references.
- Prepare for later typed handoff, WorkReport, Composable Harness Contract, and
  sandbox execution integration.
- Preserve local-first and provider-neutral Core behavior.

## 3. Non-Goals

The planning and first model phase do not authorize:

- context, evidence, artifact, report, event, or source payload dereference;
- arbitrary source-file, repository, transcript, prompt, or memory access;
- raw provider payloads, command output, parser payloads, or environment values;
- credentials, authorization headers, private keys, or token material;
- vector databases, RAG systems, transcript stores, or company knowledge bases;
- tool loading, command execution, connector activation, or provider calls;
- filesystem mounts, network access, sandbox lifecycle, or OpenShell
  integration;
- SideEffect execution or provider writes;
- runtime consumption or time-of-use authorization;
- persistence, workflow events, audit projection, or authority receipts;
- schemas, SDKs, CLI behavior, UI, or workflow-spec fields;
- automatic connector discovery or ambient context inheritance;
- hosted administration, enterprise identity, DLP, or access-control systems;
- reasoning-lineage claims or edges;
- release posture changes.

## 4. Source-Of-Truth Boundaries

| Concept | Source of truth | Must not be confused with |
| --- | --- | --- |
| Context reference | Existing typed stable identity | Permission to read its target |
| Context availability | Explicit current inventory fact | Authority, freshness forever, or payload validity |
| Capability resolution | Current scoped authority evaluation | Payload, completed access, or durable receipt |
| Step context projection | Derived reference visibility for one exact step context | Global catalog, ambient access, or invocation authority |
| EvidenceReference | Citation pointer and bounded evidence posture | Automatic access to evidence target |
| Typed handoff | Governed transfer contract and stable references | Unbounded copied context or inherited authority |
| WorkReport citation | Governed terminal disclosure reference | Access grant or audit replacement |
| Audit/event record | Operational history and source-of-truth event posture | Context payload or reusable authorization |
| Future authority receipt | Bounded proof references for consequential use | Raw approval, policy, evidence, or context payload |

Context access must compose with capability authority. A separate context
projection must not override a denied, expired, revoked, wrong-scope, or
sensitivity-insufficient capability resolution.

## 5. Access-Level Model

The first implementation should support only two positive access levels:

- `reference_only`: the stable target identity and its safe type may be
  projected;
- `bounded_metadata`: the reference plus a fixed, typed set of safe metadata
  may be projected.

`bounded_metadata` must not be an unrestricted string map. Candidate metadata
is limited to fields with existing validation and privacy posture, such as:

- reference kind;
- owning workflow, run, step, or harness identity when already part of the
  typed target;
- creation or observation timestamp;
- sensitivity;
- redaction posture;
- declared availability;
- stable correlation or source-record reference.

The first implementation must not include summary text, snippets, paths, URLs,
titles, diagnostic messages, report section text, event payloads, or provider
metadata merely because they are bounded strings.

Each access level maps to one fixed Core-owned capability reference:

- `reference_only` requires `context.reference.view`;
- `bounded_metadata` requires `context.metadata.view`.

The mapping is exact. The first helper must not infer capability hierarchies,
wildcards, aliases, or inheritance between these operations.
`context.metadata.view` authorizes returning the target reference together with
the fixed metadata defined below; it does not authorize payload access.

The first bounded metadata record contains only:

- target kind;
- declared target sensitivity;
- availability observation timestamp.

Reference-only entries retain those fields internally for validation but their
public projected metadata posture remains `reference_only`. No arbitrary
metadata extension point is included.

Future summary or payload access must be separately planned. Adding an enum
variant must never by itself make dereference possible.

## 6. Candidate Reference Taxonomy

The first core taxonomy is limited to typed target variants backed by existing
validated identities:

- `EvidenceReference`;
- workflow event;
- audit event;
- validation diagnostic reference;
- approval decision;
- policy decision;
- SideEffect;
- typed handoff;
- WorkReport.

Adapter telemetry, local-check results, approval-presentation proofs, report
artifacts, immutable bundles, hook records, and future reasoning-lineage nodes
remain later variants because their current stable-reference or access
semantics need separate review. The implementation must not use a generic
string escape hatch for them.

Filesystem paths, URLs, provider object bodies, source snippets, command output,
and arbitrary opaque strings are not context targets. A future adapter may map
an external resource to a validated stable reference, but that mapping is a
separate governed boundary.

## 7. Candidate Core Model

The smallest justified first model set is likely:

- `GovernedContextReference`;
- `GovernedContextReferenceKind`;
- `GovernedContextAccessLevel`;
- `GovernedContextAvailability`;
- `GovernedContextProjectionCandidate`;
- `GovernedContextProjectionInput`;
- `GovernedContextProjectionEntry`;
- `GovernedContextProjection`;
- `GovernedContextProjectionGap`;
- `project_step_scoped_context(...)`.

Names should follow established repository conventions after implementation
inspection. Types should be omitted when the same invariant can be expressed
cleanly with an existing Core type.

The implementation should add `CapabilityResourceKind::ContextReference`.
Every `GovernedContextReference` must derive one canonical capability resource
reference as `<target-kind>/<stable-id>`. The derivation is Core-owned,
deterministic, bounded by the existing capability resource limits, and contains
no raw path or URL. A supplied capability resolution must use that exact
resource kind and derived reference.

### Governed Context Reference

A reference should contain:

- one typed stable target;
- declared sensitivity;
- redaction metadata;
- explicit availability;
- no raw payload or unrestricted metadata.

### Projection Input

Input should contain:

- actor;
- workflow;
- run;
- step;
- optional harness contract;
- evaluation timestamp;
- maximum allowed sensitivity;
- requested access level;
- complete evaluated candidate records;
- redaction metadata.

It must read no hidden global state.

### Projection Candidate

Every candidate should retain:

- the typed stable context reference;
- declared availability;
- availability observation timestamp;
- requested access level;
- exact source capability resolution.

The source resolution must use the fixed capability mapped from the requested
access level and the exact Core-derived context resource. Callers must provide
exactly one resolution per candidate, including non-authorized or
independent-evaluation results. A missing resolution is an input error, not an
implicit denial or a gap that the helper may invent.

### Projection Entry

An entry should retain:

- the typed stable context reference;
- granted access level;
- exact source capability resolution;
- sensitivity and redaction posture;
- no payload.

Retaining the source resolution prevents a serialized projection from
substituting a grant, resource, actor, step, or harness without validation
failure.

### Projection Gap

An optional typed gap should report bounded categories only:

- unavailable;
- unknown availability;
- no matching authority;
- independent policy evaluation required;
- independent approval evaluation required;
- independent evidence or check evaluation required;
- sensitivity ceiling exceeded;
- access level not authorized.

Gaps must not echo rejected IDs, paths, snippets, policy contents, or other
caller values. A gap is not a fabricated missing citation.

The serialized projection must retain the complete ordered evaluated candidate
set. Validation and deserialization must recompute the exact entries and gaps
from those candidates and reject omission, substitution, reordering, or
inconsistent posture. This makes an empty or partial wire projection an honest
derivation rather than a caller-selected view.

## 8. Authority Composition

Context projection must consume the existing capability-resolution boundary.
The capability must be the fixed access-level capability. The resource kind
must be `ContextReference`, and its reference must equal the canonical value
derived from the typed target. Group, prefix, wildcard, repository-wide, or
ambient resource matching is not part of the first implementation.

The first implementation should require:

- exact actor, workflow, run, step, optional harness, and evaluation-time
  equality;
- `CapabilityResolutionPosture::Authorized`;
- exact resource equality or a separately validated resource-scope match;
- sensitivity compatibility;
- a selected valid grant;
- no unresolved policy, approval, evidence, or check prerequisites.

Availability alone never authorizes. A policy decision does not become a grant.
An approval does not become reusable context authority. An
`EvidenceReference` does not authorize its own dereference.

## 9. Projection Algorithm

The pure helper should:

1. Validate projection scope and redaction metadata.
2. Validate and deterministically order the complete candidate set.
3. Validate every supplied capability resolution.
4. Require exact projection-context equality.
5. Reject duplicate candidate targets and duplicate authority resolutions.
6. Require the exact capability mapped from each requested access level.
7. Require the exact Core-derived context resource.
8. Gap unavailable and unknown candidates.
9. Gap non-authorized and independently evaluated resolutions.
10. Enforce the projection sensitivity ceiling.
11. Return deterministically ordered entries and bounded gaps.
12. Retain the complete evaluated candidate set so wire validation can
    recompute the exact derivation.

The helper must not dereference a target, contact a store, inspect a repository,
load a tool, mutate a run, or emit events.

## 10. Missing And Unavailable Context

For supplied candidates, the model must distinguish:

- the target is declared unavailable;
- availability is unknown;
- the target exists but authority is absent;
- authority requires independent evaluation;
- authority exists but sensitivity or access level is incompatible.

No supplied candidates produce a valid empty projection with no gaps.
Required-context semantics belong to a later contract-consumer phase; only that
consumer can distinguish an intentionally empty request from a missing required
target. The first projection must not decide that a workflow may proceed when
required context is absent.

No missing state may create a fake target, fake evidence, inferred grant, or
placeholder payload.

## 11. Freshness And Time-Of-Use

Projection-time equality proves only that one deterministic batch used one
evaluation timestamp. It is not a lease or durable authorization.

Before any future payload dereference, mount, adapter lookup, or sandbox
exposure, the consumer must:

- re-resolve current capability availability and grant lifecycle;
- re-evaluate independent policy, approval, evidence, and check obligations;
- verify immutable run and step context;
- verify the target still matches its stable reference;
- enforce current sensitivity and redaction posture;
- record an authority receipt or equivalent reviewed proof if required.

A serialized projection must never be sufficient by itself for time-of-use
access.

## 12. Privacy And Redaction

The model must not store:

- raw source, file, issue, comment, report, event, or evidence contents;
- prompts, transcripts, model context, chain-of-thought, or memory payloads;
- provider payloads or headers;
- command output, CI logs, or parser payloads;
- environment values or credentials;
- arbitrary metadata maps;
- secret-like field names, reasons, paths, URLs, or labels.

Debug output must redact identities, references, authority, sensitivity, and
redaction metadata. Serialization may expose only validated stable references
and fixed typed metadata. Deserialization and validation errors must use stable
codes and must not echo caller values.

Context projections may be sensitive even when every target is read-only.

## 13. Relationship To Existing Concepts

### EvidenceReference

EvidenceReference is a citation substrate. A context projection may expose an
EvidenceReference ID and safe posture, but it must not recreate the evidence or
read the evidence target. A later dereference helper must enforce authority
independently.

### Typed Handoffs

A typed handoff may carry context references and obligations. The receiving
harness receives only the projection authorized for its exact context; parent
authority is not inherited automatically.

### WorkReport

WorkReports may cite projected references, but report citation does not grant
context access. Context projection must not copy report section text.

### Proportional Governance

Missing context, unresolved authority, excessive sensitivity, or required
independent evaluation may monotonically escalate governance. Inference may
never weaken explicit context-access minimums.

### SideEffects

Reference projection is not a SideEffect. A future payload access may itself
require audit or approval, and any external mutation performed using context
must remain a separately governed SideEffect.

### Composable Harness Contracts

Harness contracts may later declare required context kinds and access levels.
Runtime projection must still use exact actor/run/step/harness authority rather
than treating the contract declaration as a grant.

### Optional Sandbox Providers

A future sandbox provider such as OpenShell may enforce filesystem, process,
network, and credential containment after Workflow OS resolves context
authority. The sandbox should receive only the already-authorized projection
and immutable policy inputs. Sandbox availability or containment does not grant
context authority, and this plan does not integrate a sandbox provider.

## 14. Validation And Error Posture

Future validation should ensure:

- reference target and kind are valid and consistent;
- availability is known and valid;
- sensitivity and redaction metadata are valid;
- access level is supported;
- actor, workflow, run, step, harness, and timestamp context match exactly;
- source capability resolution is valid and authorized;
- resource scope matches the context target;
- selected grant and prerequisite posture remain valid;
- access level maps to the expected fixed capability;
- capability resource equals the Core-derived typed context resource;
- duplicate and unordered entries or gaps fail closed;
- the complete evaluated candidate set deterministically derives the exact
  entries and gaps;
- serialized entry authority cannot be substituted;
- unknown enum and field values fail with stable non-leaking codes.

Errors must never include raw references, metadata, paths, URLs, snippets,
payloads, policy contents, or rejected serialized values.

## 15. Test Plan

Future focused tests should cover:

- valid reference-only projection;
- valid bounded-metadata projection;
- all implemented stable-reference variants;
- exact first-slice target taxonomy with no string escape hatch;
- fixed capability mapping for each access level;
- exact canonical context-resource derivation;
- authorized-only inclusion;
- empty projection;
- unavailable and unknown context;
- absent authority;
- independent policy, approval, evidence, and check posture;
- wrong actor, workflow, run, step, or harness;
- stale evaluation batch;
- expired or revoked grant;
- resource mismatch;
- sensitivity ceiling violation;
- requested access broader than authority;
- duplicate references and resolutions;
- omitted, substituted, or reordered evaluated candidates, entries, and gaps;
- deterministic ordering and serialization;
- serde round trip;
- forged target, grant, context, access level, and source-resolution rejection;
- Debug and error non-leakage;
- absence of forbidden raw payload fields;
- existing capability, EvidenceReference, handoff, WorkReport, SideEffect,
  approval, policy, validation, adapter, and runtime regressions.

## 16. Proposed Implementation Sequence

1. Implement the smallest reference-only and bounded-metadata core model.
2. Implement a pure step-scoped projection helper using existing capability
   resolutions.
3. Add focused validation, serde, privacy, and deterministic-order tests.
4. Perform a phase-level maintainer review.
5. Plan required-context contract consumption separately.
6. Plan time-of-use re-resolution and authority receipts separately.
7. Only after those reviews, consider one read-only context dereference
   boundary.
8. Only after separate approval, consider sandbox or harness runtime
   projection.

Implementation should begin with model types and a pure helper only.

## 17. Open Questions

- How should a contract declare that a context reference is required without
  turning declaration into authority?
- What immutable commitment should bind a context reference to a run bundle?
- When should context access become an auditable event rather than a projection
  fact?
- What freshness policy is sufficient before a future dereference?
- How should evidence targets with stricter sensitivity than their citation be
  handled?
- How should a future sandbox receive projected context without gaining access
  to unprojected workspace content?

## 18. Final Recommendation

The next phase should be:

**Required-context contract consumption planning.**

Do not implement context dereference, source reading, memory, tool execution,
connectors, providers, sandbox lifecycle, OpenShell integration, SideEffect
execution, writes, persistence, events, receipts, schemas, SDKs, CLI behavior,
hosted administration, enterprise identity, or release changes.
