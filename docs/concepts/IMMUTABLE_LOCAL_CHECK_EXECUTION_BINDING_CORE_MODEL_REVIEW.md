# Immutable Local Check Execution Binding Core Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups. Proceed to the pure independent
local-check attestation verifier only after preserving the provenance boundary
described below.

## 2. Scope Verification

The phase stayed within the approved model-only scope. It added a payload-free,
content-addressed pre-execution binding, handler-selection vocabulary, canonical
fingerprints, safe serde and Debug behavior, focused tests, and honest
documentation.

It did not add process execution, accepted attestation, verifier behavior,
executor integration, persistence, events, schemas, CLI behavior, providers,
SideEffects, writes, hosted behavior, or release changes.

## 3. Model Assessment

`ImmutableLocalCheckExecutionBinding` is appropriately narrow. It binds:

- immutable run-bundle identity, version, and root;
- workflow, run, step, skill, and command identity;
- the complete canonical command-contract fingerprint;
- explicit registered-handler selection metadata;
- an honest `RegisteredUnattested` handler posture;
- the effective execution-policy fingerprint; and
- creation time.

The model correctly avoids claiming that current immutable run bundles already
contain local-check command definitions. The separate binding is the right
compatibility boundary until run-bundle composition is deliberately expanded.

## 4. Command And Policy Binding Assessment

The command fingerprint covers the executable, ordered arguments, execution
posture, working-directory policy, environment-name policy, network policy,
timeout, SideEffect class and allowed effects, permitted output directories,
bounded output capture, raw-output posture, redaction policy, and citation
kinds.

Fields that are semantically sets are sorted before hashing. Ordered arguments
remain ordered. The effective-policy commitment repeats the fields that govern
execution rather than relying on a partial or caller-selected summary.

The fixed-width framed hashing and domain-separated algorithm identifiers are
deterministic and suitable for a preview core model.

## 5. Handler Assurance Assessment

The handler selection commits typed selection metadata: command kind, resolved
skill identity/version, explicit registration mode, and
`RegisteredUnattested` posture. It does not inspect an opaque trait object or
claim source, binary, deployment, or implementation attestation.

That honesty is essential. The model can support future
`KernelObservedLocalProcess` assurance, but it cannot by itself establish that a
trusted handler implementation executed.

## 6. Validation And Serde Assessment

Construction validates the command contract and handler selection, then rejects
skill/version and command-kind mismatch. Handler-selection deserialization
recomputes its commitment. Binding deserialization validates the nested
selection and recomputes the complete binding fingerprint. Invalid values fail
closed through bounded errors that do not echo caller data.

The serialized fingerprint is tamper-evident, not authenticated provenance.
The algorithm is public and callers can construct model values. A future
verifier therefore must not treat deserialization or fingerprint equality as
proof that Core created or observed the binding.

## 7. Privacy And Redaction Assessment

The model stores no raw output, environment values, provider payloads, source
contents, credentials, or free-form claims. Debug output redacts workflow, run,
step, skill, command, timestamp, and fingerprint values. The nested immutable
run-bundle Debug posture remains safe under focused tests.

Serialization necessarily exposes stable identities and fingerprints because
they are the model's payload-free contract. This is compatible with the
documented preview privacy boundary and does not expose execution payloads.

## 8. Test Quality Assessment

Focused tests cover valid construction and serde round trip, stable command and
binding vectors, canonical ordering for set-like fields, distinct command
contracts, handler/skill/command mismatch, serialized tampering, run/time
sensitivity, forbidden payload markers, and Debug non-leakage.

The full workspace suite also passed, covering existing local-check, runtime,
report, evidence, adapter, approval, persistence, and provider behavior.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

1. The verifier must require Core-owned binding provenance or an equivalent
   trusted invocation boundary. It must never accept arbitrary deserialized
   bindings merely because their public fingerprint recomputes.
2. Runtime composition must create the binding before observation or execution
   and must reject mutable-context drift.
3. Stronger handler implementation provenance remains a separate assurance
   tier; do not silently upgrade `RegisteredUnattested`.
4. The dogfood phase-close approval-presentation list-cap defect remains open.

## 11. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784516244656830000-2`
- approval: `approval/run-1784516244656830000-2/review-scope-approved`
- presentation: `presentation/56de2db309cbb816`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; review inspection,
  documentation edits, and validation ran outside the kernel

## 13. Recommended Next Phase

Implement the pure crate-private independent local-check attestation verifier
described by the accepted verifier plan. The verifier may evaluate an existing
binding and crate-private kernel observation into an accepted or rejected
decision. It must not execute a process, persist records, integrate the
executor, add schemas or CLI behavior, or broaden provider writes.
