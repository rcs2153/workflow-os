# WorkReport Body Commitment Persistence Anchor Review

## Decision

Accepted for the bounded persistence-anchor boundary.

The implementation retains the expected V1 commitment outside mutable report
content without presenting a local hash as independent authorship or
storage-owner tamper resistance. It is suitable as the input to separately
reviewed read-path enforcement.

This is an owner-conducted focused maintainer/security review. It is not the
independent security review still required before release admission resumes.

## Reviewed Properties

The review confirmed:

1. New artifact construction computes V1 from the complete validated report.
2. The expected value is retained in artifact metadata, not inside the report
   body.
3. Local filesystem, SQLite, and PostgreSQL persistence write envelope and body
   in one create-only atomic operation.
4. No second-write crash-reconciliation gap was introduced.
5. Missing commitment metadata deserializes as `legacy_unverified`; Core does
   not derive an expected value from mutable legacy content.
6. Public artifact validation and supported writes compare the body with the
   retained anchor.
7. Report-only substitution fails with a stable payload-free mismatch error.
8. Debug output reports only bounded posture and does not reveal the digest.
9. Ordinary read/list enforcement remains deferred and is named as such.
10. No migration, CLI, schema, SDK, signing, witness, provider mutation, or
    publication behavior was added.

## Review Boundary Correction

The tempting implementation was to make artifact deserialization verify the
new commitment immediately. Review rejected that expansion because every
supported read and list path deserializes the artifact. Doing so would have
combined persistence anchoring with broad read-time enforcement contrary to
the approved phase scope.

The accepted implementation separates structural deserialization from public
artifact validation. Writes invoke full validation; ordinary reads retain the
anchor and posture but do not yet claim verification. The next phase can add
explicit read/list enforcement with stable backend error mapping and legacy
semantics rather than inheriting accidental serde behavior.

## Residual Risks

Until the next phase, direct ordinary reads can return a structurally valid
report whose body no longer matches the retained envelope anchor. Callers must
not call that report verified merely because the anchor exists.

A storage owner who can rewrite both metadata and report content remains inside
the local trust boundary even after read enforcement. Independent assurance
requires a separately controlled signing key, external append-only witness, or
equivalent trust anchor in a later phase.

Legacy artifacts remain unverified. No migration policy exists yet.

## Validation Reviewed

- New artifacts retained V1 and reported `persisted_envelope_anchor`.
- Serialization and local-store round trips preserved the exact anchor.
- Missing commitment metadata produced `legacy_unverified` without backfill.
- A report-only prose edit remained structurally readable at this phase but
  failed explicit validation with no payload or digest leakage.
- Existing duplicate, writer-guard, runtime-state non-mutation, and migration
  tests remained green.

## Next Phase

Proceed to bounded read/list enforcement. Compare new-format report bodies with
their retained expected commitments before returning content, preserve explicit
legacy-unverified behavior, and map failures to stable non-leaking backend
errors. Do not combine that work with inspect, recovery, replay, product
projection, migration, signing, external witnesses, provider mutation, or
publication.
