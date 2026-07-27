# Current-Authority One-Time-Use And Replay Posture Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the private same-call use-boundary implementation.

The plan selects the correct safety posture: re-resolve current authority for
every use and continuation path, keep the first use boundary private and
lexically bounded, and make no durable replay-prevention claim before
authoritative persistence and atomic consumption exist.

## 2. Scope Verification

The planning phase stayed within documentation-only scope.

It did not add Rust models or helpers, a public readiness contract, reusable
authority leases, persistence, executor integration, target dereference,
providers, OpenShell, sandbox execution, SideEffect execution, writes, events,
artifacts, schemas, SDKs, CLI behavior, hosted behavior, dependencies, or
release changes.

## 3. Architecture Decision Assessment

Rejecting a reusable TTL-based authority result is correct. Source freshness
answers whether facts may be assessed at one evaluation time; it does not turn
the resulting assessment into permission that may be cached or reused.

The plan correctly separates:

- source freshness;
- assessment currency;
- same-call non-reuse;
- durable cross-process replay prevention; and
- consumer idempotency.

This separation prevents the first private helper from overclaiming guarantees
that require persistence, atomic claims, reconciliation, and an authoritative
use record.

## 4. Resolve-And-Use Boundary Assessment

The private Core-owned resolve-and-use call is the smallest credible next
boundary. A non-cloneable, non-serializable borrowed capability and one
`FnOnce` consumer keep source records and readiness from escaping into public
runtime state.

One implementation condition is required: `FnOnce` proves one callback
invocation, not one privileged operation inside arbitrary callback code. The
callback invocation itself must be the governed use. The implementation must
keep the bounded consumer and exact operation Core-owned and must not expose a
general-purpose authority object with repeatable `authorize`, `permit`,
execute, dereference, or target methods.

The plan now states this condition explicitly.

## 5. Freshness And Invalidation Assessment

The plan requires one injected evaluation time and fresh validation of:

- immutable run and contract binding;
- actor, workflow, run, step, and harness;
- source observation, validity, generation, watermark, and inventory;
- grant lifecycle, scope, delegation, sensitivity, expiry, and revocation;
- capability availability;
- governed context references;
- policy, approval, evidence, and check prerequisites; and
- required-context consumption.

Any later use, retry, approval resume, or worker restart performs the complete
registered-source resolution again. A repeated deterministic commitment is
not accepted as reusable authority.

## 6. Retry, Resume, And Recovery Assessment

Retry semantics fail closed and do not reuse an assessment, projection,
consumption result, or capability.

Approval resume correctly treats approval as one independent prerequisite,
not as a snapshot of all current authority. Revocation, expiry, policy denial,
evidence invalidation, stale checks, or context unavailability after approval
can still block the resumed use.

Worker recovery restores only durable owning records and rebuilds current
authority. It does not restore `Ready` from workflow state, events, logs,
reports, or commitments.

## 7. Duplicate And Ambiguous Use Assessment

The plan honestly limits the first implementation to same-call non-reuse. It
does not claim to prevent two workers from resolving concurrently.

Future durable replay prevention correctly requires a Core-owned operation
identity, authoritative create-only or compare-and-set registration, atomic
claim/consume semantics, lifecycle state, stale-claim recovery, idempotency,
reconciliation, retention, and privacy review.

An ambiguous consumer outcome blocks automatic replay. That is the correct
default before reconciliation exists.

## 8. Privacy And Error Assessment

The boundary remains private and payload-free. The plan excludes target and
source contents, provider and command output, sandbox logs, credentials,
environment values, paths, endpoints, policies, raw prerequisite payloads,
and unbounded errors.

Stable failures preserve the failed stage without including caller values.
Debug output is limited to bounded posture, reasons, and counts. No failure
falls back to permission or fabricates evidence.

## 9. Relationship Assessment

Proportional governance may reduce interruption only after current authority
is resolved. Quiet success cannot cache authority, suppress blocked or
ambiguous posture, or omit evidence and reporting.

Scoped Runtime Authority and Composable Harness Contracts may consume this
boundary later but cannot replace its current-source and exact-use semantics.

OpenShell remains a sound optional future execution provider. It is not an
authority source and is correctly deferred until the governance boundary can
bind one exact sandbox invocation to current authority.

## 10. Test Plan Assessment

The future test plan covers the important positive, blocked, source-failure,
consumer-failure, ambiguity, invalidation, retry, approval-resume, restart,
privacy, determinism, and regression paths.

The implementation should add compile-fail coverage only if repository-native
privacy and public API tests cannot prove the lifetime and non-escape
properties. A new compile-test dependency is not justified in the first
slice.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add fixed assessment and outcome vectors when those commitments become
  compatibility surfaces.
- Add direct negative-path coverage for substitution, expiry, revocation,
  sensitivity, and multiple matching records.
- Define authoritative persistent use records before any durable replay claim.
- Keep consumer ambiguity explicit until reconciliation exists.
- Plan one opt-in read-only consumer only after implementation review.

## 13. Recommended Next Phase

Implement the private same-call use boundary only.

Add the private borrowed use capability, one Core-owned `FnOnce`
resolve-and-use helper, bounded payload-free outcomes, and focused tests using
a test-only read-only consumer. Do not add persistence, executor integration,
dereference, providers, OpenShell, sandbox execution, SideEffects, writes,
schemas, or CLI behavior.

## 14. Governed Review Record

- workflow: `dg/review`
- run ID: `run-1785173096314867000-2`
- approval ID:
  `approval/run-1785173096314867000-2/review-scope-approved`
- approval presentation ID: `presentation/528c9639368891ab`
- approval presentation content hash:
  `528c9639368891ab14b0c051ed754e6c2e2104b0d3555e31e3f558c0e997338e`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- review status: accepted; governed phase completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with event marker
- out-of-kernel work: the delegated maintainer independently inspected the
  plan, source-boundary foundations, roadmap, and prior review evidence and
  authored this review; the kernel governed scope and approval but did not
  inspect files, edit documentation, run checks, or mutate git
