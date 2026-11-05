# WorkReport Body Commitment Recovery And Reconciliation Report

## Summary

Workflow OS now validates retained WorkReport body commitments before existing
recovery and duplicate-reconciliation paths can return an already-persisted
artifact or accept exact equality.

The implementation adds one Core-owned reconciliation boundary rather than
assuming every injected `WorkReportArtifactStore` enforces the stronger durable
read contract introduced for the built-in backends.

## Shared Reconciliation Boundary

The shared helper validates:

1. the candidate artifact proposed for create-only persistence; and
2. the stored artifact returned after the create-only write reports a
   duplicate.

Only two validated artifacts with complete equality can produce an exact
duplicate result. A valid but unequal artifact remains a conflict. A missing,
unreadable, or validation-failing artifact remains unreadable or ambiguous
according to the existing caller posture.

The helper is used by all identified duplicate-reconciliation families:

- trusted-host terminal WorkReport artifacts;
- authoritative governance WorkReport artifacts;
- selected project-validation authority-receipt WorkReport artifacts; and
- governance authority-receipt WorkReport artifacts.

Each caller preserves its existing stable result vocabulary. The change is the
precondition for exact equality, not a new public status family.

## Existing-Artifact Recovery

The direct authoritative existing-artifact recovery helper now validates its
artifact before returning `AlreadyPersisted`. A body-commitment mismatch maps to
the existing payload-free authoritative duplicate-unreadable failure.

A valid artifact without `metadata.body_commitment` remains readable with
`legacy_unverified` posture. Recovery does not derive, add, or backfill a
commitment from mutable report content.

## Defense In Depth

Local filesystem, SQLite, and PostgreSQL store reads already enforce retained
commitments. The shared reconciliation check remains necessary because Core
accepts injected implementations of `WorkReportArtifactStore`, and a
security-sensitive equality consumer must not infer a stronger trait contract
from the built-in implementations.

This also keeps reconciliation safe when a test, host integration, or future
store implementation returns a structurally valid artifact without applying
the built-in read helper.

## Failure And Privacy Semantics

Commitment failure never reaches report-content equality. Existing callers map
it to their bounded unreadable, persistence-failed, or ambiguous posture.
Errors and debug output do not include:

- changed report summaries;
- expected or actual commitment values;
- report or run identifiers;
- storage locations; or
- serialized artifact payloads.

## Validation Evidence

Focused tests prove that:

- an injected store returning a structurally valid report-only substitution is
  rejected before duplicate equality;
- the invalid duplicate becomes the existing ambiguous/reconciliation-failed
  result and blocks retry where that caller already requires it;
- exact validated duplicates still reconcile successfully;
- valid unequal duplicates remain conflicts;
- direct existing-artifact recovery rejects commitment mismatch without
  exposing changed report text; and
- direct recovery preserves explicit legacy-unverified posture.

The phase also runs formatter, warnings-denied Clippy, the full affected core
suite, documentation checks, and diff whitespace validation.

## Security Claim

This phase proves that the identified recovery and duplicate-reconciliation
paths do not accept report-only substitution merely because an injected store
returned a structurally valid record. It does not protect against a storage
owner capable of replacing both body and retained commitment, and it does not
add a signature, witness, producer identity, or external attestation.

## Out Of Scope

This phase does not add or broaden:

- terminal or provider replay enforcement;
- migration or legacy backfill;
- CLI commands, schemas, or SDKs;
- signing or external witnesses;
- provider mutation or publication; or
- hosted or distributed trust claims.

## Next Phase

Apply the retained-commitment boundary to terminal and provider replay paths,
including transaction-local reads that do not use the ordinary artifact-store
read method.
