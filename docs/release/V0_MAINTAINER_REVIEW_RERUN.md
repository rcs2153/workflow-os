# v0 Maintainer Review Rerun

Date: 2026-05-25

This review reruns the senior-maintainer assessment after the blocker-fix hardening work. It verifies the previous blockers, rechecks the local-first v0 kernel boundary, and records the exact validation commands run.

Status note: this review concluded that the runtime was ready for internal v0 kernel use but that public release posture had not yet been defined. The repository now defines that posture as **Workflow OS v0 local kernel preview**: a public preview of the local-first kernel, not a production enterprise runtime or broad release candidate.

## Executive Verdict

**Ready for v0 internal use and suitable for a public v0 local kernel preview.**

The blocker fixes substantially improved runtime correctness, auditability, CLI honesty, schema identity, and contract gates. The evidence supports public preview of the local kernel only. It does not support a production deployment claim, adapter-complete claim, hosted product claim, production distributed runtime claim, or Level 3/4 autonomy claim.

## Commands Run

All commands were run from the repository root.

| Command | Result |
| --- | --- |
| `CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo fmt --all --check` | Pass |
| `CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo test --workspace` | Pass: CLI, vertical slice, core unit tests, adapters, local executor, policy, primitives, loader, specs, validation, runtime events, and workflow/skill model tests all passed |
| `CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | Pass |
| `CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo audit` | Pass after approved network access to fetch RustSec advisory database |
| `PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm ci` | Pass |
| `PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm run check` | Pass: docs, TypeScript typecheck/lint/test, and schema/SDK/example contracts |
| `PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm audit --audit-level=moderate` | Pass after approved network access to npm registry: 0 vulnerabilities |
| `PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm run check:contracts` | Pass |
| `PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm run check:docs` | Pass |
| `CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo build -p workflow-cli --bin workflow-os` | Pass |
| `target/debug/workflow-os --project-dir examples/vertical-slice-approval validate` | Pass: `Project is valid.` |
| `CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo metadata --locked --format-version 1` | Pass |
| `PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm ci --ignore-scripts` | Pass |

`cargo audit` and `npm audit` initially failed under restricted network access and were rerun with approval. Both completed successfully.

## Previous Blocker Status

| Blocker | Status | Evidence | Remaining Gap |
| --- | --- | --- | --- |
| 1. Run/event identity includes schema version | Fixed | `WorkflowRunIdentity`, `WorkflowRunEvent`, `WorkflowRunSnapshot`, CLI status/inspect JSON, runtime event tests, backend mismatch tests, and docs all include schema version. `cargo test --workspace` includes `all_events_retain_schema_version_identity`, `mismatched_schema_version_is_rejected`, and `persisted_event_without_schema_version_is_rejected`. | Existing legacy local state without schema version is rejected rather than migrated. This is documented and acceptable for v0. |
| 2. Local event append atomicity/corruption behavior hardened | Fixed with local-backend limits | Local backend writes files through temporary synced files and publish semantics, validates event/index consistency before append, verifies reads, reports missing index, dangling index, corrupt event files, and corrupt rehydration. Tests cover missing event ID index, index without event file, corrupt state, and corrupt stream. | Event append is not a transactional multi-file commit. A crash can leave a dangling index, but it is detected on health check/read/next append rather than silently accepted. Docs state this limitation. |
| 3. Transition validity enforced at append boundary | Fixed | `append_event` validates existing durable history before persistence, including contiguous sequence, transition, terminal mutation, run identity, schema version, workflow version, and spec hash. Contract tests cover invalid transition, terminal mutation, non-contiguous sequence, duplicate sequence, duplicate event ID, and identity mismatches. | Direct filesystem tampering can still corrupt local state; defensive read/rehydration catches it. |
| 4. Approval projection event-log authoritative or rebuildable | Fixed | Approval request event is appended before projection save; approval projection is documented as cache; approval decisions validate against event-derived approval state; tests cover rebuild from event log, missing projection with event, projection without event, terminal approval rejection, and restart survival. | There is no standalone repair CLI; rebuild happens through runtime paths. |
| 5. Approval-gated event semantics unambiguous | Fixed | `StepScheduled` and `ApprovalRequested` happen before approval; `SkillInvocationRequested` is emitted only after approval grant and resume. Docs explicitly define `SkillInvocationRequested` as post-gate authorization. Tests cover no invocation requested/started before approval and correct post-grant order. | Multi-step/branching execution is still out of scope. |
| 6. Policy decisions, including denied starts, durably audited | Fixed | `PolicyAuditRecord` and `PolicyAuditStore` record pre-run and run-scoped policy decisions. Denied starts are audited without creating misleading workflow runs. Tests cover allowed start, denied start, skill allow/deny, approval-required, kill switch, required metadata, and audit sink failure behavior. | Policy audit is local filesystem durability, not production audit infrastructure. |
| 7. Audit metadata and redaction match documented contract | Fixed for v0 | `AuditEvent` includes schema version, workflow version, spec hash, step, skill, skill version, actor, action, policy reference, correlation ID, idempotency key, redaction metadata, and source component where derivable. Tests assert metadata completeness and redaction behavior for run creation, policy, approval, retry, escalation, sensitive input/output, and Debug/Display redaction. | Deep payload inspection and schema-driven nested redaction are still deferred. Audit sink persistence is local/in-memory except durable policy audit and event-log-derived projections. Docs state this. |
| 8. Approval denial exposed through CLI | Fixed | `workflow-os approve <run-id> <approval-id> --deny --reason ...` exists, requires denial reason, captures actor, emits runtime/audit records, fails closed, and has JSON output. CLI tests cover denial transition, actor/reason capture, event emission, audit emission, no skill execution, terminal rejection, duplicate handling, grant-after-denial behavior, and JSON output. | No separate `deny` command exists; the chosen interface is `approve --deny`, which fits current CLI shape. |
| 9. Rust/schema/SDK synchronization gates exist and run in CI | Fixed | `scripts/check-contracts.mjs` validates required schemas, schema version pins, all checked-in examples, SDK-generated minimal and approval-gated projects, SDK invalid fixture failure, and schema mismatch failure. CI has a dedicated `contracts` job. `npm run check` and `npm run check:contracts` both passed locally. | Schemas and TypeScript types are still manually maintained, with contract gates rather than generated source-of-truth types. This is documented. |
| 10. Generic local mock handler behavior explicit and safe | Fixed | CLI no longer registers mock handlers by default. `--mock-all-local-skills` is explicit in help, docs, README, example README, tests, and vertical slice commands. Unregistered local skill execution fails clearly. | No real plugin or handler discovery system exists. Tests/examples use explicit mock registration or flag. |
| 11. Documentation overclaims corrected | Fixed | README, charter, runtime docs, state backend docs, event model, approvals, policy, audit/observability, CLI docs, SDK docs, vertical slice docs, readiness, known limitations, and security review now distinguish implemented, mocked, local-only, future, and unsupported behavior. Searches confirmed no current docs claim real GitHub/Jira/CI adapters, distributed workers, production backends, UI, or Level 3/4 defaults. | The historical `V0_MAINTAINER_REVIEW.md` still lists old blockers, but now carries a status note marking it as pre-hardening baseline. |

## New Blockers

No new blockers were found for **v0 internal use** of the local-first kernel.

The project should not be released as a broad v0 release candidate or production runtime until maintainers intentionally resolve or accept the remaining release-position gaps:

1. The release must be labeled as a local kernel preview, not a production runtime or broad release candidate.
2. Historical at rerun time: package/crate versions still used placeholder preview metadata; this has since been addressed for the local kernel preview with `0.1.0-preview.1`.
3. JSON Schemas and TypeScript types are manually synchronized rather than generated from Rust.
4. The local filesystem backend and local audit/observability sinks are development infrastructure, not production operational infrastructure.
5. The YAML parsing dependency strategy still includes deprecated `serde_yaml` and transitive `unsafe-libyaml`; this is documented as a v0 risk to revisit before production-readiness claims.

## Non-Blocking Issues

1. `serde_yaml` is deprecated and pulls in `unsafe-libyaml`. `cargo audit` passes, but this remains a production-readiness risk.
2. Local event append is strongly hardened but not transactional across event and index files.
3. No automatic local state repair command exists; corruption is detected clearly but not fixed by tooling.
4. `LocalAuditSink` and `LocalObservabilitySink` are in-process sinks for tests/development. Workflow events and policy records are durable, but there is no durable general audit export backend.
5. SDK/Rust/schema synchronization depends on strict contract tests, not generated TypeScript types from Rust.
6. Full nested input/output contract validation is not implemented in the executor.
7. Redaction avoids raw payload storage and tests sensitive paths, but deep payload-level DLP is out of scope.
8. No active timeout scheduler, trigger ingestion, external event listener, or stuck workflow detector exists.
9. Escalated runs do not have an operator resume/resolve CLI in v0.
10. CLI JSON output exists but is not separately versioned as a stable machine-output contract.

## Runtime Correctness Assessment

Runtime correctness is now credible for the stated v0 scope: a single-step local executor over a validating event-sourced run model and local durable backend. The event model carries immutable run identity including schema version, workflow version, and spec hash. Rehydration is deterministic and defensive. Append-time validation prevents normal runtime/backend paths from creating invalid streams. Terminal state mutation is rejected. Approval gates are enforced by event order and runtime behavior, not only docs.

Retry, escalation, cancellation, idempotency, and approval behavior are covered by behavior tests, not object-construction tests. The remaining runtime limitations are mostly scope limitations: no distributed workers, no real triggers, no active timer scheduler, no multi-step branching executor, no production backend, and no real adapters.

## Validation Correctness Assessment

The loader and validator remain one of the stronger parts of the repository. They parse v0 YAML specs, reject missing and unsupported schema versions, accumulate diagnostics, compute content hashes, reject secret-like spec data, catch duplicate IDs, validate skill references, validate retry and approval policy shape, reject Level 3/4 execution by default, and enforce capability/sensitive-field requirements before execution.

Invalid projects reliably fail before the local executor proceeds in the tested paths. Source-aware diagnostics exist and are suitable for the future CLI/editor path, though the docs/link checker is intentionally shallow.

## Policy And Governance Assessment

Policy is actually enforced in the runtime path. The executor records policy decisions before run start, before approval request, and before skill invocation. Denied pre-run starts create durable policy audit records without creating fake runs. Unknown capabilities, Level 3/4 execution, external writes, adapter invocation, missing context, and kill-switch execution are fail-closed in tests.

This is not enterprise RBAC, IdP integration, or secret-provider governance. It is a conservative kernel policy model with evidence-backed runtime enforcement.

## Security And Privacy Assessment

The security posture is coherent for local v0 kernel development. Rust workspace lints deny unsafe code, and both crates deny unsafe code. Secret-like specs are rejected. Redaction wrappers and audit projections avoid obvious sensitive disclosure. Runtime policy denies external writes and adapter invocation in v0. Audit records prefer references and summaries over raw sensitive payloads.

The biggest security risks remain correctly documented rather than hidden: YAML parser dependency risk, trusted local skill handler code, local filesystem state without encryption/tamper evidence/access control, no secret provider, no production audit sink, and no sandbox for untrusted handlers.

## Audit And Observability Assessment

Audit and observability are meaningfully present for v0. Workflow events project into audit events with identity, actor, correlation, idempotency, step/skill metadata, policy references, and redaction metadata. Policy decisions are also recorded in a durable local policy audit ledger, including denied starts before `RunCreated`. Approval, retry, escalation, skill success/failure, and policy decisions are covered by tests.

The implementation is still local-only. `LocalAuditSink` and `LocalObservabilitySink` are not production stores. There is no SIEM, OpenTelemetry exporter, retention policy, tamper-evident log, or production metrics backend. Docs now say that plainly.

## CLI Experience Assessment

The CLI is usable for v0 local workflows: `validate`, `run`, `status`, `approve`, `inspect`, and `doctor` are documented and tested. JSON output exists for supported paths. Approval denial is now available through `approve --deny`, requires a reason, and fails closed. `--mock-all-local-skills` makes mock execution explicit, and missing local handlers fail clearly by default.

The vertical slice can be validated and run locally with the documented commands. The CLI does not pretend to support real integrations, production workers, or arbitrary local skill implementations.

## TypeScript SDK Boundary Assessment

The TypeScript SDK remains an authoring/spec-generation layer. It does not execute workflows, evaluate policy, validate runtime behavior, or implement adapters. Contract fixtures validate SDK-generated projects through the Rust CLI. CI checks examples, schemas, SDK-generated valid fixtures, SDK-generated invalid fixtures, and schema-version mismatch behavior.

The boundary is acceptable for v0. The remaining weakness is manual synchronization of TypeScript types and checked-in JSON Schemas instead of generation from Rust-owned models.

## OSS Readiness Assessment

Governance files are present: license, code of conduct, contributing guide, security policy, vulnerability disclosure, changelog, maintainers, roadmap, ADR process, semantic versioning policy, release process, issue templates, and PR template.

CI quality gates are substantially better than the first review: Rust fmt/clippy/test/doc/lockfile, TypeScript install/typecheck/lint/test, docs check, npm lockfile validation, dependency audits, and schema/SDK/example contracts.

The repository is ready for contributor/internal kernel development and for a clearly labeled public local kernel preview. It is not yet ready to be represented as a broader public release candidate or production-ready runtime because package versions, manual schema sync strategy, local-only operations, and documented dependency/security risks still need deliberate maintainer acceptance.

## Test Quality Assessment

The test suite is substantial and behavior-oriented. It covers:

- primitive IDs, diagnostics, redaction, hashing, and timestamps
- project loading and parse diagnostics
- semantic validation categories
- runtime events and rehydration
- append-boundary validation and backend corruption detection
- approval event/projection consistency
- approval-gated event ordering
- retries, escalation, cancellation, and idempotency
- policy enforcement and durable policy audit
- audit metadata completeness and redaction
- CLI validate/run/status/approve/inspect/doctor behavior
- vertical-slice behavior
- adapter contracts
- SDK/Rust validation contracts

Weak spots are mostly intentional v0 scope gaps: no production backend fault model, no generated-schema equivalence proof, no deep contract validation, no distributed concurrency stress tests, and no real adapter tests.

## Documentation Honesty Assessment

Documentation is now much more honest. It consistently identifies v0 as a local-first kernel foundation and explicitly defers real GitHub/Jira/CI adapters, distributed workers, production database backends, hosted SaaS, UI, marketplace behavior, real secret providers, and Level 3/4 autonomy by default. Mock local skill behavior is now called out in README, CLI docs, local executor docs, skill-handler docs, the vertical-slice README, and known limitations.

The old `V0_MAINTAINER_REVIEW.md` is retained as historical baseline and now has a status note. That is acceptable, but future release readers should be pointed to this rerun review and the readiness/known-limitations docs.

## Final Recommendation

Proceed with v0 internal kernel use and targeted hardening. Do not build new broad features until the remaining local kernel limitations are either accepted or resolved.

Before a broader public v0 release candidate, do these small, reviewable tasks:

1. Decide when the project should move beyond "local kernel preview" into release-candidate posture, then update release docs accordingly.
2. Apply real crate/package versions and update `CHANGELOG.md`.
3. Decide whether manual schema/TypeScript synchronization is acceptable for public v0 or replace it with generated artifacts.
4. Decide whether the deprecated YAML parser risk is acceptable for public v0; document the maintainer decision.
5. Add a durable local audit export/reconstruction command or explicitly accept event-log-derived audit reconstruction for v0.
6. Add a troubleshooting command or documented procedure for local state corruption inspection.
7. Version CLI JSON output or mark it experimental.

## Do Not Build Yet

- Real GitHub adapter.
- Real Jira adapter.
- Real CI adapter.
- Generic HTTP adapter.
- Distributed workers.
- Production Postgres/Redis/queue backends.
- Hosted SaaS.
- UI.
- Marketplace/package registry.
- Level 3/4 autonomy enablement.
- Untrusted skill handler sandboxing as a side quest before the local kernel release posture is settled.
