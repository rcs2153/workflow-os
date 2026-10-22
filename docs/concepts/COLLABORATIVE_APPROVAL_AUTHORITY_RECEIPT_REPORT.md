# Collaborative Approval Authority Receipt Report

## 1. Executive Summary

Phase 2 of the accepted collaborative `ApprovalDecide` consumer plan is
implemented. Workflow OS now has a minimal payload-free
`CollaborativeApprovalAuthorityReceipt`, an explicitly unverified persisted
claim boundary, and a dedicated PostgreSQL v4 create-only store.

The receipt is evidence for one decision. It is not an authority token, cannot
be deserialized into the trusted type, and is not consumed by any approval or
executor path in this phase.

## 2. Scope Completed

- Added the closed receipt format and deterministic receipt identity.
- Bound the receipt to one approval reference and one approval-decision event.
- Fixed the capability to `ApprovalDecide`.
- Recorded only source/revision/query/snapshot/grant commitments, database
  observation time, decision kind, and explicit non-authorizing posture.
- Added an explicitly unverified deserialization boundary.
- Added a dedicated PostgreSQL table with unique approval and event bindings.
- Added exact-idempotent create, exact-ID read, and exact decision recovery.
- Added additive PostgreSQL v3-to-v4 and v1-to-v4 migration coverage.
- Added model, replay, conflict, corruption, recovery, and non-leakage tests.

## 3. Scope Explicitly Not Completed

- No collaborative approval-decision consumer.
- No executor integration or run resumption.
- No atomic event/projection/receipt composition.
- No organization advisory lock or watermark row lock in a decision path.
- No hosted authentication or identity administration.
- No provider write or additional mutation family.
- No workflow schema, CLI, SDK, or example changes.
- No cryptographic receipt or external notarization.

## 4. Model Summary

The trusted `CollaborativeApprovalAuthorityReceipt` is serialize-only and can
only be constructed inside Core from private proof material. Serialized data
deserializes as `UnverifiedCollaborativeApprovalAuthorityReceipt`. Its fixed
effect is `evidence_only_not_authorization`, and its validity is
`point_in_time_only`.

The receipt stores no organization, project, principal, external issuer,
audience, subject, provider payload, token, reason text, presentation content,
workflow content, raw grant, or revocation payload.

## 5. Persistence Summary

PostgreSQL schema v4 adds
`workflow_os.collaborative_approval_authority_receipts`. The table is
create-only and uniquely binds:

- receipt identity;
- approval reference; and
- approval-decision event reference.

Exact replay returns `AlreadyExists`. Rebinding any unique subject to different
receipt content fails closed. Reads verify the canonical payload hash, receipt
commitment, deterministic identity, and relational bindings before returning
an explicitly unverified persisted record.

## 6. Validation Boundary Summary

The trusted constructor computes the complete canonical commitment and receipt
identity. The public model validates fixed posture and commitment consistency.
Persistence accepts only the trusted type. Database reads never restore trust.
Stable errors do not include identifiers, commitments, payloads, or database
values.

## 7. Privacy And Redaction Summary

`Debug` redacts receipt identity, approval/event references, source identity,
revision, commitments, and observation time. Serialization is intentionally
payload-free. Deserialization and database-corruption errors are generic and
do not echo malformed or secret-like stored values.

## 8. Test Coverage Summary

Coverage includes:

- trusted serialization to an unverified claim;
- deterministic identity and commitment validation;
- tamper rejection and generic deserialization errors;
- Debug and forbidden-field non-leakage;
- PostgreSQL v1-to-v4 and v3-to-v4 migration;
- first write and exact replay;
- approval/event rebinding conflict;
- exact receipt and exact decision recovery; and
- canonical-payload corruption failure and restoration.

The live PostgreSQL test branches are environment-gated. This workstation did
not provide `WORKFLOW_OS_TEST_POSTGRES_URL` or
`WORKFLOW_OS_RECOVERY_POSTGRES_URL`; local validation compiled those tests, and
the required shared PostgreSQL CI job remains the live execution proof.

## 9. Commands Run And Results

- `cargo test -p workflow-core --lib collaborative_approval_authority_receipt --no-fail-fast` - passed.
- `cargo test -p workflow-core --test postgres_state_backend --no-run` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed; opt-in live tests remained ignored by their existing contracts.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

The live PostgreSQL branches did not run locally because the required test and
recovery database URLs were not configured. The shared PostgreSQL CI job is
therefore the required live persistence proof for this phase.

## 10. Governed Phase Record

- Dogfood workflow: `dg/implement`.
- Run: `run-1792653439701029000-2`.
- Approval: `approval/run-1792653439701029000-2/implementation-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/aafb61f14e75b7ec`.
- Terminal status: `Completed`.
- Event summary: 39 events, one approval, zero retries, and zero escalations.
- Out-of-kernel work: repository edits, formatting, linting, tests,
  documentation validation, and this report were performed by the delegated
  maintainer outside the kernel execution layer. No PostgreSQL service was
  created or simulated, and no git or pull-request action occurred during the
  governed phase.

## 11. Remaining Known Limitations

- No consequential consumer accepts the receipt.
- The receipt constructor is intentionally private and currently used only by
  focused tests; Phase 3 will invoke it after same-transaction authority proof.
- The current transaction-scoped source helper still obtains database time
  internally. Phase 3 must move timestamp capture to the outer decision
  transaction.
- Phase 3 must implement the reviewed organization advisory lock followed by
  watermark `FOR UPDATE` lock order before any decision mutation.
- The receipt is local unsigned evidence, not cryptographic attestation.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the collaborative approval-
authority receipt model and PostgreSQL persistence. Only after acceptance
should Phase 3 implement the same-call collaborative `ApprovalDecide` consumer.
