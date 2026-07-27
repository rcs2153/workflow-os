# Production Current-Authority Source Boundary Plan

Status: Planning, model-only implementation, focused model review, the private
registered in-memory source-interface proof, and focused source-interface
review are complete. No public source trait, production source, persistent
registry, source service, runtime consumer, or readiness path is implemented.

Related foundations:

- [Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md)
- [Required Context Current Authority Fact-Set Plan](required-context-current-authority-fact-set-plan.md)
- [Current Authority In-Memory Source Plan](current-authority-in-memory-source-plan.md)
- [Current Authority Same-Call Time-Of-Use Resolver Plan](current-authority-same-call-time-of-use-resolver-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)

## 1. Executive Summary

Workflow OS has proved same-call current-authority composition with private
test-only complete inventories. The next boundary is not a runtime consumer.
It is a production-shaped source contract that can establish where current
facts came from, which exact query they cover, whether they are complete and
coherent, and whether they are fresh enough for one evaluation.

The governing invariant is:

```text
A source response is data.
Only a Core-owned, configured source read can establish trusted current facts.
```

The first implementation should remain model-only. It should define bounded
source identity, registration, request, snapshot, watermark, completeness,
freshness, and failure vocabulary without implementing storage, networking,
runtime readiness, target dereference, or provider execution.

## 2. Why This Phase Is Next

The accepted private resolver proves the algorithm:

```text
complete current facts
  -> capability resolution
  -> context projection
  -> required-context consumption
```

It does not prove a production source.

A runtime consumer would be unsafe before Core can distinguish:

- configured source from arbitrary caller input;
- complete exact-query results from partial search results;
- coherent snapshot reads from mixed-time observations;
- fresh observations from stale state;
- source unavailability from legitimate negative facts; and
- retryable operational failure from deterministic denial.

This boundary is also required before proportional governance can select quiet
success from current authority. Lower friction must never be inferred from
stale, partial, or caller-asserted facts.

## 3. Goals

- Define a domain-neutral production current-authority source contract.
- Bind source trust to explicit Core-owned registration or construction.
- Derive requests from the exact immutable execution binding and exact
  required-context contract.
- Require exact query-set coverage.
- Commit source identity and configuration without storing credentials.
- Define one coherent snapshot or coordinated snapshot-vector boundary.
- Define opaque snapshot-watermark identity and, only where the source contract
  supports it, a bounded comparable generation.
- Define explicit observation and freshness timestamps.
- Detect concurrent source changes and fail closed.
- Distinguish valid negative authority facts from source failures.
- Define stable bounded retry and terminal-failure posture.
- Preserve payload-free records, redaction-safe Debug, and non-leaking errors.
- Keep the first implementation model-only and independently reviewable.

## 4. Non-Goals

This plan does not authorize:

- a production source implementation;
- database, filesystem, network, queue, or hosted source access;
- runtime or executor integration;
- public `Ready` or authorization APIs;
- target, evidence, report, event, artifact, or source-content dereference;
- policy, approval, evidence, or check prerequisite evaluation;
- reusable authority leases, caches, or background refresh;
- persistence, event emission, audit projection, receipts, or artifacts;
- workflow or runtime configuration;
- credentials, secret providers, or environment-value access;
- providers, connectors, OpenShell, sandboxes, tools, or inference execution;
- SideEffect execution or writes;
- schemas, SDKs, CLI, UI, examples, or domain packs;
- hosted administration, enterprise RBAC, IdP, DLP, or stewardship;
- reasoning lineage; or
- release posture changes.

## 5. Trust Root

A public model constructor cannot establish source trust.

The production boundary should require a Core-owned source registration or
construction step. Registration binds:

- a typed source ID;
- source implementation kind;
- source contract version;
- bounded configuration commitment;
- supported fact families;
- supported consistency posture;
- freshness policy reference or explicit bounded freshness requirement; and
- redaction/sensitivity posture.

The configuration commitment must exclude credentials and raw configuration
payloads. It may commit safe normalized configuration after secret references
are removed.

The first model phase may represent this registration vocabulary, but it must
not claim that a caller-built registration is authenticated. Trusted
registration and source instantiation remain Core-owned runtime work.

## 6. Candidate Core Model

The smallest future model set should be evaluated:

- `CurrentAuthoritySourceId`;
- `CurrentAuthoritySourceContractVersion`;
- `CurrentAuthoritySourceKind`;
- `CurrentAuthorityFactFamily`;
- `CurrentAuthoritySourceRegistration`;
- `CurrentAuthoritySourceRequest`;
- `CurrentAuthoritySourceSnapshotId`;
- `CurrentAuthoritySourceWatermark`;
- `CurrentAuthoritySourceGeneration`;
- `CurrentAuthoritySourceReadWindow`;
- `CurrentAuthoritySourceCompleteness`;
- `CurrentAuthoritySourceConsistency`;
- `CurrentAuthoritySourceSnapshot`;
- `CurrentAuthoritySourceFailureKind`; and
- `CurrentAuthoritySourceFailurePosture`.

Only types required to express the boundary should be implemented. A trait,
store, client, resolver, or runtime service is not required for the first
model-only phase.

## 7. Source Identity

Source identity must be stable, bounded, and non-secret.

The identity model should include:

- source ID;
- source contract version;
- source kind;
- configuration commitment;
- supported fact families; and
- registration commitment.

It must not include:

- URLs containing credentials;
- tokens, passwords, authorization headers, private keys, or cookies;
- raw environment variables;
- provider payloads;
- filesystem paths by default; or
- arbitrary user descriptions.

Debug output should expose source kind and bounded posture, not IDs or
commitments.

## 8. Exact Request Boundary

The source request must be derived from:

- exact immutable execution-binding hash;
- exact required-context contract hash;
- canonical exact query-set hash;
- typed actor, workflow, run, step, and harness identities derived internally
  from the validated immutable binding;
- requested sensitivity bounds;
- evaluation timestamp;
- accepted source registration commitment; and
- requested fact families.

Callers must not be able to substitute a prefiltered query slice, a prior
fact-set commitment, or an unrelated source snapshot.

The request is payload-free. It asks for authority metadata, not target
contents.

## 9. Fact Families

The first production boundary should distinguish fact families explicitly:

- capability grants;
- capability availability;
- governed context references; and
- future prerequisite decisions.

Policy, approval, evidence, and check prerequisite facts remain future
families. Their IDs alone do not prove satisfaction.

A source that cannot supply a requested family must report unsupported or
incomplete. It must not return an empty list and imply completeness.

## 10. Completeness

Completeness must be explicit and query-bound.

Candidate postures:

- `CompleteForExactQuery`;
- `Incomplete`;
- `Unsupported`;
- `Unavailable`; and
- `Unknown`.

Only `CompleteForExactQuery` may participate in future authoritative
resolution. It must mean:

- every requested fact family was evaluated;
- every exact capability/resource query has one bounded availability
  observation;
- all matching grant candidates in the coherent source view were included;
- every exact context target has one current reference posture; and
- the completeness statement is bound to the exact query-set hash and source
  snapshot.

Empty complete results are valid only when the source can prove that the exact
query matched no grants or references. Absence and failed lookup must remain
different.

## 11. Snapshot And High-Watermark Semantics

The source response should identify one coherent read.

For a single source, it should carry:

- opaque snapshot ID;
- opaque snapshot-watermark commitment;
- optional bounded comparable source generation when the accepted source
  contract defines ordering semantics;
- read-started timestamp;
- observed-at timestamp;
- read-completed timestamp;
- consistency posture; and
- source snapshot commitment.

Raw database sequence numbers, offsets, transaction IDs, paths, or provider
tokens should not be exposed. An opaque watermark can prove snapshot identity
or change through equality comparison; it cannot prove monotonic ordering.
Ordering claims require a separately validated, source-defined generation
whose comparison semantics are part of the accepted source contract.

For future composite sources, a coordinator may commit a canonical snapshot
vector. The first implementation should not invent distributed snapshot
coordination.

## 12. Consistency And Concurrency

Candidate consistency postures:

- `AtomicSnapshot`;
- `StableWatermark`;
- `BestEffort`;
- `Unknown`.

Future authoritative resolution should require an accepted posture. The first
local production source will likely need `AtomicSnapshot` or
`StableWatermark`.

If the snapshot watermark changes during the read and the source cannot prove
a coherent snapshot, the read fails closed as concurrent change. Core must not silently
combine grants from one observation with availability or references from
another.

## 13. Freshness

Freshness must be evaluated explicitly, not inferred from “same function
call.”

The model should retain:

- source observed-at time;
- read completion time;
- requested evaluation time;
- optional valid-through time or maximum-age commitment; and
- freshness posture.

Rules:

- no source observation may be in the future relative to evaluation time;
- no read completion may precede read start;
- valid-through must not precede observed-at;
- an expired freshness window fails closed;
- unknown freshness cannot lower governance friction; and
- wall-clock reads must be injected, never hidden inside deterministic model
  validation.

A source may supply an observation validity bound, but it cannot unilaterally
decide how long Core trusts authority. Future effective freshness must be the
stricter of the source validity bound and a Core-owned maximum-age policy. If
either required bound is absent or unknown, the result cannot lower governance
friction. Source-specific freshness configuration and Core policy enforcement
remain runtime work.

## 14. Source Snapshot

A future `CurrentAuthoritySourceSnapshot` should retain only bounded facts and
commitments:

- source registration commitment;
- exact execution-binding and query-set commitments;
- snapshot ID and watermark commitment;
- read window and freshness posture;
- consistency and completeness posture;
- canonical grant and availability records;
- canonical context-reference records;
- requested and returned fact-family counts;
- records commitment;
- snapshot commitment; and
- redaction metadata.

It must validate canonical ordering, duplicate rejection, exact coverage,
temporal consistency, and hash integrity.

The snapshot remains source output vocabulary. It does not itself confer
readiness outside a future Core-owned same-call resolver boundary.

## 15. Failure Taxonomy

The source boundary must distinguish:

- source unavailable;
- request unsupported;
- incomplete result;
- stale result;
- future-dated observation;
- concurrent change;
- ambiguous duplicate;
- corrupt or invalid source data;
- registration mismatch;
- query mismatch;
- transport or operational failure; and
- internal source failure.

Failures must use stable codes and bounded posture. Errors must not include raw
queries, resources, IDs, paths, endpoints, provider output, credentials, or
source payloads.

Valid negative facts such as no matching grant, revoked grant, or unavailable
capability are not source errors. They are inputs to capability resolution.

## 16. Retry Posture

The source boundary may classify failure as:

- deterministic terminal;
- retryable with unchanged request; or
- retryable only after source/configuration change.

The first model should not implement retries.

Future retry behavior must:

- remain explicit;
- use bounded attempts and backoff;
- preserve the exact immutable request;
- obtain a new snapshot commitment;
- never reuse partial facts; and
- remain auditable.

Retry must not turn unknown or unavailable authority into permission.

## 17. Reference Source Coordination

The private proof currently uses separate authority and context-reference
inventories.

The production boundary should prefer one source snapshot that commits grants,
availability, and references for the exact query. If separate physical sources
are unavoidable later, a Core-owned coordinator must:

- bind each registered source;
- collect one observation per fact family;
- prove accepted consistency for each source;
- commit a canonical snapshot vector;
- apply explicit freshness bounds; and
- fail closed when cross-source coherence cannot be established.

The first implementation should model one aggregate source snapshot only.

## 18. Relationship To Proportional Governance

The source does not choose governance mode.

A future proportional-governance decision may consume a reviewed
same-call authority assessment. Unknown, stale, incomplete, unsupported, or
inconsistent source posture must raise friction or deny work. It must never
select a quieter mode than an explicit policy minimum.

Visible disclosure remains a presentation/delivery concern where appropriate;
source truth and decision posture stay separate.

## 19. Relationship To OpenShell

OpenShell is a potential execution containment provider, not an authority
source.

The future boundary remains:

```text
Workflow OS source-backed governance decision
  -> governed execution commitment
  -> optional OpenShell execution provider
  -> bounded sandbox enforcement evidence
```

OpenShell must not establish Workflow OS policy, approval, evidence, or current
authority merely because a sandbox ran successfully.

## 20. Privacy And Security

The model must not store:

- raw provider, sandbox, process, command, CI, Jira, or GitHub payloads;
- target contents or repository source;
- credentials, tokens, cookies, authorization headers, or private keys;
- raw environment values;
- unbounded source errors;
- raw configuration; or
- raw database or queue cursors when a bounded commitment suffices.

Debug, Display, serialization, and deserialization errors must remain safe.
Source snapshots may be sensitive even though they contain only references and
authority metadata.

## 21. First Implementation

The model-only implementation is complete:

1. source identity and contract version;
2. fact-family vocabulary;
3. source registration commitment;
4. source request commitment;
5. snapshot ID, watermark identity, optional comparable generation, read
   window, completeness, consistency, and freshness vocabulary;
6. payload-free source snapshot commitment;
7. stable failure taxonomy;
8. validation, canonical hashing, serde, and redaction-safe Debug; and
9. focused model tests.

It adds no source trait, registered source registry, concrete source, resolver
integration, or runtime consumer. Public construction is explicitly
descriptive and cannot authenticate a source or confer readiness.

The next private interface proof is also implemented. It adds one Core-owned
registration constructor around one in-memory aggregate inventory containing
capability grants, capability availability, and governed context references.
The source derives the exact request from the immutable execution binding and
required-context contract, canonicalizes complete inventories, selects exact
matching records, and returns either:

- one complete coherent payload-free `CurrentAuthoritySourceSnapshot`
  commitment; or
- one bounded `CurrentAuthoritySourceFailure`.

The proof is crate-private. It is not exported from `workflow-core`, cannot
accept a caller-built registration as trusted input, does not expose source
records, cannot return readiness, and does not dereference context targets.

## 22. Future Implementation Sequence

1. Production source-boundary core model: implemented.
2. Focused model review: accepted.
3. Private registered-source interface proof with one in-memory aggregate
   source: implemented.
4. Focused source-interface review: accepted.
5. Compose registered source and private same-call resolver: implemented.
6. Review source-backed assessment semantics: accepted.
7. Decide one-time-use/replay posture: planned.
8. Focused one-time-use/replay plan review: accepted.
9. Implement the private same-call use boundary: next.
10. Review the private same-call use boundary.
11. Only then plan one opt-in read-only runtime consumer.
12. Plan OpenShell or another execution provider separately.

## 23. Test Plan

Implemented model tests cover:

- valid source registration and request;
- exact binding/query-set commitments;
- stable source identity and version validation;
- supported and unsupported fact families;
- exact-query completeness;
- empty-but-complete results;
- incomplete, unavailable, unsupported, and unknown posture;
- canonical record and family ordering;
- duplicate rejection;
- snapshot ID and watermark bounds;
- atomic and stable-watermark consistency;
- read-window ordering;
- future-dated, stale, and expired freshness;
- concurrent-change failure vocabulary;
- registration, query, and snapshot substitution;
- deterministic hashes and serde round trips;
- invalid serialized values failing closed;
- Debug and error non-leakage;
- no raw payload or credential fields;
- no readiness or authorization methods; and
- existing authority, context, runtime, provider, and workspace tests.

The private proof adds focused tests for Core-owned registration,
complete exact-query snapshots, canonical inventory ordering, bounded
incomplete-source failures, stale and future-dated failures, duplicate
inventory rejection, and redaction-safe Debug behavior. Focused
source-interface review accepts the proof with non-blocking negative-path test
follow-ups for the composition phase.

The private composition adds focused tests for a ready source-backed
assessment, source-failure short-circuiting, unresolved approval prerequisites,
revoked grants, canonical inventory ordering, and redaction-safe assessment
Debug output. Selected source records never leave the private source boundary,
and no public source snapshot or caller-built fact-set commitment can invoke
the composition.

## 24. Open Questions

- Should source registration remain crate-private until runtime configuration
  exists?
- What minimum consistency posture should the first local source require?
- Which first source contract, if any, can define a safe comparable generation
  in addition to opaque watermark equality?
- What Core-owned maximum-age policy should cap a source-supplied valid-through
  bound?
- Which fact families can one local aggregate source prove initially?
- How should accepted approval-presentation proof become a prerequisite fact?
- Which local-check result posture qualifies as independent current evidence?
- When should source responses become one-time-use?
- What is the first production read-only consumer after source-backed review?

## 25. Final Recommendation

Implement the accepted private same-call use boundary from the
[Current-Authority One-Time-Use And Replay Posture Plan](current-authority-one-time-use-replay-posture-plan.md).
Keep the callback invocation itself as the one governed use, keep the bounded
consumer Core-owned, and expose no generic repeatable authority operation.

Do not add a public source trait, runtime consumer, provider, OpenShell
adapter, SideEffect execution, or writes.
