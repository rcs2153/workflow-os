# Current Authority In-Memory Source Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking follow-ups.**

The plan defines a narrow internal test source that can prove complete
exact-query selection from one owned canonical inventory without creating a
public authority source or runtime readiness API.

Review found one blocker in the initial draft: grant selection would have
retained capability/resource matches without applying the exact actor and
execution-scope predicate. The plan now requires the same actor, workflow, run,
step, and harness matching boundary as capability resolution while preserving
all matching scope specificities and decision-relevant lifecycle states.

The review also restored fixed v1 hash-vector and framing regressions as
required tests. With those corrections, the plan is implementation-ready.

## 2. Scope Assessment

The plan remains limited to:

- one private `#[cfg(test)]` source inside `workflow-core`;
- owned in-memory grant and availability inventories;
- exact contract-derived queries;
- deterministic filtering and snapshot commitments;
- construction of the existing model-only fact set; and
- focused unit tests.

It does not authorize:

- a public source API or trait;
- authorization, readiness, consumption, projection, or dereference;
- runtime or executor integration;
- persistence, events, receipts, artifacts, schemas, SDKs, CLI, UI, or
  examples;
- providers, OpenShell, sandbox execution, SideEffects, or writes; or
- hosted behavior, reasoning lineage, or release changes.

## 3. Trust Boundary Assessment

Keeping the source private and test-only is the correct first boundary.

The source owns vectors before accepting a query, derives the exact query set
inside Core, computes its own snapshot hash, and supplies its own completeness
posture. Callers cannot pass a result slice, query hash, snapshot hash, or
completeness flag to the query operation.

The source remains a test proof over a caller-created fixture inventory. It
does not establish a production trust root, and the public
`CurrentAuthorityFactSet::new` path remains caller-claim-only.

## 4. Inventory Assessment

The plan correctly commits the complete canonical source inventory, including
records outside a later exact query. This distinguishes a source snapshot from
a hash of the selected result slice.

Construction consumes owned records, canonicalizes them, rejects duplicate
grant IDs and availability keys, and binds one observation time.

A private version marker and fixed hash domain keep the test source
deterministic without exposing a compatibility contract.

## 5. Query And Grant Assessment

The exact query is derived from every required and optional contract
requirement. Callers cannot omit requirements or supply their own filters.

After correction, grant selection requires:

- exact actor subject;
- exact capability and resource;
- exact workflow;
- matching optional run, step, and harness scopes.

Every grant satisfying that identity boundary remains in the fact set. The
source does not hide revoked, expired, prerequisite-bearing, delegated,
sensitivity-limited, or lower-specificity candidates. This preserves facts for
the future resolver without leaking unrelated actors' grants.

## 6. Availability Assessment

The source requires exactly one matching availability observation for each
query. It rejects missing or duplicate records and never synthesizes an
availability posture.

Explicit `DeclaredNotConnected`, `KnownUnsupported`, or `Unknown`
observations can be complete source facts. They do not become readiness.

Freshness policy remains correctly deferred. The first phase checks temporal
consistency only.

## 7. Output And Authority Assessment

The source constructs the existing `CurrentAuthorityFactSet` only after source
validation and exact selection.

No authority method is introduced. The result cannot:

- authorize;
- permit;
- declare readiness;
- consume;
- project;
- dereference; or
- execute.

No production consumer can instantiate the test source through the public
crate API.

## 8. Determinism Assessment

The plan requires:

- input-order-independent inventory hashes;
- exact-query deterministic selected records;
- full-inventory snapshot binding;
- fixed-width domain framing;
- a fixed v1 source-inventory hash vector; and
- substitution-sensitive fact-set hashes.

These are appropriate regressions for a future same-call comparison boundary.

## 9. Privacy Assessment

The source contains typed grant records and payload-free availability
observations only. It has no fields for raw target data, source contents,
commands, outputs, provider payloads, credentials, paths, policy payloads,
approval prose, evidence payloads, check output, or sandbox data.

Private Debug output and stable errors must redact identities, resources,
hashes, timestamps, and caller values.

Filtering unrelated actor and execution scopes before output further reduces
unnecessary identity exposure.

## 10. Test Assessment

The planned tests cover:

- complete exact-query construction;
- full-inventory versus selected-slice binding;
- canonical input ordering;
- every matching grant candidate;
- revoked and expired candidate retention;
- zero-grant completeness;
- exact availability coverage;
- missing and duplicate source facts;
- explicit unavailable and unknown facts;
- binding and contract substitution;
- temporal inconsistency;
- absence of caller-controlled source commitments;
- non-exported source posture;
- fixed hash and framing regressions;
- Debug and error non-leakage; and
- adjacent regression suites.

No test is expected to claim production source trust or runtime readiness.

## 11. Blockers

None remaining.

The actor and execution-scope filtering blocker was corrected in the plan
before implementation.

## 12. Non-Blocking Follow-Ups

- A future production source needs authenticated source identity, snapshot or
  high-watermark semantics, freshness, concurrency, and operational failure
  behavior.
- Policy, approval, evidence, and check facts need independently trusted
  sources rather than booleans.
- The future same-call resolver should reuse one canonical request-matching
  predicate rather than allowing source and resolver semantics to drift.
- A future source trait should wait for at least one real source consumer.

## 13. Recommended Next Phase

Implement the private Core-owned in-memory current-authority source and focused
tests.

Continue to defer time-of-use readiness, dereference, runtime integration,
persistence, providers, OpenShell, sandbox execution, SideEffects, writes,
schemas, CLI, UI, hosted behavior, reasoning lineage, and release changes.

## 14. Validation

- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.

## 15. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785149173154176000-2`
- approval ID:
  `approval/run-1785149173154176000-2/review-scope-approved`
- presentation ID: `presentation/09aeaa1e7ca5162b`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events; 1 approval; 0 retries; 0 escalations; presentation
  proof enforced with one persisted presentation record and event marker
- out-of-kernel work: plan inspection, blocker correction, review writing, and
  validation were performed by the delegated maintainer; the kernel governed
  scope and approval but did not edit files, run checks, or mutate git
