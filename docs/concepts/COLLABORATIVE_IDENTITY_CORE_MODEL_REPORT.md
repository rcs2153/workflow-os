# Collaborative Identity Core Model Report

## 1. Executive Summary

Workflow OS now has a domain-neutral, model-only collaborative identity
lifecycle boundary. The Core model separates provider trust registration,
opaque external-subject binding, principal lifecycle, exact-project grants,
additive revocation, and point-in-time current authority.

This phase does not authenticate users, persist identity records, change hosted
runtime behavior, or authorize provider mutations. Its purpose is to make the
future identity and revocation boundary explicit, deterministic, inspectable,
and fail-closed before transport or storage is added.

## 2. Scope Completed

- Added validated identity-provider registration with canonical audience
  ordering and commitment verification.
- Added opaque external-subject-to-principal bindings without raw subject
  storage.
- Added active, disabled, and superseded binding lifecycle vocabulary.
- Added active and disabled principal lifecycle records.
- Added immutable exact-project grants using the existing closed
  `HostedProjectCapability` vocabulary.
- Added create-only revocations with exact replay reconciliation and conflict
  rejection.
- Added complete-set validation that rejects duplicate binding IDs, one active
  subject mapped to multiple principals, and one principal with multiple active
  external identities.
- Added a trusted constructor-only current-authority projection that rejects
  inactive identity facts, stale revisions, cross-scope grants, ambiguous
  revocations, orphaned revocations, expired grants, and fully revoked grants.
- Added an explicitly untrusted deserializable projection shape for future
  evidence or persistence work.
- Exported the model through `workflow-core` and added focused tests.

## 3. Scope Explicitly Not Completed

- No OIDC, JWT, JWKS, login, session, refresh, or logout implementation.
- No PostgreSQL, SQLite, or other identity/grant persistence.
- No hosted authentication cutover or principal-filtered inbox.
- No SCIM, directory synchronization, groups, invitations, or admin UI.
- No tenant administration, enterprise IdP configuration, or hostile-tenant
  isolation claim.
- No workflow schema, CLI, SDK, example, or release change.
- No provider write, OpenShell, nested harness, or broader mutation behavior.
- No production identity or authorization claim.

## 4. Model And API Summary

The model adds provider, binding, grant, and revocation identifiers; positive
provider, principal, and grant revisions; opaque issuer, audience, subject,
grant, revocation, and projection commitments; lifecycle and revocation reason
enums; and validated records for each authority fact.

`validate_collaborative_identity_binding_set` validates complete binding sets.
`reconcile_collaborative_project_grant_revocation` distinguishes create, exact
replay, and conflicting replay. `project_collaborative_current_authority`
constructs one trusted point-in-time projection from explicit current facts.

## 5. Validation Boundary Summary

Construction rejects empty, oversized, malformed, duplicate, secret-like, or
cross-scope inputs with stable non-leaking errors. Provider, grant, and
revocation deserialization reconstructs through validated constructors and
verifies canonical commitments. Trusted current authority is not ordinarily
deserializable and can only be produced by the resolver.

The resolver requires active provider, binding, and principal records; matching
organization, provider, revision, and principal context; non-future identity
facts; an exact project/principal grant set; and at least one effective,
unexpired, unrevoked grant.

## 6. Revocation And Time-Of-Use Summary

Grants are immutable. Revocation is additive and binds the exact grant ID,
commitment, organization, project, principal, reason, actor, and effective
time. Exact replay is idempotent; conflicting replay fails closed.

The projection records its evaluation time and source authority revision. It is
a point-in-time decision artifact, not a reusable bearer credential. Trusted
database time, persistence concurrency, active-session invalidation, and
runtime rechecks remain deferred.

## 7. Redaction And Privacy Summary

The model stores opaque subject commitments rather than raw external subject
values. It does not require emails, usernames, display names, provider claims,
groups, raw tokens, credentials, signing material, or discovery payloads.
Debug output redacts identity and scope identifiers. Deserialization failures
and validation errors use bounded stable messages without echoing input values.

## 8. Test Coverage Summary

Focused tests cover canonical provider serialization, tamper rejection,
duplicate audiences, invalid revisions, lifecycle round trips, raw-subject
absence, secret-like input rejection, canonical grant ordering, duplicate
capabilities, invalid time windows, additive revocation, replay conflict,
active authority union, expired and revoked exclusion, inactive identity facts,
stale authority, cross-project contamination, ambiguous active bindings,
trusted/untrusted projection separation, and absence of fake authority.

## 9. Validation Commands And Results

- Focused `workflow-core` collaborative identity tests: passed, 11 tests.
- Focused collaborative identity clippy with warnings denied: passed.
- `cargo fmt --all --check`: passed after formatting one focused test block.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed. An initial run performed concurrently with
  a clean clippy build caused ten existing live-deadline scheduler tests to
  expire; all 451 Core library tests and then the complete workspace passed
  when rerun without competing compilation load.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- No external identity assertion is validated.
- No durable identity, principal, grant, or revocation store exists.
- Trusted time and concurrent revocation semantics are not implemented.
- Current authority is not wired into hosted request handling, approval routing,
  capability consumption, provider invocation, or SideEffect transitions.
- The one-active-binding-per-principal rule is conservative and may require a
  separately reviewed multi-identity policy later.
- The current deployment bearer and static hosted registry remain evaluation
  compatibility surfaces rather than production identity.

## 11. Recommended Next Phase

Perform focused maintainer/security review of the Collaborative Identity Core
Model. Review canonicalization, commitment verification, complete-set
ambiguity rejection, exact-project isolation, additive revocation,
constructor-only trusted projection, privacy, and non-leaking failures.

Do not begin external identity transport, persistence, hosted cutover,
administration, provider writes, schema exposure, or production identity work
before that review.

## 12. Governed Phase Evidence

- Dogfood workflow: `dg/implement`.
- Run: `run-1792541359152388000-2`.
- Approval: `approval/run-1792541359152388000-2/implementation-approved`.
- Approval presentation: `presentation/e195a85aacb0cfe6`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval grant,
  six governed skill steps, zero retries, and zero escalations.
- Out-of-kernel work: repository inspection, Rust and documentation edits,
  validation, and later git/PR operations are performed by the delegated
  maintainer; the kernel governs phase scope and records the approval/event
  trail.
