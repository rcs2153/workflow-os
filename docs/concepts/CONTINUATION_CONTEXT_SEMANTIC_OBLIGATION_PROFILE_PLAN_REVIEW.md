# Continuation Context Semantic-Obligation Profile Plan Review

## 1. Verdict

**Accepted after three security corrections. Proceed to one private model-only
implementation increment.**

The plan closes the remaining caller-downgrade paths in the current
continuation-context model. Collector expectations are already Core-owned, but
authoritative understanding is not operationally meaningful while callers can
clear required claims, mark material sources optional, or force legitimate
multi-valued work through a singleton conflict algorithm.

## 2. Reviewed Scope

The phase is planning and review only. It does not add:

- Rust behavior;
- collectors or persistence;
- accepted-intent capture;
- CLI or schema exposure;
- runtime consumers or compaction hooks;
- Salsa;
- provider mutation;
- nested harnesses; or
- execution authority.

The proposed profile remains private to Core and composes the accepted
continuation inventory, coverage, bundle, understanding, and same-call
authority boundaries.

## 3. Finding One: Coverage Is Not Semantic Completeness

The first draft direction risked treating the collector expectation matrix as
the semantic requirement matrix. That would be incorrect.

A collector can run successfully and return empty. This proves that the source
class was checked. It does not prove that objective, scope, remaining work,
validation, reporting, or lawful next actions are known. Conversely, a source
class may be optional for collection while an admitted authoritative source
from that class becomes necessary to satisfy a required claim.

The corrected plan keeps two commitments:

- source coverage proves collector participation and outcome; and
- semantic completeness proves Core-owned claim and source obligations.

Both must pass before the private decision can become ready.

## 4. Finding Two: Empty Posture Must Be Typed

Many understanding categories must be accounted for even when empty. Requiring
one value for every blocker, wait, question, uncertainty, amendment, or
follow-up would force callers or models to emit fabricated `none` text.

The corrected plan distinguishes required values from required accounting. An
authoritative empty result is a typed outcome tied to collector and source
posture. It is not a semantic string and cannot be invented by a model.

This distinction is necessary for quiet success: low-risk work should not be
blocked merely because there are no blockers, but the kernel must still prove
that blocker posture was assessed.

## 5. Finding Three: Work Is Often Set-Valued

The current model treats different values at the same authority rank as a
conflict. That is correct for objective and scope. It is incorrect for multiple
decisions, non-goals, completed obligations, evidence requirements, checks,
blockers, follow-ups, or lawful next options.

The corrected plan makes cardinality a Core-owned part of the semantic profile.
Singleton claims require one unique winning value. Set-valued claims retain all
distinct winning values in deterministic order with per-value provenance.

The first increment does not infer contradiction between natural-language set
members. It preserves evidence rather than pretending the kernel can understand
semantic opposition without a typed relation.

## 6. Source-Necessity Review

The proposed source matrix is conservative and appropriate for the first
private increment:

- pre-run source necessity follows the accepted pre-run coverage posture;
- run-bound sources are mandatory except agent advisory and normally provider
  observation;
- provider observation becomes mandatory for provider reconciliation; and
- agent advisory can never satisfy a required claim.

This does not mean a mandatory collector must fabricate a source when it
observes nothing. Coverage records the empty observation; semantic accounting
then determines whether that emptiness is sufficient.

The implementation review must reject any path that copies
`required_for_purpose` into the new model under another name.

## 7. Claim-Matrix Review

The purpose-specific matrix is intentionally strict at material boundaries:

- objective, scope, and lawful next options always require values;
- implementation and material continuation require remaining work and concrete
  validation/reporting posture;
- approval review requires evidence, checks, SideEffect, validation, and
  reporting posture;
- provider reconciliation requires evidence and SideEffect posture; and
- blockers, waits, uncertainty, and follow-ups require complete accounting even
  when empty.

This is governance orientation, not execution admission. A complete projection
still cannot grant policy satisfaction, approval, capability, or authority.

## 8. Commitment And Privacy Review

The semantic profile identity, obligations, accounting outcomes, cardinality,
resolution, retained value commitments, provenance, and gaps must affect all
dependent commitments. Otherwise a caller could reuse a projection commitment
across different semantic contracts.

Errors and Debug output must remain payload-free. The implementation must not
expose claim text, source identity, protected topology, paths, prompts, or
provider payloads merely to explain a semantic failure.

## 9. Salsa Review

This phase does not select Salsa. The semantic profile and expected canonical
outputs must be implemented and accepted using straightforward deterministic
Core logic first.

A later Salsa prototype may optimize invalidation and recomputation only after
it proves identical commitments, precise invalidation, irrelevant-input
stability, empty-cache reconstruction, and no authority-bearing cache state.

## 10. Test Review

The required tests cover the important adversarial paths:

- caller substitution;
- coverage-complete but semantics-incomplete posture;
- false optional source labeling;
- fabricated negative facts;
- advisory satisfaction attempts;
- singleton conflict;
- deterministic set union;
- per-value provenance;
- commitment binding; and
- non-authorizing readiness.

The implementation report must include the encoded matrices. A count of tests
without the matrices would be insufficient evidence that all purposes, claims,
and source classes were covered.

## 11. Blockers

None after the three corrections above.

Implementation remains blocked from collectors and runtime consumers until the
private profile is implemented, the focused and full test suites pass, and a
separate implementation/security review accepts the exact head.

## 12. Accepted Next Phase

Implement the private semantic-obligation profile in
`crates/workflow-core/src/continuation_context.rs` only, plus focused tests,
roadmap status, an implementation report, and an implementation review.

Do not add collectors, persistence, accepted-intent capture, CLI output,
schemas, Salsa, provider behavior, or runtime integration.
