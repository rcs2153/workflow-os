# Proportional-Governance Runtime-Fact Source Executor Consumer Plan

## 1. Executive Summary

The registered runtime-fact source and freshness boundary is implemented and
reviewed. This phase adds one explicit opt-in local executor consumer so a run
can derive its proportional-governance assessment from current source-bound
facts rather than a caller-assembled fact vector.

The integration remains local and additive. It does not activate proportional
governance by default, enforce the selected execution disposition, invoke a
provider, integrate OpenShell, or add schemas, CLI behavior, SideEffects, or
writes.

## 2. Goals

- Resolve current facts for the exact stored immutable run bundle.
- Validate source identity, contract version, freshness, and exact step
  coverage in the same call as assessment.
- Persist the existing governance assessment binding before run events and
  step execution.
- Return the accepted payload-free source snapshot for later evidence/report
  composition.
- Re-resolve current facts on exact retry and require equality with the durable
  assessment binding before rehydration.
- Preserve existing workflow, executor, approval, and retry semantics.

## 3. Non-Goals

- Default or automatic activation.
- Enforcement of quiet, visible, approval-required, or denied disposition.
- Source snapshot persistence or deserialization as authority.
- Approval-resume source resolution.
- Automatic local checks or provider calls.
- OpenShell or another execution-provider integration.
- SideEffect execution, external writes, schemas, CLI behavior, hosted runtime,
  workflow configuration, or additional mutation families.

## 4. API Boundary

Add an explicit request containing the existing immutable-bundle execution
request, governance profile, validated source registration, Core-selected
evaluation time, and optional expected aggregate fingerprint. The injected
source remains a function dependency rather than hidden configuration.

Return the run, immutable bundle binding, durable assessment binding, and
accepted payload-free source snapshot. Debug output must redact paths,
identifiers, bundle bindings, hashes, and source metadata.

## 5. Fresh Execution Semantics

For a new run, Core must:

1. prepare the existing local execution plan and evaluate existing pre-run
   policy;
2. build, validate, and persist the immutable run bundle;
3. read the exact stored bundle;
4. invoke the registered source exactly once;
5. validate and assess current facts in the same call;
6. validate the optional expected aggregate fingerprint;
7. construct and persist the existing assessment binding;
8. attach immutable-bundle and assessment bindings to the plan;
9. append run-start events; and
10. execute through the existing local executor.

Source or assessment failure must occur before run events or skill execution.

## 6. Retry Semantics

For an existing run, Core must validate the supplied immutable-bundle request
against the durable run and stored bundle before invoking the source. It must
then resolve a new current snapshot exactly once, derive a new binding, and
require exact equality with the durable binding before returning the
rehydrated run. Changed posture fails closed without appending events or
re-executing work.

Different valid snapshot identities may produce the same durable assessment.
The snapshot is call-local evidence metadata; it is not persisted or reused as
authority in this phase.

## 7. Error And Privacy Posture

- Preserve stable Core-owned source and executor error codes.
- Replace source-local failures before they cross the public boundary.
- Do not include paths, source IDs, snapshot IDs, fact values, tokens, provider
  output, or command output in errors.
- Keep request and result Debug implementations redaction-safe.
- Do not copy source payloads into run events or the assessment binding.

## 8. Test Plan

- Fresh source-backed execution completes and persists its assessment binding
  before execution events.
- The source is called exactly once per invocation.
- Exact retry re-resolves facts without duplicate skill execution.
- Changed retry facts fail closed without new events.
- Source failure is stable and non-leaking and starts no run.
- Request and result Debug output do not expose source, bundle, or path values.
- Existing executor, proportional-governance, approval, report, SideEffect,
  adapter, and runtime tests continue to pass.

## 9. Validation

- Focused local executor tests.
- Focused clippy.
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 10. Recommended Follow-Up

Define and review the durable source-snapshot commitment contract before
approval-resume consumption. Do not broaden default proportional-governance
enforcement or provider mutation families first.
