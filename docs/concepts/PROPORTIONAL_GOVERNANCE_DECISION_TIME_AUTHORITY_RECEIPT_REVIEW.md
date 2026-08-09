# Proportional-Governance Decision-Time Authority Receipt Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The model closes the call-local explainability gap without turning evidence
into reusable authority or broadening existing approval defaults.

## 2. Scope Verification

The phase stayed within the dedicated model and opaque-producer boundary. It
did not add WorkReport citation, persistence, schemas, CLI/UI behavior,
automatic approval, providers, OpenShell, SideEffects, writes, hosted
expansion, enterprise identity, or release changes.

## 3. Model Assessment

The sibling model is preferable to broadening the existing context-access
receipt before a stable common envelope exists. V1 has one operation and fixed
posture values. It records bounded references and commitments only.

The complete commitment covers every semantic field, and the deterministic ID
derives from that commitment. Unknown vocabulary and inconsistent commitments
fail closed.

## 4. Construction And Trust Assessment

The trusted constructor accepts only an opaque proof with private executor
fields. The proof is produced only after the existing proof-enforced
fresh-current-fact wrapper succeeds and Core resolves exactly one matching
proof-marked grant event plus matching V3 bindings.

The existing approval API is unchanged. The new receipt-bearing wrapper is
explicit and additive. Denial returns no proof or receipt.

Trusted receipts are serialize-only, while deserialization yields an
explicitly unverified claim. Claim validation proves self-consistency only and
cannot restore producer trust, freshness, or execution authority.

## 5. Binding Assessment

The receipt binds the exact run and workflow, approval reference and event,
proof-marker commitment, immutable bundle, durable assessment, registered
source, decision-time snapshot, fact set, aggregate assessment, and issuance
time. It intentionally proves the granted decision rather than claiming the
entire resumed execution completed successfully.

## 6. Privacy And Error Assessment

No raw fact, source ID, snapshot ID, approval reason, presentation content,
path, command output, provider payload, credential, or token is stored. Debug
redacts identities, commitments, and timestamps. Errors remain stable and do
not echo malformed values.

## 7. Test Quality Assessment

The focused integration cases use the real current-fact source, durable
approval presentation, immutable bundle, and executor state machine. They
cover grant production, denial non-production, fixed evidence-only posture,
trusted validation, unverified round trip, commitment tampering, payload-field
absence, and Debug/serialization safety. Existing suites continue to own the
underlying missing/stale/ambiguous proof, source failure, fact mismatch,
registration, and immutable-bundle cases.

## 8. Product Feedback Assessment

The latest evaluator feedback is credible and aligns with the roadmap. The
kernel is coherent and honest; ceremony is now the main adoption constraint.
This receipt strengthens the evidence needed to make low-risk decisions quiet
without making them invisible or unauditable.

Visible disclosure should not collapse into transient UI state. A UI can show
quiet-capture decisions live, but policy-required disclosure remains a durable
obligation. Configuration should infer recommendations from safe validated
metadata while explicit minima and overrides remain authoritative. Changed
facts must invalidate prior assessment; inference cannot silently downgrade a
declared gate.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add receipt-ID citation vocabulary to WorkReport in a separate phase.
- Decide whether a future authenticated verifier is needed before persistence.
- Keep terminal resume-outcome evidence distinct from approval-decision
  evidence.
- Continue quiet-success work only with evidence-completeness metrics.

## 11. Recommended Next Phase

WorkReport citation vocabulary for the decision-time authority receipt ID,
model-only. Citation derivation, report composition, persistence, providers,
OpenShell, SideEffects, writes, and defaults should remain later boundaries.

## 12. Validation Reviewed

- Formatting and focused Core compilation passed.
- Focused receipt integration tests passed.
- Workspace clippy, full workspace tests, and docs checks passed locally.
- The required PR Rust job remains the merge-time validation boundary.

## 13. Governed Review Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786295055710411000-2`
- Approval ID: `approval/run-1786295055710411000-2/composition-approved`
- Presentation ID: `presentation/dec1f9bb48b4e007`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation review, tests, documentation, validation,
  and git/PR work
