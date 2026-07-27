# Required Context Immutable Execution Binding Report

## 1. Executive Summary

Workflow OS now has a payload-free immutable commitment that binds one exact
required-context contract and future consumption scope to a validated stored
immutable run bundle.

The model detects substitution of bundle, workflow, run, step, actor, harness
contract, sensitivity, time, or contract content. It does not authorize
context access, resolve current capabilities, dereference targets, integrate
with the executor, invoke providers or sandboxes, or perform writes.

## 2. Scope Completed

- Added `RequiredContextExecutionBindingVersion`.
- Added `RequiredContextExecutionBindingInput`.
- Added `RequiredContextExecutionBinding`.
- Added a constructor rooted in `StoredImmutableRunBundle`.
- Derived workflow, run, and bundle identity from the validated stored
  manifest.
- Required one matching canonical frozen workflow record.
- Proved the requested step exists in that workflow.
- Bound exact actor, harness contract ID/version/content hash, sensitivity
  ceiling, and binding time.
- Added fixed-width framed, domain-separated SHA-256 commitment hashing.
- Added fail-closed validation and deserialization.
- Added redaction-safe Debug behavior and focused privacy tests.

## 3. Scope Explicitly Not Completed

This phase did not add:

- time-of-use capability or availability re-resolution;
- target, evidence, event, report, handoff, SideEffect, artifact, repository,
  or source payload dereference;
- runtime or executor integration;
- persistence, workflow events, audit projection, authority receipts, or
  report artifacts;
- workflow, harness, immutable-bundle, schema, SDK, CLI, UI, or example shape
  changes;
- providers, connectors, OpenShell, filesystem, network, process, inference,
  environment, credential, or sandbox execution;
- SideEffect execution or writes;
- enterprise administration, reasoning lineage, hosted behavior, or release
  changes.

## 4. Model And Constructor Summary

`RequiredContextExecutionBindingInput` accepts:

- a validated `StoredImmutableRunBundle`;
- an exact `RequiredContextContractBinding`;
- actor;
- step;
- maximum sensitivity; and
- binding timestamp.

The constructor derives immutable bundle, workflow, and run identity from the
stored manifest. It finds the matching canonical workflow record, verifies
record identity and source hash against the manifest, and proves the step is
part of the frozen workflow.

The resulting binding retains no context target or source payload.

## 5. Validation Boundary

Validation fails closed when:

- the canonical workflow record is missing or duplicated;
- workflow identity, version, schema, or source hash differs from the manifest;
- the step is absent from the frozen workflow;
- sensitivity is unknown;
- binding time predates bundle creation;
- the binding version is unknown; or
- any serialized field no longer matches the deterministic binding hash.

Errors use stable `required_context.execution_binding.*` codes and do not
include caller-supplied values.

## 6. Authority Boundary

The binding proves only the exact pre-consumption commitment. It is not:

- a capability grant;
- proof that a grant remains active;
- proof that a target remains available;
- proof that policy, approval, evidence, or checks are satisfied;
- a lease;
- a dereference permission; or
- proof that any payload was read.

Fresh time-of-use authority remains a separately planned phase.

## 7. Privacy And Redaction

The binding stores identities, hashes, an enumerated sensitivity ceiling, and a
timestamp only. It stores no raw source, provider, parser, command,
environment, credential, log, artifact, or context-target payload.

Debug output redacts workflow, run, step, actor, harness, timestamp, and hash
values. Deserialization errors use fixed text and do not echo rejected values.

## 8. Test Coverage

Focused tests cover:

- deterministic valid binding;
- stored bundle, workflow, run, and root derivation;
- exact contract identity/version/content hash;
- absent immutable step;
- unknown sensitivity;
- binding timestamp before bundle creation;
- bundle, contract, actor, and sensitivity substitution;
- valid serde round trip;
- serialized field tampering;
- secret-like rejected values without error leakage;
- redaction-safe Debug; and
- payload-free serialized shape.

Adjacent required-context and immutable-bundle builder/store tests also pass.

## 9. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test required_context_execution_binding --test required_context --test immutable_run_bundle_store --test immutable_run_bundle_builder`:
  passed, 44 tests.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1785137962988429000-2`
- approval ID:
  `approval/run-1785137962988429000-2/implementation-approved`
- presentation ID: `presentation/1742a5a7ca996493`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof was persisted and enforced
- out-of-kernel work: source, tests, docs, and validation commands were
  performed by the delegated maintainer; the kernel governed scope and
  approval but did not edit files, invoke tools, or mutate git

## 11. Product And Feedback Reconciliation

Fresh-pull evaluation describes current Workflow OS accurately as a coherent
local governance kernel whose next product challenge is reducing ceremony
without weakening evidence or integrity. This binding strengthens the
integrity prerequisite beneath quiet success: low-risk presentation can become
less interruptive later, but accepted run and context identity cannot be
inferred or substituted.

The evaluator's Node 24 integration-check and duplicate missing-manifest
papercuts are already fixed and reviewed on current main. This phase does not
duplicate those lanes.

An optional OpenShell execution provider remains complementary future work.
Workflow OS should own governed intent, authority, evidence, and reports while
a sandbox provider owns containment. No OpenShell integration is implemented
or authorized here.

## 12. Remaining Limitations

- The binding is not used by a runtime consumer.
- Harness contracts are not canonical immutable-bundle definition records.
- No complete current grant/availability fact-set model exists.
- No same-call time-of-use re-resolution exists.
- No audited read-only dereference exists.
- No authority receipt or one-time-use semantics exist.

## 13. Recommended Next Phase

Perform a focused maintainer review of the required-context immutable execution
binding.

After acceptance, define the complete current authority-fact-set boundary
before implementing any authoritative time-of-use `Ready` result. Continue to
defer dereference, executor integration, providers, OpenShell, SideEffects,
writes, hosted behavior, and release changes.
