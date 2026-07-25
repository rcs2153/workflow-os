# Canonical Local Check Declaration-Set Resolver Report

## 1. Executive Summary

Workflow OS can now resolve one validated workflow step's authored local-check
requirements against an explicit validated allowlisted command-contract
inventory and return a deterministic, content-addressed declaration-set record.
The resolver is pure and in-memory. It does not inspect a repository, discover
commands, execute checks, publish immutable bundles, or grant runtime
authority.

## 2. Scope Completed

- Added a validated explicit local-check command-contract inventory.
- Added a resolved canonical local-check obligation model.
- Added a versioned canonical declaration-set record for one workflow step.
- Added a pure resolver over explicit workflow, step, inventory, and immutable
  bundle version inputs.
- Added deterministic obligation and declaration-set fingerprints.
- Added fail-closed serde, fixed non-leaking errors, and redaction-safe
  `Debug`.
- Added focused behavior, determinism, privacy, and corruption tests.

## 3. Scope Explicitly Not Completed

This phase does not add:

- immutable-bundle record publication, manifest references, or storage;
- local-check execution, handler discovery, registration, or defaults;
- authoritative structural-coverage adaptation or aggregate posture;
- proportional-governance reassessment or executor checkpoints;
- runtime persistence, events, evidence, reports, artifacts, or CLI behavior;
- repository inspection, inferred requirements, provider calls, SideEffects,
  writes, schemas, examples, hosted behavior, or release changes.

## 4. API Summary

`LocalCheckCommandContractInventory::new` validates every supplied contract and
rejects duplicate command identities.

`resolve_canonical_local_check_declaration_set` accepts:

- one already validated `WorkflowDefinition`;
- one exact `StepId`;
- one explicit validated `LocalCheckCommandContractInventory`; and
- one `ImmutableRunBundleVersion`.

It returns `CanonicalLocalCheckDeclarationSetRecord`. The API reads no hidden
state and performs no I/O.

## 5. Resolution And Validation Boundary

For every authored declaration, the resolver:

1. resolves exactly one command contract by command identity;
2. rejects unavailable command references;
3. verifies that contract network and SideEffect posture do not exceed the
   declaration maxima;
4. computes the existing canonical command-contract fingerprint;
5. constructs the existing independent attestation requirement and retains its
   independently derived fingerprint;
6. derives an obligation identity from workflow, workflow version, step,
   bundle version, declaration fields, command kind, and both fingerprints;
7. sorts obligations by that identity; and
8. rejects duplicate requirement, command, or obligation identities.

An empty authored list produces a valid content-addressed authoritative empty
record. A missing record is therefore distinguishable from an empty record in
the future bundle phase.

## 6. Determinism And Integrity

Authored declaration order and inventory order do not change the result.
Changes to command-contract policy, declaration posture, workflow/step binding,
or immutable bundle version change the derived identity.

Record deserialization reconstructs each independent attestation requirement,
recomputes every obligation identity, canonicalizes ordering, and recomputes
the declaration-set fingerprint. A mismatch fails closed with a fixed serde
error.

## 7. Privacy And Redaction

The canonical record stores bounded identities, enums, and fingerprints only.
It excludes executable text, arguments, working directories, environment
values, output, source content, provider payloads, credentials, and evidence
bodies.

`Debug` redacts workflow, step, requirement, command, bundle-version, and
fingerprint values. Validation and serde errors do not echo caller-supplied
identities or payloads.

## 8. Test Coverage

Focused tests cover:

- deterministic resolution independent of declaration and inventory order;
- exclusion of unreferenced inventory contracts from the canonical set;
- exact command-contract and attestation-requirement fingerprints;
- authoritative empty records;
- duplicate inventory identities and unknown commands;
- SideEffect maximum enforcement;
- exact-step selection and duplicate semantic obligations;
- contract and bundle-version fingerprint invalidation;
- serde round trip, nested-declaration tamper rejection, and outer-fingerprint
  tamper rejection;
- redaction-safe `Debug`; and
- exclusion of executable and raw-payload fields.

## 9. Governed Execution

- workflow: `dg/implement`
- run: `run-1784994396370848000-2`
- approval: `approval/run-1784994396370848000-2/implementation-approved`
- presentation: `presentation/78b6cc9b3048dab8`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations
- presentation-proof close posture: bounded proof-store read error; the helper
  did not claim an available proof record or event marker
- kernel boundary: governance coordination only; inspection, edits, tests, and
  documentation ran outside the kernel

## 10. Validation

The final implementation tree passes:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo test -p workflow-core --test local_check_declaration_set`;
- `npm run check`, including docs, dogfood-helper, integration-helper,
  TypeScript, SDK, and contract checks; and
- `git diff --check`.

Live provider integration tests and other explicitly opt-in external checks
were not enabled. No handler, provider, artifact, or runtime behavior was
simulated to claim broader coverage.

## 11. Known Limitations

- Records are not yet published into immutable run bundles.
- The resolver trusts its workflow input to have passed project validation,
  while still failing closed on missing or duplicate step/obligation identity.
- Historical bundles have no authoritative declaration-set record.
- Structural coverage cannot yet consume these records as authoritative input.
- No runtime gate reads this model.

## 12. Recommended Next Phase

Perform a phase-level maintainer review of deterministic identity,
contract-resolution semantics, fail-closed serde, and privacy. If accepted,
proceed to immutable-run bundle publication as a separate implementation
phase.
