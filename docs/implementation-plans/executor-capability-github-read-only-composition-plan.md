# Executor Capability-Gated GitHub Read-Only Composition Plan

Status: Implementation complete. This plan selected the first executor-facing
composition of the accepted, crate-private capability-gated GitHub repository
metadata read. The implementation is recorded in the
[Executor Capability-Gated GitHub Read-Only Composition Report](../concepts/EXECUTOR_CAPABILITY_GITHUB_READ_ONLY_COMPOSITION_REPORT.md).
Focused maintainer/security review remains a separate governed phase.

Related foundations:

- [Scoped Capability GitHub Read-Only Integration Plan](scoped-capability-github-read-only-integration-plan.md)
- [Capability Source Continuity Plan](capability-source-continuity-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)
- [Authoritative Agent Continuation Context And Rehydration Plan](authoritative-agent-continuation-context-rehydration-plan.md)
- [Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md)
- [Terminal Local Report Generation Plan](terminal-local-report-generation-plan.md)

## 1. Executive Summary

Workflow OS has accepted the pieces required for one governed read-only
provider operation: immutable run identity, durable event state, policy
evaluation, current scoped capability authority, step-scoped projection, a
GitHub read-only adapter, a payload-free capability-use receipt, and Core-owned
durable source continuity. Those pieces are not yet composed into an executor
path.

The next implementation should add one crate-private, explicit, opt-in
executor operation for the existing GitHub repository-metadata read. Core must
rehydrate the current run and immutable bundle, prove the exact current step and
cursor, resolve current authority, atomically reconcile source continuity,
consume one cursor-bound continuation claim, invoke the existing injected
read-only adapter once, and project bounded runtime evidence.

This phase is deliberately not a generic adapter framework. It adds no second
provider operation, provider mutation, workflow schema, CLI command, automatic
handler registration, ambient credentials, or default execution behavior.

## 2. Why This Phase Is Next

The active roadmap requires runtime composition of accepted foundations rather
than another disconnected model family. The private repository-metadata helper
already proves the authority and provider-call boundary, and its final known
source-continuity blocker is closed. Leaving it callable only from focused tests
would preserve the gap between documented governance and executor behavior.

The selected operation is low-risk and information-preserving:

- it is read-only;
- it already uses an injected adapter client;
- it has exact canonical repository identity;
- it already returns bounded telemetry and a payload-free receipt;
- it has durable rollback and substitution protection; and
- it does not require provider-write authority.

## 3. Goals

- Compose the existing GitHub repository-metadata read into one real local
  executor boundary.
- Require an existing immutable-bundle-backed run and exact current step.
- Rehydrate current run state instead of trusting caller or agent memory.
- Bind the operation to the exact event cursor, actor, workflow, run, step,
  harness, capability, repository resource, and correlation identity.
- Resolve capability authority from the registered Core source in the same
  governed call.
- Derive first, replayed, or advanced source posture from durable SQLite state.
- Consume one durable cursor-bound continuation claim before provider entry.
- Invoke only the existing `GitHubReadOnlyAdapter::read_repository_metadata`
  operation through an injected client.
- Preserve truthful distinctions between blocked-before-read, provider failure,
  read success, and read-success/receipt-failure.
- Project bounded adapter telemetry and a stable evidence reference without
  copying provider payloads.
- Preserve existing executor, adapter, and report APIs unless the new path is
  explicitly selected.

## 4. Non-Goals

This plan does not authorize:

- a second GitHub read operation or another provider;
- GitHub comments, pull requests, issues, merges, or any provider mutation;
- default capability gating for all adapters;
- changing ordinary `LocalExecutor::execute` behavior;
- generic callback authority or a reusable capability lease;
- workflow, policy, skill, harness, schema, SDK, or example changes;
- CLI or UI exposure;
- runtime configuration or credential discovery;
- live-network acceptance tests;
- report artifact persistence;
- hosted identity, enterprise RBAC, OpenShell, nested harnesses, or reasoning
  lineage;
- automatic grants, automatic approvals, or model judgment as authority;
- release posture changes.

## 5. Selected Runtime Boundary

The first slice should add one crate-private operation adjacent to
`LocalExecutor`, conceptually:

```text
execute_current_github_repository_metadata_read(...)
    -> LocalGitHubRepositoryMetadataReadExecutionResult
```

The operation acts on an already-created immutable-bundle-backed local run. It
must not create a second run, execute unrelated steps, or operate after a
terminal state.

The boundary should remain crate-private until focused review proves its
ordering, failure, replay, privacy, and event semantics. A later public or CLI
surface requires separate compatibility planning.

## 6. Why The Generic Skill Handler Is Insufficient

The current `SkillHandler` receives `SkillInput` containing a run ID and mapped
string values. That is intentionally useful for local deterministic handlers,
but it cannot independently prove the complete authoritative context required
for this provider operation:

- actor;
- workflow and version;
- immutable bundle root;
- exact current step;
- harness contract;
- current event cursor;
- current source registration and continuity;
- exact policy-precheck provenance; and
- selected capability grant and projection.

Passing those facts through string input would turn caller-authored values into
authority. The first composition must therefore remain a closed Core operation,
not a generic handler convention or an adapter ID branch that silently changes
ordinary execution.

## 7. Explicit Input Contract

The crate-private input should contain or reference:

- SQLite state backend;
- exact workflow run ID;
- expected current event sequence and event ID;
- expected immutable run-bundle binding;
- expected actor and correlation ID;
- exact current step ID;
- exact harness contract ID and version;
- registered current-authority source;
- evaluation timestamp;
- requested sensitivity;
- redaction metadata;
- canonical repository owner and repository segments;
- the existing policy precheck;
- the injected `GitHubReadOnlyAdapter`; and
- one stable operation/idempotency identity.

The caller may select the requested target and supply injected dependencies,
but Core must derive and compare every authoritative run, bundle, step, policy,
source, and capability fact before provider entry.

## 8. Core Rehydration And Admission

The operation should perform this closed sequence:

1. Rehydrate the run from SQLite.
2. Reject terminal, corrupt, legacy-unbundled, or wrong-backend posture.
3. Read and validate the exact immutable run bundle bound to the run.
4. Verify actor, workflow, run, step, harness, and correlation binding.
5. Require the selected step to be the current actionable step.
6. Require the step's validated definition to select the accepted GitHub
   repository-metadata operation contract.
7. Re-evaluate policy independently; a capability grant must not imply policy
   success.
8. Resolve the registered current-authority exact-query fact set.
9. Reconcile source generation and watermark through the accepted SQLite V8
   continuity store.
10. Derive the exact step-scoped capability projection.
11. Construct the governed continuation brief for this operation and consume
    one durable cursor-bound claim.
12. Re-read the current cursor before provider invocation.
13. Invoke the existing adapter once.
14. Project the bounded result, telemetry, receipt, and evidence posture.

Any failure through step 12 must produce zero provider calls.

## 9. One-Time Continuation And Replay

Source continuity and operation idempotency solve different problems:

- source continuity proves whether the current authoritative fact source is
  first observed, replayed, or advanced; and
- the continuation claim proves that only one consumer may act for the exact
  run cursor and operation binding.

The implementation must use both. It must not treat an accepted source replay
as permission to repeat a provider call.

Exact retry after an ambiguous local commit must reconcile durable state. A
stale cursor burns the claimed operation and requires fresh rehydration. A
provider read failure may be retried only through a newly authorized operation
attempt; the prior source observation remains valid. A successful read with a
failed receipt must remain explicitly ambiguous for downstream evidence and
must not be reported as if no provider call occurred.

## 10. Runtime Event And Audit Posture

The first implementation must use existing event vocabulary where it is
truthful. It should not invent a completed workflow merely because the read
succeeds.

At minimum, the operation must preserve:

- one exact invocation identity;
- the existing adapter telemetry record;
- provider-called versus provider-not-called posture;
- bounded success, failure, and receipt-failure outcomes; and
- ordered correlation to the current workflow step.

If current event vocabulary cannot represent this without claiming a generic
skill completed, implementation must stop and return a planning blocker rather
than add misleading events. New event vocabulary requires a separate reviewed
scope.

## 11. Evidence And Report Posture

Successful composition should derive one bounded `EvidenceReference` from the
existing capability-use receipt and adapter invocation identity. The evidence
must cite stable references only and must not copy:

- repository metadata payloads;
- repository owner or name into report summaries by default;
- source inventories;
- capability grants;
- policy payloads;
- credentials or headers; or
- provider error bodies.

The first slice may return the evidence reference in memory. It must not write a
report artifact or persist a new receipt family. Terminal report generation may
consume the reference only through an explicit later or existing input path.
If the current evidence model cannot represent the receipt truthfully without
new public taxonomy, keep the receipt and telemetry separate and record that as
a focused implementation limitation rather than fabricate evidence.

## 12. Failure Semantics

The result must preserve these domains:

- `BlockedBeforeRead`: no provider call occurred;
- `ReadFailed`: the provider call failed;
- `Succeeded`: read, projection, receipt, and bounded evidence succeeded; and
- `ReadSucceededReceiptFailed`: the provider read completed but receipt or
  evidence construction failed.

Executor admission, policy, authority, continuity, cursor, and claim failures
must use stable non-leaking error codes. Errors must not include repository
names, paths, source values, grants, hashes, tokens, headers, provider payloads,
or raw command output.

No operation outcome may retroactively rewrite unrelated workflow pass/fail
semantics. The run changes only through explicit accepted event semantics.

## 13. Privacy And Redaction

- Use existing validated constructors and redaction metadata gates.
- Keep source and provider payloads out of events, receipts, evidence summaries,
  Debug, and errors.
- Treat repository identity as sensitive at this boundary even though GitHub
  repository names may be public.
- Store only commitments, bounded identifiers, enumerated posture, timestamps,
  and stable references.
- Preserve the current unverified-on-deserialization receipt posture.
- Extend SQLite privacy tests to main, WAL, and shared-memory files if this
  phase adds any durable operation projection.

## 14. Test Plan

Focused tests should prove:

1. one exact active immutable-bundle-backed step performs one injected read;
2. ordinary executor APIs remain unchanged;
3. terminal, wrong-step, wrong-actor, wrong-run, wrong-harness, wrong-bundle,
   wrong-cursor, and wrong-correlation inputs fail before provider entry;
4. policy denial remains independent from capability authority;
5. stale, incomplete, revoked, unavailable, or prerequisite-bearing authority
   fails before provider entry;
6. first source observation, exact replay, and monotonic advancement work;
7. rollback, missing generation, same-generation substitution, query
   substitution, and registration substitution fail before provider entry;
8. concurrent exact consumers produce at most one provider call;
9. stale-after-claim cursor posture produces zero provider calls;
10. adapter failure remains distinct from blocked-before-read;
11. read-success/receipt-failure does not invite blind retry;
12. successful output includes bounded telemetry, receipt, and truthful evidence
    posture;
13. no provider, source, grant, repository, credential, or command payload is
    copied into events, errors, Debug, evidence, or serialization;
14. restart replay reconstructs durable source and operation posture;
15. SQLite main, WAL, and shared-memory privacy scans find no forbidden marker;
16. existing executor, immutable bundle, current authority, capability,
    continuation, adapter, evidence, report, and state tests pass; and
17. the workspace format, Clippy, tests, and documentation checks pass.

## 15. Implementation Sequence

1. Add the crate-private input and closed result vocabulary.
2. Add rehydration and immutable binding validation without provider entry.
3. Compose policy, current authority, durable source continuity, and
   cursor-bound one-time continuation.
4. Invoke the existing injected read-only adapter.
5. Map bounded telemetry, receipt, and evidence posture.
6. Add focused race, restart, failure-ordering, and privacy tests.
7. Update the roadmap and create an implementation report.
8. Perform focused maintainer/security review before any public adoption or
   second operation.

## 16. Acceptance Criteria

- One crate-private opt-in executor operation composes the accepted GitHub
  repository-metadata read.
- Core rehydrates and validates current run, immutable bundle, step, cursor,
  policy, authority, and source continuity before provider entry.
- One exact continuation claim permits at most one provider call.
- Stable telemetry, receipt, and evidence posture are returned without payload
  copying.
- Existing executor and adapter APIs remain unchanged by default.
- No writes, schemas, CLI, runtime config, hosted behavior, or second provider
  operation are introduced.

## 17. Open Questions For Focused Review

- Can existing invocation event vocabulary represent this dedicated operation
  without falsely claiming a generic skill result?
- Should bounded evidence be created inside the operation or returned as a
  receipt-plus-telemetry pair for an existing report builder to cite?
- Which existing continuation action code can truthfully bind this operation,
  or is one additive closed code required?
- Can the current SQLite transaction boundaries preserve an exact operation
  claim and event projection without expanding the V8 migration?
- What is the smallest exact validated step declaration that selects this
  operation without a schema change?

## 18. Final Recommendation

Proceed next to focused maintainer/security review of this plan. If the review
accepts the event, operation-selection, evidence, and transaction boundaries,
implement this one crate-private executor composition. Continue to defer a
second provider operation, every provider mutation, public/default behavior,
CLI/schema exposure, hosted identity, OpenShell, and nested harness execution.
