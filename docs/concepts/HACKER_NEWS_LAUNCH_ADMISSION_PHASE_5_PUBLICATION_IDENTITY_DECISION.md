# Hacker News Launch Admission Phase 5 Publication Identity Decision

Status: **publication blocked; new release identity required**

Decision date: 2026-11-03

## Governed Phase

- Workflow: `dg/release`
- Run: `run-1793755468725174000-2`
- Approval: `approval/run-1793755468725174000-2/release-approved`
- Presentation: `presentation/e6c3513cce256b7d`
- Approval outcome: granted under delegated maintainer authority after the
  complete release handoff was presented
- Approved scope: verify merged CI, produce exact-commit native artifacts and
  checksums, test install and uninstall, and prepare the publication-decision
  record and pull request
- Scope correction: exact-commit artifact production stopped before writing an
  artifact because the immutable public release identity and final-packaging
  preconditions were not satisfied
- Phase status: completed with publication denied
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof
- Out-of-kernel work: read-only Git and GitHub release inspection, semantic
  version analysis, documentation changes, and deterministic documentation
  validation

## Decision

Workflow OS must not publish the current launch-admission candidate as
`0.3.0-preview.1` and must not move, replace, or recreate the existing
`v0.3.0-preview.1` tag.

The next release-preparation lane should use `0.4.0-preview.1`, subject to a
separately governed version-change review and implementation phase. This Phase
5 decision does not change repository versions, create a tag, upload an
artifact, publish a release, or authorize a Hacker News submission.

## Authoritative Facts

- GitHub already contains a published, non-draft prerelease at
  `v0.3.0-preview.1`.
- The annotated tag resolves to commit
  `48403d6718eb3ee23dc6365f53541cf492508b68`.
- The release was published on 2026-11-01 and has no uploaded native archive
  assets.
- The accepted Hacker News launch-admission Phase 4 commit is
  `aff3e991acf8dcd25c569776400f15fd31f71cfb`.
- Thirty-nine commits separate the published tag from the accepted Phase 4
  commit.
- Those commits include additive public preview capabilities for authoritative
  continuation context, external-action authority projection, provider-fixture
  lifecycle execution, portable onboarding guidance, provider-aware
  `next-action`, and the launch-admission golden path.
- The repository's semantic-versioning policy classifies additive public
  capabilities and experimental contract evolution as minor-version work.

Timestamps are not used to establish release identity. The immutable tag,
exact commits, repository version policy, and accepted release evidence are the
decision inputs.

## Why `0.3.0-preview.2` Is Not Selected

The post-tag changes are not only corrective packaging or backward-compatible
bug fixes. They add material preview capabilities and public behavior. A patch
prerelease would understate the scope and obscure the boundary between the
already published governed-execution preview and the newly accepted
continuation, external-action, and launch-admission surfaces.

`0.4.0-preview.1` creates an honest immutable boundary without implying
production readiness or stable v1 contracts.

## Why The Existing Tag Must Not Move

Moving a published tag would make existing clones, release links, checksums,
and third-party references disagree about what `v0.3.0-preview.1` means. It
would also destroy the exact-commit evidence boundary that Workflow OS expects
from governed external actions.

The existing release remains a historical public preview. Its lack of native
assets is not repaired by rewriting history. Any native artifacts must be
built for and attached only to a new immutable release identity.

## Additional Packaging Blocker

The native archive contract and `scripts/package-release.mjs` still authorize
provisional rehearsal output only. The packager deliberately requires
`--provisional` and rejects final publication posture. A new release identity
alone is therefore insufficient.

Before publication, a separately reviewed lane must:

1. align every Rust crate and npm package on `0.4.0-preview.1`;
2. update the changelog, release notes, checklist, and release draft without
   rewriting the historical `0.3.0-preview.1` record;
3. promote the native archive contract from provisional rehearsal to a bounded
   final-candidate contract;
4. add a fail-closed final packaging path that proves a clean checkout, exact
   source commit, matching binary/package versions, native target, deterministic
   archive, checksum, install, overwrite refusal, uninstall, and state
   preservation;
5. rebuild every final artifact from the exact reviewed release commit on its
   native target;
6. run focused release/security review and terminal-green exact-head CI; and
7. require a new governed publication decision after all evidence exists.

## Current Phase Outcome

Phase 5 has made a publication decision: **deny publication under the current
identity and evidence set**.

This denial is not a failure of the accepted Phase 4 product evidence. It is a
release-integrity correction discovered before an irreversible external
action. The lawful next action is a bounded `0.4.0-preview.1` release-identity
and final-packaging plan, not a tag push or artifact upload.

## Validation

- `npm run check:docs`: passed
- `git diff --check`: passed
- no native archive, checksum, tag, release asset, registry package, or
  announcement was created

## Non-Goals

This decision does not authorize:

- changing package or crate versions;
- changing runtime behavior;
- moving or deleting any existing tag or release;
- uploading assets to the existing release;
- publishing crates or npm packages;
- expanding supported platforms;
- code signing, notarization, package-manager distribution, or containers;
- a production-readiness or v1 claim; or
- a Hacker News submission.
