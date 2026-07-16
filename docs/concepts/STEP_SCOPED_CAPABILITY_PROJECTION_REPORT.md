# Step-Scoped Capability Projection Report

## 1. Executive Summary

Workflow OS now has a pure helper that projects validated capability
resolutions into the bounded set of capability references visible to one exact
actor, workflow, run, step, and optional harness context. Only resolutions with
`authorized` posture are included. Denied or independently gated capabilities
remain absent.

This is a model/helper phase only. The projection does not load tools, expose
context payloads, execute commands, invoke providers, persist authority, append
events, or authorize time-of-use invocation.

## 2. Scope Completed

- Added explicit `StepScopedCapabilityProjectionInput`.
- Added validated projection and entry models with read-only accessors.
- Added pure `project_step_scoped_capabilities` construction.
- Required exact actor, workflow, run, step, harness, and evaluation-time
  equality across every supplied resolution.
- Included only authorized capability/resource/grant references.
- Kept denied and independently gated capabilities absent.
- Rejected duplicate capability/resource resolutions.
- Sorted visible entries deterministically.
- Retained each entry's validated authorized source resolution so serialized
  grant or context substitution fails closed.
- Added redaction-safe Debug and validated serde behavior.

## 3. Scope Explicitly Not Completed

- No tool registry, loading, invocation, or command execution.
- No connector activation or live adapter calls.
- No provider reads or writes.
- No context dereference or payload access.
- No runtime executor or harness integration.
- No persistence, events, audit projection, schemas, SDKs, or CLI behavior.
- No authority receipts, cryptographic proof, hosted administration, or
  enterprise identity integration.
- No release posture changes.

## 4. Helper And Model Summary

`project_step_scoped_capabilities` accepts explicit scope identities, an exact
projection timestamp, validated capability resolutions, and redaction metadata.
It rejects a resolution from another actor, workflow, run, step, harness, or
evaluation batch. It rejects duplicate capability/resource decisions rather
than choosing one silently.

An output entry carries the full payload-free authorized source resolution plus
the selected grant ID. Validation proves that the entry posture is authorized
and that its grant ID equals the resolution's selected grant. Projection
validation then proves exact context and timestamp equality.

## 5. Authority Boundary

The projection is a deterministic view of one explicit resolution batch. Exact
timestamp equality means batch consistency; it is not a TTL, lease, or proof
that authority remains current later. A future invocation boundary must resolve
or revalidate availability, grant lifecycle, actor, resource, run, step,
harness, policy, approval, evidence, checks, and time-of-use posture.

Capability visibility does not imply invocation success. A projection cannot
create a grant, satisfy an independent prerequisite, resume a workflow, or
authorize provider mutation.

## 6. Validation And Privacy

- Stable validation codes identify context mismatch, duplicate resolution,
  missing authorized grant, inconsistent entry, and invalid ordering without
  echoing caller-supplied identifiers.
- IDs, resources, sensitivity, resolutions, and redaction metadata reuse the
  existing bounded validated models.
- Debug output redacts actor, workflow, run, step, harness, capability,
  resource, and grant identifiers.
- Custom deserialization validates source resolution authority and complete
  projection context.
- The model stores no credentials, provider payloads, command output, source
  contents, parser payloads, environment values, or unrestricted metadata.

## 7. Test Coverage

Focused tests cover:

- authorized-only projection;
- denied capability absence and explicit empty projection;
- stale evaluation-batch rejection;
- cross-step rejection;
- duplicate capability/resource rejection;
- deterministic validated serde round trip;
- forged grant and forged source-context rejection;
- redaction-safe Debug and forbidden-payload-field absence.

## 8. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  48 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1784178975897172000-2`
- approval ID:
  `approval/run-1784178975897172000-2/implementation-approved`
- presentation ID: `presentation/b750140bd6a95a06`
- approval outcome: granted under delegated-maintainer authority after the full
  approval handoff was presented.

## 10. Remaining Limitations

- Evaluation-batch equality is not runtime freshness enforcement.
- The projection is not committed into an immutable run bundle.
- No durable capability projection store exists.
- No executor, harness, adapter, or tool consumer exists.
- No context-access projection or authority receipt exists.
- No workflow-level contract declares required projected capabilities.

## 11. Recommended Next Phase

Perform a focused step-scoped capability projection maintainer review. Review
authorized-only filtering, exact context binding, source-resolution wire
integrity, deterministic ordering, privacy, and the non-executing authority
boundary before beginning governed context-access projection.
