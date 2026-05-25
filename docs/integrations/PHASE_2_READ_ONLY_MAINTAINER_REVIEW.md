# Phase 2 Read-Only Maintainer Review

Review date: 2026-05-25

Reviewed scope:

- GitHub read-only adapter
- Jira read-only adapter
- CI/GitHub Actions read-only adapter
- Read-only reference examples
- Phase 2 integration contract gate
- CI configuration

## 1. Executive Verdict

**Ready for internal read-only integration use.**

This is not yet ready to call a public read-only integration preview.

The Phase 2 adapters are disciplined, fixture-tested, offline by default, and appear read-only by construction. The integration gate passes and CI includes it. The remaining concerns are not about accidental writes; they are about public-contract rigor around policy-precheck provenance, durable adapter telemetry, and live-provider proof.

## 2. Blocking Issues

No blocker prevents internal fixture-gated read-only integration use.

The following issues should block a public read-only integration preview:

1. **Adapter request helper constructors pre-authorized policy decisions.**
   `github_read_request`, `jira_read_request`, and `github_actions_read_request` construct `AdapterRequest` values with `AdapterPolicyPrecheck::Allowed` directly. Evidence: `crates/workflow-core/src/github.rs:895`, `crates/workflow-core/src/jira.rs:913`, and `crates/workflow-core/src/ci.rs:857`.

   This does not create write risk because Phase 2 writes are denied, but it weakens the architectural claim that adapter calls cannot bypass policy. For a public preview, request construction should either require an explicit policy decision/precheck argument, be marked fixture/test-only, or be routed through a runtime policy-authorized adapter invocation API.

   Follow-up status: fixed after this review by requiring explicit adapter policy pre-check provenance on request helpers. Fixture paths now use fixture/test provenance, and tests cover missing, denied, fixture, and runtime-allowed prechecks.

2. **Adapter audit/observability records are produced but not durably integrated into example runtime sinks.**
   The adapters return `AdapterInvocationRecord` and `AdapterObservabilityRecord`, and focused tests assert those records exist. The CLI example handlers call adapter methods and then return a skill output summary; they do not persist adapter invocation records as first-class runtime audit/observability events.

   For internal review this is acceptable as a contract proof. For public preview, operator-facing docs and examples should either persist those adapter records or state that adapter telemetry is currently contract-level, not durable runtime telemetry.

3. **Live Jira authentication posture is not proven and may not match common Jira Cloud API-token usage.**
   The Jira live client sends `Authorization: Bearer <token>` (`crates/workflow-core/src/jira.rs:201-204`). The docs describe a generic read-only token but do not distinguish bearer tokens from common Atlassian Cloud email/API-token Basic auth. The live test is opt-in and ignored by default, so normal CI gives no proof this works against Jira Cloud.

   Before public preview, either document supported Jira auth precisely or implement the supported auth mode deliberately.

## 3. Non-Blocking Issues

1. **Live-provider coverage is intentionally absent from CI.**
   This is correct for normal CI, but it means live GitHub, Jira, and GitHub Actions behavior is not proven by the default gate.

2. **Read-only example handlers are CLI fixture conveniences, not a general adapter execution framework.**
   This is documented, but it remains easy for users to overread the examples as generic adapter execution.

3. **Adapter response size metadata is conservative.**
   Responses track stored summary bytes, but original provider response size is generally `None`. This is fine for preview, but richer metadata would help operations later.

4. **Provider-specific read capabilities are enforced at adapter level, while workflow specs still declare `external.read`.**
   Runtime policy recognizes supported symbolic read-only adapters using `external.read`. The lower adapter request layer uses `github.read`, `jira.read`, or `ci.read`. This is coherent but should remain documented because it is a two-layer capability model.

5. **Docs still contain some long-term/v0 language that can feel temporally awkward after Phase 2.**
   The major public docs are honest, but older charter/release language still emphasizes adapters as future work in places. It does not claim write support, but should be cleaned before any public read-only preview.

## 4. Adapter-By-Adapter Assessment

### GitHub

Assessment: credible read-only adapter contract for internal use.

Strengths:

- Supports repository metadata, default branch, file metadata/reference, pull request metadata, diff summary, changed files, comments, and check summaries.
- Uses only GET operations in the live client.
- Requires `AdapterKind::GitHub` and `AdapterCapability::GitHubRead`.
- Denies unsupported/write-style operations in tests.
- Classifies auth, permission, not found, rate limit, timeout, validation, malformed, and transient failures.
- Health output reports credential presence without token value.
- Fixture tests are meaningful and do not require credentials.
- Live test is ignored by default.

Concerns:

- Fixed follow-up: helper calls now require explicit pre-check provenance.
- Live GitHub behavior is opt-in only and not proven by default CI.

### Jira

Assessment: credible fixture/read-only contract, but live mode needs auth hardening before public preview.

Strengths:

- Supports issue metadata, summary, description reference, comments reference, status, priority, labels, people, and project metadata.
- Uses only GET operations in the live client.
- Requires `AdapterKind::Jira` and `AdapterCapability::JiraRead`.
- Denies write operations in tests.
- Treats descriptions and comments as reference-only/sensitive.
- Fixture tests cover status, priority, labels, people, comments, descriptions, error classification, health, redaction, and live-test skip behavior.

Concerns:

- Live auth mode is underspecified for real Jira deployments.
- Fixed follow-up: helper calls now require explicit pre-check provenance.

### CI/GitHub Actions

Assessment: strong read-only fixture contract, especially around no rerun/dispatch behavior.

Strengths:

- Supports workflow run metadata, jobs, check summaries, failure summaries, log references, and explicit bounded log excerpts.
- Uses only GET operations in the live client.
- Requires `AdapterKind::Ci` and `AdapterCapability::CiRead`.
- Explicitly denies rerun and workflow dispatch operations in tests.
- Log redaction and size limiting are tested with token/password-like fixture content.
- Health/debug/audit output is tested for token non-leakage.
- Live test is ignored by default.

Concerns:

- Fixed follow-up: helper calls now require explicit pre-check provenance.
- Durable runtime audit/observability integration for adapter reads is not yet first-class in examples.

## 5. Security/Privacy Assessment

The security posture is appropriate for internal read-only integration use.

Positive evidence:

- Specs do not contain provider tokens.
- Credentials are loaded from environment variables for live mode.
- Health output reports credential presence without values.
- Tests assert no token-like values appear in health/debug/audit-style records.
- CI log excerpts are bounded and redact sensitive-looking lines.
- GitHub file contents, Jira descriptions/comments, and CI logs are represented as summaries or references rather than full raw payloads by default.
- `cargo audit` and `npm audit --audit-level=moderate` passed.

Residual risk:

- Redaction is heuristic and should not be treated as enterprise DLP.
- Live provider data may include sensitive titles, file paths, issue summaries, PR metadata, labels, and display names. Current summaries intentionally retain some of that context.
- Public preview should not proceed until docs make the adapter telemetry limitation explicit or runtime sinks persist adapter records.

## 6. Policy/Capability Assessment

The policy/capability model is directionally correct but needs one hardening pass before public preview.

Positive evidence:

- Adapter preconditions deny Phase 2 write mode.
- Write-capable actions are denied.
- Unknown capabilities fail closed.
- GitHub requires `github.read`.
- Jira requires `jira.read`.
- CI requires `ci.read`.
- Runtime policy allows only explicitly supported symbolic Phase 2 read-only adapter references.
- `external.write`, `ci.rerun`, and `adapter.write` are denied or unavailable.

Gap:

- Public request helpers construct an allowed policy precheck internally. This makes tests and examples ergonomic, but it blurs provenance of the policy decision. For a public integration preview, policy-precheck values should come from the runtime/policy layer or be explicitly fixture/test-only.

## 7. Test Quality Assessment

The test suite is meaningful and substantially better than object-construction tests.

Positive evidence:

- Adapter fixture tests cover successful reads, classified failures, write denial, credential health, redaction, audit/observability record production, and live-test skip posture.
- Example tests validate, run, approve, inspect, and check missing-fixture failures.
- `npm run check:integrations` exercises all three adapter test suites and all three read-only examples.
- CI includes the integration gate in `.github/workflows/ci.yml:96-119`.
- Live tests are visibly ignored by default in `cargo test --workspace`.

Limitations:

- Default CI does not exercise live providers. This is correct, but it means live compatibility is a maintainer-run check.
- Durable runtime storage of adapter-specific invocation records is not proven by example tests.

## 8. Documentation Honesty Assessment

Documentation is mostly honest and careful.

Positive evidence:

- Phase 2 docs explicitly exclude writes, reruns, dispatch, webhooks, OAuth, hosted services, distributed workers, production database backends, and Level 3/4 autonomy.
- Provider setup docs state fixture mode is the normal CI path and live tests are opt-in.
- Example READMEs state what is real and what is mocked.
- CI log redaction docs describe the current implementation as a preview safety layer, not complete DLP.

Gaps:

- Jira live auth is not precise enough.
- Adapter telemetry docs should distinguish contract-produced records from durably persisted runtime audit records until that integration exists.
- Some older product-charter language still frames adapters as fully future/deferred, which is now only true for write-capable/production integrations.

## 9. Do-Not-Build-Yet List

Do not build these until the public-preview blockers are fixed:

- GitHub write actions: branches, commits, PR creation, comments, reviews, labels, merges, closes.
- Jira write actions: issue updates, comments, transitions, assignments, labels, links.
- CI write actions: rerun, dispatch, cancel, artifact mutation, check mutation.
- Webhook/event ingestion services.
- OAuth app flows.
- Hosted integration service.
- Distributed workers or production backend.
- Level 3/4 autonomy enablement.
- Generic live adapter execution from arbitrary workflow specs.

## 10. Final Recommendation

Proceed with internal Phase 2 read-only integration evaluation using fixture mode and opt-in maintainer live tests.

Do not announce a public read-only integration preview yet. First:

1. Keep adapter request policy-precheck provenance explicit as new adapter paths are added.
2. Persist or clearly separate adapter invocation/observability records from runtime audit/observability records in examples and docs.
3. Clarify or fix Jira live authentication.
4. Run at least one maintainer-owned live smoke test for each provider and record the exact supported auth/permission posture.

## Commands Run

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed. Live GitHub, Jira, and GitHub Actions tests were ignored by default as intended. |
| `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` | Passed |
| `cargo audit` | Passed |
| `npm ci` | Passed |
| `npm run check` | Passed |
| `npm run check:contracts` | Passed |
| `npm run check:integrations` | Passed |
| `npm audit --audit-level=moderate` | Passed, 0 vulnerabilities |

`npm run check:contracts` validated all checked-in examples through Rust validation. `npm run check:integrations` validated and ran the GitHub, Jira, and CI read-only fixture-backed examples through validate, run, approve, and inspect paths.
