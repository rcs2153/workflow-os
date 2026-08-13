# Proportional-Governance Visible-Disclosure Surface-Acceptance Proof Report

## 1. Executive Summary

The selected local project-validation visible-governance route now records one
payload-free `GovernanceDisclosureSurfaceAccepted` workflow event before run
validation and skill execution. Terminal reports cite that durable event, and
the CLI reports surface acceptance only after persistence.

The event proves only that the explicitly injected surface accepted the exact
request. It does not prove external delivery, display, human observation,
understanding, or acknowledgement.

## 2. Scope Completed

- Added durable event vocabulary and snapshot projection.
- Added exact identity, ordering, duplicate, and idempotency validation.
- Composed the event into the selected visible runtime route.
- Added bounded generic audit projection.
- Added automatic WorkReport event citation.
- Moved CLI success posture after durable route completion.
- Added focused runtime, audit, executor, report, and CLI regression tests.

## 3. Scope Explicitly Not Completed

This phase did not add disclosure retries or outbox behavior, new surfaces,
notifications, UI delivery, human acknowledgement, approval changes, schemas,
examples, providers, OpenShell execution, SideEffect execution, external
writes, hosted expansion, or release changes.

## 4. Runtime And Event Boundary

The event is accepted only while the run remains `Created` and after the exact
assessment is durably bound. It requires idempotency and rejects duplicate
delivery identities or identity/timestamp mismatch. The event is appended
before `RunValidated`, `RunStarted`, step scheduling, and skill invocation.

## 5. Audit And Report Boundary

Audit projection exposes only the bounded surface kind and explicit
observation/acknowledgement non-claims. It stores no surface reference,
delivery identity, assessment fingerprint, or disclosure content.

The selected terminal report composer discovers the persisted acceptance
event and adds its event ID to report citations. Callers do not need to
fabricate or supply that proof.

## 6. Failure And Idempotency Posture

Surface rejection fails before run events and skills. Event persistence
failure stops execution. Duplicate durable delivery identities fail closed.

A crash can occur after a surface accepts the request but before the event is
persisted, so retries may invoke the surface again. The phase does not claim
exactly-once external delivery; a future retry/outbox phase would be required
for stronger semantics.

## 7. Privacy And Redaction

The durable receipt remains payload-free and validated. Debug, audit, errors,
and CLI output avoid surface references, fingerprints, raw payloads, command
output, provider output, paths, tokens, and secret-like values. CLI output uses
bounded posture and explicit non-claims.

## 8. Test Coverage

Coverage includes durable rehydration, legacy snapshot compatibility,
idempotency, duplicate rejection, identity mismatch, ordering before skill
execution, audit non-leakage, report citation, and post-persistence CLI output.

## 9. Governed Implementation Record

- Workflow: `dg/runtime-composition`
- Run: `run-1786570019444128000-2`
- Approval: `approval/run-1786570019444128000-2/composition-approved`
- Presentation: `presentation/179fd83d0b926914`
- Approval outcome: granted by delegated maintainer through proof-enforced
  approval
- Kernel boundary: the kernel governed scope, approval, and event history;
  code inspection, edits, checks, and Git work remained outside the kernel

## 10. Validation

The final local matrix passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- complete `runtime_events`, `audit_projection`, `local_executor`, and CLI
  integration-test binaries;
- `cargo test --workspace`;
- `npm run check:docs`;
- `npm run check:integrations`;
- `npm run check`; and
- `git diff --check`.

## 11. Remaining Limitations

- Only the selected injected-local visible route records this event.
- No independent delivery or human observation proof exists.
- No retry/outbox semantics exist.
- The snapshot stores a vector for future surfaces, but the selected route
  currently records one receipt.

## 12. Recommended Next Phase

Run a focused maintainer review. If accepted, continue roadmap runtime
composition without broadening the event into an acknowledgement claim.
