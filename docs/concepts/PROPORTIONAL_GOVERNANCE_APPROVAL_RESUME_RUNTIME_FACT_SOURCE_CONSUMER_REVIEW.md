# Proportional-Governance Approval-Resume Runtime-Fact Source Consumer Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation closes the grant-time source-authority gap on one explicit
local path without changing defaults. It validates both the exact approved
execution context and fresh registered facts before mutating approval state.

## 2. Scope Verification

The phase remained within the accepted local opt-in consumer scope. It did not
add automatic approval, default activation, raw fact persistence, schemas,
CLI/UI behavior, providers, OpenShell, SideEffect execution, writes, hosted
expansion, reusable authority, enterprise identity, or release changes.

## 3. API Assessment

The API requires the caller to provide the approval request, profile,
registration, evaluation time, store, and injected source. It does not discover
authority or source configuration from ambient state. The result distinguishes
granted source-backed reassessment from source-free denial without fabricating
evidence.

## 4. Ordering Assessment

The implementation improves on the plan's initial wording. Approval preparation
validates pending state, while resolved-context integrity belongs to resume-plan
preparation. The code therefore reconstructs and freezes the resume plan before
the source call, then performs durable registration and assessment validation,
then applies events and execution through that frozen plan.

This is the correct order. It prevents changed workflow, skill, or policy state
from being read after the fresh-fact decision and prevents grant/resume mutation
before either context or fact validation succeeds.

## 5. Grant And Denial Assessment

Matching fresh facts complete the run and return payload-free decision-time
metadata while preserving the initial commitment. Changed facts leave exact
event history untouched and invoke no skill.

Denial invokes no source and returns no decision-time snapshot. That preserves
fail-closed availability and avoids making rejection dependent on a healthy
observation system.

## 6. Provenance And Compatibility Assessment

Registration commitment equality is checked before source invocation. The
existing source-assessment helper owns identity, version, bundle, freshness,
coverage, and canonical assessment validation. V1 and V2 bindings are rejected
on this path because they cannot prove source provenance.

The durable initial snapshot remains provenance rather than reusable authority.
The fresh snapshot is call-local metadata and does not overwrite durable state.

## 7. Privacy And Error Assessment

The new request and result Debug implementations expose posture and presence
only. Stable errors omit paths, IDs, timestamps, fact values, approval reasons,
and source-local messages. Source failure tests prove secret-like markers do not
cross the Core boundary.

## 8. Test Quality Assessment

The focused matrix proves successful grant, changed-fact failure, exact event
equality, registration preflight before source access, denial without source
access, context validation before source access, source-error non-leakage,
legacy-binding rejection, no failed-path skill invocation, and Debug
non-leakage. Existing source freshness, source identity, bundle identity,
coverage, duplicate decision, presentation proof, and workspace tests supply
the remaining regression coverage.

## 9. Product Feedback Reconciliation

Fresh-pull evaluation again identifies low-risk ceremony as the main product
friction and recommends proportional governance plus quiet success. This phase
is a prerequisite rather than a visible UX change: a kernel cannot safely reduce
prompts at approval-resume boundaries while trusting caller-classified current
facts. The implementation strengthens the evidence needed for later
deterministic quiet decisions without enabling those defaults now.

The reported Node 24 integration-check opacity and duplicate missing-manifest
diagnostic are already fixed on current `main` and did not require scope drift.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Compose persisted approval-presentation proof with this source consumer.
- Decide whether report evidence cites initial provenance, a bounded
  decision-time receipt, or both.
- Design deterministic escalation for changed assessments separately.
- Keep production clock authority and authenticated source identity separate.

## 12. Recommended Next Phase

Implement a proof-enforced approval-resume wrapper that reuses the accepted
source reassessment and frozen resume-plan boundary. Keep defaults, provider
mutations, OpenShell, SideEffects, and writes unchanged.

## 13. Validation Reviewed

- Focused approval-resume runtime-fact tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed in the required PR Rust job; the equivalent
  local run was stopped after passing CLI unit and integration sets because
  macOS process startup made the 64-binary run impractical.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 14. Governed Review Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786284384652726000-2`
- Approval ID: `approval/run-1786284384652726000-2/composition-approved`
- Presentation ID: `presentation/ec421e5cdf4c2feb`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation review, focused and workspace validation,
  documentation, and git/PR work
