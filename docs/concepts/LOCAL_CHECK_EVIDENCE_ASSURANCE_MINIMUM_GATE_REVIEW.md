# Local Check Evidence Assurance Minimum Gate Review

## Decision

Accepted after two bounded corrections.

The implementation binds an optional evidence-assurance minimum into the
content-addressed local-check declaration set and exact immutable run bundle,
then evaluates the Core-owned durable projection before workflow events or
skill execution. It preserves prior behavior when the minimum is absent and
does not allow a runtime caller to weaken the reviewed declaration.

## Corrections

The first complete CLI validation exposed a missing public label for
`registered_content_bound`. The CLI projection now covers both producer
postures and has direct exhaustiveness coverage.

The initial producer comparison relied on derived Rust enum ordering. That
would have made future declaration order capable of changing authorization
semantics. Producer comparison now uses an explicit closed table, parallel to
the existing attestation comparison. The producer enum no longer derives
ordering traits, so later code cannot silently reintroduce ordinal policy.

## Reviewed Properties

The review confirmed:

1. The minimum is recovered from the exact canonical declaration stored in the
   immutable run bundle, matched by step and requirement fingerprint.
2. The minimum participates in declaration-set hashing only when present, so
   existing declaration fingerprints remain compatible while new minimums are
   identity-bound.
3. The gate evaluates the create-only durable projection rather than a
   caller-authored runtime minimum.
4. A missing minimum preserves prior behavior.
5. The current operator-selected, trusted-host-observed, unattested posture
   satisfies only its exact weak minimum.
6. Registered-producer and kernel-accepted-attestation requirements fail closed
   with stable payload-free errors.
7. Denial occurs after the bounded local check and durable classification, but
   before workflow events or skill execution. This preserves the evidence being
   classified without permitting downstream work.
8. Reassessment cannot replace the first create-only projection with a changed
   posture. Subject or posture disagreement fails closed.
9. Schema, Rust, and TypeScript vocabulary remain synchronized.
10. CLI human and JSON inspection can render every current producer posture
    without falling through or inventing stronger assurance.

## Evidence Boundary

This gate enforces declared minimums over a Core-owned durable projection. It
does not prove that the trusted host, operator-selected executable, or local
storage owner is independent or uncompromised. Projection fingerprints detect
ordinary serialized tampering against their bound fields, but they are not
signatures or external witness receipts.

The stronger `registered_content_bound` and
`kernel_accepted_local_process` values are closed requirement vocabulary. The
current route cannot establish them. No producer registry, authentication,
signing, external witness, or automatic assurance upgrade was introduced.

## Residual Risks

- A privileged storage owner able to replace all local records and their
  anchors remains inside the local trust boundary.
- The trusted host can execute an operator-selected validator that behaves
  differently from what a reviewer expects; content binding identifies bytes,
  not semantic trustworthiness.
- External verification remains unavailable.
- Independent adversarial review has not yet tested downgrade, replay,
  substitution, or malformed-declaration attacks across the complete public
  surface.

These limits are disclosed and do not invalidate the narrow gate. They prevent
using this slice as a claim of independent evidence or production-grade
attestation.

## Validation Reviewed

- Complete Core all-feature tests passed, including the minimum comparison,
  immutable declaration, executor denial-ordering, replay, and durable-store
  suites.
- Complete CLI tests passed after the registered-producer label correction.
- Warnings-denied all-target/all-feature Core and CLI Clippy passed.
- Formatting, schema/SDK contracts, TypeScript typecheck/lint/tests,
  documentation validation, and diff checks passed.
- Live credential-backed and external provider tests were not run because this
  slice adds no live transport or provider mutation behavior.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793899710591461000-2`
- Approval: `approval/run-1793899710591461000-2/review-scope-approved`
- Presentation: `presentation/a07c43115e8b0db1`
- Presentation hash:
  `a07c43115e8b0db10bc682251cf414089f881e46e96e9c91c7763d482509928a`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Review phase status: completed with 39 events, one approval, zero retries,
  and zero escalations; approval-presentation proof enforcement was present

Repository edits, validation, and review analysis were performed outside the
kernel under the approved scope. The kernel coordinated the phase and durable
approval proof; it did not execute the review or manufacture evidence.

## Next Phase

Proceed to independent adversarial assurance review. Do not add a producer
registry, arbitrary executable discovery, provider mutation, hosted behavior,
signatures, external witnesses, or broader authorization use before that
review reports its findings.
