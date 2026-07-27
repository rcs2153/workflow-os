# Required Context Current Authority Fact-Set Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation provides a deterministic, payload-free commitment to the
grant and availability facts supplied for one exact required-context contract.
It derives its query set from the contract, rejects noncanonical or
out-of-scope records, and does not expose an authority, readiness, or
dereference result.

The review found one test blocker: the approved plan required a fixed v1 hash
vector and an explicit framing regression in the first implementation. Both
tests were added and pass before this verdict.

## 2. Scope Verification

The phase stayed within its approved model-only scope.

Implemented:

- exact query derivation from every contract requirement;
- a versioned current-authority fact-set commitment;
- caller-owned in-memory source-snapshot vocabulary;
- canonical grant and availability retention;
- claimed exact-query completeness with structural coverage validation;
- deterministic framed hashes and fail-closed serde;
- redaction-safe Debug behavior; and
- focused tests and honest documentation.

Not introduced:

- trusted source completeness;
- an authority, permit, readiness, or dereference API;
- a source or store implementation;
- accepted policy, approval, evidence, or check fact composition;
- runtime or executor integration;
- persistence, events, receipts, artifacts, schemas, SDKs, CLI, UI, or
  examples;
- providers, OpenShell, sandbox execution, SideEffects, or writes;
- hosted behavior, reasoning lineage, or release changes.

## 3. Query Model Assessment

`CurrentAuthorityQuerySet` is derived from the complete typed
`RequiredContextContractBinding`; callers cannot omit an inconvenient
requirement.

Each query retains the requirement ID, typed target, access level, obligation,
sensitivity ceiling, derived capability, and derived resource. Validation
recomputes capability and resource derivation and rejects unknown sensitivity,
duplicate targets, unordered requirements, and query-hash substitution.

This is the correct exact-query boundary for a future completeness-capable
source.

## 4. Source And Completeness Assessment

`AuthorityFactSourceBinding` commits:

- source kind;
- snapshot hash;
- observation time;
- claimed completeness;
- exact query-set hash;
- record counts; and
- canonical records hash.

`CompleteForExactQuery` remains explicitly a caller-owned claim. Public
construction and valid deserialization prove only internal commitment
consistency. No API interprets that posture as trusted authority.

That boundary is safe for this model-only phase. A future consumer must accept
the posture only from a Core-owned source that owns the complete relevant
inventory and compares the exact query and snapshot in the same call.

## 5. Grant And Availability Assessment

Grants are retained with their existing typed lifecycle, scope, prerequisite,
delegation, and sensitivity posture. The aggregate does not independently
reinterpret grant authority.

Availability records remain distinct from grants. Claimed exact completeness
requires one unambiguous availability record for every exact query. Missing,
duplicate, extra, out-of-query, and noncanonical records fail closed.

The aggregate correctly permits zero grants. Absence of a grant is a fact that
a future resolver must evaluate; the fact-set model must not invent one.

## 6. Canonicalization And Hash Assessment

Construction canonicalizes:

- queries by requirement ID;
- grants by grant ID; and
- availability by capability and resource identity.

Validation now rejects noncanonical wire ordering before checking source and
aggregate hashes. This prevents an external producer from minting a different
self-consistent ordering for the same logical record set.

Hashes are SHA-256 commitments over canonical serde bytes with domain labels
and fixed-width length framing. The review added:

- a fixed known vector for the v1 aggregate fact-set hash; and
- an explicit regression proving ambiguous domain/value pairs do not collide
  through concatenation.

Before schema, SDK, persistence, or cross-version compatibility exposure, the
canonical serde shape must remain versioned and change-controlled.

## 7. Validation And Error Assessment

Validation fails closed for:

- contract and immutable execution-binding mismatch;
- invalid observation and evaluation ordering;
- invalid, duplicate, unordered, or incorrectly derived queries;
- duplicate, unordered, or out-of-query grants;
- duplicate, unordered, missing, or out-of-query availability;
- inconsistent query hashes, record counts, record hashes, or aggregate hash;
  and
- unknown wire enum values.

Errors use stable `current_authority.fact_set.*` codes without caller values.
Serde presents bounded generic failures rather than rejected identities,
hashes, resources, paths, or secret-like values.

## 8. Privacy And Redaction Assessment

The model stores typed identities, hashes, postures, timestamps, counts,
grants, and payload-free availability observations.

It has no fields for raw source or target content, provider payloads, policy
inputs, approval prose, evidence payloads, check output, commands, parser
payloads, environment values, credentials, logs, paths, or sandbox data.

Debug output redacts binding, source, query, record, resource, timestamp, and
aggregate identities while retaining bounded versions, postures, kinds, and
counts.

## 9. Authority Boundary Assessment

The implementation exposes no:

- `authorize`;
- `permit`;
- `ready`;
- `consume`;
- `project`; or
- `dereference`

operation.

It therefore cannot turn a caller-owned snapshot or deserialized wire object
into runtime authority. This is the most important safety property of the
phase.

Policy, approval, evidence, checks, freshness, sensitivity composition, and
SideEffect constraints remain independent future facts. They must not be
collapsed into caller booleans in the next phase.

## 10. Test Quality Assessment

Tests cover:

- complete exact-query derivation;
- multiple typed requirements;
- valid construction and serde round trip;
- missing and duplicate availability;
- noncanonical serialized ordering;
- serialized tampering without value leakage;
- fixed v1 aggregate hash;
- framing separation;
- Debug non-leakage; and
- payload-free serialized shape.

Adjacent required-context, immutable-bundle, capability-authority, executor,
provider, report, adapter, and workspace suites pass.

Non-blocking test gaps:

- no trusted source exists yet to prove claimed completeness;
- no freshness policy is evaluated;
- no accepted policy, approval, evidence, or check fact set is composed; and
- no future consumer test compares a fact set to its validated immutable
  execution binding in the same authority call.

## 11. Documentation Review

The roadmap, plan, phase report, and adjacent required-context plans accurately
state:

- the fact-set core model is implemented;
- source completeness is claimed and non-authoritative;
- time-of-use readiness and dereference are not implemented;
- source/store ownership remains open; and
- runtime integration, providers, OpenShell, sandbox execution, SideEffects,
  writes, schemas, CLI, UI, hosted behavior, and release changes remain
  unsupported by this phase.

No documentation overclaims current execution capability.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test current_authority_fact_set`: passed,
  5 tests.
- fixed framing unit regression: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed before the review-only regression additions;
  production behavior was unchanged, and the new focused tests and workspace
  clippy passed afterward.
- `npm run check:docs`: passed before this review.
- `git diff --check`: passed before this review.

## 13. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785147357439137000-2`
- approval ID:
  `approval/run-1785147357439137000-2/review-scope-approved`
- presentation ID: `presentation/555c14ae9546984a`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events; 1 approval; 0 retries; 0 escalations; presentation
  proof enforced with one persisted presentation record and event marker
- out-of-kernel work: implementation and test inspection, focused blocker
  correction, review writing, documentation updates, and validation were
  performed by the delegated maintainer; the kernel governed scope and
  approval but did not edit files, invoke cargo, or mutate git

## 14. Blockers

None remaining.

The missing known-vector and framing regressions were corrected during the
approved review scope.

## 15. Non-Blocking Follow-Ups

- Keep `CompleteForExactQuery` non-authoritative outside a future Core-owned
  source boundary.
- Add validated source bindings for policy, approval, evidence, and checks
  rather than booleans.
- Define freshness semantics before a fact set can support readiness.
- Treat canonical serde bytes as an internal preview format until a separate
  compatibility phase.
- Add read-only bounded source accessors only when a concrete source consumer
  requires them.

## 16. Recommended Next Phase

Proceed to a **Core-owned in-memory current-authority source model for tests
only**.

That phase should own a complete bounded inventory, answer one exact query set,
produce a source-bound fact set, and prove that callers cannot mint trusted
completeness by supplying arbitrary slices.

Continue to defer authoritative time-of-use readiness, dereference, executor
integration, persistence, providers, OpenShell, sandbox execution, SideEffects,
writes, schemas, CLI, UI, hosted behavior, reasoning lineage, and release
changes.
