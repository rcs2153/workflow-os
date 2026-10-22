# Collaborative Identity Core Model Review

## 1. Executive Verdict

Needs blocker fixes.

The model is domain-neutral, deterministic, privacy-conscious, and correctly
limited to provider registration, opaque subject binding, principal lifecycle,
exact-project grants, additive revocation, and point-in-time projection. The
implementation stays within model-only scope and its focused and workspace
tests pass.

One authority-boundary blocker prevents acceptance: the public projection
function accepts caller-assembled identity and grant records, a caller-supplied
authority revision, and a caller-supplied evaluation time, then returns a type
named and documented as trusted current authority. The function validates
coherence, but it cannot establish authoritative provenance, complete current
fact coverage, or trusted time. The projection is not consumed by runtime code,
so the defect is contained. It must be fixed before persistence, protocol
handling, or authority consumption is added.

## 2. Scope Verification

The phase stayed within the approved Core-model scope. It added one model
module, exports, focused tests, documentation status updates, and an
implementation report.

It did not add OIDC/JWT/JWKS handling, login or sessions, identity persistence,
hosted authentication cutover, a principal inbox, SCIM, groups, invitations,
administration, provider writes, workflow schema changes, CLI behavior,
examples, or release posture changes.

## 3. Model Assessment

The model separates the intended concepts cleanly:

- provider registration is organization-scoped and commitment-bound;
- external subjects are stored only as opaque commitments;
- bindings and principals carry explicit lifecycle;
- grants are immutable, exact-project, and reuse the closed
  `HostedProjectCapability` vocabulary;
- revocations are create-only and bind the exact grant commitment; and
- projected capability sets are deterministic and payload-free.

The model is appropriately conservative about one active binding per principal.
`validate_collaborative_identity_binding_set` rejects duplicate binding IDs,
one active provider subject mapped to multiple principals, and one principal
with multiple active external identities. Disabled and superseded history
remains representable.

## 4. Canonicalization And Serde Assessment

Provider audiences and grant capabilities are sorted before commitment
construction, and duplicates are rejected. Provider, grant, and revocation
deserialization reconstructs through validated constructors and verifies the
stored commitment. Unknown fields fail closed.

The trusted projection type intentionally does not implement `Deserialize`.
Serialized projection data may deserialize only into
`UnverifiedCollaborativeCurrentAuthorityProjection`, which has no conversion
into the trusted type. That is a sound wire-boundary design.

Provider registration currently exposes its canonical commitment through the
general `SpecContentHash` type rather than a provider-specific commitment type.
This is not a blocker because the field remains canonical and verified, but a
typed provider-registration commitment would reduce accidental cross-domain
substitution in a later persistence model.

## 5. Binding And Principal Lifecycle Assessment

Binding lifecycle supports active, disabled, and superseded states. Principal
lifecycle supports active and disabled states. Current projection correctly
rejects inactive providers, bindings, and principals, future identity facts,
provider/revision mismatch, and principal mismatch.

Bindings and principal records do not carry their own canonical commitments.
That is acceptable for the current in-memory model-only phase because they are
not persisted or consumed as evidence. A persistence phase must add immutable
record identity and canonical commitment before these records can participate
in durable current-authority resolution.

## 6. Grant And Revocation Assessment

Grants bind organization, project, principal, capabilities, issuer, grant
revision, authority-registry revision, effective time, and optional expiry.
Cross-project or cross-principal grant contamination fails closed rather than
being silently filtered.

Revocations bind the exact grant ID and commitment plus organization, project,
principal, time, reason, and revoking actor. Exact replay reconciles;
conflicting replay fails closed. Orphaned, mismatched, future, and duplicate
revocation state is rejected during projection. Historical grants and
revocations remain inspectable.

## 7. Current-Authority Blocker

`project_collaborative_current_authority` is public and accepts:

- publicly constructible provider, binding, principal, grant, and revocation
  records;
- a caller-supplied `authority_registry_revision`;
- slices described as complete but with no completeness proof or source
  capability; and
- a caller-supplied `evaluated_at` described as trusted time.

It then returns `CollaborativeCurrentAuthorityProjection`, documented as a
trusted point-in-time authority projection. A caller can therefore construct a
self-consistent authority universe and receive a value whose type claims more
than Core has established. Preventing ordinary deserialization does not close
this source-provenance gap.

The blocker fix must choose one narrow posture:

1. demote the public result to a validated, explicitly non-authorizing
   projection claim; or
2. make trusted projection construction private to a Core-owned resolver that
   consumes a registered authoritative source, a complete fact-set proof, and
   trusted time.

The first option is the smallest model-only fix. A future persistence/runtime
phase may add the second option after source, completeness, revision, and time
semantics are reviewed. No public caller-authored type should be described as
trusted current authority before then.

## 8. Privacy And Error Assessment

The model stores no raw external subject, token, provider claim, email,
username, display name, group list, credential, signing material, discovery
payload, or unrestricted metadata. Debug output redacts identity and scope
identifiers. Stable errors do not echo caller values, and secret-like test
values do not leak through errors, Debug, or serialization.

The model's conservative rejection of secret-like identifiers is stricter than
ordinary identity systems may eventually require. That is acceptable for this
preview model but should be revisited only with an explicit privacy and display
metadata design.

## 9. Test Quality Assessment

Focused coverage is strong for canonical ordering, serde round trips, tamper
rejection, duplicate audiences and capabilities, lifecycle, raw-subject
absence, non-leaking failures, time windows, revocation replay/conflict,
expired and revoked grants, stale authority revisions, inactive identity facts,
cross-project contamination, ambiguous active bindings, and trusted/untrusted
wire separation.

Missing blocker-fix tests should prove:

- public caller-authored facts cannot produce a type represented as trusted
  authority;
- a validated projection claim is explicitly non-authorizing if the demotion
  approach is chosen;
- any future trusted resolver requires a non-forgeable source boundary,
  complete fact-set proof, and trusted time; and
- serialized claims remain unable to regain trusted posture.

Persistence, concurrency, database-time, migration, key rotation, session
revocation, and active runtime-consumption tests remain correctly deferred.

## 10. Documentation Assessment

The implementation plan, roadmap, planning report, planning review, and
implementation report accurately state that the Core model is implemented and
that protocol, persistence, hosted cutover, administration, provider writes,
schemas, CLI behavior, and production identity remain unsupported.

The implementation report currently calls the projection trusted. That wording
matches the implementation but is the blocked claim. The blocker-fix phase must
update both code and documentation to the chosen validated-claim or
Core-owned-source posture; the original finding must remain visible here.

## 11. Blockers

1. **Caller-authored facts can produce a type represented as trusted current
   authority.** Demote the public projection to an explicitly non-authorizing
   validated claim, or require a Core-owned authoritative source, completeness
   proof, and trusted time before producing trusted authority.

## 12. Non-Blocking Follow-Ups

- Add a provider-registration-specific commitment type before durable storage.
- Add canonical commitments and immutable record identities to binding and
  principal lifecycle records in the persistence plan.
- Define administrative issuance/revocation capabilities before grant mutation
  APIs.
- Define database-time, revision high-watermark, and concurrent revocation
  semantics before runtime consumption.
- Preserve one active identity per principal unless a separately reviewed
  multi-identity policy is introduced.

## 13. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed. The existing live-deadline scheduler tests
  first expired while a clean clippy build ran concurrently; isolated Core and
  subsequent full-workspace reruns passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 14. Recommended Next Phase

Collaborative Identity Core Model blocker fix.

Implement the smallest fix that prevents caller-authored model facts from
producing a type represented as trusted current authority. Prefer demoting the
public projection to a validated, explicitly non-authorizing claim in this
model-only phase. Defer a genuinely trusted projection until a Core-owned
authoritative source, complete fact-set proof, and trusted-time boundary are
implemented and reviewed.

Do not begin identity transport, persistence, hosted authentication cutover,
administration, provider writes, schema exposure, or production identity work.

## 15. Governed Phase Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792562074758344000-2`.
- Approval: `approval/run-1792562074758344000-2/review-scope-approved`.
- Approval presentation: `presentation/941c471011a22aa5`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval grant,
  six governed skill steps, zero retries, and zero escalations.
- Out-of-kernel work: repository inspection, security analysis,
  documentation edits, validation, and later git/PR operations are performed by
  the delegated maintainer; the kernel governs review scope and records the
  approval/event trail.

## 16. Fix-Forward Status

The blocker is fixed forward without erasing this finding. The public resolver
and trusted-looking result vocabulary were replaced by
`validate_collaborative_authority_projection_claim` and
`CollaborativeAuthorityProjectionClaim`. The result carries a fixed serialized
`non_authorizing` posture, exposes descriptive capability membership rather
than permission semantics, and has no conversion into trusted current
authority. Authoritative source provenance, complete current fact coverage,
and trusted time remain deferred prerequisites for any future authorizing
projection.
