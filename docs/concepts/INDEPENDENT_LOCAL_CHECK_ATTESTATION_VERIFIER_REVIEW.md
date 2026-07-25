# Independent Local Check Attestation Verifier Review

## 1. Executive Verdict

Needs blocker fixes.

Fix-forward note: the blocker is corrected in
[Independent Local Check Attestation Verifier Blocker Fix Report](INDEPENDENT_LOCAL_CHECK_ATTESTATION_VERIFIER_BLOCKER_FIX_REPORT.md).
This note preserves the original review finding; focused re-review determines
whether the corrected phase may proceed.

The crate-private authority boundary, deterministic command/handler/result
matching, policy enforcement, full-precision freshness evaluation, accepted
record privacy, and non-leaking errors are sound. The verifier does not yet
consume the validated stored immutable bundle required by the accepted plan.

## 2. Scope Verification

The phase stayed within its pure-verifier scope. It did not execute checks,
invoke handlers, integrate the executor, persist records, emit events, attach
evidence, write artifacts, expose schemas or CLI behavior, call providers,
model new SideEffects, perform writes, or change release posture.

## 3. Authority Boundary Assessment

`KernelObservedLocalCheck`, its definition and constructor, the verification
input, and the verifier are crate-private. The accepted record has private
fields, read-only accessors, no public constructor, and no deserialization path.
Public callers therefore cannot directly manufacture observation authority or
accepted proof.

The `RegisteredUnattested` handler posture remains honest. Acceptance proves a
Core-observed local process under the exact pre-bound selection and policy; it
does not claim handler implementation, binary, host, or third-party provenance.

## 4. Deterministic Verification Assessment

The verifier deterministically matches requirement, candidate fingerprint,
workflow/run/step identity, command ID/kind and complete contract fingerprint,
handler-selection fingerprint, effective-policy fingerprint, invocation,
idempotency reference, result identity/status/exit/duration/truncation, and
observation timing.

It rejects caller/mock/external assurance, unaccepted statuses, forbidden
truncation, timeout excess, impossible time ordering, and stale observations.
The maximum-age check uses full duration precision. Focused tests caught and
fixed a one-millisecond boundary issue during implementation.

## 5. Stored Bundle Blocker

The accepted verifier plan requires a validated `StoredImmutableRunBundle` so
the verifier can establish manifest and canonical definition-record integrity.
The implementation instead accepts an `ImmutableRunBundleBinding` containing
only bundle ID, version, and root hash.

Equality among the candidate, execution binding, observation, and supplied root
reference is necessary but insufficient. It proves that inputs cite the same
root; it does not establish that the referenced stored manifest and all
canonical definition records are present, complete, and valid at verification
time.

This is a blocker because accepted check proof must not be issued over an
unresolved or incomplete immutable run bundle.

## 6. Accepted Record Assessment

The record is payload-free and preserves requirement, candidate, execution
binding, bundle, handler, result, status, exit, completion, verification,
freshness, and truncation context. Canonical proof identity excludes
caller-selected attestation record ID. Debug redacts identities, fingerprints,
and timestamps.

No serialization was added, which is the conservative first-phase choice.

## 7. Privacy And Error Assessment

Observation and accepted records contain no raw stdout, stderr, arguments,
paths, environment values, source contents, credentials, tokens, provider
payloads, or free-form claims. Stable error families do not echo supplied
values. Debug output is bounded and redacted.

## 8. Test Quality Assessment

Focused tests cover exact acceptance and a stable proof vector, command and
observation substitution, bundle-reference mismatch, assurance downgrade,
invalid evaluation time, result duration mismatch, unaccepted status,
truncation policy, exact freshness boundary, one-millisecond expiry, and Debug
non-leakage.

Missing blocker coverage: acceptance from a complete validated
`StoredImmutableRunBundle`, and rejection when stored manifest/record integrity
cannot be established.

The full workspace validation suite passed.

## 9. Blockers

1. Replace the verification input's bare `ImmutableRunBundleBinding` with a
   validated `StoredImmutableRunBundle`.
2. Derive the trusted binding from `stored_bundle.manifest().run_binding()` and
   compare candidate, execution-binding, and observation context to it.
3. Add focused coverage proving a complete stored bundle is accepted and an
   unresolved/mismatched stored bundle cannot produce accepted proof.

## 10. Non-Blocking Follow-Ups

- Future runtime composition must create the pre-execution binding before
  execution and derive the observation only from Core-owned process results.
- Freshness must be reevaluated by later time-of-use consumers.
- Stronger handler implementation provenance remains a separate assurance tier.
- The dogfood approval-presentation list-cap defect remains open.

## 11. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784519101054547000-2`
- approval: `approval/run-1784519101054547000-2/review-scope-approved`
- presentation: `presentation/1450380b5ee5c9f4`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; review inspection,
  documentation edits, and validation ran outside the kernel

## 13. Recommended Next Phase

Run a focused blocker fix that changes only the verifier's immutable-bundle
input and tests to require `StoredImmutableRunBundle`. Do not begin DocsCheck
runtime composition until focused re-review accepts that correction.
