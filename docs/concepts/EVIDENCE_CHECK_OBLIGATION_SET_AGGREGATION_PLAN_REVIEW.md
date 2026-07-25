# Evidence And Check Obligation-Set Aggregation Plan Review

## 1. Executive Verdict

Needs planning blocker fixes.

The plan correctly identifies complete coverage as the missing safety boundary,
but its first implementation sequence still permits an explicitly supplied set
to produce an aggregate result while admitting that no authoritative canonical
declaration source exists. That ambiguity could let caller assertion become
governance authority.

## 2. Scope Verification

The plan remains planning-only. It does not authorize implementation, schemas,
executor integration, reassessment, persistence, CLI behavior, providers,
SideEffects, or writes.

## 3. Strengths

- It separates repository recommendations from enforceable declarations.
- It binds future sets to immutable run and declaration identity.
- It rejects incomplete, duplicate, unexpected, mismatched, stale, and
  unsupported coverage.
- It keeps execution disposition and disclosure obligation independent.
- It preserves monotonic proportional-governance minima.
- It defines bounded, payload-free results and non-leaking errors.

## 4. Blocker: Untrusted Set Can Produce Aggregate-Looking Output

The plan states that today's schemas do not provide a complete canonical
declaration source and that the first model remains unwired. It then proposes a
pure aggregator over an explicitly supplied validated set and tests complete
coverage producing `Satisfied`.

Structural validation can prove that contributions exactly cover the supplied
set. It cannot prove that the supplied set contains every obligation the kernel
must enforce. A private caller can still omit an obligation before constructing
the set.

The correction must split two outputs:

1. a **structural coverage candidate** that proves exact coverage of an
   explicitly supplied set but is not a `GovernanceWorkloadEvidenceCheckPosture`
   and carries no authority; and
2. a future **authoritative aggregate** that is available only after the set is
   derived from canonical declarations stored in the immutable bundle.

The first model-only implementation may build and test the structural
candidate. It must not name or expose it as an authoritative aggregate or map
it to proportional governance.

## 5. Blocker: V1 Obligation Kind Is Undecided

The candidate model includes generic evidence and check obligation kinds while
the only accepted contribution is a private `DocsCheck` local-check
attestation. The plan leaves open whether evidence and checks share one set.

The first implementation must support only the current local-check attestation
obligation kind. Generic evidence obligations and additional check families
remain future vocabulary after their identity and source rules are planned.

This keeps the model phase reviewable and prevents speculative generic types
from becoming accidental public architecture.

## 6. Required Additional Decisions

The blocker fix should also state:

- an unresolved or caller-asserted source can never produce aggregate workload
  posture;
- a canonical authoritative empty set may eventually be vacuously satisfied,
  but an absent or unresolved set maps to `Unknown`, not `Satisfied`;
- an optional check that runs and fails remains `Failed`; optionality permits
  absence, not failed acceptance criteria; and
- the accepted `DocsCheck` leaf contribution should be adapted in the same call
  stack or by a private identity-checked adapter, not serialized or recreated.

## 7. Privacy And Failure Assessment

The proposed privacy posture is adequate. Errors and Debug output are bounded
and exclude IDs, commands, paths, source content, process output, credentials,
provider payloads, and proof material.

## 8. Test Plan Assessment

The proposed coverage tests are strong but must distinguish structural
candidate tests from future authoritative aggregation tests. A positive
structural coverage test should assert explicitly that its output cannot be
converted to `GovernanceWorkloadEvidenceCheckPosture`.

Future authoritative tests must use a canonical stored declaration source and
prove omitted declaration records cannot be hidden by caller input.

## 9. Planning Blockers

1. Split structural coverage candidate from authoritative aggregate posture.
2. Narrow v1 to local-check attestation obligations only.
3. Define empty, optional-failure, and leaf-adaptation semantics explicitly.

## 10. Non-Blocking Follow-Ups

- Decide the future schema location for canonical declarations.
- Decide whether evidence and executable checks later share an envelope or use
  typed sub-sets.
- Define steward constraint composition only after local declaration semantics
  are stable.

## 11. Governed Review

- workflow: `dg/review`
- run: `run-1784959466512836000-2`
- approval: `approval/run-1784959466512836000-2/review-scope-approved`
- presentation: `presentation/c70c91118c64799a`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; analysis, documentation, and
  validation ran outside the kernel

## 12. Validation

- `npm run check:docs` - passed;
- `git diff --check` - passed.

## 13. Recommended Next Phase

The focused correction is documented in the
[Evidence And Check Obligation-Set Aggregation Plan Blocker Fix Report](EVIDENCE_CHECK_OBLIGATION_SET_AGGREGATION_PLAN_BLOCKER_FIX_REPORT.md).

Perform focused re-review. Do not implement the model until the correction is
accepted.
