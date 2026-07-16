# Capability Grant And Availability Core Model Report

## 1. Executive Summary

Workflow OS now has a domain-neutral, model-only foundation for scoped runtime
authority. The implementation represents validated capability definitions,
resource and workflow/run/step scope, actor-bound grants, grant lifecycle and
delegation posture, prerequisite references, sensitivity ceilings, redaction
metadata, and explicit capability availability states.

This phase does not resolve grants at runtime, project tools or context, request
capabilities, issue authority receipts, invoke tools, connect providers, or
authorize writes.

## 2. Scope Completed

- Added validated capability, resource-scope, grant-scope, requirements,
  definition, grant, lifecycle, delegation, availability, and availability
  record types.
- Added bounded identifiers and stable validation error codes.
- Added private fields with read-only accessors.
- Added validated serialization/deserialization boundaries where serialized
  model surfaces are supported.
- Added redaction-safe `Debug` behavior.
- Exported the model from `workflow-core`.
- Added focused model, validation, serde, and non-leakage tests.

## 3. Scope Explicitly Not Completed

This phase did not add:

- runtime grant resolution or enforcement;
- capability request records;
- step-scoped tool or governed-context projection;
- authority receipts;
- tool execution, connectors, credential discovery, or provider calls;
- provider writes or new mutation families;
- workflow schemas, SDK fields, CLI behavior, examples, or runtime config;
- agent teams, recursive agents, memory infrastructure, hosted administration,
  enterprise identity, or cryptographic claims;
- release posture changes.

## 4. Model Types Added

- `CapabilityGrantId`
- `CapabilityReference`
- `CapabilityResourceKind`
- `CapabilityResourceScope`
- `CapabilityGrantScope`
- `CapabilityGrantLifecycle`
- `CapabilityDelegationPosture`
- `CapabilityGrantRequirements`
- `CapabilityGrantDefinition`
- `CapabilityGrant`
- `CapabilityAvailability`
- `CapabilityAvailabilityRecord`

## 5. Validation Boundary Summary

The model validates bounded identifiers, resource references, workflow/run/step
scope, grant issuance and expiry ordering, lifecycle/revocation consistency,
delegation depth, prerequisite-reference uniqueness, sensitivity posture, and
redaction metadata. Every grant is workflow-scoped; step scope requires run
scope. Delegation is disabled by default and bounded when enabled.

Unknown resource kinds, raw paths and URLs, traversal-like resource references,
unknown sensitivity, malformed lifecycle state, and secret-like caller-supplied
values fail closed with stable errors that do not echo raw values.

## 6. Availability Semantics

Availability is explicit inventory/connectivity posture rather than one
permissive boolean:

- `available`
- `declared_not_connected`
- `known_unsupported`
- `unknown`

The blocker fix removed authorization, denial, expiry, and revocation outcomes
from this inventory source of truth. Those outcomes must be derived from grants
and independent decisions. The model does not connect adapters or make tools
executable.

## 7. Privacy And Redaction

The model stores stable references and bounded metadata only. It does not store
credentials, tokens, provider payloads, raw evidence, command output, source
contents, raw policy payloads, or unrestricted metadata maps. Debug output for
grant, scope, requirements, definition, and availability records is redacted.
Serialization and deserialization validation fail closed without echoing
secret-like input values.

## 8. Test Coverage

Focused tests cover:

- valid definition, grant, and availability construction;
- invalid identifiers and resource references;
- workflow/run/step scope rules;
- expiry, revocation, and delegation posture;
- prerequisite-reference validation and uniqueness;
- sensitivity and redaction validation;
- serde round trips and invalid serialized input;
- redaction-safe Debug and error behavior;
- all availability states.

Existing workspace tests continue to cover WorkReport, EvidenceReference,
approval, policy, SideEffect, immutable run bundle, executor, adapter, runtime,
and CLI behavior.

## 9. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Governed Phase Record

- workflow: `dg/implement`
- run ID: `run-1784159939860725000-2`
- approval ID:
  `approval/run-1784159939860725000-2/implementation-approved`
- approval presentation ID: `presentation/184b765a50b10f60`
- approval outcome: granted under delegated maintainer authority
- phase status: completed before repository phase close

## 11. Remaining Known Limitations

- Availability records are descriptive inputs, not runtime authority decisions.
- Grants are not resolved against actor, resource, run, or step context by a
  runtime helper yet.
- Policy, approval, evidence, and check prerequisites are stable references but
  are not evaluated by this model.
- Grant expiry and revocation are modeled but not enforced at invocation time.
- Tool visibility, context access, authority receipts, and capability requests
  remain future phases.

## 12. Recommended Next Phase

Perform a focused maintainer review of this core model. If accepted, implement a
pure capability resolution helper that consumes explicit validated inputs and
returns a deterministic decision without runtime mutation, adapter access,
provider behavior, or writes.
