# Proportional-Governance Selected Local Project Consumer CLI Adoption Plan Review

## 1. Executive Verdict

Plan accepted after compatibility corrections; proceed to the selected
fresh-run report adapter only.

## 2. Scope Verification

The plan remains limited to adopting the already-declared local
project-validation governance path. It does not activate governance for
undeclared projects, add commands or flags, broaden check profiles, execute a
provider or SideEffect, add schemas or examples, enable hosted behavior, or
change release posture.

The proposed sequence keeps CLI behavior unchanged until separately reviewed
Core prerequisites preserve the existing output and artifact contract.

## 3. Current Boundary Assessment

The inventory accurately identifies the current CLI path. `run` uses the older
Core-owned authoritative report helper, persists presentation proof for waiting
approvals, persists terminal artifacts through workflow-derived gates, and
prints the accepted human and JSON shape. `approve` currently uses separate
aggregate and authored-step compositions.

The accepted selected consumer owns the fixed runtime-fact source and fresh
evaluation time. It preserves separate aggregate and authored-step approvals
through one selected decision helper. It is therefore the correct authority
boundary for eventual CLI adoption.

## 4. Fresh-Run Adapter Assessment

The selected route retains the actual canonical check result, so the proposed
adapter can reuse the existing private report composition without executing the
check again. The adapter must expose the exact `LocalCheckResultReference`,
retain route truth on report failure, generate terminal reports for quiet,
visible, denied, and existing-terminal routes, and defer reports for waiting
approvals.

This is a small, reviewable Core-only prerequisite. It does not require CLI
changes or another authority model.

## 5. Approval Adoption Assessment

The original plan understated the selected decision result gap. The current
result does not expose the exact decision-time check reference or a bounded gate
kind, even though the existing CLI prints the reference and distinguishes
`approval_decision` from `authored_approval_decision`.

The corrected plan therefore requires a bounded Core adoption envelope. Core
derives gate kind from the durable approval request and uses one selected
decision path for both gates. CLI may use gate kind only to preserve the public
route label; it may not choose divergent execution behavior.

The current selected helper returns a transient receipt when an aggregate grant
advances to a separate authored gate. That receipt is not persisted and no
artifact is written while the run is non-terminal. The corrected plan now
states this explicitly.

## 6. Artifact Compatibility Assessment

The existing CLI artifact path derives high-assurance disclosure and
approval-proof-marker policy from the immutable workflow definition and writes
proof-marker projections through a dedicated local store. The current selected
decision helper accepts caller policy values and does not expose equivalent
workflow-derived projection results.

CLI adoption must not treat those paths as equivalent. The corrected plan
requires workflow-derived gate parity and retains the deterministic projection
store at `<state-root>/audit-projections/approval-proof-markers`. Receipt
records use
`<state-root>/governance-decision-authority-receipts`; artifacts and SideEffect
records continue through the existing state backend.

## 7. Denial Compatibility Assessment

The current declared initial-denial route generates and persists a truthful
terminal report artifact. The selected approval denial is intentionally
check-free, source-free, receipt-free, and currently artifact-free.

Before CLI cutover, the selected approval envelope must preserve terminal
denial reporting from durable run and prior selected-assessment references. It
must not rerun the project check, invoke a source, fabricate evidence, or issue
an authority receipt. This resolves the original plan's open denial question
without weakening the selected denial boundary.

## 8. Output And Semantics Assessment

The plan freezes the existing commands, activation declaration, JSON keys,
route labels, quiet-success summary, visible disclosure, approval handoff,
report and artifact posture, exit behavior, stable error families, and event
ordering. It correctly forbids production shadow execution because duplicate
checks or workflow effects would not be observational.

Ordinary execution remains unchanged for projects without the exact supported
declaration.

## 9. Privacy And Failure Assessment

The plan preserves Core-owned facts, source registration, evaluation time,
governance disposition, disclosure, and receipt derivation. Debug, errors, and
CLI output remain bounded and may not expose raw facts, paths, command output,
report text, environment values, provider payloads, or credentials.

Pre-decision failures remain mutation-free. Post-decision report or persistence
failures retain truthful workflow status. Ambiguous writes block blind retry.

## 10. Test Assessment

The corrected test plan covers route and report parity, one check invocation,
exact check-reference propagation, both approval gate kinds, transient receipt
posture, denial artifact closure without recheck, workflow-derived artifact
policy, proof-marker projection, output compatibility, retries, and privacy.

The eventual CLI phase must retain exact JSON key/value compatibility and the
accepted semantic human-output assertions. It must also prove that undeclared
projects and existing public Core APIs remain unchanged.

## 11. Planning Corrections Made

- Added the selected approval adoption envelope as a reviewed prerequisite.
- Required exact local-check reference propagation from Core.
- Required bounded Core-owned approval-gate kind projection.
- Corrected the non-terminal aggregate receipt description.
- Fixed deterministic receipt and proof-marker store paths.
- Required workflow-derived artifact and proof-marker gate parity.
- Resolved denial posture in favor of truthful terminal artifacts without a
  decision-time check rerun.
- Removed implementation-significant open questions.

## 12. Blockers

None after the planning corrections above.

## 13. Non-Blocking Follow-Ups

- Keep the fresh-run adapter and approval adoption envelope as separate
  implementation/review phases.
- Keep old public Core APIs available through compatibility observation.
- Retire duplicate CLI-only composition only after the complete CLI adoption is
  accepted.

## 14. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1786445276985535000-2`.
- Approval ID:
  `approval/run-1786445276985535000-2/review-scope-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/63c7b37f08ef4bde`.
- Terminal status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, no retries, and no
  escalations.
- Approval-presentation enforcement: proof enforced with the presentation
  marker present in the durable event trail.
- Validation: `npm run check:docs` and `git diff --check` passed.

The delegated maintainer performed source review, planning corrections,
validation interpretation, documentation, and git work outside the kernel. The
kernel governed scope and approval and retained the durable phase trail; it did
not edit files, execute checks, mutate git state, push the branch, or merge the
pull request.

## 15. Recommended Next Phase

Implement the selected fresh-run report-composition adapter only. Review it
before implementing the selected approval adoption envelope. Do not change CLI
behavior until both Core prerequisites are accepted.
