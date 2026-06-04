# Phase 2 Public Read-Only Preview Readiness Rerun

Review date: 2026-06-04

Reviewed posture:

- Workflow OS `0.2.0-preview.1` public read-only integration preview.
- GitHub read-only adapter.
- Jira read-only adapter.
- GitHub Actions / CI read-only adapter.
- Phase 2 live smoke evidence recorded in `PHASE_2_LIVE_SMOKE_EVIDENCE.md`.
- Fixture-backed integration gate, adapter telemetry mapping, policy/capability model, provider docs, examples, and CI configuration.

This review does not approve production integration readiness, write-capable adapters, hosted operation, distributed workers, generic live adapter execution, OAuth, webhooks, or Level 3/4 autonomy.

## 1. Executive Verdict

**Ready for public read-only integration preview.**

The previous public-preview blocker was missing maintainer-owned live smoke evidence. That evidence is now recorded for GitHub, Jira, and GitHub Actions / CI, and the offline fixture-backed integration gate remains green.

This readiness decision is intentionally narrow. It supports a public preview positioned as:

**Workflow OS 0.2.0-preview.1 — Public Read-Only Integration Preview**

The approved posture is:

- read-only only;
- opt-in for live providers;
- fixture-first in normal CI;
- local/kernel-oriented, not hosted;
- not production-ready;
- not write-capable;
- not distributed-runtime-ready;
- not Level 3/4 autonomy-ready.

The live smoke evidence is sufficient for public preview if release materials clearly state that live coverage is shallow smoke coverage, not broad provider compatibility proof. Broader live operation coverage remains a follow-up, not a release blocker.

## 2. Live Smoke Evidence Assessment

Evidence source: `docs/integrations/PHASE_2_LIVE_SMOKE_EVIDENCE.md`.

| Provider | Recorded | Result | Scope Exercised | Assessment |
| --- | --- | --- | --- | --- |
| GitHub | Yes | Passed | Repository metadata read for `octocat/Hello-World` | Acceptable for public preview as a minimal GitHub API reachability smoke. Must not be described as AGT repo coverage or full GitHub operation coverage. |
| Jira | Yes | Passed | Jira issue metadata read for sandbox issue `KAN-1` using Atlassian Cloud Basic auth | Acceptable for public preview as Jira Cloud read/auth proof. Token rotation remains recommended because a sandbox token was pasted into the local evaluation thread. |
| GitHub Actions / CI | Yes | Passed | Workflow run metadata read for `rcs2153/AGT` workflow run `26415289853` | Acceptable for public preview as GitHub Actions run metadata proof. Must not be described as jobs/logs/rerun coverage. |

No evidence document includes token values, authorization headers, raw provider payloads, raw issue bodies, full PR content, raw CI logs, or private credential values.

No write, comment, branch, issue update, status transition, workflow rerun, workflow dispatch, cancellation, merge, assignment, or provider mutation was part of the recorded passing smoke paths.

Failures and retries were recorded honestly:

- GitHub Actions initially returned `ci.github_actions.http.404` with the wrong identifier, then passed with workflow run ID `26415289853`.
- Jira initially returned `jira.http.404` while a placeholder token was configured, then passed with the real sandbox API token.

Limitations requiring explicit release-note language:

- GitHub live smoke currently exercises `octocat/Hello-World`, not the approved `rcs2153/AGT` repository.
- GitHub Actions live smoke exercises workflow run metadata only.
- Jira live smoke exercises issue metadata only.
- The Jira sandbox API token should be rotated because it was pasted into the local evaluation thread before smoke execution.

These limitations do not block public read-only preview if they are documented and the preview remains narrow.

## 3. Provider-By-Provider Verdict

### GitHub

Verdict: **Ready for public read-only integration preview with accepted live-smoke limitation.**

Positive evidence:

- Fixture tests cover repository metadata, default branch, file reference behavior, pull request metadata, diff summary, changed files, comments as read-only data, check summaries, error classification, no-write behavior, credential health, policy precheck behavior, and redaction.
- Live smoke passed for repository metadata against `octocat/Hello-World`.
- Write actions remain unsupported or denied: branch creation, commits, PR creation, comments, reviews, labels, merges, PR closure, reruns, workflow dispatch, webhooks, and OAuth.
- Credentials are environment-variable based and not stored in specs.

Accepted limitation:

- Live smoke has not exercised GitHub reads against `rcs2153/AGT`, nor live PR/file/comment/check paths. Release language must say fixture tests cover the broader contract while live proof is minimal repository metadata reachability.

Recommended follow-up:

- Parameterize the GitHub live smoke target so maintainers can exercise an approved repository such as `rcs2153/AGT` before broader GitHub-specific claims.

### Jira

Verdict: **Ready for public read-only integration preview with sandbox-token rotation required as operational hygiene.**

Positive evidence:

- Fixture tests cover issue metadata, description references, comments references, status, priority, labels, assignee/reporter display data, project metadata, error classification, no-write behavior, credential health, policy precheck behavior, and redaction.
- Live smoke passed against Atlassian Cloud sandbox issue `KAN-1`.
- Atlassian Cloud Basic auth is documented and tested; bearer auth remains explicitly deployment-specific.
- Write actions remain unsupported or denied: issue creation, issue updates, comments, transitions, assignment, labels, links, webhooks, and OAuth.

Accepted limitation:

- Live smoke exercised issue metadata only, not separate comments, descriptions, or project metadata reads.

Security follow-up:

- Rotate the sandbox Jira API token because it was pasted into the local evaluation thread. This is not evidence of adapter leakage, but it is still credential hygiene.

### GitHub Actions / CI

Verdict: **Ready for public read-only integration preview with accepted live-smoke limitation.**

Positive evidence:

- Fixture tests cover workflow run metadata, jobs, check status summary, failure summary, log reference, bounded redacted log excerpts, rate/auth/permission/not-found classifications, no-rerun behavior, no-dispatch behavior, credential health, policy precheck behavior, and redaction.
- Live smoke passed against `rcs2153/AGT` workflow run `26415289853`.
- Write/rerun/dispatch actions remain unsupported or denied: rerun workflow, rerun failed jobs, cancel workflow, workflow dispatch, artifact upload, log deletion, and check mutation.

Accepted limitation:

- Live smoke exercised workflow run metadata only. It did not live-read jobs, check summaries, failure summaries, log references, or redacted log excerpts.

## 4. Accepted Limitations

These limitations are accepted for `0.2.0-preview.1` only if public materials keep them visible:

- Public preview is read-only only.
- Live provider use is opt-in and disabled by default.
- Normal CI remains fixture-first and credential-free.
- Live smoke coverage is shallow provider reachability evidence, not full live operation coverage.
- GitHub live smoke used `octocat/Hello-World`, not `rcs2153/AGT`.
- GitHub Actions live smoke used workflow run metadata only.
- Jira live smoke used issue metadata only.
- Jira token rotation is recommended because a sandbox API token was pasted into the local evaluation thread.
- Adapter telemetry mapping is local/runtime-preview telemetry, not production telemetry export.
- Redaction is deterministic preview redaction, not enterprise DLP.
- No generic live adapter execution from arbitrary workflow specs exists.
- No production database backend, distributed workers, hosted integration service, UI, marketplace, OAuth flow, webhook ingestion, write-capable adapters, or Level 3/4 autonomy enablement exists.

## 5. Release Blockers

No blocker remains for a narrow public read-only integration preview.

Do not proceed if release materials imply any of the following:

- production integration readiness;
- write support;
- full live operation coverage for every adapter operation;
- generic live adapter execution;
- provider automation or mutation;
- hosted/distributed runtime readiness;
- Level 3/4 autonomy readiness.

## 6. Security/Privacy Assessment

Verdict: **Acceptable for public read-only preview with explicit limitations.**

Positive evidence:

- Credentials are loaded through environment variables only.
- Specs and fixtures do not store provider credentials.
- Smoke wrappers fail closed when required environment variables are missing and redact token-like values in failure output.
- Recorded smoke evidence does not include token values, authorization headers, raw provider payloads, raw issue bodies, PR content, or CI logs.
- Health/debug/audit-style tests assert no token exposure.
- Jira descriptions/comments, GitHub file contents, and CI logs are summarized or reference-oriented by default.
- CI log excerpts are bounded and redacted in fixture tests.
- `cargo audit` passed.
- `npm audit --audit-level=moderate` passed with `0 vulnerabilities` after the sandbox-blocked first attempt was rerun with registry access.

Residual risks:

- Provider metadata can still be sensitive even when read-only.
- Preview redaction is not enterprise DLP.
- The pasted Jira sandbox token should be rotated.
- Live smoke did not exercise sensitive paths such as Jira comments/descriptions or CI log excerpts.

## 7. Policy/Capability Assessment

Verdict: **Pass for public read-only preview.**

Evidence:

- Adapter requests carry explicit policy precheck provenance.
- Fixture/test authorization is distinct from runtime policy approval.
- Missing policy precheck fails closed.
- Denied precheck prevents adapter invocation.
- Runtime-policy allowed precheck permits read-only invocation.
- GitHub uses `github.read`.
- Jira uses `jira.read`.
- CI uses `ci.read`.
- Write capabilities remain denied or unavailable, including `github.write`, `jira.write`, `ci.write`, `ci.rerun`, and `adapter.write`.
- Adapters do not mutate core workflow state directly.

## 8. Telemetry Assessment

Verdict: **Pass for public read-only preview as local/runtime-preview telemetry.**

Evidence:

- Adapter invocation and observability records are mapped into runtime-visible local telemetry for controlled fixture-backed examples.
- Telemetry includes adapter kind, action, capability, operation mode, policy precheck provenance, correlation ID, redaction metadata, and run-scoped identity where available.
- `workflow-os inspect` can show concise adapter telemetry summaries for local example runs.
- Docs distinguish scoped runtime-visible adapter telemetry from production telemetry export.

Limitations:

- Telemetry mapping is not a generic adapter execution framework.
- Telemetry is not SIEM/OpenTelemetry export.
- Runtime-visible telemetry is proven through fixture-backed examples, not through live provider workflow execution.

## 9. Test Quality Assessment

Verdict: **Strong enough for public read-only preview.**

Positive evidence:

- `cargo test --workspace` passed.
- Live GitHub, Jira, and GitHub Actions tests were ignored by default during normal tests.
- Adapter fixture tests cover success paths, classified failures, credential health, redaction, token non-leakage, policy precheck behavior, write denial, and live-test skip posture.
- Read-only example tests validate, run, approve, inspect, verify fixture behavior, and assert runtime adapter telemetry.
- `npm run check:integrations` passed and exercises all three adapter contract suites plus all three fixture-backed examples.
- CI includes a dedicated `Phase 2 Read-Only Integration Contracts` job.

Limitations:

- Live tests are maintainer-run only and not CI-gated.
- Live smoke exercises one narrow read path per provider family.
- Broader live read coverage should be added before stronger provider-compatibility claims.

## 10. Documentation Honesty Assessment

Verdict: **Pass with release-pack follow-up required.**

Current docs do not claim:

- write support;
- production integration readiness;
- public production readiness;
- distributed workers;
- hosted service;
- OAuth;
- webhook ingestion;
- generic live adapter execution;
- Level 3/4 autonomy enablement.

Current docs still describe Phase 2 as pending public read-only preview approval in several places. That was correct before this rerun. If maintainers accept this review, the next release posture pack should update public-facing docs for `0.2.0-preview.1` while preserving the exact limitations above.

## 11. Do-Not-Build-Yet List

Do not build or announce:

- GitHub writes: branches, commits, PR creation, comments, reviews, labels, merges, closes.
- Jira writes: issue updates, comments, transitions, assignments, labels, links.
- CI writes: rerun, dispatch, cancel, artifact mutation, check mutation.
- Generic live adapter execution from arbitrary workflow specs.
- Generic HTTP adapter execution.
- Webhooks or event ingestion services.
- OAuth app flows.
- Hosted integration service.
- Production database backend.
- Distributed workers.
- Production telemetry export, SIEM integration, or OpenTelemetry export.
- Level 3/4 autonomy enablement.
- Domain packs.

## 12. Commands Run And Results

All commands were run from the repository root unless noted.

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed; live GitHub, Jira, and GitHub Actions tests were ignored by default |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | Passed |
| `cargo audit` | Passed |
| `npm ci` | Passed |
| `npm run check` | Passed |
| `npm run check:contracts` | Passed after rerun following `npm ci`; the earlier parallel run failed because `npm ci` was reinstalling `node_modules` at the same time |
| `npm run check:integrations` | Passed |
| `npm audit --audit-level=moderate` | Initial sandbox run failed with DNS resolution for `registry.npmjs.org`; escalated rerun passed with `0 vulnerabilities` |
| Verify live tests skipped by default | Passed through `cargo test --workspace`; provider live tests were reported ignored |
| Live tests | Not rerun in this review; this review used the recorded maintainer evidence in `PHASE_2_LIVE_SMOKE_EVIDENCE.md` |

## 13. Final Recommendation

Prepare a `0.2.0-preview.1` release posture pack for **Workflow OS Public Read-Only Integration Preview**.

The release pack must preserve the narrow positioning:

- read-only only;
- opt-in live provider mode;
- fixture-first normal CI;
- no writes, reruns, dispatches, webhooks, OAuth, hosted operation, distributed workers, production backend, generic runtime adapter execution, production telemetry export, domain packs, or Level 3/4 autonomy;
- live smoke evidence exists but is shallow and should be described exactly.

Recommended immediate next steps:

1. Rotate the sandbox Jira API token that was pasted into the local evaluation thread.
2. Create `0.2.0-preview.1` release notes/checklist/draft language that documents the accepted smoke limitations.
3. Consider a follow-up issue to parameterize the GitHub live smoke target for an approved repository such as `rcs2153/AGT`.
4. Add broader opt-in live smoke coverage later for GitHub PR/file/check paths, Jira descriptions/comments/project metadata, and GitHub Actions jobs/log references before making stronger compatibility claims.
