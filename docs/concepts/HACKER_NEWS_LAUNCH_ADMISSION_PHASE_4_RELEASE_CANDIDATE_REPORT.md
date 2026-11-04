# Hacker News Launch Admission Phase 4 Release-Candidate Report

## Status

Local release-candidate review is complete and focused review is accepted.
Terminal-green exact-head pull-request CI remains required. This report does not authorize a
tag, GitHub Release, package publication, Hacker News submission, provider
mutation, or production-readiness claim.

## Governed Phase

- workflow: `dg/release`
- run: `run-1793753820320602000-2`
- approval: `approval/run-1793753820320602000-2/release-approved`
- presentation: `presentation/11cc4ed469983ca8`
- outcome: granted under delegated-maintainer authority after the complete
  persisted handoff was reviewed
- phase close: `Completed`, 39 events, one approval, zero retries, zero
  escalations, and one proof-enforced approval-presentation record

## Scope

The phase reviewed the `0.3.0-preview.1` release identity, product claims,
security posture, complete local test matrix, dependency audits, release notes,
release draft, checklist, and the accepted Hacker News launch-admission
evidence through Phase 3.

Strict non-goals remained unchanged: no tag, publication, submission, version
change, provider mutation, runtime feature, check bypass, or production claim.

## Validation

The current release-candidate checkout passed:

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace --all-targets`;
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`;
- `cargo metadata --locked --format-version 1`;
- `cargo audit`, with only the documented allowed `unsafe-libyaml`
  maintenance warning;
- pinned-Node-20 `npm ci`;
- `npm run check`;
- `npm run check:contracts`;
- `npm run check:integrations`;
- `npm audit --audit-level=moderate`, with zero vulnerabilities;
- CLI text and JSON release-identity probes;
- Rust and npm package-version probes;
- provider user-agent identity inspection; and
- `git diff --check`.

Representative suites included 773 Core unit tests, 176 CLI integration tests,
345 local-executor integration tests with one opt-in live write proof ignored,
43 local-host foreground tests, 15 TypeScript SDK tests, 35 dogfood-helper
tests, 12 public GitHub fixture tests, four release-package tests, and three
integration-helper tests.

## External Fresh-Rerun Reconciliation

An external fresh-clone evaluation at public commit `d3f6df9` found:

1. one P0 contradiction between the declared Active Delivery Queue and five
   continuation-context commits;
2. one P1 dead generated-agent-guidance reference;
3. one P1 provider-fixture `next-action` routing defect; and
4. one P2 independent-review assurance gap.

The findings were accurate for the evaluated commit. They are not silently
discarded because current `main` is newer.

The first three are resolved on the release-candidate head:

- the authoritative queue records the P0 preemption and remaining ordered
  sequence, while Current Lane Evidence is explicitly non-authorizing;
- generated guidance references only local files produced by its scaffold
  entry point; and
- provider-fixture `next-action` uses the fixture state and immutable bundle,
  with running, waiting, and terminal behavior covered by the 12-test public
  fixture suite.

The fourth is a truthful limitation. Governed focused reviews in this
repository are owner-conducted engineering reviews unless explicitly stated
otherwise. They are not independent security reviews. A broad v1 or
production-readiness claim should require separately attributable independent
assurance; this preview does not claim it.

## Focused Review

Focused maintainer/product/security review accepted the release-candidate
evidence with no remaining local blocker. The review confirmed that:

- every checked release-checklist item has current evidence;
- the external rerun's three actionable findings are fixed on the candidate
  head rather than merely reclassified;
- current release language does not broaden the fixture, trusted-host,
  provider-write, hosted, OpenShell, or autonomy boundaries;
- the candidate does not present owner-conducted review as independent
  security assurance; and
- exact-head pull-request CI and publication remain separate gates.

The review record is [Hacker News Launch Admission Phase 4 Release-Candidate
Review](HACKER_NEWS_LAUNCH_ADMISSION_PHASE_4_RELEASE_CANDIDATE_REVIEW.md).

## Release Assessment

No local release-candidate blocker remains for a reviewed preparation pull
request. The candidate is a strong local-first preview, not a production
control plane. Exact-head CI and focused review must pass before merge, and any
tag or publication remains a separate governed decision.
