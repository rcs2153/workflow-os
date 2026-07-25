# Independent Local Check Attestation Stored Manifest Identity Blocker Fix Report

## 1. Executive Summary

The pure local-check attestation verifier now derives workflow and run identity
from the validated stored immutable-run manifest and requires the immutable
execution binding to match both values.

This closes a consistent-relabelling gap discovered before the first runtime
composition helper was implemented.

## 2. Blocker Fixed

The verifier already required the execution binding, candidate, and Core-owned
observation to cite the stored bundle root and agree on workflow/run identity.
It did not compare those identities directly to the stored manifest.

A caller could therefore construct an execution binding over the correct root
with different workflow/run IDs, relabel candidate and observation identically,
and pass the agreement checks.

## 3. Implementation

The verifier now:

1. obtains the validated stored manifest;
2. derives its immutable run binding;
3. preserves existing exact root checks; and
4. requires execution-binding workflow and run identity to equal the manifest
   workflow and run identity.

Mismatch returns the existing stable, non-leaking
`local_check_attestation.verify.bundle_mismatch` error. No partial accepted
record is returned.

## 4. Test Coverage

A new regression constructs an otherwise valid execution binding with a
different workflow and run ID, then consistently relabels the candidate and
observation. Verification fails because the stored manifest remains the source
of truth.

Existing exact-context, command, observation, bundle, assurance, status,
freshness, truncation, stable-vector, and non-leakage tests remain unchanged.

## 5. Scope Preservation

The fix does not add check execution, runtime composition, default
registration, executor changes, persistence, events, evidence, reports,
artifacts, schemas, CLI, providers, SideEffects, writes, hosted behavior, or
release changes.

## 6. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784586836933053000-2`
- approval: `approval/run-1784586836933053000-2/fix-approved`
- presentation: `presentation/d8d5b8e3b202d275`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits, tests,
  documentation, and validation ran outside the kernel

## 7. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- governed phase close - completed with 39 events, one proof-backed approval,
  zero retries, and zero escalations.

The phase-close helper reported the known 250-record presentation-read cap;
the exact proof was persisted and enforced before the fix began.

## 8. Remaining Limitations

- the accepted runtime-composition helper remains unimplemented;
- stronger handler implementation provenance remains future work;
- accepted proof is not persisted or projected into events/evidence/reports;
- later consumers must reevaluate freshness; and
- the dogfood phase-close presentation-record cap defect remains open.

## 9. Recommended Next Phase

Perform a focused maintainer review of this verifier fix. Resume the accepted
`DocsCheck` runtime-composition implementation only after review acceptance.
