# Trusted-Host Build-Input Discovery Core Model Blocker Fix Report

## 1. Executive Summary

The two blockers from the Core model review are fixed. Version-one canonical
field framing is architecture independent, and the new discovery request and
result wire boundary now fails closed on unknown top-level and nested fields
without leaking caller-supplied field names or values.

The model remains non-authorizing and model-only. This phase does not discover
repository inputs, read files, run processes, select providers, persist state,
or grant authority.

## 2. Blockers Fixed

1. Replaced machine-width `usize` canonical length prefixes with fixed-width
   `u64` big-endian prefixes.
2. Replaced permissive derived deserialization at the discovery boundary with
   private strict wire representations that reject unknown fields.

## 3. Canonicalization Repair

`hash_field` now converts both label and value lengths to `u64` and writes
eight-byte big-endian prefixes. The conversion follows the repository's
existing checked canonical-framing pattern and cannot vary between supported
32-bit and 64-bit hosts.

The version-one request fingerprint and result graph commitment are both
pinned by fixed-vector tests. The result vector is unchanged on the current
64-bit host because the prior machine-width encoding happened to be eight
bytes there; the request vector is newly pinned so the transitive request
binding is independently reviewable.

## 4. Strict Wire Boundary

Custom deserializers use private wire representations and payload-discarding
unknown-field maps. Unknown fields fail closed at:

- the request definition;
- the result definition;
- host-obligation entries;
- reused input-kind count entries; and
- reused observation-reference entries.

The private wrappers preserve the existing public model and serialization
shape. They avoid broad compatibility changes to reused Core types.

## 5. Validation And Error Boundary

Valid request and result serde still re-enters the established constructors and
validation rules. Nested counts and references are reconstructed through their
existing validated constructors.

Unknown-field errors are stable, generic, and independent of the rejected
field name or value. Existing non-leaking enum and validation behavior is
unchanged.

## 6. Privacy And Redaction

The fix stores no new payloads. Unknown values are ignored during strict wire
decoding and are never copied into the model or error text. Debug remains
redaction safe, and derived commitments remain omitted from serialization and
recomputed on deserialization.

## 7. Test Coverage

Focused tests now cover:

- a fixed request-fingerprint vector;
- the fixed result-commitment vector;
- unknown request-field rejection;
- unknown result-field rejection;
- unknown host-obligation-field rejection;
- unknown input-count-field rejection;
- unknown observation-reference-field rejection;
- non-leakage of every injected field name and value; and
- unchanged valid request/result serde round trips.

The focused discovery model suite passes 22 tests.

## 8. Scope Explicitly Not Completed

This phase did not add fixture parsing, workspace discovery, filesystem reads,
TOML parsing, Cargo invocation, build-code execution, host observation,
provider calls, handler registration, persistence, runtime events, reports,
artifacts, schemas, CLI behavior, SDK changes, writes, capability grants, or
authority expansion.

## 9. Validation

Completed before finalization:

- focused trusted-host build-input discovery model tests: passed, 22 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed on the complete rerun. The first complete
  run reported two unrelated concurrent scheduler-test failures; each exact
  test then passed independently before the complete workspace rerun passed.
- `npm run check`: passed under the repository's Node 20 toolchain.
- `npm run check:integrations`: passed under the repository's Node 20
  toolchain.
- `npm run check:docs`: passed as part of `npm run check`.
- `git diff --check`: passed.

## 10. Remaining Limitations

- The model still contains caller-constructed claims, not observed truth.
- No parser or filesystem producer exists.
- Static completeness does not prove live build-input completeness.
- All nine host obligations remain unavailable or unsupported.
- No provider or executor consumes this model.

## 11. Recommended Next Phase

Perform a focused maintainer and security re-review of this blocker fix. Do not
begin fixture parser, workspace discovery, host observation, provider
integration, persistence, or execution work before acceptance.

## 12. Governed Fix Evidence

- Workflow ID: `dg/blocker`.
- Run ID: `run-1792824164494414000-2`.
- Approval ID:
  `approval/run-1792824164494414000-2/fix-approved`.
- Approval-presentation ID: `presentation/ebe43fdd4772d113`.
- Approval outcome: granted with persisted presentation proof under standing
  delegated-maintainer authority for the exact blocker-fix scope.
- Phase status: `Completed`.
- Event summary: 39 events, one approval, zero retries, and zero escalations.
- Approval-presentation enforcement: proof enforced; the approval event trail
  contains the persisted presentation marker.
- Out-of-kernel work: the delegated maintainer edited source, tests, and docs
  and ran validation. The kernel governed scope and approval only.

## 13. Compatibility Posture

The public request, result, obligation, count, and reference shapes are
unchanged. The intentional compatibility tightening is that additional unknown
fields at this exact evidence-binding boundary are no longer silently ignored.
