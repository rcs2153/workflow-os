# Release Process

Workflow OS is not production-ready yet. This process defines the release shape the project will use as it matures.

## Release Requirements

Before any release:

- CI must pass.
- Changelog entries must be current.
- Public contracts must be documented.
- Breaking changes must reference migration notes.
- Security-sensitive changes must be reviewed.
- Known limitations must be reviewed and linked from release notes.
- Version numbers must follow [SEMVER.md](SEMVER.md).

## Release Steps

1. Confirm the release scope.
2. Confirm CI is passing on the release commit.
3. Update `CHANGELOG.md`.
4. Update package and crate versions consistently.
5. Tag the release.
6. Publish artifacts only after maintainers approve.
7. Announce known limitations and experimental features.

For the v0 readiness baseline, see [V0_READINESS.md](V0_READINESS.md) and [V0_KNOWN_LIMITATIONS.md](V0_KNOWN_LIMITATIONS.md).

## Public Preview Releases

The first public v0 posture is **Workflow OS v0 local kernel preview**.

The first local-kernel-preview version is `0.1.0-preview.1`, applied consistently to Rust crates and TypeScript packages. Future preview versions must keep Rust and TypeScript package versions aligned unless a release note explicitly documents why they differ.

A local-kernel-preview release is acceptable when:

- the release is clearly labeled as a local kernel preview
- CI and local quality gates pass
- README, changelog, release notes, and known limitations are aligned
- the vertical-slice example validates and runs locally
- schemas, SDK-generated specs, and checked-in examples pass Rust validation
- security review and dependency audits are current
- limitations are prominent, especially local-only state, no production DB, no distributed workers, no unsupported adapters or write-capable adapters, no UI, no marketplace, no active timeout scheduler, no trigger ingestion service, no Level 3/4 enablement, trusted local mock handlers only, manual schema/TypeScript synchronization, and the deprecated YAML parser risk

A local-kernel-preview release must be blocked when:

- docs imply production deployment readiness
- docs imply production distributed runtime support
- docs imply unsupported GitHub/Jira write behavior, CI, SaaS, generic HTTP, or other external adapters exist
- docs imply Level 3/4 autonomy is enabled by default
- validation, CLI, SDK contract, example, docs, or dependency checks fail
- known security or privacy limitations are hidden or softened
- mock-only behavior is presented as production behavior

## Future Public Release Candidates

A future public release candidate is a stronger posture than the v0 local kernel preview. It requires a separate maintainer decision and should be blocked until the project has:

- a settled public version and release artifact strategy
- generated or stronger mechanically enforced schema/SDK synchronization, or an explicit maintainer acceptance of manual synchronization
- a production backend plan or an explicit statement that the release candidate remains local-only
- production-grade audit/export posture or an explicit limitation
- resolved or explicitly accepted YAML parser risk
- documented upgrade and migration expectations for persisted local state
- maintained release notes and security review for the candidate scope

## Production Readiness

A release tag does not imply production readiness. Production readiness must be stated explicitly and supported by documentation, tests, security review, and operational runbooks.
