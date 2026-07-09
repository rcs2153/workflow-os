# Approval Proof Marker WorkReport And Audit Citation Plan Report

## 1. Executive Summary

This phase planned how WorkReports and future audit summaries should cite approval decision proof markers.

The plan keeps approval-presentation payloads out of reports and audit projections. It recommends citing proof-enforced approval decisions through existing approval decision and workflow event references, with bounded summaries and explicit missing-citation behavior.

## 2. Scope Completed

- Created [Approval Proof Marker WorkReport And Audit Citation Plan](../implementation-plans/approval-proof-marker-workreport-audit-citation-plan.md).
- Defined citation source-of-truth boundaries.
- Defined WorkReport citation policy.
- Defined future audit projection policy.
- Defined missing and marker-free compatibility behavior.
- Defined privacy, redaction, error-handling, tests, and implementation sequence.

## 3. Scope Explicitly Not Completed

- No implementation.
- No automatic report generation.
- No report artifact writes.
- No persistence changes.
- No CLI rendering changes.
- No workflow schema changes.
- No examples.
- No provider writes.
- No side-effect execution.
- No hosted behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Planning Summary

The plan treats the approval decision event as the source of truth for proof-marker use. A durable approval-presentation record proves the approval scope was presented; the approval decision proof marker proves the approval decision used that proof.

The first implementation should be a pure in-memory citation derivation helper that consumes explicit `WorkflowRun` input and emits bounded report citations without mutating runtime state.

## 5. Privacy Summary

The plan forbids copying approval handoff text, work summaries, strict non-goals, validation expectations, provider payloads, command output, source/spec contents, credentials, tokens, and secret-like values into report citations or audit projection.

Citation summaries should remain short posture text, not payload replicas.

## 6. Validation

- passed: `npm run check:docs`

Rust tests were not run because this phase changed planning documentation only.

Governed planning phase:

- workflow ID: `dg/d`
- run ID: `run-1783612458303465000-2`
- approval ID: `approval/run-1783612458303465000-2/planning-approved`
- approval-presentation ID: `presentation/31668ae90dc455fa`
- approval-presentation content hash: `31668ae90dc455fab0abb51568323f6c6a546a67d9c829c645fd5cc1289bed00`
- approval outcome: granted

## 7. Recommended Next Phase

Recommended next phase: approval proof marker citation derivation helper implementation.

Why: proof markers are already modeled, wired into approval events, visible in inspect output, and accepted by review. A pure helper is the smallest code slice that reduces the gap between proof-marker runtime data and WorkReport/audit usage without introducing automatic reports, artifact writes, schemas, provider writes, or release posture changes.
