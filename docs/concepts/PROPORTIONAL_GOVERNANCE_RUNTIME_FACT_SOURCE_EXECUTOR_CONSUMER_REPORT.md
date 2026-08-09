# Proportional-Governance Runtime-Fact Source Executor Consumer Report

## 1. Executive Summary

Workflow OS now has one explicit opt-in local executor path that obtains
current proportional-governance facts from a registered source for the exact
immutable run bundle. Core validates and assesses those facts in the same call,
durably records the existing governance assessment binding before run events,
and returns the accepted payload-free source snapshot.

## 2. Scope Completed

- Added explicit source-backed executor request and result types.
- Added one injected-source local execution helper.
- Reused immutable-bundle construction, validation, and persistence.
- Reused the reviewed current-fact source freshness boundary.
- Reused the durable governance assessment binding and event projection.
- Re-resolved current facts on exact retry.
- Failed closed when current facts no longer reproduce the durable binding.
- Added focused executor and non-leakage tests.

## 3. Scope Explicitly Not Completed

The phase did not add default activation, execution-disposition enforcement,
automatic checks, approval-resume source consumption, source snapshot
persistence, schemas, CLI behavior, OpenShell, provider calls, SideEffects,
writes, hosted behavior, or new provider mutation families.

## 4. API Summary

`LocalExecutionWithCurrentRuntimeFactsGovernanceRequest` carries the existing
immutable-bundle execution request, profile, source registration, evaluation
time, and optional expected fingerprint.

`execute_with_current_runtime_facts_governance_assessment_binding` accepts the
executor, immutable-bundle store, injected source, and explicit request. It
returns `LocalExecutionWithCurrentRuntimeFactsGovernanceResult`, containing the
run, immutable bundle binding, durable assessment binding, and accepted
payload-free source snapshot.

## 5. Runtime Semantics

Fresh execution persists the immutable bundle, resolves and validates current
facts once, persists the assessment binding, and only then emits run events and
executes steps. Existing runs validate exact request/bundle identity, resolve a
new snapshot once, and require the derived binding to equal durable state before
rehydration succeeds.

Report or governance metadata does not rewrite workflow pass/fail semantics.
Existing executor entry points remain unchanged.

## 6. Durability Boundary

The governance assessment binding is durable and projected into the run event
history. The accepted source snapshot is returned to the caller but is not
persisted. It remains payload-free, serialize-only evidence metadata and is not
reusable authority.

## 7. Privacy And Error Posture

Source-local errors are replaced by the stable Core source-failure error.
Request and result Debug output redact execution inputs, paths, source identity,
bundle identity, hashes, and snapshot metadata. No raw fact payload, provider
output, command output, token, or credential is copied into events or errors.

## 8. Test Coverage

Focused tests cover fresh execution, one source call per invocation, durable
binding order, exact retry without duplicate execution, changed retry facts,
source-failure non-leakage, no run events on source failure, and redacted Debug
output. Existing workspace coverage remains the regression boundary.

## 9. Validation Commands And Results

- Focused source-backed local executor tests: passed.
- Focused clippy: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Known Limitations

- Source snapshot commitments are not durable records yet.
- Approval-resume paths do not resolve source-bound facts yet.
- The selected proportional-governance disposition is recorded, not enforced,
  by this generic integration.
- Source registration is an embedding trust decision, not authenticated remote
  attestation.
- No workflow or project configuration selects a source.

## 11. Recommended Next Phase

Implement and review a durable source-snapshot commitment binding before
approval-resume source consumption. Keep the next slice local, additive,
payload-free, and independent of provider execution.

## 12. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786279690870244000-2`
- Approval ID: `approval/run-1786279690870244000-2/composition-approved`
- Presentation ID: `presentation/cda0ec008370e532`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: source-backed executor implementation, tests,
  documentation, validation, and git/PR work
- Missing coverage: source snapshot persistence, approval-resume source
  consumption, and default disposition enforcement were disclosed and not
  simulated
