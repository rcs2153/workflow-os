# Registered Current-Authority Source Resolver Composition Review

## 1. Executive Verdict

Phase accepted; proceed to one-time-use and replay-posture planning.

The private composition establishes a credible same-call source-backed
assessment without exporting source records, caller-asserted readiness, or a
reusable authorization handle. No blocker was found.

## 2. Scope Verification

The phase stayed within the approved private composition scope.

It added one crate-private operation that reads the registered source,
constructs the exact fact set, resolves capability authority, rebuilds
step-scoped projections, reruns required-context consumption, and returns a
bounded private assessment.

It did not add public readiness, executor integration, target dereference,
persistence, events, audit projection, providers, OpenShell, sandbox
execution, SideEffect execution, writes, schemas, SDKs, CLI behavior, hosted
behavior, reasoning lineage, or release changes.

## 3. Source Trust Assessment

The composition invokes the Core-owned private registered source directly.
Callers cannot supply:

- a trusted source registration;
- a prefiltered query;
- selected authority records;
- a caller-built fact set;
- a source snapshot to replay; or
- a prior assessment to reuse.

The private source derives its exact request from the immutable execution
binding and required-context contract. Its completeness, consistency, and
freshness failures short-circuit before authority resolution.

## 4. Snapshot And Selected-Record Integrity

The registered source now has an internal selection outcome that carries the
coherent payload-free source snapshot together with the exact selected grants,
availability records, and governed context references.

The public read path still exposes only the snapshot or bounded failure. The
composition consumes selected records inside the source module and builds the
fact set from the same selection. No record leaves the private trust boundary,
and no independent public snapshot commitment is treated as authority.

The source snapshot and resulting fact-set commitments are both bound into the
private assessment commitment.

## 5. Exact Binding And Time Assessment

The request and fact-set constructors validate the exact:

- immutable execution binding;
- contract identity, version, and content hash;
- actor, workflow, run, step, and harness scope;
- canonical required query set;
- source observation time; and
- injected evaluation time.

There is no hidden clock read. Future-dated or stale source posture returns a
source failure rather than permission.

## 6. Capability And Projection Assessment

Each contract requirement derives its capability and resource from the
validated requirement rather than caller input. Capability resolution uses
the exact execution scope, requested sensitivity, evaluation time, complete
selected grants, and complete availability records.

Unresolved policy, approval, evidence, and check prerequisites are retained as
typed reasons. The existing governed-context projection constructor remains
the authority gate: a non-authorized capability result cannot project a
required reference.

The composition groups candidates by requested access level and uses the
existing step-scoped projection helper. It does not access context payloads or
turn references into dereference authority.

## 7. Required-Context Assessment

The composition rebuilds `RequiredContextConsumptionContext` from the exact
immutable execution binding and injected time, then consumes the exact
required-context contract through the existing constructor.

Required gaps produce `Blocked`. Optional gaps remain explicit reasons while
preserving the accepted non-blocking semantics for a contract whose required
obligations are satisfied.

The `Ready` reason appears only when no gap or independent-prerequisite reason
exists. A revoked grant and an unresolved approval prerequisite both fail to
produce required readiness in focused tests.

## 8. Result And Replay Assessment

The assessment is crate-private, non-`Clone`, and not serializable. Its Debug
implementation redacts all commitments and time. It contains no target data
and exposes no dereference or execution method.

This is sufficient for the current proof, but it is not a one-time-use
capability. A future runtime consumer must define freshness, replay,
reassessment, and use-consumption semantics rather than treating the
assessment commitment as a lease.

## 9. Error And Privacy Assessment

Source failures remain typed, bounded, and payload-free. Resolution,
projection, consumption, fact-set, and assessment construction failures map to
stable non-leaking error codes.

The composition does not read or retain provider payloads, command output,
source contents, credentials, environment values, endpoints, raw
configuration, or unbounded errors. Debug output does not expose source IDs,
context targets, grant IDs, timestamps, or commitments.

## 10. Test Quality Assessment

Focused tests cover:

- complete source-backed readiness;
- source failure before resolution;
- unresolved approval prerequisites;
- revoked grants;
- canonical inventory-order determinism; and
- assessment Debug non-leakage.

Existing source and same-call resolver tests continue to cover contract and
binding mismatch, timestamps, stale sources, missing and optional context,
independent prerequisites, sensitivity, projection, required consumption,
canonical hashing, and error safety.

Non-blocking follow-ups before persistence or runtime consumption:

- add a fixed v1 assessment-commitment vector;
- add direct registered-composition regressions for contract substitution,
  expired source validity, multiple matching candidates, and sensitivity;
- consider consolidating duplicated private orchestration only if it can keep
  selected records inside the registered-source trust boundary; and
- define explicit one-time-use and replay semantics.

## 11. Documentation Assessment

The roadmap, production source-boundary plan, and implementation report
accurately describe a private in-memory composition.

They do not claim production current authority, runtime readiness, target
dereference, executor use, provider execution, OpenShell integration,
SideEffect execution, writes, persistence, schemas, hosted behavior, or
reasoning lineage.

The phase remains correctly sequenced beneath proportional governance and
quiet success. Lower-friction governance must depend on current authority,
not caller-classified input or a stale stored assessment.

## 12. Blockers And Follow-Ups

Blockers: none.

Non-blocking follow-ups:

- define one-time-use, replay, and reassessment posture;
- add fixed commitment and direct negative-path coverage;
- keep trusted source registration and invocation private;
- do not make the private assessment a public capability token;
- preserve independent prerequisite verification;
- keep OpenShell as an optional future execution provider rather than an
  authority source; and
- do not connect quiet success until a reviewed runtime consumer preserves
  these boundaries.

## 13. Recommended Next Phase

Plan one-time-use and replay posture for a future private source-backed
assessment.

The plan should define when assessment freshness expires, which changes force
re-resolution, whether an assessment may be consumed once or multiple times,
how retries and approval resume obtain current authority, and what a future
read-only consumer may observe without turning commitments into ambient
permission.

Do not implement executor integration, dereference, persistence, providers,
OpenShell, sandbox execution, SideEffect execution, schemas, CLI behavior, or
writes.

## 14. Governed Review Record

- workflow: `dg/review`
- run ID: `run-1785172031301005000-2`
- approval ID:
  `approval/run-1785172031301005000-2/review-scope-approved`
- approval presentation ID: `presentation/d6a94d24dc77e0e5`
- approval presentation content hash:
  `d6a94d24dc77e0e57eff53d140534cab42c61d679e9dfb74934f516472426b12`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- review status: accepted and governed phase completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with event marker
- out-of-kernel work: the delegated maintainer independently inspected the
  implementation, tests, documentation, and prior validation evidence and
  authored this review; the kernel governed scope and approval but did not
  inspect code, edit files, execute checks, or mutate git
