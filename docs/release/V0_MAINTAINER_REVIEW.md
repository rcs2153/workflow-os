# Workflow OS v0 Maintainer Review

Status note: this review records the blocking issues found in the pre-hardening v0 tree. Several blockers named below have since been addressed in follow-up hardening work, including schema version in run/event identity, safer local append validation, approval/event-log consistency, approval-gated invocation ordering, durable pre-run policy audit, audit metadata completeness, CLI approval denial, schema/SDK/example contract checks, and explicit CLI mock-skill opt-in. Treat this document as the historical review baseline, not the current implementation status. The current public posture is **Workflow OS v0 local kernel preview**. See [V0_MAINTAINER_REVIEW_RERUN.md](V0_MAINTAINER_REVIEW_RERUN.md), [V0_READINESS.md](V0_READINESS.md), and [V0_KNOWN_LIMITATIONS.md](V0_KNOWN_LIMITATIONS.md) for the current readiness and limitation statements.

This review evaluates the current Workflow OS v0 working tree as a release candidate for the local-first kernel. It is intentionally strict: the question is not whether the codebase is promising, but whether it is honest, coherent, and safe to present as a serious v0 foundation.

## Executive verdict

**Not ready for public open-source release.**

Workflow OS v0 is credible as an internal kernel prototype for continued hardening. It has a strong architecture direction, a real Rust core, deterministic project loading and validation, an event-sourced run model, a local executor, approval/retry/escalation/policy paths, a CLI, a TypeScript spec-generation SDK, and a working vertical slice. The implementation is not yet ready to publish as an enterprise-grade open-source v0 because several runtime, audit, and contract invariants are documented more strongly than they are implemented.

## Review scope

Reviewed:

- `README.md`
- `docs/ENGINEERING_STANDARD.md`
- `docs/PROJECT_CHARTER.md`
- all ADRs under `docs/adr/`
- `docs/architecture/*`
- `docs/runtime/*`
- `docs/specs/*`
- `docs/cli/*`
- `docs/sdk/*`
- `docs/security/*`
- `docs/operations/*`
- `docs/release/*`
- Rust crates under `crates/workflow-core/` and `crates/workflow-cli/`
- TypeScript package under `packages/sdk-typescript/`
- `examples/vertical-slice-approval/`
- `.github/workflows/ci.yml`
- test suites in Rust and TypeScript

## Commands run

All commands were run from the repository root.

```sh
/bin/zsh -lc 'CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo fmt --all --check'
```

Result: passed.

```sh
/bin/zsh -lc 'CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo clippy --workspace --all-targets -- -D warnings'
```

Result: passed.

```sh
/bin/zsh -lc 'CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo test --workspace'
```

Result: passed.

Observed test coverage included CLI tests, vertical-slice example tests, adapter contract tests, local executor tests, policy tests, primitive tests, project loader tests, project spec tests, semantic validator tests, runtime event tests, and workflow/skill definition tests.

```sh
/bin/zsh -lc 'CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps'
```

Result: passed.

```sh
/bin/zsh -lc 'CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup PATH=/Users/rsegar/Documents/WorkflowOS/.tools/cargo/bin:$PATH cargo metadata --locked --format-version 1 >/tmp/workflow-os-cargo-metadata-review.json'
```

Result: passed.

```sh
/bin/zsh -lc 'PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm run check'
```

Result: passed.

```sh
/bin/zsh -lc 'PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm ci --ignore-scripts'
```

Result: passed.

```sh
/bin/zsh -lc 'CARGO_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/cargo RUSTUP_HOME=/Users/rsegar/Documents/WorkflowOS/.tools/rustup .tools/cargo/bin/cargo audit'
```

Result: passed after rerunning with network approval. The first sandboxed attempt could not fetch the advisory database.

```sh
/bin/zsh -lc 'PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache .tools/node-v20.19.5-darwin-arm64/bin/npm audit --audit-level=moderate'
```

Result: passed after rerunning with network approval. The first sandboxed attempt could not reach the npm registry.

## Top 10 blocking issues

1. **Run and event identity omit schema version.**

   The architecture docs require each run to reference schema version, workflow version, and spec content hash. `WorkflowRunIdentity` and `WorkflowRunEvent` carry run ID, workflow ID, workflow version, and spec content hash, but not schema version. This weakens the immutable run identity contract and creates ambiguity during future schema evolution.

2. **Local event append is not atomic as a complete durable operation.**

   The local backend writes the event file and then writes the event ID index. A failure between those writes can leave an event that exists by sequence but is not indexed by event ID. This violates the documented expectation that event append is atomic at the backend level where practical.

3. **The backend can persist invalid event streams if used directly.**

   State transition validation exists in rehydration, but the local event store itself only rejects duplicate event IDs and duplicate sequence numbers. It does not enforce next-sequence contiguity or transition validity before append. A corrupted or buggy caller can persist streams that cannot rehydrate.

4. **The CLI generic local skill handler makes arbitrary local skills appear executable.**

   `workflow-os run` registers a generic handler for every `local/*` skill and returns deterministic mock output. This is useful for the vertical slice, but it blurs the boundary between a real local skill implementation and a convenience mock. Public release should require explicit example/test handler registration or label this behavior more narrowly.

5. **Approval-gated execution emits `SkillInvocationRequested` before approval.**

   The executor emits `SkillInvocationRequested` during run start before checking approval gates. The docs say approval-gated steps must not execute before approval, and some docs imply skill invocation events are after approval. This may be acceptable if the event means "planned", but the current naming and docs make the runtime appear to request invocation before approval is granted.

6. **Pre-run policy decisions are not durably audited.**

   The executor evaluates start-workflow policy before appending `RunCreated`, but that decision is not recorded as a workflow event. Denied starts therefore have no durable event-level audit trail, even though policy decisions are documented as auditable.

7. **Approval projection can diverge from the event log.**

   Approval requests are saved to the approval store before the `ApprovalRequested` event is appended. A crash or backend failure between those operations can leave an approval projection without the corresponding event. That weakens the event log as the source of truth.

8. **Audit event completeness does not match the stated contract.**

   Audit events do not include schema version, skill version is not consistently populated from workflow events, and redaction is partly heuristic. The audit foundation is useful, but it does not yet meet the documented enterprise audit metadata contract.

9. **The CLI cannot deny approvals.**

   The runtime supports `ApprovalDenied`, but the primary v0 user interface only implements `workflow-os approve`, which grants. Denial behavior is tested in core code but not available through the CLI path that operators are expected to use.

10. **Schema and SDK synchronization is not enforced strongly enough.**

   JSON Schemas are checked in, TypeScript types are manually maintained, and contract tests cover representative fixtures. There is no CI gate proving Rust models, checked-in schemas, examples, and SDK emitted specs remain synchronized as a whole.

## Top 10 non-blocking issues

1. **The README opening line overstates maturity.**

   It calls Workflow OS an enterprise-grade framework before immediately qualifying v0 as a local-first kernel. The qualification is present, but the first impression is stronger than the implementation supports.

2. **The TypeScript SDK emits JSON text into `.yml` files.**

   JSON is valid YAML, but this conflicts with the stated human-authored YAML posture and may confuse contributors reading generated examples.

3. **Markdown checks are too shallow.**

   The docs check verifies required files, trailing whitespace, and empty Markdown files. It does not validate links, command snippets, schema examples, or cross-document consistency.

4. **`RunPaused` state transition logic can ignore event payload intent.**

   The event kind carries a pause status, but the transition mapping always targets `WaitingForExternalEvent`. This creates room for misleading event payloads.

5. **Retry policy defaults are implicit.**

   The runtime defaults missing retry max attempts to `2` in some paths. That is bounded, but enterprise validation would be clearer if retry bounds were explicit in specs for retry-enabled steps.

6. **CLI JSON output is not a documented stable machine contract.**

   JSON output exists, but enum strings and response shapes appear implementation-shaped rather than versioned CLI contracts.

7. **Observability is mostly local and in-process.**

   The hooks and events are useful, but latency, stuck workflow detection, and alerting are not yet operationally strong.

8. **The dependency posture is documented but still carries YAML parser risk.**

   The use of `serde_yaml` and its transitive parser risk is documented. It is acceptable for internal v0 work but should remain visible before public release.

9. **Some tests prove construction and happy-path compatibility more than adversarial behavior.**

   The suite is much better than placeholder-only testing, but fault injection, partial write failures, malformed event logs, and SDK/schema drift are under-tested.

10. **Tracked TypeScript build output requires a maintenance decision.**

    `packages/sdk-typescript/dist` appears intended for package distribution. If kept, CI should ensure it is regenerated or intentionally pinned.

## Architecture assessment

The architecture is directionally strong and mostly matches the charter. Rust owns the core runtime, schemas, validation, event model, state abstractions, policy model, and CLI. TypeScript is limited to spec authoring helpers and contract tests. The repository communicates a disciplined open-core model through a project manifest, declarative specs, local validation, CLI-first workflows, and a vertical-slice example.

The boundary around real integrations is mostly honest. GitHub, Jira, CI, generic HTTP, and other external systems are represented as future adapter contracts, not implemented clients. The docs consistently defer distributed workers, production database backends, hosted SaaS, UI, marketplace behavior, and Level 3/4 autonomy by default.

The largest architecture gap is contract fidelity: several docs describe stronger invariants than the data model enforces today. The missing schema version in run/event identity is the clearest example. The local executor also makes local mock execution too easy to mistake for real skill execution.

## Runtime correctness assessment

The runtime is not toy behavior, but it is not yet release-hard.

Positive findings:

- Run rehydration is deterministic for ordered event streams.
- Terminal state mutation is rejected by state transition logic.
- Duplicate sequence numbers are rejected by local and in-memory stores.
- Duplicate event IDs are rejected by state stores.
- Approval pause/resume has real runtime behavior and persistence.
- Retry and escalation paths are implemented and tested.
- Policy checks happen on the normal skill invocation path.
- Local backend rehydration proves restart-shaped behavior for happy paths.

Blocking concerns:

- Event append atomicity is incomplete in the local filesystem backend.
- Direct store writes can persist streams that later fail rehydration.
- Approval request projections can get ahead of the event log.
- Skill invocation request events occur before approval, creating semantic ambiguity.
- Pre-run policy decisions are not durably represented.
- Schema version is not part of run/event identity.

The current runtime is good enough for internal kernel iteration. It is not yet strong enough to advertise as a trustworthy public v0 runtime contract.

## Validation correctness assessment

Validation is one of the stronger areas of the codebase. The project loader discovers expected spec files, parses YAML, accumulates diagnostics, computes spec hashes, rejects unsupported schema versions, and catches obvious secret material in specs. The semantic validator catches missing skill references, duplicate IDs, missing triggers, missing steps, unbounded retry behavior, Level 3/4 defaults, unknown capabilities, missing approval on sensitive action, sensitive fields without redaction, and multiple diagnostics.

Important caveats:

- Source locations are useful but not yet editor-grade for every semantic path.
- Schema synchronization is not mechanically enforced.
- Some runtime expectations are enforced by validation rather than state append contracts.
- The validator is deterministic, but the docs should be careful not to imply full JSON Schema validation coverage for every field.

Invalid projects generally fail before execution through the CLI path. That is a real strength.

## Policy and governance assessment

The policy model is meaningfully enforced in the normal runtime path. Unknown capabilities fail closed, Level 3/4 is denied by default, `external.write` is denied in v0, kill switch behavior exists, and policy denials prevent skill execution.

The governance gap is audit durability and completeness. Policy decisions before skill invocation are represented, but pre-run policy decisions are not durably emitted as workflow events. The audit data model is also not complete enough for the stated enterprise audit contract.

Policy is not a no-op. It is real, but the audit story around it is incomplete.

## Security/privacy assessment

Positive findings:

- Rust crates deny unsafe code.
- Specs reject obvious secrets and secret-like fields.
- Sensitive wrapper types redact `Debug` and `Display`.
- Capability checks fail closed for unknown or unsupported actions.
- `external.write` is denied in v0.
- Security policy, threat model, and security review docs exist.
- Dependency audit commands pass.

Risks and gaps:

- Redaction is not uniformly schema-driven across audit and observability data.
- Audit events can include decision context and references that rely on heuristic redaction.
- Local skill handlers run in-process; this is acceptable for local v0 but must not be framed as sandboxed execution.
- The generic CLI local handler can create false confidence about skill isolation.
- YAML parsing supply-chain and parser risk remains documented but unresolved.

There is no obvious secret-printing behavior in tested paths, but the system should not be considered hardened against malicious project specs or malicious local skill code.

## Auditability and observability assessment

The foundations are present: audit sinks, observability sinks, local sink implementations, runtime integration, correlation IDs, policy decision events, approval events, retry/escalation events, and redaction tests.

The system is not yet genuinely enterprise-operable. Audit records are incomplete relative to the stated contract, local sinks are not durable operational infrastructure, and observability is not yet rich enough for real alerting or stuck workflow operations. This is acceptable for a local kernel if documented as a foundation, not a completed operations layer.

## CLI experience assessment

The CLI covers the right v0 surface:

- `validate`
- `run`
- `status`
- `approve`
- `inspect`
- `doctor`

The vertical slice can validate, run, pause for approval, resume, complete, inspect, and produce audit/observability artifacts. Diagnostics and exit behavior are usable.

Key gaps:

- Approval denial is not exposed.
- JSON output exists but is not yet a stable public contract.
- The generic local mock handler should be narrowed or documented more aggressively.
- CLI behavior depends on local-only assumptions that should remain explicit in help and examples.

A new user can run the vertical slice locally. They should not infer from it that arbitrary declared skills have real implementations.

## TypeScript SDK boundary assessment

The SDK stays on the correct side of the Rust/TypeScript boundary. It emits spec artifacts, does not execute workflows, does not implement adapters, does not implement a parallel runtime, and includes contract tests against Rust validation.

The main weakness is synchronization. Types are manually aligned with checked-in schemas and Rust models. Contract tests are valuable but representative rather than comprehensive. Before public release, CI should prove SDK-generated examples, checked-in schemas, and Rust validation remain aligned.

## Open-source readiness assessment

The repository has the expected OSS governance files:

- license
- code of conduct
- contributing guide
- security policy
- vulnerability reporting guidance
- maintainers file
- changelog
- semver policy
- release process
- ADR process
- roadmap
- issue templates
- PR template
- known limitations
- CI workflow

Governance readiness is good. Technical readiness is not yet sufficient for public release. The project can be shared privately or internally for design review and kernel hardening, but public release should wait until the blocking runtime/audit/schema-contract issues are fixed.

## Test quality assessment

The test suite is substantial and not placeholder-only. It covers real behavior across primitives, project loading, validation, runtime events, local backend contracts, local executor behavior, approvals, retry/escalation, policy, audit/observability, CLI behavior, SDK generation, and the vertical slice example.

Weak spots:

- Backend fault-injection is limited.
- Partial-write and corrupted-index scenarios are not sufficiently tested.
- Schema/SDK/Rust synchronization is not exhaustively tested.
- CLI JSON output is not contract-tested as a stable interface.
- Some SDK tests focus on generated shape rather than adversarial invalid inputs.
- Audit completeness is tested for emission, not for full required metadata coverage.

Overall, test quality is above average for an early codebase, but the missing tests map directly to the highest-risk blockers.

## Documentation honesty assessment

The documentation is broad, coherent, and mostly honest. It consistently says v0 is local-first, defers real external adapters, defers distributed workers, defers hosted SaaS and UI, and disables Level 3/4 autonomy by default.

Overclaims or inconsistencies:

- Some runtime docs state invariants that are not fully represented in event metadata, especially schema version.
- Atomic append and durable-state language is stronger than the local filesystem implementation.
- Audit metadata requirements are stronger than the current audit event model.
- The README's opening framing should be softened until the blockers are fixed.
- The vertical slice should make the generic local mock handler boundary unmistakable.

The docs are not marketing fluff, but they need another honesty pass after the blockers are addressed.

## Required fix plan

1. **Fix run/event identity contract.**

   Add schema version to run identity, events, snapshots, audit projections, CLI inspect output, docs, and tests. Add compatibility notes for existing local state.

2. **Harden event append semantics.**

   Make local append atomic as a unit where practical. Write through a temporary record, validate expected sequence, update indexes deterministically, and add crash/corruption tests.

3. **Move transition validation closer to append.**

   Add an append API that validates next sequence and transition against current durable history. Keep lower-level raw append internal or clearly test-only.

4. **Fix approval projection ordering.**

   Append `ApprovalRequested` first, then update approval projection, or make projection rebuildable and self-healing from the event log. Add failure-mode tests.

5. **Clarify skill invocation event semantics.**

   Either move `SkillInvocationRequested` after approval grant or rename/split the pre-approval event into a scheduling/planning event. Update state machine docs and tests.

6. **Durably audit all policy decisions.**

   Record pre-run policy allow/deny decisions, including denied starts. Define how audit exists when no run has been created yet.

7. **Complete audit metadata.**

   Add schema version, skill version, policy decision references, and systematic redaction metadata. Add tests that assert required audit fields, not just event existence.

8. **Expose approval denial in the CLI.**

   Add a small, explicit CLI path such as `workflow-os approve --deny` or `workflow-os deny`, document it, and test denial behavior end to end.

9. **Strengthen schema and SDK synchronization gates.**

   Add CI checks that validate examples and SDK-generated artifacts against checked-in schemas and Rust validation. Decide whether schemas are generated or manually versioned with strict contract fixtures.

10. **Narrow or rename the generic local mock handler.**

    Make it explicit example/test behavior, or require project-local handler registration. Prevent users from thinking every `local/*` skill has a real implementation.

## Do-not-build-yet list

Do not build these until the blockers above are fixed:

- real GitHub adapter
- real Jira adapter
- real CI adapter
- generic HTTP adapter
- distributed workers
- production Postgres backend
- Redis, SQS, NATS, or distributed locking
- hosted SaaS
- UI
- marketplace or package registry
- Level 3/4 autonomy enablement
- secret provider integrations
- enterprise RBAC or IdP integration
- docs generation
- workflow branching expansion beyond the validated local kernel path

## Final recommendation

Historical recommendation at the time of this review: treat this repository as a strong internal v0 kernel candidate, not as a public release candidate.

That hardening sequence has since been completed and rerun in [V0_MAINTAINER_REVIEW_RERUN.md](V0_MAINTAINER_REVIEW_RERUN.md). The current public framing is a local kernel preview, not a production runtime or broad release candidate.
