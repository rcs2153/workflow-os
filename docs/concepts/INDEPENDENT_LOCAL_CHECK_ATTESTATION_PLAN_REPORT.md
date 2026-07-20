# Independent Local Check Attestation Planning Report

## 1. Executive Summary

External dogfood testing correctly identified that a bounded local check result
or mock skill success is not independent engineering evidence. Planning now
defines a payload-free attestation boundary tied to exact command, invocation,
handler, immutable-run, result, provenance, and freshness posture.

## 2. Scope Completed

- Reconciled current local check, immutable bundle, report, evidence, approval,
  capability, and proportional-governance foundations.
- Defined source-of-truth boundaries between results, references, evidence,
  reports, requirements, and accepted attestations.
- Defined candidate core model, verifier, binding, freshness, privacy, failure,
  persistence, event, and test posture.
- Sequenced the first implementation as model-only.

## 3. Scope Explicitly Not Completed

No model, verifier, handler, execution, registration, persistence, event,
evidence attachment, report integration, schema, CLI, UI, provider call, write,
hosted runner, cryptographic proof, or release change was implemented.

## 4. Product Decision

Workflow OS must not equate a valid `LocalCheckResult`, a stable result
reference, a mock handler outcome, or a caller assertion with independent proof.
An accepted future attestation must be produced by a separately reviewed
verification boundary over kernel-observed invocation context and the immutable
run bundle.

## 5. Recommended Next Phase

Implement independent local check attestation requirement, assurance/source,
freshness, and payload-free binding models only, followed by focused maintainer
review.

## 6. Governed Phase Record

- Workflow: `dg/d`
- Run ID: `run-1784508051568281000-2`
- Approval ID: `approval/run-1784508051568281000-2/planning-approved`
- Presentation ID: `presentation/cd8bc556b8d58388`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: repository inspection, architecture reasoning,
  documentation edits, and validation commands
