# Step-Scoped Capability Projection Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation is a bounded, deterministic, non-executing projection of
fresh same-context capability resolutions. It includes only authorized
capability references, retains validated source-resolution proof across serde,
fails closed on context or grant substitution, and does not introduce runtime
tool access or provider behavior.

## 2. Scope Verification

The phase stayed within the approved pure model/helper scope.

It did not add tool loading, invocation, command execution, connector
activation, provider calls, provider writes, context dereference, persistence,
events, schemas, SDKs, CLI behavior, hosted administration, enterprise
identity, authority receipts, cryptographic claims, or release changes.

## 3. Model Assessment

`StepScopedCapabilityProjectionInput` requires explicit actor, workflow, run,
step, optional harness, evaluation timestamp, capability resolutions, and
redaction metadata. It reads no hidden state.

`StepScopedCapabilityProjection` owns the exact scope and a deterministic list
of authorized entries. Entries expose only capability, resource, and selected
grant references through read-only accessors. They retain the complete
payload-free `CapabilityResolution` used to establish their posture.

The model remains domain-neutral and does not assume a particular tool,
connector, provider, or agent runtime.

## 4. Authority And Scope Assessment

The helper validates every source resolution and requires exact equality for:

- actor;
- workflow;
- run;
- step;
- optional harness contract;
- evaluation timestamp.

It rejects duplicate capability/resource resolutions rather than selecting a
winner. Only `authorized` resolutions become entries. `not_authorized` and
`requires_independent_evaluation` resolutions remain absent.

The exact evaluation timestamp establishes resolution-batch consistency only.
It is not a TTL, lease, durable authority receipt, or time-of-use freshness
proof. The implementation and report state this boundary clearly.

## 5. Wire Integrity Assessment

Each serialized entry retains its source resolution and selected grant ID.
Entry validation proves:

- the source resolution itself is valid;
- its posture is `authorized`;
- it carries a selected grant;
- the serialized grant ID equals that selected grant.

Projection validation then proves that every source resolution matches the
outer projection context and timestamp. Tampering with a grant or step fails
closed during deserialization.

A valid deserialized projection is still an inspectable resolution snapshot,
not self-authenticating runtime authority. Future runtime integration must use
trusted immutable inputs or re-resolve authority at invocation time.

## 6. Determinism Assessment

Capability/resource pairs are unique and sorted by capability, resource kind,
and resource reference. Deserialization rejects duplicates and out-of-order
entries. Empty projections are valid and explicitly represent a step with no
currently authorized projected capabilities.

## 7. Privacy And Redaction Assessment

- Input, entry, and projection Debug implementations redact identities,
  capabilities, resources, grants, and redaction metadata.
- Stable errors describe failure classes without echoing caller values.
- Existing bounded ID, resource, sensitivity, resolution, and redaction models
  remain the validation boundary.
- The projection stores no provider payload, source content, parser output,
  command output, credentials, environment values, or unrestricted metadata.

Serialization intentionally contains bounded stable references and source
resolution posture. It must therefore be handled according to the declared
sensitivity even though it contains no raw payloads or credentials.

## 8. Test Quality Assessment

Focused tests cover:

- authorized-only inclusion;
- denied-only empty projection;
- stale batch rejection;
- cross-step rejection;
- duplicate resolution rejection;
- serde round trip;
- forged grant rejection;
- forged source-context rejection;
- Debug non-leakage;
- absence of forbidden raw payload field categories.

The workspace regression suite, including capability request, grant,
availability, resolution, EvidenceReference, Diagnostic, validation, adapter,
runtime, SideEffect, report, and approval behavior, passes.

## 9. Documentation Assessment

The roadmap, implementation plan, and phase report state that:

- pure step-scoped projection is implemented;
- the model is non-executing;
- exact timestamp equality is batch consistency, not a runtime lease;
- runtime re-resolution remains required;
- tool loading, context access, provider calls, writes, persistence, events,
  schemas, CLI behavior, receipts, hosted behavior, and enterprise identity
  remain unimplemented.

The documentation does not overclaim current capability.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Define an explicit runtime freshness policy before any consumer treats a
  projection as actionable.
- Bind future runtime projections to immutable run inputs or a reviewed
  authority receipt rather than trusting arbitrary serialized snapshots.
- Consider a stable projection identity only when persistence or audit linkage
  is separately planned.
- Preserve the distinction between capability visibility and invocation
  success in future harness and adapter integration.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test capability_authority --quiet`: passed,
  48 tests.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Recommended Next Phase

Proceed to **governed context-access model and projection planning**, followed
by a model-only implementation if the plan is accepted. Begin with stable
references and bounded metadata only. Do not dereference source, provider, or
memory payloads; do not add tool execution, provider calls, writes, persistence,
receipts, schemas, or CLI behavior.
