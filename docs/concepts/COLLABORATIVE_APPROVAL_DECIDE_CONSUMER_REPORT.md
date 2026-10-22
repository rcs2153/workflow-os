# Collaborative ApprovalDecide Consumer Implementation Report

## 1. Executive Summary

Workflow OS now implements one explicit, opt-in executor-facing collaborative
approval decision path. A decision commits only when current exact-project
`ApprovalDecide` authority is proven inside the same PostgreSQL transaction
that records the authoritative decision event, approval projection, and
payload-free authority receipt.

The executor rehydrates the committed run before continuation. Exact restart
replay recovers the existing receipt and does not append a duplicate decision
event.

## 2. Scope Completed

- Added a closed collaborative project approval decision request/result API.
- Fixed the capability to `ApprovalDecide` in Core.
- Required authenticated-principal and decision-actor agreement.
- Reused durable approval-presentation proof and existing decision validation.
- Reused the organization advisory lock used by authority revision activation.
- Moved database-time authority observation into the outer decision
  transaction.
- Locked and rechecked the current authority watermark.
- Required an active exact-project run binding.
- Atomically persisted the decision event, approval projection, and receipt.
- Rehydrated committed state before resume or denial completion.
- Added exact replay and restart reconciliation without duplicate events.

## 3. Scope Explicitly Not Completed

- No hosted approval-decision endpoint.
- No hosted authentication cutover, OIDC, SAML, SCIM, groups, or invitations.
- No identity administration or dynamic grant API.
- No second collaborative capability consumer.
- No provider mutation or new write family.
- No workflow schema, CLI, SDK, or example changes.
- No reusable authority token or production enterprise-identity claim.

## 4. Executor API Summary

`CollaborativeProjectApprovalDecisionRequest` combines trusted exact project
scope, authenticated principal, and the existing proof-enforced approval
decision request. `decide_collaborative_project_approval_with_presentation`
prepares and proof-marks the decision through existing validation, invokes the
atomic backend operation, rehydrates the committed run, and continues without
re-appending the decision.

`CollaborativeProjectApprovalDecisionResult` returns the resulting run, stable
receipt ID, and whether receipt persistence was new or an exact replay. Debug
output does not expose identity facts, presentation content, or receipt
payloads.

## 5. Transaction And Lock Boundary

The PostgreSQL operation acquires the same organization-scoped session
advisory lock as authority activation, then opens a serializable transaction.
It reads and locks the active watermark, captures database time, reconstructs
and validates the complete active authority revision, requires exactly one
active matching grant, locks the exact run binding and approval state, records
the decision, rechecks the watermark, and writes the receipt before commit.

Any failure rolls back the event, approval projection, and receipt together.
Authority evidence is call-local and cannot survive as reusable permission.

## 6. Replay And Recovery

The backend accepts exact replay only when the durable approval projection and
authoritative decision event are equivalent. Receipt persistence then
reconciles as `AlreadyExists`. Conflicting replay fails closed.

The executor detects an already-decided approval, validates the supplied proof
against the original pending request, locates exactly one matching decision
event, recovers its receipt, and resumes from rehydrated state. It does not call
an append-first approval helper.

## 7. Gate Preservation

The consumer preserves approval subject validation, durable presentation proof,
proof-marker construction, immutable run context used by resume preparation,
exact project/run binding, policy evaluation during continuation, and existing
denial behavior. Collaborative authority is one additional prerequisite, not
a replacement for those gates.

This narrow path does not claim that every high-assurance, evidence, check,
SideEffect, or report obligation is universally composed. Those obligations
remain path-specific and must be reviewed before broader adoption.

## 8. Privacy And Redaction

The request cannot provide identity facts, current-source evidence, source
revision, trusted time, or capability. Receipts contain commitments and closed
vocabulary only. Private authorizing views are not serializable. Public Debug
implementations redact sensitive structures, and stable errors do not echo
actors, project IDs, approval reasons, presentation text, provider subjects,
database values, paths, tokens, or payloads.

## 9. Test Coverage

Focused coverage includes:

- actor mismatch rejection before opening PostgreSQL;
- successful same-call decision and receipt recovery;
- exact replay with one durable decision event;
- concurrent duplicate calls producing one write and one exact replay;
- a database-trigger transaction barrier that pauses the decision after its
  authority read while revocation attempts to activate, proving the shared
  lock forces a lawful serial order;
- post-revocation denial with no decision mutation, event, or receipt; and
- direct public executor-consumer coverage for committed-decision recovery
  after a post-commit audit-signal failure, exact terminal replay without a
  second handler invocation, and revoked-authority non-invocation; and
- existing PostgreSQL migration, source, receipt, recovery, and workspace
  coverage.

The live PostgreSQL cases are environment-gated locally and mandatory in the
required Shared PostgreSQL State CI job.

## 10. Validation Commands And Results

- `cargo check -p workflow-core`: passed.
- Focused actor-mismatch test: passed.
- PostgreSQL conformance target compilation: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Live PostgreSQL conformance: deferred to required CI because
  `WORKFLOW_OS_TEST_POSTGRES_URL` is not configured on this machine.

## 11. Remaining Known Limitations

- The API is not wired to a hosted route or CLI.
- Collaborative authority currently governs only this explicit approval path.
- No identity-provider authentication or administration surface exists.
- Receipt evidence is non-authorizing and cannot be reused.
- Live database evidence depends on required CI in environments without a local
  PostgreSQL fixture.
- Broader gate composition requires separate reviewed consumers.

The focused review identified an executor-boundary verification blocker. The
blocker fix now locks the exact run binding inside the decision transaction and
adds live PostgreSQL coverage through the public executor consumer. Local
compilation and all non-live validation pass; the required Shared PostgreSQL
State CI job remains the acceptance evidence for the live database path.

## 12. Recommended Next Phase

Perform a **focused blocker-fix review of the same-call collaborative
`ApprovalDecide` consumer** after required Shared PostgreSQL State CI executes
the public executor-consumer proof. Do not add another identity consumer
before acceptance.
