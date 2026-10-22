# Scoped Capability GitHub Read-Only Integration Report

## 1. Executive Summary

Workflow OS now has one complete, internal, opt-in runtime composition of
current capability authority with an existing provider adapter. The new path
authorizes and performs exactly one GitHub repository-metadata read through an
injected `GitHubReadOnlyAdapter` and emits a separate payload-free
`CapabilityUseAuthorityReceipt` only after the bounded read succeeds.

This is a no-write proof, not automatic adapter governance. Existing direct
GitHub adapter APIs are unchanged, and no CLI, schema, SDK, persistence,
runtime configuration, live-network requirement, or provider mutation was
added.

## 2. Scope Completed

- Added a private single-query capability-use authority fact path for
  `github.read` and one canonical repository resource.
- Required fresh source-backed resolution for the exact actor, workflow, run,
  step, harness, resource, and sensitivity.
- Required the selected grant itself to contain exact run, step, and harness
  scope.
- Preserved policy as an independent gate with operation-mode-specific
  provenance.
- Derived a step-scoped capability projection only from the fresh authorized
  resolution.
- Called the existing repository-metadata adapter path exactly once after all
  pre-call gates passed.
- Added explicit success, pre-read block, read failure, and post-read receipt
  failure outcomes.
- Added a separate versioned capability-use receipt and an explicitly
  unverified deserialization target.

## 3. Scope Explicitly Not Completed

The phase did not add GitHub writes, another read operation, automatic
capability gating, public integration APIs, workflow configuration, schemas,
SDKs, CLI behavior, live GitHub validation, token handling, receipt
persistence, report-artifact integration, automatic grants, prerequisite
evaluation, hosted identity, nested harness execution, sandbox integration,
enterprise administration, cryptographic signing, or release changes.

## 4. Runtime Integration Summary

The crate-private helper validates policy provenance and canonical repository
identity before source use. It then builds a dedicated capability-use query,
selects matching source facts, verifies freshness, resolves authority,
requires exact selected-grant scope, projects only the authorized capability,
constructs the existing adapter request, and invokes
`read_repository_metadata`.

Fixture mode accepts only fixture-test allowed policy provenance. Live
read-only mode accepts only runtime-policy allowed provenance. Capability
authority never synthesizes policy success.

## 5. Canonical Identity And Source Boundary

GitHub owner and repository segments are independently bounded and validated,
then normalized to ASCII lowercase. The same canonical values define the
source query, capability resource, adapter request metadata, and adapter call.

The authority input comes from the registered Core-owned source. Callers
cannot supply grants, availability records, fact sets, projections, receipts,
or a serialized snapshot as authority.

## 6. Receipt And Failure Semantics

`CapabilityUseAuthorityReceipt` is separate from the existing
required-context `AuthorityReceipt`. It binds commitments for exact execution
identity, capability resource, selected grant, source snapshot, fact set,
assessment, operation, and bounded result without storing repository names,
raw responses, credentials, or headers.

Trusted construction is crate-private. Serialized receipts deserialize only
as `UnverifiedCapabilityUseAuthorityReceipt`; they cannot recover trusted
authority. A successful provider read followed by receipt failure is preserved
as `ReadSucceededReceiptFailed`, preventing callers from treating the call as
unattempted and retrying blindly.

## 7. Privacy And Redaction Summary

New runtime input and proof types are private and do not derive serialization.
Debug output redacts execution, resource, grant, source, and receipt identity.
Errors use stable codes and bounded messages without owner, repository,
payload, path, token, credential, or source-fact values. The receipt stores a
resource commitment rather than repository plaintext.

## 8. Test Coverage Summary

Focused tests prove:

- exact authority produces one canonical adapter call, one projection entry,
  and a valid payload-free receipt;
- fixture/runtime policy-provenance mismatch blocks before the adapter;
- workflow-wide authority is insufficient for this exact-scope slice;
- stale source data, unavailable capability, unmet independent policy
  prerequisites, and excess sensitivity block before the adapter;
- adapter failure produces no receipt;
- receipt failure after a successful read has its own non-retry posture;
- secret-like repository input is rejected without leakage; and
- serialized receipt claims remain explicitly unverified.

Existing workspace tests cover the unchanged capability, projection,
registered-source, required-context receipt, GitHub adapter, telemetry, and
runtime behavior.

## 9. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live-network tests remained skipped
  by design.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Workflow Semantics And Compatibility

The helper has no state backend, filesystem, event, CLI, or mutation behavior.
It does not alter workflow status or existing executor behavior. The existing
`AuthorityReceipt` V1 implementation and direct GitHub adapter APIs were not
modified.

## 11. Remaining Known Limitations

- The integration is crate-private and has no executor or workflow-schema
  wiring.
- Required validation is fixture-backed; live network behavior is not an
  acceptance dependency.
- Independent grant prerequisites are rejected rather than evaluated.
- The capability-use receipt is local, unsigned, evidence-only, and not
  persisted.
- Only GitHub repository metadata is supported.
- This phase does not prove generalized adapter capability governance.

## 12. Governed Phase Record

- workflow ID: `dg/implement`
- run ID: `run-1792692834172091000-2`
- approval ID:
  `approval/run-1792692834172091000-2/implementation-approved`
- presentation ID: `presentation/d53980702c92d605`
- approval outcome: granted by delegated maintainer with persisted
  presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one persisted
  presentation record and event marker
- validation summary: formatting, workspace Clippy, workspace tests,
  documentation checks, and diff integrity passed; no live-network check was
  required
- kernel boundary: governance coordination only; implementation, tests,
  documentation, Git, pull-request, CI, and merge work remain out of kernel
- out-of-kernel work: source inspection, Rust and documentation edits, local
  command execution, validation, Git operations, pull-request publication, CI
  observation, and merge handling

## 13. Recommended Next Phase

Perform a focused maintainer and security review of this implementation. The
review should verify exact source-backed authorization, zero provider calls on
all pre-read denial paths, receipt truthfulness, privacy, compatibility, and
the continued absence of writes or automatic adapter gating before any public
API or operation expansion is considered.
