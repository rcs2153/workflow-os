# Required Context Contract Consumption Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to required-context contract consumption core model
and pure helper.**

The plan defines a narrow, deterministic bridge from typed context
requirements to the accepted governed context projection. It preserves
least-privilege and payload-free boundaries and does not authorize runtime
context access.

## 2. Scope Verification

The plan stays within planning scope. It does not authorize payload
dereference, repository reads, runtime consumption, schemas, SDKs, CLI
behavior, providers, OpenShell, persistence, events, SideEffect execution,
writes, hosted administration, enterprise identity, or release changes.

## 3. Contract Boundary Assessment

The plan correctly identifies that the existing name-only
`HarnessContextRequirement` cannot be enforced safely. It forbids inferred
name-to-target conventions and recommends a typed companion boundary before
any compatibility or schema decision.

The proposed requirement fields are sufficient for a first model:

- stable requirement identity;
- exact typed target;
- exact access level;
- required or optional obligation;
- sensitivity ceiling; and
- immutable contract identity and content hash.

## 4. Authority Assessment

The plan preserves the central invariant that a declaration does not grant
authority. It requires an already validated governed context projection and
exact capability-backed entries.

Approval is also kept separate from authority and availability. The plan
correctly forbids using approval to manufacture missing required context.

## 5. Least-Privilege Assessment

Exact access-level matching is appropriately conservative. Allowing
bounded-metadata authority to satisfy a reference-only requirement would expose
more than the contract requested.

Rejecting extra projection candidates and entries is also correct. A consumer
that ignored undeclared context would normalize ambient overexposure.

## 6. Required And Optional Semantics

Required gaps block. Optional gaps remain explicit and bounded. Neither path
fabricates targets, citations, evidence, authority, or availability.

The result should retain source requirements and the projection so
deserialization can recompute exact satisfaction and gap posture. The plan
states this requirement clearly.

## 7. Immutable And Runtime Assessment

Contract ID and version are not treated as sufficient proof of reviewed
content. The plan requires content-addressed binding and defers immutable-run
composition to a later reviewed phase.

The plan also correctly states that a satisfied result is not a lease.
Time-of-use capability, policy, availability, sensitivity, and immutable
context checks remain mandatory before any future dereference.

## 8. Proportional Governance Assessment

The plan separates execution eligibility from operator disclosure:

- missing required context remains blocking;
- optional gaps may influence quiet or visible disclosure;
- disclosure cannot weaken the contract; and
- relevant changes require reassessment.

This aligns with the accepted two-axis proportional-governance correction and
the fresh-pull evaluator's recommendation to reduce low-risk ceremony without
losing evidence.

## 9. OpenShell Assessment

The plan positions OpenShell correctly as a possible optional containment
provider after Workflow OS resolves authority and required context. It does not
allow sandbox availability to become authority and does not authorize an
integration or fork.

## 10. Test Plan Assessment

The future test plan covers valid consumption, every target kind, exact
matching, required and optional gaps, overbroad projections, immutable
identity, context mismatch, deterministic serde, privacy, and regression
behavior. It tests behavioral invariants rather than construction alone.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Decide during implementation whether the typed contract is reusable at the
  workflow-step level or initially harness-specific.
- Select the existing immutable-bundle owner for the content-hash binding.
- Keep future access-lattice semantics separate from the exact-match first
  slice.
- Define sandbox attestation only after a real read-only dereference boundary
  is accepted.

## 13. Recommended Next Phase

Implement the **required-context contract consumption core model and pure
helper only**.

Do not add target dereference, runtime consumption, schemas, SDKs, CLI
behavior, providers, OpenShell, persistence, events, authority receipts,
SideEffect execution, writes, hosted administration, enterprise identity,
reasoning lineage, or release changes.
