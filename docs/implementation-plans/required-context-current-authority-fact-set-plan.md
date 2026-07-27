# Required Context Current Authority Fact-Set Plan

## 1. Executive Summary

The immutable required-context execution binding now proves which frozen run,
step, actor, harness contract, sensitivity ceiling, and time are committed for
future context consumption. It deliberately does not prove that authority or
availability remains current.

Before Workflow OS can return an authoritative same-call time-of-use `Ready`
decision, Core needs one complete, validated, payload-free fact set covering
the authority sources relevant to the exact required-context contract.

This plan defines that boundary only. It does not implement a fact-set model,
time-of-use helper, target dereference, runtime integration, providers,
sandboxes, SideEffect execution, or writes.

## 2. Problem

The current capability resolver accepts caller-supplied slices of grants and
availability records. That is sufficient for deterministic bounded resolution,
but it cannot prove that the caller supplied every relevant current fact.

An authoritative consumer must not interpret:

- an omitted revoked or expired grant as an active authority path;
- an omitted availability record as evidence of availability;
- an arbitrary subset of policy, approval, evidence, or check records as
  complete prerequisite satisfaction;
- a prior projection or prior consumption result as current authority; or
- a self-asserted `complete: true` flag as source provenance.

Completeness is therefore a source and query-boundary property, not a boolean
owned by an untrusted caller.

## 3. Goals

- Define a domain-neutral, payload-free current authority fact set.
- Bind it to one exact immutable execution binding and contract.
- Derive required capability/resource queries from typed contract
  requirements.
- Retain all relevant grants and availability observations for those queries.
- Retain accepted prerequisite records by stable reference and exact context.
- Make omissions, duplicates, ambiguity, staleness, and source mismatch fail
  closed.
- Preserve independent ownership of policy, approval, evidence, checks,
  sensitivity, and SideEffect constraints.
- Prepare for a future pure same-call time-of-use helper.
- Support quiet-success UX later without weakening authority proof.

## 4. Non-Goals

This plan does not authorize:

- implementation in this phase;
- a time-of-use `Ready` result;
- target or payload dereference;
- runtime or executor integration;
- persistence or a new authority store;
- workflow events, audit projection, receipts, or report artifacts;
- schema, SDK, CLI, UI, or example changes;
- provider, connector, OpenShell, filesystem, network, process, inference, or
  credential execution;
- SideEffect execution or writes;
- enterprise identity, hosted administration, reasoning lineage, or release
  changes.

## 5. Source-Of-Truth Boundaries

The future fact set must compose existing sources without replacing them.

| Concern | Existing source of truth | Fact-set posture |
| --- | --- | --- |
| Immutable run identity | `StoredImmutableRunBundle` and `RequiredContextExecutionBinding` | Exact comparison required |
| Contract requirements | `RequiredContextContractBinding` | Exact ID, version, and content hash required |
| Grant scope and lifecycle | `CapabilityGrant` | Retain every relevant candidate |
| Connectivity/availability | `CapabilityAvailabilityRecord` | Retain one unambiguous current observation per query |
| Policy | accepted policy decision/event record | Stable exact-context reference, not boolean |
| Approval | approval request plus granted decision and proof posture where required | Stable exact-scope reference, not boolean |
| Evidence | validated `EvidenceReference` | Stable reference and sensitivity posture only |
| Checks | accepted local-check attestation or exact result reference | Stable same-run/check binding, not caller assertion |
| Sensitivity | contract requirement, binding ceiling, grant ceiling, target/reference classification | Most restrictive applicable bound wins |
| SideEffects | workflow/step declaration and accepted SideEffect authority records when relevant | Explicit none, prohibited, or accepted reference |
| Proportional governance | validated assessment/binding | Presentation posture only; never substitutes for authority |

The fact set must not copy raw policy input, approval prose, evidence payload,
check output, provider data, command output, source content, or SideEffect
payload.

## 6. Exact Evaluation Scope

Every fact set should bind:

- fact-set model version;
- immutable execution-binding hash;
- immutable bundle ID, version, and root hash;
- workflow ID;
- run ID;
- step ID;
- actor ID;
- harness contract ID, version, and content hash;
- one evaluation timestamp;
- maximum sensitivity;
- deterministic required capability/resource queries; and
- source completeness bindings for each fact category.

The workflow, run, and step identity must remain derived from the immutable
bundle and execution binding. Callers must not independently relabel them.

## 7. Required Query Derivation

The required-context contract already provides typed targets and access
levels. A future pure derivation helper should map each requirement to the
exact `CapabilityReference` and `CapabilityResourceScope` used by the existing
capability resolver.

The complete query set must:

- include every required and optional contract requirement;
- sort deterministically;
- reject duplicate requirement IDs;
- reject duplicate target/access pairs whose semantics are ambiguous;
- retain the requirement-to-query mapping;
- reject targets or access levels without a defined capability mapping; and
- be committed by hash.

The caller must not be allowed to omit a contract requirement from the query
set.

## 8. Completeness Provenance

The first authoritative fact source must be Core owned. It should execute one
bounded query against a validated source and return both records and a
payload-free source binding.

Candidate source-binding fields:

- source kind and version;
- source instance or snapshot ID;
- exact query-set hash;
- snapshot or high-watermark reference;
- observed-at timestamp;
- per-category record counts;
- deterministic records hash;
- redaction and sensitivity posture; and
- source-completeness posture.

Accepted completeness postures should be conservative:

- `CompleteForExactQuery`: the source contract proves all matching records for
  the exact query and snapshot;
- `Unavailable`: the source cannot answer and the consumer must block; or
- `Unknown`: completeness is not proven and the consumer must block.

There should be no caller-controlled `complete: bool`.

An in-memory test source may implement the same contract by owning the complete
inventory supplied at construction. A public helper that merely receives
arbitrary record slices cannot produce authoritative completeness.

## 9. Grant Fact Requirements

For every derived capability/resource query, the complete grant candidate set
must include all grants that could affect the decision for the exact actor,
workflow, run, step, and optional harness.

The fact set must preserve:

- grant ID;
- subject;
- capability and resource scope;
- workflow/run/step/harness scope;
- issuer;
- issued-at and optional expiry;
- active or revoked lifecycle;
- revocation reference where required;
- delegation posture;
- prerequisite references;
- sensitivity ceiling; and
- source binding.

Revoked and expired candidates must not be filtered out before deterministic
resolution. Their presence can be decision-relevant and must remain
inspectable.

## 10. Availability Fact Requirements

For every derived capability/resource query, availability must be observed at
or before the fact-set evaluation time under an explicit freshness policy.

The future model should reject:

- missing records;
- duplicate records for the same exact query;
- future-dated observations;
- observations older than the accepted freshness bound;
- mismatched capability or resource scope; and
- `Unknown` availability for an authoritative ready path.

`Available` proves bounded inventory/connectivity posture only. It does not
prove authority, successful authentication, sandbox containment, or provider
reachability.

## 11. Independent Prerequisite Facts

`CapabilityGrantRequirements` retains policy, approval, evidence, and check
IDs. Current resolution correctly returns
`RequiresIndependentEvaluation` when those prerequisites exist.

The future fact set must retain validated prerequisite facts rather than
turning these IDs into booleans.

### Policy

An accepted policy fact must bind the evaluated action, capability set, actor,
workflow, run, step where available, policy identity/effects, decision
posture, and event or audit reference. A generic `allowed: true` record without
exact context is insufficient.

### Approval

An accepted approval fact must bind the approval request subject to the exact
workflow/run/step/skill or action, include a granted decision, respect expiry,
and retain approval-presentation proof where the governing path requires it.
A decision ID alone is insufficient.

### Evidence

An accepted evidence fact must be a validated `EvidenceReference` whose ID,
scope, kind, sensitivity, and redaction posture satisfy the prerequisite. The
fact set must not dereference or copy the evidence target.

### Checks

An accepted check fact should prefer a verified local-check attestation or
exact immutable check binding plus result reference. A caller-asserted passed
status or mock output is not independent execution evidence.

Every prerequisite fact must be unique, exact-context, current under its own
rules, and traceable to its source record.

## 12. Sensitivity Composition

The future time-of-use path must compare:

- contract requirement sensitivity;
- execution-binding maximum sensitivity;
- selected grant sensitivity ceiling;
- evidence/reference sensitivity;
- availability/source classification where applicable; and
- consumer or sandbox sensitivity limits.

Unknown sensitivity blocks. No layer may silently lower a more restrictive
classification. The fact set should retain the effective sensitivity posture
and stable blocking reason, not raw protected data.

## 13. SideEffect Posture

Read-only required-context consumption must not silently become write
authority.

The fact set should retain an explicit SideEffect posture for the exact step:

- `NoneDeclared`;
- `ProhibitedForConsumption`;
- or a future accepted governed reference where separately required.

This plan does not add SideEffect execution. A future sandbox or provider
adapter must remain independently governed and cannot infer write permission
from readable context.

## 14. Candidate Core Model

Candidate model-only vocabulary:

- `CurrentAuthorityFactSetVersion`;
- `CurrentAuthorityFactSetScope`;
- `CurrentAuthorityQuery`;
- `CurrentAuthorityQuerySet`;
- `AuthorityFactSourceBinding`;
- `AuthorityFactSourceKind`;
- `AuthorityFactCompletenessPosture`;
- `AcceptedPolicyFact`;
- `AcceptedApprovalFact`;
- `AcceptedEvidenceFact`;
- `AcceptedCheckFact`;
- `CurrentAuthorityFactSet`; and
- `CurrentAuthorityFactSetHash`.

Only types needed to represent and validate the complete payload-free boundary
should be implemented. Do not add a time-of-use decision type in the first
model phase.

## 15. Construction And Visibility

The authoritative constructor should not accept arbitrary slices without a
validated source binding.

Recommended posture:

1. Public model validation and serde may prove internal consistency.
2. A Core-owned builder accepts the exact immutable binding, contract, derived
   query set, and results from a completeness-capable source.
3. The builder recomputes ordering, counts, category hashes, and aggregate
   fact-set hash.
4. Runtime consumers later require both a valid fact set and fresh comparison
   against the source/bundle boundary.

A deserialized fact set is a portable commitment, not proof that its source
snapshot is still current.

## 16. Determinism And Hashing

The fact set should use:

- a versioned domain-separated hash;
- fixed-width field framing;
- canonical sorting by typed identity;
- explicit category labels;
- exact query-set and source-binding hashes;
- duplicate rejection before hashing; and
- a fixed known-vector test.

The hash must commit every retained field except itself. Different omissions,
orderings, source snapshots, evaluation times, or prerequisite facts must
produce different hashes.

## 17. Validation And Failure Semantics

Stable blocking reasons should include:

- immutable binding mismatch;
- contract mismatch;
- query-set mismatch;
- incomplete, unknown, or unavailable source;
- source snapshot mismatch;
- missing or duplicate grant facts;
- missing, duplicate, stale, future, unknown, disconnected, or unsupported
  availability;
- revoked or expired grant;
- sensitivity exceeded or unknown;
- policy missing, denied, stale, or context-mismatched;
- approval missing, denied, expired, unproven, or context-mismatched;
- evidence missing, invalid, or too sensitive;
- check missing, failed, unattested, stale, or context-mismatched;
- SideEffect posture incompatible with read-only consumption; and
- serialized aggregate inconsistency.

Errors must use stable codes and must not contain caller values, paths,
payloads, tokens, raw policy details, approval reasons, evidence targets,
check output, or provider data.

## 18. Privacy And Redaction

The model may retain bounded typed IDs, hashes, enum postures, timestamps,
counts, and source references.

It must not retain:

- raw source or repository contents;
- raw context targets;
- policy input payloads or unrestricted violation text;
- approval prose beyond separately validated bounded records;
- evidence payloads;
- raw local-check stdout or stderr;
- provider responses or logs;
- command or parser output;
- environment values, credentials, headers, private keys, or tokens;
- unrestricted paths or mount lists; or
- sandbox payloads.

Debug should expose versions, postures, and counts while redacting identities,
hashes, and source references.

## 19. Relationship To Proportional Governance

Proportional governance decides the least interruptive presentation and
approval posture that satisfies declared constraints. It does not weaken the
fact-set proof.

Quiet capture may suppress unnecessary interruption only after the same
authority, completeness, evidence, and check requirements are satisfied.
Visible disclosure is a presentation obligation, not an alternative source of
authority. Blocking approval remains required when policy, sensitivity,
SideEffect, or enterprise stewardship demands it.

## 20. Relationship To Optional Sandbox Providers

An optional OpenShell or other sandbox provider could later consume an
accepted, minimized context projection after Workflow OS produces a current
authority result.

Workflow OS should own:

- governed intent;
- immutable identity;
- authority and prerequisite decisions;
- evidence and audit references; and
- report posture.

The sandbox provider should own:

- filesystem, network, and process containment;
- credential delivery boundaries;
- runtime policy enforcement; and
- stable sandbox outcome references.

The fact set must not be treated as sandbox attestation, and no sandbox
integration is implemented by this plan.

## 21. Test Plan

Future model tests should prove:

- exact binding, contract, query set, and complete source produce a valid fact
  set;
- required and optional contract requirements are all represented;
- caller-omitted queries fail closed;
- arbitrary record slices cannot claim authoritative completeness;
- grant and availability duplicates fail closed;
- revoked and expired grants remain represented;
- missing, unknown, stale, future, disconnected, and unsupported availability
  fail closed;
- policy, approval, evidence, and check references require exact accepted
  facts;
- denied or expired approval fails closed;
- unattested or failed check fails closed;
- sensitivity composition is monotonic;
- SideEffect posture cannot imply write authority;
- substitutions change the aggregate hash;
- valid serde round trip succeeds;
- wire tampering fails closed;
- known-vector and framing tests pass;
- Debug and errors do not leak protected values; and
- existing capability, context, required-context, immutable-bundle, approval,
  evidence, local-check, SideEffect, proportional-governance, and runtime tests
  still pass.

## 22. Proposed Implementation Sequence

1. Implement query-set derivation and model-only completeness/source-binding
   vocabulary.
2. Implement `CurrentAuthorityFactSet` model and pure validation only.
3. Add deterministic hash, serde, privacy, and completeness tests.
4. Perform a focused maintainer review.
5. Implement one Core-owned in-memory completeness-capable source for tests.
6. Review the source boundary.
7. Implement the pure same-call time-of-use helper without dereference.
8. Review before selecting one read-only runtime consumer.
9. Plan optional sandbox integration separately.

The first implementation must remain model-only.

Implementation status: the model-only query, source-binding, grant, and
availability fact-set commitment is implemented in the
[Required Context Current Authority Fact-Set Report](../concepts/REQUIRED_CONTEXT_CURRENT_AUTHORITY_FACT_SET_REPORT.md).
Trusted source completeness, accepted prerequisite facts, and time-of-use
readiness remain deferred. The next source boundary is now defined in the
[Current Authority In-Memory Source Plan](current-authority-in-memory-source-plan.md).
It keeps the first completeness-capable source private and test-only. That
source is now implemented in the
[Current Authority In-Memory Source Report](../concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_REPORT.md);
focused source review accepts that private boundary in the
[Current Authority In-Memory Source Review](../concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_REVIEW.md).
Production source trust and time-of-use readiness remain deferred.
Pure same-call resolver planning is now complete in the
[Current Authority Same-Call Time-Of-Use Resolver Plan](current-authority-same-call-time-of-use-resolver-plan.md).
The first implementation remains private and test-only so caller-constructed
fact-set commitments cannot confer readiness.

## 23. Open Questions

- Which existing accepted policy record has sufficient exact step/action
  binding, or is a new payload-free policy-fact wrapper required?
- Should granted approval facts require presentation proof for every path or
  only proof-enforced approval policies?
- What freshness policy applies per availability source?
- Which check paths qualify as independent attested evidence?
- Should evidence prerequisites require only reference validity or an
  additional accepted-evidence observation?
- How should a local in-memory authority source prove complete ownership of its
  inventory?
- When should harness contracts become immutable-bundle definition records?
- Does the first read-only consumer need one-time-use claim semantics?

## 24. Final Recommendation

The fact-set core model, private test source, and focused reviews are complete.
The private test-only same-call time-of-use resolver is also implemented in the
[Current Authority Same-Call Time-Of-Use Resolver Report](../concepts/CURRENT_AUTHORITY_SAME_CALL_TIME_OF_USE_RESOLVER_REPORT.md).
The next phase should perform focused implementation review.

The implementation remains pure, payload-free, resolver-only, and incapable of
target dereference or runtime execution.

Continue to defer authoritative `Ready`, dereference, runtime integration,
persistence, events, schemas, CLI behavior, providers, OpenShell, sandbox
execution, SideEffects, writes, hosted behavior, reasoning lineage, and release
changes.

## 25. Planning Governance And Validation

- workflow: `dg/d`
- run ID: `run-1785141328610692000-2`
- approval ID:
  `approval/run-1785141328610692000-2/planning-approved`
- presentation ID: `presentation/ff85148ac611a7ff`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof was persisted and enforced
- `npm run check:docs`: passed
- `git diff --check`: passed
- out-of-kernel work: model inventory, plan writing, roadmap updates, and
  validation commands were performed by the delegated maintainer; the kernel
  governed scope and approval but did not inspect code, edit files, invoke
  tools, or mutate git
