# Proportional-Governance Approval-Resume Runtime-Fact Source Presentation-Proof Report

## 1. Executive Summary

Workflow OS now has one explicit local approval-resume API that requires both
durable proof of the exact presented approval scope and fresh current facts
from the registered runtime-fact source. Presentation proof is validated before
source access. A grant then must reproduce the durable V3 governance assessment
before any approval, resume, policy, or skill event is appended.

Denial also requires presentation proof but remains source-free. The phase is
additive and opt-in; it does not change default approval behavior or add
automatic approval, schemas, CLI/UI behavior, providers, OpenShell,
SideEffects, writes, hosted behavior, or release changes.

## 2. Scope Completed

- Added an explicit proof-enforced current-runtime-fact approval request.
- Added one local wrapper composing existing proof and source gates.
- Reused the existing approval preparation and mutation state machine.
- Validated proof before source invocation.
- Preserved fresh registered-source reassessment before grant mutation.
- Preserved proof-enforced, source-free denial.
- Reused the existing bounded proof marker and payload-free snapshot result.
- Added focused ordering, atomicity, privacy, and compatibility tests.

## 3. Scope Explicitly Not Completed

The phase did not change ordinary approval APIs or defaults, infer or grant
approval authority, persist raw facts or presentation content, add schemas,
CLI/UI behavior, reports or artifacts, provider execution, OpenShell
integration, SideEffect execution, external writes, hosted expansion,
enterprise identity, or release posture.

## 4. API Summary

`LocalCurrentRuntimeFactsGovernanceApprovalPresentationDecisionRequest` carries
the existing proof-enforced approval request plus the explicit profile,
registered source commitment, decision time, and optional expected aggregate
fingerprint.

`decide_approval_with_current_runtime_facts_governance_reassessment_and_presentation(...)`
accepts the executor, immutable-bundle store, injected source, and request. It
returns the existing source-backed approval result rather than inventing a
second state or evidence model.

## 5. Ordering And Atomicity

The implementation prepares the exact pending approval, resolves and validates
the durable presentation record, and attaches the bounded proof marker in
memory before source access. A grant then freezes the resume plan, validates
the immutable bundle and V3 source commitment, resolves current facts once,
and requires the fresh assessment to reproduce durable governance state before
mutation.

Proof failure therefore takes precedence over source failure. Source or
assessment failure leaves the ordered event vector exactly unchanged and does
not invoke a skill.

## 6. Grant And Denial Semantics

A valid grant returns the completed run, accepted durable assessment binding,
and payload-free decision-time snapshot. The approval event carries the
existing proof marker.

A valid denial invokes no decision-time source, returns no reassessment binding
or snapshot, carries the proof marker, and follows the existing fail-closed run
semantics.

## 7. Privacy And Error Posture

The new request redacts the nested approval request, time, and expected
fingerprint and exposes no presentation or source identifiers through Debug.
The composition reuses stable proof and source error codes. It persists neither
raw facts nor presentation text and adds no provider payloads, command output,
paths, tokens, or credentials to events or errors.

## 8. Test Coverage

Focused tests cover successful proof-enforced grant, bounded proof-marker
presence, accepted decision-time metadata, missing/stale/ambiguous proof before
source access, changed facts with exact event equality, source-free denial,
request/result Debug safety, and no skill invocation on failed reassessment.
Existing proof-only and source-only suites retain corrupt/mismatched proof,
registration, bundle, freshness, legacy-binding, duplicate-decision, and error
non-leakage regression coverage.

## 9. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo check -p workflow-core --tests`: passed.
- Focused proof-enforced current-runtime-fact approval tests: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: the local run built the workspace and passed its
  first test binary, then was stopped because macOS startup was taking about
  five minutes per already-built binary across the multi-binary suite; the
  required PR Rust job remains the authoritative full-workspace validation.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Limitations

- The API remains explicit and opt-in.
- Decision-time snapshots are returned but not durably cited in reports.
- Changed assessments fail closed rather than route to a wider escalation UX.
- Source registration is an embedding trust decision rather than authenticated
  remote attestation.
- Proportional-governance configuration and quiet-success defaults remain
  separately scoped.

## 11. Product Feedback Reconciliation

Fresh-pull evaluation confirms that first-run honesty and approval/audit
boundaries are credible, while low-risk ceremony remains the main adoption
constraint. This phase strengthens the resume-time authority needed before
quiet capture can safely become common. It does not mistake a quieter UI for a
weaker governance obligation: live disclosure may be rendered by a local UI,
but policy-required disclosure must remain durable and auditable independent
of presentation surface.

## 12. Recommended Next Phase

Plan a bounded decision-time snapshot citation or authority-receipt boundary so
reports can explain which fresh facts authorized resume without persisting raw
facts. Keep broad approval defaults, provider mutations, and OpenShell wiring
unchanged until that evidence boundary is reviewed.

## 13. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786290901603991000-2`
- Approval ID: `approval/run-1786290901603991000-2/composition-approved`
- Presentation ID: `presentation/63e9ba7176ac33cf`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR work
- Missing coverage: default activation, raw fact persistence, report citation,
  providers, OpenShell, SideEffects, and writes were disclosed and not simulated
