# Private Trusted-Host Caller And Typed-Wait Plan Report

## 1. Executive Summary

Planning is complete and focused-review accepted for the smallest private
caller and typed-wait boundary over the accepted local SQLite operational
entry helper.

The source review found a prerequisite that must precede caller integration:
trusted-host yields currently register no waits, and private durable wait state
does not bind the exact dependency source expected to satisfy a condition. The
plan therefore sequences authoritative dependency binding, typed wait
registration, one source-specific wake verifier, opaque handoff, and explicit
reinvocation before adding a private caller.

The focused review chooses `TimeWindow` as the first wake source and requires
fresh verification against the accepted trusted-time source. Executor wait
declarations remain untrusted liveness requests and cannot create approval or
policy authority.

## 2. Scope Completed

- Inspected the accepted operational entry, supervisor, redispatch,
  continuity, wait-transition, and current-authority boundaries.
- Defined private typed wait declaration, yield request, wait handoff, wake
  observation, and caller outcome candidates.
- Defined exact dependency binding and deterministic kind-to-trigger rules.
- Defined source-specific same-call wake verification.
- Defined explicit reinvocation through the accepted operational entry helper.
- Defined replay, concurrency, privacy, migration, and test posture.
- Added phased sequencing that prevents a trigger-only wait from becoming a
  false governance boundary.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- runtime code, state migration, or tests;
- a scheduler, daemon, queue, poller, timer, or wake bus;
- model-turn creation or automatic executor recirculation;
- provider execution, provider mutation, OpenShell, or sandbox integration;
- nested harnesses, recursive agents, or agent swarms;
- public configuration, workflow fields, CLI, SDK, UI, or examples;
- filesystem, PostgreSQL, multi-host, hosted, or distributed parity;
- automatic approval, enterprise identity, RBAC, or administration; or
- release-posture changes.

## 4. Key Finding

The accepted supervisor converts `Yielded(reason)` into an atomic yield with
an empty wait list. Private authoritative wait state stores a wake-trigger
class but not the exact required dependency reference before satisfaction.

That is safe while no caller claims automatic wake behavior, but it is not
sufficient for a real typed-wait caller. The first implementation must harden
dependency binding before it exposes wait registration or wake transitions.

## 5. Proposed Boundary

The future caller remains a synchronous crate-private coordinator. It may ask
Core to enter work, observe an opaque wait handoff, or apply one
source-verified wake transition. It cannot poll, schedule, choose
continuation, construct authority, satisfy a wait with a boolean, or invoke an
executor outside the accepted operational entry helper.

## 6. Security And Privacy Summary

- Waits bind exact dependency commitments and compatible trigger classes.
- Core derives all execution and cursor binding.
- Wake facts are verified in source-specific same-call boundaries.
- Handoffs remain non-authoritative.
- Reinvocation always rehydrates current durable state.
- Durable state remains payload-free and redaction-safe.
- Capabilities remain private, one-use, non-serializable, and
  non-reconstructable.

## 7. Validation

Required for phase close:

- `npm run check:docs`
- `git diff --check`

No Rust validation is required because this planning phase changes
documentation only.

## 8. Remaining Known Limitations

- The exact first wake source remains a plan-review decision.
- The SQLite migration shape remains open.
- No current trusted-host path registers a genuine wait.
- No private or public caller invokes the operational entry boundary.
- No scheduler or external execution substrate exists.

## 9. Recommended Next Phase

Implement exact authoritative wait dependency binding and the private
`TimeWindow` verifier only. Do not implement the caller first.

That prerequisite implementation is now complete and documented in the
[Trusted Host TimeWindow Wait Binding Implementation
Report](TRUSTED_HOST_TIME_WINDOW_WAIT_BINDING_REPORT.md). The historical
planning finding remains unchanged; the next phase is its focused
maintainer/security implementation review, not caller integration.

## 10. Governed Phase Record

- workflow: `dg/d`
- run: `run-1791022225276561000-2`
- approval: `approval/run-1791022225276561000-2/planning-approved`
- presentation: `presentation/d593d8b8c93e58cf`
- presentation hash:
  `d593d8b8c93e58cf8cee0f81b05081b31d1cdce2b1ad5c73852d0e6c8261f650`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: planning and documentation only
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations, 0
  retries, and 0 escalations; approval-presentation proof enforced
- validation summary: documentation and diff checks passed
- out-of-kernel work: source inspection, plan authoring, validation, and git
  actions were performed by the delegated trusted host; Workflow OS governed
  scope and approval but did not perform those actions
