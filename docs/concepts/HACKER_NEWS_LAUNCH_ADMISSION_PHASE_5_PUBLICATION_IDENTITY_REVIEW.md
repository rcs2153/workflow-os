# Hacker News Launch Admission Phase 5 Publication Identity Review

Review date: 2026-11-03

Review target: [Phase 5 Publication Identity
Decision](HACKER_NEWS_LAUNCH_ADMISSION_PHASE_5_PUBLICATION_IDENTITY_DECISION.md)

## Decision

**Accepted with no P0 or P1 blocker.**

The publication denial is correct. The existing `v0.3.0-preview.1` tag is an
immutable public release identity, the accepted launch-admission candidate is a
different commit, and the repository cannot currently produce a non-provisional
native archive. Publishing or retagging would violate the release process and
the kernel's exact-commit governance posture.

`0.4.0-preview.1` is accepted as the next **release-preparation identity**. It
is not accepted as a release, tag, artifact, registry publication, or
announcement. Those actions require implementation evidence, exact-head CI,
focused review, and a new governed publication decision.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793755764886367000-2`
- Approval: `approval/run-1793755764886367000-2/review-scope-approved`
- Presentation: `presentation/0093e68e4a5dc33d`
- Approval outcome: granted under delegated maintainer authority after the
  complete review handoff was presented
- Approved scope: exact tag and release facts, semantic-version selection,
  historical-document corrections, provisional packaging blockers, roadmap
  authority, and documentation consistency
- Phase status: completed
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof
- Out-of-kernel work: read-only Git inspection, package-script inspection,
  documentation review and edits, and deterministic documentation validation

## Evidence Reviewed

- Annotated tag object: `5244167e5a92d40291b38d0509ac203ec0e13e17`
- Tag target: `48403d6718eb3ee23dc6365f53541cf492508b68`
- Accepted Phase 4 commit:
  `aff3e991acf8dcd25c569776400f15fd31f71cfb`
- Commit count from the tag target through Phase 4: 39
- Existing GitHub release posture: published prerelease with no native archive
  assets
- Semantic-version policy: additive public capabilities are minor-version work
- Native archive contract: provisional and explicitly not publishable
- Package script: final mode fails closed unless provisional rehearsal is
  selected

## Findings

### No Blocker: Existing Release Identity Is Preserved

The decision refuses tag movement, replacement, and artifact attachment from a
later commit. That is the only posture consistent with immutable release
identity and exact-commit evidence.

### No Blocker: Minor Preview Is The Honest Version Boundary

The post-tag work adds material preview capability rather than only fixes. A
new minor preview is consistent with the repository policy and communicates a
new evaluation boundary without implying v1 stability.

### Required Next-Lane Constraint: Version Selection Is Not Publication

The next lane must treat `0.4.0-preview.1` as a proposed implementation target.
It must update every version-bearing surface and release record coherently,
then prove them. It cannot infer publication authority from this review.

### Required Next-Lane Constraint: Final Packaging Must Be A New Reviewed Path

The current packager's rejection of final mode is a safety property, not an
inconvenience to bypass. The next lane must define and test final-candidate
admission explicitly. It must not weaken provisional safeguards or promote a
dirty rehearsal archive.

### P2 Assurance Limitation: Review Is Owner-Conducted

This focused review is governed and evidence-backed but not independent
external security assurance. That distinction must remain visible in the new
release notes and publication decision.

## Validation

- `npm run check:docs`: passed
- `git diff --check`: passed
- immutable tag and exact commit facts: verified locally
- provisional packaging guard: verified by source inspection
- packaging execution: not run because final packaging is not authorized
- runtime tests: not rerun because this phase changes documentation only
- tag, artifact, registry, release, and announcement actions: not performed

## Accepted Next Action

Create one separately governed `0.4.0-preview.1` release-identity and
final-packaging implementation plan. The plan must enumerate every
version-bearing surface, preserve historical `0.3.0-preview.1` records, define
the final-candidate archive admission contract, and end before publication.
