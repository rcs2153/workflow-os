# OpenShell Optional No-Write Execution Provider Report

## 1. Executive Summary

The first implementation slice of the optional OpenShell provider milestone is
complete in code. Workflow OS now has provider-neutral effective-runtime
attestation, a provider-agnostic hosted worker boundary, and an optional
OpenShell no-write lifecycle provider driven by an injected client.

This is not yet a complete OpenShell integration. A pinned v0.0.101 CLI
compatibility transport is now implemented, but it is not selected as the
provider client because the reviewed CLI does not expose every attestation
surface required by the provider contract. No sandbox smoke proof has been
run. The milestone remains incomplete until those surfaces are available,
integrated, and reviewed.

## 2. Scope Completed

- Added effective-policy, runtime-image, enforcement, control, observation, and
  cleanup attestation to hosted receipts.
- Preserved the legacy unattested receipt constructor for existing providers.
- Allowed providers to require attestation.
- Added provider-specific request validation and deterministic execution ID
  hooks to the provider trait.
- Made `HostedWorker` consume an explicitly injected provider trait object.
- Kept the existing inert no-write provider as the default binary provider.
- Added `OpenShellNoWriteExecutionProvider` and an injected client lifecycle.
- Added fixed-operation, policy-reinspection, denied-egress, and cleanup gates.
- Added focused Core and scripted-provider tests.

## 3. Scope Explicitly Not Completed

- No OpenShell fork.
- No OpenShell install, gateway, compute-driver, CLI, or Python SDK client.
- No complete `OpenShellNoWriteClient` transport or live image pin.
- No live sandbox smoke test.
- No automatic/default provider selection.
- No arbitrary command, credential, inference, provider mutation, or write.
- No schema, SDK, broad CLI, multi-tenant, or production-readiness change.

## 4. API Summary

Core adds `HostedExecutionAttestation`, control/enforcement/cleanup posture,
effective policy revision, and bounded observation summary. Providers may
require attestation and validate requests through the existing trait.

The hosted crate adds `OpenShellNoWriteClient`, sandbox snapshot and fixed
operation outcome types, and `OpenShellNoWriteExecutionProvider`. The client
does transport; the provider owns sequencing, validation, cleanup, and receipt
construction.

## 5. Security Boundary

The provider rejects SideEffects, access material, and non-read capabilities
before sandbox creation. Completion requires exact image and effective-policy
digest, stable policy revision, hard enforcement across filesystem/process/
network controls, bounded process observations, denied-egress proof, zero
unexpected network/policy/security observations, and confirmed cleanup.

## 6. Workflow Semantics

The provider never mutates a workflow run, event history, approvals, or state
store. The existing worker persists the attempt before invocation and Core owns
terminal projection. Provider uncertainty continues to map to reconciliation,
not retry or fabricated success.

## 7. Test Coverage

Focused tests cover attestation serde and redaction, successful attested
execution, audit-mode rejection, policy revision drift, cleanup ambiguity,
pre-invocation SideEffect rejection, denied-egress proof, safe Debug output,
serialized policy-binding tampering, and provider-specific runtime-image
substitution.

## 8. Validation

The following validation passed from the final source state:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --offline -- -D warnings`
- `cargo test -p workflow-core --test hosted_execution_attestation --offline`
  (`4 passed`)
- `cargo test -p workflow-hosted openshell --offline` (`9 passed`)
- `cargo test --workspace --offline` (all default workspace tests passed; only
  explicitly opt-in live-provider and live-local-check tests were ignored)
- `npm run check:docs`
- `git diff --check`

The local Rust toolchain showed unusually long test-binary startup pauses. The
checks were allowed to complete and their successful exit status, rather than
the observed process posture, is the recorded validation evidence.

## 9. Known Limitations

The injected client is still a contract, not a live OpenShell transport.
Scripted provider tests prove Workflow OS lifecycle semantics but provide no
sandbox-containment evidence. The separate pinned CLI compatibility transport
strictly parses reviewed machine output but cannot safely satisfy the complete
client contract. No OpenShell installation or machine configuration was
changed.

## 10. Recommended Next Phase

Resolve the missing runtime-image, complete-observation, and cleanup
attestation surfaces through an upstream/API-compatible boundary, then run one
explicit local no-write sandbox smoke proof. Do not weaken attestation, parse
human output, add credentials, accept arbitrary commands, add provider
mutations, or enable automatic defaults.

## 11. Governed Phase Evidence

- Workflow: `dg/runtime-composition`.
- Run ID: `run-1786227846893152000-2`.
- Approval ID:
  `approval/run-1786227846893152000-2/composition-approved`.
- Approval presentation ID: `presentation/1a657509318f00a0`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: provider attestation, injected OpenShell client boundary,
  fixed no-write operation, bounded observations, receipt/event/report
  composition, tests, and documentation.
- Out-of-kernel work: Codex inspected upstream documentation, edited code and
  docs, and ran validation. The kernel governed scope and approval but did not
  install OpenShell, edit files, execute tests, or perform git actions.
