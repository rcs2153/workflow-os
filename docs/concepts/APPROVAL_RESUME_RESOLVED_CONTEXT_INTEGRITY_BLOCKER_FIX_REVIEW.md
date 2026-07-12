# Approval Resume Resolved-Context Integrity Blocker Fix Review

Review date: 2026-07-12

## 1. Executive Verdict

**Blocker fixed with non-blocking follow-ups; proceed to immutable run-bundle
boundary planning.**

The fix closes the confirmed approval/resume TOCTOU boundary for current local
executor grant paths. One review blocker was found and fixed: candidate-context
reconstruction errors could have surfaced loader diagnostics containing paths.
Those failures now use a stable, bounded approval-specific error before any
durable mutation.

## 2. Scope Verification

The implementation stayed within the approved blocker scope. It added one
backward-readable approval field, deterministic commitment computation,
pre-mutation resume validation, focused tests, and honest documentation.

It did not add a durable run bundle, raw spec persistence, handler attestation,
provider calls or writes, CLI behavior, schemas, examples, hosted runtime,
RBAC/IdP integration, automatic approval, reasoning lineage, or release posture
changes.

## 3. Ordering Assessment

All approval APIs converge on `apply_approval_decision`. For granted decisions,
that function now builds and validates the candidate resume plan before:

- `ApprovalGranted`;
- resume-policy audit records;
- `RunResumed`;
- skill, hook, or SideEffect execution events.

The already validated candidate receives the advanced event builder only after
the grant-side records succeed. Mismatch and reconstruction failures therefore
leave the durable waiting run untouched.

Denial remains independent of current project reconstruction and continues to
append `ApprovalDenied` followed by the existing terminal fail-closed result.

## 4. Commitment Assessment

The v1 commitment is domain-separated and deterministic. It covers the
canonical workflow hash, ordered step and resolved-skill identities and hashes,
referenced-policy identities and hashes, checkpoint requirements, hook-input
presence, SideEffect input counts, and report-artifact policy posture.

The workflow hash covers complete workflow declaration changes. Skill and
referenced-policy hashes close the same-ID/version drift gap. Policy references
are deduplicated and sorted, and checkpoint IDs are sorted and deduplicated.
Unreferenced policy changes correctly remain outside the approval boundary.

No raw definitions, paths, prompts, provider payloads, command output,
credentials, or approval reasons enter the commitment.

## 5. Compatibility Assessment

The optional field permits historical event JSON to deserialize. New approval
requests always populate it. Tests reconstruct legacy approval events without
the field and verify:

- grant fails with `executor.approval.resume_context_missing`;
- no grant event is appended and the approval remains undecided;
- denial remains available and fails the run through existing semantics.

No historical commitment is inferred from current files.

## 6. Error And Privacy Assessment

Fixed mismatch, missing-commitment, workflow-identity, and reconstruction error
codes use bounded messages without IDs, paths, hashes, snippets, reasons,
payloads, credentials, or token-like values.

Focused review found that raw project reconstruction errors were initially
propagated. The fix-forward mapping now returns
`executor.approval.resume_context_unavailable`, and a missing-skill regression
verifies path non-leakage and zero grant mutation.

## 7. Test Quality Assessment

Coverage verifies unchanged grant behavior; workflow, skill, referenced-policy,
and missing-skill changes; unreferenced policy changes; deterministic
commitments; required checkpoint non-loss; legacy grant and denial behavior;
waiting status, event count, approval decision, and handler-call preservation;
and the existing presentation, high-assurance, report, artifact, SideEffect,
provider, state, and runtime paths.

Non-blocking gaps remain for direct tests of explicit hook-input and SideEffect
input non-loss. Their non-default presence/count is included in the same
commitment and therefore follows the tested checkpoint mismatch path, but
dedicated regressions would improve local clarity.

## 8. Documentation Assessment

The roadmap, execution semantics, implementation plan, and blocker-fix report
accurately state that:

- the TOCTOU fix is implemented;
- grant validation precedes durable mutation;
- legacy grants fail closed while denials remain available;
- transient non-default posture blocks instead of disappearing;
- this is not a self-contained immutable run bundle;
- provider writes and broader production authority remain outside scope.

## 9. Blockers

None after the reconstruction-error mapping fix.

## 10. Non-Blocking Follow-Ups

- Replace the internal Debug-derived report-artifact policy encoding with
  explicit versioned labels when the commitment becomes an immutable bundle
  integrity root.
- Add direct explicit-hook and SideEffect-input mismatch regressions.
- Decide whether the commitment should gain a dedicated newtype after the run
  bundle model stabilizes.

## 11. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1783873725401799000-2`.
- Approval:
  `approval/run-1783873725401799000-2/review-scope-approved`.
- Presentation: `presentation/a7d03703e5e692a2`.
- Approval outcome: granted through the proof-enforced presentation path.
- Out-of-kernel work: Codex inspected code and tests, identified and fixed the
  reconstruction-error leakage blocker, authored this review, and ran local
  validation. The kernel governed scope and approval only.

## 12. Recommended Next Phase

Proceed to immutable run-bundle boundary planning. The next phase should define
content-addressed, durable, payload-conscious references for validated workflow,
skill, policy, governance, and configuration inputs. It must not expand provider
mutations, claim replay-grade handler attestation, or add hosted behavior.
