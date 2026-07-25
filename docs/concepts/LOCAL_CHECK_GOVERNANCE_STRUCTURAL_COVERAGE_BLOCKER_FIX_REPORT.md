# Local Check Governance Structural Coverage Blocker Fix Report

## 1. Executive Summary

The private structural coverage candidate can no longer relabel a `DocsCheck`
leaf obligation across candidate bundle bindings. Candidate construction now
derives exact obligation identity from its own bundle and step binding plus the
exact requirement fingerprint.

## 2. Blocker Fixed

Previously, candidate input accepted an opaque obligation fingerprint together
with independently supplied bundle metadata. Because the leaf fingerprint was
already hashed, the candidate could not prove that its visible binding fields
matched the context encoded in that fingerprint.

The adapter checked membership, but a caller could place a bundle-A leaf
fingerprint inside a candidate labeled as bundle B and then adapt the leaf into
that relabeled set.

## 3. Implementation Approach

- Replaced caller-supplied obligation fingerprints with private obligation
  definitions containing exact requirement fingerprints and requirement
  levels.
- Extracted one shared domain-separated `DocsCheck` obligation-identity
  derivation function.
- Candidate construction derives each obligation fingerprint from candidate
  bundle ID, bundle version, bundle root, step ID, and requirement fingerprint.
- Candidate-set identity commits to both derived obligation and exact
  requirement identity.
- The adapter requires the runtime leaf fingerprint to match a derived
  candidate obligation before binding a structural contribution.
- Contributions remain bound to the exact candidate-set fingerprint during
  evaluation.

## 4. Binding Integrity

The same identity algorithm is now used at runtime leaf creation and candidate
construction. Changing bundle ID, bundle version, bundle root, step, or
requirement changes the derived obligation fingerprint before adaptation.

An opaque leaf fingerprint is no longer accepted as declaration input. A
relabeled candidate therefore derives a different expected obligation and the
adapter fails closed without producing a contribution.

## 5. Privacy And Scope

The fix adds no payload storage, public API, serde, canonical declaration
source, aggregate posture conversion, proportional-governance reassessment,
executor checkpoint, persistence, schema, CLI behavior, provider call,
SideEffect, or write.

Debug and errors remain bounded and do not expose bundle identities,
requirement fingerprints, obligation fingerprints, paths, output, credentials,
or provider payloads.

## 6. Test Coverage

Focused tests prove:

- a contribution already bound to candidate A cannot be evaluated against
  candidate B;
- a runtime leaf from bundle A cannot be adapted into a candidate relabeled as
  bundle B;
- candidate identity changes with binding or requirement substitution; and
- all existing structural coverage, runtime attestation, privacy, and
  non-authority tests remain green.

## 7. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784964027407008000-2`
- approval: `approval/run-1784964027407008000-2/fix-approved`
- presentation: `presentation/f8345807def2008e`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits, tests,
  documentation, and validation ran outside the kernel

## 8. Validation

- `cargo test -p workflow-core --lib local_check_attestation` - passed, 34
  tests;
- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed; and
- `git diff --check` - passed.

## 9. Remaining Limitations

- declaration provenance remains unresolved and non-authoritative;
- requirement level is not yet derived from a canonical frozen declaration;
- only the accepted private `DocsCheck` leaf is supported;
- no aggregate workload posture or reassessment exists; and
- no executor checkpoint consumes structural coverage.

## 10. Recommended Next Phase

Perform focused blocker-fix re-review. If accepted, plan canonical local-check
declaration fields and immutable-run-bundle derivation before any aggregate
conversion or executor integration.
