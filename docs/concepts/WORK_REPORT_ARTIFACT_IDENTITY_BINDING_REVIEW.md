# WorkReport Artifact Identity Binding Review

## Verdict

Accepted as the bounded implementation candidate for continuity-of-authority
Gate 2, **Verified Means Correctly Bound**, after two review blockers were
corrected.

This review did not close Gate 2. Exact-head CI with credential-backed
PostgreSQL execution and maintainer-operated post-merge exact-main regression
evidence now pass. An independent backend-misplacement rerun remains required.

## Blockers Found And Corrected

### Immutable identity was checked after body commitment

The first candidate used the default expectation-aware trait methods over each
built-in backend's legacy read/list method. Those legacy methods validated the
retained body commitment using only run/report location before the outer method
checked workflow version, schema version, and specification content hash.

A structurally valid artifact with wrong immutable workflow identity and a
corrupted body could therefore return `work_report_artifact.read.corrupt`
before `work_report_artifact.read.location_mismatch`. That contradicted the
documented identity-before-body invariant.

The correction adds expectation-aware overrides for filesystem, SQLite, and
PostgreSQL reads and lists. Each backend now decodes and structurally validates,
checks the complete supplied location and immutable workflow expectation, and
only then verifies the retained body commitment. PostgreSQL transaction-local
hosted replay now receives the candidate's complete immutable identity rather
than only run/report identity.

Focused filesystem and SQLite regressions combine wrong immutable workflow
identity with body corruption and prove that location mismatch wins without
identifier or payload leakage. The PostgreSQL CI scenario contains the same
adversarial substitution and restores the original record afterward.

### Hosted report projections bypassed immutable identity binding

The hosted and collaborative report metadata endpoints still called the legacy
run-only artifact methods and then projected metadata. They were externally
visible consumers omitted from the first enforcement-surface inventory.

The correction rehydrates the authoritative run, creates a complete immutable
identity expectation, and uses expectation-aware exact reads or listings before
returning report metadata. No report body, metadata projection, or terminal
report selection is now available through those endpoints before the artifact
is bound to the durable run identity.

## Reviewed Properties

1. Structure is validated before external identity without verifying the body
   commitment during deserialization.
2. Filesystem path, SQLite relational keys, PostgreSQL record keys, and
   transaction-local PostgreSQL keys bind run/report identity.
3. Available workflow ID, workflow version, schema version, and specification
   content hash are checked before body commitment.
4. Human and JSON inspect, first-run replay, authoritative recovery,
   current-authority projection, continuation-context collection, duplicate
   reconciliation, and hosted metadata projection use expectation-aware reads.
5. Wrong-location errors remain stable and payload-free.
6. Body-only corruption remains `work_report_artifact.read.corrupt`.
7. Legacy artifacts remain explicitly unverified for body commitment but do
   not receive an identity exception.
8. Exact duplicate, valid conflict, and unreadable/ambiguous reconciliation
   semantics remain unchanged.

## Validation Reviewed

- 243 focused WorkReport tests pass after the corrections;
- filesystem and SQLite combined immutable-identity/body-corruption regressions
  pass locally;
- the PostgreSQL adversarial regression compiles locally and remains scheduled
  for credential-backed exact-head CI execution;
- the complete Core all-features suite passes, including 784 Core unit tests,
  348 local-executor tests with one opt-in live test ignored, all 243
  WorkReport tests, integration targets, and doc tests;
- the complete CLI package suite passes, including unit, integration, and
  example targets;
- the 38-test hosted crate suite passes;
- Clippy and rustdoc pass with warnings denied; and
- formatter, documentation, release-state, and diff whitespace checks pass.

## Residual Boundary

- An injected trait implementation may fail internally before returning an
  artifact; Core validates every returned artifact but cannot reorder code that
  an external implementation executes before return.
- This is backend and consumer identity integrity, not storage-owner tamper
  resistance, signing, or external attestation.
- No legacy migration or backfill is provided.
- No schema, SDK, provider mutation, publication, or Gate 3 authority work is
  authorized by this review.
- Independent review remains required before release admission.

## Next Acceptance Work

PR #741 merged the unchanged reviewed head after all nine exact-head checks
passed. The credential-backed PostgreSQL job passed. A maintainer-operated
post-merge rerun on the exact merge commit confirmed filesystem and SQLite
misplacement rejection, identity-before-body ordering, CLI inspect/replay
rejection before projection or `already_persisted`, PostgreSQL test compilation,
and the complete hosted suite. See the [post-merge evidence
report](WORK_REPORT_ARTIFACT_IDENTITY_BINDING_POST_MERGE_EVIDENCE.md).

The remaining acceptance work is an independent filesystem/backend
misplacement rerun. Maintainer-operated evidence is not relabeled as
independent assurance. Gate 3 remains blocked until that final requirement
passes and Gate 2 is explicitly closed.
