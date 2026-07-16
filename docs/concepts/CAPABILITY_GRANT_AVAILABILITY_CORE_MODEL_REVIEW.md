# Capability Grant And Availability Core Model Review

## 1. Executive Verdict

**Needs blocker fixes.**

The capability-grant model is appropriately scoped, deterministic,
redaction-safe, and suitable as a foundation. The availability model has one
source-of-truth blocker that must be corrected before a runtime resolution
helper consumes it: a caller can assert authorization outcomes through an
availability record without supplying or validating any grant or decision
reference.

## 2. Scope Verification

The phase stayed within the approved model-only scope. It added no runtime
resolution, capability request behavior, tool or context projection, authority
receipt, adapter activation, tool execution, provider write, schema, CLI,
example, hosted feature, enterprise identity integration, or release posture
change.

## 3. Grant Model Assessment

The grant model is domain-neutral and appropriately bounded. It represents:

- stable grant and capability identifiers;
- subject and issuer actors;
- bounded resource scope;
- workflow, optional run, optional step, and optional harness scope;
- issuance, expiry, active/revoked lifecycle, and revocation reference;
- delegation disabled by default with bounded depth when enabled;
- policy, approval, evidence, and local-check prerequisite references;
- sensitivity ceiling and redaction metadata.

Private validated model fields and read-only accessors prevent callers from
mutating an accepted grant into an invalid state. The public definition is an
input aggregate; `CapabilityGrant::new` remains the validation boundary.

## 4. Availability Model Assessment

The model correctly avoids activating adapters or executing tools. It also uses
an explicit taxonomy instead of a permissive boolean.

However, `CapabilityAvailabilityRecord` accepts
`CapabilityAvailability::AvailableAndAuthorized`, `Denied`, and
`ExpiredOrRevoked` while carrying only capability, resource, timestamp, and
redaction fields. It carries no grant ID, grant lifecycle proof, policy
decision, approval decision, or resolution reference. Deserialization accepts
the same caller-asserted posture.

That conflicts with the plan's mandatory boundary:

```text
Capability availability = current bounded runtime/adapter inventory.
Capability grant = durable scoped authority record.
Policy and approval decisions remain independent sources of truth.
```

An availability observation must not be able to manufacture an authorization
or denial fact. The current enum would let a future resolver trust an authority
claim that the model cannot substantiate.

## 5. Validation Assessment

Grant validation correctly enforces:

- bounded, canonical, non-secret-like identifiers and references;
- known resource kind;
- workflow scope for every grant;
- exact run scope before step scope;
- expiry after issuance;
- active/revoked and revocation-reference consistency;
- bounded delegation depth;
- bounded, unique prerequisite references;
- known sensitivity;
- bounded, non-secret-like redaction metadata.

Errors use stable codes and do not echo raw caller values. The blocker is not a
missing string validation rule; it is a missing source-of-truth invariant in
the availability shape.

## 6. Serde And Privacy Assessment

Serialized grant, resource scope, grant scope, requirements, and availability
records deserialize through validating constructors. Invalid redaction and
secret-like metadata fail closed without echoing the supplied value.

`Debug` output redacts grant IDs, actors, capability IDs, resource references,
scope IDs, prerequisite references, revocation references, and redaction
metadata content. The model stores no credentials, provider payloads, command
output, source contents, environment values, or unrestricted metadata maps.

The availability authority assertion remains serde-valid because the wire
shape has no proof field to validate; this is part of the blocker.

## 7. Relationship To Existing Governance Sources

The grant model composes stable references without copying policy, approval,
evidence, or check payloads. It does not treat a grant as proof that every
invocation passed policy or approval.

The availability model currently blurs that separation. Correcting it before
runtime resolution is important because proportional governance, SideEffect
authority, and future harness projection all depend on authorization being
derived from explicit validated inputs rather than asserted as inventory
metadata.

## 8. Test Quality Assessment

The 15 focused tests cover valid construction, identifiers, resources,
run/step scope, expiry, revocation, delegation, duplicate prerequisites,
sensitivity, serde round trips, invalid serde, Debug non-leakage, forbidden
payload fields, stable errors, and every current availability variant.

The tests are strong for the implemented invariants. The missing blocker test
is structural: there is currently no way to require an authority proof when
constructing `AvailableAndAuthorized`, or a decision reference when
constructing `Denied` or `ExpiredOrRevoked`.

Existing workspace tests passed and provide regression coverage across the
executor, immutable run bundle, approval, policy, EvidenceReference,
WorkReport, SideEffect, adapter, runtime, and CLI layers.

## 9. Documentation Review

The plan, roadmap, and implementation report accurately state that the core
model is implemented and runtime behavior remains deferred. This review adds
the missing blocker disclosure. No document claims that capability authority
is currently enforced at runtime.

## 10. Blockers

### Availability can assert authority without proof

Before the pure resolver phase, separate inventory/connectivity availability
from authority outcomes. The preferred minimal fix is:

- keep availability limited to facts such as available/connected,
  declared-not-connected, known-unsupported, and unknown;
- remove authorization, denial, expiry, and revocation from the availability
  source-of-truth type;
- let the future pure resolver derive authorization posture from an explicit
  validated grant plus policy/approval/evidence/check inputs;
- add serde and constructor tests proving inventory records cannot assert
  authorization.

An alternative proof-bearing combined record would be more complex and is not
justified for the first model slice.

## 11. Non-Blocking Follow-Ups

- Decide during resolver planning whether definition registries need a
  distinct declared capability model rather than reusing grant references.
- Consider false-positive ergonomics in conservative secret-like matching only
  after real callers expose a concrete problem.
- Document freshness semantics when the pure resolver introduces an evaluation
  timestamp.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Recommended Next Phase

Implement a focused **capability availability source-of-truth blocker fix**.
Do not begin the pure capability resolution helper until that fix is reviewed
and accepted. Do not add runtime enforcement, projections, requests, receipts,
tools, connectors, provider writes, schemas, CLI behavior, or hosted features
as part of the fix.

## 14. Fix-Forward Note

The blocker is fixed in
[Capability Grant And Availability Core Model Blocker Fix Report](CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_BLOCKER_FIX_REPORT.md).
The original finding remains the review verdict for the pre-fix model. A
focused blocker-fix review is still required before the pure resolver phase.
