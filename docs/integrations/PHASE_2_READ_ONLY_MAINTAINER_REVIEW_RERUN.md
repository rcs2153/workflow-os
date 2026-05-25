# Phase 2 Read-Only Maintainer Review Rerun

Review date: 2026-05-25

Reviewed scope:

- GitHub read-only adapter.
- Jira read-only adapter.
- CI/GitHub Actions read-only adapter.
- Phase 2 read-only reference examples.
- Adapter policy-precheck provenance hardening.
- Adapter telemetry posture documentation.
- Jira live-auth hardening.
- Maintainer live smoke-test procedure.
- Integration contract gate and CI configuration.

## 1. Executive Verdict

**Not ready for public read-only integration preview.**

The Phase 2 read-only integration work is still **ready for internal read-only integration use**. The previous public-preview blockers around policy-precheck provenance, telemetry honesty, Jira authentication, live-smoke procedure, and temporal documentation have been addressed in code, tests, and docs.

The remaining blocker is evidence, not scope: this review did not run maintainer-owned live smoke tests because no maintainer credentials were explicitly available and safe. Fixture and contract evidence is strong, but a public read-only integration preview should not be announced until GitHub, Jira, and GitHub Actions live smoke results are recorded against approved non-sensitive test resources.

## 2. Previous Blocker Status

| Blocker | Status | Evidence | Remaining gap |
| --- | --- | --- | --- |
| Adapter request helpers silently pre-authorized policy decisions. | Fixed | `github_read_request`, `jira_read_request`, and `github_actions_read_request` now require an explicit `AdapterPolicyPrecheck` argument. Tests cover missing, denied, fixture/test, and runtime-allowed prechecks. | None for fixture/internal use. |
| Adapter policy-precheck provenance was not explicit. | Fixed | `AdapterPolicyPrecheck` records `RuntimePolicy`, `ApprovalDecision`, or `FixtureTest` provenance. Docs require operators to distinguish runtime policy from fixture/test authorization. | Future runtime adapter execution must continue using runtime or approval provenance, not fixture provenance. |
| Runtime and fixture/test adapter authorization paths were blurred. | Fixed | CLI fixture examples call `fixture_policy_precheck(...)`; tests use `runtime_allowed(...)` separately for runtime-authorized request behavior. Missing and denied prechecks fail closed. | No generic runtime adapter execution path exists yet; that remains out of scope. |
| Adapter audit/observability records were produced but not durably integrated into runtime sinks. | Fixed by explicit separation | Docs now state Phase 2 adapter telemetry is contract-level telemetry. Example READMEs say fixture-backed CLI paths do not persist adapter records as first-class runtime audit/observability records. `npm run check:integrations` enforces this language. | Public preview docs must keep this limitation visible until durable runtime mapping exists. |
| Jira live authentication posture was underspecified. | Fixed | Jira config supports Atlassian Cloud Basic auth with `WORKFLOW_OS_JIRA_EMAIL` plus `WORKFLOW_OS_JIRA_API_TOKEN`, fallback `JIRA_EMAIL` plus `JIRA_API_TOKEN`, and explicit bearer auth for deployments that support it. Basic auth precedence, partial auth failure, redacted metadata, and secret non-leakage are tested. | Live Jira compatibility still needs maintainer smoke execution before public preview. |
| Maintainer live smoke test procedure was missing. | Fixed | `docs/integrations/live-smoke-tests.md` documents GitHub, Jira, and GitHub Actions smoke procedures. `package.json` includes `smoke:github-live`, `smoke:jira-live`, `smoke:ci-live`, and `smoke:integrations-live`. | Procedures were not executed in this review because safe credentials were not available. |
| Docs blurred internal Phase 2 state and public preview posture. | Fixed | README, charter, limitations, integration docs, setup docs, and examples now distinguish `0.1.0-preview.1` local-kernel release, Phase 2 development-branch adapters, and pending public read-only preview approval. | Historical release-review docs intentionally preserve their original local-kernel evidence. |
| Docs might imply write support or production integration readiness. | Fixed | Search results show write and production-integration references are denials, non-goals, future work, or historical review context. Phase 2 docs explicitly reject writes, reruns, dispatch, webhooks, OAuth, hosted operation, distributed workers, production DB, and Level 3/4 autonomy. | Keep this posture during any public preview announcement. |

## 3. New Blockers

1. **Maintainer live smoke results are not recorded.**
   Live tests are correctly skipped by default and were not run here because maintainer credentials were not explicitly available and safe. Before a public read-only integration preview, maintainers should run and record the documented smoke procedures for GitHub, Jira, and GitHub Actions against approved non-sensitive resources.

No blocker prevents continued internal fixture-gated read-only integration use.

## 4. Non-Blocking Issues

1. Adapter telemetry remains contract-level, not durable runtime telemetry. This is now honest and tested, but it should remain a prominent public limitation.
2. Read-only examples rely on explicit fixture local handlers through `--mock-all-local-skills`; they are useful references, not a generic adapter execution framework.
3. Live-provider behavior is opt-in and cannot be proven by normal CI. This is correct for safety, but maintainers need periodic manual smoke evidence.
4. Provider-specific adapter capabilities (`github.read`, `jira.read`, `ci.read`) coexist with workflow-level symbolic requirements such as `external.read`; docs explain this, but it remains a subtle two-layer capability model.
5. Redaction is deterministic and tested for preview paths, but it is not enterprise DLP.

## 5. Adapter-By-Adapter Assessment

### GitHub

Assessment: credible internal read-only adapter, fixture-proven and policy-gated.

Evidence:

- Supports repository metadata, default branch, file reference/content metadata, PR metadata, diff summary, changed files, comments, and check summaries.
- Uses read-only actions and denies write-style behavior in tests.
- Requires `github.read`.
- Missing and denied policy prechecks fail closed.
- Health output reports credential presence without token values.
- Live test is ignored by default.

Public-preview gap: live GitHub smoke was not run in this review.

### Jira

Assessment: credible internal read-only adapter with now-deliberate auth posture.

Evidence:

- Supports issue metadata, description reference, comments reference, status, priority, labels, assignee/reporter display data, and project metadata.
- Uses read-only actions and denies write behavior in tests.
- Requires `jira.read`.
- Atlassian Cloud Basic auth and explicit bearer auth are modeled, documented, and tested.
- Partial Basic auth fails clearly without exposing email or token.
- Issue descriptions and comments are treated as sensitive/reference-only data.
- Live test is ignored by default.

Public-preview gap: live Jira smoke was not run in this review.

### CI/GitHub Actions

Assessment: strong internal read-only adapter, especially around no-rerun/no-dispatch boundaries.

Evidence:

- Supports workflow run metadata, jobs, check summaries, failure summaries, log references, and bounded redacted log excerpts.
- Denies rerun and workflow dispatch operations in tests.
- Requires `ci.read`.
- Log redaction and size limiting are tested.
- Health/debug/audit-style output is tested for token non-leakage.
- Live test is ignored by default.

Public-preview gap: live GitHub Actions smoke was not run in this review.

## 6. Policy/Capability Assessment

Policy and capability enforcement is defensible for Phase 2 internal use.

Positive evidence:

- Adapter requests require explicit policy-precheck provenance.
- Missing policy precheck fails closed.
- Denied precheck prevents adapter invocation.
- Runtime-allowed precheck permits read-only invocation.
- Fixture/test authorization is distinguishable and used only by fixture paths.
- Write mode and write/rerun/dispatch capabilities remain denied.
- Unknown capabilities fail closed.

Remaining caution:

- Future runtime adapter execution must not reuse fixture/test precheck provenance. That should be a review gate for any prompt that moves adapters into ordinary workflow execution.

## 7. Security/Privacy Assessment

The security posture is appropriate for internal read-only evaluation and close to public-preview quality, pending live smoke evidence.

Positive evidence:

- Specs do not store provider credentials.
- Live credentials are read from environment variables only.
- Health output reports credential presence without values or token prefixes.
- Tests assert token-like secrets do not appear in debug, health, audit-style records, or CLI inspect output.
- GitHub file contents, Jira descriptions/comments, and CI logs are summarized or reference-only by default.
- CI log excerpts are bounded and redacted.
- `cargo audit` and `npm audit --audit-level=moderate` passed.

Residual risks:

- Live provider payloads can contain sensitive titles, paths, comments, issue summaries, labels, and display names even when credentials are read-only.
- Redaction is heuristic and not a substitute for enterprise DLP.
- No live smoke was executed in this review, so provider behavior against real services is not evidenced here.

## 8. Test Quality Assessment

The test suite is meaningful and behavior-oriented.

Positive evidence:

- `cargo test --workspace` passed with live GitHub, Jira, and GitHub Actions tests ignored by default.
- Adapter tests cover successful reads, classified failures, write denial, precheck provenance, missing/denied policy, credential health, redaction, adapter invocation records, observability records, and live-test skip posture.
- CLI example tests validate, run, approve, inspect, check missing fixtures, and assert no writes.
- `npm run check:integrations` runs focused adapter tests, read-only example tests, CI gate checks, documentation posture checks, and fixture-backed CLI smoke paths.
- `npm run check:contracts` passed.

Limitations:

- Live-provider compatibility is not part of default CI and was not manually executed during this review.
- Durable runtime audit/observability mapping for adapter records remains future work by design.

## 9. Documentation Honesty Assessment

Documentation is now temporally precise.

Positive evidence:

- `0.1.0-preview.1` remains documented as a local-kernel preview with no real adapters in its release contract.
- Phase 2 development-branch adapters are described as internal read-only work pending public preview approval.
- Provider setup docs state live mode is opt-in and fixture mode is the default test path.
- Example READMEs distinguish real runtime behavior, fixture data, deterministic local mock handlers, and contract-level adapter telemetry.
- Docs reject write support, production integration readiness, OAuth, webhook ingestion, hosted operation, distributed workers, production DB, and Level 3/4 autonomy.

Remaining caution:

- The public read-only integration preview should not be announced until live smoke results are added to release evidence or maintainer notes.

## 10. Do-Not-Build-Yet List

Do not build these next:

- GitHub write actions: branches, commits, PR creation, comments, reviews, labels, merges, closes.
- Jira write actions: issue updates, comments, transitions, assignments, labels, links.
- CI write actions: rerun, dispatch, cancel, artifact mutation, check mutation.
- Webhook/event ingestion services.
- OAuth app flows.
- Hosted integration service.
- Distributed workers or production backend.
- Level 3/4 autonomy enablement.
- Generic live adapter execution from arbitrary workflow specs.
- Durable adapter telemetry export to SIEM/OTel before the runtime mapping contract exists.

## 11. Final Recommendation

Continue internal Phase 2 read-only integration evaluation and fixture-gated development.

Do not announce a public read-only integration preview yet. First, run the documented maintainer live smoke tests for GitHub, Jira, and GitHub Actions with approved non-sensitive test resources, record the results, and confirm no credentials or raw sensitive provider payloads appear in output.

If those live smoke tests pass, the code, docs, CI, and examples otherwise look ready to support a narrowly scoped public read-only integration preview with clear limitations:

- read-only only
- fixture-first CI
- opt-in live tests
- no writes
- no production integration readiness
- adapter telemetry contract-level only in fixture-backed CLI examples

## Commands Run

All commands were run from the repository root.

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed. Live GitHub, Jira, and GitHub Actions tests were ignored by default as intended. |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | Passed |
| `cargo audit` | Passed |
| `npm ci` | Passed |
| `npm run check` | Passed |
| `npm run check:contracts` | Passed |
| `npm run check:integrations` | Passed |
| `npm audit --audit-level=moderate` | Passed, 0 vulnerabilities |
| `target/debug/workflow-os --project-dir examples/github-read-only-review-context validate` | Passed with experimental lifecycle warnings only |
| `target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality validate` | Passed with experimental lifecycle warnings only |
| `target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary validate` | Passed with experimental lifecycle warnings only |
| `target/debug/workflow-os --project-dir examples/github-read-only-review-context --state-dir /private/tmp/workflow-os-p2-rerun-gh-20260525-a --mock-all-local-skills run ex/gh` | Passed, paused at `WaitingForApproval` |
| `target/debug/workflow-os --project-dir examples/github-read-only-review-context --state-dir /private/tmp/workflow-os-p2-rerun-gh-20260525-a --mock-all-local-skills approve run-1779714517509734000-2 approval/run-1779714517509734000-2/ctx --actor user/phase2-reviewer --reason phase2-rerun-review` | Passed, completed |
| `target/debug/workflow-os --project-dir examples/github-read-only-review-context --state-dir /private/tmp/workflow-os-p2-rerun-gh-20260525-a inspect run-1779714517509734000-2` | Passed, showed event history through `RunCompleted` |
| `target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality --state-dir /private/tmp/workflow-os-p2-rerun-jira-20260525-a --mock-all-local-skills run ex/jira` | Passed, paused at `WaitingForApproval` |
| `target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality --state-dir /private/tmp/workflow-os-p2-rerun-jira-20260525-a --mock-all-local-skills approve run-1779714534498930000-2 approval/run-1779714534498930000-2/intake --actor user/phase2-reviewer --reason phase2-rerun-review` | Passed, completed |
| `target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality --state-dir /private/tmp/workflow-os-p2-rerun-jira-20260525-a inspect run-1779714534498930000-2` | Passed, showed event history through `RunCompleted` |
| `target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary --state-dir /private/tmp/workflow-os-p2-rerun-ci-20260525-a --mock-all-local-skills run ex/ci` | Passed, paused at `WaitingForApproval` |
| `target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary --state-dir /private/tmp/workflow-os-p2-rerun-ci-20260525-a --mock-all-local-skills approve run-1779714548530813000-2 approval/run-1779714548530813000-2/diagnose-ci-failure --actor user/phase2-reviewer --reason phase2-rerun-review` | Passed, completed |
| `target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary --state-dir /private/tmp/workflow-os-p2-rerun-ci-20260525-a inspect run-1779714548530813000-2` | Passed, showed event history through `RunCompleted` |
| `rg` searches for write-support, public-preview, and production-integration overclaims | Passed; matches were denials, pending-review language, future work, or historical review context |

Live tests were not run because maintainer credentials were not explicitly available and safe.
