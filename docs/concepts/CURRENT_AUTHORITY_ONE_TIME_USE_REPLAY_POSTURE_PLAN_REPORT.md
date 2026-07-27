# Current-Authority One-Time-Use And Replay Posture Planning Report

## 1. Executive Summary

Planning is complete for the next private current-authority safety boundary.

The plan rejects reusable TTL-based readiness and authority tokens. It requires
Workflow OS Core to re-read and re-resolve current authority for every use,
retry, approval resume, and worker restart. The first implementation should
keep a non-cloneable, non-serializable capability inside one Core-owned
`FnOnce` call.

This planning phase adds no runtime behavior.

## 2. Scope Completed

- Defined source freshness, assessment currency, same-call non-reuse, durable
  replay prevention, and consumer idempotency as separate concerns.
- Required re-resolution for every use and continuation path.
- Defined a private borrowed use-capability and `FnOnce` API shape.
- Defined retry, approval-resume, worker-restart, duplicate-call, concurrency,
  and ambiguous-outcome posture.
- Connected the boundary to proportional governance, scoped runtime authority,
  Composable Harness Contracts, and optional future execution providers.
- Defined privacy, errors, tests, implementation sequence, and open questions.

## 3. Scope Explicitly Not Completed

No Rust model or helper, public readiness API, bearer token, lease, persistence,
durable replay record, executor integration, dereference, provider, OpenShell,
sandbox execution, SideEffect execution, write, event, artifact, schema, SDK,
CLI, UI, hosted behavior, enterprise administration, reasoning lineage, or
release change is implemented.

## 4. Primary Architecture Decision

Use **re-resolve per use**, not a reusable validity window.

The future private helper should resolve current authority and invoke one
bounded consumer in the same lexical call. A move-only borrowed capability may
prove that one in-process callback cannot reuse the assessment. It cannot prove
cross-process replay prevention, so the implementation must not claim that
property.

## 5. Continuation Semantics

Retries and approval resume must load durable owning records and then obtain a
new source-backed assessment. Worker restart must never restore prior
readiness from workflow state, an event, report, log, or commitment.

Ambiguous prior consumer completion blocks automatic replay until a future
reconciliation boundary exists.

## 6. Security And Privacy

The planned boundary keeps source records, assessments, and use capability
private. It prohibits raw target payloads, source contents, command/provider
output, sandbox logs, credentials, environment values, policies, paths, and
unbounded errors.

Caller nonces, timestamps, or IDs cannot manufacture replay protection.

## 7. Recommended First Implementation

Implement only the private same-call use capability and Core-owned `FnOnce`
helper with a test-only bounded read-only consumer.

Prove that `Ready` invokes exactly once, blocked and failed assessments never
invoke, authority cannot escape the call, and all outcomes and errors remain
bounded and redaction-safe.

## 8. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Recommended Next Phase

Focused maintainer review of the plan.

After acceptance, implement the private same-call use boundary. Keep
persistence, runtime consumers, dereference, providers, OpenShell, sandbox
execution, SideEffects, writes, schemas, and CLI behavior deferred.

## 10. Governed Planning Record

- workflow: `dg/d`
- run ID: `run-1785172800755866000-2`
- approval ID: `approval/run-1785172800755866000-2/planning-approved`
- approval presentation ID: `presentation/3696148f44080ea1`
- approval presentation content hash:
  `3696148f44080ea1d98144c6ded925d29faa9e1207f694ebf1ee3b2da6d1b824`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted planning handoff was presented
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations; approval
  presentation proof enforced
- out-of-kernel work: the delegated maintainer inspected existing authority,
  freshness, retry, approval-resume, and replay foundations and authored the
  roadmap, plan, and report; the kernel governed scope and approval but did not
  inspect files, write documentation, run checks, or mutate git
