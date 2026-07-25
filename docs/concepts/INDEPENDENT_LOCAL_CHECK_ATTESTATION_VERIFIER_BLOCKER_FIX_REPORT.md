# Independent Local Check Attestation Verifier Blocker Fix Report

## 1. Executive Summary

The pure verifier now requires a validated `StoredImmutableRunBundle` instead
of accepting a bare immutable-bundle root binding. It derives the trusted run
binding from the stored manifest and compares the pre-execution binding,
candidate, and Core-owned observation against that source of truth.

The fix remains model-only and pure. It does not execute checks, integrate the
executor, persist accepted attestations, append events, create evidence or
reports, expose schemas or CLI behavior, call providers, or enable writes.

## 2. Blocker Fixed

The original verifier could prove that all supplied inputs cited the same
bundle root, but it could not establish that the root belonged to a complete,
validated stored manifest and canonical definition-record set. That was
insufficient for accepted independent check proof.

The verification input now accepts only `StoredImmutableRunBundle`, a type
returned by the local immutable-bundle store after manifest identity, referenced
record resolution, canonical-record integrity, and bundle completeness checks.

## 3. Implementation Approach

The verifier:

1. obtains the trusted binding from
   `stored_bundle.manifest().run_binding()`;
2. requires the immutable local-check execution binding to match it;
3. requires the candidate attestation binding to match it;
4. requires the Core-owned observation to match it; and
5. records that derived binding in the accepted proof.

No public constructor or deserialization path was added for
`StoredImmutableRunBundle`. Callers cannot substitute a root-only reference at
this boundary.

## 4. Validation Boundary

The store remains responsible for validating the complete stored bundle. The
pure verifier consumes the validated result and does not reread files or repeat
store validation. This keeps storage integrity and proof verification as
separate, explicit ownership boundaries.

Any mismatch returns the stable, non-leaking
`local_check_attestation.verify.bundle_mismatch` error. No partial accepted
record is returned.

## 5. Test Coverage

Focused tests now prove:

- a complete store-written and store-read bundle produces accepted proof;
- the accepted record retains the manifest-derived run binding;
- canonical stored definition records are present in the accepted fixture;
- a different independently valid stored bundle fails closed;
- a mismatched observation bundle still fails closed; and
- the stable accepted-proof vector reflects the canonical stored root.

Existing mismatch, assurance, result, freshness, truncation, and Debug
non-leakage tests remain intact.

## 6. Privacy And Redaction

The fix adds no raw command output, paths, arguments, environment values, source
contents, credentials, provider payloads, or free-form claims to verifier input,
accepted proof, Debug output, or errors. Stored bundle records remain validated
definition metadata rather than execution payloads.

## 7. Validation Commands

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

All five commands passed. The workspace suite completed with no failures; one
explicit opt-in live sandbox test remained ignored by design.

## 8. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784519369342221000-2`
- approval: `approval/run-1784519369342221000-2/fix-approved`
- presentation: `presentation/cc978ff3c4f3d73d`
- approval outcome: granted by the delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits and validation run
  outside the kernel

## 9. Remaining Limitations

- no executor or handler composition;
- no attestation persistence, events, audit projection, evidence, or reports;
- no default or automatic check execution;
- no stronger handler implementation provenance beyond registered-unattested;
- no schema, SDK, CLI, provider, hosted, SideEffect, or write behavior; and
- later consumers must reevaluate freshness at time of use.

## 10. Recommended Next Phase

Perform a focused maintainer review of this blocker fix. Only if accepted should
the roadmap plan one explicit opt-in `DocsCheck` runtime composition path.
