# Canonical Local-Check Declaration Immutable-Bundle Publication Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation publishes canonical local-check declaration sets through a
narrow trusted bundle-builder path, commits payload-free references into the
bundle root, validates content-addressed records before manifest publication
and on read, and preserves legacy bundle semantics. Proceed to the private
stored-record-to-structural-coverage adapter.

## 2. Scope Verification

The phase stayed within the approved publication boundary. It added typed
references, enriched in-memory bundle construction, create-only local storage,
focused tests, roadmap updates, and a phase report.

It did not add check execution, default handlers, repository inference,
structural-coverage authority, aggregate posture conversion, executor gates,
CLI behavior, workflow schemas, providers, SideEffect execution, writes,
hosted behavior, or release changes.

## 3. Model Assessment

`CanonicalLocalCheckDeclarationSetReference` is appropriately payload-free and
domain-specific. It binds workflow identity and version, exact step,
immutable-bundle version, resolution algorithm, and declaration-set
fingerprint.

The reference does not copy declarations, command arguments, paths,
environment values, outputs, evidence bodies, provider payloads, or
credentials. Its `Debug` representation redacts identities and hashes.

## 4. Builder Assessment

The existing `build_immutable_run_bundle` path remains legacy and produces no
authoritative local-check declaration references.

The explicit
`build_immutable_run_bundle_with_local_check_declarations` path accepts a
validated command-contract inventory and uses the existing pure resolver for
every workflow step. This guarantees one record per step, including
authoritative empty records, without repository inspection or execution.

The enriched manifest constructor is crate-private. Callers cannot use the
public manifest API to fabricate a partial authoritative declaration-set
collection.

## 5. Root And Compatibility Assessment

Canonical declaration-set references are sorted by step and included in the
fixed-width-framed bundle root calculation. Referenced declaration or contract
changes therefore invalidate the record fingerprint and bundle root.
Unreferenced inventory contracts do not change the root.

The manifest wire shape defaults an absent declaration-set collection to empty
and omits empty collections during serialization. Existing serialized bundles
retain their prior root behavior and remain readable. An absent collection is
documented as legacy/non-authoritative; authoritative emptiness requires the
enriched per-step records.

## 6. Persistence And Atomicity Assessment

Declaration-set records are stored by their validated fingerprint before the
run-addressed manifest commit marker. Identical record writes are idempotent.
Conflicting, corrupt, missing, mismatched, ambiguous, and unreferenced records
fail closed.

Manifest publication resolves both canonical definition records and
declaration-set records first. A failed manifest publication may leave only
content-addressed immutable orphans and cannot silently rebind an existing run.
Read paths revalidate the envelope hashes and referenced records.

## 7. Error And Privacy Assessment

New validation and store errors use stable codes and bounded messages. They do
not echo workflow, step, command, path, output, provider, credential, or
secret-like values. Custom declaration-set deserialization recomputes
fingerprints, and manifest deserialization recomputes the bundle root.

No raw source, executable payload, command output, environment value, provider
payload, or evidence body is introduced into the manifest reference shape.

## 8. Test Assessment

Tests cover:

- exactly one declaration-set record/reference per step;
- authoritative empty step records;
- legacy serialization and non-authoritative omission;
- deterministic enriched roots;
- root separation between enriched and legacy bundles;
- unreferenced inventory stability;
- create-only enriched store round trips;
- missing record rejection before manifest publication; and
- missing referenced records after store restart.

The full workspace suite passes, preserving immutable-bundle, executor,
attestation, proportional-governance, provider, evidence, and report behavior.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add a checked legacy JSON fixture if immutable bundle wire compatibility is
  promoted from preview-internal behavior to a supported external contract.
- In the next adapter phase, reject legacy bundles and partial/missing
  declaration collections before constructing authoritative structural
  coverage.
- Keep command-inventory provenance explicit when a future plugin or project
  source is introduced.

## 11. Validation

Passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

The workspace suite completed with no failures. Only explicitly opt-in live
integration tests were ignored.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1785000337564551000-2`
- approval:
  `approval/run-1785000337564551000-2/review-scope-approved`
- presentation: `presentation/8cd2e781b5b794a6`
- approval outcome: granted by delegated maintainer through proof enforcement
- approval event marker: present and bound to the expected presentation
- kernel boundary: governance coordination only; implementation, inspection,
  and validation ran outside the kernel

The repo-local phase-close helper reported `proof_record_read_error` after its
bounded global presentation-store scan reached 250 records. Direct bounded
inspection of this run confirmed the proof-enforced `ApprovalGranted` marker.
The phase-close lookup defect is recorded in the roadmap as a P0 dogfood
follow-up; it does not change this review verdict.

## 13. Recommended Next Phase

Implement the private authoritative adapter from validated stored
declaration-set records to the existing structural-coverage evaluator. The
adapter must fail closed for legacy, missing, duplicate, or mismatched records
and must not yet convert aggregate governance posture or add executor gates.
