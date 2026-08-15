# Authorized Execution Continuity Atomic State Plan Report

## 1. Executive Summary

The P0 atomic-state plan is documented and accepted after focused
maintainer/security review. It defines the durable coordination
boundary required before Workflow OS can preserve lawful non-terminal work
across executor-turn and process boundaries without duplicate or unsafe retry.
This phase is planning only.

## 2. Scope Completed

- Defined authoritative window, yield, wait, directive, attempt, event, and
  snapshot boundaries.
- Defined atomic yield registration, directive consumption plus attempt start,
  and attempt-outcome reconciliation operations.
- Defined exact replay, conflict, stale-state, unsupported-backend, and recovery
  semantics.
- Defined crash posture and attempt ambiguity after consumer entry.
- Defined backend capability, conformance, privacy, trusted-time, and rollout
  requirements.
- Updated the roadmap and parent continuity plan.

## 3. Scope Explicitly Not Completed

No state capability trait, records, backend writes, schema migrations, runtime
events, supervisor, automatic resume, delegated approval, executor invocation,
provider mutation, CLI behavior, workflow schema, nested harness runtime, or
release change is implemented.

## 4. Recommended Implementation

The first implementation should add a separate
`AuthorizedExecutionContinuityStore` capability contract, authoritative
request/result and attempt-lifecycle models, explicit backend support posture,
and a test-only in-memory reference implementation with executable conformance.
No existing durable backend should advertise support during that phase.

## 5. Security Boundary

The plan requires `started` to be durable before executor entry. Missing
outcome after start is ambiguous and blocks automatic retry. Public serialized
continuity models, host identity, final responses, and delivery
acknowledgements cannot authorize execution. Current authority is resolved
immediately before atomic consumption and bound by commitment.

## 6. Backend Posture

- in-memory reference store: eligible for test-only conformance;
- local filesystem: unsupported;
- SQLite: unsupported until a later transactional schema implementation;
- PostgreSQL: unsupported until explicit transactions and dedicated
  conformance replace blanket declaration.

## 7. Privacy Posture

Only bounded IDs, hashes, revisions, enums, timestamps, and stable references
may be stored. Raw prompts, transcripts, source contents, command output,
provider payloads, environment values, credentials, and bearer authority are
forbidden. Debug, serde, storage, conflict, and delivery errors must not echo
sensitive values.

## 8. Governed Planning Record

- workflow: `dg/d`
- run: `run-1786802089487421000-2`
- approval: `approval/run-1786802089487421000-2/planning-approved`
- presentation: `presentation/2ffbb9e610ace8ec`
- approval outcome: granted by delegated maintainer after complete handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof-enforced
- planning work: document and source inspection occurred outside the kernel;
  the kernel governed planning scope and approval

Out-of-kernel work included repository/source inspection, documentation edits,
independent delegated analysis, documentation validation, and diff hygiene.
The kernel did not edit files, run checks automatically, commit, push, or open
a pull request. No handler, report-artifact, backend-transaction, or runtime
continuity coverage was simulated.

## 9. Validation

- independent state-backend and security analyses: completed;
- focused maintainer/security review: accepted after fix-forward hardening;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 10. Remaining Limitations

- No backend implements the planned capability.
- No continuity event vocabulary exists.
- No supervisor can schedule or resume an executor.
- Delegated approval remains unimplemented.
- Trusted-time details for SQLite remain an open design question.

## 11. Recommended Next Phase

Implement the atomic state capability contract and in-memory reference
conformance suite only.
