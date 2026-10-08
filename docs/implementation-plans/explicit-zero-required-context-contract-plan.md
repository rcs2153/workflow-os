# Explicit Zero Required-Context Contract Plan

Status: implemented; focused implementation review required before trusted-host
admission resumes.

## 1. Executive Summary

The trusted-host local application admission slice needs to prove that one
exact closed docs-check operation requires no capability grants, no capability
availability facts, and no governed-context references. Core cannot express
that proof today: required-context contracts reject an empty requirement set,
and consumption rejects an empty projection set.

The smallest safe correction is an explicit zero-required-context posture in
the canonical contract model. It must be different from missing, omitted,
unknown, incomplete, or caller-supplied empty data. Existing non-empty
contracts keep their current constructors and semantics. Only a private
Core-owned constructor for the reviewed closed docs-check profile may create
the first explicit-zero binding.

This plan does not implement the model or resume trusted-host admission.

## 2. Goals

- Represent an explicit, complete declaration that an exact contract requires
  zero governed-context facts.
- Preserve fail-closed behavior for missing, omitted, unknown, and incomplete
  requirement declarations.
- Bind the declaration into the contract content hash and downstream execution
  commitments.
- Preserve current behavior and serialization compatibility for established
  non-empty contracts.
- Permit zero projections during consumption only for an explicit-zero
  contract.
- Let registered current authority prove complete empty inventories for the
  exact closed docs-check binding.
- Keep the first issuer private, Core-owned, and profile-specific.

## 3. Non-Goals

This phase does not authorize:

- implementation during planning;
- a public constructor for arbitrary empty contracts;
- optional or inferred zero-context posture;
- caller-authored authority inventories;
- trusted-host application admission or a local-host binary;
- additional local-check profiles or handler families;
- provider access or mutation;
- released CLI, SDK, schema, or example changes;
- hosted or distributed execution;
- OpenShell or nested harness execution; or
- release posture changes.

## 4. Existing Model Conflict

`RequiredContextContractBinding::new` requires at least one requirement and
returns `required_context.contract.requirements_empty` for an empty vector.
Deserialization runs the same invariant. `consume_required_context` separately
returns `required_context.consumption.projections_empty` for an empty
projection set.

Those invariants were correct while every contract described context that had
to be consumed. They cannot represent a reviewed closed operation whose
complete requirement declaration is exactly zero. Treating an absent contract
or empty caller vector as equivalent would weaken the fail-closed boundary.

## 5. Candidate Core Model

Add a closed contract requirement posture, conceptually:

```text
RequiredContextRequirementPosture
  - DeclaredRequirements
  - ExplicitNone
```

`RequiredContextContractBinding` retains its canonical requirement vector and
adds the posture to its validated and hashed state.

### 5.1 Non-empty contracts

The existing public `new(id, version, requirements)` constructor remains
unchanged at its call sites and continues to require a non-empty canonical
requirement vector. It creates `DeclaredRequirements`.

### 5.2 Explicit-zero contract

Add a crate-private constructor that creates `ExplicitNone` with an empty
requirement vector. The constructor must require a private closed-profile
authorization value issued only after the docs-check profile proves:

- its command contract is canonical;
- it requests no capabilities;
- it requests no capability-availability facts;
- it requests no governed-context references; and
- no caller field can alter those declarations.

The private authorization is not a capability grant. It is construction proof
for one exact zero-requirement contract.

### 5.3 Invalid combinations

Validation rejects:

- `DeclaredRequirements` with an empty vector;
- `ExplicitNone` with any requirement;
- missing posture paired with an empty vector;
- unknown posture values;
- duplicate or unordered non-empty requirements; and
- content hashes that omit or mismatch the posture.

## 6. Hashing And Identity

The contract content hash must include a versioned domain separator plus the
requirement posture. An explicit-zero contract must not hash as an omitted
field, an empty legacy vector, or any non-empty contract.

The first implementation should retain the existing hash behavior for legacy
non-empty contracts if possible. If including the posture necessarily changes
those hashes, implementation must stop for a compatibility decision rather
than silently rebinding existing contracts.

The preferred shape is:

- preserve the current non-empty hash domain and bytes for
  `DeclaredRequirements`; and
- use a new domain-separated hash path for `ExplicitNone`.

## 7. Serialization And Compatibility

The wire representation must make explicit zero distinguishable from absence.

Recommended compatibility posture:

- existing serialized non-empty contracts without a posture marker continue
  to deserialize as `DeclaredRequirements`;
- serializers may omit the marker for `DeclaredRequirements` to preserve the
  existing wire shape;
- `ExplicitNone` must serialize an explicit posture marker and an empty
  requirement list;
- empty requirements without that marker fail closed;
- a marker with non-empty requirements fails closed; and
- deserialization errors remain stable and payload-free.

No public workflow schema field is added in this phase. The representation is
an internal Core contract until separately reviewed.

## 8. Consumption Semantics

`consume_required_context` must branch on the validated posture:

- `DeclaredRequirements` keeps current behavior, including rejection of an
  empty projection set;
- `ExplicitNone` requires exactly zero projections and returns `Satisfied`
  with zero satisfactions and zero gaps; and
- any projection supplied to `ExplicitNone` fails closed because it conflicts
  with the exact zero declaration.

The result must retain the exact contract and execution context so the zero
consumption remains inspectable and commitment-bound. It must not manufacture
a satisfaction record or evidence reference.

## 9. Closed Docs-Check Profile Binding

The resolved explicit docs-check profile needs a private, closed declaration
of its current-authority requirements. The declaration must be part of the
profile's deterministic binding material and must state explicit zero for:

- capability grants;
- capability availability; and
- governed-context references.

Core may issue the explicit-zero required-context contract only after matching
that declaration to the canonical docs-check command contract. A resolver
mismatch, unknown profile, modified command contract, or future non-zero
requirement blocks issuance.

The application cannot request `ExplicitNone` directly.

## 10. Registered Current-Authority Integration

For the exact explicit-zero execution binding, the private registered source
may hold complete empty inventories. Its configuration commitment must bind:

- the immutable run bundle;
- explicit-zero contract ID, version, posture, and content hash;
- the docs-check command-contract fingerprint;
- the exact workflow, run, step, and actor binding;
- the three explicit empty inventory declarations;
- source generation, freshness, validity, sensitivity, and redaction posture.

Resolution succeeds only when all inventories are complete and empty for that
binding. Missing source state, stale observations, unknown completeness,
non-empty facts, conflicting facts, or a non-zero contract remains blocked.

Policy, approval, evidence, checks, operational-window authority, and dispatch
reservation remain independent and cannot be satisfied by this contract.

## 11. Privacy And Error Handling

- Debug output exposes posture and counts only, never IDs or payloads.
- Validation and deserialization errors use stable non-leaking codes.
- No source paths, command arguments, environment values, context targets,
  tokens, credentials, or private policy details appear in errors.
- Invalid zero declarations fail before authority resolution or execution.
- No fallback converts invalid explicit zero into an ordinary empty vector.

## 12. Test Plan

Future implementation tests must prove:

1. existing non-empty contracts construct, hash, serialize, deserialize, and
   consume unchanged;
2. the public constructor still rejects an empty requirement vector;
3. the private closed-profile path constructs one explicit-zero contract;
4. explicit zero has a deterministic domain-separated content hash;
5. explicit zero serializes with an unambiguous marker;
6. valid explicit zero round-trips;
7. empty requirements without the marker fail closed;
8. `DeclaredRequirements` plus empty requirements fails closed;
9. `ExplicitNone` plus any requirement fails closed;
10. unknown posture values fail closed without leakage;
11. explicit zero consumes zero projections as `Satisfied` with zero
    satisfactions and gaps;
12. explicit zero rejects any supplied projection;
13. non-empty contracts still reject zero projections;
14. only the canonical docs-check profile can issue the private authorization;
15. substituted profile or command-contract bindings fail closed;
16. complete empty current-authority inventories resolve only for the exact
    explicit-zero execution binding;
17. missing, stale, incomplete, non-empty, or conflicting inventories block;
18. Debug and serde errors remain non-leaking; and
19. existing required-context, current-authority, local-check, executor,
    SQLite, trusted-host, CLI, and workspace tests pass.

## 13. Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Add the closed posture and compatibility-preserving validation/hash/serde
   behavior.
3. Add exact zero-projection consumption semantics.
4. Add the private docs-check issuer and deterministic profile declaration.
5. Add registered-source exact-empty integration.
6. Add focused and regression tests.
7. Run full workspace validation.
8. Perform a focused implementation review.
9. Resume the trusted-host local application admission vertical slice only
   after acceptance.

## 14. Open Questions

- Can the non-empty contract hash remain byte-for-byte unchanged while adding
  the internal posture?
- Should the explicit-zero marker use a new contract model version even though
  it remains internal?
- Should the zero-consumption result expose a dedicated bounded posture, or is
  `Satisfied` plus zero counts sufficient and less disruptive?
- Which private type should prove the docs-check profile's three exact empty
  declarations without becoming reusable ambient authority?
- Does any persistence codec assume at least one requirement or projection
  beyond the canonical constructors?

## 15. Final Recommendation

Proceed next to a focused maintainer/security review. If accepted, implement
the explicit-zero model and consumption semantics as a prerequisite-only
phase. Do not combine that implementation with the trusted-host binary or
admission wiring.
