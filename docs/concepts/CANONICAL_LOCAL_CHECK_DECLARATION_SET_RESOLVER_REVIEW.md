# Canonical Local Check Declaration-Set Resolver Review

## 1. Executive Verdict

**Phase accepted; proceed to immutable-run bundle declaration-set publication.**

The implementation provides the missing pure boundary between authored
step-scoped local-check requirements and future immutable-run authority. It
resolves only explicit allowlisted command contracts, enforces authored
maxima, derives deterministic content identities, and returns a validated
in-memory record without executing checks or granting runtime authority.

## 2. Scope Verification

The phase stayed within its approved model-and-resolver scope.

It added:

- an explicit validated command-contract inventory;
- a canonical resolved obligation model;
- a versioned declaration-set record;
- deterministic obligation and set fingerprints;
- a pure exact-step resolver;
- fail-closed serde and redaction-safe `Debug`;
- focused tests; and
- honest roadmap, plan, and phase-report updates.

It did not publish immutable bundles, inspect repositories, discover commands,
execute checks, register handlers, persist records, emit runtime events,
construct evidence or reports, expose CLI behavior, change schemas, call
providers, authorize SideEffects or writes, add hosted behavior, or change
release posture.

## 3. Inventory And Resolution Assessment

`LocalCheckCommandContractInventory` accepts explicit validated contracts and
rejects duplicate command identities. Resolution does not read hidden state or
infer commands from a repository.

The resolver selects exactly one workflow step, resolves each declaration by
command identity, rejects missing command contracts, and rejects duplicate
requirement or command obligations. Unreferenced inventory contracts do not
affect the resulting declaration set.

Network posture must match the currently supported disabled-only declaration
vocabulary. SideEffect posture uses an explicit bounded ordering and rejects
unclassified values. This is appropriate for v0. Future taxonomy expansion
must revise the versioned algorithm rather than silently changing v1 meaning.

## 4. Canonical Record Assessment

Each resolved declaration binds:

- workflow, workflow version, and exact step identity;
- immutable bundle model version;
- authored requirement and command identity;
- command kind;
- the existing canonical command-contract fingerprint;
- an independently constructed attestation-requirement fingerprint;
- requirement level, assurance, accepted statuses, freshness, binding,
  truncation, network, and SideEffect posture; and
- a deterministic obligation identity.

The enclosing record sorts obligations by identity and derives its own
content-addressed fingerprint. Authored declaration order, inventory order,
and unrelated inventory contracts do not affect the result.

An empty declaration list produces an explicit authoritative empty record.
This preserves the future distinction between no declarations and a missing
record.

## 5. Integrity And Serde Assessment

Deserialization is fail closed:

- unknown fields are rejected;
- each independent attestation requirement is reconstructed and validated;
- its fingerprint is recomputed;
- each obligation identity is recomputed against enclosing context;
- ordering is canonicalized;
- duplicate identities are rejected; and
- the outer declaration-set fingerprint is recomputed and compared.

Focused tests reject both nested declaration tampering and outer fingerprint
tampering with a fixed error.

This integrity boundary is not an authenticity boundary. A caller that controls
an entire serialized record can recompute a self-consistent set of hashes.
The next publication phase must therefore publish only records produced from
trusted validated workflow and allowlisted inventory inputs through this
resolver. It must not treat arbitrary standalone serialized records as
authoritative merely because their internal hashes are consistent.

## 6. Privacy And Redaction Assessment

The canonical record stores bounded identities, enums, and fingerprints. It
does not store:

- executable text or arguments;
- working-directory or environment values;
- raw output or source content;
- parser or provider payloads;
- credentials; or
- evidence bodies.

`Debug` redacts workflow, step, bundle, requirement, command, and fingerprint
values. Resolver and validation errors use stable codes without echoing
caller-supplied identities. Serde failures use one fixed bounded message.

Serialization remains a structured governance record, not a secrecy boundary.
Its bounded identifiers should still receive the sensitivity treatment of the
future immutable bundle that contains them.

## 7. Determinism And Compatibility Assessment

The v1 hash domains are explicit and field framing is length-delimited. The
obligation identity includes every decision-relevant declaration field plus
the full canonical command-contract fingerprint. The set fingerprint binds the
workflow context, bundle version, algorithm, and canonically ordered obligation
identities.

Existing workflow behavior is unchanged. The API is additive, pure, and
in-memory. No executor or structural-coverage code consumes these records yet.

## 8. Test Quality Assessment

Focused tests cover:

- order-independent declarations and inventories;
- exclusion of unreferenced inventory contracts;
- command-contract and independent requirement fingerprints;
- authoritative empty records;
- duplicate inventory identities;
- unavailable commands;
- SideEffect maximum enforcement;
- exact-step selection;
- duplicate semantic obligations;
- command-policy and bundle-version invalidation;
- serde round trip;
- nested and outer tamper rejection;
- redaction-safe `Debug`; and
- exclusion of executable and payload-bearing fields.

The full workspace suite covers existing Workflow OS behavior. Live provider
tests remain explicitly opt-in and were not enabled.

## 9. Documentation Assessment

The roadmap, implementation plan, and phase report accurately state that the
record and resolver are implemented while immutable-bundle publication,
runtime enforcement, structural-coverage authority, execution, persistence,
providers, and writes remain unimplemented.

The documentation does not overclaim that a content-addressed record is
cryptographically signed, externally authenticated, or currently enforced.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Publish resolver-produced records into the immutable-run bundle through one
  explicit trusted construction path.
- Distinguish a missing declaration-set record from an authoritative empty
  record for historical and newly created bundles.
- Bind the record fingerprint into the enclosing immutable bundle identity.
- Reject publication when workflow, step, inventory, or bundle context differs
  from the inputs used during resolution.
- Keep standalone deserialized records non-authoritative until their enclosing
  bundle provenance is validated.
- Version the algorithm before expanding network or SideEffect taxonomy.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo test -p workflow-core --test local_check_declaration_set`: passed.
- `npm run check`: passed.
- `git diff --check`: passed.

## 13. Governed Review

- workflow: `dg/review`
- run: `run-1784996936900663000-2`
- approval:
  `approval/run-1784996936900663000-2/review-scope-approved`
- presentation: `presentation/4bd5da00f08dce86`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations
- presentation-proof close posture: bounded proof-store read error; the helper
  did not claim an available proof record or event marker
- kernel boundary: governance coordination only; code inspection, review
  authorship, and validation ran outside the kernel

## 14. Recommended Next Phase

Proceed to **immutable-run bundle declaration-set publication
implementation**.

That phase should resolve a canonical declaration set from validated workflow
and explicit allowlisted inventory inputs, publish the record into one
immutable bundle construction path, and bind its fingerprint into the bundle
identity. It must preserve authoritative-empty semantics and fail closed on
context mismatch.

It must not execute checks, add default handlers, infer repository commands,
convert structural coverage into aggregate posture, add executor gates, call
providers, authorize SideEffects or writes, expose schemas or CLI behavior, or
broaden hosted and release posture.
