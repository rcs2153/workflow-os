# Proportional-Governance Runtime-Fact Source Executor Consumer Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation closes the first executor-consumer gap without broadening
defaults or mistaking source observations for authority. It composes the exact
immutable bundle, reviewed source-freshness helper, durable assessment binding,
and existing executor in a narrow additive path.

## 2. Scope Verification

The phase stayed within the approved explicit opt-in local executor scope. It
did not add automatic activation, disposition enforcement, checks, provider
calls, OpenShell, source snapshot persistence, schemas, CLI behavior,
SideEffects, writes, hosted behavior, or new mutation families.

## 3. Integration Boundary Assessment

The API takes explicit request values and an injected source. It does not read
hidden configuration or global state. Core persists or validates the immutable
bundle before source observation, and the source receives the exact stored
bundle binding and evaluation time.

Fresh execution derives and persists the assessment binding before run events.
This preserves the existing executor event ordering and avoids a detached
caller-supplied fact vector.

## 4. Retry Assessment

Exact retries first validate the request against the durable run and stored
immutable bundle. They then resolve one new source snapshot and require the
derived binding to equal both the run snapshot and create-only stored binding.
Mismatch fails before new events or duplicate skill execution.

The implementation correctly allows a new snapshot identity to reproduce the
same assessment. Snapshot identity is not itself the durable execution
authority.

## 5. Durability Assessment

The existing payload-free assessment binding is durably stored and projected.
The source snapshot remains call-local and serialize-only. That is an honest
boundary for this phase, but it means later approval-resume consumption needs a
separately reviewed durable snapshot commitment contract if source provenance
must be proven across process boundaries.

## 6. Privacy And Error Assessment

Source errors remain wrapped by the stable Core-owned source-failure error.
Executor-specific mismatch and missing-binding errors contain no caller values.
Request and result Debug output redact paths, source identities, bundle values,
hashes, timestamps, and snapshot identifiers. No raw facts or execution output
enter the run event stream through this integration.

## 7. Test Quality Assessment

Focused tests prove fresh success, durable binding event order, exactly one
source call per invocation, retry reassessment, no duplicate execution,
changed-fact rejection, no new events on mismatch, source-error non-leakage,
and redacted Debug output. Workspace validation protects existing executor,
approval, report, evidence, SideEffect, adapter, and runtime behavior.

## 8. Blockers

None for this explicit opt-in executor-consumer phase.

## 9. Non-Blocking Follow-Ups

- Define a durable source-snapshot commitment binding before using source facts
  during approval resume.
- Add persisted corruption and replay coverage with that durable model.
- Decide whether later report generation cites the accepted snapshot
  commitment or a separately persisted evidence reference.
- Keep registration as an explicit local trust decision until authenticated
  source identity is separately designed.

## 10. Product Feedback Reconciliation

Fresh-pull feedback says the kernel is coherent but ceremony must fall for
low-risk work. This phase advances that goal by making trustworthy current facts
available to an executor path without weakening evidence or audit posture. It
does not itself enable quiet-success defaults. The Node 24 integration-check
UX and duplicate missing-manifest diagnostic remain independent polish work.

## 11. Recommended Next Phase

Durable source-snapshot commitment binding, followed by a separately reviewed
approval-resume source consumer. Additional provider mutations and default
proportional-governance activation should remain later.

## 12. Validation Reviewed

- Focused source-backed local executor tests: passed.
- Focused clippy gate: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Governed Review Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786279690870244000-2`
- Approval ID: `approval/run-1786279690870244000-2/composition-approved`
- Presentation ID: `presentation/cda0ec008370e532`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: source-backed executor implementation, tests,
  documentation, validation, and git/PR work
