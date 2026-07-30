# Local Unsigned Authority Receipt Review

## 1. Executive Verdict

Phase accepted after focused blocker fixes; production issuance remains a
separate phase.

Independent review found that the initial source-backed draft did not
authenticate serialized provenance and could issue before a concrete operation
outcome. The fix removes production issuance and separates trusted receipts
from unverified serialized claims.

## 2. Scope Verification

The corrected phase stays within the approved model boundary. It does not add
runtime authorization, dereference, tool or provider execution, OpenShell,
access material, SideEffects, writes, persistence, events, schemas, CLI
behavior, hosted administration, signatures, or release changes.

## 3. Model Assessment

The receipt vocabulary binds exact immutable execution identity, one
required-context obligation, a capability and resource commitment, a selected
grant, and source assessment commitments.

Its fixed `point_in_time_only`, `local_unsigned`, and
`evidence_only_not_authorization` postures prevent the model from claiming
freshness, issuer authenticity, or reusable permission.

## 4. Trust Boundary Assessment

No public or crate-visible production constructor can mint a trusted receipt.
The trusted type is serialize-only. Deserialization yields an explicitly
unverified claim with no conversion into the trusted type.

This is the appropriate model-only fix. An unkeyed deterministic hash proves
field consistency, not issuer or source authenticity. Production issuance
must wait for one concrete Core-owned operation that resolves current
authority and binds a successful outcome.

## 5. Validation And Compatibility Assessment

Every field participates in a versioned, length-framed commitment. Receipt ID
is derived from that commitment. Unknown wire fields and substituted committed
fields fail closed for unverified claims.

The fixed V1 vector protects deterministic compatibility. Validation is
documented as internal consistency only and cannot be mistaken for source
authentication.

## 6. Privacy Assessment

The model stores references and commitments only. It contains no raw context,
provider payload, command output, path, environment value, credential, policy
body, approval body, evidence body, or check output.

Manual `Debug` redacts identities, timestamps, and commitments. Deserialization
errors are bounded and do not echo substituted values.

## 7. Test Quality Assessment

Focused tests prove deterministic model identity, fixed-vector stability,
trusted-to-unverified serialization, continued unverified posture for a
different self-consistent claim, tamper rejection, and non-leakage.

A future external-schema phase should add independent cross-language vectors.
That is not a blocker because schema and SDK exposure remain out of scope.

## 8. Blockers

None for the model-only phase.

The re-review confirms that serialized input cannot become trusted evidence by
self-consistency, no crate-wide production minting API exists, and successful
operation-outcome binding remains an explicit prerequisite for the next
producer phase.

## 9. Non-Blocking Follow-Ups

- Add one reviewed read-only receipt production path only after blocker-fix
  acceptance.
- Add durable audit projection only after persistence semantics are planned.
- Add policy, approval, evidence, and check citations only from independently
  evaluated prerequisites.
- Evaluate OpenShell later as an optional execution provider, never as an
  authority source.

## 10. Recommended Next Phase

Focused blocker-fix re-review. If accepted, proceed to one opt-in Core-owned
read-only receipt production path. It must freshly resolve current authority,
issue only after successful operation outcome, and never accept a receipt as
permission.

## 11. Governed Review Record

This review is part of the approved `dg/implement` phase:

- run ID: `run-1785417096382389000-2`;
- approval ID:
  `approval/run-1785417096382389000-2/implementation-approved`;
- presentation ID: `presentation/739c56ff4340a055`;
- approval outcome: granted under delegated-maintainer authority; and
- approval-presentation enforcement: proof persisted before execution.
