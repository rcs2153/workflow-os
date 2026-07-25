# Canonical Local-Check Declaration Immutable-Bundle Publication Report

## 1. Executive Summary

Workflow OS now has an explicit immutable-bundle construction path that
publishes one canonical local-check declaration-set record for every workflow
step, including authoritative empty records. The manifest stores payload-free
typed references, includes them in the bundle root, and the local create-only
store validates the referenced records before publishing or reading a bundle.

This is declaration publication only. It does not execute checks, infer
commands, convert aggregate governance posture, or enforce executor gates.

## 2. Scope Completed

- Added typed declaration-set references.
- Added an explicit enriched immutable-bundle builder.
- Bound declaration-set references into the manifest root hash.
- Published content-addressed declaration-set records before the manifest
  commit marker.
- Validated declaration-set records and references on write and read.
- Preserved legacy bundle readability and root behavior.
- Added focused builder, compatibility, storage, and failure tests.

## 3. Scope Explicitly Not Completed

- No local-check execution or handler default registration.
- No repository inspection or inferred command inventory.
- No structural-coverage authority or aggregate posture conversion.
- No executor checkpoint or run-creation gate.
- No CLI, workflow schema, provider, SideEffect execution, or write behavior.
- No hosted/distributed behavior or release-posture change.

## 4. Model And API Summary

`CanonicalLocalCheckDeclarationSetReference` carries only workflow identity,
workflow version, step identity, immutable-bundle version, algorithm, and the
declaration-set fingerprint. It does not copy declarations or command
payloads.

`build_immutable_run_bundle_with_local_check_declarations` accepts the existing
bundle request plus an explicit validated command-contract inventory. It
resolves every workflow step through the existing pure resolver and constructs
an enriched manifest. `build_immutable_run_bundle` remains the legacy path and
publishes no authoritative declaration-set references.

Build and stored-bundle results expose the declaration-set records through
read-only accessors and their consuming parts APIs.

## 5. Immutable-Bundle Boundary

Every enriched bundle contains exactly one declaration-set reference per
workflow step. A step with no declarations receives an authoritative empty
record. A missing record is therefore distinguishable from an empty record.

References participate in the bundle root. A changed referenced declaration,
requirement, command contract, step binding, bundle version, or algorithm
changes the declaration-set fingerprint and therefore the root. Unreferenced
inventory contracts do not affect the bundle.

Legacy manifests omit the reference collection. They remain deserializable,
but the omission is non-authoritative and cannot be interpreted as empty
local-check coverage.

## 6. Persistence And Atomicity

The local immutable-bundle store writes declaration-set records by content
address before publishing the run-addressed manifest. The manifest remains the
commit marker. Identical content-addressed writes are idempotent; conflicting,
missing, corrupt, mismatched, ambiguous, or unreferenced records fail closed.

If manifest publication fails, any preceding content-addressed record is an
immutable orphan and cannot rebind an existing run.

## 7. Privacy And Error Posture

Manifest references and stored declaration records exclude raw command output,
raw source, arbitrary repository paths, environment values, provider payloads,
credentials, and evidence bodies. `Debug` output remains redacted. Validation,
storage, and deserialization errors use stable bounded codes and do not echo
caller-supplied values.

## 8. Test Coverage

Focused tests cover:

- one record and reference per workflow step;
- authoritative empty step records;
- legacy bundle serialization and non-authoritative posture;
- deterministic roots and unreferenced-inventory stability;
- enriched roots differing from legacy roots;
- create-only store round trips;
- manifest refusal when a declaration-set record is absent; and
- post-restart failure when a referenced record is missing.

Existing immutable-bundle builder and store tests remain green.

## 9. Validation Commands

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

All commands passed. The workspace suite completed with no failures; only
explicitly opt-in live integration tests were ignored.

The governed implementation and review runs completed. Review approval used
presentation proof `presentation/8cd2e781b5b794a6`, and the run-local
`ApprovalGranted` event exposes a bounded `present` proof marker for that
presentation. The repo-local `phase-close` helper could not independently
reread the proof record because its bounded global presentation-store scan
reached 250 records. That scaling defect is separately recorded in the roadmap
and does not erase the persisted run-local proof marker.

## 10. Remaining Limitations

The records are not yet adapted into authoritative structural-coverage input.
No executor or run-creation path requires enriched bundles. The command
inventory remains an explicit caller-owned validated value rather than a
project spec or plugin inventory. Legacy bundles remain readable but cannot
prove local-check declaration coverage.

Dogfood phase-close proof discovery is not scale-safe yet: it scans the shared
presentation store rather than resolving the exact run and approval record.

## 11. Recommended Next Phase

Perform the immutable-bundle publication maintainer review. If accepted,
implement the private authoritative adapter from validated stored declaration
records to the existing structural-coverage evaluator. Do not add executor
gates until that adapter has been reviewed.
