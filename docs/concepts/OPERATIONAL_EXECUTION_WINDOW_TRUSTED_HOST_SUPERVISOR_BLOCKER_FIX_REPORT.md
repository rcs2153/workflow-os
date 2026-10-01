# Operational Execution Window Trusted-Host Supervisor Blocker Fix Report

## 1. Executive Summary

The accepted one-shot trusted-host supervisor review blockers are repaired in
the existing private, local, SQLite-only boundary. Dispatch authority is now
owned and consumed by one supervisor call, current durable attempt state is
validated before executor entry, and the complete bounded skill invocation
plus an explicit executor identity is committed before opening authority is
issued.

Projected result persistence now reconciles an ambiguous write through the
existing fresh-connection reconciliation API. The focused regression matrix
covers every closed executor result, stale replay, all previously unbound
invocation fields, executor substitution, missing yield identity, and
confirmed-present versus confirmed-absent persistence faults.

This fix does not add scheduling, providers, public runtime configuration, or
another continuity operation family.

## 2. Blockers Fixed

1. **One-use dispatch authority.** `TrustedHostSupervisorInput` and its attempt
   capability are consumed by value. Before executor entry, Core reads fresh
   durable continuity state and accepts only the exact still-started attempt,
   current window revision, authority binding, cursor, actor, and immutable
   bundle binding. A reconstructed stale capability cannot re-enter the
   executor after the first outcome.
2. **Complete invocation binding.** The opening authority commitment now
   includes workflow and run identity, workflow and schema versions, spec
   hash, step, skill identity and version, correlation identity, canonical
   bounded values, and an explicit executor binding commitment. Any
   substitution fails before executor entry.
3. **Ambiguous persistence reconciliation.** If a projected outcome, yield, or
   ambiguity write returns an error, the supervisor reconciles the exact
   operation, request commitment, and receipt on a fresh SQLite connection.
   It distinguishes durable success, durable security rejection, confirmed
   absence, and unreadable/corrupt reconciliation state with stable,
   non-leaking errors.
4. **Closed regression matrix.** Focused tests now exercise success,
   retryable failure, terminal failure, turn-boundary yield,
   ambiguous-may-have-started, missing yield generation, stale replay, full
   invocation substitution, executor substitution, and before/after
   persistence ambiguity.

## 3. Implementation Approach

The repair reuses the accepted opening and five-operation continuity model.
No dispatch-claim operation was added. Rust ownership prevents ordinary reuse
of the private capability, while a fresh durable dispatchability read rejects
stale or reconstructed authority before the executor callback.

The invocation commitment extends the existing operation commitment rather
than storing raw invocation values. Executor identity is an explicit bounded
commitment supplied before authority issuance. It proves selection consistency
within this private trusted-host boundary; it is not a cryptographic process,
binary, or hardware identity claim.

## 4. Validation Boundary

Before executor entry, the supervisor verifies:

- the complete invocation and executor commitment;
- workflow, run, step, actor, cursor, and immutable-bundle equality;
- current window state and revision;
- current attempt state, revision, origin operation, and authority; and
- trusted-time source, provenance, epoch, monotonicity, and expiry.

Validation failures use stable codes and do not echo identifiers, values,
paths, payloads, or authority material.

## 5. Persistence And Reconciliation

Normal projected writes retain their atomic continuity-state, runtime-event,
snapshot, and projection-binding behavior. On an ambiguous return, the
supervisor opens a fresh connection through
`reconcile_projected_operation(...)` and validates the exact operation,
request commitment, receipt, durable disposition, and projection binding.

A durably committed success is accepted. A committed security rejection is
returned as a bounded security error. Confirmed absence and unreadable or
corrupt state fail closed and do not fabricate success.

## 6. Privacy And Redaction

The fix stores commitments rather than raw invocation values. Existing custom
Debug output remains redacted. Reconciliation errors are remapped to stable
supervisor codes and never propagate raw SQLite errors or caller values. No
prompt, transcript, source body, command output, environment value,
credential, provider payload, or reconstructable authority is added.

## 7. Test Coverage

Focused coverage proves:

- one owned capability drives one executor call;
- stale or reconstructed capability cannot invoke the executor twice;
- every `SkillInput` field and executor identity is bound;
- all closed executor result variants persist deterministically;
- a yield requires an explicit generation identity;
- after-commit ambiguity reconciles to durable success;
- before-commit ambiguity reconciles to confirmed absence; and
- supervisor success still does not complete the workflow run.

Existing operational-opening, continuity, executor, adapter, report, and
runtime tests remain in the broader validation boundary.

## 8. Commands And Results

The final governed phase close records the authoritative command results.
Passed:

- `cargo fmt --all --check`;
- `cargo clippy -p workflow-core --all-targets -- -D warnings`;
- `cargo test -p workflow-core --lib` with 343 passing tests;
- focused operational-opening and supervisor module coverage with 20 passing
  tests, also included in the full Core library suite;
- `npm run check:docs`; and
- `git diff --check`.

`cargo test --workspace` was not rerun in this blocker phase. The changes are
private Workflow Core internals, the complete Core library suite passed, and
this host has a previously observed multi-minute startup delay for each of 71
workspace integration binaries. Repository CI must complete the workspace
matrix before merge; this report does not call it a local pass.

## 9. Remaining Known Limitations

- The supervisor remains crate-private, one-shot, local, and SQLite-only.
- Explicit executor identity is a trusted-host commitment, not remote
  attestation or cryptographic binary identity.
- There is no scheduler, daemon, polling loop, or automatic redispatch.
- There is no provider, OpenShell, model-turn, or nested-harness execution.
- Confirmed-absent result persistence fails closed; the host must not invent a
  retry or completed result.
- No public API, runtime configuration, CLI, SDK, or workflow schema selects
  this path.

## 10. Recommended Next Phase

Perform a **focused maintainer/security re-review of the one-shot supervisor
blocker fix**. Repeated scheduling and provider broadening remain blocked until
the re-review accepts ownership, binding, reconciliation, and test coverage.

## 11. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1790894609265334000-2`
- approval: `approval/run-1790894609265334000-2/fix-approved`
- presentation: `presentation/c1e67ca23d65fe94`
- presentation hash:
  `c1e67ca23d65fe94319ef188eb314c8b7c6dd417cff9caed1f4fbeac6cf3df30`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- approval presentation enforcement: proof enforced with one durable
  presentation record and an event-trail marker
- validation summary: formatting, strict Workflow Core clippy, 343 Core
  library tests, 20 focused opening/supervisor tests, docs, and diff checks
  passed; full workspace execution is deferred to CI as disclosed above
- out-of-kernel work: source edits and validation commands were executed by
  the delegated local maintainer; the kernel governed scope and approval but
  did not execute shell commands or edit repository files
