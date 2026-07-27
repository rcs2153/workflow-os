# Current-Authority Same-Call Use Boundary Report

## 1. Executive Summary

Workflow OS now has a private Core-owned same-call use boundary for the
registered current-authority source.

One call reads and resolves the current registered source, stops before use
when source or authority posture is not ready, borrows one private capability
only for one `FnOnce` consumer invocation, and returns one bounded payload-free
outcome. The callback invocation itself is the governed use. The capability
does not expose generic authorization, execution, dereference, target, or
repeatable operation methods.

This phase proves same-call non-reuse only. It does not claim durable replay
prevention, consumer idempotency, production time-of-use readiness, or runtime
execution.

## 2. Scope Completed

- Added one crate-private current-authority use input.
- Added bounded use, consumer-result, and use-outcome postures.
- Added one private borrowed use capability.
- Added one crate-private resolve-and-use method on the registered in-memory
  source.
- Reused the accepted registered-source resolver for every call.
- Invoked one Core-owned `FnOnce` consumer only when the fresh assessment was
  `Ready`.
- Kept blocked and source-failure paths from invoking the consumer.
- Preserved explicit consumer success, failure, and ambiguous-outcome
  categories.
- Added redaction-safe Debug output for the private capability and outcome.
- Added focused unit tests.

## 3. Scope Explicitly Not Completed

This phase does not add:

- a public readiness, authority, or capability API;
- a bearer token, TTL lease, session, nonce, or reusable authority handle;
- durable replay prevention or an authority-use ledger;
- consumer idempotency or ambiguous-outcome reconciliation;
- executor or runtime integration;
- target dereference or payload access;
- persistence, events, audit projection, artifacts, schemas, SDKs, CLI, or UI;
- providers, OpenShell, sandbox execution, SideEffect execution, or writes;
- hosted or distributed behavior;
- enterprise identity or administration;
- cryptographic receipts;
- reasoning lineage;
- dependencies; or
- release posture changes.

## 4. Boundary Summary

`RegisteredCurrentAuthorityUseInput` carries the exact immutable execution
binding, exact required-context contract, one injected evaluation timestamp,
and validated redaction metadata.

`use_current_authority` calls the existing private
`resolve_current_authority` path. Source failures become bounded source-failure
outcomes. Non-ready assessments become `BlockedBeforeUse`. Neither path
constructs a use capability or invokes the consumer.

Only a fresh `Ready` assessment constructs
`RegisteredCurrentAuthorityUseCapability<'call>`. The capability borrows the
assessment for the lexical lifetime of the call and is passed to one
`FnOnce` consumer. It has no operation methods. The consumer returns one
bounded result category, which the boundary maps to:

- `ConsumerSucceeded`;
- `ConsumerFailed`; or
- `ConsumerOutcomeAmbiguous`.

## 5. One-Time-Use And Replay Posture

The implementation establishes these bounded properties:

- the callback is invoked at most once for one call;
- the borrowed capability cannot outlive the call through the public API;
- the capability is not cloneable or serializable;
- the capability exposes no repeatable privileged operation;
- every later call reruns registered-source selection, freshness,
  capability resolution, projection, and required-context consumption; and
- ambiguous consumer completion remains distinguishable from known failure.

The implementation does not establish:

- cross-call, cross-thread, cross-worker, or cross-process replay prevention;
- atomic durable consumption;
- a stable use-operation identity;
- automatic retry safety;
- consumer idempotency; or
- reconciliation after an ambiguous consumer outcome.

Those properties require separately reviewed authoritative persistence and
consumer-specific semantics.

## 6. Privacy And Error Handling

The private capability and outcome contain no target payload, command output,
provider response, credential, environment value, source file, raw log, or
SideEffect payload.

Capability Debug output exposes only bounded posture, reason count, and a
redacted commitment marker. Outcome Debug output exposes only typed bounded
posture and reason/failure categories. It does not expose actor, workflow, run,
step, harness, target, contract, report, timestamp, source, or commitment
values.

Existing source and resolver validation errors retain their stable
non-leaking behavior. Source failure does not invoke the consumer.

## 7. Test Coverage

Focused tests prove:

- ready authority invokes one bounded consumer exactly once;
- blocked authority never invokes the consumer;
- source failure never invokes the consumer;
- consumer failure remains explicit;
- ambiguous consumer completion remains explicit;
- repeated calls each perform a fresh resolve-and-use call;
- the private capability and outcome have redaction-safe Debug output; and
- existing registered-source tests continue to pass.

Rust module privacy, the borrowed lifetime, absence of `Clone`/serde derives,
and the `FnOnce` callback enforce the non-export and same-call shape. No new
compile-test dependency was added.

## 8. Validation Commands And Results

- focused registered-source unit tests: passed, 18 tests.
- focused `workflow-core` clippy with all targets and warnings denied: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed before the governed phase close.

## 9. Remaining Known Limitations

- The source and use boundary remain private and in-memory only.
- No runtime consumer exists.
- The test consumer performs no target access or external work.
- No independent prerequisite fact source is integrated.
- Durable replay prevention remains unimplemented.
- Ambiguous consumer outcomes are classified but not reconciled.
- Proportional governance cannot consume this boundary yet.
- OpenShell remains a separate optional execution-provider concern.

## 10. Recommended Next Phase

Perform a focused maintainer review of the private same-call use boundary.

The review should verify that fresh resolution occurs for every call, only
`Ready` invokes the consumer, the callback itself is the single governed use,
the capability cannot become a general-purpose authority object, Debug and
errors remain non-leaking, and no durable replay or idempotency claim is made.

Do not add executor integration, persistence, events, providers, OpenShell,
sandbox execution, SideEffect execution, schemas, CLI behavior, or writes.

## 11. Governed Phase Record

- workflow: `dg/implement`
- run ID: `run-1785173747485844000-2`
- approval ID:
  `approval/run-1785173747485844000-2/implementation-approved`
- approval presentation ID: `presentation/c5660bc6f140c9ad`
- approval presentation content hash:
  `c5660bc6f140c9ad5061b39d1ac728a25adba078af8169bda986266270404378`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof persisted before approval
- out-of-kernel work: the delegated maintainer inspected and edited the
  implementation, tests, roadmap, plans, and report; the kernel governed scope
  and approval but did not inspect code, edit files, execute checks, or mutate
  git
