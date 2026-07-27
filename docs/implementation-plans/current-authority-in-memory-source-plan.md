# Current Authority In-Memory Source Plan

## 1. Executive Summary

The current-authority fact-set model commits one exact required-context query
set and the grant and availability records supplied for that query. Its public
constructor intentionally treats `CompleteForExactQuery` as a caller-owned
claim rather than trusted authority.

The next bounded phase should add one Core-owned in-memory source for tests.
The source will own a complete canonical inventory before a query is accepted,
derive the exact query set from the required-context contract, filter the owned
inventory deterministically, and produce a source-bound
`CurrentAuthorityFactSet`.

The source is a test instrument for proving completeness semantics. It is not a
runtime authority source, readiness decision, dereference lease, provider,
store, sandbox, or execution path.

## 2. Problem

A caller that supplies records directly to `CurrentAuthorityFactSet::new` can
prove only that those records form an internally consistent commitment. The
caller cannot prove that an omitted grant, revocation, expiry, or availability
record does not exist.

Completeness becomes meaningful only when one boundary:

- owns the complete relevant inventory before query execution;
- derives the query from the exact immutable contract rather than caller
  filters;
- applies deterministic matching rules;
- retains decision-relevant inactive, expired, and revoked grants;
- provides exact availability coverage without inventing availability;
- commits the complete inventory snapshot independently of the result slice;
  and
- constructs the result inside Core.

The first source should prove those mechanics without establishing a public or
runtime trust root prematurely.

## 3. Goals

- Add a Core-owned in-memory current-authority source for tests only.
- Own one complete bounded grant and availability inventory at construction.
- Canonicalize and validate the complete inventory before queries.
- Bind the inventory to one deterministic snapshot hash and observation time.
- Accept only an exact `RequiredContextExecutionBinding` and
  `RequiredContextContractBinding`.
- Derive the complete query set from the contract inside Core.
- Return every matching grant candidate, including inactive, revoked, and
  expired candidates.
- Require exactly one matching availability observation per exact query.
- Reject ambiguous, duplicate, unavailable, or incomplete source inventory.
- Produce a payload-free source-bound `CurrentAuthorityFactSet`.
- Keep caller-owned public fact-set construction non-authoritative.
- Add focused completeness, canonicalization, substitution, and privacy tests.

## 4. Non-Goals

This plan does not authorize:

- a public authority-source API;
- a runtime, executor, retry, or approval-resume consumer;
- an authorization, permit, readiness, consumption, projection, or
  dereference result;
- target access or payload retrieval;
- freshness policy beyond bounded source timestamps;
- accepted policy, approval, evidence, or check fact composition;
- persistence, a local authority database, events, audit receipts, or report
  artifacts;
- workflow schema, SDK, CLI, UI, or example changes;
- providers, connectors, OpenShell, filesystem, network, process, inference,
  or credential execution;
- SideEffect execution, provider mutations, or writes;
- hosted administration, enterprise identity, reasoning lineage, or release
  changes.

## 5. Trust Boundary

The first source must remain internal to `workflow-core` tests.

Recommended posture:

- compile the source under `#[cfg(test)]`;
- keep the source type and constructors private to the crate;
- exercise it through unit tests in the source module;
- do not re-export it from `workflow_core`;
- do not serialize or deserialize the source itself; and
- do not expose a trait or provider abstraction without a concrete runtime
  consumer.

This prevents downstream callers from constructing the test source and
mistaking its output for production authority.

The returned `CurrentAuthorityFactSet` remains the existing public model. Its
source completeness vocabulary does not become generally trusted merely
because the test source can produce it.

## 6. Candidate Internal Model

The smallest first implementation should use an internal type similar to:

```text
InMemoryCurrentAuthoritySource
  version = v1
  observed_at
  grants
  availability_records
  inventory_hash

InMemoryCurrentAuthoritySourceInput
  observed_at
  complete_grant_inventory
  complete_availability_inventory

CurrentAuthoritySourceQueryInput
  execution_binding
  contract
  evaluated_at
```

No public source ID, remote endpoint, persistence handle, callback, async
interface, or trait is needed.

## 7. Inventory Ownership

The source constructor must consume owned vectors. It must not retain borrowed
caller slices whose contents can change after validation.

Construction should:

1. validate every grant and availability record through existing typed models;
2. sort grants by grant ID;
3. reject duplicate grant IDs;
4. sort availability by exact capability and resource identity;
5. reject duplicate exact availability identities;
6. compute an inventory hash over the entire canonical inventory; and
7. retain the observation time independently of any query.

The inventory hash must include records that no later query selects. Otherwise
the source snapshot would be only a commitment to a result slice rather than
the owned source state.

## 8. Exact Query Execution

Query execution must accept the exact execution binding and contract and call
`CurrentAuthorityQuerySet::from_contract` internally.

The source must not accept:

- caller-supplied capability filters;
- caller-supplied resource filters;
- caller-supplied requirement IDs;
- a caller-supplied query-set hash;
- a caller-supplied completeness posture; or
- a caller-supplied source snapshot hash.

The exact query set must include required and optional contract requirements.
Optional means the future consumer may disclose a gap; it does not mean the
source may omit the query.

## 9. Grant Selection

For each exact capability/resource query, the source should retain all grants
in the owned inventory that can be candidates for that query.

Selection must apply the existing exact request-matching identity boundary:

- grant subject equals the execution-binding actor;
- grant capability and resource equal the derived query;
- grant workflow equals the immutable workflow;
- an optional grant run equals the immutable run;
- an optional grant step equals the exact step; and
- an optional grant harness equals the exact harness contract.

The source must retain every grant that satisfies that identity and scope
predicate. It must not reduce the set to only the highest-specificity grant;
specificity selection belongs to the existing resolver.

The source must not pre-filter matching candidates by:

- active lifecycle;
- expiry;
- revocation;
- prerequisite satisfaction;
- delegation posture;
- sensitivity; or
- a future readiness interpretation.

Those fields remain decision facts for the future time-of-use resolver.
Filtering them at the source could hide a conflict or allow omission to look
like authority.

The first test source may therefore return candidates that a future resolver
will reject.

## 10. Availability Selection

Availability remains separate from authority.

For every exact capability/resource query, the owned inventory must contain
exactly one matching `CapabilityAvailabilityRecord`.

The source must fail closed for:

- no matching record;
- duplicate matching records;
- a future-dated source observation;
- a record observed after the source snapshot time;
- capability or resource mismatch; or
- any inventory ambiguity.

The source must not synthesize `Available`, `Unavailable`, or `Unknown` when a
record is absent. Missing source data is a source error, not an observation.

An explicit `Unavailable` or `Unknown` record may be returned as a complete
fact. A future authority resolver, not the source, decides its effect.

## 11. Source Binding

The source-owned fact set should bind:

- `AuthorityFactSourceKind::InMemoryInventorySnapshot`;
- the canonical full-inventory hash as source snapshot hash;
- the source observation time;
- `CompleteForExactQuery`;
- the internally derived exact query-set hash;
- selected grant and availability counts; and
- the existing canonical selected-records hash.

`CompleteForExactQuery` means only that the internal test source returned every
matching record from the inventory it owned at construction. It does not prove
that the inventory reflects an external authority system.

## 12. Output Construction

The test source may call the existing `CurrentAuthorityFactSet::new` only after
it has:

- validated source ownership;
- derived the exact query set;
- selected records from the full inventory;
- established exact availability coverage; and
- derived the source snapshot hash itself.

If implementation needs a crate-private constructor or marker to distinguish
source-owned construction from public caller construction, add only the
smallest internal API required. Do not add a public trusted-completeness flag.

The result exposes no readiness or authority method.

## 13. Failure Semantics

Source errors must use stable `current_authority.source.*` codes.

Candidate codes:

- `current_authority.source.inventory.grant_duplicate`;
- `current_authority.source.inventory.availability_duplicate`;
- `current_authority.source.inventory.time_invalid`;
- `current_authority.source.query.binding_mismatch`;
- `current_authority.source.query.availability_missing`;
- `current_authority.source.query.availability_ambiguous`;
- `current_authority.source.query.time_invalid`; and
- `current_authority.source.fact_set.invalid`.

Errors must not include grant IDs, requirement IDs, actor IDs, workflow IDs,
run IDs, step IDs, resources, hashes, timestamps, paths, or caller values.

## 14. Privacy And Redaction

The source must not store:

- target or evidence payloads;
- source or repository contents;
- policy inputs or approval prose;
- check output or command output;
- provider responses or logs;
- environment values;
- credentials, authorization headers, private keys, or tokens;
- filesystem paths or mount plans; or
- sandbox or process data.

Debug output should retain only model version, record counts, and bounded
posture. It must redact inventory hashes, timestamps, identities, resources,
and selected records.

Because the source is not serialized, there is no wire representation to
stabilize or expose.

## 15. Determinism

The same complete inventory and source observation time must produce the same
inventory hash regardless of input order.

The same exact contract, execution binding, inventory, and evaluation time
must produce the same fact-set hash.

Changing any grant, availability observation, source timestamp, query,
contract, or execution-binding commitment must change or invalidate the
result.

Canonical hashing must reuse the existing fixed-width domain framing pattern.
Do not introduce ad hoc concatenation.

## 16. Test Plan

Focused unit tests should prove:

1. a complete inventory produces a complete exact-query fact set;
2. inventory input order does not change the inventory or fact-set hash;
3. records outside the exact query remain committed by the source snapshot but
   are not returned in the selected fact set;
4. every matching grant candidate is retained;
5. revoked and expired grants are not hidden by source filtering;
6. zero matching grants remains a complete result;
7. exactly one availability record is returned for every exact query;
8. missing availability fails closed;
9. duplicate inventory availability fails closed;
10. duplicate grant IDs fail closed;
11. explicit unavailable and unknown observations are retained;
12. changed contract or execution binding fails closed;
13. future or inconsistent source/query times fail closed;
14. callers cannot supply query hashes, snapshot hashes, or completeness
    posture;
15. the source is not exported from the public crate surface;
16. Debug and errors do not leak protected values;
17. no raw payload fields exist;
18. a fixed v1 full-inventory hash vector remains stable;
19. fixed-width framing separates ambiguous domain and value pairs; and
20. existing current-authority, capability, required-context, immutable-run,
    approval, evidence, checks, SideEffect, provider, and runtime tests pass.

## 17. Compatibility Posture

The source is internal test infrastructure and has no compatibility guarantee.

The public `CurrentAuthorityFactSet` remains preview model vocabulary. This
phase does not expose its canonical bytes through schemas, SDKs, persistence,
or external APIs.

Any future production source requires separate planning for:

- source authentication and authorization;
- snapshot consistency or high-watermark semantics;
- freshness;
- durable source identity;
- concurrency and retry;
- trusted policy, approval, evidence, and check facts;
- audit and receipt projection; and
- operational failure handling.

## 18. Relationship To Proportional Governance

The source may eventually provide deterministic authority facts to
proportional-governance assessment. It must not invoke proportional governance
or choose quiet, visible, blocking, or denied posture in this phase.

Low-friction governance is safe only when authority completeness is proven.
The test source prepares that invariant without changing current behavior.

## 19. Relationship To Execution Providers

OpenShell or another execution provider may eventually consume a governed
execution commitment after authority, policy, and capability checks succeed.
This source does not create a sandbox policy, run a process, inject
credentials, access a target, or emit an enforcement receipt.

Execution-provider planning remains separate and must consume Workflow OS
authority rather than establish it.

## 20. Proposed Implementation Sequence

1. Add a test-only internal in-memory source module.
2. Canonicalize and commit the full owned inventory.
3. Derive the exact query set inside the source.
4. Select all matching grants and exact availability records.
5. Construct the existing fact-set model with source-derived commitments.
6. Add focused completeness, determinism, substitution, and privacy tests.
7. Run workspace validation.
8. Perform a focused maintainer review.
9. Plan the pure same-call time-of-use resolver only after review acceptance.

## 21. Open Questions

- Should a future production source use a trait or an explicit enum of
  supported source kinds?
- What authoritative source can prove complete grant and revocation inventory
  in a real local deployment?
- Should availability have a source-specific freshness policy or a single Core
  maximum age?
- Which source owns accepted policy, approval, evidence, and check facts?
- Should a future source snapshot be bound to the immutable run bundle before
  query execution?
- How should source refresh behave across approval resume and retry?
- What receipt proves that an execution provider consumed the exact authority
  result without exposing protected data?

## 22. Final Recommendation

Implement the **Core-owned in-memory current-authority source for tests only**.

Keep it private, synchronous, deterministic, payload-free, and incapable of
returning readiness or dereference authority. After implementation and focused
review, proceed to the pure same-call time-of-use resolver.

Do not implement runtime consumption, persistence, providers, OpenShell,
sandbox execution, SideEffects, writes, schemas, CLI, UI, hosted behavior,
reasoning lineage, or release changes.

Implementation status: the private test-only source and focused tests are
implemented in the
[Current Authority In-Memory Source Report](../concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_REPORT.md).
Focused maintainer review accepts the source boundary in the
[Current Authority In-Memory Source Review](../concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_REVIEW.md).
Runtime authority and same-call readiness remain deferred pending the pure
same-call resolver phase.
