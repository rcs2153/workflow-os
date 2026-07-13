# Proportional Governance Workload Assessment Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation has the right product and architecture boundary: typed
facts, deterministic conservative mapping, monotonic composition with explicit
minima, independent execution and disclosure axes, explicit unresolved facts,
and no runtime authority. Full validation passes.

Two correctness blockers prevent accepting the fingerprint and explainability
contract. The fingerprint currently depends on machine pointer width, and an
action-class recommendation can be mislabeled as a workflow-declared reason.
Both fixes are small and local.

## 2. Scope Verification

The phase stayed within the approved model-only scope.

It did not add executor enforcement, automatic approvals, persistence, events,
audit records, report artifacts, schemas, CLI behavior, UI, repository
scanning, provider calls, new mutation families, hosted administration,
enterprise identity, or release-posture changes.

The implementation accepts explicit typed inputs and returns an in-memory
assessment. Existing runtime and provider paths do not consume it.

## 3. Product Boundary Assessment

The implementation correctly preserves the distinction raised by external
feedback:

- execution disposition decides proceed, approval, or denial;
- disclosure obligation decides quiet or visible presentation;
- a UI may display quiet decisions without changing governance;
- visible disclosure alone does not block execution.

The focused local-mutation test proves `Proceed + Visible`. No UI preference is
accepted as assessment input or authority.

The model also establishes the right onboarding precursor. Callers can provide
bounded action, authority, evidence/check, sensitivity, and SideEffect facts
instead of manually classifying every selector input. Safe repository metadata
derivation remains correctly deferred.

## 4. Assessment Model Assessment

The typed vocabulary is domain-neutral and appropriately narrow:

- action class distinguishes reads, local mutation, external mutation,
  unknown, and unsupported behavior;
- authority distinguishes sufficient, approval-required, unavailable, and
  unknown posture;
- evidence/check posture distinguishes satisfied, optional absence, required
  absence, failure, and unknown posture;
- sensitivity distinguishes routine, elevated, restricted, and unknown;
- SideEffect posture distinguishes none, local/external reversibility,
  irreversible, ambiguous, unsupported, and unknown posture.

Unknown facts are stable enum categories. Completeness is deterministic fact
coverage, not probabilistic confidence. No free-form workload payload is stored.

The v1 mapping is conservative. External mutations require approval, failed or
missing required checks deny, unavailable authority denies, ambiguous and
unsupported SideEffects deny, and unknown safety facts require approval.

## 5. Monotonicity And Authority Assessment

The helper reuses the accepted selector rather than creating a second policy
engine. Explicit workflow, policy, authority, evidence/check, sensitivity,
SideEffect, runtime, prior-decision, profile, and steward minima remain part of
the final composition.

Focused tests prove that quiet inference cannot downgrade those minima.
Strict-enterprise posture still fails closed without an explicit steward
minimum. Required-check failure and unavailable authority cannot be converted
into approval as a substitute for satisfying the invariant.

## 6. Fingerprint Assessment

The fingerprint has a sound intended shape:

- versioned domain identity;
- immutable definition root;
- governance profile;
- explicit minima;
- typed workload facts;
- runtime escalation;
- prior accepted posture;
- steward minimum.

Identical inputs are deterministic on the current architecture, and focused
tests show that changes across every modeled fact family alter the fingerprint.

The current byte encoding is not architecture-stable, however. `hash_field`
uses `usize::to_be_bytes()` for field lengths. A 64-bit host hashes an eight-byte
length while a 32-bit host hashes a four-byte length. That violates the claimed
stable cross-platform invalidation boundary.

## 7. Explainability Assessment

The accepted decision exposes stable reason codes, but inferred action posture
is currently merged into `input.workflow` before selection. An external action
therefore contributes `WorkflowRequirement`, whose documentation says a
workflow declaration supplied the requirement. No such declaration may exist.

This is a false provenance claim at the exact boundary intended to improve
explainability. Workload-assessment posture needs its own selector input and
stable reason, while the explicit workflow minimum remains mapped only to
`WorkflowRequirement`.

## 8. Privacy And Serialization Assessment

The boundary is payload-free. It cannot store prompts, source contents,
provider payloads, command output, paths, environment values, credentials, or
tokens. Debug output redacts the definition root and fingerprint. Serialized
output contains bounded enums, stable reasons, completeness, unresolved fact
categories, and a hash.

The result exposes assessed-not-enforced and not-persisted posture through
accessors. A future review should decide whether those posture values must also
be explicit serialized fields before any machine-facing projection or schema
exposure. That is non-blocking while this API remains in-memory and model-only.

## 9. Test Quality Assessment

Coverage is strong for:

- quiet read-only work;
- visible non-blocking local mutation;
- approval-gated external mutation;
- denial and unknown-fact matrices;
- every explicit-minimum family;
- strict-enterprise behavior;
- deterministic same-input output;
- invalidation across relevant inputs and immutable definition root;
- execution/disclosure independence;
- Debug and serialization non-leakage;
- existing selector and workspace regressions.

Missing blocker regressions:

- a fixed known fingerprint vector using fixed-width field framing;
- an assertion that inferred action escalation produces a workload-assessment
  reason and not `WorkflowRequirement`;
- an assertion that an explicit workflow minimum still produces
  `WorkflowRequirement` after the reason-source correction.

Non-blocking additions should cover unsupported action and SideEffect variants
explicitly and consider serialized assessed/not-enforced posture before a
machine-facing integration.

## 10. Documentation Assessment

The roadmap, implementation plan, quiet-success plan, and phase report are
clear that the helper is implemented but not reviewed, enforced, persisted,
configured through schemas, exposed through CLI/UI, or integrated into
onboarding. They correctly preserve separate mutation-family reviews for
GitHub comments, pull requests, Jira issues, and future provider writes.

The implementation report's stable-fingerprint claim must not be accepted
until fixed-width framing is implemented and tested.

## 11. Blockers

1. Replace architecture-width `usize` field-length framing with an explicit
   fixed-width encoding, preferably checked `u64` big-endian lengths, and add a
   known-vector regression test.
2. Add a distinct workload-assessment selector requirement and
   `GovernanceDecisionReason`, then keep inferred action posture separate from
   the explicit workflow-declared requirement. Add provenance regressions for
   both inferred and declared sources.

## 12. Non-Blocking Follow-Ups

- Add explicit unsupported action and SideEffect mapping tests.
- Decide whether assessment and persistence posture must serialize before any
  machine-facing projection.
- Keep the safe repository-metadata derivation adapter as the next phase only
  after blocker re-review acceptance.
- Do not let future natural-language or model inference become enforcement
  authority.

## 13. Recommended Next Phase

Implement a focused proportional-governance workload-assessment blocker fix.
Do not begin onboarding or runtime integration until fixed-width fingerprinting
and truthful reason provenance pass focused re-review.

## 14. Validation

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed before this review document and must be rerun at
  review close;
- `git diff --check`: passed before this review document and must be rerun at
  review close.

## 15. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783929002081152000-2`.
- Approval ID:
  `approval/run-1783929002081152000-2/review-scope-approved`.
- Approval presentation: `presentation/8fd669702d245e55`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Out-of-kernel work: Codex inspected source, tests, docs, and validation
  results and authored this review. The kernel coordinated governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.

## 16. Fix-Forward Note

The original blocker verdict remains part of the phase record. A subsequent
focused blocker phase replaced machine-width field lengths with fixed `u64`
big-endian framing, pinned a known v1 fingerprint vector, and added a distinct
workload-assessment selector input and reason so inferred action posture no
longer claims workflow-declaration provenance. Focused re-review accepts both
corrections; see
[Proportional Governance Workload Assessment Blocker Fix Review](PROPORTIONAL_GOVERNANCE_WORKLOAD_ASSESSMENT_BLOCKER_FIX_REVIEW.md).
