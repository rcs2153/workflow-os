# Proportional-Governance Approval-Resume Runtime-Fact Source Presentation-Proof Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation composes the two accepted approval-resume integrity gates
without creating another approval state machine or changing defaults. The exact
presented decision is proven before source access, and fresh source facts are
proven before grant mutation.

## 2. Scope Verification

The phase remained within the explicit local wrapper scope. It did not add
automatic approval, default activation, raw fact persistence, schemas, CLI/UI
behavior, providers, OpenShell, SideEffect execution, writes, hosted expansion,
enterprise authority, or release changes.

## 3. API And Composition Assessment

The request requires explicit store, source, source registration, profile,
decision time, approval request, proof selector, and optional freshness and
fingerprint constraints. No authority is discovered from ambient state.

The implementation factors existing proof validation and source reassessment
behind narrow private composition helpers. Existing proof-only and source-only
public APIs retain their behavior.

## 4. Ordering Assessment

Pending approval validation occurs first. Presentation proof is then resolved
and validated before the runtime-fact source can be invoked. For a grant, the
existing grant precondition freezes the resume plan and owns immutable-bundle,
registration, source, freshness, coverage, and assessment validation before
events or execution.

This ordering is correct. It prevents an unpresented decision from consulting
runtime authority and prevents stale or changed facts from authorizing a
mutated run.

## 5. Grant And Denial Assessment

Successful grants carry the proof marker, return payload-free decision-time
metadata, and complete only after exact reassessment. Changed facts preserve
the exact pre-decision event vector and invoke no skill.

Denial proves what was presented but makes no decision-time source call. It
returns no fabricated reassessment binding or snapshot and follows existing
fail-closed semantics.

## 6. Privacy And Error Assessment

Debug output does not expose approval, presentation, source, snapshot, bundle,
time, or fingerprint identifiers. Existing stable proof and source errors own
their failures without copying caller values. No raw fact or presentation
payload enters durable events.

## 7. Test Quality Assessment

The new focused matrix directly proves valid grant, missing/stale/ambiguous
proof precedence, exact event equality after fact change, source-free denial,
proof-marker presence, and Debug safety. Existing focused suites continue to
cover corrupt and mismatched proofs, registration and bundle mismatch, source
failure, V1/V2 rejection, duplicate decisions, and non-leaking errors.

The absence of duplicated cases in the new wrapper suite is acceptable because
the wrapper delegates to those already-tested owners and the new tests target
the composition ordering.

## 8. Product Feedback Assessment

The latest fresh-pull review is directionally correct: Workflow OS now explains
its local kernel boundary well, and proportional governance should reduce
low-risk ceremony next. That does not justify inference-only enforcement or
collapsing visible disclosure into UI state. A UI may display quiet-capture
records live, but a declared disclosure obligation must remain durable,
machine-readable, and enforceable when no UI is open.

The current phase is a prerequisite for that product direction because quiet
or inferred decisions cannot safely survive approval resume without exact
presentation and current-fact integrity.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Define a bounded report citation for decision-time source authority.
- Add direct integration cases for corrupt and cross-decision proof if future
  refactoring weakens delegation to the proof owner.
- Design deterministic escalation for changed assessments separately.
- Keep inference advisory unless explicit minima, authority, and policy permit
  the resulting disposition.

## 11. Recommended Next Phase

Decision-time runtime-fact snapshot citation planning, followed by a model-only
bounded authority receipt if the plan is accepted. This is more valuable than
broadening mutation families because it closes the explainability loop for the
new resume-time authority.

## 12. Validation Reviewed

- `cargo fmt --all --check`: passed.
- `cargo check -p workflow-core --tests`: passed.
- Focused proof-enforced current-runtime-fact approval tests: passed.
- Workspace clippy, docs, and diff checks passed. The local workspace test run
  built successfully and passed its first binary before being stopped due to
  multi-minute macOS startup per binary; the required PR Rust job remains the
  authoritative full-workspace test boundary.

## 13. Governed Review Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786290901603991000-2`
- Approval ID: `approval/run-1786290901603991000-2/composition-approved`
- Presentation ID: `presentation/63e9ba7176ac33cf`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation review, validation, documentation, and
  git/PR work
