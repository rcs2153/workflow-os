# Independent Local Check Attestation Verifier Plan Blocker Fix Report

## 1. Executive Summary

The verifier plan's immutable-input blockers are fixed at the planning layer.
The corrected design introduces a separate pre-execution immutable local-check
execution binding and narrows the first assurance claim to what Core can prove.

No Rust model, verifier, process execution, runtime integration, persistence,
event, schema, CLI, provider, SideEffect, or write behavior was implemented.

## 2. Blockers Fixed

The original plan incorrectly assumed that the current immutable run bundle
contained local-check command definitions and a trusted handler implementation
identity. Current bundle definitions contain workflow, skill, and policy
records, while current handler posture does not attest implementation integrity.

The corrected plan no longer relies on either unsupported assumption.

## 3. Implementation Approach

The next model phase should add a content-addressed
`ImmutableLocalCheckExecutionBinding` created by Core before observation or
execution. It references the immutable run bundle and binds:

- workflow, run, step, and skill identity;
- canonical command-contract fingerprint;
- Core-derived handler selection fingerprint over typed registration metadata;
- honest `RegisteredUnattested` posture;
- registration profile or mode;
- effective working-directory, environment, network, timeout, SideEffect,
  output-capture, redaction, and citation policy commitments.

The binding remains separate from the current immutable bundle definition
taxonomy. A later composition phase may decide whether to add it to a manifest
or retain a separately rooted execution-input ledger.

## 4. Assurance Boundary

The first accepted assurance is `KernelObservedLocalProcess`. It proves that
Core observed a local process under an exact pre-bound registered handler and
execution policy. It does not prove handler source integrity, binary provenance,
cryptographic identity, trusted-host posture, or third-party execution.

Mock, declared-only, unavailable, substituted, caller-supplied, or unbound
handler posture remains insufficient.

## 5. Validation Boundary

The later verifier must validate both the stored immutable run bundle and the
pre-execution binding, recompute the current command contract against the frozen
commitment, and match bundle/run/step/skill/handler/policy context exactly.
Changing the binding invalidates a candidate even when command ID and result
status remain unchanged.

## 6. Privacy And Errors

The binding is payload-free. It stores fingerprints, typed posture, and bounded
references rather than raw output, arguments, paths, source contents,
environment values, credentials, or provider payloads. Errors remain stable and
must not echo caller values.

## 7. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784512884428399000-2`
- approval: `approval/run-1784512884428399000-2/fix-approved`
- presentation: `presentation/6dd03f0e56d2d534`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; documentation edits and checks
  were performed outside the kernel

## 8. Validation

- `npm run check:docs`
- `git diff --check`

## 9. Remaining Limitations

- The immutable execution binding is not implemented.
- The verifier and accepted proof are not implemented.
- Handler implementation integrity remains explicitly unattested.
- Persistence, time-of-use checks, events, evidence, reports, runtime wiring,
  schemas, CLI behavior, providers, SideEffects, and writes remain deferred.
- The dogfood phase-close presentation-record list-cap defect remains open.

## 10. Recommended Next Phase

Repeat focused plan review. If accepted, implement the immutable local-check
execution binding core model only, then review it before verifier implementation.
