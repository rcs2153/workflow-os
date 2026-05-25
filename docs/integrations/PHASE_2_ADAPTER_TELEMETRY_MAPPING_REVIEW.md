# Phase 2 Adapter Telemetry Mapping Review

Review date: 2026-05-25

Reviewed scope:

- Phase 2 read-only adapter contracts and provider adapters.
- Scoped runtime-visible adapter telemetry mapping.
- Local executor integration.
- Local state backend adapter telemetry storage.
- CLI `inspect` adapter telemetry output.
- GitHub, Jira, and CI read-only reference examples.
- Related tests and integration gate.

## 1. Executive Verdict

**Ready for internal read-only telemetry use.**

The scoped adapter telemetry mapping is correct enough for internal fixture-backed read-only evaluation. It maps adapter invocation and observability records into clearly named runtime-visible telemetry records, persists them through the local backend, exposes concise redacted summaries through `workflow-os inspect`, and keeps fixture/test authorization visibly distinct from runtime policy approval.

This is **not ready for public read-only telemetry preview** because the broader Phase 2 public-preview blocker remains unresolved: maintainer-owned live smoke evidence for GitHub, Jira, and GitHub Actions has not been recorded against approved non-sensitive resources.

The mapping does not justify write-capable adapters, generic live adapter execution, production telemetry export, SIEM/OpenTelemetry claims, distributed workers, production backends, or Level 3/4 autonomy.

## 2. Scope Discipline

Verdict: **Pass for internal use.**

Evidence:

- The mapping is limited to read-only adapter telemetry returned by controlled fixture-backed example handlers.
- The local executor accepts telemetry only as part of `SkillOutput`, and the current CLI fixture handlers are the paths that attach read-only adapter telemetry for GitHub, Jira, and CI examples.
- The mapping does not add a generic adapter execution path from workflow specs.
- Live adapter tests remain opt-in and ignored by default.
- Write operations remain unavailable or denied in adapter tests and integration gates.
- Docs explicitly state the mapping is not generic adapter execution, not live execution by default, not production telemetry export, and not SIEM/OpenTelemetry integration.

Accepted limitation:

- Because telemetry is returned by controlled local skill handlers, this is a scoped runtime mapping, not a general runtime adapter invocation framework.

## 3. Telemetry Completeness

Verdict: **Pass with non-blocking coverage gaps.**

Mapped runtime audit and observability records preserve these fields where available:

| Field | Status |
| --- | --- |
| adapter ID | Present |
| adapter kind | Present |
| action | Present |
| capability | Present |
| operation mode | Present |
| policy precheck provenance | Present |
| workflow ID | Present when run-scoped |
| workflow version | Present when run-scoped |
| schema version | Present when run-scoped |
| spec hash | Present when run-scoped |
| workflow run ID | Present when run-scoped |
| step ID | Present from execution plan |
| skill ID/version | Present from execution plan |
| actor/system actor | Present |
| correlation ID | Present |
| idempotency key | Present when supplied by request |
| success/failure | Present |
| error classification | Present on failures |
| duration/latency | Present |
| redaction metadata | Present |
| response summary/reference metadata | Present on audit record |
| source component | Present |

Non-blocking gaps:

- Runtime example tests assert the most important fields on persisted records, but they do not exhaustively assert every mapped field for every provider record.
- Example-backed persisted telemetry coverage is success-path focused. Adapter contract tests cover classified failure telemetry, but there is not yet a fixture example that persists failed adapter telemetry through the local executor.
- Adapter audit and observability telemetry are persisted as separate local records. There is no transactional pair guarantee between the two records; this is acceptable for local preview telemetry but should be revisited before production-grade telemetry storage.

## 4. Policy Provenance

Verdict: **Pass.**

Evidence:

- `AdapterRequest` carries explicit `AdapterPolicyPrecheck`.
- Request helpers no longer silently pre-authorize without provenance.
- Fixture authorization is represented as `FixtureTest` provenance.
- Tests cover missing precheck fail-closed behavior.
- Tests cover denied precheck preventing provider invocation.
- Tests cover runtime-policy allowed precheck for read-only adapter calls.
- Write operations remain denied or unavailable in GitHub, Jira, and CI adapter tests.
- Persisted telemetry includes policy-precheck provenance and inspect output shows it.

Result:

Fixture/test authorization is distinguishable from production policy approval. The mapping does not weaken the Phase 2 policy boundary.

## 5. Redaction And Privacy

Verdict: **Pass for internal fixture-backed use.**

Evidence:

- Adapter records store summaries, references, response size metadata, and redaction metadata rather than raw provider payloads by default.
- GitHub file reads are reference-oriented.
- Jira descriptions and comments are reference-oriented in fixture tests.
- CI log excerpts are bounded and redacted.
- Tests assert no token-like values appear in debug/audit/health-style paths.
- New runtime telemetry tests assert no token-like GitHub value, no raw CI secret fixture value, and no raw Jira body text appears in persisted adapter telemetry debug output.

Accepted limitations:

- Redaction is deterministic preview redaction, not enterprise DLP.
- Live provider payloads have not been smoke-tested, so public preview remains blocked.
- Operator review is still required before using private or sensitive provider resources.

## 6. Audit And Observability Relationship

Verdict: **Pass.**

The implementation and docs now describe adapter telemetry as:

- contract-level `AdapterInvocationRecord` and `AdapterObservabilityRecord` values produced by adapters;
- mapped by controlled fixture-backed examples into adapter-specific runtime telemetry records:
  - `AdapterRuntimeAuditRecord`
  - `AdapterRuntimeObservabilityRecord`
- persisted in the local filesystem backend by workflow run;
- visible through `workflow-os inspect` as concise redacted summaries and JSON fields;
- local artifacts only, not production telemetry export.

The mapping is intentionally adapter-specific runtime telemetry rather than first-class workflow state transitions in the workflow event log. This is acceptable for the current scope because the task did not authorize generic runtime adapter execution or production telemetry infrastructure.

## 7. Example Behavior

Verdict: **Pass.**

Reviewed examples:

- `examples/github-read-only-review-context`
- `examples/jira-read-only-intake-quality`
- `examples/ci-read-only-failure-summary`

Evidence:

- All three examples validate through the integration gate.
- All three examples run through fixture/mock mode.
- All three examples pause for approval and complete after approval.
- All three examples persist adapter audit and observability telemetry.
- `workflow-os inspect` shows adapter telemetry counts.
- No live provider credentials are required.
- Live mode remains skipped by default.
- Example docs distinguish fixture-backed adapter reads, deterministic local mock handlers, runtime-visible telemetry mapping, and unsupported production behavior.
- Examples do not perform writes, reruns, dispatches, comments, issue updates, branch creation, or provider mutation.

## 8. Test Quality

Verdict: **Good for internal use; not complete enough for production telemetry claims.**

Strong coverage:

- GitHub fixture-backed example persists two runtime adapter audit records and two runtime adapter observability records.
- Jira fixture-backed example persists three runtime adapter audit records and three runtime adapter observability records.
- CI fixture-backed example persists five runtime adapter audit records and five runtime adapter observability records.
- Tests assert operation mode, capability, run ID, correlation ID where checked, and fixture/test policy provenance.
- Tests assert no token-like or raw sensitive fixture values appear in persisted telemetry debug output.
- Adapter contract tests cover missing/denied policy prechecks, runtime-policy allowed prechecks, write denial, classified errors, redacted response metadata, health output, and live-test skip posture.
- `npm run check:integrations` validates examples, runs fixture-backed paths, approves runs, inspects output, and checks telemetry posture docs.

Weaknesses / gaps:

- Persisted runtime telemetry tests do not exhaustively assert every preserved field for every record.
- Runtime telemetry persistence is tested mainly on successful example paths.
- Corrupt adapter telemetry local artifacts are not covered by health-check tests.
- There is no test proving paired adapter audit and observability records are atomically persisted together. The current implementation does not claim that guarantee.

These are non-blocking for internal read-only telemetry use.

## 9. Documentation Honesty

Verdict: **Pass.**

Docs do not claim:

- public read-only preview readiness;
- production integration readiness;
- production audit export;
- SIEM or OpenTelemetry integration;
- generic adapter runtime execution;
- write support;
- live provider support by default.

Docs explicitly state:

- public read-only preview remains blocked by live smoke evidence;
- telemetry mapping is scoped to controlled read-only fixture-backed examples;
- fixture/test authorization is distinct from runtime policy authorization;
- live tests are opt-in;
- redaction is preview-grade and not enterprise DLP.

Historical review docs still describe prior limitations as historical findings. That is acceptable because they are dated review artifacts rather than current operator guidance.

## 10. Remaining Limitations

- Public read-only preview is still blocked by missing live smoke evidence.
- Telemetry is local/runtime-preview only.
- Fixture-backed examples are not generic adapter execution.
- Live adapter execution is not enabled by default.
- Adapter telemetry records are stored outside the workflow event log.
- Adapter audit and observability telemetry are not persisted as an atomic pair.
- Redaction is not enterprise DLP.
- Live tests are skipped by default.
- Production telemetry export, SIEM integration, and OpenTelemetry integration are not implemented.

## 11. Do-Not-Build-Yet List

Do not build or announce:

- GitHub writes.
- Jira writes.
- CI writes, reruns, or workflow dispatch.
- Webhooks.
- OAuth.
- Hosted integration service.
- Production backend.
- Distributed workers.
- Generic live adapter execution.
- Level 3/4 autonomy.
- Domain packs.

## 12. Commands Run And Results

All commands were run from the repository root.

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed. Live GitHub, Jira, and GitHub Actions tests were ignored by default. |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | Passed |
| `cargo audit` | Passed |
| `npm ci` | Passed |
| `npm run check` | Passed |
| `npm run check:contracts` | Passed |
| `npm run check:integrations` | Passed |
| `npm audit --audit-level=moderate` | Passed, 0 vulnerabilities |
| Validate all read-only examples | Passed through `npm run check:integrations` |
| Run all fixture-backed read-only examples | Passed through `npm run check:integrations` |
| Verify live tests skipped by default | Passed through `cargo test --workspace`; provider live tests were reported ignored |

Live tests were not run because safe credentials and approved non-sensitive live resources were not explicitly available in this review.

## 13. Final Recommendation

Keep the adapter telemetry mapping.

It is ready for internal read-only telemetry use and improves the Phase 2 fixture-backed examples without expanding runtime scope. The next useful hardening step is not writes or broader adapter execution; it is maintainer-owned live smoke evidence for GitHub, Jira, and GitHub Actions. After that evidence is recorded, run a public read-only integration preview readiness review before preparing any `0.2.0-preview.1` posture.
