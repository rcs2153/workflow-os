# Authoritative Collaborative Identity Source Implementation Report

## 1. Executive Summary

Workflow OS now implements the first persistent collaborative identity source
slice. A Core-owned PostgreSQL source stores and activates complete canonical
identity authority revisions, resolves exact organization/project/principal/
capability queries at database time, and returns payload-free current-authority
source evidence.

This phase does not authorize approval decisions. It creates the reviewed
source boundary that a later same-call `ApprovalDecide` consumer may use only
after focused maintainer/security review.

## 2. Scope Completed

- Added a closed versioned collaborative authority query.
- Added complete provider, binding, principal, grant, and revocation fact
  families to source evidence vocabulary.
- Preserved the existing V1 required-context source request unchanged.
- Added canonical complete-revision candidate validation and commitment.
- Added PostgreSQL schema V3 identity fact and authority high-watermark tables.
- Added atomic higher-revision activation, exact replay reconciliation,
  rollback rejection, and same-revision conflict rejection.
- Added full durable-row hash checking and canonical candidate reconstruction.
- Added database-time exact matching for active provider, binding, principal,
  grant activation/expiry, and effective revocation.
- Added payload-free complete source snapshots and match posture.
- Added always-on model tests and PostgreSQL conformance coverage.

## 3. Scope Explicitly Not Completed

- No approval-decision or run-resume consumer.
- No OIDC, OAuth, SAML, SCIM, groups, directories, or invitations.
- No hosted authentication or session cutover.
- No public identity administration or provider mutation API.
- No workflow schema, CLI, or example exposure.
- No new provider write family.
- No production identity, certification, or enterprise-readiness claim.

## 4. Source Model Summary

`CollaborativeIdentityAuthorityQuery` commits one exact organization, project,
local principal, and closed capability query. A separate internal request model
shares the existing registration and snapshot validation machinery rather than
forcing collaborative subjects into required-context hashes.

`CollaborativeIdentityAuthorityRevisionCandidate` is validated caller input to
trusted activation, not authority by itself. It requires complete provider,
binding, and principal inventories, canonical ordering, coherent references,
and bounded family sizes.

## 5. Persistence And Revision Boundary

PostgreSQL schema V3 stores immutable facts by organization, revision, family,
and logical identity. Each row contains a canonical payload and payload hash;
duplicated query columns are checked against the decoded value.

The organization high watermark activates one complete revision. Activation is
serializable. Lower revisions fail, exact replay reconstructs and compares the
durable inventory, same-revision content conflicts fail, and higher revisions
insert the full inventory before the high watermark advances atomically.

## 6. Exact Read Boundary

The source captures PostgreSQL `clock_timestamp()` once, loads the full active
revision, verifies every row and the revision commitment, reconstructs the
validated candidate, and evaluates the exact query. A matching grant requires
an active effective provider, binding, and principal; exact project/principal/
capability scope; an effective unexpired grant; and no effective matching
revocation.

The result is payload-free evidence. It does not authorize an action and cannot
be reused as a capability token.

## 7. Privacy And Redaction

External subjects and provider assertions are never stored by this source;
only existing opaque commitments are persisted. Debug output redacts
organization, project, principal, query commitment, revision commitment, and
snapshot detail. Errors use stable codes and do not include record values,
database credentials, payloads, or query subjects.

## 8. Test Coverage

Focused tests cover active, expired, and revoked exact matches; deterministic
candidate commitment; dangling identity rejection; closed vocabulary; Debug
and serialization posture; schema V1-to-V3 migration; activation, replay,
same-revision conflict, higher-revision revocation, rollback rejection, restart
read, and payload-free match evidence.

The PostgreSQL conformance test is environment-gated. It compiles locally and
runs when `WORKFLOW_OS_TEST_POSTGRES_URL` is supplied; the current local run
skipped database operations because that fixture was absent.

## 9. Validation Commands

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test collaborative_identity_authority_source
  --test postgres_state_backend --no-fail-fast`: passed; PostgreSQL operations
  were environment-skipped locally.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- The source has no runtime authorization consumer.
- Initial revision activation is a trusted backend API, not a reviewed admin
  surface.
- The first implementation stores one complete inventory per revision rather
  than independent append-only lifecycle deltas.
- Local validation does not prove live PostgreSQL behavior without the test
  fixture; CI must execute the conformance path.
- Recovery tooling has no collaborative-identity-specific operator command.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of the persistent collaborative
identity source. Only after acceptance should Workflow OS plan one same-call
`ApprovalDecide` consumer with transactionally revalidated revision and
revocation posture.

## 12. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1792648262242672000-2`.
- Approval: `approval/run-1792648262242672000-2/implementation-approved`.
- Approval presentation: `presentation/3dcf1da3ab1b7120`.
- Outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase-close reinspection could not rehydrate the completed run because the
  helper-selected temporary state directory contained an empty event stream.
  No state was repaired or synthesized; this is disclosed as a dogfood runner
  durability limitation.
- Out-of-kernel work: repository edits, tests, documentation, git, and later PR
  actions are executed by the delegated maintainer; Workflow OS governs scope
  and records the phase trail.

## 13. Product Boundary

The delivered code closes a source-provenance gap. It does not make Workflow OS
a hosted identity provider, enterprise directory, or production authorization
service. The kernel remains local-first and preview.
