# Proportional-Governance Decision-Time Authority Receipt Report

## 1. Executive Summary

Workflow OS now has a dedicated payload-free receipt for one exact successful
proof-enforced, fresh-current-fact approval resume. The receipt explains which
run, approval event, immutable bundle, presentation proof commitment,
registered source commitment, decision-time snapshot, fact set, and reproduced
assessment supported the resume without copying raw facts or presentation
content.

The receipt is point-in-time, local, unsigned, reference-only, and explicitly
evidence rather than authorization. The phase does not add WorkReport citation,
persistence, schemas, CLI/UI behavior, automatic approval, provider execution,
OpenShell, SideEffects, writes, hosted expansion, enterprise identity, or
release changes.

## 2. Scope Completed

- Added the dedicated `GovernanceDecisionAuthorityReceipt` model.
- Added deterministic receipt identity and complete commitment validation.
- Added fixed operation, freshness, validity, signature, effect, and redaction
  vocabulary.
- Added an explicitly unverified deserialized claim type.
- Added an opaque Core-owned successful-outcome proof whose fields and
  construction remain private to the executor module.
- Added one additive wrapper around the accepted proof-enforced fresh-fact
  approval API.
- Emitted a receipt only for an exact proof-marked granted decision with
  matching V3 bindings.
- Added focused grant, denial, serialization, tamper, Debug, and payload
  exclusion tests.

## 3. Scope Explicitly Not Completed

The phase did not modify existing approval methods or defaults, add reusable
authority, persist receipts or facts, create WorkReport or EvidenceReference
citations, add workflow schemas or SDK fields, render CLI/UI output, infer or
automate approvals, invoke providers or OpenShell, model or execute new
SideEffects, add writes, expand hosted behavior, add enterprise identity, or
change release posture.

## 4. Model And Trust Boundary

The receipt is a sibling of the existing context-access `AuthorityReceipt`,
not a broadened generic envelope. Its only operation is
`approval_resume_reassessment_v1`.

Public callers cannot construct the trusted model from field definitions. The
receipt constructor consumes an opaque proof that only the executor module can
create after the accepted approval path has returned a matching grant event,
proof marker, durable V3 assessment binding, and decision-time snapshot.
Trusted receipts serialize but do not deserialize. Serialized data enters Core
as `UnverifiedGovernanceDecisionAuthorityReceipt`; structural validation does
not authenticate the producer, restore freshness, or confer authority.

## 5. Receipt Bindings

The V1 receipt binds the workflow and run, approval reference, approval
decision event, proof-marker commitment, immutable run bundle, durable
assessment-binding commitment, source registration commitment, decision-time
snapshot commitment, fact-set commitment and count, reproduced assessment
fingerprint, and issuance time. The receipt ID derives from the complete
commitment.

The operation is bound to the granted approval decision event rather than the
complete terminal execution outcome. Later report and artifact phases must not
reinterpret that event-scoped evidence as proof that all resumed work
succeeded.

## 6. Grant And Denial Semantics

The new API delegates approval, proof, immutable-plan, source, freshness, and
assessment enforcement to the existing accepted wrapper. It constructs the
receipt only after the returned run exposes exactly one matching proof-marked
`ApprovalGranted` event and matching payload-free current-fact bindings.

A proof-enforced denial follows existing fail-closed workflow semantics and
emits no receipt because no resume was authorized. Failed proof, source,
freshness, reassessment, or binding paths emit no receipt.

## 7. Privacy And Error Posture

The model stores no source or snapshot IDs, raw facts, approval reason,
presentation ID or content, paths, prompts, command output, provider payloads,
credentials, or tokens. Debug output redacts all identities, timestamps, and
commitments. Deserialization and validation errors use stable static messages
without caller values.

## 8. Product Feedback Reconciliation

Fresh-pull testing continues to validate the product boundary: Workflow OS is
a credible, honest governance kernel but remains more kernel than turnkey
execution platform. The next product problem is reducing ceremony for
low-risk work while preserving the evidence trail.

This receipt is a prerequisite for quiet success. Quiet capture can remove an
unnecessary prompt without erasing why a decision proceeded. Execution
disposition and operator disclosure remain independent: a UI may render quiet
records live, but durable disclosure obligations cannot depend on a UI being
open. Safe metadata may infer most recommended posture, but explicit policy,
authority, evidence, sensitivity, SideEffect, and steward minima remain the
deterministic enforcement floor.

## 9. Test Coverage

Focused tests exercise a real proof-enforced current-fact approval grant,
trusted receipt validation, bounded fixed posture, deterministic ID/commitment
relationship, unverified deserialization, tamper rejection, Debug and
serialization non-leakage, forbidden payload-field absence, and denial without
a receipt. Existing approval, runtime-fact, authority-receipt, WorkReport, and
executor tests remain the compatibility boundary.

## 10. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo check -p workflow-core --lib`: passed.
- Focused decision-time authority receipt tests: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `CARGO_TARGET_DIR=/private/tmp/workflow-os-target cargo test --workspace`:
  passed locally.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Limitations

- Receipts are returned in memory and are not persisted.
- WorkReport and EvidenceReference citation vocabulary is not implemented.
- Serialized claims have no authenticated verifier.
- Source registration remains an embedding trust decision.
- Receipts prove the approval-resume decision, not terminal work success.
- Quiet-success defaults and configuration remain separately scoped.

## 12. Recommended Next Phase

Add WorkReport citation vocabulary for the receipt ID only, then review it
before deriving citations or composing them into report generation. Do not add
persistence, providers, OpenShell, SideEffects, writes, hosted behavior, or
default changes in that phase.

## 13. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786295055710411000-2`
- Approval ID: `approval/run-1786295055710411000-2/composition-approved`
- Presentation ID: `presentation/dec1f9bb48b4e007`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR work
