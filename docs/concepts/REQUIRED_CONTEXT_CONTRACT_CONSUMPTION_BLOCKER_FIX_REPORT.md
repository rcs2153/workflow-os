# Required Context Contract Consumption Blocker Fix Report

## 1. Executive Summary

This phase fixes the independent execution-context binding blocker found in
[Required Context Contract Consumption Review](REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_REVIEW.md).

The original helper proved only that supplied projections agreed with each
other. It did not prove that they belonged to the actor, workflow, run, step,
harness, and evaluation time for which consumption was requested. The corrected
boundary accepts one explicit payload-free consumption context, retains it in
the result, and requires every projection to match it exactly.

## 2. Blocker Fixed

The helper no longer treats the first projection as the source of truth for
execution identity.

The fix prevents a coherent capability-backed projection from a different
execution context from satisfying the required-context contract.

## 3. Implementation Approach

The fix adds `RequiredContextConsumptionContext` with:

- actor ID;
- workflow ID;
- run ID;
- step ID;
- harness contract ID; and
- evaluation timestamp.

`RequiredContextConsumptionInput` now requires this context in addition to the
contract and projections. `RequiredContextConsumptionResult` retains it.

Construction, `validate`, and deserialization require:

- the context harness contract to equal the required-context contract ID; and
- every projection actor, workflow, run, step, harness, and projection time to
  equal the independent context.

The helper continues to sort projections only by access level. It does not
infer, repair, or relabel execution identity.

## 4. Scope Completed

- Added one explicit payload-free consumption context.
- Bound every projection to that context.
- Retained the context in aggregate results.
- Recomputed context equality during result validation and deserialization.
- Added stable non-leaking context mismatch errors.
- Added mismatch regression tests for actor, workflow, run, step, harness, and
  evaluation time.
- Added serialized result context-substitution regression coverage.

## 5. Scope Explicitly Not Completed

- No context or target dereference.
- No repository or source inspection.
- No executor or runtime integration.
- No immutable-run-bundle binding.
- No persistence, workflow events, audit records, or authority receipts.
- No schemas, SDKs, CLI behavior, UI, or examples.
- No connectors, providers, OpenShell integration, process execution, network
  access, credential injection, SideEffect execution, or writes.
- No hosted administration, enterprise identity, reasoning lineage, or release
  posture changes.

## 6. Validation Boundary

The correction uses two stable error codes:

- `required_context.consumption.contract_context_mismatch` when the independent
  harness context does not match the contract; and
- `required_context.consumption.projection_context_mismatch` when any
  projection differs from the declared actor, workflow, run, step, harness, or
  evaluation time.

Errors identify only the failure class and do not echo IDs, timestamps,
references, paths, payloads, credentials, or secret-like values.

## 7. Privacy And Redaction

The new context stores bounded typed identity and time only. It contains no raw
context, provider data, command output, source contents, parser payloads,
credentials, environment values, authorization headers, or private keys.

Its Debug implementation redacts actor, workflow, run, step, and harness
identities. The timestamp remains visible because it is bounded execution
metadata rather than caller-supplied prose or payload.

## 8. Test Coverage

Focused tests prove:

- valid exact-context consumption remains unchanged;
- actor mismatch fails closed;
- workflow mismatch fails closed;
- run mismatch fails closed;
- step mismatch fails closed;
- harness mismatch fails closed;
- evaluation-time mismatch fails closed;
- a serialized result cannot substitute a different run context; and
- failure messages do not disclose substituted secret-like values.

The required-context focused suite passes 12 tests.

## 9. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-core --test required_context -- -D warnings`:
  passed.
- `cargo test -p workflow-core --test required_context`: passed, 12 tests.
- `cargo test -p workflow-core --test capability_authority --test governed_context_access --test required_context`:
  passed, 71 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Dogfood Governance

- workflow: `dg/blocker`
- run ID: `run-1785133651162159000-2`
- approval ID: `approval/run-1785133651162159000-2/fix-approved`
- presentation ID: `presentation/23e7175b43ef6952`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted blocker-fix handoff was presented
- out-of-kernel work: source, tests, documentation, and validation commands
  were performed by the delegated maintainer; the kernel governed scope and
  approval but did not edit files, invoke tools, or mutate git

## 11. Remaining Limitations

- The consumption context is not yet bound to the immutable run-input bundle.
- Evaluation-time equality is batch identity, not freshness or a lease.
- Time-of-use authority re-resolution is not implemented.
- No target dereference or ambient workspace-access prevention exists.
- Candidate completeness remains relative to caller-supplied projections.

## 12. Recommended Next Phase

Perform a focused **required-context execution-binding blocker-fix review**.

After acceptance, plan immutable-run-bundle binding and time-of-use
re-resolution before any context dereference, executor integration, sandbox
provider, or runtime consumer.
