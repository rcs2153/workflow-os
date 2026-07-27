# Required Context Contract Consumption Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed with non-blocking follow-ups.**

The independent execution-context binding blocker identified in
[Required Context Contract Consumption Review](REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_REVIEW.md)
is fixed. Required-context consumption now accepts and retains an explicit
actor, workflow, run, step, harness, and evaluation time and requires every
projection to match that context exactly.

The phase remains a pure, payload-free model/helper boundary. It does not
dereference context, integrate with the executor, invoke providers or
sandboxes, execute SideEffects, or authorize writes.

## 2. Scope Verification

The fix stayed within the approved blocker scope.

Implemented:

- one explicit `RequiredContextConsumptionContext`;
- input and result retention of that context;
- exact projection comparison against the independent context;
- contract-to-context harness equality;
- validation and deserialization recomputation;
- stable non-leaking mismatch errors;
- focused mismatch and serialized-substitution tests; and
- honest phase documentation.

Not introduced:

- target or payload dereference;
- repository or source inspection;
- executor or runtime integration;
- persistence, events, audit records, or authority receipts;
- schemas, SDKs, CLI behavior, UI, or examples;
- connectors, providers, OpenShell integration, process or network execution,
  credential injection, SideEffect execution, or writes;
- hosted administration, enterprise identity, reasoning lineage, or release
  posture changes.

## 3. Original Blocker Restatement

The original helper accepted only a contract and projection set. It proved that
all projections agreed with the first projection, but it had no independent
source for the execution identity being evaluated.

A coherent projection set from a different actor, workflow, run, or step could
therefore return `Satisfied` when used with the same harness contract. That
made the consumer boundary misleading even though the wrong projection
identity remained visible in the result.

## 4. Fix Approach Assessment

The fix adds a dedicated payload-free context rather than duplicating
projection internals or inventing runtime authority.

This is the right first boundary because:

- the consumer receives the expected execution identity independently from the
  projections it evaluates;
- every projection is checked against the same expected identity;
- the contract must match the expected harness;
- the result retains all sources needed to recompute equality; and
- no projection is relabeled or repaired.

The implementation is minimal and consistent with existing typed Core IDs,
timestamps, redaction-safe Debug, stable errors, and validated aggregate serde.

## 5. Validation Boundary Assessment

Construction and result validation now require exact equality for:

- actor;
- workflow ID;
- run ID;
- step ID;
- harness contract ID; and
- evaluation timestamp.

Harness mismatch between the context and contract fails with
`required_context.consumption.contract_context_mismatch`. Any projection
context mismatch fails with
`required_context.consumption.projection_context_mismatch`.

These errors disclose only the failure class. They do not include identities,
timestamps, targets, paths, payloads, credentials, or rejected secret-like
values.

## 6. Wire Integrity Assessment

`RequiredContextConsumptionResult` serializes the contract, independent
context, complete projections, satisfactions, gaps, and posture.
Deserialization:

1. validates the contract;
2. compares every projection with the retained independent context;
3. validates exact target/access equality;
4. recomputes satisfactions, gaps, and posture; and
5. rejects any inconsistent aggregate.

The focused wire test substitutes a secret-like run identity in the retained
context and proves deserialization fails without echoing it.

Changing both the independent context and all otherwise valid source
projections could construct a valid result for a different execution context.
That is not hidden aggregate tampering; it is the remaining trusted-input
boundary. Binding these sources to the immutable run bundle is correctly
deferred and must precede runtime consumption.

## 7. Authority And Privacy Assessment

The correction does not turn a declaration into a grant, availability into
authority, approval into context access, or satisfaction into a dereference
lease.

The new context stores only bounded typed identities and time. Debug output
redacts actor, workflow, run, step, and harness IDs. No raw provider payload,
source contents, command output, parser payload, environment value, credential,
authorization header, or private key can be stored by the model.

## 8. Test Quality Assessment

Focused coverage now proves:

- exact-context success remains valid;
- actor mismatch fails closed;
- workflow mismatch fails closed;
- run mismatch fails closed;
- step mismatch fails closed;
- harness mismatch fails closed;
- evaluation-time mismatch fails closed;
- serialized result context substitution fails closed; and
- mismatch errors remain non-leaking.

The focused required-context suite passes 12 tests. Adjacent capability,
context-projection, workspace runtime, report, adapter, approval, provider-write,
catalog, and validation tests also pass.

## 9. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-core --test required_context -- -D warnings`:
  passed.
- `cargo test -p workflow-core --test required_context`: passed, 12 tests.
- `cargo test -p workflow-core --test capability_authority --test governed_context_access --test required_context`:
  passed, 71 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed before this review file.
- `git diff --check`: passed before this review file.

## 10. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785136647630748000-2`
- approval ID:
  `approval/run-1785136647630748000-2/review-scope-approved`
- presentation ID: `presentation/06285c3786648f06`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- out-of-kernel work: source and test inspection, review writing, and validation
  commands were performed by the delegated maintainer; the kernel governed
  scope and approval but did not edit files, invoke tools, or mutate git

## 11. Blockers

None.

The original execution-context binding blocker is fixed.

## 12. Non-Blocking Follow-Ups

- Bind the contract, independent consumption context, and source projections to
  the immutable run-input bundle before runtime use.
- Define time-of-use freshness and authority re-resolution before dereference.
- Consider reducing standalone serde exposure of derived satisfaction and gap
  records before schema or SDK exposure.
- Preserve exact context and policy identity if a future optional sandbox
  execution provider consumes the result.

## 13. Recommended Next Phase

Proceed to **required-context immutable-run binding and time-of-use
re-resolution planning**.

Do not implement target dereference, ambient workspace access, executor
consumption, persistence, events, schemas, CLI behavior, providers, OpenShell
integration, SideEffect execution, writes, hosted administration, or release
changes in that planning phase.
