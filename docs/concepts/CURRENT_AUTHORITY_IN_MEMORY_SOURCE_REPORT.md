# Current Authority In-Memory Source Report

## 1. Executive Summary

Workflow OS now has a private Core-owned in-memory current-authority source for
tests.

The source owns and commits a complete canonical grant and availability
inventory before query execution. It derives the exact query set from the
required-context contract, selects every grant candidate matching the exact
actor and immutable workflow/run/step/harness scope, requires exact
availability coverage, and constructs the existing payload-free
`CurrentAuthorityFactSet`.

The source is compiled only for `workflow-core` tests. It is not public,
serialized, persisted, or consumed by runtime code, and it exposes no
authorization, readiness, dereference, or execution API.

## 2. Scope Completed

- Added a test-only internal in-memory current-authority source.
- Added a private v1 source version.
- Consumed owned complete grant and availability inventories.
- Canonicalized the full inventory before queries.
- Rejected duplicate grant and availability identities.
- Rejected temporally inconsistent source inventories.
- Committed the full inventory with deterministic framed hashing.
- Derived exact queries from the required-context contract inside Core.
- Applied exact actor, workflow, run, step, and harness scope matching.
- Retained all matching lifecycle and specificity candidates.
- Required exactly one availability observation per query.
- Constructed the existing fact-set model with source-derived commitments.
- Added stable non-leaking `current_authority.source.*` errors.
- Added redaction-safe source Debug behavior.
- Added focused unit tests.

## 3. Scope Explicitly Not Completed

- No public source type, trait, constructor, or export was added.
- No source serialization or compatibility contract was added.
- No production authority source was selected.
- No authorization, permit, readiness, consumption, projection, or
  dereference result was added.
- No target payload was accessed.
- No runtime, executor, retry, or approval-resume behavior changed.
- No persistence, events, audit receipts, report artifacts, schemas, SDKs,
  CLI, UI, or examples were added.
- No policy, approval, evidence, or check fact source was added.
- No providers, OpenShell integration, sandbox execution, SideEffects, or
  writes were added.
- No hosted behavior, enterprise identity, reasoning lineage, or release
  posture changed.

## 4. Source API Summary

The private test module defines:

- `InMemoryCurrentAuthoritySourceInput`;
- `CurrentAuthoritySourceQueryInput`; and
- `InMemoryCurrentAuthoritySource`.

The source constructor accepts an observation time plus owned complete grant
and availability inventories. The query operation accepts only an exact
`RequiredContextExecutionBinding`, exact
`RequiredContextContractBinding`, and evaluation time.

Callers cannot supply a query set, filter, source snapshot hash, query-set hash,
or completeness posture.

## 5. Completeness Boundary

The source snapshot hash covers:

- private v1 source version;
- source observation time;
- every canonical grant in the owned inventory; and
- every canonical availability record in the owned inventory.

Records outside a later exact query still change the source snapshot hash.
Only records relevant to the exact actor and immutable execution scope enter
the selected fact set.

`CompleteForExactQuery` therefore means the private test source returned every
matching record from its owned fixture inventory. It remains explicitly
insufficient as production source authority.

## 6. Grant And Availability Behavior

Grant selection requires:

- exact actor;
- exact capability and resource;
- exact workflow;
- matching optional run;
- matching optional step; and
- matching optional harness contract.

The source does not hide matching grants because they are revoked, expired,
prerequisite-bearing, delegated, sensitivity-limited, or less specific. A
future resolver remains responsible for those decisions.

Availability selection requires one exact record per query. Missing or
duplicate records fail closed. Explicit disconnected, unsupported, and unknown
postures remain complete facts without becoming readiness.

## 7. Determinism

The full-inventory hash is independent of caller input order.

A fixed v1 inventory hash vector protects the canonical representation:

```text
7a6b4d1950768957abc7807420bed208a8267592d5a99f5cb842b3ac1b67bf2e
```

A framing regression proves that ambiguous domain/value concatenations do not
collide.

## 8. Privacy And Security

The source stores typed grants and payload-free availability observations. It
contains no target contents, source files, commands, outputs, provider
payloads, credentials, paths, policy inputs, approval prose, evidence
payloads, check output, sandbox data, or SideEffect payloads.

Debug output redacts timestamps, hashes, identities, resources, and records.
Errors use stable codes and bounded messages without caller values.

Unrelated actor and execution-scope grants remain committed by the source
snapshot but are excluded from the returned fact set.

## 9. Test Coverage

Focused tests prove:

- complete exact-query fact-set construction;
- exact actor and scope filtering;
- revoked and lower-specificity candidate retention;
- expired candidate retention;
- complete zero-grant results;
- input-order-independent source hashing;
- source commitment to out-of-query records;
- missing availability failure;
- duplicate availability and grant failure;
- explicit disconnected, unsupported, and unknown availability retention;
- source and query temporal consistency;
- contract substitution failure;
- Debug and error non-leakage;
- fixed v1 inventory hash; and
- collision-resistant framing.

## 10. Validation

- `cargo fmt --all --check`: passed.
- focused source unit tests: passed, 9 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.

## 11. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1785149347643603000-2`
- approval ID:
  `approval/run-1785149347643603000-2/implementation-approved`
- presentation ID: `presentation/c7078d2d85c546d1`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted implementation handoff was presented
- phase status: completed
- event summary: 39 events; 1 approval; 0 retries; 0 escalations; presentation
  proof enforced with one persisted presentation record and event marker
- out-of-kernel work: Rust implementation, tests, documentation edits, and
  validation were performed by the delegated maintainer; the kernel governed
  scope and approval but did not edit files, invoke cargo or npm, or mutate git

## 12. Remaining Limitations

- Completeness is proven only over a private test fixture inventory.
- No production source identity, authentication, high-watermark, freshness,
  concurrency, or retry semantics exist.
- Policy, approval, evidence, and check fact sources remain deferred.
- No same-call resolver interprets the fact set.
- No runtime consumer or execution provider can use the source.

## 13. Recommended Next Phase

Perform a focused maintainer review of the source implementation.

If accepted, plan and implement the pure same-call time-of-use resolver without
target dereference or runtime integration.
