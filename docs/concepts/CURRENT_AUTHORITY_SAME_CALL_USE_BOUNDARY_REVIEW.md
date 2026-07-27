# Current-Authority Same-Call Use Boundary Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The private boundary correctly reruns registered-source resolution for every
call, invokes one borrowed `FnOnce` consumer only for a fresh `Ready`
assessment, keeps source-failure and blocked paths non-invoking, and returns
bounded explicit consumer outcomes. It does not expose a public readiness
result or claim durable replay prevention.

Proceed to direct negative-path and fixed-vector hardening before planning a
real read-only runtime consumer.

## 2. Scope Verification

The phase stayed within the approved private model/helper and test scope.

It did not add a public authority API, reusable authority handle, TTL lease,
persistence, durable replay record, executor integration, target dereference,
provider call, OpenShell integration, sandbox execution, SideEffect
execution, write behavior, runtime event, artifact, schema, SDK, CLI, UI,
dependency, hosted behavior, or release change.

## 3. Boundary Assessment

`RegisteredCurrentAuthorityUseInput` accepts the exact execution binding,
required-context contract, evaluation timestamp, and redaction metadata. The
helper delegates to the existing registered-source resolution path instead of
accepting caller-asserted readiness or a caller-built fact set.

The helper constructs `RegisteredCurrentAuthorityUseCapability<'call>` only
after a fresh `Ready` assessment. The capability borrows that assessment, has
no public export, has no clone or serialization implementation, and exposes no
authorization, execution, dereference, target, or repeatable operation method.

The `FnOnce` callback is invoked at most once. In this test-only slice, that
callback invocation is the governed use.

## 4. Ready, Blocked, And Failure Semantics

The behavior is deterministic and appropriately separated:

- `Ready` constructs the borrowed capability and invokes one consumer.
- `Blocked` returns `BlockedBeforeUse` and never invokes the consumer.
- source failure returns bounded source-failure kind and posture without
  invoking the consumer.
- known consumer failure returns `ConsumerFailed`.
- uncertain completion returns `ConsumerOutcomeAmbiguous`.

The outcome does not collapse ambiguous completion into known failure or
silent retry eligibility. No automatic retry is implemented.

## 5. Same-Call And Replay Assessment

The implementation proves a narrow same-call property:

- no prior assessment is accepted as input;
- every helper call reruns source-backed resolution;
- one helper call invokes at most one callback;
- the borrowed capability cannot be returned through the public API; and
- the capability itself has no repeatable privileged operation.

It correctly does not prove:

- cross-call or cross-process replay prevention;
- atomic durable consumption;
- consumer idempotency;
- worker-restart protection;
- ambiguous-outcome reconciliation; or
- safe retry of a real external operation.

Those claims remain blocked on separately reviewed persistence and
consumer-specific boundaries.

## 6. Core-Owned Consumer Assessment

The current generic callback is acceptable only as a private test seam because
it receives a capability with no operation methods and no real consumer exists.

Before runtime integration, replace or specialize that seam with one concrete
Core-owned read-only operation. Do not expose the callback as a public handler
surface or give the borrowed capability generic methods that could repeat
privileged work inside one callback.

This is a non-blocking follow-up because the current method remains private,
has no runtime caller, performs no dereference, and cannot reach a provider.

## 7. Privacy And Error Assessment

Capability Debug output exposes bounded posture and reason count while
redacting the assessment commitment. Outcome Debug output exposes typed
posture and bounded reason/failure categories only.

Tests confirm that actor, report, run, timestamp, and other fixture identities
do not appear in capability or outcome Debug output. The boundary stores no
payload, source contents, command output, provider response, credential,
environment value, target contents, or raw log.

Existing source/resolver errors remain stable and non-leaking. The phase does
not add consumer-provided error text.

## 8. Test Quality Assessment

Focused tests cover:

- one ready consumer invocation;
- blocked non-invocation;
- source-failure non-invocation;
- explicit consumer failure;
- explicit ambiguous completion;
- fresh resolve-and-use calls for repeated uses; and
- redaction-safe capability and outcome Debug behavior.

Existing registered-source and resolver tests continue to cover exact
identity, source completeness, freshness, canonical ordering, revoked grants,
unresolved prerequisites, and non-leaking errors.

Rust privacy and type shape provide useful compile-time enforcement. A
compile-fail dependency is not justified while the boundary remains private.

## 9. Validation Assessment

The reviewed worktree passed:

- focused registered-source unit tests: 18 passed;
- focused `workflow-core` clippy with all targets and warnings denied;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

The broad workspace validation ran after the code change. Review-only
documentation was validated again before phase close.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add direct later-use invalidation tests at the use-boundary level for
  expiry, revocation, changed binding/contract, and unresolved prerequisites.
- Add fixed bounded outcome vectors only where compatibility requires them.
- Keep the generic callback private and test-only.
- Before any real consumer, replace or specialize it with one concrete
  Core-owned read-only operation.
- Revisit compile-fail privacy tests only if visibility broadens.
- Do not claim durable replay prevention before authoritative persistence and
  atomic consumption exist.

## 12. Recommended Next Phase

Direct current-authority use-boundary negative-path and fixed-vector
hardening.

This is the smallest next phase because it strengthens the accepted private
boundary against later-use invalidation before introducing a real consumer.
It must remain private and test-focused, with no executor, provider, OpenShell,
sandbox, SideEffect, write, persistence, event, schema, CLI, dependency, or
release work.

## 13. Governed Review Record

- workflow: `dg/review`
- run ID: `run-1785177083493325000-2`
- approval ID:
  `approval/run-1785177083493325000-2/review-scope-approved`
- approval presentation ID: `presentation/0149bb0839073b1a`
- approval presentation content hash:
  `0149bb0839073b1ab8c4a4b3c77673995546aab58af4bdad057ca17b221a906e`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof persisted before approval
- out-of-kernel work: the delegated maintainer inspected the implementation,
  tests, plans, roadmap, and report; the kernel governed scope and approval but
  did not inspect code, execute validation, edit files, or mutate git
