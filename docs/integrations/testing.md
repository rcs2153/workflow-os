# Integration Testing

Workflow OS Phase 2 read-only integrations use a fixture-first test posture. The integration gate proves the GitHub, Jira, and CI/GitHub Actions read-only adapters still obey the generic adapter contract without requiring live credentials.

This testing posture applies to the development branch. It does not expand the `0.1.0-preview.1` local kernel release contract, and it does not announce a public read-only integration preview.

## Fixture Gate

Run the full read-only integration gate from the repository root:

```sh
npm run check:integrations
```

The gate runs:

- GitHub read-only adapter contract tests.
- Jira read-only adapter contract tests.
- CI/GitHub Actions read-only adapter contract tests.
- GitHub, Jira, and CI read-only reference example tests.
- CLI fixture smoke tests for all read-only examples.

The fixture gate verifies:

- examples validate with Rust validation
- examples run against local fixtures
- approval-gated examples resume and complete
- adapter requests carry fixture/test policy-precheck provenance in fixture paths
- write/rerun/dispatch capabilities are denied or unavailable
- adapter audit and observability records are emitted
- adapter responses are summarized and redacted
- token-like values do not appear in health, audit, debug, or CLI inspect output
- live tests remain skipped by default

CI runs this gate in the `Phase 2 Read-Only Integration Contracts` job. CI must not require GitHub, Jira, or CI provider credentials.

## Fixture Mode

Fixture mode uses checked-in static data under each example directory. Fixture mode is the default contract test mode for integrations because it is deterministic, offline, and safe for contributors.

Fixture tests must not:

- call live provider APIs
- require credentials
- mutate external systems
- pretend fixture/test policy authorization is runtime policy authorization
- store raw credentials or private payloads in logs
- hide skipped live tests

## Live Read-Only Tests

Live tests are opt-in and skipped by default. They are useful for maintainers who want to verify provider behavior manually, but they are not required for normal CI.

Maintainer-owned live smoke procedures and npm wrappers are documented in [live smoke tests](live-smoke-tests.md).

Read the provider-specific setup docs before enabling live tests:

- [GitHub read-only setup](../operations/github-read-only-setup.md)
- [Jira read-only setup](../operations/jira-read-only-setup.md)
- [GitHub Actions read-only setup](../operations/github-actions-read-only-setup.md)

Live tests must use read-only credentials. Do not use credentials with write, rerun, dispatch, comment, merge, issue-update, or administrative scopes.

Jira live read-only tests support Atlassian Cloud Basic auth with `WORKFLOW_OS_JIRA_EMAIL` plus `WORKFLOW_OS_JIRA_API_TOKEN`. Bearer auth is available only for Jira deployments that explicitly support bearer tokens. Normal CI must not set these variables or require live Jira credentials.

Available maintainer wrappers:

- `npm run smoke:github-live`
- `npm run smoke:jira-live`
- `npm run smoke:ci-live`
- `npm run smoke:integrations-live`

These wrappers check required environment variables before invoking ignored Rust live tests and do not print secret values.

## Failure Expectations

The integration gate is intentionally conservative. A failure usually means one of:

- the adapter contract changed without updating tests and docs
- a fixture file is missing or malformed
- a read-only example no longer validates
- a write-like capability was introduced accidentally
- redaction no longer protects token-like values
- a live test became required by default

Treat credential leakage, raw sensitive payloads in audit/log output, and accidental write capability as release-blocking integration bugs.
