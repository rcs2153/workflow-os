# Immutable Run Bundle Executor Binding Report

## 1. Executive Summary

Workflow OS now has one explicit opt-in local executor path that creates or
verifies a complete immutable run bundle before durable run creation and binds
that bundle's ID, format version, and integrity root into the workflow run.

The existing executor remains unchanged. This phase does not make bundles a
default, execute from stored definitions, attest handlers or checks, expose
bundles through the CLI or schemas, authorize capabilities, or broaden provider
mutations.

## 2. Scope Completed

- Added a bounded `ImmutableRunBundleBinding` to optional run identity.
- Added explicit bundle inputs and a bundle-backed local execution request.
- Added `execute_with_immutable_run_bundle` as an additive local helper.
- Published or verified the complete bundle before appending `RunCreated`.
- Bound bundle identity and root hash into `RunCreated` and the rehydrated run.
- Added exact retry rehydration without duplicate skill invocation.
- Added exact explicit retry-posture comparison against the stored manifest.
- Added fail-closed legacy, rebinding, mismatch, and persistence boundaries.
- Preserved backward readability for events and runs without bundle identity.

## 3. Scope Explicitly Not Completed

- No automatic or default bundle generation.
- No executable replay or execution from stored definitions.
- No handler, check, command, binary, model, or tool attestation.
- No approval, policy, hook, SideEffect, report, or provider-write semantic
  changes.
- No CLI, workflow schema, SDK, example, UI, hosted, or distributed exposure.
- No scoped authority or capability enforcement.
- No provider mutation expansion or release-posture change.

## 4. API And Ordering Boundary

`LocalExecutionWithImmutableRunBundleRequest` combines an existing explicit
`LocalExecutionRequest` with caller-selected bundle identity, format version,
stable creation timestamp, sensitivity, and redaction posture.

`execute_with_immutable_run_bundle` follows this order for a new run:

1. prepare the existing execution plan;
2. evaluate pre-run policy;
3. load the validated project and build the immutable bundle from the same
   resolved workflow identity and execution-context commitment;
4. reject any plan/manifest mismatch;
5. persist or exactly verify the complete bundle;
6. bind the manifest identity into the execution plan;
7. append `RunCreated`, `RunValidated`, and `RunStarted`; and
8. execute the existing step path.

A bundle failure therefore cannot produce `RunCreated`. Bundle records may
exist as immutable orphans after a later event-store failure, but retry is
accepted only when the caller supplies the same deterministic bundle inputs and
the stored complete manifest matches exactly.

## 5. Durable Identity And Compatibility

`WorkflowRunIdentity` and `RunCreated` carry an optional
`ImmutableRunBundleBinding`. The binding contains only bundle ID, bundle model
version, and root hash. Later events continue to carry the existing event
identity fields; event rehydration compares those core fields while retaining
the bundle binding established by `RunCreated`.

The optional field uses serde defaults and omission when absent. Historical
events remain readable and existing executor APIs continue to create unbundled
runs. The explicit bundle-required helper rejects a pre-existing legacy
unbundled run rather than silently upgrading or rebinding it.

## 6. Execution And Handler Posture

The helper records only bounded posture available at preparation time:

- required checkpoint step IDs;
- whether hook or SideEffect inputs were supplied but not preserved;
- report-artifact posture as present but not preserved; and
- each resolved skill handler as registered but unattested or unavailable.

It does not claim execution attestation. The bundle remains an inspection and
integrity substrate, not proof that a particular handler binary, command,
model, or external system produced an outcome.

## 7. Failure And Privacy Boundary

Stable executor errors cover legacy unbundled use and binding mismatch without
echoing IDs, paths, definition content, or caller values. Existing bundle store
errors remain stable and non-leaking.

Debug implementations redact execution requests, bundle identities,
timestamps, root hashes, and run content. The bundle stores canonical validated
models, not raw YAML, parser payloads, command output, provider bodies,
environment values, credentials, or tokens.

## 8. Test Coverage

Focused tests verify:

- bundle publication precedes durable run creation;
- the rehydrated run carries the exact stored bundle binding;
- exact retry rehydrates without duplicate invocation;
- changed explicit retry posture fails closed without reinvocation;
- a persistence failure leaves the event backend empty;
- legacy unbundled runs fail closed only on the explicit bundle-required path;
- a bound run cannot be rebound to another bundle;
- legacy `RunCreated` JSON remains readable;
- bound `RunCreated` JSON round-trips into run identity; and
- default execution remains unbundled.

Existing builder, store, runtime, approval, policy, SideEffect, report,
provider, and workspace behavior remains covered by the repository suite.

## 9. Governed Phase Evidence

- Workflow: `dg/runtime-composition`.
- Run: `run-1784153600919627000-2`.
- Approval: `approval/run-1784153600919627000-2/composition-approved`.
- Presentation: `presentation/092b779f4b89b382`.
- Approval outcome: granted under delegated-maintainer authority after the
  complete proof-enforced handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof record with event marker present.
- Out-of-kernel work: Codex inspected the accepted bundle and executor
  boundaries, authored code and tests, and ran validation. The kernel governed
  scope and approval; it did not edit files or execute repository checks.

## 10. Validation Commands And Results

All required validation passes:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Focused bundle builder, bundle store, runtime event, and executor-binding tests
pass. The full suite preserves existing behavior; only explicit opt-in live
integration tests remain ignored by default.

## 11. Remaining Known Limitations

- Bundle creation is explicit and caller-configured rather than derived by a
  default runtime policy.
- Existing external request inputs are represented by bounded posture, not
  replayable references.
- The bundle cannot independently prove handler or check implementation
  identity.
- There is no read-only historical comparison helper yet.
- Immutable orphan cleanup and executor recovery after partial cross-store
  publication remain future operational work.

## 12. Recommended Next Phase

Perform a focused maintainer review of this executor binding. If accepted,
proceed to the scoped runtime authority and capability core model already
planned on the roadmap, while keeping execution, connectors, provider writes,
hosted administration, enterprise identity, schemas, and cryptographic
receipts out of scope.
