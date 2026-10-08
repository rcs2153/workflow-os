# Explicit Zero Required-Context Contract Plan Review

## 1. Executive Verdict

**Plan accepted with one mandatory implementation condition; proceed to the
explicit-zero prerequisite model implementation.**

The plan closes the real semantic gap found during trusted-host admission
implementation without treating absence as authority. The model is narrow,
fail closed, and compatible with the accepted no-authority docs-check slice.

The mandatory condition is that established non-empty contract hashes and
wire output remain byte-for-byte unchanged. Explicit zero must use a separate
marker and hash domain rather than rebinding existing contracts.

## 2. Scope Verification

The plan remains prerequisite-only. It does not authorize the trusted-host
binary, admission wiring, additional profiles, provider access, writes,
released CLI, SDKs, schemas, examples, hosted execution, OpenShell, nested
harnesses, or release changes.

## 3. Model Assessment

An explicit closed posture is the correct distinction:

- `DeclaredRequirements` means a canonical non-empty requirement set;
- `ExplicitNone` means a complete, intentional declaration of zero required
  context; and
- missing, unknown, omitted, incomplete, or arbitrary empty input remains
  invalid.

This is preferable to a sentinel requirement because a sentinel would
fabricate a context target and contaminate consumption, evidence, and authority
semantics.

## 4. Construction Boundary

The existing public constructor must continue to reject empty requirements.
The first explicit-zero issuer must be private and must require Core-owned proof
that the exact canonical docs-check profile declares zero capability grants,
zero capability-availability facts, and zero governed-context references.

The private construction proof must not be reusable as execution authority and
must not cross into the application.

## 5. Hash And Compatibility Assessment

The plan correctly identifies compatibility as a stop condition. Review makes
the following requirement mandatory:

- non-empty contracts retain the existing
  `required-context-contract-v1` hash algorithm and bytes;
- their existing serialized output remains unchanged;
- explicit zero uses a new domain-separated hash path; and
- the explicit marker participates in validation and identity.

Implementation must not add a serialized default field to every established
contract. If custom serde cannot preserve the existing shape, implementation
must stop for another compatibility review.

## 6. Serde Assessment

The proposed compatibility behavior is sound:

- legacy non-empty wire values without a marker remain valid;
- explicit zero requires its marker plus an empty list;
- an empty list without the marker remains invalid;
- explicit zero plus non-empty requirements is invalid; and
- unknown markers fail closed.

Errors must remain stable and payload-free. There must be no
`#[serde(default)]` behavior that turns an omitted marker or list into explicit
zero.

## 7. Consumption Assessment

Zero projections may be accepted only after the contract validates as
`ExplicitNone`. The result may use the existing `Satisfied` posture with zero
satisfactions and gaps because the retained contract and execution context
make the proof inspectable.

Any projection supplied to explicit zero must fail closed. Existing non-empty
contracts must continue to reject an empty projection set and require an exact
target/access match.

## 8. Current-Authority Assessment

Complete empty inventories are valid only for the exact explicit-zero
execution binding and closed docs-check profile. The source configuration
commitment must include the explicit-zero contract identity and posture,
profile command-contract fingerprint, exact execution binding, and three
complete empty inventory declarations.

Missing source state, unknown completeness, stale validity, non-empty or
conflicting facts, a substituted profile, or a non-zero contract must block.
Policy, approval, evidence, checks, operational opening, and dispatch remain
independent.

## 9. Privacy And Failure Assessment

The plan preserves bounded Debug and error output. It adds no payload-bearing
authority or context. Invalid combinations fail before authority resolution or
execution, and no fallback converts invalid data into explicit zero.

## 10. Test Assessment

The planned tests cover compatibility, construction privacy, marker and hash
determinism, malformed wire combinations, zero and non-zero consumption,
profile substitution, authority completeness/freshness, and regression
surfaces. Implementation must add a direct golden assertion for at least one
existing non-empty contract hash and serialized value to prove the mandatory
compatibility condition.

## 11. Blockers

None for the prerequisite model implementation, subject to the mandatory
compatibility condition.

## 12. Non-Blocking Follow-Ups

- Keep the explicit-zero constructor crate-private even if later profiles need
  the same posture; broaden issuance only through separate review.
- Reconsider whether a public posture accessor is needed only when a real
  consumer requires it.
- Keep schema exposure separate from the internal Core representation.

## 13. Recommended Next Phase

Implement only the explicit-zero required-context prerequisite:

```text
compatibility-preserving contract posture
  -> private closed docs-check issuance
  -> exact zero-projection consumption
  -> exact empty registered-authority resolution
  -> focused tests and full validation
```

After focused implementation review, resume the trusted-host local application
admission vertical slice.

## 14. Validation

- `npm run check:docs`: required and passed for the planning source before
  review.
- `git diff --check`: required and passed for the planning source before
  review.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791391641930523000-2`
- approval: `approval/run-1791391641930523000-2/review-scope-approved`
- presentation: `presentation/6743bce4fdd57ccc`
- presentation hash:
  `6743bce4fdd57ccc32aedc33df3bf61b21d70e0a8f7554d8d36e2ee3a6bc7a4a`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused documentation-only maintainer/security review
- phase status: completed
- out-of-kernel work: plan and source inspection, review authoring,
  documentation validation, and later git or pull-request work
