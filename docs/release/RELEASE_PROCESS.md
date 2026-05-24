# Release Process

Workflow OS is not production-ready yet. This process defines the release shape the project will use as it matures.

## Release Requirements

Before any release:

- CI must pass.
- Changelog entries must be current.
- Public contracts must be documented.
- Breaking changes must reference migration notes.
- Security-sensitive changes must be reviewed.
- Version numbers must follow [SEMVER.md](SEMVER.md).

## Release Steps

1. Confirm the release scope.
2. Confirm CI is passing on the release commit.
3. Update `CHANGELOG.md`.
4. Update package and crate versions consistently.
5. Tag the release.
6. Publish artifacts only after maintainers approve.
7. Announce known limitations and experimental features.

## Production Readiness

A release tag does not imply production readiness. Production readiness must be stated explicitly and supported by documentation, tests, security review, and operational runbooks.
