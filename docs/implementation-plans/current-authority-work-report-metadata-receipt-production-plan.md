# Current-Authority WorkReport Metadata Receipt Production Plan

## 1. Executive Summary

Implement one internal, opt-in, Core-owned producer for the local unsigned
`AuthorityReceipt`. The producer must be inseparable from the already accepted
exact WorkReport artifact bounded-metadata read and may issue a trusted receipt
only after that read succeeds.

This phase does not create a generic receipt factory, public receipt producer,
executor integration, provider integration, OpenShell integration, persistence,
events, SideEffect execution, or writes.

## 2. Why This Operation

The exact WorkReport metadata read is the smallest accepted operation that
already proves:

- immutable workflow, run, step, actor, and harness identity;
- one exact required-context target;
- one fresh registered current-authority assessment;
- one selected active grant;
- one exact successful artifact-store read; and
- bounded metadata and sensitivity validation.

Binding receipt production to that operation demonstrates useful provenance
without widening authority or execution scope.

## 3. Approved Scope

- Add an exact operation kind and payload-free operation-outcome commitment to
  the receipt commitment.
- Add an internal, non-cloneable successful-read proof.
- Construct that proof only after the exact artifact exists, validates, matches
  report/run identity, and satisfies sensitivity ceilings.
- Add a separate opt-in receipt-bearing metadata-read method.
- Preserve the existing non-receipt metadata-read method.
- Issue no receipt for any non-success outcome.
- Add focused deterministic, privacy, and non-regression tests.

## 4. Strict Non-Goals

- No generic or caller-populated trusted receipt constructor.
- No trust restoration from serialized receipt claims.
- No authorization reuse from a receipt.
- No public API or executor wiring.
- No report-body access.
- No artifact writes, receipt persistence, event append, or CLI behavior.
- No provider, OpenShell, sandbox, credential, or network behavior.
- No SideEffect execution or write-capable adapter behavior.
- No schemas, SDKs, examples, hosted behavior, or release changes.

## 5. Trust Boundary

The registered source owns a private successful-operation proof. Its fields are
not caller-settable and it is consumed by the receipt constructor. The
constructor is crate-private and accepts only that proof.

The receipt remains:

- `FreshAtIssuance`;
- `PointInTimeOnly`;
- `LocalUnsigned`;
- `EvidenceOnlyNotAuthorization`; and
- `ReferenceOnly`.

Serialized receipts deserialize only as `UnverifiedAuthorityReceipt`, which has
no conversion to the trusted type.

## 6. Success And Failure Semantics

`Found` is the only issuance outcome. `NotFound`, blocked authority, source
failure, store failure, stale authority, changed immutable identity, invalid
artifact identity, excessive sensitivity, ambiguity, and internal
inconsistency issue no receipt.

Each opt-in call independently resolves current authority and reads the exact
artifact once. Receipts are not cached or reused.

## 7. Privacy And Redaction

The operation stores no report body, source payload, path, command output,
provider payload, credential, token, or raw log. Resource scope and operation
outcome are committed rather than copied. Debug output redacts identities,
grant IDs, commitments, timestamps, and report metadata identifiers.

## 8. Test Plan

- Successful exact read produces one valid trusted receipt.
- Receipt binds immutable execution, exact requirement, selected grant, source
  commitments, operation kind, and outcome commitment.
- Serialized output loads only as an unverified claim.
- Debug output does not expose identities or payload-like values.
- Not-found, blocked, stale, source-failure, and store-failure paths issue no
  receipt.
- Repeated calls re-resolve and produce point-in-time-distinct receipts.
- The existing non-receipt read remains unchanged.
- Store writes and list operations remain zero.

## 9. Validation

- focused authority-receipt tests;
- focused registered current-authority source tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 10. Final Recommendation

Implement and review this single producer. After acceptance, return to the
active proportional-governance and quiet-success lane. Do not broaden receipt
production into an executor, provider, sandbox, or mutation path.
