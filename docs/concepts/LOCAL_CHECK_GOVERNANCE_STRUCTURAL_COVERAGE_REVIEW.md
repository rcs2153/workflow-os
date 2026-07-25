# Local Check Governance Structural Coverage Review

## 1. Executive Verdict

Needs blocker fixes.

The private structural evaluator is deterministic, bounded, and correctly
unwired from aggregate governance. However, candidate construction still
allows an opaque leaf obligation fingerprint to be relabeled under unrelated
candidate bundle metadata. That leaves one cross-bundle substitution route
open at the private adapter boundary.

Fix-forward status: the blocker is addressed in the
[Local Check Governance Structural Coverage Blocker Fix Report](LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_BLOCKER_FIX_REPORT.md).
This finding remains the original review verdict until focused re-review.

## 2. Scope Verification

The phase stayed within the approved private model-only scope. It introduced
no public export, serde model, canonical declaration source, aggregate workload
conversion, proportional-governance reassessment, executor checkpoint,
persistence, schema, CLI behavior, provider call, SideEffect, or write.

## 3. Model Assessment

The candidate model is appropriately narrow for the current `DocsCheck`
attestation family. It binds candidate identity to bundle, workflow, run, and
step metadata and keeps declaration provenance explicitly unresolved.

Required and optional obligations are representable. Structural results expose
bounded counts, disposition, and redacted fingerprints only. No result can be
converted into `GovernanceWorkloadEvidenceCheckPosture`.

## 4. Coverage Semantics Assessment

The evaluator correctly:

- rejects duplicate expected obligations;
- rejects duplicate and unexpected contributions;
- rejects contribution requirement-level mismatches;
- distinguishes missing required from missing optional coverage;
- preserves executed optional failure as `Failed`;
- applies failure-first precedence;
- canonicalizes input ordering; and
- leaves an empty candidate structurally vacuous but source-unresolved.

One successful contribution cannot mask failed or missing coverage.

## 5. Binding Integrity Blocker

`LocalCheckGovernanceObligationSetCandidateDefinition` accepts an already
computed obligation fingerprint alongside independently supplied bundle,
workflow, run, and step fields. The private `DocsCheck` adapter checks that the
leaf fingerprint appears in the candidate, then binds the adapted contribution
to that candidate-set fingerprint.

The leaf fingerprint already commits to the original bundle and step, but the
candidate cannot verify that opaque commitment against its separately supplied
binding fields. A caller inside the crate can therefore place a leaf
fingerprint from bundle A into a candidate labeled as bundle B, adapt it, and
obtain structurally satisfied coverage for the relabeled candidate.

The new evaluator regression proves that a contribution already bound to
candidate A cannot be evaluated against candidate B. It does not cover this
construction-time relabeling path.

The fix should derive the local-check obligation fingerprint from candidate
binding fields plus the exact requirement fingerprint, or carry enough typed
private binding material to compare exact bundle, workflow, run, step, and
requirement identity before adaptation. It must not trust an opaque caller-
supplied leaf fingerprint as both obligation declaration and binding proof.

## 6. Source Authority Assessment

The model correctly avoids claiming canonical declaration authority.
`source_posture: unresolved` is visible in bounded Debug output, and no
aggregate conversion exists. The blocker concerns identity integrity inside
the structural candidate, not accidental aggregate authority.

## 7. Privacy And Redaction Assessment

No raw command output, source content, path, environment value, credential,
token, provider payload, or evidence payload is stored. Debug output redacts
binding identities and fingerprints. Stable validation errors do not echo
supplied identities.

## 8. Test Quality Assessment

The focused suite covers complete and missing coverage, precedence,
duplicates, unexpected inputs, requirement-level mismatches, ordering,
identity changes, empty candidates, and Debug/error non-leakage. The complete
workspace suite remains green.

Missing blocker regression:

- construct a candidate labeled with bundle B from a `DocsCheck` leaf or
  requirement originating in bundle A;
- prove the private adapter fails before producing a contribution.

## 9. Validation

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test -p workflow-core --lib local_check_attestation` - passed, 34
  tests;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed before this review document; and
- `git diff --check` - passed before this review document.

## 10. Blockers

1. Candidate obligation construction must cryptographically and structurally
   bind the exact candidate bundle/workflow/run/step context to the requirement
   instead of accepting an opaque leaf fingerprint under independently
   supplied candidate metadata.
2. Add a direct construction-time cross-bundle relabeling regression test.

## 11. Non-Blocking Follow-Ups

- Clarify in the plan that future `Unknown` aggregate posture is outside the
  v1 structural disposition vocabulary.
- When canonical declarations are planned, derive requirement level from the
  frozen declaration set rather than adapter caller input.

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784963847008182000-2`
- approval: `approval/run-1784963847008182000-2/review-scope-approved`
- presentation: `presentation/422fbe627eba8d8f`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code inspection,
  documentation, and validation ran outside the kernel

## 13. Recommended Next Phase

Run a focused blocker-fix phase. Derive or verify exact local-check obligation
identity from the candidate binding and requirement material, add the missing
cross-bundle relabeling regression, and repeat phase-level review. Do not begin
canonical declaration schemas, aggregate conversion, reassessment, or executor
integration yet.
